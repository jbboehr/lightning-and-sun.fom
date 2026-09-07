//! Offline mask authoring. Similar components are suggestions, never approvals.
use crate::{assets::*, installed, palette};
use anyhow::{Context, Result, ensure};
use image::{DynamicImage, RgbaImage};
use rc_zip_sync::ReadZip;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    label: String,
    prefixes: Vec<String>,
    source_colors: Vec<String>,
    preview_colors: Vec<String>,
    #[serde(default)]
    color_groups: Vec<Vec<String>>,
    reference_profile: Option<PathBuf>,
}
#[derive(Deserialize)]
struct Metadata {
    asset_properties: Frames,
}
#[derive(Deserialize)]
struct Frames {
    frame_size: [u32; 2],
    #[serde(default = "one")]
    frame_len: u32,
}
fn one() -> u32 {
    1
}

#[derive(Serialize)]
struct Component {
    seed: [u32; 2],
    // [linear pixel offset, length, index into the batch's color table].
    runs: Vec<[u32; 3]>,
    signature: String,
}
#[derive(Serialize)]
struct Occurrence {
    asset: String,
    frame: u32,
}
#[derive(Serialize)]
struct Group {
    id: String,
    image: String,
    size: [u32; 2],
    bounds: [u32; 4],
    components: Vec<Component>,
    selected: Vec<usize>,
    status: &'static str,
    occurrences: Vec<Occurrence>,
    references: Vec<Occurrence>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    reference_selections: Vec<Vec<usize>>,
}

fn components(image: &RgbaImage, colors: &[[u8; 4]], palette: &palette::Palette) -> Vec<Component> {
    let width = image.width() as usize;
    let shades: Vec<_> = image
        .pixels()
        .map(|p| colors.iter().position(|c| c == &p.0))
        .collect();
    let mut seen = vec![false; shades.len()];
    let mut components = Vec::new();
    for start in 0..shades.len() {
        if seen[start] || shades[start].is_none() {
            continue;
        }
        let mut stack = vec![start];
        let mut pixels = Vec::new();
        seen[start] = true;
        while let Some(i) = stack.pop() {
            pixels.push(i);
            let adjacent = [
                (i % width > 0).then(|| i - 1),
                (i % width + 1 < width).then_some(i + 1),
                i.checked_sub(width),
                (i + width < shades.len()).then_some(i + width),
            ];
            for j in adjacent.into_iter().flatten() {
                if !seen[j]
                    && shades[j].is_some_and(|shade| {
                        palette.connected(&colors[shades[i].unwrap()], &colors[shade])
                    })
                {
                    seen[j] = true;
                    stack.push(j);
                }
            }
        }
        pixels.sort_unstable();
        let mut runs: Vec<[u32; 3]> = Vec::new();
        for i in pixels {
            let shade = shades[i].unwrap() as u32;
            if let Some(last) = runs.last_mut()
                && last[0] + last[1] == i as u32
                && last[2] == shade
                && i % width != 0
            {
                last[1] += 1;
            } else {
                runs.push([i as u32, 1, shade]);
            }
        }
        let signature =
            digest(&serde_json::to_vec(&(image.width(), image.height(), &runs)).unwrap());
        components.push(Component {
            seed: [(start % width) as u32, (start / width) as u32],
            runs,
            signature,
        });
    }
    components
}

fn bounds(image: &RgbaImage) -> [u32; 4] {
    let (mut left, mut top, mut right, mut bottom) = (image.width(), image.height(), 0, 0);
    for (x, y, _) in image.enumerate_pixels().filter(|(_, _, p)| p[3] != 0) {
        left = left.min(x);
        top = top.min(y);
        right = right.max(x + 1);
        bottom = bottom.max(y + 1);
    }
    if right == 0 {
        return [0, 0, image.width(), image.height()];
    }
    left = left.saturating_sub(3);
    top = top.saturating_sub(3);
    right = (right + 3).min(image.width());
    bottom = (bottom + 3).min(image.height());
    [left, top, right - left, bottom - top]
}

fn relative_asset(name: &str) -> bool {
    name.starts_with("assets/")
        && !name.contains('\\')
        && name
            .split('/')
            .all(|p| !p.is_empty() && p != "." && p != "..")
}

