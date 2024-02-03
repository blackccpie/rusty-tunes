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

use reqwasm::http::Request;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::vnode::VNode;

extern crate core;

use crate::fetchstates::{FetchError, FetchState, FetchStateMsg};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

pub struct XmlPlist {
    xmlplist: FetchState<core::common::XmlPlist>,
}

impl Component for XmlPlist {
    type Message = FetchStateMsg<core::common::XmlPlist>;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            xmlplist: FetchState::NotFetching,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        ctx.link().send_message(FetchStateMsg::GetData);
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        let uri: String = format!("/api/xmlplist/{}", _ctx.props().id);

        match _msg {
            FetchStateMsg::SetDataFetchState(state) => {
                self.xmlplist = state;
                true
            }
            FetchStateMsg::GetData => {
                _ctx.link().send_future(async move {
                    match Request::get(uri.as_str()).send().await {
                        Ok(xmlplist) => match xmlplist.json().await {
                            Ok(xmlplist) => {
                                FetchStateMsg::SetDataFetchState(FetchState::Success(xmlplist))
                            }
                            Err(err) => {
                                FetchStateMsg::SetDataFetchState(FetchState::Failed(FetchError {
                                    err: err.to_string(),
                                }))
                            }
                        },
                        Err(err) => {
                            FetchStateMsg::SetDataFetchState(FetchState::Failed(FetchError {
                                err: err.to_string(),
                            }))
                        }
                    }
                });
                _ctx.link()
                    .send_message(FetchStateMsg::SetDataFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if matches!(&self.xmlplist, &FetchState::NotFetching) {
            _ctx.link().send_message(FetchStateMsg::GetData);
        }
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&self.render_xmlplist());
        let node = Node::from(div);
        let vnode = VNode::VRef(node);
        vnode
    }
}

impl XmlPlist {
    fn render_xmlplist(&self) -> String {
        let Self { xmlplist } = self;
        //let mut options = Options::empty();
        //options.insert(Options::ENABLE_STRIKETHROUGH); // TODO : NEEDED??

        match xmlplist {
            FetchState::Success(xmlplist) => {
                let xp = xmlplist.xml_plist.clone();
                xp
            }
            _ => "Failed to Load ...".to_string(),
        }
    }
}