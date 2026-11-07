use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: impl AsRef<Path>) -> Value {
    let path = root().join(path);
    serde_json::from_slice(&fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
        .unwrap()
}

fn rgba(text: &str) -> [u8; 4] {
    let hex = text.strip_prefix('#').unwrap();
    assert!(hex.len() == 6 || hex.len() == 8);
    let mut result = [255; 4];
    for (i, channel) in result.iter_mut().enumerate().take(hex.len() / 2) {
        *channel = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap();
    }
    result
}

struct Case {
    id: String,
    contract: Value,
    set: Value,
    profile: Value,
    before: PathBuf,
    after: PathBuf,
}

fn cases() -> Vec<Case> {
    let baseline = read("tests/fixtures/native-tone-face-baseline.json");
    let result: Vec<_> = baseline["characters"]
        .as_object()
        .unwrap()
        .keys()
        .filter(|id| *id != "celine")
        .map(|id| {
            let contract = read(format!(
                "docs/development/native-tone-face-corrections/{id}.json"
            ));
            assert_eq!(contract["character"], *id);
            let status = contract["status"].as_str().unwrap();
            assert!(matches!(status, "corrected" | "audited-no-change"));
            let set = read(format!("palettes/sets/{id}-native-tones.json"));
            let presets = set["presets"].as_array().unwrap();
            assert_eq!(presets.len(), 10, "{id}");
            let selected = presets[4].get("profile").unwrap_or(&set["profile"]);
            for preset in &presets[4..] {
                assert_eq!(preset.get("profile").unwrap_or(&set["profile"]), selected);
            }
            let profile = read(Path::new("palettes/sets").join(selected.as_str().unwrap()));
            let before = root()
                .join("generated/native-tone-build/characters")
                .join(id);
            let after = if let Some(path) = std::env::var_os("FOM_ROSTER_FACE_BUILD") {
                root().join(path).join(id)
            } else if id == "holt" {
                root().join("generated/holt-moustache-edge")
            } else if id == "adeline" {
                root().join("generated/adeline-holt-face-followup").join(id)
            } else {
                let candidate = root().join("generated/native-tone-roster-face").join(id);
                if status == "audited-no-change" && !candidate.exists() {
                    before.clone()
                } else {
                    candidate
                }
            };
            Case {
                id: id.clone(),
                contract,
                set,
                profile,
                before,
                after,
            }
        })
        .collect();
    assert_eq!(result.len(), 35);
    result
}

#[test]
#[ignore = "requires retained original PNGs, reviewed facial contracts and current roster candidates"]
fn reviewed_blush_mouth_and_protected_landmarks_match_all_six_native_tones() {
    let mut failures = Vec::new();
    let (mut corrected, mut landmarks, mut protected) = (0, 0, 0);
    let mut roles = BTreeSet::new();
    for case in cases() {
        let points = case.contract["landmarks"].as_array().unwrap();
        let edge_skin = case.contract["edge_skin_landmarks"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let hair = case.contract["edge_protected_hair_landmarks"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        if !edge_skin.is_empty() || !hair.is_empty() {
            assert_eq!(case.id, "holt");
            assert_eq!(edge_skin.len(), 304);
            assert_eq!(hair.len(), 16);
        }
        if case.contract["status"] == "corrected" {
            corrected += 1;
            assert!(!points.is_empty(), "{} has no correction landmark", case.id);
        }
        for (points, preserve, all_choices) in [
            (points.as_slice(), false, false),
            (
                case.contract["protected_landmarks"]
                    .as_array()
                    .unwrap()
                    .as_slice(),
                true,
                false,
            ),
            (edge_skin, false, true),
            (hair, true, true),
        ] {
            for point in points {
                let asset = point["asset"].as_str().unwrap();
                let x = point["point"][0].as_u64().unwrap() as u32;
                let y = point["point"][1].as_u64().unwrap() as u32;
                let color = rgba(point["source"].as_str().unwrap());
                let original = image::open(case.before.join("original").join(asset))
                    .unwrap()
                    .to_rgba8();
                assert_eq!(
                    original.get_pixel(x, y).0,
                    color,
                    "{} source {asset} [{x},{y}]",
                    case.id
                );
                let role = point["role"].as_str().unwrap();
                let presets = case.set["presets"].as_array().unwrap();
                for preset in if all_choices {
                    presets.as_slice()
                } else {
                    &presets[4..]
                } {
                    let id = preset["id"].as_str().unwrap();
                    let expected = if preserve {
                        color
                    } else {
                        rgba(point["expected"][id].as_str().unwrap())
                    };
                    let actual = image::open(case.after.join("variants").join(id).join(asset))
                        .unwrap()
                        .to_rgba8();
                    if actual.get_pixel(x, y).0 != expected {
                        failures.push(format!(
                            "{} {id} {role} {asset} [{x},{y}]: {:?}, expected {expected:?}",
                            case.id,
                            actual.get_pixel(x, y).0
                        ));
                    }
                    if preserve {
                        protected += 1;
                    } else {
                        landmarks += 1;
                    }
                }
                if !preserve {
                    roles.insert(role.to_owned());
                }
            }
        }
    }
    assert!(corrected > 0 && landmarks > 0 && protected > 0);
    assert!(roles.contains("blush"));
    assert!(roles.contains("mouth_skin") || roles.contains("outer_lip"));
    println!(
        "35 dispositions: {corrected} corrected; {landmarks} literal correction and {protected} protected-material checks across six added tones and all ten Holt edge choices; {} failures",
        failures.len()
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn reviewed_selection(
    case: &Case,
    asset: &str,
    original: &image::RgbaImage,
    region: Option<&Value>,
) -> Vec<Option<usize>> {
    let (width, height) = original.dimensions();
    let mut selected = vec![None; (width * height) as usize];
    let mut roles = BTreeMap::new();
    let Some(region) = region else {
        return selected;
    };
    assert_eq!(
        region["size"],
        serde_json::json!([width, height]),
        "{} {asset}",
        case.id
    );
    let sources = case.profile["source_colors"].as_array().unwrap();
    for component in region["components"].as_array().unwrap() {
        let expected_counts: BTreeMap<[u8; 4], u64> = if let Some(colors) = component.get("colors")
        {
            colors
                .as_object()
                .unwrap()
                .iter()
                .map(|(color, count)| (rgba(color), count.as_u64().unwrap()))
                .collect()
        } else {
            BTreeMap::from([(
                rgba(component["color"].as_str().unwrap()),
                component["pixels"].as_u64().unwrap(),
            )])
        };
        if let Some(color) = component.get("color") {
            assert!(expected_counts.contains_key(&rgba(color.as_str().unwrap())));
        }
        let x = component["seed"][0].as_u64().unwrap() as u32;
        let y = component["seed"][1].as_u64().unwrap() as u32;
        assert!(x < width && y < height);
        let seed = y * width + x;
        let mut seen = BTreeSet::from([seed]);
        let mut pending = vec![seed];
        let mut actual_counts = BTreeMap::new();
        while let Some(index) = pending.pop() {
            let (x, y) = (index % width, index / width);
            let color = original.get_pixel(x, y).0;
            assert!(
                expected_counts.contains_key(&color),
                "{} seed outside reviewed colors {asset}",
                case.id
            );
            *actual_counts.entry(color).or_insert(0u64) += 1;
            let slot = sources
                .iter()
                .position(|source| rgba(source.as_str().unwrap()) == color)
                .unwrap();
            let index_usize = index as usize;
            if let Some(old_slot) = selected[index_usize].replace(slot) {
                assert_eq!(old_slot, slot);
            }
            let role = component["role"].as_str().unwrap();
            if let Some(old_role) = roles.insert(index_usize, role) {
                assert_eq!(
                    old_role, role,
                    "{} overlapping material roles {asset} [{x},{y}]",
                    case.id
                );
            }
            for (nx, ny) in [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ] {
                let next = ny.wrapping_mul(width).wrapping_add(nx);
                if nx < width
                    && ny < height
                    && expected_counts.contains_key(&original.get_pixel(nx, ny).0)
                    && seen.insert(next)
                {
                    pending.push(next);
                }
            }
        }
        assert_eq!(
            actual_counts, expected_counts,
            "{} component counts {asset} seed [{x},{y}]",
            case.id
        );
        assert_eq!(seen.len() as u64, component["pixels"].as_u64().unwrap());
    }
    selected
}

#[test]
#[ignore = "requires the complete retained 35-character original and generated PNG corpora"]
fn complete_roster_delta_is_confined_to_reviewed_facial_components() {
    let (
        mut strips,
        mut old_exact,
        mut world_exact,
        mut pngs,
        mut metadata,
        mut changed_pngs,
        mut changed_pixels,
        mut decoded_pixels,
        mut restored_pngs,
        mut restored_pixels,
    ) = (0, 0, 0, 0, 0, 0, 0u64, 0u64, 0, 0u64);
    for case in cases() {
        let mut regions = BTreeMap::new();
        for region in case.contract["regions"].as_array().unwrap() {
            let asset = region["asset"].as_str().unwrap();
            assert!(
                asset.contains("/Portraits/"),
                "{} reviewed world change",
                case.id
            );
            assert!(regions.insert(asset, region).is_none());
        }
        let mut restorations = BTreeMap::new();
        if let Some(regions) = case.contract["restored_components"].as_array() {
            assert_eq!(case.id, "holt");
            assert!(!regions.is_empty());
            for region in regions {
                let asset = region["asset"].as_str().unwrap();
                assert!(asset.contains("/Holt/Portraits/"));
                for component in region["components"].as_array().unwrap() {
                    assert_eq!(component["role"], "moustache");
                }
                assert!(restorations.insert(asset, region).is_none());
            }
        }
        let mut character_changes = 0;
        for region in case.profile["regions"].as_array().unwrap() {
            strips += 1;
            let asset = region["asset"].as_str().unwrap();
            let original_bytes = fs::read(case.before.join("original").join(asset)).unwrap();
            let hash = format!("{:x}", Sha256::digest(&original_bytes));
            assert_eq!(
                hash,
                region["source_sha256"].as_str().unwrap(),
                "{} {asset}",
                case.id
            );
            let reviewed = regions.remove(asset);
            if let Some(reviewed) = reviewed {
                assert_eq!(reviewed["source_sha256"], region["source_sha256"]);
            }
            let original = image::load_from_memory(&original_bytes).unwrap().to_rgba8();
            let selected = reviewed_selection(&case, asset, &original, reviewed);
            let restored = restorations.remove(asset);
            if let Some(restored) = restored {
                assert_eq!(restored["source_sha256"], region["source_sha256"]);
            }
            let restored = reviewed_selection(&case, asset, &original, restored);
            assert!(
                selected
                    .iter()
                    .zip(&restored)
                    .all(|(a, b)| a.is_none() || b.is_none())
            );
            let has_restoration = restored.iter().any(Option::is_some);
            let source_meta = fs::read(
                case.before
                    .join("original")
                    .join(asset)
                    .with_extension("meta.toml"),
            )
            .unwrap();
            for (index, preset) in case.set["presets"].as_array().unwrap().iter().enumerate() {
                let id = preset["id"].as_str().unwrap();
                let before = case.before.join("variants").join(id).join(asset);
                let after = case.after.join("variants").join(id).join(asset);
                let old_bytes = fs::read(&before).unwrap();
                let new_bytes = fs::read(&after).unwrap();
                assert_eq!(
                    fs::read(before.with_extension("meta.toml")).unwrap(),
                    source_meta
                );
                assert_eq!(
                    fs::read(after.with_extension("meta.toml")).unwrap(),
                    source_meta,
                    "{} {id} {asset} metadata",
                    case.id
                );
                pngs += 1;
                metadata += 1;
                if (index < 4
                    || !asset.contains("/Portraits/")
                    || selected.iter().all(Option::is_none))
                    && !has_restoration
                {
                    assert_eq!(
                        old_bytes, new_bytes,
                        "{} {id} {asset} must remain byte-exact",
                        case.id
                    );
                    if index < 4 {
                        old_exact += 1;
                    } else if !asset.contains("/Portraits/") {
                        world_exact += 1;
                    }
                    continue;
                }
                let old = image::load_from_memory(&old_bytes).unwrap().to_rgba8();
                let new = image::load_from_memory(&new_bytes).unwrap().to_rgba8();
                assert_eq!(old.dimensions(), original.dimensions());
                assert_eq!(new.dimensions(), original.dimensions());
                let targets: Vec<_> = preset["colors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|color| rgba(color.as_str().unwrap()))
                    .collect();
                for (pixel, ((old, new), original)) in old
                    .pixels()
                    .zip(new.pixels())
                    .zip(original.pixels())
                    .enumerate()
                {
                    assert_eq!(
                        new[3], original[3],
                        "{} {id} {asset} alpha at {pixel}",
                        case.id
                    );
                    assert_eq!(old[3], new[3]);
                    decoded_pixels += 1;
                    if restored[pixel].is_some() {
                        assert_eq!(
                            new, original,
                            "{} {id} {asset} restored moustache at {pixel}",
                            case.id
                        );
                        restored_pixels += u64::from(old != new);
                    } else if let Some(slot) = selected[pixel].filter(|_| index >= 4) {
                        assert_eq!(
                            new.0, targets[slot],
                            "{} {id} {asset} reviewed pixel {pixel}",
                            case.id
                        );
                    } else {
                        assert_eq!(old, new, "{} {id} {asset} unrelated pixel {pixel}", case.id);
                    }
                    changed_pixels += u64::from(old != new);
                }
                if old_bytes != new_bytes {
                    changed_pngs += 1;
                    character_changes += 1;
                    if index < 4 {
                        assert_eq!(case.id, "holt");
                        assert!(has_restoration);
                        restored_pngs += 1;
                    }
                }
            }
        }
        assert!(
            regions.is_empty(),
            "{} contract asset outside selected inventory",
            case.id
        );
        assert!(
            restorations.is_empty(),
            "{} unknown restored asset",
            case.id
        );
        if case.contract["status"] == "corrected" {
            assert!(
                character_changes > 0,
                "{} correction has no changed PNG",
                case.id
            );
        } else {
            assert_eq!(character_changes, 0, "{} audited unchanged", case.id);
        }
    }
    assert!(changed_pngs > 0 && changed_pixels > 0);
    assert_eq!(restored_pngs, 64);
    assert_eq!(restored_pixels, 5_760);
    println!(
        "35 characters / {strips} strips / {pngs} PNGs and {metadata} metadata: {old_exact} historical and {world_exact} new-tone world PNGs byte-exact; {restored_pngs} old-choice Holt PNGs change only at restored moustache pixels; {restored_pixels} restored moustache pixels across all choices; {changed_pngs} corrected PNGs / {changed_pixels} changed facial pixels; alpha and unrelated materials exact across {decoded_pixels} decoded portrait pixels"
    );
}

#[test]
#[ignore = "requires the frozen first roster correction and initial generated PNG corpora"]
fn first_roster_correction_preserves_all_prior_four_choice_pngs() {
    let mut checked = 0;
    for case in cases() {
        let frozen = root()
            .join("generated/native-tone-roster-face")
            .join(&case.id);
        for preset in &case.set["presets"].as_array().unwrap()[..4] {
            let id = preset["id"].as_str().unwrap();
            for region in case.profile["regions"].as_array().unwrap() {
                let asset = region["asset"].as_str().unwrap();
                assert_eq!(
                    fs::read(case.before.join("variants").join(id).join(asset)).unwrap(),
                    fs::read(frozen.join("variants").join(id).join(asset)).unwrap(),
                    "{} {id} {asset} frozen first correction",
                    case.id
                );
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 23_688);
}
