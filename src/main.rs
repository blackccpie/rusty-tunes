/*
The MIT License

Copyright (c) 2017-2017 Albert Murienne

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

use clap::Parser;
use deezer_rs::Deezer;
use eframe::egui;
use rand::seq::SliceRandom;
//use reqwest::blocking;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Clone)]
struct Track {
    #[serde(rename = "Track ID")]
    track_id: i32,
    Name: String,
    #[serde(default = "default_artist")]
    Artist: String, // some tracks are missing artist...
}

fn default_artist() -> String {
    "Unknown".to_string()
}

#[derive(Debug, Deserialize)]
struct Playlist {
    Name: String,
    #[serde(rename = "Playlist ID")]
    playlist_id: i32,
    #[serde(default)]
    Tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
struct ApplePlist {
    #[serde(rename = "Major Version")]
    major_version: i32,
    #[serde(rename = "Minor Version")]
    minor_version: i32,
    Tracks: HashMap<String, Track>,
    Playlists: Vec<Playlist>,
}

fn print_hashmap(hashmap: &HashMap<String, Track>) {
    for (key, value) in hashmap {
        println!("Key: {:?}, Value: {:?}", key, value.Name);
    }
}

#[derive(Parser)]
struct Cli {
    mode: String,
    path: std::path::PathBuf,
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
        println!("----------- TRACKS -----------");
        let tracks = &itunes_library.Tracks;

        //println!("{:?}, {:?}", itunes_library.major_version, itunes_library.minor_version);
        //print_hashmap(tracks);

        let client = Deezer::new();

        for (_key, value) in tracks
        {
            let search_string: String = format!("{} {}", value.Name, value.Artist);
            println!("*********** {:?} ***********", search_string);
            let search_results_res = client.search.get(&search_string).await;
            
            let search_results = search_results_res.unwrap();

            // check search result is not empty
            if search_results.data.is_empty()
            {
                println!("Search didn't provide any result... Sorry!");
                return;
            }
            
            /*let search_results = match search_results_res {
                Ok(search) => search,
                Err(_) => continue,
            };*/
        
            // print first result
            let search_result = &search_results.data[0];
            println!("{:?} / {:?}", search_result.title, search_result.artist.name);
            
            // print all results
            /*for search_result in search_results.data.iter()
            {
                println!("{:?} / {:?}", search_result.title, search_result.artist.name);
            }*/        
        }
    }
    else if args.mode == "R"
    {
        println!("----------- RANDOM TRACK! -----------");

        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
        };

        let mut random_track_str : String = "--".to_owned();

        eframe::run_simple_native("Rusty-Tunes Randomnessssss", options, move |ctx, _frame| {

            egui_extras::install_image_loaders(ctx);
                
            let client = Deezer::new();
            //let mut search_result : deezer_rs::search::SearchResult;

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui|
                {
                    let name_label = ui.label("Track: ");
                    ui.text_edit_singleline(&mut random_track_str)
                        .labelled_by(name_label.id);
                });
                if ui.button("Randomize").clicked()
                {
                    // not very optimal random pick...
                    let all_tracks: Vec<_> = itunes_library.Tracks.values().cloned().collect();
                    let random_track = all_tracks.choose(&mut rand::thread_rng()).unwrap().clone();
                    println!("Selected track: {:?} / {:?}", random_track.Name, random_track.Artist);
                    random_track_str = format!("{} / {}", random_track.Name, random_track.Artist);

                    let _ = tokio::spawn(async move {
                        let search_string: String = format!("{} {}", random_track.Name, random_track.Artist);
                        let search_results_res = client.search.get(&search_string).await; 
                        let search_results = search_results_res.unwrap();

                        // check search result is not empty
                        if search_results.data.is_empty()
                        {
                            println!("Search didn't provide any result... Sorry!");
                            return;
                        }

                        let search_result = &search_results.data[0]; // first result
                        println!("{:?}",search_result.link);

                        //let image_bytes = reqwest::blocking::get(&search_result.album.cover_small).unwrap().bytes().unwrap();
                    });
                }
                if ui.button("Open").clicked()
                {
                    //let _ = open::with(&search_result.link, "firefox");
                }
                ui.image("https://e-cdns-images.dzcdn.net/images/cover/2e018122cb56986277102d2041a592c8/250x250-000000-80-0-0.jpg");
            });
        });
    }
    else
    {
        println!("Unknown mode!");
    }

    println!("Goodbye!");
}
