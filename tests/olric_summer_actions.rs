use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/olric and the local accepted Olric world baseline"]
fn olric_summer_actions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/olric");
    let baseline = root.join("local/fixtures/slice-024-build/characters/olric");
    let set = std::env::var_os("FOM_OLRIC_SUMMER_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/olric-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_OLRIC_SUMMER_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/olric-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..95];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..95], prior);
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
    // The drinking elbow uses a second midtone, mapped to the existing midtone target.
    let source = [0xF2D7A7, 0xE4BD7C, 0xC4854C, 0x684810, 0xE8B171];
    // Reviewed landmarks distinguish moving skin from protected materials.
    // Literal per-frame counts guard brief and occluded skin exposure.
    let landmarks = [
        // Every additional drinking-arm midtone pixel, verified on the source grid.
        ("drink_east", 0, 37, 43, 0xE8B171, true),
        ("drink_east", 0, 37, 44, 0xE8B171, true),
        ("drink_east", 0, 39, 44, 0xE8B171, true),
        ("drink_east", 1, 36, 43, 0xE8B171, true),
        ("drink_east", 1, 38, 43, 0xE8B171, true),
        ("drink_east", 1, 36, 44, 0xE8B171, true),
        ("drink_east", 1, 37, 44, 0xE8B171, true),
        ("drink_east", 2, 37, 43, 0xE8B171, true),
        ("drink_east", 2, 37, 44, 0xE8B171, true),
        ("drink_east", 2, 39, 44, 0xE8B171, true),
        ("drink_south", 1, 34, 43, 0xE8B171, true),
        ("drink_east", 0, 42, 45, 0x000000, false),
        ("drink_east", 0, 36, 44, 0x010101, false),
        ("drink_east", 0, 37, 32, 0x342323, false),
        ("drink_east", 0, 41, 44, 0x475A66, false),
        ("drink_east", 0, 41, 29, 0x4E3F3F, false),
        ("drink_east", 0, 38, 47, 0x576069, false),
        ("drink_east", 0, 36, 35, 0x684810, true),
        ("drink_east", 0, 42, 43, 0x6F85A0, false),
        ("drink_east", 0, 43, 30, 0x746161, false),
        ("drink_east", 0, 39, 47, 0x79868B, false),
        ("drink_east", 0, 38, 42, 0x9B3F3F, false),
        ("drink_east", 0, 40, 43, 0x9EB0BB, false),
        ("drink_east", 0, 43, 36, 0xA59DA2, false),
        ("drink_east", 0, 38, 36, 0xC2B9BE, false),
        ("drink_east", 0, 41, 40, 0xC4854C, true),
        ("drink_east", 0, 41, 49, 0xC66659, false),
        ("drink_east", 0, 44, 46, 0xD7D7DB, false),
        ("drink_east", 0, 42, 48, 0xDDA199, false),
        ("drink_east", 0, 44, 45, 0xE4BD7C, true),
        ("drink_east", 0, 45, 48, 0xE67C6D, false),
        ("drink_east", 0, 37, 44, 0xE8B171, true),
        ("drink_east", 0, 43, 37, 0xECF0E9, false),
        ("drink_east", 0, 41, 46, 0xF2D7A7, true),
        ("drink_east", 0, 42, 47, 0xF4F4F4, false),
        ("drink_east", 1, 36, 42, 0xC4854C, true),
        ("drink_east", 1, 44, 45, 0xE4BD7C, true),
        ("drink_east", 1, 41, 46, 0xF2D7A7, true),
        ("drink_north", 1, 45, 43, 0x684810, true),
        ("drink_north", 1, 45, 42, 0xC4854C, true),
        ("drink_north", 1, 40, 42, 0xE4BD7C, true),
        ("drink_south", 1, 35, 43, 0x684810, true),
        ("drink_south", 1, 34, 42, 0xC4854C, true),
        ("drink_south", 1, 40, 42, 0xE4BD7C, true),
        ("eat_east", 1, 40, 43, 0x684810, true),
        ("eat_east", 1, 41, 42, 0xE4BD7C, true),
        ("eat_east", 1, 43, 45, 0xF2D7A7, true),
        ("eat_north", 0, 47, 45, 0x000000, false),
        ("eat_north", 0, 43, 36, 0x342323, false),
        ("eat_north", 0, 35, 45, 0x475A66, false),
        ("eat_north", 0, 36, 32, 0x4E3F3F, false),
        ("eat_north", 0, 42, 46, 0x576069, false),
        ("eat_north", 0, 36, 44, 0x684810, true),
        ("eat_north", 0, 35, 46, 0x6F85A0, false),
        ("eat_north", 0, 41, 32, 0x746161, false),
        ("eat_north", 0, 37, 47, 0x79868B, false),
        ("eat_north", 0, 37, 42, 0x9B3F3F, false),
        ("eat_north", 0, 38, 47, 0x9EB0BB, false),
        ("eat_north", 0, 35, 44, 0xC4854C, true),
        ("eat_north", 0, 41, 44, 0xC66659, false),
        ("eat_north", 0, 37, 43, 0xD7D7DB, false),
        ("eat_north", 0, 41, 45, 0xDDA199, false),
        ("eat_north", 0, 36, 43, 0xE4BD7C, true),
        ("eat_north", 0, 40, 43, 0xE67C6D, false),
        ("eat_north", 0, 44, 44, 0xF2D7A7, true),
        ("eat_north", 0, 39, 44, 0xF4F4F4, false),
        ("eat_north", 1, 45, 43, 0x684810, true),
        ("eat_north", 1, 45, 42, 0xC4854C, true),
        ("eat_north", 1, 40, 42, 0xE4BD7C, true),
        ("eat_south", 1, 42, 40, 0x684810, true),
        ("eat_south", 1, 40, 41, 0xC4854C, true),
        ("eat_south", 1, 43, 44, 0xE4BD7C, true),
        ("eat_south", 1, 36, 46, 0xF2D7A7, true),
        ("sit_south", 0, 46, 47, 0x000000, false),
        ("sit_south", 0, 45, 47, 0x293F4A, false),
        ("sit_south", 0, 45, 31, 0x342323, false),
        ("sit_south", 0, 44, 45, 0x475A66, false),
        ("sit_south", 0, 35, 30, 0x4E3F3F, false),
        ("sit_south", 0, 39, 46, 0x576069, false),
        ("sit_south", 0, 43, 44, 0x684810, true),
        ("sit_south", 0, 35, 46, 0x6F85A0, false),
        ("sit_south", 0, 42, 30, 0x746161, false),
        ("sit_south", 0, 40, 45, 0x79868B, false),
        ("sit_south", 0, 42, 42, 0x9B3F3F, false),
        ("sit_south", 0, 41, 46, 0x9EB0BB, false),
        ("sit_south", 0, 42, 36, 0xC2B9BE, false),
        ("sit_south", 0, 43, 42, 0xC4854C, true),
        ("sit_south", 0, 42, 44, 0xC66659, false),
        ("sit_south", 0, 37, 43, 0xD7D7DB, false),
        ("sit_south", 0, 37, 49, 0xDDA199, false),
        ("sit_south", 0, 43, 43, 0xE4BD7C, true),
        ("sit_south", 0, 43, 47, 0xE67C6D, false),
        ("sit_south", 0, 42, 37, 0xECF0E9, false),
        ("sit_south", 0, 44, 36, 0xF2D7A7, true),
        ("sit_south", 0, 40, 44, 0xF4F4F4, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("drink_east", &[50, 57, 50]),
        ("drink_north", &[30, 25, 30]),
        ("drink_south", &[57, 67, 57]),
        ("eat_east", &[52, 56, 53, 54, 50]),
        ("eat_north", &[30, 25, 30]),
        ("eat_south", &[60, 61, 57, 66, 61]),
        ("sit_east", &[51]),
        ("sit_north", &[24]),
        ("sit_south", &[61]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [8, 9, 10, 11, 9]
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
