use argh::FromArgs;
use image::ImageError;

include!("../src/cli.rs");
include!("../src/seuils.rs");
include!("../src/white.rs");
include!("../src/no_alpha.rs");
include!("../src/palette.rs");
include!("../src/dithering.rs");
include!("../src/tramage_bayer.rs");
include!("../src/diffusion_erreur.rs");
include!("../src/constants.rs");

fn main() -> Result<(), ImageError> {
    let args: DitheringCli = argh::from_env();
    let input = args.input;
    let output = args.output;
    let mode = args.mode;

    let image = image::open(&input)?;

    match mode {
        Mode::NoAlpha(_) => {
            let mut image = image;
            let result = no_alpha(&mut image);
            result.save(&output)?;
        }
        Mode::Seuils(args) => {
            let mut image = image;
            let seuil = args.seuil;
            let color1 = get_color_from_name(&args.color1);
            let color2 = get_color_from_name(&args.color2);
            let result = seuillage(&mut image, seuil, color1, color2);
            result.save(&output)?;
        }
        Mode::White(_) => {
            let mut image = image;
            let result = semi_white(&mut image);
            result.save(&output)?;
        }
        Mode::Palette(args) => {
            let mut image = image;
            let nb_colors = args.nb_colors;
            let result = palette(&mut image, PALETTE[0..nb_colors].to_vec());
            result.save(&output)?;
        }
        Mode::Dithering(args) => {
            let mut image = image;
            let color1 = get_color_from_name(&args.color1);
            let color2 = get_color_from_name(&args.color2);
            let result = dithering(&mut image, color1, color2);
            result.save(&output)?;
        }
        Mode::TramageBayer(args) => {
            let mut image = image;
            let color1 = get_color_from_name(&args.color1);
            let color2 = get_color_from_name(&args.color2);
            let result = tramage_bayer(&mut image, args.order, color1, color2);
            result.save(&output)?;
        }
        Mode::DiffusionErreur(args) => {
            let mut image = image;
            let nb_colors = args.nb_colors;
            let result =
                diffusion_erreur_floyd_steinberg(&mut image, PALETTE[0..nb_colors].to_vec());
            result.save(&output)?;
        }
    }

    Ok(())
}
