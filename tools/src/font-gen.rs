///
/// Simple tool to parse a 160x24 pixel grid into a font glyph lookup table
///
/// All Glyphs are formatted for easy insertion into a data table
///
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

pub mod bitmap;

const WHITE: bitmap::Colour = bitmap::Colour {
    red: 0xFF,
    green: 0xFF,
    blue: 0xFF,
};
const BLACK: bitmap::Colour = bitmap::Colour {
    red: 0x00,
    green: 0x00,
    blue: 0x00,
};
const IGNORE: bitmap::Colour = bitmap::Colour {
    red: 0x00,
    green: 0x00,
    blue: 0xFF,
};

const FILE_NAME: &str = "Nx5-font.bmp";
const MAX_GLYPH_WIDTH: usize = 5;
const MAX_GLYPH_HEIGHT: usize = 5;

struct Glyph {
    width: usize,
    data: Vec<u8>,
}

impl Glyph {
    fn from_bitmap(bitmap: &bitmap::Bitmap, x_offset: usize, y_offset: usize) -> Self {
        let x_offset = x_offset * MAX_GLYPH_WIDTH;
        let y_offset = y_offset * MAX_GLYPH_HEIGHT;
        let mut data: Vec<u8> = Vec::new();

        for x in x_offset..(x_offset + MAX_GLYPH_WIDTH) {
            match bitmap.pixel(x, y_offset) {
                BLACK | WHITE => {
                    data.push(
                        (0..MAX_GLYPH_HEIGHT)
                            .map(|y| match bitmap.pixel(x, y_offset + y) {
                                BLACK => (1 << y) as u8,
                                _ => 0u8,
                            })
                            .sum(),
                    );
                }
                _ => {}
            }
        }

        Glyph {
            width: data.len(),
            data,
        }
    }
}

impl std::fmt::Display for Glyph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut chars: Vec<char> = Vec::new();
        for y in 0..MAX_GLYPH_HEIGHT {
            for x in 0..self.width {
                chars.push(if (self.data[x] >> y) & 1 == 0 {
                    ' '
                } else {
                    '#'
                });
            }
            chars.push('\n');
        }
        write!(f, "{}", String::from_iter(chars.iter()))
    }
}

fn main() -> Result<(), bitmap::Error> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut file = File::open(FILE_NAME)?;
    file.read_to_end(&mut buffer)?;

    let img = bitmap::Bitmap::read_from_buffer(buffer)?;
    eprintln!("{}", img);

    let mut glyphs: Vec<Glyph> = Vec::new();
    for y in 0..(img.height() / MAX_GLYPH_HEIGHT) {
        for x in 0..(img.width() / MAX_GLYPH_WIDTH) {
            glyphs.push(Glyph::from_bitmap(&img, x, y));
        }
    }

    println!(
        "pub const FONT: [(u8, [u8; {}])); {}] = [",
        MAX_GLYPH_WIDTH,
        glyphs.len()
    );
    for idx in 0..glyphs.len() {
        println!(
            "    ({}, [{}]),  // {}",
            glyphs[idx].width,
            glyphs[idx]
                .data
                .iter()
                .map(|c| format!("{}, ", c))
                .collect::<String>(),
            ((0x20 + idx) as u8) as char
        );
    }
    println!("];");

    //
    // println!("pub const FONT_5X6_WIDTH: usize = 5;");
    // println!(
    //     "pub const FONT_5X6: [u8; {}] = [",
    //     glyph_count * GLYPH_WIDTH
    // );
    // for glyph in 0..glyph_count {
    //     print!("    ");
    //     for byte in 0..GLYPH_WIDTH {
    //         let offset = (glyph * GLYPH_WIDTH) + byte;
    //         print!("0x{:02X}, ", output_buffer[offset]);
    //     }
    //     println!(" // {}", ((0x20 + glyph) as u8) as char);
    // }
    // println!("];");

    Ok(())
}
