use clap::{App, Arg};
use image::imageops::FilterType;
pub struct ImageArgs {
    pub filename: String,
    pub width: u32,
    pub height: Option<u32>,
    pub greyscale: bool,
    pub output_char: String,
    pub filtertype: FilterType,
}
impl ImageArgs {
    pub fn new() -> ImageArgs {
        let matches = App::new("img-print")
            .version("1.0")
            .author("Joseph Dao <joedao55@gmail.com>")
            .about("")
            .arg(
                Arg::with_name("input")
                    .help("input image file name")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("width")
                    .help("output width(characters)")
                    .required(false)
                    .index(2),
            )
            .arg(
                Arg::with_name("height")
                    .help("output width(characters)")
                    .required(false)
                    .index(3),
            )
            .arg(
                Arg::with_name("output_char")
                    .help("output character")
                    .required(false)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("greyscale")
                    .short("g")
                    .long("greyscale")
                    .help("turn on greyscale output"),
            )
            .arg(
                Arg::with_name("filtertype")
                    .short("f")
                    .required(false)
                    .long("filtertype")
                    .help("select scaling algorithm")
                    .takes_value(true),
            )
            .get_matches();
        let filename = matches.value_of("input").unwrap();

        let width = match matches.value_of("width") {
            Some(x) => x.parse::<u32>().unwrap_or(50),
            None => 50,
        };
        let height: Option<u32> = match matches.value_of("height") {
            Some(x) => Some(x.parse::<u32>().unwrap()),
            None => None,
        };
        let greyscale = matches.is_present("greyscale");
        let output_char = matches.value_of("output_char").unwrap_or("▇");
        let filtertype = match matches.value_of("filtertype") {
            Some("Nearest") => FilterType::Nearest,
            Some("Triangle") => FilterType::Triangle,
            Some("CatmullRom") => FilterType::CatmullRom,
            Some("Gaussian") => FilterType::Gaussian,
            Some("Lanczos3") => FilterType::Lanczos3,
            _ => FilterType::Lanczos3,
        };
        ImageArgs {
            filename: String::from(filename),
            width,
            height,
            greyscale,
            output_char: String::from(output_char),
            filtertype,
        }
    }

    pub fn update_height(&mut self, x: u32, y: u32) {
        self.height = match self.height {
            Some(val) => Some(val),
            None => Some(((f64::from(self.width) * f64::from(y) / f64::from(x)) / 2.15) as u32),
        };
    }
}
