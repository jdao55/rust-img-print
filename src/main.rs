use image::{imageops, GenericImageView};
use img_print::arg_parse;
use img_print::print_util;

fn main() {
    let mut args = arg_parse::ImageArgs::new();

    let img = image::open(&args.filename).unwrap();

    let (x, y) = img.dimensions();

    args.update_height(x, y);

    let img_scaled = img.resize_exact(
        args.width,
        args.height.unwrap(),
        imageops::FilterType::CatmullRom,
    );
    let img_buffer = img_scaled.to_rgba();
    if args.ascii {
        img_buffer.enumerate_rows().for_each(|(_, row)| {
            row.for_each(|(_, _, pixel)| {
                print!("{}", print_util::char_from_density(pixel));
            });
            print!("\n");
        });
    } else {
        if args.greyscale {
            img_buffer.enumerate_rows().for_each(|(_, row)| {
                row.for_each(|(_, _, pixel)| {
                    print_util::print_char_gs(pixel, &args.output_char);
                });
                print!("\n");
            });
        } else {
            img_buffer.enumerate_rows().for_each(|(_, row)| {
                row.for_each(|(_, _, pixel)| {
                    print_util::print_char_rgb(pixel, &args.output_char);
                });
                print!("\n");
            });
        }
    }
}
