use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/olric and the local accepted Olric world baseline"]
fn olric_autumn_write_sweep_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/olric");
    let baseline = root.join("local/fixtures/slice-037-build/characters/olric");
    let set = std::env::var_os("FOM_OLRIC_AUTUMN_WRITE_SWEEP_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/olric-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_OLRIC_AUTUMN_WRITE_SWEEP_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/olric-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..169];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..169], prior);
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
        ("sweep_end_south", 0, 44, 47, 0x000000, false),
        ("sweep_end_south", 0, 37, 50, 0x272646, false),
        ("sweep_end_south", 0, 37, 49, 0x2B395A, false),
        ("sweep_end_south", 0, 45, 31, 0x342323, false),
        ("sweep_end_south", 0, 44, 45, 0x425882, false),
        ("sweep_end_south", 0, 35, 42, 0x47446D, false),
        ("sweep_end_south", 0, 35, 30, 0x4E3F3F, false),
        ("sweep_end_south", 0, 37, 41, 0x583530, false),
        ("sweep_end_south", 0, 41, 46, 0x613B25, false),
        ("sweep_end_south", 0, 41, 52, 0x653D25, false),
        ("sweep_end_south", 0, 35, 35, 0x684810, true),
        ("sweep_end_south", 0, 44, 44, 0x6E89AE, false),
        ("sweep_end_south", 0, 42, 30, 0x746161, false),
        ("sweep_end_south", 0, 41, 45, 0x7F5231, false),
        ("sweep_end_south", 0, 37, 42, 0x84524B, false),
        ("sweep_end_south", 0, 45, 50, 0x936244, false),
        ("sweep_end_south", 0, 39, 45, 0xB5ADA0, false),
        ("sweep_end_south", 0, 36, 45, 0xB7824D, false),
        ("sweep_end_south", 0, 40, 51, 0xBB8151, false),
        ("sweep_end_south", 0, 40, 40, 0xC4854C, true),
        ("sweep_end_south", 0, 40, 41, 0xE4BD7C, true),
        ("sweep_end_south", 0, 46, 53, 0xE9B03A, false),
        ("sweep_end_south", 0, 40, 42, 0xEFEDE3, false),
        ("sweep_end_south", 0, 38, 36, 0xF2D7A7, true),
        ("sweep_end_south", 0, 42, 55, 0xFFE36F, false),
        ("sweep_end_south", 1, 40, 40, 0xC4854C, true),
        ("sweep_end_south", 1, 40, 41, 0xE4BD7C, true),
        ("sweep_start_south", 1, 40, 40, 0xC4854C, true),
        ("sweep_start_south", 1, 40, 41, 0xE4BD7C, true),
        ("write_end_south", 0, 36, 46, 0x000000, false),
        ("write_end_south", 0, 38, 50, 0x272646, false),
        ("write_end_south", 0, 37, 46, 0x2A1816, false),
        ("write_end_south", 0, 36, 44, 0x2B395A, false),
        ("write_end_south", 0, 37, 32, 0x342323, false),
        ("write_end_south", 0, 33, 42, 0x373357, false),
        ("write_end_south", 0, 40, 43, 0x425882, false),
        ("write_end_south", 0, 38, 51, 0x47446D, false),
        ("write_end_south", 0, 41, 29, 0x4E3F3F, false),
        ("write_end_south", 0, 43, 47, 0x57342B, false),
        ("write_end_south", 0, 42, 43, 0x583530, false),
        ("write_end_south", 0, 42, 52, 0x613B25, false),
        ("write_end_south", 0, 44, 35, 0x684810, true),
        ("write_end_south", 0, 33, 43, 0x6B5C7C, false),
        ("write_end_south", 0, 38, 44, 0x6E89AE, false),
        ("write_end_south", 0, 43, 30, 0x746161, false),
        ("write_end_south", 0, 44, 46, 0x7D4F3D, false),
        ("write_end_south", 0, 38, 52, 0x7F5231, false),
        ("write_end_south", 0, 45, 44, 0x837CA0, false),
        ("write_end_south", 0, 42, 42, 0x84524B, false),
        ("write_end_south", 0, 46, 44, 0xA8A5B5, false),
        ("write_end_south", 0, 39, 45, 0xB5ADA0, false),
        ("write_end_south", 0, 42, 49, 0xB7824D, false),
        ("write_end_south", 0, 41, 40, 0xC4854C, true),
        ("write_end_south", 0, 42, 45, 0xC5895C, false),
        ("write_end_south", 0, 33, 45, 0xE3779D, false),
        ("write_end_south", 0, 40, 41, 0xE4BD7C, true),
        ("write_end_south", 0, 32, 39, 0xE6B944, false),
        ("write_end_south", 0, 40, 45, 0xEFEDE3, false),
        ("write_end_south", 0, 39, 36, 0xF2D7A7, true),
        ("write_end_south", 0, 47, 43, 0xF5F5F5, false),
        ("write_end_south", 1, 42, 40, 0x684810, true),
        ("write_end_south", 1, 39, 41, 0xC4854C, true),
        ("write_end_south", 1, 39, 42, 0xE4BD7C, true),
        ("write_loop_south", 1, 41, 40, 0xC4854C, true),
        ("write_loop_south", 1, 40, 41, 0xE4BD7C, true),
        ("write_start_south", 0, 33, 48, 0x000000, false),
        ("write_start_south", 0, 38, 51, 0x272646, false),
        ("write_start_south", 0, 37, 47, 0x2A1816, false),
        ("write_start_south", 0, 36, 45, 0x2B395A, false),
        ("write_start_south", 0, 45, 32, 0x342323, false),
        ("write_start_south", 0, 34, 45, 0x373357, false),
        ("write_start_south", 0, 37, 45, 0x425882, false),
        ("write_start_south", 0, 35, 43, 0x47446D, false),
        ("write_start_south", 0, 35, 31, 0x4E3F3F, false),
        ("write_start_south", 0, 46, 47, 0x57342B, false),
        ("write_start_south", 0, 37, 46, 0x583530, false),
        ("write_start_south", 0, 42, 52, 0x613B25, false),
        ("write_start_south", 0, 42, 40, 0x684810, true),
        ("write_start_south", 0, 34, 46, 0x6B5C7C, false),
        ("write_start_south", 0, 38, 45, 0x6E89AE, false),
        ("write_start_south", 0, 42, 31, 0x746161, false),
        ("write_start_south", 0, 47, 46, 0x7D4F3D, false),
        ("write_start_south", 0, 38, 52, 0x7F5231, false),
        ("write_start_south", 0, 48, 44, 0x837CA0, false),
        ("write_start_south", 0, 38, 46, 0x84524B, false),
        ("write_start_south", 0, 49, 44, 0xA8A5B5, false),
        ("write_start_south", 0, 40, 47, 0xB5ADA0, false),
        ("write_start_south", 0, 45, 49, 0xB7824D, false),
        ("write_start_south", 0, 39, 41, 0xC4854C, true),
        ("write_start_south", 0, 45, 45, 0xC5895C, false),
        ("write_start_south", 0, 34, 48, 0xE3779D, false),
        ("write_start_south", 0, 39, 42, 0xE4BD7C, true),
        ("write_start_south", 0, 33, 42, 0xE6B944, false),
        ("write_start_south", 0, 40, 46, 0xEFEDE3, false),
        ("write_start_south", 0, 39, 37, 0xF2D7A7, true),
        ("write_start_south", 0, 50, 43, 0xF5F5F5, false),
        ("write_start_south", 1, 41, 40, 0xC4854C, true),
        ("write_start_south", 1, 40, 41, 0xE4BD7C, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("sweep_end_south", &[50, 43, 50]),
        ("sweep_loop_south", &[47, 51, 49, 51, 47, 51, 49, 51, 47]),
        ("sweep_start_south", &[50, 43, 50]),
        ("write_end_south", &[53, 60]),
        ("write_loop_south", &[48, 48, 48, 48]),
        ("write_start_south", &[60, 53]),
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
            let prefix = "spr_npc_olric_specialanimation_autumn";
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
