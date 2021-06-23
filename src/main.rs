use std::process::Command;

#[macro_use]
extern crate rocket;

// enpoint that takes a query parameter.
#[get("/cameras/<query>")]
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
#[get("/cameras")]
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![cameras_query])
        .mount("/", routes![cameras])
}
