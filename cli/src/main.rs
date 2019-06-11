extern crate clap;
extern crate zecpaperlib;

use clap::{Arg, App};
use zecpaperlib::paper::*;
use zecpaperlib::pdf;
use std::io;
use std::io::prelude::*;

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
                .possible_values(&["pdf", "json"])
                .default_value("json"))
        .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .index(1)
                .help("Name of output file."))
        .arg(Arg::with_name("z_addresses")
                .short("z")
                .long("z_addresses")
                .help("Number of Z addresses (sapling) to generate")
                .takes_value(true)
                .default_value("1")                
                .validator(|i:String| match i.parse::<i32>() {
                        Ok(_)   => return Ok(()),
                        Err(_)  => return Err(format!("Number of addresses '{}' is not a number", i))
                }))
       .get_matches();  

    let testnet: bool = matches.is_present("testnet");
    if !testnet {
        eprintln!("Mainnet addresses are not supported yet. Please re-run with --testnet");
        return;
    }

    let filename = matches.value_of("output");
    let format   = matches.value_of("format").unwrap();

    // Writing to PDF requires a filename
    if format == "pdf" && filename.is_none() {
        eprintln!("Need an output file name when writing to PDF");
        return;
    }

    let num_addresses = matches.value_of("z_addresses").unwrap().parse::<u32>().unwrap();

    print!("Generating {} Sapling addresses.........", num_addresses);
    io::stdout().flush().ok();
    let addresses = generate_wallet(testnet, num_addresses); 
    println!("[OK]");
    
    // If the default format is present, write to the console if the filename is absent
    if format == "json" {
        if filename.is_none() {
            println!("{}", addresses);
        } else {
            std::fs::write(filename.unwrap(), addresses).expect("Couldn't write to file!");
            println!("Wrote {:?} as a plaintext file", filename);
        }
    } else if format == "pdf" {
        // We already know the output file name was specified
        print!("Writing {:?} as a PDF file...", filename.unwrap());
        io::stdout().flush().ok();
        pdf::save_to_pdf(&addresses, filename.unwrap());
        println!("[OK]");
    }    
}