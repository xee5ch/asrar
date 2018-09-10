extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Asrar")
        .version("0.1.0")
        .author("Al S <xee5ch.gh.al@il5.in>")
        .about("pwgen clone written in Rust")
        .arg(Arg::with_name("Count")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("Number of chars to generate"))
        .get_matches();
    let count = matches.value_of("Count").unwrap();
    println!("{}", count);
}