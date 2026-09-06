//! Opt-in check of the isolated Adeline installation described in docs/development/momi-lab.md.
use anyhow::{Context, Result, ensure};
use image::{GenericImageView, RgbaImage};
use rc_zip_sync::{ArchiveHandle, ReadZip};
use serde_json::json;
use std::{collections::BTreeSet, fs};

const UID: &str = "5e2770bc75d292ac";
const STEM: &str =
    "assets/animations/NPCs/Adeline/Portraits/Spring/spr_portrait_adeline_spring_neutral";
const ATLAS_META: &str = "assets/atlases/PortraitsSpringAtlas.meta.toml";
const ATLAS_PNG: &str = "assets/atlases/PortraitsSpringAtlas.png";

fn bytes(archive: &ArchiveHandle<'_, fs::File>, name: &str) -> Result<Vec<u8>> {
    archive
        .by_name(name)
        .with_context(|| format!("Missing member: {name}"))?
        .bytes()
        .map_err(Into::into)
}

fn document(archive: &ArchiveHandle<'_, fs::File>, name: &str) -> Result<toml::Value> {
    Ok(toml::from_str(std::str::from_utf8(&bytes(
        archive, name,
    )?)?)?)
}

fn other_placements(document: &toml::Value, uid: &str) -> Vec<toml::Value> {
    document["asset_properties"]["animations"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|row| {
            let mut row = row.clone();
            let ids = row.get_mut("texture_ids").unwrap().as_array_mut().unwrap();
            ids.retain(|id| !id.as_str().unwrap().starts_with(&format!("{uid}::")));
            if ids.is_empty() { None } else { Some(row) }
        })
        .collect()
}

fn check_frames(
    atlas: &RgbaImage,
    metadata: &toml::Value,
    uid: &str,
    expected: &RgbaImage,
) -> Result<()> {
    ensure!(
        expected.dimensions() == (592, 180),
        "Unexpected replacement canvas"
    );
    for frame in 0..2 {
        let id = toml::Value::String(format!("{uid}::{frame}"));
        let matches: Vec<_> = metadata["asset_properties"]["animations"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| row["texture_ids"].as_array().unwrap().contains(&id))
            .collect();
        ensure!(
            matches.len() == 1,
            "Missing or ambiguous frame placement: {frame}"
        );
        let placement: Vec<u32> = matches[0]["placement"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| u32::try_from(v.as_integer().unwrap()))
            .collect::<Result<_, _>>()?;
        let [x, y, w, h, ow, oh, tx, ty]: [u32; 8] = placement
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid placement"))?;
        ensure!((ow, oh) == (296, 180), "Changed frame canvas");
        ensure!(
            x.checked_add(w).is_some_and(|end| end <= atlas.width())
                && y.checked_add(h).is_some_and(|end| end <= atlas.height()),
            "Atlas crop out of bounds"
        );
        ensure!(
            tx.checked_add(w).is_some_and(|end| end <= ow)
                && ty.checked_add(h).is_some_and(|end| end <= oh),
            "Trim offset out of bounds"
        );
        let mut recovered = RgbaImage::new(ow, oh);
        // Copy exact RGBA, without alpha compositing or dropping transparent RGB.
        image::imageops::replace(
            &mut recovered,
            &atlas.view(x, y, w, h).to_image(),
            i64::from(tx),
            i64::from(ty),
        );
        ensure!(
            recovered == expected.view(frame * 296, 0, 296, 180).to_image(),
            "Frame {frame} differs"
        );
    }
    Ok(())
}

