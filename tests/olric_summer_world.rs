use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/olric and the local accepted Olric world baseline"]
fn olric_summer_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/olric");
    let baseline = root.join("generated/slice-023-build/characters/olric");
    let set = std::env::var_os("FOM_OLRIC_SUMMER_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/olric-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_OLRIC_SUMMER_WORLD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/olric-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..87];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..87], prior);
    assert_eq!(candidate["source_colors"], profile["source_colors"]);
    assert_eq!(candidate["color_groups"], profile["color_groups"]);
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("bundle");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["build-presets", "--original"])
        .arg(&original)
        .arg("--presets")
        .arg(&set)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let source = [0xF2D7A7, 0xE4BD7C, 0xC4854C, 0x684810];
    // Reviewed landmarks distinguish moving skin from protected materials.
    // Literal per-frame counts guard brief and occluded skin exposure.
    let landmarks = [
        ("blink_east", 0, 36, 48, 0x000000, false),
        ("blink_east", 0, 35, 47, 0x293F4A, false),
        ("blink_east", 0, 37, 32, 0x342323, false),
        ("blink_east", 0, 44, 45, 0x475A66, false),
        ("blink_east", 0, 41, 29, 0x4E3F3F, false),
        ("blink_east", 0, 37, 48, 0x576069, false),
        ("blink_east", 0, 37, 44, 0x684810, true),
        ("blink_east", 0, 45, 46, 0x6F85A0, false),
        ("blink_east", 0, 43, 30, 0x746161, false),
        ("blink_east", 0, 38, 48, 0x79868B, false),
        ("blink_east", 0, 43, 42, 0x9B3F3F, false),
        ("blink_east", 0, 39, 48, 0x9EB0BB, false),
        ("blink_east", 0, 43, 37, 0xA59DA2, false),
        ("blink_east", 0, 38, 37, 0xC2B9BE, false),
        ("blink_east", 0, 43, 43, 0xC4854C, true),
        ("blink_east", 0, 43, 44, 0xC66659, false),
        ("blink_east", 0, 38, 43, 0xD7D7DB, false),
        ("blink_east", 0, 39, 52, 0xDDA199, false),
        ("blink_east", 0, 41, 41, 0xE4BD7C, true),
        ("blink_east", 0, 41, 52, 0xE67C6D, false),
        ("blink_east", 0, 39, 50, 0xF2D7A7, true),
        ("blink_east", 0, 40, 45, 0xF4F4F4, false),
        ("blink_east", 1, 37, 44, 0x684810, true),
        ("blink_east", 1, 43, 43, 0xC4854C, true),
        ("blink_east", 1, 41, 41, 0xE4BD7C, true),
        ("blink_east", 1, 39, 50, 0xF2D7A7, true),
        ("blink_south", 1, 43, 44, 0x684810, true),
        ("blink_south", 1, 35, 43, 0xC4854C, true),
        ("blink_south", 1, 43, 43, 0xE4BD7C, true),
        ("blink_south", 1, 38, 50, 0xF2D7A7, true),
        ("idle_south", 0, 43, 48, 0x000000, false),
        ("idle_south", 0, 46, 47, 0x293F4A, false),
        ("idle_south", 0, 45, 31, 0x342323, false),
        ("idle_south", 0, 45, 45, 0x475A66, false),
        ("idle_south", 0, 35, 30, 0x4E3F3F, false),
        ("idle_south", 0, 39, 49, 0x576069, false),
        ("idle_south", 0, 43, 44, 0x684810, true),
        ("idle_south", 0, 32, 47, 0x6F85A0, false),
        ("idle_south", 0, 42, 30, 0x746161, false),
        ("idle_south", 0, 37, 48, 0x79868B, false),
        ("idle_south", 0, 42, 42, 0x9B3F3F, false),
        ("idle_south", 0, 40, 47, 0x9EB0BB, false),
        ("idle_south", 0, 42, 36, 0xC2B9BE, false),
        ("idle_south", 0, 35, 43, 0xC4854C, true),
        ("idle_south", 0, 42, 44, 0xC66659, false),
        ("idle_south", 0, 42, 43, 0xD7D7DB, false),
        ("idle_south", 0, 37, 52, 0xDDA199, false),
        ("idle_south", 0, 43, 43, 0xE4BD7C, true),
        ("idle_south", 0, 43, 47, 0xE67C6D, false),
        ("idle_south", 0, 42, 37, 0xECF0E9, false),
        ("idle_south", 0, 38, 50, 0xF2D7A7, true),
        ("idle_south", 0, 39, 45, 0xF4F4F4, false),
        ("walk_east", 1, 39, 51, 0x684810, true),
        ("walk_east", 1, 37, 44, 0xC4854C, true),
        ("walk_east", 1, 41, 42, 0xE4BD7C, true),
        ("walk_east", 1, 44, 44, 0xF2D7A7, true),
        ("walk_north", 1, 42, 40, 0x684810, true),
        ("walk_north", 1, 40, 41, 0xC4854C, true),
        ("walk_north", 1, 36, 43, 0xE4BD7C, true),
        ("walk_north", 1, 35, 45, 0xF2D7A7, true),
        ("walk_south", 0, 43, 48, 0x000000, false),
        ("walk_south", 0, 46, 47, 0x293F4A, false),
        ("walk_south", 0, 45, 31, 0x342323, false),
        ("walk_south", 0, 45, 45, 0x475A66, false),
        ("walk_south", 0, 35, 30, 0x4E3F3F, false),
        ("walk_south", 0, 39, 49, 0x576069, false),
        ("walk_south", 0, 43, 44, 0x684810, true),
        ("walk_south", 0, 32, 47, 0x6F85A0, false),
        ("walk_south", 0, 42, 30, 0x746161, false),
        ("walk_south", 0, 37, 48, 0x79868B, false),
        ("walk_south", 0, 42, 42, 0x9B3F3F, false),
        ("walk_south", 0, 40, 47, 0x9EB0BB, false),
        ("walk_south", 0, 42, 36, 0xC2B9BE, false),
        ("walk_south", 0, 35, 43, 0xC4854C, true),
        ("walk_south", 0, 42, 44, 0xC66659, false),
        ("walk_south", 0, 42, 43, 0xD7D7DB, false),
        ("walk_south", 0, 37, 52, 0xDDA199, false),
        ("walk_south", 0, 43, 43, 0xE4BD7C, true),
        ("walk_south", 0, 43, 47, 0xE67C6D, false),
        ("walk_south", 0, 42, 37, 0xECF0E9, false),
        ("walk_south", 0, 38, 50, 0xF2D7A7, true),
        ("walk_south", 0, 39, 45, 0xF4F4F4, false),
        ("walk_south", 1, 42, 40, 0x684810, true),
        ("walk_south", 1, 40, 41, 0xC4854C, true),
        ("walk_south", 1, 43, 43, 0xE4BD7C, true),
        ("walk_south", 1, 41, 51, 0xF2D7A7, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[59, 65, 59]),
        ("blink_south", &[72, 78, 72]),
        ("idle_east", &[55]),
        ("idle_north", &[34]),
        ("idle_south", &[68]),
        ("walk_east", &[55, 60, 55, 51]),
        ("walk_north", &[34, 27, 34, 27]),
        ("walk_south", &[68, 63, 68, 63]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [8, 9, 10, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_olric_summer";
            let asset = format!("assets/animations/NPCs/Olric/Sprites/Summer/{prefix}_{case}.png");
            let before = image::open(original.join(&asset)).unwrap().to_rgba8();
            let after = image::open(variant.join(&asset)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), (80 * frames, 80));
            assert_eq!(before.dimensions(), after.dimensions());
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(variant.join(meta)).unwrap()
            );
            let mut per_frame = vec![0; frames as usize];
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                // All reviewed world shades are skin in these Summer strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Olric Summer material mismatch: {id} {case} [{x},{y}]"
                );
                mask.push(p != q);
                if p != q {
                    let index = source
                        .iter()
                        .position(|c| rgba(*c) == p.0)
                        .expect("non-skin material changed");
                    assert_eq!(q.0, rgba(target[index]));
                    per_frame[x as usize / 80] += 1;
                }
            }
            assert!(per_frame.iter().all(|n| *n > 0), "empty frame in {case}");
            assert_eq!(per_frame, expected_per_frame, "{case}");
            for &(name, frame, x, y, color, skin) in &landmarks {
                if name != case {
                    continue;
                }
                let x = x + frame * 80;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    rgba(color),
                    "source {case} [{x},{y}]"
                );
                let expected = if skin {
                    target[source.iter().position(|c| *c == color).unwrap()]
                } else {
                    color
                };
                assert_eq!(
                    after.get_pixel(x, y).0,
                    rgba(expected),
                    "{id} {case} [{x},{y}] skin={skin}"
                );
            }
        }
        if let Some(first) = &common_mask {
            assert_eq!(first, &mask);
        } else {
            common_mask = Some(mask);
        }
        for region in prior {
            let asset = region["asset"].as_str().unwrap();
            for file in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(
                    fs::read(variant.join(&file)).unwrap(),
                    fs::read(baseline.join("variants").join(id).join(&file)).unwrap(),
                    "previous output changed: {id}/{file}"
                );
            }
        }
    }
    for region in prior {
        let asset = region["asset"].as_str().unwrap();
        for file in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
            assert_eq!(
                fs::read(original.join(&file)).unwrap(),
                fs::read(baseline.join("original").join(&file)).unwrap(),
                "previous source changed: {file}"
            );
        }
    }
}
