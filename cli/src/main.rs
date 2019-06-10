extern crate clap; 
extern crate zecpaperlib;

use clap::{Arg, App};
use zecpaperlib::paper::get_address;
use json::object;

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
                .help("Name of output file."))
       .get_matches();

    let (addr, pk) = get_address();
    let ans = object!{
        "address"       => addr,
        "private_key"   => pk
    }; 

    println!("{}", json::stringify_pretty(ans, 2));
}