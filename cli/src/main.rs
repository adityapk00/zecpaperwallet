extern crate clap;
extern crate zecpaperlib;

mod version;

use clap::{Arg, App};
use zecpaperlib::paper::*;
use zecpaperlib::pdf;
use std::io;
use std::io::prelude::*;

fn main() { 
    let matches = App::new("zecpaperwaller")
       .version(version::version())
       .about("A command line Zcash Sapling paper wallet generator")
       .arg(Arg::with_name("testnet")
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
        .arg(Arg::with_name("nohd")
                .short("n")
                .long("nohd")
                .help("Don't reuse HD keys. Normally, zecpaperwallet will use the same HD key to derive multiple addresses. This flag will use a new seed for each address"))
        .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .index(1)
                .help("Name of output file."))
        .arg(Arg::with_name("entropy")
                .short("e")
                .long("entropy")
                .takes_value(true)
                .help("Provide additional entropy to the random number generator. Any random string, containing 32-64 characters"))
        .arg(Arg::with_name("vanity_prefix")
                .long("vanity")
                .help("Generate a vanity address with the given prefix")
                .takes_value(true))
        .arg(Arg::with_name("threads")
                .long("threads")
                .help("Number of threads to use for the vanity address generator. Set this to the number of CPUs you have")
                .takes_value(true)
                .default_value("1"))
        .arg(Arg::with_name("t_addresses")
                .short("t")
                .long("taddrs")
                .help("Number of T addresses to generate")
                .takes_value(true)
                .default_value("0")
                .validator(|i:String| match i.parse::<i32>() {
                        Ok(_)   => return Ok(()),
                        Err(_)  => return Err(format!("Number of addresses '{}' is not a number", i))
                }))
        .arg(Arg::with_name("z_addresses")
                .short("z")
                .long("zaddrs")
                .help("Number of Z addresses (Sapling) to generate")
                .takes_value(true)
                .default_value("1")                
                .validator(|i:String| match i.parse::<i32>() {
                        Ok(_)   => return Ok(()),
                        Err(_)  => return Err(format!("Number of addresses '{}' is not a number", i))
                }))
       .get_matches();  

    let is_testnet: bool = matches.is_present("testnet");
    
    let nohd: bool    = matches.is_present("nohd");

    // Get the filename and output format
    let filename = matches.value_of("output");
    let format   = matches.value_of("format").unwrap();

    // Writing to PDF requires a filename
    if format == "pdf" && filename.is_none() {
        eprintln!("Need an output file name when writing to PDF");
        return;
    }

    // Get the filename and output format
    let filename = matches.value_of("output");
    let format   = matches.value_of("format").unwrap();

    // Writing to PDF requires a filename
    if format == "pdf" && filename.is_none() {
        eprintln!("Need an output file name when writing to PDF");
        return;
    }

    // Number of t addresses to generate
    let t_addresses = matches.value_of("t_addresses").unwrap().parse::<u32>().unwrap();    

    // Number of z addresses to generate
    let z_addresses = matches.value_of("z_addresses").unwrap().parse::<u32>().unwrap();    

    let addresses = if !matches.value_of("vanity_prefix").is_none() {
        if z_addresses != 1 {
            eprintln!("Can only generate 1 zaddress in vanity mode. You specified {}", z_addresses);
            return;
        }

        if t_addresses != 0 {
            eprintln!("Can't generate vanity t-addressses yet");
            return;
        }

        let num_threads = matches.value_of("threads").unwrap().parse::<u32>().unwrap();

        let prefix = matches.value_of("vanity_prefix").unwrap().to_string();
        println!("Generating address starting with \"{}\"", prefix);
        let addresses = match generate_vanity_wallet(is_testnet, num_threads, prefix) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        // return
        addresses
    } else {
        // Get user entropy. 
        let mut entropy: Vec<u8> = Vec::new();
        // If the user hasn't specified any, read from the stdin
        if matches.value_of("entropy").is_none() {
            // Read from stdin
            println!("Provide additional entropy for generating random numbers. Type in a string of random characters, press [ENTER] when done");
            let mut buffer = String::new();
            let stdin = io::stdin();
            stdin.lock().read_line(&mut buffer).unwrap();

            entropy.extend_from_slice(buffer.as_bytes());
        } else {
            // Use provided entropy. 
            entropy.extend(matches.value_of("entropy").unwrap().as_bytes());
        }

        print!("Generating {} Sapling addresses and {} Transparent addresses...", z_addresses, t_addresses);
        io::stdout().flush().ok();
        let addresses = generate_wallet(is_testnet, nohd, z_addresses, t_addresses, &entropy); 
        println!("[OK]");

        addresses
    };

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
        match pdf::save_to_pdf(is_testnet, &addresses, filename.unwrap()) {
            Ok(_)   => { println!("[OK]");},
            Err(e)  => {
                eprintln!("[ERROR]");
                eprintln!("{}", e);
            }
        };       
    }    
}