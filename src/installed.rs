//! Check the pixels the engine reads, not just MOMI's successful exit status.
use crate::{
    assets::{inventory, rgba},
    toggle,
};
use anyhow::{Context, Result, ensure};
use image::{GenericImageView, RgbaImage};
use rc_zip_sync::{ArchiveHandle, ReadZip};
use serde::Deserialize;
use std::{collections::BTreeSet, fs, path::Path};

pub const SOURCE: &str =
    "assets/animations/NPCs/Adeline/Portraits/Spring/spr_portrait_adeline_spring_neutral";
// MOMI's namespace for the fixed author/name in mod/toggle/manifest.toml.
const PALETTE_SCRIPTS: &str = "assets/gml/scripts/lightning_and_sun_adeline_palette_toggle_study";

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
struct AtlasPage {
    image: RgbaImage,
    placements: Vec<Placement>,
}

pub fn verify(archive: &Path, original: &Path, modified: &Path) -> Result<()> {
    let file = fs::File::open(archive)?;
    let archive = file.read_zip()?;
    bytes(&archive, "manifest.toml")?;
    let (images, _) = inventory(original)?;
    let mut pairs = Vec::new();
    let mut ids = BTreeSet::new();
    // MOMI adds atlas frames and metadata; it does not add loose variant PNGs.
    let mut pages = Vec::new();
    for entry in archive.entries() {
        let Some(suffix) = entry
            .name
            .strip_prefix("assets/atlases/PortraitsSpringAtlas")
            .and_then(|s| s.strip_suffix(".meta.toml"))
        else {
            continue;
        };
        if !suffix.is_empty()
            && !suffix
                .strip_prefix('_')
                .is_some_and(|n| n.parse::<u32>().is_ok())
        {
            continue;
        }
        let placements: Atlas =
            toml::from_str(std::str::from_utf8(&bytes(&archive, &entry.name)?)?)?;
        let png = format!("{}.png", entry.name.strip_suffix(".meta.toml").unwrap());
        pages.push(AtlasPage {
            image: rgba(&bytes(&archive, &png)?)?,
            placements: placements.asset_properties.animations,
        });
    }
    ensure!(!pages.is_empty(), "Missing PortraitsSpring atlas pages");
    for (name, path) in images {
        let pair = toggle::sprite_pair(&name)?;
        let variant_path = format!("assets/animations/LightningAndSun/{}", pair[1]);
        let metadata_path = name
            .strip_suffix(".png")
            .context("Expected a PNG portrait")?;
        let before = fs::read(path)?;
        ensure!(bytes(&archive, &name)? == before, "Vanilla PNG changed");
        let meta: toml::Value = toml::from_str(&fs::read_to_string(
            original.join(format!("{metadata_path}.meta.toml")),
        )?)?;
        ensure!(
            document(&archive, &format!("{metadata_path}.meta.toml"))? == meta,
            "Vanilla metadata changed"
        );
        let variant = document(&archive, &format!("{variant_path}.meta.toml"))?;
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
        ensure!(
            ids.insert(original_id.clone()) && ids.insert(variant_id.clone()),
            "Portrait animation IDs overlap"
        );
        let blue = fs::read(modified.join(&name))?;
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
            check_frames(&pages, &id, &expected, [width, height], count)?;
        }
        pairs.push(pair);
    }
    let asset_script = toggle::asset_script(&pairs)?;
    for (name, expected) in [
        (
            "palette_toggle.gml",
            include_bytes!("../mod/toggle/gml/palette_toggle.gml").as_slice(),
        ),
        ("palette_assets.gml", asset_script.as_slice()),
    ] {
        ensure!(
            bytes(&archive, &format!("{PALETTE_SCRIPTS}/{name}"))? == expected,
            "Installed palette script differs: {name}"
        );
    }
    Ok(())
}

fn check_frames(
    pages: &[AtlasPage],
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
        let matches: Vec<_> = pages
            .iter()
            .flat_map(|page| {
                page.placements
                    .iter()
                    .map(move |placement| (&page.image, placement))
            })
            .filter(|(_, p)| p.texture_ids.contains(&name))
            .collect();
        ensure!(
            matches.len() == 1,
            "Missing or duplicate atlas frame: {name}"
        );
        let (atlas, placement) = matches[0];
        let [x, y, w, h, ow, oh, tx, ty] = placement.placement;
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
