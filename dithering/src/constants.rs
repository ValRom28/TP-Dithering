const NOIR: [u8; 3] = [0, 0, 0];
const BLANC: [u8; 3] = [255, 255, 255];
const ROUGE: [u8; 3] = [255, 0, 0];
const VERT: [u8; 3] = [0, 255, 0];
const BLEU: [u8; 3] = [0, 0, 255];
const JAUNE: [u8; 3] = [255, 255, 0];
const CYAN: [u8; 3] = [0, 255, 255];
const MAGENTA: [u8; 3] = [255, 0, 255];

const PALETTE: [[u8; 3]; 8] = [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA];

pub fn get_color_from_name(color_name: &str) -> Option<[u8; 3]> {
    match color_name.to_lowercase().as_str() {
        "noir" => Some(NOIR),
        "blanc" => Some(BLANC),
        "rouge" => Some(ROUGE),
        "vert" => Some(VERT),
        "bleu" => Some(BLEU),
        "jaune" => Some(JAUNE),
        "cyan" => Some(CYAN),
        "magenta" => Some(MAGENTA),
        _ => None,
    }
}
