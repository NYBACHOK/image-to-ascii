use crate::character_set::CHARACTER_SET;
use crate::resize::get_resized;
use eyre::{Result, WrapErr};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::fs::File;
use std::io::Write;

pub fn parse_image(path: &str, output: Option<String>) -> Result<()> {
    let mut img = ImageReader::open(path)
        .wrap_err("Failed to open image")?
        .decode()
        .unwrap_or_else(|_| {
            eprintln!("Failed to decode image");
            std::process::exit(2)
        })
        .grayscale();

    let (width, height) = match output {
        None => get_resized(img.width(), img.height()),
        Some(_) => (img.width(), img.height()),
    };

    img = img.resize(width, height, FilterType::Nearest);

    let mut art = String::with_capacity((img.height() * img.width()) as usize);
    let mut last_y = 0;
    for pixel in img.pixels() {
        if last_y != pixel.1 {
            art.push_str("\n");
            last_y = pixel.1;
        }

        let pixel_data = pixel.2;
        let brightness: f64 =
            ((pixel_data[0] as u64 + pixel_data[1] as u64 + pixel_data[2] as u64) / 3) as f64;
        let character_position =
            ((brightness / 255.0) * (CHARACTER_SET.len() - 1) as f64).round() as usize;
        art.push_str(CHARACTER_SET[character_position])
    }

    match output {
        None => {
            println!("{}", art);
        }
        Some(path) => {
            let mut file = File::create(path).wrap_err("Failed to create file")?;

            file.flush().unwrap();
            file.write_all(art.as_bytes()).unwrap();
        }
    };

    Ok(())
}
