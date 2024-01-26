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

#[macro_use] extern crate rocket;

extern crate core;
use core::common::XmlPlist;

use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;
use rocket::serde::json::Json;
use std::fs;

#[get("/xml/<path..>")]
fn get_xmlplist(path: PathBuf) -> Json<XmlPlist> {
    let path = PathBuf::from("data/xml/").join(path);
    match fs::read_to_string(path) {
        Ok(xmlplist) => Json(XmlPlist {
            xml_plist: String::from(xmlplist),
        }),
        Err(e) => Json(XmlPlist {
            xml_plist: String::from(format!("Error: {}", e)),
        }),
    }
}

// return the index file as a Rocket NamedFile
async fn get_index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("../frontend/dist/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

// create a route for any url that is a path from the /
#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("../frontend/dist").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}

#[get("/data/<path..>")]
async fn data(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("./data/").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}

// return the index when the url is /
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[launch]
fn rocket() -> _ {
    // you must mount the static_files route
    rocket::build()
        .mount("/", routes![index, static_files, data])
        .mount("/api", routes![get_xmlplist])
}