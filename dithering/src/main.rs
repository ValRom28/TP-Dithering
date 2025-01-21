use image::{io::Reader as ImageReader, ImageError};
include!("../src/seuils.rs");
include!("../src/white.rs");
include!("../src/no_alpha.rs");
include!("../src/palette.rs");
include!("../src/dithering.rs");
include!("../src/tramage_bayer.rs");
include!("../src/diffusion_erreur.rs");

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

    let mut pal = img.clone();
    let palette_img = palette(
        &mut pal,
        vec![
            [0, 0, 0],       // Noir
            [255, 255, 255], // Blanc
            [255, 0, 0],     // Rouge
            [0, 255, 0],     // Vert
            [0, 0, 255],     // Bleu
            [255, 255, 0],   // Jaune
            [255, 0, 255],   // Magenta
            [0, 255, 255],   // Cyan
        ],
    );
    palette_img.save("image/palette.jpg")?;

    let mut pal_vide = img.clone();
    let palette_vide_img = palette(&mut pal_vide, vec![]);
    palette_vide_img.save("image/palette_vide.jpg")?;

    let mut dithering_mono = img.clone();
    let dithering_mono_img = dithering(&mut dithering_mono, None, None);
    dithering_mono_img.save("image/dithering.jpg")?;

    let mut tramage = img.clone();
    let tramage_img = tramage_bayer(&mut tramage, 3, None, None);
    tramage_img.save("image/tramage.jpg")?;

    let mut diffusion = image::imageops::grayscale(&rgb);
    let diffusion_img = diffusion_erreur_noir_blanc(&mut diffusion);
    diffusion_img.save("image/diffusion.jpg")?;

    let mut diffusion_palette = img.clone();
    let diffusion_palette_img = diffusion_erreur_palette(
        &mut diffusion_palette,
        vec![
            [0, 0, 0],       // Noir
            [255, 255, 255], // Blanc
            [255, 0, 0],     // Rouge
            [0, 255, 0],     // Vert
            [0, 0, 255],     // Bleu
        ],
    );
    diffusion_palette_img.save("image/diffusion_palette.jpg")?;

    let mut diffusion_floyd_steinberg = img.clone();
    let diffusion_floyd_steinberg_img = diffusion_erreur_palette(
        &mut diffusion_floyd_steinberg,
        vec![
            [0, 0, 0],       // Noir
            [255, 255, 255], // Blanc
            [255, 0, 0],     // Rouge
            [0, 255, 0],     // Vert
            [0, 0, 255],     // Bleu
        ],
    );
    diffusion_floyd_steinberg_img.save("image/diffusion_floyd_steinberg.jpg")?;

    Ok(())
}
