use crate::{assets::*, contact_sheet, installed};
use anyhow::{Result, ensure};
use image::{DynamicImage, Rgb, RgbImage};
use rc_zip_sync::ReadZip;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

const LUT: &str = "assets/animations/Player/Base/spr_player_base_lut.png";
// Main skin through deepest shadow in the shipped player lookup table.
const SKIN_ROWS: [u32; 4] = [225, 186, 125, 82];

fn hex(p: [u8; 4]) -> String {
    format!("#{:02X}{:02X}{:02X}{:02X}", p[0], p[1], p[2], p[3])
}

pub fn build(archive_path: &Path, output: &Path) -> Result<Value> {
    let output = fresh_output(output, &[archive_path])?;
    let file = fs::File::open(archive_path)?;
    let archive = file.read_zip()?;
    let bytes = installed::bytes(&archive, LUT)?;
    let lut = rgba(&bytes)?;
    ensure!(
        lut.height() == 256 && (2..=256).contains(&lut.width()),
        "Expected a 256-row player LUT with a source column and preset columns"
    );
    let mut seen = BTreeMap::new();
    let mut presets = Vec::new();
    let mut sheet = RgbImage::from_pixel(1152, (lut.width() - 1).div_ceil(6) * 92, Rgb([238; 3]));
    for column in 1..lut.width() {
        let colors = SKIN_ROWS.map(|row| lut.get_pixel(column, row).0);
        ensure!(
            colors.iter().all(|p| p[3] == 255),
            "Skin LUT entries must be opaque"
        );
        let id = format!("player_{column:02}");
        let same = seen.get(&colors).cloned();
        seen.entry(colors).or_insert(id.clone());
        let mut row = json!({"id":id, "label":format!("Player {column:02}"), "column":column, "colors":colors.map(hex)});
        if let Some(same) = same {
            row["same_ramp_as"] = json!(same);
        }
        presets.push(row);
        let x = ((column - 1) % 6) * 192 + 8;
        let y = ((column - 1) / 6) * 92 + 8;
        contact_sheet::text(&mut sheet, x, y, &format!("Player {column:02}"));
        for (index, p) in colors.iter().enumerate() {
            for yy in 0..48 {
                for xx in 0..42 {
                    sheet.put_pixel(
                        x + index as u32 * 42 + xx,
                        y + 20 + yy,
                        Rgb([p[0], p[1], p[2]]),
                    );
                }
            }
        }
    }
    let mut portraits = BTreeMap::new();
    let mut names = BTreeSet::new();
    for entry in archive.entries() {
        let name = entry.name.clone();
        if !(name.starts_with("assets/animations/NPCs/")
            || name.starts_with("assets/animations/Cameos/"))
            || !name.contains("/Portraits/")
            || !name.ends_with(".png")
        {
            continue;
        }
        ensure!(
            names.insert(name.clone()),
            "Duplicate portrait archive member: {name}"
        );
        let data = entry.bytes()?;
        let image = rgba(&data)?;
        let mut colors = BTreeMap::<String, u64>::new();
        for p in image.pixels().filter(|p| p[3] == 255) {
            *colors.entry(hex(p.0)).or_default() += 1;
        }
        portraits.insert(name.clone(), json!({"asset":name,"source_sha256":digest(&data),"size":[image.width(),image.height()],"status":"unreviewed_colors","opaque_colors":colors}));
    }
    let report = json!({
        "archive_sha256":file_digest(archive_path)?,
        "player_source":{"asset":LUT,"source_sha256":digest(&bytes),"size":[lut.width(),lut.height()],"skin_rows":SKIN_ROWS},
        "player_presets":presets,"distinct_player_ramps":seen.len(),
        "portraits":portraits.into_values().collect::<Vec<_>>()
    });
    write_tree(
        &output,
        Outputs::from([
            ("catalog.json".into(), json_bytes(&report)?),
            (
                "player-palettes.png".into(),
                png(&DynamicImage::ImageRgb8(sheet))?,
            ),
        ]),
    )?;
    Ok(
        json!({"player_presets":lut.width()-1,"distinct_player_ramps":seen.len(),"portrait_strips":names.len(),"catalog":output.join("catalog.json")}),
    )
}
