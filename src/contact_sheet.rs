use crate::assets::*;
use anyhow::{Result, ensure};
use font8x8::{BASIC_FONTS, UnicodeFonts};
use image::{DynamicImage, Rgb, RgbImage};
use serde_json::{Value, json};
use std::{fs, path::Path};

fn text(sheet: &mut RgbImage, x: u32, y: u32, label: &str) {
    for (index, character) in label.chars().enumerate() {
        let glyph = BASIC_FONTS
            .get(character)
            .or_else(|| BASIC_FONTS.get('?'))
            .unwrap();
        for (dy, bits) in glyph.into_iter().enumerate() {
            for dx in 0..8 {
                if bits & (1 << dx) != 0 {
                    sheet.put_pixel(x + index as u32 * 8 + dx, y + dy as u32, Rgb([0; 3]));
                }
            }
        }
    }
}

pub fn build(original: &Path, modified: &Path, output: &Path, zoom: u32) -> Result<Value> {
    ensure!(zoom == 4 || zoom == 8, "Zoom must be 4 or 8");
    ensure!(
        output
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("png")),
        "Contact sheet output must end in .png"
    );
    let report_path = fresh_output(&output.with_extension("json"), &[original, modified])?;
    let output = fresh_output(output, &[original, modified])?;
    let mut report = compare(original, modified)?;
    let rows = report["files"].as_array_mut().unwrap();
    let (margin, heading) = (12u64, 42u64);
    let mut column = 8 * 8; // Leave room for both column headings on tiny sprites.
    let mut height = margin;
    for row in rows.iter() {
        column = column.max(row["size"][0].as_u64().unwrap() * u64::from(zoom));
        column = column.max(row["path"].as_str().unwrap().chars().count() as u64 * 8);
        height += row["size"][1].as_u64().unwrap() * u64::from(zoom) + heading + margin;
    }
    let width = column * 2 + margin * 3;
    ensure!(
        width
            .checked_mul(height)
            .is_some_and(|area| area <= 100_000_000),
        "Contact sheet is too large; select fewer assets"
    );
    let mut sheet = RgbImage::from_pixel(width as u32, height as u32, Rgb([238; 3]));
    let mut y = margin as u32;
    for row in rows {
        let path = row["path"].as_str().unwrap().to_owned();
        text(&mut sheet, margin as u32, y, &path);
        for (index, (label, root)) in [("original", original), ("modified", modified)]
            .into_iter()
            .enumerate()
        {
            let x = (margin + index as u64 * (column + margin)) as u32;
            text(&mut sheet, x, y + 17, label);
            let image = rgba(&fs::read(root.join(&path))?)?;
            let (w, h) = (image.width() * zoom, image.height() * zoom);
            let box_y = y + heading as u32;
            for dy in 0..h {
                for dx in 0..w {
                    let pixel = image.get_pixel(dx / zoom, dy / zoom);
                    let background = if (dx / (4 * zoom) + dy / (4 * zoom)).is_multiple_of(2) {
                        255
                    } else {
                        204
                    };
                    let alpha = u32::from(pixel[3]);
                    let rgb = std::array::from_fn(|i| {
                        ((u32::from(pixel[i]) * alpha + background * (255 - alpha) + 127) / 255)
                            as u8
                    });
                    sheet.put_pixel(x + dx, box_y + dy, Rgb(rgb));
                }
            }
            row[format!("{label}_box")] = json!([x, box_y, w, h]);
        }
        y += row["size"][1].as_u64().unwrap() as u32 * zoom + heading as u32 + margin as u32;
    }
    report["zoom"] = json!(zoom);
    let bytes = png(&DynamicImage::ImageRgb8(sheet))?;
    write_new(&output, &bytes)?;
    write_new(&report_path, &json_bytes(&report)?)?;
    Ok(report)
}
