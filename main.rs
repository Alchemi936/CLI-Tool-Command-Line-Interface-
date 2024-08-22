#![deny(clippy::all)]

use clap::{Arg, Command};

fn main() {
    // Define the CLI tool
    let matches = Command::new("Simple CLI Tool")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("Does awesome things")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .takes_value(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Turn on verbose mode"),
        )
        .get_matches();

  

    // Check if verbose mode is enabled
    if matches.is_present("verbose") {
        println!("Verbose mode is on");
    } else {
        println!("Verbose mode is off");
    }
}
