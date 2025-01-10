use image::{io::Reader as ImageReader, ImageError};
include!("../src/seuils.rs");
include!("../src/white.rs");
include!("../src/no_alpha.rs");

fn main() -> Result<(), ImageError> {
    let img = ImageReader::open("image/BUTInfo.jpg")?.decode()?;
    let rgb = no_alpha(&mut img.clone());
    rgb.save("image/no_alpha.jpg")?;

    let pixel = rgb.get_pixel(32, 52);
    println!("Pixel (32, 52) : {:?}", pixel);

    let mut white = img.clone();
    let semi_wight = semi_white(&mut white);
    semi_wight.save("image/white.jpg")?;

    let mut seuil = img.clone();
    let seuil_img = seuillage(&mut seuil, 128, None, None);
    seuil_img.save("image/seuil.jpg")?;

    let mut seuil_color = img.clone();
    let seuil_color_img = seuillage(&mut seuil_color, 128, Some([0, 0, 255]), Some([0, 255, 0]));
    seuil_color_img.save("image/seuil_color.jpg")?;

    Ok(())
}
