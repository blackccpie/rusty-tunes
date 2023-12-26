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

use deezer_rs::Deezer;

pub struct Wrapper {
    client: Deezer
}

impl Wrapper {
    pub fn new() -> Self {
        Self { client: Deezer::new() }
    }
    pub async fn search(&mut self, artist: &String, title: &String) -> (String, String) {

        let search_string: String = format!("{} {}", title, artist);
        
        //println!("*********** {:?} ***********", search_string);
        
        let search_results_res = self.client.search.get(&search_string).await;
        let search_results = search_results_res.unwrap();

        // check search result is not empty
        if search_results.data.is_empty()
        {
            println!("Search didn't provide any result... Sorry!");
            return ("".to_owned(),"".to_owned());
        }

        /*let search_results = match search_results_res {
            Ok(search) => search,
            Err(_) => continue,
        };*/
        
        // print all results
        /*for search_result in search_results.data.iter()
        {
            println!("{:?} / {:?}", search_result.title, search_result.artist.name);
        }*/    
    
        // get first result
        let search_result = &search_results.data[0];

        (search_result.artist.name.to_owned(), search_result.title.to_owned())
    }
}