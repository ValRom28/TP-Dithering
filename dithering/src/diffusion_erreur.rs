use image::{GrayImage, Luma, Rgb};

pub fn diffusion_erreur_noir_blanc(img: &mut GrayImage) -> GrayImage {
    let (width, height) = img.dimensions();
    let threshold = 128u8;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y)[0];
            let new_pixel = if pixel < threshold { 0 } else { 255 };
            let error = pixel as i16 - new_pixel as i16;

            // Mise à jour du pixel courant
            img.put_pixel(x, y, Luma([new_pixel as u8]));

            // Diffusion de l'erreur selon la matrice spécifiée
            // en multipliant l'erreur par le coefficient correspondant
            if x + 1 < width {
                let right_pixel = img.get_pixel(x + 1, y)[0] as i16;
                img.put_pixel(
                    x + 1,
                    y,
                    Luma([clamp_pixel(right_pixel + (error as f32 * 0.5) as i16)]),
                );
            }

            if y + 1 < height {
                let bottom_pixel = img.get_pixel(x, y + 1)[0] as i16;
                img.put_pixel(
                    x,
                    y + 1,
                    Luma([clamp_pixel(bottom_pixel + (error as f32 * 0.5) as i16)]),
                );
            }
        }
    }

    img.clone()
}

pub fn diffusion_erreur_palette(img: &DynamicImage, palette: Vec<[u8; 3]>) -> DynamicImage {
    let mut img = img.to_rgb8();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let orig_color = [pixel[0], pixel[1], pixel[2]];

            // Trouver la couleur la plus proche dans la palette
            let new_color = find_nearest_color(&orig_color, &palette);

            // Calculer l'erreur de quantification (différence entre la couleur d'origine et la nouvelle couleur)
            let error = [
                orig_color[0] as i16 - new_color[0] as i16,
                orig_color[1] as i16 - new_color[1] as i16,
                orig_color[2] as i16 - new_color[2] as i16,
            ];

            // Appliquer la nouvelle couleur au pixel actuel
            img.put_pixel(x, y, Rgb(new_color));

            // Diffuser l'erreur selon la matrice spécifiée
            if x + 1 < width {
                let right_pixel = img.get_pixel_mut(x + 1, y);
                for i in 0..3 {
                    right_pixel.0[i] =
                        clamp_pixel(right_pixel.0[i] as i16 + (error[i] as f32 * 0.5) as i16);
                }
            }

            if y + 1 < height {
                let bottom_pixel = img.get_pixel_mut(x, y + 1);
                for i in 0..3 {
                    bottom_pixel[i] =
                        clamp_pixel(bottom_pixel[i] as i16 + (error[i] as f32 * 0.5) as i16);
                }
            }
        }
    }

    DynamicImage::ImageRgb8(img)
}

pub fn diffusion_erreur_floyd_steinberg(img: &DynamicImage, palette: Vec<[u8; 3]>) -> DynamicImage {
    let mut img = img.to_rgb8();
    let (width, height) = img.dimensions();

    let coefficients: [(i32, i32, f32); 4] = [
        (1, 0, 7.0 / 16.0),  // Droite
        (-1, 1, 3.0 / 16.0), // Bas-gauche
        (0, 1, 5.0 / 16.0),  // Bas
        (1, 1, 1.0 / 16.0),  // Bas-droite
    ];

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let pixel = img.get_pixel(x as u32, y as u32);
            let orig_color = [pixel[0], pixel[1], pixel[2]];

            // Trouver la couleur la plus proche dans la palette
            let new_color = find_nearest_color(&orig_color, &palette);

            // Calcul de l'erreur (différence entre l'ancienne et la nouvelle couleur)
            let error = [
                orig_color[0] as i16 - new_color[0] as i16,
                orig_color[1] as i16 - new_color[1] as i16,
                orig_color[2] as i16 - new_color[2] as i16,
            ];

            // Appliquer la nouvelle couleur au pixel courant
            img.put_pixel(x as u32, y as u32, Rgb(new_color));

            // Diffuser l'erreur aux pixels voisins en appliquant les coefficients de la matrice
            for &(dx, dy, coef) in &coefficients {
                let nx = x + dx;
                let ny = y + dy;

                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let neighbor_pixel = img.get_pixel_mut(nx as u32, ny as u32);

                    for i in 0..3 {
                        let new_value = neighbor_pixel[i] as i16 + (error[i] as f32 * coef) as i16;
                        neighbor_pixel[i] = clamp_pixel(new_value);
                    }
                }
            }
        }
    }

    DynamicImage::ImageRgb8(img)
}

fn clamp_pixel(value: i16) -> u8 {
    value.clamp(0, 255) as u8
}

fn color_distance(c1: &[u8; 3], c2: &[u8; 3]) -> u32 {
    let r_diff = c1[0] as i32 - c2[0] as i32;
    let g_diff = c1[1] as i32 - c2[1] as i32;
    let b_diff = c1[2] as i32 - c2[2] as i32;
    (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as u32
}

fn find_nearest_color(pixel: &[u8; 3], palette: &Vec<[u8; 3]>) -> [u8; 3] {
    palette
        .iter()
        .min_by(|&&c1, &&c2| color_distance(pixel, &c1).cmp(&color_distance(pixel, &c2)))
        .copied()
        .unwrap_or([0, 0, 0])
}
