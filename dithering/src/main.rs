use image::{io::Reader as ImageReader, DynamicImage, ImageError, ImageOutputFormat};
use std::io::Cursor;

fn main() -> Result<(), ImageError> {
    let img = ImageReader::open("image/BUTInfo.jpg")?.decode()?;

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)?;

    let rgb = img.to_rgb8();
    let no_alpha = DynamicImage::ImageRgb8(rgb);

    no_alpha.save("image/no_alpha.jpg")?;

    Ok(())
}
