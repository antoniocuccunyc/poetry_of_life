/// Maps a byte (0-255) onto a printable ASCII character, used to render a
/// patch of cells as a single glyph whose "weight" reflects live-cell density.
pub fn glyph(byte: u8) -> char {
    // Widen before multiplying, narrow after dividing.
    (32 + (byte as u16 * 95 / 256) as u8) as char
}
