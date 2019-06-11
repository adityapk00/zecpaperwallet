extern crate clap;
extern crate zecpaperlib;

use clap::{Arg, App};
use zecpaperlib::paper::*;
use zecpaperlib::pdf;

fn main() { 
    let matches = App::new("zecpaperwaller")
       .version("1.0")
       .about("A command line Zcash Sapling paper wallet generator")
       .arg(Arg::with_name("testnet")
                .short("t")
                .long("testnet")
                .help("Generate Testnet addresses"))
        .arg(Arg::with_name("format")
                .short("f")
                .long("format")
                .help("What format to generate the output in")
                .takes_value(true)
                .value_name("FORMAT")
                .possible_values(&["png", "pdf", "json"])
                .default_value("json"))
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

    let testnet: bool = matches.is_present("testnet");
    if !testnet {
        eprint!("Mainnet addresses are not supported yet. Please re-run with --testnet\n");
        return;
    }

    let num_addresses = matches.value_of("num_addresses").unwrap().parse::<u32>().unwrap();
    let addresses = generate_wallet(testnet, num_addresses); 
    println!("{}", addresses);
    pdf::save_to_pdf(&addresses, "test_working.pdf");
}