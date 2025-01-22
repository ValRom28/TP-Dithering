#[derive(FromArgs, PartialEq, Debug, Clone)]
/// Traitement d'images
struct DitheringCli {
    #[argh(positional)]
    /// chemin de l'image d'entrée
    input: String,

    #[argh(positional)]
    /// chemin de l'image de sortie
    output: String,

    #[argh(subcommand)]
    /// mode de traitement
    mode: Mode,
}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand)]
enum Mode {
    Seuils(SeuilsArgs),
    White(WhiteArgs),
    NoAlpha(NoAlphaArgs),
    Palette(PaletteArgs),
    Dithering(DitheringArgs),
    TramageBayer(TramageBayerArgs),
    DiffusionErreur(DiffusionErreurArgs),
}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand, name = "seuil")]
/// Seuillage d'une image
struct SeuilsArgs {
    #[argh(option)]
    /// seuil de luminosité
    seuil: u8,

    #[argh(option)]
    /// première couleur
    color1: String,

    #[argh(option)]
    /// deuxième couleur
    color2: String,
}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand, name = "no-alpha")]
/// Suppression de la transparence d'une image
struct NoAlphaArgs {}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand, name = "white")]
/// Passage d'un pixel sur deux en blanc
struct WhiteArgs {}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand, name = "palette")]
/// Traitement avec une palette
struct PaletteArgs {
    #[argh(option)]
    /// palette à utiliser
    nb_colors: usize,
}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand, name = "dithering")]
/// Dithering d'une image
struct DitheringArgs {
    #[argh(option)]
    /// première couleur
    color1: String,

    #[argh(option)]
    /// deuxième couleur
    color2: String,
}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand, name = "tramage-bayer")]
/// Tramage de Bayer d'une image
struct TramageBayerArgs {
    #[argh(option)]
    /// taille de la matrice
    order: u32,

    #[argh(option)]
    /// première couleur
    color1: String,

    #[argh(option)]
    /// deuxième couleur
    color2: String,
}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand, name = "diffusion-erreur")]
/// Diffusion d'erreur d'une image
struct DiffusionErreurArgs {
    #[argh(option)]
    /// nombre de couleurs de la palette
    nb_colors: usize,
}
