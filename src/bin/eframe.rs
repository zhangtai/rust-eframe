use image::{DynamicImage, GenericImageView, RgbImage};
use rust_it8951::{It8951, Mode};
use std::thread;
use std::time::Duration;
use std::env;

fn main() -> anyhow::Result<()> {
    println!("Start");
    let mut it8951 = It8951::connect()?;

    let inquiry_result = it8951.inquiry()?;
    println!("vendor: {}", inquiry_result.vendor);
    println!("product: {}", inquiry_result.product);
    println!("revision: {}", inquiry_result.revision);
    thread::sleep(Duration::from_millis(100));
    println!("We are now reading data");
    let display = it8951.get_system_info().unwrap();
    println!("width: {}", display.width);
    println!("height: {}", display.height);
    println!("mode: {}", display.mode);
    println!("version: {}", display.version);

    let dwidth: u32 = display.width;
    let dheight: u32 = display.height;
    let swidth: u32 = 60;  // Shrink W
    let sheight: u32 = 40;  // Shrink H

    println!("Display data");
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let rgb: RgbImage = RgbImage::new(dwidth, dheight);
    let cavas: DynamicImage = DynamicImage::ImageRgb8(rgb).grayscale();
    let mut img_from_file = image::open(file_path)?;
    if img_from_file.height() > img_from_file.width() {
        img_from_file = img_from_file.rotate270();
    }
    let flipped_grayscale = img_from_file.fliph().grayscale();
    let final_image = flipped_grayscale.resize(dwidth - swidth, dheight - sheight, image::imageops::FilterType::Nearest);

    it8951.update_region(&cavas, 0, 0, Mode::INIT)?;
    thread::sleep(Duration::from_millis(500));

    println!("FX: {}", final_image.width());
    println!("FY: {}", final_image.height());

    let px: u32 = (dwidth-final_image.width()) / 2;
    let py: u32 = (dheight-final_image.height()) / 2;

    println!("PX: {}", px);
    println!("PY: {}", py);

    it8951.update_region(&final_image, px, py, Mode::GC16)?;
    println!("End");
    Ok(())
}
