use image::RgbImage;

pub fn no_alpha(image: &mut DynamicImage) -> RgbImage {
    let image_no_alpha = image.to_rgb8();
    return image_no_alpha;
}
