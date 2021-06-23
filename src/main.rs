use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

use std::process::Command;

#[macro_use]
extern crate rocket;

// enpoint that takes a query parameter.
#[get("/name/<query>")]
fn cameras_query(query: String) -> Vec<u8> {
    let output = if cfg!(target_os = "windows") {
        Command::new("search.exe")
            .args(&["--name", query.as_str()])
            .output()
            .expect("failed to execute search.exe")
    } else {
        Command::new("./search")
            .args(&["--name", query.as_str()])
            .output()
            .expect("failed to execute search")
    };

    output.stdout
}

// Endpoint that returns all the data.
#[get("/")]
fn cameras() -> Vec<u8> {
    let output = if cfg!(target_os = "windows") {
        Command::new("search.exe")
            .output()
            .expect("failed to execute search.exe")
    } else {
        Command::new("./search")
            .output()
            .expect("failed to execute search")
    };

    output.stdout
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        //.mount("/name", routes![cameras_query])
        .mount("/", routes![cameras])
}
