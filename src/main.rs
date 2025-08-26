
use image;
use image::{GenericImageView};
use image::imageops::FilterType;

struct AsciiArt
{
    image: image::DynamicImage,
    width: u32
}

impl AsciiArt
{
    fn new(input_image: image::DynamicImage, out_width: u32) -> Self
    {
        return Self
        {
            image: input_image,
            width: out_width
        };
    }

    fn convert(&self)
    {
        let original_width = self.image.width() as f32;
        let original_height = self.image.height() as f32;
        let aspect_ratio: f32 = 0.5;

        let ascii = b"@%#*+=-:. ";


        let heigth: u32 = ((original_height * self.width as f32 * aspect_ratio) / original_width) as u32;

        let img = self.image.resize(self.width, heigth, FilterType::Nearest);

        // let res = String::new();

        for j in 0..img.height()
        {
            for i in 0..img.width()
            {
                let color = img.get_pixel(i , j);
                let luminance: usize = (((0.299 * color[0] as f32 +
                                        0.587 * color[1] as f32 +
                                        0.114 * color[2] as f32) / 255.0) * (ascii.len() - 1) as f32).round() as usize;

                let letter = format!("\x1b[38;2;{};{};{}m{}\x1b[0m", color[0], color[1], color[2], ascii[luminance] as char);

                print!("{}", letter);
            }
            println!();
        }

    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];
    let width: u32;
    if args.len() == 4
    {
        width = match &args[3].parse()
        {
            Ok(num) => *num,
            Err(_) => { 0}
        };
    }
    else { width = 200}

    let img = match image::open(file_path)
    {
        Ok(image) => image,
        Err(_) =>
            {
                println!("Cant find file");
                std::process::exit(1);
            }
    };
    let art = AsciiArt::new(img, width);
    art.convert();
}
