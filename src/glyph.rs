/// Maps a 24-bit patch pattern onto a CJK/kana/Hangul character.
///
/// The script is chosen by the glyph's position on the wall, not by the bits —
/// so the wall carries a fixed diagonal pattern of writing systems, and the
/// cell pattern only selects which character within that script.

/// (block start, block length). All double-width, so the wall stays square.
const BLOCKS: &[(u32, u32)] = &[
    (0x2190, 112),   // Arrows
    (0x2200, 256),   // Mathematical Operators
    (0x2300, 256),   // Miscellaneous Technical
    (0x2500, 128),   // Box Drawing
    (0x2580, 96),    // Block Elements
    (0x25A0, 96),    // Geometric Shapes
    (0x2600, 256),   // Miscellaneous Symbols
    (0x2700, 192),   // Dingbats
    (0x2800, 256),   // Braille Patterns
    (0x16A0, 89),    // Runic
    (0x10A0, 88),    // Georgian
    (0x0530, 89),    // Armenian
    (0x0590, 112),   // Hebrew
    (0x0600, 255),   // Arabic
    (0x0900, 128),   // Devanagari
    (0x0E00, 128),   // Thai
    (0x13A0, 92),    // Cherokee
    (0x1400, 640),   // Canadian Aboriginal Syllabics
    (0x3041, 86),    // Hiragana
    (0x30A1, 90),    // Katakana
    (0x4E00, 20992), // CJK Unified Ideographs
    (0xAC00, 11172), // Hangul syllables
];

/// Number of distinct patch patterns: 6 x 4 cells = 24 bits.
const STATES: u64 = 1 << 24;

/// Ideographic space — double-width, so empty patches align with the rest.
const BLANK: char = '\u{3000}';

pub fn glyph(bits: u32, wall_row: usize, wall_col: usize) -> char {
    if bits == 0 {
        return BLANK;
    }

    let (base, span) = BLOCKS[(wall_row + wall_col) % BLOCKS.len()];
    let offset = (bits as u64 * span as u64 / STATES) as u32;

    char::from_u32(base + offset).unwrap_or(BLANK)
}