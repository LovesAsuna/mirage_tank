use clap::Parser;
use image::ImageBuffer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The first image path
    first: String,

    /// The second image path
    second: String,
}

fn main() {
    let command: Cli = Cli::parse();
    let mut first = image::imageops::grayscale(&image::open(command.first).unwrap());
    let mut seconnd = image::imageops::grayscale(&image::open(command.second).unwrap());

    if first.width() != seconnd.width() || first.height() != seconnd.height() {
        println!("The size of given two images does not match");
        println!(
            "  First: width - {} height - {}",
            first.width(),
            first.height()
        );
        println!(
            "  Second: width - {} height - {}",
            seconnd.width(),
            seconnd.height()
        );
        return;
    }

    // first区间映射到[128, 255]
    for pixel in first.pixels_mut() {
        let mut c = pixel.0[0] as i32;
        c = c * 127 / 255 + 128;
        pixel.0[0] = c as u8;
    }
    // second区间映射到[0, 127]
    for pixel in seconnd.pixels_mut() {
        let mut c = pixel.0[0] as i32;
        c = c * 127 / 255;
        pixel.0[0] = c as u8;
    }

    // 导出图
    let mut buffer = ImageBuffer::new(first.width(), seconnd.height());
    for w in 0..first.width() {
        for h in 0..seconnd.height() {
            let pixel1 = first.get_pixel(w, h);
            let pixel2 = seconnd.get_pixel(w, h);
            let alpha = 255 - pixel1.0[0] as i16 + pixel2.0[0] as i16;
            let color = pixel2.0[0] as i16 * 255 / alpha;
            let a = alpha as u8;
            let c = color as u8;
            let pixel = image::Rgba([c, c, c, a]);
            buffer.put_pixel(w, h, pixel);
        }
    }

    buffer.save("final.png").unwrap();
}
