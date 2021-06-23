use std::process::Command;

#[macro_use] extern crate rocket;

// Api has one endpoint
#[get("/<query>")]
fn index(query: String) -> Vec<u8> {
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
