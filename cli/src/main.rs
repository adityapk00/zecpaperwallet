extern crate clap;
extern crate zecpaperlib;
extern crate lodepng;
extern crate printpdf;
extern crate image;
extern crate qrcode;

use qrcode::QrCode;
use qrcode::types::Color;
use std::io::BufWriter;
use std::convert::From;
use std::fs::File;
use clap::{Arg, App};
use zecpaperlib::paper::*;
use printpdf::*;

// imports the `image` library with the exact version that we are using
//use printpdf::*;

fn get_qrcode_scaled(data: &str) -> (Vec<u8>, usize) {
    let code = QrCode::new(data.as_bytes()).unwrap();
    let output_size = code.width();
    println!("Total QR Code Modules: {}", output_size);

    let imgdata = code.to_colors();

    // Scale it to 10x
    let scalefactor = 10;
    let padding     = 10;
    let scaledsize  = output_size * scalefactor;
    let finalsize   = scaledsize + (2 * padding);

    // Build a scaled image
    let scaledimg: Vec<u8> = (0..(finalsize*finalsize)).flat_map( |i| {
        let x = i / finalsize;
        let y = i % finalsize;
        if x < padding || y < padding || x >= (padding+scaledsize) || y >= (padding+scaledsize) {
            vec![255u8; 3]
        } else {
            if imgdata[(x - padding)/scalefactor * output_size + (y - padding)/scalefactor] != Color::Light {vec![0u8; 3] } else { vec![255u8; 3] }
        }
    }).collect();

    return (scaledimg, finalsize);
}

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
        eprint!("Mainnet addresses are not supported yet. Please re-run with --testnet");
        return;
    }

    let num_addresses = matches.value_of("num_addresses").unwrap().parse::<i32>().unwrap();
    let addresses = gen_addresses_as_json(testnet, num_addresses); 
    println!("{}", addresses);


//     let path = &Path::new("write_test.png");

//     // Use number of QR modules to specify the image dimensions.
//     if let Err(e) = lodepng::encode_file(path, &scaledimg, finalsize, finalsize, ColorType::RGB, 8) {
//         panic!("failed to write png: {:?}", e);
//     }

    let (doc, page1, layer1) = PdfDocument::new("Zec Sapling Paper Wallet", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font  = doc.add_builtin_font(BuiltinFont::Courier).unwrap();
    let font_bold = doc.add_builtin_font(BuiltinFont::CourierBold).unwrap();

    let keys = json::parse(&addresses).unwrap();
    for kv in keys.members() {
        add_address_to_page(&current_layer, &font, &font_bold, kv["address"].as_str().unwrap(), 0);

        add_pk_to_page(&current_layer, &font, &font_bold, kv["private_key"].as_str().unwrap(), 1);

        // Is the shape stroked? Is the shape closed? Is the shape filled?
        let line1 = Line {
            points: vec![(Point::new(Mm(5.0), Mm(160.0)), false), (Point::new(Mm(205.0), Mm(160.0)), false)],
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        let outline_color = printpdf::Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));

        current_layer.set_outline_color(outline_color);
        current_layer.set_outline_thickness(2.0);

        // Draw first line
        current_layer.add_shape(line1);
    };
    
    doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
}

fn add_address_to_page(current_layer: &PdfLayerReference, font: &IndirectFontRef, font_bold: &IndirectFontRef,address: &str, pos: u32) {
    let (scaledimg, finalsize) = get_qrcode_scaled(address);

    let ypos = 297.0 - 5.0 - (50.0 * ((pos+1) as f64));
    add_qrcode_image_to_page(current_layer, scaledimg, finalsize, Mm(10.0), Mm(ypos));

    current_layer.use_text("Address", 14, Mm(55.0), Mm(ypos+27.5), &font_bold);
    let strs = split_to_max(&address, 44);
    for i in 0..strs.len() {
        current_layer.use_text(strs[i].clone(), 12, Mm(55.0), Mm(ypos+15.0-((i*5) as f64)), &font);
    }
}

fn add_pk_to_page(current_layer: &PdfLayerReference, font: &IndirectFontRef, font_bold: &IndirectFontRef, pk: &str, pos: u32) {
    let (scaledimg, finalsize) = get_qrcode_scaled(pk);

    let ypos = 297.0 - 5.0 - (50.0 * ((pos+1) as f64));
    add_qrcode_image_to_page(current_layer, scaledimg, finalsize, Mm(145.0), Mm(ypos-17.5));

    current_layer.use_text("Private Key", 14, Mm(10.0), Mm(ypos+27.5), &font_bold);
    let strs = split_to_max(&pk, 51);
    for i in 0..strs.len() {
        current_layer.use_text(strs[i].clone(), 12, Mm(10.0), Mm(ypos+15.0-((i*5) as f64)), &font);
    }
}

fn add_qrcode_image_to_page(current_layer: &PdfLayerReference, qr: Vec<u8>, qrsize: usize, x: Mm, y: Mm) {
    // you can also construct images manually from your data:
    let image_file_2 = ImageXObject {
            width: Px(qrsize),
            height: Px(qrsize),
            color_space: ColorSpace::Rgb,
            bits_per_component: ColorBits::Bit8,
            interpolate: true,
            /* put your bytes here. Make sure the total number of bytes =
            width * height * (bytes per component * number of components)
            (e.g. 2 (bytes) x 3 (colors) for RGB 16bit) */
            image_data: qr,
            image_filter: None, /* does not work yet */
            clipping_bbox: None, /* doesn't work either, untested */
    };
    
    let image2 = Image::from(image_file_2);
    image2.add_to_layer(current_layer.clone(), Some(x), Some(y), None, None, None, None);
}


fn split_to_max(s: &str, max: usize) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();

    for i in 0..((s.len() / max)+1) {
        let start = i * max;
        let end   = if start + max > s.len() { s.len() } else { start + max };
        ans.push(s[start..end].to_string());
    }

    return ans;
}