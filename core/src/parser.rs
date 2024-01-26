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

// Could this be avoided using struct macros?
#![allow(non_snake_case)]

use std::collections::HashMap;
use std::path::PathBuf;

fn print_hashmap(hashmap: &HashMap<String, Track>) {
    for (key, value) in hashmap {
        println!("Key: {:?}, Value: {:?}", key, value.Name);
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Track {
    #[serde(rename = "Track ID")]
    track_id: i32,
    pub Name: String,
    #[serde(default = "default_artist")]
    pub Artist: String, // some tracks are missing artist...
}

fn default_artist() -> String {
    "Unknown".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Playlist {
    pub Name: String,
    #[serde(rename = "Playlist ID")]
    playlist_id: i32,
    #[serde(default)]
    Tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct ApplePlist {
    #[serde(rename = "Major Version")]
    major_version: i32,
    #[serde(rename = "Minor Version")]
    minor_version: i32,
    pub Tracks: HashMap<String, Track>,
    pub Playlists: Vec<Playlist>,
}

pub fn parse_xml_plist(file_path: &PathBuf) -> ApplePlist {
    let itunes_library: ApplePlist = plist::from_file(file_path).unwrap();
    itunes_library
}

pub fn parse_xmlreader_plist<R: std::io::Read + Clone>(file_reader: &R) -> ApplePlist {
    let itunes_library: ApplePlist = plist::from_reader_xml(file_reader.clone()).unwrap();
    itunes_library
}

pub fn parse_xmlbytes_plist(file_bytes: &[u8]) -> ApplePlist {
    let itunes_library: ApplePlist = plist::from_bytes(file_bytes).unwrap();
    itunes_library
}
