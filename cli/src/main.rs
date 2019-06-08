extern crate clap; 
use clap::{Arg, App, SubCommand};

fn main() { 
    App::new("zecpaperwaller")
       .version("1.0")
       .about("A command line Zcash Sapling paper wallet generator")
       .arg(Arg::with_name("testnet")
                .short("t")
                .long("testnet")
                .help("Generate Testnet addresses."))
        .arg(Arg::with_name("format")
                .short("f")
                .long("format")
                .help("What format to generate the output in.")
                .takes_value(true)
                .value_name("FORMAT")
                .possible_values(&["png", "pdf", "txt"]))
        .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .index(1)
                .required(true)
                .help("Name of output file."))
       .get_matches(); 
}