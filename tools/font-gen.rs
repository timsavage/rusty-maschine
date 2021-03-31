///
/// Simple tool to parse a 160x24 pixel grid into a font glyph lookup table
///
/// All Glyphs are formatted for easy insertion into a data table
///

fn main() {
    let glyph_count = FONT_BUFFER.len() / (GLYPH_WIDTH * GLYPH_HEIGHT);
    let glyph_row_size = GLYPH_WIDTH * GLYPHS_PER_ROW;
    let mut output_buffer: Vec<u8> = Vec::new();

    for row in 0..(glyph_count / GLYPHS_PER_ROW) {
        for col in 0..GLYPHS_PER_ROW {
            for byte in 0..GLYPH_WIDTH {
                let mut slice = 0;
                for bit in 0..8 {
                    let offset = (((row * GLYPH_HEIGHT) + bit) * glyph_row_size) + (col * GLYPH_WIDTH) + byte;
                    // Output from GIMP is inverted
                    if FONT_BUFFER[offset] == 0 {
                        slice |= 1 << bit;
                    }
                }
                output_buffer.push(slice)
            }
        }
    }

    println!("const FONT: [u8; {}] = [", glyph_count * GLYPH_WIDTH);
    for glyph in 0..glyph_count  {
        print!("    ");
        for byte in 0..GLYPH_WIDTH {
            let offset = (glyph * GLYPH_WIDTH) + byte;
            print!("0x{:02X}, ", output_buffer[offset]);
        }
        println!(" // {}", ((0x20 + glyph) as u8) as char);
    }
    println!("\n];");
}

