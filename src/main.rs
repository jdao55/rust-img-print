//extern crate image;
//extern crate clap;

use clap::{App, Arg};
use image::{GenericImageView, imageops};
use img_print::print_util;

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
            Arg::with_name("output_char")
                .help("output character")
                .required(false)
                .index(4))
        .arg(
            Arg::with_name("greyscale")
                .short("g")
                .long("greyscale")
                .help("turn on greyscale output")
        )
        .get_matches();
    let name = matches.value_of("input").unwrap();


    let img = image::open(&name).unwrap();

    let (x, y) = img.dimensions();


    let dim_x = match matches.value_of("width")
    {
        Some(x) => x.parse::<u32>().unwrap_or(50),
        None => 40,
    };
    let dim_y = match matches.value_of("height")
    {
        Some(x) => x.parse::<u32>().unwrap(),
        None => (( f64::from(dim_x)*f64::from(y)/ f64::from(x))/2.0) as u32
    };

    let output_char = matches.value_of("output_char").unwrap_or("▇");

    let img_scaled = img.resize_exact(dim_x, dim_y, imageops::FilterType::CatmullRom);
    let img_buffer = img_scaled.to_rgba();
    if matches.is_present("greyscale")
    {
        img_buffer.enumerate_rows().for_each(
            |(_, row)|{
                row.for_each(|(_, _,pixel)| {print_util::print_char_gs(pixel, output_char);});
                print!("\n");
            });
    }
    else
    {
        img_buffer.enumerate_rows().for_each(
            |(_, row)|{
                row.for_each(|(_, _,pixel)| {print_util::print_char_rgb(pixel, output_char);});
                print!("\n");
            });
    }
}
