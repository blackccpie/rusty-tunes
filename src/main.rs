/*
The MIT License

Copyright (c) 2024 Albert Murienne

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

#[macro_use]
extern crate serde_derive;

use crate::parser::Track;
use crate::parser::ApplePlist;

mod parser;

mod deezer_wrapper;

use clap::Parser;
use eframe::egui;
use rand::seq::SliceRandom;

#[derive(Parser)]
struct Cli {
    mode: String,
    path: std::path::PathBuf,
}

struct RandomTrack { 
    track_url: String,
    cover_url: String 
} 

// our application state
//#[derive(Default)]
struct RandomTrackApp
{
    //client: Deezer,
    itunes_tracks: Vec<Track>,
    random_track_str: String,
    random_track_url: String,
    random_cover_url: String,

    message_channel: (
        std::sync::mpsc::Sender<RandomTrack>,
        std::sync::mpsc::Receiver<RandomTrack>,
    )
}

#[tokio::main]
async fn main() {
    
    let args = Cli::parse();

    let itunes_library: ApplePlist = plist::from_file(args.path).unwrap();
    
    if args.mode == "P"
    {
        println!("----------- PLAYLISTS -----------");
        let playlists = &itunes_library.Playlists;
        for playlist in playlists.iter()
        {
            println!("{:?}", playlist.Name);
        }
    }
    else if args.mode == "T"
    {
        println!("----------- TRACKS (deezer matched) -----------");
        let tracks = &itunes_library.Tracks;

        //println!("{:?}, {:?}", itunes_library.major_version, itunes_library.minor_version);
        //print_hashmap(tracks);

        let mut dee = deezer_wrapper::Wrapper::new(); // TODO : try to remove mut attribute?

        for (_key, value) in tracks
        {
            let (artist, title, _link, _cover) = dee.search(&value.Name, &value.Artist).await; // TODO : remove await, why search has to be async?
            println!("{:?} / {:?}", artist, title);    
        }
    }
    else if args.mode == "R"
    {
        println!("----------- RANDOM TRACK! -----------");

        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
        };

        let itunes_tracks_vec = itunes_library.Tracks.values().cloned().collect();

        eframe::run_native("Rusty-Tunes Randomnessssss", options, Box::new(|ctx| {
                // This gives us image support:
                egui_extras::install_image_loaders(&ctx.egui_ctx);

                Box::new( RandomTrackApp{ 
                    //client: Deezer::new(),
                    itunes_tracks: itunes_tracks_vec,
                    random_track_str: "--".to_owned(),
                    random_track_url: "undefined".to_owned(),
                    //random_cover_url: "https://e-cdns-images.dzcdn.net/images/cover/2e018122cb56986277102d2041a592c8/250x250-000000-80-0-0.jpg".to_owned(),
                    random_cover_url: "https://t3.ftcdn.net/jpg/03/13/23/76/240_F_313237633_0thdqc4pwnBsjDbFw6rxV8b8fIh6ncPd.jpg".to_owned(),
                    message_channel: std::sync::mpsc::channel()
                })
            })
        );

    }
    else
    {
        println!("Unknown mode!");
    }

    println!("Goodbye!");
}

impl eframe::App for RandomTrackApp {
    /// the update method we have to keep fast
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame)
    {
        ctx.request_repaint();
        loop {
            match self.message_channel.1.try_recv() {
                Ok(random_track) => {
                    //println!("{:?}",random_track);
                    self.random_track_url = random_track.track_url;
                    self.random_cover_url = random_track.cover_url;
                }
                Err(_) => {
                    break;
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal(|ui|
            {
                let name_label = ui.label("Track: ");
                ui.text_edit_singleline(&mut self.random_track_str)
                    .labelled_by(name_label.id);
            });
            if ui.button("Randomize").clicked()
            {
                // not very optimal random pick...
                let random_track = self.itunes_tracks.choose(&mut rand::thread_rng()).unwrap().clone();
                println!("Selected track: {:?} / {:?}", random_track.Name, random_track.Artist);
                self.random_track_str = format!("{} / {}", random_track.Name, random_track.Artist);

                let message_sender = self.message_channel.0.clone();
                let _ = tokio::spawn(async move {

                    // TODO : not very clean to instanciate new client each time...
                    let mut dee = deezer_wrapper::Wrapper::new();

                    let (_artist, _title, link, cover) = dee.search(&random_track.Name, &random_track.Artist).await;

                    message_sender.send( RandomTrack {
                        track_url: link,
                        cover_url: cover
                    }).unwrap();
                });
            }
            if ui.button("Open").clicked()
            {
                let _ = open::with(&self.random_track_url, "firefox");
            }
            ui.image(self.random_cover_url.clone());
        });
    }
}
