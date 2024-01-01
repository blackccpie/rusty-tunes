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

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {

    let onclick = {
        //let counter = app_ctx.clone();
        Callback::from(move |_| {})
    };

    html! {
        <main>
        <h1>{ "Hello World" }</h1>
        <button {onclick} class="button button-primary">{"Randomize"}</button>
        <iframe title="deezer-widget" src="https://widget.deezer.com/widget/dark/playlist/1479458365" width="100%" height="300" frameborder="0" allowtransparency="true" allow="encrypted-media; clipboard-write"></iframe>
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