#[test]
#[ignore = "requires local game archive, generated Rust package, and installed isolated MOMI lab"]
fn installed_adeline_frames_and_metadata_match() -> Result<()> {
    let original_file = fs::File::open("tmp/fields-of-mistria/assets.zip")?;
    let installed_file = fs::File::open("tmp/momi-lab/assets.zip")?;
    let original = original_file.read_zip()?;
    let installed = installed_file.read_zip()?;
    let old = document(&original, ATLAS_META)?;
    let new = document(&installed, ATLAS_META)?;
    let atlas = image::load_from_memory_with_format(
        &bytes(&installed, ATLAS_PNG)?,
        image::ImageFormat::Png,
    )?
    .to_rgba8();
    let expected = image::open(
        "generated/momi-adeline-stylized/images/replace/spr_portrait_adeline_spring_neutral.png",
    )?
    .to_rgba8();
    check_frames(&atlas, &new, UID, &expected)?;
    ensure!(
        other_placements(&old, UID) == other_placements(&new, UID),
        "Other atlas placements changed"
    );
    let animation_meta = format!("{STEM}.meta.toml");
    ensure!(
        document(&original, &animation_meta)? == document(&installed, &animation_meta)?,
        "Animation metadata changed"
    );
    let old_names: BTreeSet<_> = original.entries().map(|e| e.name.clone()).collect();
    let new_names: BTreeSet<_> = installed.entries().map(|e| e.name.clone()).collect();
    ensure!(
        old_names.difference(&new_names).next().is_none(),
        "Archive members removed"
    );
    ensure!(
        new_names
            .difference(&old_names)
            .map(String::as_str)
            .collect::<Vec<_>>()
            == ["manifest.toml"],
        "Unexpected new archive members"
    );
    let mut changed = Vec::new();
    for entry in original.entries() {
        let current = installed.by_name(&entry.name).unwrap();
        if (entry.crc32, entry.uncompressed_size) != (current.crc32, current.uncompressed_size) {
            changed.push(entry.name.clone());
            if entry.name.ends_with(".meta.toml") && entry.name != ATLAS_META {
                ensure!(
                    document(&original, &entry.name)? == document(&installed, &entry.name)?,
                    "Metadata changed: {}",
                    entry.name
                );
            } else {
                ensure!(
                    entry.name == ATLAS_META || entry.name == ATLAS_PNG,
                    "Unexpected changed member: {}",
                    entry.name
                );
            }
        }
    }
    changed.sort();
    let report = json!({"frames_recovered_exactly":2,"other_placements_unchanged":true,
        "animation_metadata_preserved":true,"changed_members":changed});
    let report = format!("{}\n", serde_json::to_string_pretty(&report)?);
    fs::write("tmp/momi-verification.json", &report)?;
    print!("{report}");
    Ok(())
}

#[test]
#[ignore = "requires a locally generated toggle package installed in tmp/toggle-lab"]
fn installed_toggle_preserves_vanilla_and_adds_both_blue_frames() -> Result<()> {
    let original_file = fs::File::open("tmp/fields-of-mistria/assets.zip")?;
    let installed_file = fs::File::open("tmp/toggle-lab/assets.zip")?;
    let original = original_file.read_zip()?;
    let installed = installed_file.read_zip()?;
    let original_meta = document(&original, &format!("{STEM}.meta.toml"))?;
    ensure!(
        original_meta == document(&installed, &format!("{STEM}.meta.toml"))?,
        "Vanilla animation metadata changed"
    );
    ensure!(
        bytes(&original, &format!("{STEM}.png"))? == bytes(&installed, &format!("{STEM}.png"))?,
        "Vanilla PNG changed"
    );
    let variant = "animations/LightningAndSun/spr_lns_adeline_spring_neutral_blue";
    let variant_meta = document(&installed, &format!("assets/{variant}.meta.toml"))?;
    let uid = variant_meta["meta_properties"]["id"]
        .as_str()
        .context("Missing new sprite ID")?;
    ensure!(uid != UID, "Variant reused the vanilla ID");
    ensure!(
        variant_meta["asset_properties"] == original_meta["asset_properties"],
        "Variant lost animation metadata"
    );
    let atlas_meta = document(&installed, ATLAS_META)?;
    let atlas = image::load_from_memory_with_format(
        &bytes(&installed, ATLAS_PNG)?,
        image::ImageFormat::Png,
    )?
    .to_rgba8();
    let before = image::load_from_memory_with_format(
        &bytes(&original, &format!("{STEM}.png"))?,
        image::ImageFormat::Png,
    )?
    .to_rgba8();
    let blue = image::open(format!("generated/momi-adeline-toggle/{variant}.png"))?.to_rgba8();
    check_frames(&atlas, &atlas_meta, UID, &before)?;
    check_frames(&atlas, &atlas_meta, uid, &blue)?;
    ensure!(
        other_placements(&document(&original, ATLAS_META)?, uid)
            == other_placements(&atlas_meta, uid),
        "Existing portrait atlas placements changed"
    );
    let source = fs::read("mod/toggle/gml/palette_toggle.gml")?;
    ensure!(
        installed
            .entries()
            .filter(|e| e.name.ends_with("/palette_toggle.gml"))
            .count()
            == 1,
        "Missing or duplicate toggle script"
    );
    let entry = installed
        .entries()
        .find(|e| e.name.ends_with("/palette_toggle.gml"))
        .unwrap();
    ensure!(entry.bytes()? == source, "Installed toggle script differs");
    println!(
        "Both vanilla frames and both blue frames match exactly; metadata and existing placements are preserved."
    );
    Ok(())
}
