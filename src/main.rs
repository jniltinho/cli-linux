extern crate clap;

use clap::{Arg, App};

fn main() {
    // basic app information
    let app = App::new("cli-linux")
        .version("1.0")
        .about("Says hello")
        .author("Nilton Oliveira");

    // Define the name command line option
    let name_option = Arg::with_name("name")
        .short("n")
        .long("name") // allow --name
        .takes_value(true)
        .help("Who to say hello to")
        .required(true);

    // now add in the argument we want to parse
    let app = app.arg(name_option);

    // extract the matches
    let matches = app.get_matches();

    // Extract the actual name
    let name = matches.value_of("name")
        .expect("This can't be None, we said it was required");

    println!("Hello, {}!", name);
}
