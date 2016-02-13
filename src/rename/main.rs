extern crate liquery;
extern crate clap;
extern crate walkdir;

use std::env;
use walkdir::WalkDir;
use clap::{App, Arg};

fn main() {
    let app = App::new("lirename")
                  .version(env!("CARGO_PKG_VERSION"))
                  .about("Renames based on liquery")
                  .arg(Arg::with_name("query")
                           .help("Query to run")
                           .required(true)
                           .takes_value(true))
                  .arg(Arg::with_name("path")
                           .help("Path to be iterated over")
                           .required(true)
                           .takes_value(true))
                  .get_matches();

    let path = app.value_of("path").unwrap();
    let query = app.value_of("query").unwrap();

    println!("path:{}, query:{}", path, query);

    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(e) => println!("{:?}", e),
        }
    }
}
