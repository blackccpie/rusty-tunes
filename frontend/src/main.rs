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

extern crate core;
use core::parser::Track;

mod fetchstates; // TODO : change modules architectures accordingly to rust book...

mod xmlplist;
use crate::xmlplist::XmlPlist;

use rand::seq::SliceRandom;

use std::path::PathBuf;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {

    let itunes_tracks = use_state(|| None::<Vec<Track>>);
    let track_name = use_state(|| "Uninitialized".to_string());

    // TODO : little code cleanup...
    let onload = {
        let itunes_tracks = itunes_tracks.clone();
        Callback::from(move |_| {
            use web_sys::console;
            let file_uri : String = "/api/xml/itunes_library_redux.xml".to_string();
            wasm_bindgen_futures::spawn_local({
                let itunes_tracks = itunes_tracks.clone();
                async move {
                    let body = reqwasm::http::Request::get(file_uri.as_str())
                        .send().await.unwrap().json::<std::collections::HashMap<String,String>>().await.unwrap();
                    console::log_1(&body["xml_plist"].clone().into());
                    let library = core::parser::parse_xmlbytes_plist(&body["xml_plist"].clone().into_bytes());
                    let tracks : Vec<Track> = library.Tracks.values().cloned().collect::<Vec<Track>>();
                    itunes_tracks.set( Some(tracks) );
                }
            });
            let message = String::from("xml plist loaded!");
            console::log_1(&message.into());
        })
    };

    let onrand = {
        let itunes_tracks = itunes_tracks.clone();
        let track_name = track_name.clone();
        Callback::from(move |_| {
            let rand_track = itunes_tracks.as_ref().unwrap().choose(&mut rand::thread_rng()).unwrap().clone();
            use web_sys::console;
            track_name.set(rand_track.Name.clone());
            console::log_1(&rand_track.Name.into());
        })
    };

    html! {
        <main>
        <h1>{ "Hello World" }</h1>
        <button onclick={onload} class="button button-primary">{"Load"}</button>
        <button onclick={onrand} class="button button-primary">{"Randomize"}</button>
        <p>
            <b>{ "Current track: " }</b>
            { (*track_name).clone() }
        </p>
        //<div class="container-sm justify-content-center m-5">
        //    <XmlPlist id={"itunes_library_redux.xml"}/>
        //</div>
        //<iframe title="deezer-widget" src="https://widget.deezer.com/widget/dark/playlist/1479458365" width="100%" height="300" frameborder="0" allowtransparency="true" allow="encrypted-media; clipboard-write"></iframe>
        </main>
    }
}

// entry point for starting the Yew application
pub fn main() {
    // create the logger
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    // start the Yew framework
    yew::Renderer::<App>::new().render();
}