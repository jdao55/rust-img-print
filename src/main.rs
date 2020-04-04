//extern crate image;
extern crate clap;

use clap::{App, Arg};
//use std::env;
//use image::{GenericImageView,imageops};

fn main() {
    let matches = App::new("img-print")
        .version("1.0")
        .author("Joseph Dao <joedao55@gmail.com>")
        .about("")
        .arg(
            Arg::with_name("input")
                .help("input image file name")
                .required(true)
                .index(1))
        .arg(
            Arg::with_name("width")
                .help("output width(characters)")
                .required(false)
                .index(2))
        .arg(
            Arg::with_name("height")
                .help("output width(characters)")
                .required(false)
                .index(3))
        .arg(
            Arg::with_name("greyscale")
                .short("g")
                .long("greyscale")
                .help("turn on greyscale output")
        )
        .get_matches();
    let name = matches.value_of("input").unwrap();
    println!("filename is: {}", name);
    // let args: Vec<String> = env::args().collect();
    // let img = image::open(& args[1]).unwrap();
    // let (x, y) = img.dimensions();
    // //TODO have output dimensions as args
    // let dim_x = 80;
    // let dim_y =(y /(x/120))/2;

    // let img_scaled = img.resize(dim_x, dim_x, imageops::FilterType::Nearest);
    // let mut img_grey = imageops::colorops::grayscale(&img_scaled);

    // for (x, _, pixel) in img_grey.enumerate_pixels_mut() {
    //     if pixel[0] > 127 {
    //         print!("#");
    //     }
    //     else {
    //         print!{" "};
    //     }
    //     if x == dim_x-1 {
    //         print!("\n");
    //     }
    // }
}
