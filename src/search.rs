extern crate clap;

use clap::{Arg, App};

// Include the data as a raw str slice.
const DATA: &str = include_str!("cameras-defb.csv");

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

      // Naively loop over DATA as lines, return every match of the query.
      if let Some(query) = matches.value_of("NAME") {
         for line in DATA.lines() {
            if line.contains(query) {
               println!("{}", line);
            }
         }
      }
}