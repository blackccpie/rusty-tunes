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

use std::collections::HashMap;

use clap::Parser;

use deezer_rs::Deezer;

#[derive(Debug, Deserialize)]
struct Track {
    #[serde(rename = "Track ID")]
    track_id: i32,
    Name: String,
    Artist: String,
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
    
    if(args.mode == "P")
    {
        println!("----------- PLAYLISTS -----------");
        let playlists = &itunes_library.Playlists;
        for playlist in playlists.iter()
        {
            println!("{:?}", playlist.Name);
        }
    }
    else if(args.mode == "T")
    {
        println!("----------- TRACKS -----------");
        let tracks = &itunes_library.Tracks;

        //println!("{:?}, {:?}", itunes_library.major_version, itunes_library.minor_version);
        //print_hashmap(tracks);

        let client = Deezer::new();

        for (key, value) in tracks
        {
            let search_string: String = format!("{} {}", value.Name, value.Artist);
            println!("*********** {:?} ***********", search_string);
            let search_results_res = client.search.get(&search_string).await;
            
            let search_results = search_results_res.unwrap();
            
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
    else
    {
        println!("Unknown mode!");
    }
}
