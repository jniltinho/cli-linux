extern crate clap;
use clap::{App, Arg};

mod net;
mod utils;

fn main() {
    let matches = App::new("cli-linux")
        .version("0.0.1")
        .author("Nilton Oliveira <jniltinho@gmail.com>")
        .about("Rust CLI Linux Tools")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Who to say hello to")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("distro")
                .short("d")
                .long("distro")
                .help("Get Linux Distro Info"),
        )
        .arg(
            Arg::with_name("net")
                .long("get-ip")
                .help("Get IP and Interfaces on Linux")
                .default_value("all"),
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if matches.is_present("distro") {
        utils::get_sudo();
        let dist = utils::get_distro();
        println!("Distro: {}", dist);
    }

    if matches.occurrences_of("net") != 0 {
        let net = matches.value_of("net").unwrap();
        if net == "all" {
            net::get_interfaces();
        } else {
            net::get_ip_net(net);
        }
    }

    if let Some(c) = matches.value_of("name") {
        println!("Hello, {}!, {}", c, utils::get_date());
    }
}
