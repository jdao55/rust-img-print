extern crate image;
use image::Rgba;

pub fn print_char_rgb(pixel:& Rgba<u8>, c: char) {
    if pixel[3] > 30 {
        print!("\x1b[38;2;{};{};{}m{}", pixel[0], pixel[1], pixel[2], c);
    } else {
        print!(" ");
    }
}

pub fn print_char_gs(pixel:& Rgba<u8>, c: char) {
    if pixel[3] > 30 {
        let avg_color = (u32::from(pixel[0]) + u32::from(pixel[1]) + u32::from(pixel[2])) / 3;
        print!("\x1b[38;2;{};{};{}m{}", avg_color, avg_color, avg_color, c);
    } else {
        print!(" ");
    }

}