pub fn build(archive_path: &Path, config_path: &Path, output: &Path) -> Result<Value> {
    let config: Config = serde_json::from_slice(&fs::read(config_path)?)?;
    ensure!(!config.label.trim().is_empty(), "Review label is required");
    ensure!(
        !config.prefixes.is_empty()
            && config
                .prefixes
                .iter()
                .all(|p| p.ends_with('/') && relative_asset(p.trim_end_matches('/'))),
        "Review prefixes must be relative assets/ directories ending in /"
    );
    ensure!(
        (1..=32).contains(&config.source_colors.len())
            && config.source_colors.len() == config.preview_colors.len(),
        "Supply 1–32 source colors and one preview color per source"
    );
    let mapping: BTreeMap<_, _> = config
        .source_colors
        .iter()
        .zip(&config.preview_colors)
        .collect();
    ensure!(
        mapping.len() == config.source_colors.len(),
        "Duplicate source color"
    );
    let mut recipe = json!({"rgba_map": mapping, "color_groups": config.color_groups});
    let base = palette::parse(&serde_json::to_vec(&recipe)?)?;
    ensure!(
        base.mapping.keys().all(|c| c[3] == 255),
        "Review source colors must be opaque"
    );
    let colors: Vec<_> = base.mapping.keys().copied().collect();
    let targets: Vec<_> = base.mapping.values().copied().collect();
    let reference_path = config
        .reference_profile
        .as_ref()
        .map(|p| config_path.parent().unwrap_or(Path::new(".")).join(p));
    let mut inputs = vec![archive_path, config_path];
    if let Some(path) = &reference_path {
        inputs.push(path);
        let reference: palette::Profile = serde_json::from_slice(&fs::read(path)?)?;
        ensure!(
            config.color_groups.is_empty(),
            "Color groups must come from the referenced profile"
        );
        // Let the existing palette decoder normalize colors and reject ambiguous aliases.
        let identity: BTreeMap<_, _> = reference.source_colors.iter().map(|c| (c, c)).collect();
        ensure!(
            identity.len() == reference.source_colors.len(),
            "Duplicate reference source color"
        );
        let reference_colors = palette::parse(&serde_json::to_vec(&json!({"rgba_map":identity}))?)?;
        ensure!(
            reference_colors.mapping.keys().eq(base.mapping.keys()),
            "Reference source colors differ from review colors"
        );
        recipe["regions"] = json!(reference.regions);
        recipe["color_groups"] = json!(reference.color_groups);
    }
    let palette = palette::parse(&serde_json::to_vec(&recipe)?)?;
    let known: BTreeSet<_> = palette.assets().unwrap_or_default().into_iter().collect();
    let output = fresh_output(output, &inputs)?;
    let file = fs::File::open(archive_path)?;
    let archive = file.read_zip()?;
    let mut selected = BTreeSet::new();
    for entry in archive.entries() {
        if entry.name.ends_with(".png") && config.prefixes.iter().any(|p| entry.name.starts_with(p))
        {
            ensure!(
                relative_asset(&entry.name),
                "Invalid archive asset path: {}",
                entry.name
            );
            ensure!(
                selected.insert(entry.name.clone()),
                "Duplicate archive member: {}",
                entry.name
            );
        }
    }
    ensure!(!selected.is_empty(), "No PNGs matched the review prefixes");
    let names: BTreeSet<_> = selected.union(&known).cloned().collect();
    ensure!(names.len() <= 2000, "Review at most 2000 strips per batch");
    let mut outputs = Outputs::new();
    let mut groups: BTreeMap<String, Group> = BTreeMap::new();
    let mut references: BTreeMap<String, BTreeSet<Vec<usize>>> = BTreeMap::new();
    let mut evidence: BTreeMap<String, BTreeSet<bool>> = BTreeMap::new();
    let mut assets = Vec::new();
    let mut total_pixels = 0u64;
    for name in names {
        let bytes = installed::bytes(&archive, &name)?;
        let image = rgba(&bytes).with_context(|| name.clone())?;
        total_pixels += u64::from(image.width()) * u64::from(image.height());
        ensure!(
            total_pixels <= 50_000_000,
            "Review batch exceeds 50 million pixels; select fewer directories"
        );
        let meta_name = format!(
            "{}.meta.toml",
            name.strip_suffix(".png")
                .context("Reference asset must end in .png")?
        );
        let metadata: Metadata = toml::from_str(std::str::from_utf8(&installed::bytes(
            &archive, &meta_name,
        )?)?)?;
        let Frames {
            frame_size: [w, h],
            frame_len: count,
        } = metadata.asset_properties;
        ensure!(
            w > 0
                && h > 0
                && count > 0
                && w.checked_mul(count) == Some(image.width())
                && h == image.height(),
            "Expected horizontal animation strip metadata: {name}"
        );
        // Profiles flood-fill whole strips. Reject joined frames rather than export
        // seeds that silently select a neighbor the reviewer did not approve.
        for boundary in 1..count {
            let x = boundary * w;
            ensure!(
                !(0..h)
                    .any(|y| palette
                        .connected(&image.get_pixel(x - 1, y).0, &image.get_pixel(x, y).0)),
                "Source-color regions cross a frame boundary: {name}"
            );
        }
        let mask = if known.contains(&name) {
            palette.mask(&name, &bytes, &image)?
        } else {
            None
        };
        let mut frames = Vec::new();
        for frame in 0..count {
            let cropped = image::imageops::crop_imm(&image, frame * w, 0, w, h).to_image();
            let mut key = Vec::from(w.to_le_bytes());
            key.extend(h.to_le_bytes());
            key.extend(cropped.as_raw());
            let id = digest(&key);
            if !groups.contains_key(&id) {
                let path = format!("frames/{id}.png");
                outputs.insert(
                    path.clone(),
                    png(&DynamicImage::ImageRgba8(cropped.clone()))?,
                );
                groups.insert(
                    id.clone(),
                    Group {
                        id: id.clone(),
                        image: path,
                        size: [w, h],
                        bounds: bounds(&cropped),
                        components: components(&cropped, &colors, &palette),
                        selected: Vec::new(),
                        status: "unreviewed",
                        occurrences: Vec::new(),
                        references: Vec::new(),
                        reference_selections: Vec::new(),
                    },
                );
            }
            let group = groups.get_mut(&id).unwrap();
            if selected.contains(&name) {
                group.occurrences.push(Occurrence {
                    asset: name.clone(),
                    frame,
                });
            }
            if let Some(mask) = &mask {
                let mut selection = Vec::new();
                for (i, component) in group.components.iter().enumerate() {
                    let [x, y] = component.seed;
                    let take = mask[(y * image.width() + frame * w + x) as usize];
                    if take {
                        selection.push(i);
                    }
                    evidence
                        .entry(component.signature.clone())
                        .or_default()
                        .insert(take);
                }
                references.entry(id.clone()).or_default().insert(selection);
                group.references.push(Occurrence {
                    asset: name.clone(),
                    frame,
                });
            }
            frames.push(id);
        }
        if selected.contains(&name) {
            assets.push(json!({"asset":name,"source_sha256":digest(&bytes),"size":[image.width(),image.height()],"frame_size":[w,h],"frames":frames}));
        }
    }
    // Only omitted groups need fixed evidence. Reference choices in the gallery
    // remain replaceable by the reviewer's decisions.
    let mut reference_evidence: BTreeMap<String, BTreeSet<bool>> = BTreeMap::new();
    for (id, choices) in &references {
        let group = &groups[id];
        if !group.occurrences.is_empty() {
            continue;
        }
        for (i, component) in group.components.iter().enumerate() {
            reference_evidence
                .entry(component.signature.clone())
                .or_default()
                .extend(choices.iter().map(|selection| selection.contains(&i)));
        }
    }
    groups.retain(|_, g| !g.occurrences.is_empty());
    for (id, group) in &mut groups {
        if let Some(choices) = references.get(id) {
            if choices.len() == 1 {
                group.selected = choices.first().unwrap().clone();
                group.status = "reused";
            } else {
                group.status = "conflict";
                group.reference_selections = choices.iter().cloned().collect();
            }
        } else {
            group.selected = group
                .components
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    (evidence.get(&c.signature) == Some(&BTreeSet::from([true]))).then_some(i)
                })
                .collect();
            if !group.selected.is_empty() {
                group.status = "suggested";
            }
        }
    }
    outputs.retain(|path, _| groups.values().any(|g| g.image == *path));
    let frame_count: usize = groups.values().map(|g| g.occurrences.len()).sum();
    let reused: usize = groups
        .values()
        .filter(|g| g.status == "reused")
        .map(|g| g.occurrences.len())
        .sum();
    let summary = json!({"strips":assets.len(),"frames":frame_count,"unique_frames":groups.len(),"reused_frames":reused,
        "review_groups":groups.values().filter(|g|g.status != "reused").count(),
        "suggested_groups":groups.values().filter(|g|g.status == "suggested").count(),
        "conflicting_groups":groups.values().filter(|g|g.status == "conflict").count()});
    let mut groups: Vec<_> = groups.into_values().collect();
    groups.sort_by(|a, b| {
        let a = &a.occurrences[0];
        let b = &b.occurrences[0];
        (&a.asset, a.frame).cmp(&(&b.asset, b.frame))
    });
    let mut report = json!({"version":1,"label":config.label,"archive_sha256":file_digest(archive_path)?,
        "source_colors":config.source_colors,"colors":colors,"preview_colors":targets,
        "color_groups":recipe["color_groups"],
        "assets":assets,"groups":groups,"reference_evidence":reference_evidence,"summary":summary});
    report["id"] = json!(digest(&serde_json::to_vec(&report)?));
    outputs.insert("batch.json".into(), json_bytes(&report)?);
    outputs.insert(
        "batch.js".into(),
        format!(
            "window.REVIEW_BATCH = {};\n",
            serde_json::to_string(&report)?
        )
        .into_bytes(),
    );
    outputs.insert(
        "index.html".into(),
        include_bytes!("review/index.html").to_vec(),
    );
    outputs.insert(
        "review.js".into(),
        include_bytes!("review/review.js").to_vec(),
    );
    outputs.insert(
        "review.css".into(),
        include_bytes!("review/review.css").to_vec(),
    );
    write_tree(&output, outputs)?;
    Ok(json!({"summary":summary,"gallery":output.join("index.html")}))
}
