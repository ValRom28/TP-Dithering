pub fn semi_white(image: &mut DynamicImage) -> RgbImage {
    let mut image = image.to_rgb8();
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if x % 2 == 0 && y % 2 == 0 {
            *pixel = image::Rgb([255, 255, 255]);
        }
    }

    return image;
}
