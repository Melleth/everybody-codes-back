extern crate clap;
extern crate csv;


use clap::{Arg, App};
use serde::Deserialize;

// Include the data as a raw str slice.
//    - could also be read from path with another CLI arg, but
//      I consider it beyond the scope of this assignment.
const DATA: &str = include_str!("cameras-defb.csv");

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Camera {
   Camera: String,
   Latitude: String,
   Longitude: String,
}

// Deserializes CSV into structs
//    - preferably would make this some const thing so we wouldn't be doing
//      this every time the program is executed. e.g. by
//      creating a build script that deserializes the csv into structs and
//      then serializes those structs to some binary data that we include here.
fn create_records() -> Vec<Camera> {
   let mut out: Vec<Camera> = vec![];

   // Deserialize csv data into Record struct to sanitize.
   let mut reader = csv::ReaderBuilder::new()
      .delimiter(b';').from_reader(DATA.as_bytes());

   for result in reader.deserialize() {
      match result {
         Ok(record) => { 
               out.push(record);
         },
         // We may want to check stderr in the future to see if there are invalid records.
         //    - Right now, we don't really care so just do nothing.
         Err(_e) => {
            //eprintln!("{}", e)
            ()
         },
      }
   }
   out

}

fn main() {
   // Defines the CLI
   let matches = App::new("search")
      .version("1.0")
      .author("Siemen")
      .about("Searches security cameras by name from csv")
      .arg(Arg::with_name("NAME")
         .short("n")
         .long("name")
         .help("Sets the search query")
         .takes_value(true)
         .required(true))
      .get_matches();

      let records: Vec<Camera> = create_records();

      // If we provide some --name / -n to the CLI, print the hits.
      if let Some(query) = matches.value_of("NAME") {
         // Filter cameras
         let result: Vec<Camera> = records.into_iter()
            .filter(|record| record.Camera.contains(query))
            .collect(); 

         // Format output
         for camera in result {
            let id = &camera.Camera[7..10];
            let name = &camera.Camera;
            let lat = &camera.Latitude;
            let lon = &camera.Longitude;
            println!("{} | {} | {} | {}", id, name, lat, lon)
         }
      }
}