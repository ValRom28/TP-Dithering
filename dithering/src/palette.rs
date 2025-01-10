pub fn palette(image: &mut DynamicImage, palette: Vec<[u8; 3]>) -> image::RgbImage {
    let mut rgb_image = image.to_rgb8();

    if palette.is_empty() {
        return rgb_image;
    }

    for (_x, _y, pixel) in rgb_image.enumerate_pixels_mut() {
        let mut min_distance = 255.0 * 3.0;
        let mut best_color = [0, 0, 0];
        for color in palette.iter() {
            let distance = (pixel[0] as f32 - color[0] as f32).abs()
                + (pixel[1] as f32 - color[1] as f32).abs()
                + (pixel[2] as f32 - color[2] as f32).abs();
            if distance < min_distance {
                min_distance = distance;
                best_color = *color;
            }
        }
        *pixel = image::Rgb(best_color);
    }

    return rgb_image;
}
