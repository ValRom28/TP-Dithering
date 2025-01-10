fn generate_bayer_matrix(order: u32) -> Vec<Vec<u32>> {
    if order == 0 {
        return vec![vec![0]];
    }

    let prev_matrix = generate_bayer_matrix(order - 1);
    let size = 2_usize.pow(order);
    let mut matrix = vec![vec![0; size]; size];

    for i in 0..size / 2 {
        for j in 0..size / 2 {
            let val = prev_matrix[i][j];
            matrix[i][j] = 4 * val;
            matrix[i + size / 2][j] = 4 * val + 3;
            matrix[i][j + size / 2] = 4 * val + 2;
            matrix[i + size / 2][j + size / 2] = 4 * val + 1;
        }
    }

    matrix
}

pub fn tramage_bayer(
    image: &mut DynamicImage,
    order: u32,
    color1: Option<[u8; 3]>,
    color2: Option<[u8; 3]>,
) -> image::RgbImage {
    let color1 = color1.unwrap_or([255, 255, 255]); // Blanc par défaut
    let color2 = color2.unwrap_or([0, 0, 0]); // Noir par défaut

    let bayer_matrix = generate_bayer_matrix(order);
    let matrix_size = bayer_matrix.len();
    let mut rgb_image = image.to_rgb8();

    for (x, y, pixel) in rgb_image.enumerate_pixels_mut() {
        let pixel_luminosity =
            0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32;
        let threshold = bayer_matrix[(x as usize % matrix_size)][(y as usize % matrix_size)] as f32
            / (matrix_size * matrix_size) as f32;
        if pixel_luminosity / 255.0 > threshold {
            *pixel = image::Rgb(color1);
        } else {
            *pixel = image::Rgb(color2);
        }
    }

    rgb_image
}