const GLYPH_WIDTH: usize = 5;
const GLYPH_HEIGHT: usize = 8;
const GLYPHS_PER_ROW: usize = 32;
const FONT_BUFFER: [u8; 3840] = [
    1,1,1,1,1,1,1,0,1,1,1,0,1,0,1,1,
    0,1,0,1,1,1,0,1,1,1,1,1,1,1,1,0,
    0,1,1,1,1,0,1,1,1,1,1,0,1,1,0,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,
    1,0,0,0,1,1,1,0,1,1,1,0,0,0,1,1,
    0,0,0,1,1,1,1,0,1,0,0,0,0,0,1,0,
    0,0,1,0,0,0,0,0,1,0,0,0,1,1,0,0,
    0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,1,
    1,1,1,1,1,1,1,0,1,1,1,0,1,0,1,1,
    0,1,0,1,0,0,0,0,0,1,0,1,1,1,0,1,
    1,0,1,1,1,0,1,1,1,1,0,1,1,1,1,0,
    1,1,1,1,0,1,1,1,1,0,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,
    0,1,1,1,0,1,0,0,1,1,0,1,1,1,0,0,
    1,1,1,0,1,1,0,0,1,0,1,1,1,1,0,1,
    1,1,0,1,1,1,1,0,0,1,1,1,0,0,1,1,
    1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,
    0,1,1,1,1,1,0,0,1,1,1,0,1,1,1,0,
    1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,0,
    0,0,0,0,0,1,0,1,1,1,1,1,1,0,0,1,
    1,0,1,1,1,1,1,1,1,0,1,1,1,1,1,1,
    0,1,0,1,0,1,0,1,1,0,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,
    0,1,0,1,0,1,1,0,1,1,1,1,1,1,0,1,
    1,1,1,0,1,0,1,0,1,0,1,1,1,1,0,1,
    1,1,1,1,1,1,0,1,0,1,1,1,0,0,1,1,
    1,0,1,1,0,1,1,1,1,0,1,1,1,0,0,1,
    1,0,0,0,0,0,1,1,0,0,1,1,1,1,1,0,
    1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,
    0,1,0,1,1,0,0,0,1,1,0,0,0,1,1,0,
    0,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,
    0,1,1,0,0,0,1,0,0,0,0,0,1,1,1,1,
    1,0,0,0,0,0,1,1,1,1,1,1,1,0,1,1,
    0,1,0,1,0,1,1,0,1,1,1,1,1,0,1,1,
    0,0,0,1,0,1,1,0,1,0,0,0,0,1,0,0,
    0,0,1,1,1,0,1,1,1,0,0,0,1,1,0,0,
    0,0,1,1,1,1,1,1,1,1,1,1,0,1,1,1,
    1,1,1,1,1,1,1,1,1,1,0,1,1,0,0,1,
    1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,0,
    0,0,0,0,1,1,0,1,0,0,1,1,1,1,0,1,
    0,1,0,1,1,1,1,1,1,0,1,1,1,1,1,1,
    0,1,1,1,0,1,1,1,1,0,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,
    0,1,0,1,0,1,1,0,1,1,1,1,0,1,1,1,
    1,1,1,0,0,0,0,0,0,1,1,1,1,0,0,1,
    1,1,0,1,1,0,1,1,0,1,1,1,0,1,1,1,
    1,0,1,1,0,1,1,1,1,0,1,1,1,0,0,1,
    1,0,0,0,0,0,1,1,0,0,1,1,1,0,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    0,1,0,1,0,0,0,0,0,1,1,1,0,1,0,1,
    1,0,1,1,1,1,1,1,1,1,0,1,1,1,1,0,
    1,1,1,0,1,0,1,1,1,0,1,1,1,1,0,1,
    1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,
    0,1,1,1,0,1,1,0,1,1,1,0,1,1,1,0,
    1,1,1,0,1,1,1,0,1,0,1,1,1,0,0,1,
    1,1,0,1,0,1,1,1,0,1,1,1,0,0,1,1,
    1,0,1,1,1,1,1,1,0,1,1,1,1,1,1,0,
    0,1,1,1,1,1,0,0,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,
    0,1,0,1,1,1,0,1,1,1,1,1,1,1,1,0,
    0,1,0,1,1,1,1,1,1,1,1,0,1,1,0,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,
    1,1,1,1,1,1,1,1,0,1,1,1,0,1,1,1,
    1,0,0,0,1,1,0,0,0,1,0,0,0,0,0,1,
    0,0,0,1,1,1,1,0,1,1,0,0,0,1,1,0,
    0,0,1,0,1,1,1,1,1,0,0,0,1,1,0,0,
    0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,0,0,0,1,1,0,0,0,1,0,0,0,0,1,1,
    0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,
    0,0,0,1,0,0,0,1,0,1,1,1,0,0,0,0,
    0,0,0,0,0,0,0,0,1,1,1,0,0,1,1,1,
    1,0,1,1,1,0,0,1,1,1,0,1,0,0,0,1,
    0,0,0,0,1,1,0,0,0,1,0,0,0,0,1,1,
    0,0,0,1,0,0,0,0,0,0,1,1,1,0,0,1,
    1,1,0,0,1,1,1,0,0,1,1,1,0,0,1,1,
    1,0,0,0,0,0,0,1,0,0,0,1,1,0,1,1,
    1,1,0,0,0,1,1,1,0,1,1,1,1,1,1,1,
    0,1,1,1,0,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,0,0,1,1,1,0,0,1,1,1,1,0,1,
    1,1,1,0,1,1,1,0,0,1,1,1,0,1,1,0,
    1,1,1,1,1,0,1,0,1,1,0,1,0,1,1,1,
    1,0,1,1,1,0,0,1,1,1,0,0,1,1,1,0,
    0,1,1,1,0,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,0,1,1,0,1,1,0,1,1,1,0,0,1,
    1,1,0,0,1,1,1,0,0,1,1,1,0,0,1,1,
    1,0,1,1,1,1,0,1,0,1,1,1,1,0,1,1,
    1,1,1,1,0,1,1,0,1,0,1,1,1,1,1,1,
    0,1,1,0,0,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,1,0,1,1,1,0,0,1,1,1,1,0,1,
    1,1,1,0,1,1,1,1,0,1,1,1,0,1,1,0,
    1,1,1,1,1,0,1,0,1,0,1,1,0,1,1,1,
    1,0,0,1,0,0,0,0,1,1,0,0,1,1,1,0,
    0,1,1,1,0,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,1,1,1,0,1,1,0,1,1,1,0,1,0,
    1,0,1,0,1,1,1,0,1,0,1,0,1,1,0,1,
    0,1,1,1,1,0,1,1,0,1,1,1,1,1,0,1,
    1,1,1,1,0,1,0,1,1,1,0,1,1,1,1,1,
    0,1,0,1,0,0,0,0,0,0,0,0,0,0,1,0,
    1,1,1,1,0,1,1,1,0,0,0,0,0,1,0,0,
    0,0,1,0,1,1,1,1,0,0,0,0,0,1,1,0,
    1,1,1,1,1,0,1,0,0,1,1,1,0,1,1,1,
    1,0,1,0,1,0,0,1,0,1,0,0,1,1,1,0,
    0,0,0,0,1,0,1,1,1,0,0,0,0,0,1,1,
    0,0,0,1,1,1,0,1,1,0,1,1,1,0,1,0,
    1,0,1,0,1,0,1,0,1,1,0,1,1,1,1,0,
    1,1,1,1,0,1,1,1,0,1,1,1,1,1,0,1,
    1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,
    0,1,1,0,1,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,1,0,1,1,1,0,0,1,1,1,1,0,1,
    1,1,1,0,1,1,0,0,0,1,1,1,0,1,1,0,
    1,1,1,1,1,0,1,0,1,0,1,1,0,1,1,1,
    1,0,1,0,1,0,0,1,1,0,0,0,1,1,1,0,
    0,1,1,1,1,0,1,0,1,0,0,1,0,1,1,1,
    1,1,1,0,1,1,0,1,1,0,1,1,1,0,1,0,
    1,0,1,0,1,0,1,0,1,0,1,0,1,1,0,1,
    1,1,1,0,1,1,1,1,0,1,1,1,1,1,0,1,
    1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,
    0,1,1,1,1,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,0,0,1,1,1,0,0,1,1,1,1,0,1,
    1,1,1,0,1,1,1,0,0,1,1,1,0,1,1,0,
    1,1,0,1,1,0,1,0,1,1,0,1,0,1,1,1,
    1,0,1,0,1,0,0,1,1,1,0,0,1,1,1,0,
    0,1,1,1,1,0,1,1,0,0,0,1,1,0,1,0,
    1,1,1,0,1,1,0,1,1,0,1,1,1,0,1,1,
    0,1,1,1,0,1,0,1,0,1,1,1,0,0,1,1,
    1,1,0,1,1,1,1,1,0,1,1,1,1,1,1,0,
    1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,
    1,0,0,0,1,0,1,1,1,0,0,0,0,0,1,1,
    0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,1,
    1,1,1,1,0,0,0,1,0,1,1,1,0,0,0,0,
    0,0,1,0,0,1,1,0,1,1,1,0,0,0,0,0,
    0,0,1,0,1,0,0,1,1,1,0,1,0,0,0,1,
    0,1,1,1,1,1,0,0,0,0,0,1,1,1,0,1,
    0,0,0,1,1,1,0,1,1,1,0,0,0,1,1,1,
    0,1,1,1,0,1,0,1,0,1,1,1,0,0,1,1,
    1,1,0,0,0,0,0,1,0,0,0,1,1,1,1,0,
    1,1,0,0,0,1,1,1,1,1,1,0,0,0,0,0,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,0,1,1,1,1,0,0,0,1,0,0,0,0,1,1,
    0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,
    0,0,0,1,0,0,0,1,0,1,1,1,0,0,0,0,
    0,0,0,0,0,0,0,0,1,1,1,0,0,1,1,1,
    1,0,1,1,1,0,0,1,1,1,0,1,0,0,0,1,
    0,0,0,0,1,1,0,0,0,1,0,0,0,0,1,1,
    0,0,0,1,0,0,0,0,0,0,1,1,1,0,0,1,
    1,1,0,0,1,1,1,0,0,1,1,1,0,0,1,1,
    1,0,0,0,0,0,0,1,1,0,0,1,1,1,0,1,
    1,1,0,0,1,1,1,0,1,1,1,1,1,1,1,1,
    1,1,0,1,1,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,0,0,1,1,1,0,0,1,1,1,1,0,1,
    1,1,1,0,1,1,1,0,0,1,1,1,0,1,1,0,
    1,1,1,1,1,0,1,0,1,1,0,1,0,1,1,1,
    1,0,1,1,1,0,0,1,1,1,0,0,1,1,1,0,
    0,1,1,1,0,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,0,1,1,0,1,1,0,1,1,1,0,0,1,
    1,1,0,0,1,1,1,0,0,1,1,1,0,0,1,1,
    1,0,1,1,1,1,0,1,0,1,1,1,1,1,0,1,
    1,1,1,1,0,1,0,1,0,1,0,1,1,1,1,1,
    1,1,1,0,1,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,1,0,1,1,1,0,0,1,1,1,1,0,1,
    1,1,1,0,1,1,1,1,0,1,1,1,0,1,1,0,
    1,1,1,1,1,0,1,0,1,0,1,1,0,1,1,1,
    1,0,0,1,0,0,0,0,1,1,0,0,1,1,1,0,
    0,1,1,1,0,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,1,1,1,0,1,1,0,1,1,1,0,1,0,
    1,0,1,0,1,1,1,0,1,0,1,0,1,1,0,1,
    0,1,1,1,1,0,1,1,0,1,1,1,1,1,0,1,
    1,1,1,1,0,1,1,1,1,0,1,1,1,1,1,1,
    1,1,1,1,1,0,0,0,0,0,0,0,0,0,1,0,
    1,1,1,1,0,1,1,1,0,0,0,0,0,1,0,0,
    0,0,1,0,1,1,1,1,0,0,0,0,0,1,1,0,
    1,1,1,1,1,0,1,0,0,1,1,1,0,1,1,1,
    1,0,1,0,1,0,0,1,0,1,0,0,1,1,1,0,
    0,0,0,0,1,0,1,1,1,0,0,0,0,0,1,1,
    0,0,0,1,1,1,0,1,1,0,1,1,1,0,1,0,
    1,0,1,0,1,0,1,0,1,1,0,1,1,1,1,0,
    1,1,1,1,0,1,1,0,1,1,1,1,1,1,0,1,
    1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,1,0,1,1,1,0,0,1,1,1,1,0,1,
    1,1,1,0,1,1,0,0,0,1,1,1,0,1,1,0,
    1,1,1,1,1,0,1,0,1,0,1,1,0,1,1,1,
    1,0,1,0,1,0,0,1,1,0,0,0,1,1,1,0,
    0,1,1,1,1,0,1,0,1,0,0,1,0,1,1,1,
    1,1,1,0,1,1,0,1,1,0,1,1,1,0,1,0,
    1,0,1,0,1,0,1,0,1,0,1,0,1,1,0,1,
    1,1,1,0,1,1,1,1,0,1,1,1,1,1,0,1,
    1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,0,1,1,1,0,0,1,1,1,0,0,
    1,1,1,0,0,1,1,1,0,0,1,1,1,1,0,1,
    1,1,1,0,1,1,1,0,0,1,1,1,0,1,1,0,
    1,1,0,1,1,0,1,0,1,1,0,1,0,1,1,1,
    1,0,1,0,1,0,0,1,1,1,0,0,1,1,1,0,
    0,1,1,1,1,0,1,1,0,0,0,1,1,0,1,0,
    1,1,1,0,1,1,0,1,1,0,1,1,1,0,1,1,
    0,1,1,1,0,1,0,1,0,1,1,1,0,0,1,1,
    1,1,0,1,1,1,1,1,0,1,1,1,1,1,0,1,
    1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,0,1,1,1,0,0,0,0,0,1,1,
    0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,1,
    1,1,1,1,0,0,0,1,0,1,1,1,0,0,0,0,
    0,0,1,0,0,1,1,0,1,1,1,0,0,0,0,0,
    0,0,1,0,1,0,0,1,1,1,0,1,0,0,0,1,
    0,1,1,1,1,1,0,0,0,0,0,1,1,1,0,1,
    0,0,0,1,1,1,0,1,1,1,0,0,0,1,1,1,
    0,1,1,1,0,1,0,1,0,1,1,1,0,0,1,1,
    1,1,0,0,0,0,0,1,1,0,0,1,1,1,0,1,
    1,1,0,0,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1
];