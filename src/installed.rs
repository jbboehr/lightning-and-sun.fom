//! Check the pixels the engine reads, not just MOMI's successful exit status.
use crate::assets::rgba;
use anyhow::{Context, Result, ensure};
use image::{GenericImageView, RgbaImage};
use rc_zip_sync::{ArchiveHandle, ReadZip};
use serde::Deserialize;
use std::{fs, path::Path};

pub const SOURCE: &str =
    "assets/animations/NPCs/Adeline/Portraits/Spring/spr_portrait_adeline_spring_neutral";
const VARIANT: &str = "assets/animations/LightningAndSun/spr_lns_adeline_spring_neutral_blue";

pub fn bytes(archive: &ArchiveHandle<'_, fs::File>, name: &str) -> Result<Vec<u8>> {
    let mut matches = archive.entries().filter(|e| e.name == name);
    let entry = matches
        .next()
        .with_context(|| format!("Missing archive member: {name}"))?;
    ensure!(matches.next().is_none(), "Duplicate archive member: {name}");
    Ok(entry.bytes()?)
}
fn document(archive: &ArchiveHandle<'_, fs::File>, name: &str) -> Result<toml::Value> {
    Ok(toml::from_str(std::str::from_utf8(&bytes(
        archive, name,
    )?)?)?)
}
#[derive(Deserialize)]
struct Atlas {
    asset_properties: Placements,
}
#[derive(Deserialize)]
struct Placements {
    animations: Vec<Placement>,
}
#[derive(Deserialize)]
struct Placement {
    texture_ids: Vec<String>,
    placement: [u32; 8],
}

pub fn verify(archive: &Path, original: &Path, modified: &Path) -> Result<()> {
    let file = fs::File::open(archive)?;
    let archive = file.read_zip()?;
    bytes(&archive, "manifest.toml")?;
    let before = fs::read(original.join(format!("{SOURCE}.png")))?;
    ensure!(
        bytes(&archive, &format!("{SOURCE}.png"))? == before,
        "Vanilla PNG changed"
    );
    let meta: toml::Value = toml::from_str(&fs::read_to_string(
        original.join(format!("{SOURCE}.meta.toml")),
    )?)?;
    ensure!(
        document(&archive, &format!("{SOURCE}.meta.toml"))? == meta,
        "Vanilla metadata changed"
    );
    let variant = document(&archive, &format!("{VARIANT}.meta.toml"))?;
    ensure!(
        variant.get("asset_properties") == meta.get("asset_properties"),
        "Variant properties differ"
    );
    let uid = |v: &toml::Value| -> Result<String> {
        Ok(v.get("meta_properties")
            .and_then(|v| v.get("id"))
            .and_then(toml::Value::as_str)
            .context("Animation has no ID")?
            .to_owned())
    };
    let original_id = uid(&meta)?;
    let variant_id = uid(&variant)?;
    ensure!(original_id != variant_id, "Variant reused the vanilla ID");
    let blue = fs::read(modified.join(format!("{SOURCE}.png")))?;
    // MOMI adds the atlas frames and metadata; it does not add a loose variant PNG.
    let atlas = rgba(&bytes(&archive, "assets/atlases/PortraitsSpringAtlas.png")?)?;
    let placements: Atlas = toml::from_str(std::str::from_utf8(&bytes(
        &archive,
        "assets/atlases/PortraitsSpringAtlas.meta.toml",
    )?)?)?;
    let frames = meta
        .get("asset_properties")
        .context("Missing frame properties")?;
    let size = frames
        .get("frame_size")
        .and_then(toml::Value::as_array)
        .context("Missing frame size")?;
    ensure!(size.len() == 2, "Expected a two-dimensional frame size");
    let width = u32::try_from(size[0].as_integer().context("Invalid frame width")?)?;
    let height = u32::try_from(size[1].as_integer().context("Invalid frame height")?)?;
    let count = u32::try_from(
        frames
            .get("frame_len")
            .and_then(toml::Value::as_integer)
            .context("Missing frame count")?,
    )?;
    for (id, expected) in [(original_id, rgba(&before)?), (variant_id, rgba(&blue)?)] {
        check_frames(
            &atlas,
            &placements.asset_properties.animations,
            &id,
            &expected,
            [width, height],
            count,
        )?;
    }
    let scripts: Vec<_> = archive
        .entries()
        .filter(|e| e.name.ends_with("/palette_toggle.gml"))
        .collect();
    ensure!(scripts.len() == 1, "Missing or duplicate palette script");
    ensure!(
        scripts[0].bytes()? == include_bytes!("../mod/toggle/gml/palette_toggle.gml"),
        "Installed palette script differs"
    );
    Ok(())
}

fn check_frames(
    atlas: &RgbaImage,
    placements: &[Placement],
    id: &str,
    expected: &RgbaImage,
    [width, height]: [u32; 2],
    count: u32,
) -> Result<()> {
    ensure!(
        width > 0
            && height > 0
            && count > 0
            && width.checked_mul(count) == Some(expected.width())
            && height == expected.height(),
        "Invalid animation strip"
    );
    for frame in 0..count {
        let name = format!("{id}::{frame}");
        let matches: Vec<_> = placements
            .iter()
            .filter(|p| p.texture_ids.contains(&name))
            .collect();
        ensure!(
            matches.len() == 1,
            "Missing or duplicate atlas frame: {name}"
        );
        let [x, y, w, h, ow, oh, tx, ty] = matches[0].placement;
        ensure!(
            [ow, oh] == [width, height],
            "Frame dimensions changed: {name}"
        );
        ensure!(
            x.checked_add(w).is_some_and(|n| n <= atlas.width())
                && y.checked_add(h).is_some_and(|n| n <= atlas.height())
                && tx.checked_add(w).is_some_and(|n| n <= ow)
                && ty.checked_add(h).is_some_and(|n| n <= oh),
            "Frame placement out of bounds: {name}"
        );
        let mut recovered = RgbaImage::new(ow, oh);
        image::imageops::replace(
            &mut recovered,
            &atlas.view(x, y, w, h).to_image(),
            i64::from(tx),
            i64::from(ty),
        );
        ensure!(
            recovered == expected.view(frame * width, 0, width, height).to_image(),
            "Installed pixels differ: {name}"
        );
    }
    Ok(())
}
