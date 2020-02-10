extern crate image;

use std::env;
use image::ImageFormat;
use image::{GenericImageView,imageops};
use std::fs::File;


fn main() {
    let args: Vec<String> = env::args().collect();
    let img = image::open(& args[1]).unwrap();
    let (x, y) = img.dimensions();
    let img_scaled = img.resize(x/2, y/2, imageops::FilterType::Lanczos3);
    let mut output= File::create("test2.png").unwrap();
    img_scaled.write_to(&mut output, ImageFormat::Png).unwrap();


}
