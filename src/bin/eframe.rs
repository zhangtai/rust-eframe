use image::GenericImageView;
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
    let system_info = it8951.get_system_info().unwrap();
    println!("width: {}", system_info.width);
    println!("height: {}", system_info.height);
    println!("mode: {}", system_info.mode);
    println!("version: {}", system_info.version);

    println!("Display data");
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut img = image::open(file_path)?;
    if img.height() > img.width() {
        img = img.rotate270();
    }
    let flipped_grayscale = img.fliph().grayscale();
    let final_image = flipped_grayscale.resize_to_fill(1872, 1404, image::imageops::FilterType::Nearest);

    // it8951.update_region(&system_info, &[], 0, 0, 0).unwrap();

    // 0 INIT: works - whole screen blanks
    // 1 DU:
    // 2: GC16: partial update, greyscale
    // 3: GL16
    // 4: GLR16
    // 5: GLD16
    // 6: DU4: 4 gray times
    // 7: A2: 2 bit pictures

    it8951.update_region(&final_image, 0, 0, Mode::GC16)?;
    println!("End");
    Ok(())
}
