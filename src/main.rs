mod static_data;

extern crate image;

use std::env::args;
use static_data::ASCII_BITMAPS;
use image::{Rgb, RgbImage};
use rand::{Rng, thread_rng};

fn main() {
    let input = args().nth(1).unwrap();
    let width = 300;
    let height = 100;
    let mut img = RgbImage::new(width, height);
    let mut x = 10;
    let mut y = 10;
    let mut rng = thread_rng();

    for c in input.chars() {
        if c == '\n' {
            y += 10;
            x = 10;
        } else {
            draw_char(&mut img, x, y, c, Rgb([rng.gen_range(100..=255u8), 0, rng.gen_range(100..=255u8)]));
            x += 10;
        }
    }
    img.save(format!("{}.png", input)).unwrap();
}

fn draw_char(img: &mut RgbImage, x: u32, y: u32, c: char, color: Rgb<u8>) {
    let bitmap = match c {
        'A'..='Z' => ASCII_BITMAPS[c as usize - 'A' as usize],
        'a'..='z' => ASCII_BITMAPS[c as usize - 'a' as usize + 26],
        '0'..='9' => ASCII_BITMAPS[c as usize - '0' as usize + 52],
        _ => ASCII_BITMAPS[62], // " "
    };

    for i in 0..8 {
        for j in 0..8 {
            if bitmap[i as usize] & (1 << (7 - j)) != 0 {
                img.put_pixel(x + j, y + i, color);
            }
        }
    }
}
