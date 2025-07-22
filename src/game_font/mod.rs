pub const FONT_ASSET_PATH: &str = "hud elements/fonts.png";

pub const FONT_GLYPH_SIZE: u32 = 7;
pub const FONT_OFFSET: usize = 10;

pub const FONT_EXCLAMATION_MARK_INDEX: usize = 36;
pub const FONT_QUESTION_MARK_INDEX: usize = 37;
pub const FONT_SLASH_INDEX: usize = 38;

pub const FONT_SPACEBAR_INDEX: usize = 99;

pub fn get_font_char_index(char: &char) -> usize {
    if *char == ' ' {
        return FONT_SPACEBAR_INDEX;
    }
    if char.is_ascii_punctuation() {
        eprintln!("Punctuation marks are not yet supported.");
        return FONT_SPACEBAR_INDEX;
    }
    if char.is_ascii_alphabetic() {
        println!("char: {}", char);
        let offset = (char.to_ascii_uppercase() as u8 - b'A') as usize;
        return FONT_OFFSET + offset;
    } else {
        eprintln!("Missing font index for: {}", char);
        return FONT_SPACEBAR_INDEX;
    }
}

pub fn get_font_indices_from_text(text: &String) -> Vec<usize> {
    text.chars()
        .map(|char| get_font_char_index(&char))
        .collect()
}
