use image::{io::Reader as ImageReader, ImageError, ImageOutputFormat};
use std::io::Cursor;

fn main() -> Result<(), ImageError> {
    let img = ImageReader::open("image/BUTInfo.jpg")?.decode()?;

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)?;

    let rgb = img.to_rgb8();

    rgb.save("image/no_alpha.jpg")?;

    let pixel = rgb.get_pixel(32, 52);
    println!("Pixel (32, 52) : {:?}", pixel);

    Ok(())
}
