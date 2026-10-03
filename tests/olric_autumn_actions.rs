use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/olric and the local accepted Olric world baseline"]
fn olric_autumn_actions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/olric");
    let baseline = root.join("generated/slice-032-build/characters/olric");
    let set = std::env::var_os("FOM_OLRIC_AUTUMN_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/olric-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_OLRIC_AUTUMN_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/olric-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..143];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..143], prior);
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
        ("drink_east", 0, 42, 45, 0x000000, false),
        ("drink_east", 0, 36, 44, 0x010101, false),
        ("drink_east", 0, 37, 43, 0x272646, false),
        ("drink_east", 0, 38, 47, 0x2B395A, false),
        ("drink_east", 0, 37, 32, 0x342323, false),
        ("drink_east", 0, 38, 46, 0x425882, false),
        ("drink_east", 0, 37, 42, 0x47446D, false),
        ("drink_east", 0, 41, 29, 0x4E3F3F, false),
        ("drink_east", 0, 38, 41, 0x583530, false),
        ("drink_east", 0, 44, 47, 0x613B25, false),
        ("drink_east", 0, 36, 35, 0x684810, true),
        ("drink_east", 0, 41, 46, 0x6E89AE, false),
        ("drink_east", 0, 43, 30, 0x746161, false),
        ("drink_east", 0, 40, 44, 0x7F5231, false),
        ("drink_east", 0, 38, 42, 0x84524B, false),
        ("drink_east", 0, 43, 36, 0xA59DA2, false),
        ("drink_east", 0, 44, 48, 0xB7824D, false),
        ("drink_east", 0, 38, 36, 0xC2B9BE, false),
        ("drink_east", 0, 41, 40, 0xC4854C, true),
        ("drink_east", 0, 39, 38, 0xE4BD7C, true),
        ("drink_east", 0, 43, 37, 0xECF0E9, false),
        ("drink_east", 0, 41, 36, 0xF2D7A7, true),
        ("drink_north", 1, 42, 40, 0x684810, true),
        ("drink_north", 1, 41, 40, 0xC4854C, true),
        ("drink_north", 1, 40, 40, 0xE4BD7C, true),
        ("drink_south", 1, 42, 40, 0x684810, true),
        ("drink_south", 1, 39, 41, 0xC4854C, true),
        ("drink_south", 1, 39, 42, 0xE4BD7C, true),
        ("eat_north", 0, 32, 46, 0x000000, false),
        ("eat_north", 0, 44, 43, 0x272646, false),
        ("eat_north", 0, 42, 46, 0x2A1816, false),
        ("eat_north", 0, 43, 44, 0x2B395A, false),
        ("eat_north", 0, 43, 36, 0x342323, false),
        ("eat_north", 0, 42, 44, 0x425882, false),
        ("eat_north", 0, 43, 42, 0x47446D, false),
        ("eat_north", 0, 36, 32, 0x4E3F3F, false),
        ("eat_north", 0, 42, 45, 0x583530, false),
        ("eat_north", 0, 34, 47, 0x613B25, false),
        ("eat_north", 0, 37, 39, 0x684810, true),
        ("eat_north", 0, 40, 43, 0x6E89AE, false),
        ("eat_north", 0, 41, 32, 0x746161, false),
        ("eat_north", 0, 35, 45, 0x7F5231, false),
        ("eat_north", 0, 41, 43, 0x84524B, false),
        ("eat_north", 0, 35, 46, 0xB7824D, false),
        ("eat_north", 0, 41, 39, 0xC4854C, true),
        ("eat_north", 0, 40, 39, 0xE4BD7C, true),
        ("eat_north", 0, 35, 37, 0xF2D7A7, true),
        ("eat_north", 1, 42, 40, 0x684810, true),
        ("eat_north", 1, 41, 40, 0xC4854C, true),
        ("eat_north", 1, 40, 40, 0xE4BD7C, true),
        ("eat_south", 1, 42, 40, 0x684810, true),
        ("eat_south", 1, 39, 41, 0xC4854C, true),
        ("eat_south", 1, 40, 42, 0xE4BD7C, true),
        ("sit_south", 0, 43, 47, 0x000000, false),
        ("sit_south", 0, 38, 48, 0x272646, false),
        ("sit_south", 0, 42, 45, 0x2A1816, false),
        ("sit_south", 0, 43, 44, 0x2B395A, false),
        ("sit_south", 0, 45, 31, 0x342323, false),
        ("sit_south", 0, 40, 44, 0x425882, false),
        ("sit_south", 0, 36, 42, 0x47446D, false),
        ("sit_south", 0, 35, 30, 0x4E3F3F, false),
        ("sit_south", 0, 42, 43, 0x583530, false),
        ("sit_south", 0, 45, 47, 0x613B25, false),
        ("sit_south", 0, 44, 35, 0x684810, true),
        ("sit_south", 0, 39, 44, 0x6E89AE, false),
        ("sit_south", 0, 42, 30, 0x746161, false),
        ("sit_south", 0, 38, 49, 0x7F5231, false),
        ("sit_south", 0, 42, 42, 0x84524B, false),
        ("sit_south", 0, 40, 45, 0xB5ADA0, false),
        ("sit_south", 0, 35, 47, 0xB7824D, false),
        ("sit_south", 0, 42, 36, 0xC2B9BE, false),
        ("sit_south", 0, 40, 40, 0xC4854C, true),
        ("sit_south", 0, 40, 41, 0xE4BD7C, true),
        ("sit_south", 0, 42, 37, 0xECF0E9, false),
        ("sit_south", 0, 40, 42, 0xEFEDE3, false),
        ("sit_south", 0, 44, 36, 0xF2D7A7, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("drink_east", &[41, 47, 41]),
        ("drink_north", &[12, 12, 12]),
        ("drink_south", &[50, 58, 50]),
        ("eat_east", &[41, 42, 42, 46, 43]),
        ("eat_north", &[12, 12, 12]),
        ("eat_south", &[50, 52, 50, 60, 50]),
        ("sit_east", &[43]),
        ("sit_north", &[12]),
        ("sit_south", &[50]),
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
            let prefix = "spr_npc_olric_autumn";
            let asset = format!("assets/animations/NPCs/Olric/Sprites/Autumn/{prefix}_{case}.png");
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
                // All reviewed world shades are skin in these Autumn strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Olric Autumn material mismatch: {id} {case} [{x},{y}]"
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
