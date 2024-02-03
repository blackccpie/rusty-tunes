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

use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
  pub id: u64,
  pub name: String,
  pub link: Option<String>,
  pub share: Option<String>,
  pub picture: Option<String>,
  pub picture_small: Option<String>,
  pub picture_medium: Option<String>,
  pub picture_big: Option<String>,
  pub picture_xl: Option<String>,
  pub nb_album: Option<u32>,
  pub nb_fan: Option<u32>,
  pub radio: Option<bool>,
  pub tracklist: String,
  pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
  pub id: u64,
  pub title: String,
  pub upc: Option<String>,
  pub link: Option<String>,
  pub share: Option<String>,
  pub cover: String,
  pub cover_small: String,
  pub cover_medium: String,
  pub cover_big: String,
  pub cover_xl: String,
  pub genre_id: Option<i64>,
  pub label: Option<String>,
  pub nb_tracks: Option<u32>,
  pub duration: Option<u32>,
  pub fans: Option<u32>,
  pub rating: Option<u32>,
  pub release_date: Option<String>,
  pub record_type: Option<String>,
  pub available: Option<bool>,
  pub tracklist: String,
  pub explicit_lyrics: Option<bool>,
  pub explicit_content_lyrics: Option<u32>,
  pub explicit_content_cover: Option<u32>,
  pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
  pub id: u64,
  pub readable: bool,
  pub title: String,
  pub title_short: String,
  pub title_version: Option<String>,
  pub link: String,
  pub duration: u64,
  pub rank: u64,
  pub explicit_lyrics: bool,
  pub preview: String,
  pub artist: Artist,
  pub album: Album,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResults {
  pub data: Vec<SearchResult>,
}

pub struct SearchService {
  client: Client,
}

impl SearchService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn get(&self, q: &str) -> Result<SearchResults, reqwest::Error> {
    let res: SearchResults = self
      .client
      .get(format!("https://cors-proxy.fringe.zone/https://api.deezer.com/search?q={}", q))
      .send()
      .await?
      .json()
      .await?;
    Ok(res)
  }
}

pub struct Deezer {
    pub search: SearchService,
}

const BASE_URL: &str = "https://cors-anywhere.herokuapp.com/https://api.deezer.com/";

impl Deezer {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            //.base_url(Url::parse(BASE_URL).unwrap())
            //.timeout(Duration::from_secs(5))
            .build().unwrap();
        Self {
            search: SearchService::new(&client),
        }
    }
}