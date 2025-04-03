#![forbid(unsafe_code)]
#![deny(
    clippy::complexity,
    clippy::perf,
    clippy::checked_conversions,
    clippy::filter_map_next
)]
#![warn(
    clippy::style,
    clippy::map_unwrap_or,
    clippy::missing_const_for_fn,
    clippy::use_self,
    future_incompatible,
    rust_2018_idioms,
    nonstandard_style
)]
// with configurable values
#![warn(
    clippy::blacklisted_name,
    clippy::cognitive_complexity,
    clippy::disallowed_method,
    clippy::fn_params_excessive_bools,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::trivially_copy_pass_by_ref,
    clippy::type_repetition_in_bounds,
    clippy::unreadable_literal
)]
#![deny(clippy::wildcard_imports)]
// crate-specific exceptions:

use anyhow::Result;
use image::io::Reader;
use image::{Rgb, RgbImage};
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::fs::read_dir;
use std::path::Path;
use std::ffi::CStr;

#[derive(Debug, Copy, Clone)]
struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    const fn new(color: [u8; 3]) -> Self {
        Self {
            red: color[0],
            green: color[1],
            blue: color[2],
        }
    }

    const fn black() -> Self {
        Self::new([0, 0, 0])
    }

    const fn white() -> Self {
        Self::new([255, 255, 255])
    }

    const fn to_rgb(self) -> Rgb<u8> {
        Rgb([self.red, self.green, self.blue])
    }
}

fn filter_image_rgb(from_path: &str, dest_path: &str, from_color: Color, to_color: Color) -> Result<RgbImage> {
    let img = Reader::open(from_path)?.decode()?;

    let mut img = img.to_rgb8();

    // Convert all pixel not in the range full black
    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(_x, _y, pixel_ref)| {
            let [r, g, b] = pixel_ref.0;

            if !(r >= from_color.red
                && r <= to_color.red
                && g >= from_color.green
                && g <= to_color.green
                && b >= from_color.blue
                && b <= to_color.blue)
            {
                *pixel_ref = Color::black().to_rgb()
            } else {
                *pixel_ref = Color::white().to_rgb()
            }
        });

    img.save(dest_path)?;

    Ok(img)
}

fn main() -> Result<()> {
    const FROM_RANGE: Color = Color::new([253, 157, 89]);
    const TO_RANGE: Color = Color::white();

    for entry in read_dir("data/")? {
        let file = entry?;
        let path = file.path().to_str().unwrap().to_string();

        if path.ends_with(".png") {
            let name = file.file_name().to_str().unwrap().to_string();
            let dest_path = &["output/", &name].concat();

            filter_image_rgb(&path, dest_path, FROM_RANGE, TO_RANGE)?;

            let mut api = leptess::tesseract::TessApi::new(None, "fra")?;
            let pix = leptess::leptonica::pix_read(Path::new(dest_path))?;

            api.set_image(&pix);
            api.set_source_resolution(70);

            api.raw.set_variable(leptess::Variable::TesseditPagesegMode.as_cstr(), CStr::from_bytes_with_nul(b"6\0")?)?;

            println!("{}", api.get_utf8_text()?);
        }
    }

    Ok(())
}
