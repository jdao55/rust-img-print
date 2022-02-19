use image::Rgba;

pub fn print_char_rgb(pixel: &Rgba<u8>, c: &str) {
    if pixel[3] > 30 {
        print!("\x1b[38;2;{};{};{}m{}", pixel[0], pixel[1], pixel[2], c);
    } else {
        print!(" ");
    }
}

pub fn print_char_gs(pixel: &Rgba<u8>, c: &str) {
    if pixel[3] > 30 {
        let avg_color = (u32::from(pixel[0]) + u32::from(pixel[1]) + u32::from(pixel[2])) / 3;
        print!("\x1b[38;2;{};{};{}m{}", avg_color, avg_color, avg_color, c);
    } else {
        print!(" ");
    }
}

pub fn char_from_density(pixel: &Rgba<u8>) -> char {
    let avg_color = (u32::from(pixel[0]) + u32::from(pixel[1]) + u32::from(pixel[2])) / 3;
    let char_density_map: [char; 32] = [
        '@', '%', '&', 'M', 'a', 'k', 'd', 'q', 'm', 'O', 'Q', 'C', 'U', 'X', 'c', 'u', 'x', 'j',
        't', '\\', '(', '1', '}', ']', '-', '+', '<', 'i', ';', ',', '.', ' ',
    ];
    let index: usize =
        (((avg_color as f32) / 255.00) * (char_density_map.len() as f32 - 1.0)) as usize;
    char_density_map[index]
}
