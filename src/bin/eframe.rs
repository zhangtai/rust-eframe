use image::{DynamicImage, GenericImageView, RgbImage};
use rust_it8951::{It8951, Inquiry, Mode, SystemInfo};
use std::thread;
use std::time::Duration;
use clap::Parser;

struct DisplayDimention(u32, u32);
struct DisplayShrinks(u32, u32);

#[derive(Parser,Default,Debug)]
struct Arguments {
    file: String,
    resize: String,
}

fn main() -> anyhow::Result<()> {
    println!("Start");
    let mut it8951 = It8951::connect()?;

    let inquiry = it8951.inquiry()?;
    thread::sleep(Duration::from_millis(100));
    let display_info = it8951.get_system_info().unwrap();
    print_display_info(inquiry, display_info);

    let dimention = DisplayDimention(display_info.width, display_info.height);
    let shrinks = DisplayShrinks(64, 34);
    let dwidth: u32 = display_info.width;
    let dheight: u32 = display_info.height;
    let clear_display = true;

    let args = Arguments::parse();

    if clear_display {
        let rgb: RgbImage = RgbImage::new(dimention.0, dimention.1);
        let cavas: DynamicImage = DynamicImage::ImageRgb8(rgb).grayscale();
        it8951.update_region(&cavas, 0, 0, Mode::INIT)?;
        thread::sleep(Duration::from_millis(1500));
    }

    println!("Pushing image");
    let img_from_file = image::open(args.file)?;
    let img = convert_image(img_from_file, dimention, shrinks, args.resize);

    let px: u32 = (dwidth-img.width()) / 2;
    let py: u32 = (dheight-img.height()) / 2 - 8;
    
    // println!("FX: {}", img.width());
    // println!("FY: {}", img.height());
    // println!("PX: {}", px);
    // println!("PY: {}", py);

    it8951.update_region(&img, px, py, Mode::GC16)?;
    println!("End");
    Ok(())
}


fn print_display_info(inquiry: Inquiry, display: &SystemInfo) {
    println!("Reading device info:");

    println!("    vendor: {}", inquiry.vendor);
    println!("    product: {}", inquiry.product);
    println!("    revision: {}", inquiry.revision);

    println!("    width: {}", display.width);
    println!("    height: {}", display.height);
    println!("    mode: {}", display.mode);
    println!("    version: {}", display.version);
}

fn convert_image(input_image: DynamicImage, dimension: DisplayDimention, shrinks: DisplayShrinks, resize: String) -> DynamicImage {
    println!("Converting image");

    let rotated: DynamicImage;
    let resized: DynamicImage;

    if input_image.height() > input_image.width() {
        // input_image = input_image.rotate270();
        rotated = input_image.rotate270().fliph().grayscale();
    } else {
        rotated = input_image.fliph().grayscale();
    }

    if resize == "fill" {
        resized = rotated.resize_to_fill(dimension.0 - shrinks.0, dimension.1 - shrinks.1, image::imageops::FilterType::Nearest);
    } else {
        resized = rotated.resize(dimension.0 - shrinks.0, dimension.1 - shrinks.1, image::imageops::FilterType::Nearest);
    }

    return resized;
}
