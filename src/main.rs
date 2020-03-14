extern crate image;

use std::env;
use image::{GenericImageView,imageops};

fn main() {
    let args: Vec<String> = env::args().collect();
    let img = image::open(& args[1]).unwrap();
    let (x, y) = img.dimensions();
    //TODO have output dimensions as args
    let dim_x = 80;
    let dim_y =(y /(x/120))/2;

    let img_scaled = img.resize(dim_x, dim_x, imageops::FilterType::Nearest);
    let mut img_grey = imageops::colorops::grayscale(&img_scaled);

    for (x, _, pixel) in img_grey.enumerate_pixels_mut() {
        if pixel[0] > 127 {
            print!("#");
        }
        else {
            print!{" "};
        }
        if x == dim_x-1 {
            print!("\n");
        }
    }
}
