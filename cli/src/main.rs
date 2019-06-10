extern crate clap;
extern crate zecpaperlib;

use clap::{Arg, App};
use zecpaperlib::paper::get_address;
use json::{array, object};

fn main() { 
    let matches = App::new("zecpaperwaller")
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
        .arg(Arg::with_name("num_addresses")
                .short("n")
                .long("num_addresses")
                .help("Number of addresses to generate")
                .takes_value(true)
                .default_value("1")                
                .validator(|i:String| match i.parse::<i32>() {
                        Ok(_)   => return Ok(()),
                        Err(_)  => return Err(format!("Number of addresses '{}' is not a number", i))
                }))
       .get_matches();  

    let num_addresses = matches.value_of("num_addresses").unwrap().parse::<i32>().unwrap();
    let mut ans = array![];

    for count in 0..num_addresses {
        let (addr, pk) = get_address(true);
        ans.push(object!{
                "num"           => count,
                "address"       => addr,
                "private_key"   => pk
        }).unwrap(); 
    }      

    
    println!("{}", json::stringify_pretty(ans, 2)); 
}