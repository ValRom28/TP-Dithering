use image::DynamicImage;

pub fn seuillage(
    image: &mut DynamicImage,
    seuil: u8,
    color1: Option<[u8; 3]>,
    color2: Option<[u8; 3]>,
) -> image::RgbImage {
    let color1 = color1.unwrap_or([255, 255, 255]); // Blanc par défaut
    let color2 = color2.unwrap_or([0, 0, 0]); // Noir par défaut

    let mut rgb_image = image.to_rgb8();
    for (_x, _y, pixel) in rgb_image.enumerate_pixels_mut() {
        let pixel_luminosity =
            0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32;
        if pixel_luminosity > seuil as f32 {
            *pixel = image::Rgb(color1);
        } else {
            *pixel = image::Rgb(color2);
        }
    }

    return rgb_image;
}
