use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/olric and the local accepted Olric world baseline"]
fn olric_spring_tools_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/olric");
    let baseline = root.join("local/fixtures/slice-018-build/characters/olric");
    let set = std::env::var_os("FOM_OLRIC_SPRING_TOOLS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/olric-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_OLRIC_SPRING_TOOLS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/olric-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..59];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..59], prior);
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
        ("hammer_east", 0, 47, 47, 0x000000, false),
        ("hammer_east", 0, 40, 48, 0x1C2613, false),
        ("hammer_east", 0, 38, 49, 0x303D27, false),
        ("hammer_east", 0, 51, 46, 0x305064, false),
        ("hammer_east", 0, 37, 32, 0x342323, false),
        ("hammer_east", 0, 43, 47, 0x4A3429, false),
        ("hammer_east", 0, 41, 29, 0x4E3F3F, false),
        ("hammer_east", 0, 41, 50, 0x505938, false),
        ("hammer_east", 0, 52, 46, 0x6099A8, false),
        ("hammer_east", 0, 38, 45, 0x662B2F, false),
        ("hammer_east", 0, 43, 46, 0x665347, false),
        ("hammer_east", 0, 36, 35, 0x684810, true),
        ("hammer_east", 0, 43, 30, 0x746161, false),
        ("hammer_east", 0, 44, 50, 0x798459, false),
        ("hammer_east", 0, 51, 42, 0x85CED4, false),
        ("hammer_east", 0, 48, 45, 0x936244, false),
        ("hammer_east", 0, 41, 43, 0x9B3F3F, false),
        ("hammer_east", 0, 42, 46, 0xA08A6F, false),
        ("hammer_east", 0, 38, 43, 0xA49CA1, false),
        ("hammer_east", 0, 43, 36, 0xA59DA2, false),
        ("hammer_east", 0, 47, 45, 0xBB8151, false),
        ("hammer_east", 0, 38, 36, 0xC2B9BE, false),
        ("hammer_east", 0, 41, 40, 0xC4854C, true),
        ("hammer_east", 0, 38, 53, 0xC66659, false),
        ("hammer_east", 0, 38, 42, 0xD7D7DB, false),
        ("hammer_east", 0, 46, 45, 0xE3B57F, false),
        ("hammer_east", 0, 41, 41, 0xE4BD7C, true),
        ("hammer_east", 0, 43, 37, 0xECF0E9, false),
        ("hammer_east", 0, 44, 44, 0xF2D7A7, true),
        ("hammer_east", 0, 39, 43, 0xF4F4F4, false),
        ("hammer_east", 1, 39, 40, 0x684810, true),
        ("hammer_east", 1, 33, 41, 0xC4854C, true),
        ("hammer_east", 1, 38, 41, 0xF2D7A7, true),
        ("pickaxe_east", 1, 39, 40, 0x684810, true),
        ("pickaxe_east", 1, 33, 41, 0xC4854C, true),
        ("pickaxe_east", 1, 38, 41, 0xF2D7A7, true),
        ("tongs_end_east", 0, 37, 49, 0x000000, false),
        ("tongs_end_east", 0, 38, 51, 0x1C2613, false),
        ("tongs_end_east", 0, 44, 49, 0x2E3D43, false),
        ("tongs_end_east", 0, 38, 33, 0x342323, false),
        ("tongs_end_east", 0, 40, 48, 0x4A3429, false),
        ("tongs_end_east", 0, 42, 30, 0x4E3F3F, false),
        ("tongs_end_east", 0, 41, 50, 0x505938, false),
        ("tongs_end_east", 0, 47, 47, 0x606C83, false),
        ("tongs_end_east", 0, 42, 45, 0x662B2F, false),
        ("tongs_end_east", 0, 38, 47, 0x665347, false),
        ("tongs_end_east", 0, 39, 40, 0x684810, true),
        ("tongs_end_east", 0, 44, 31, 0x746161, false),
        ("tongs_end_east", 0, 43, 50, 0x798459, false),
        ("tongs_end_east", 0, 42, 44, 0x9B3F3F, false),
        ("tongs_end_east", 0, 41, 47, 0xA08A6F, false),
        ("tongs_end_east", 0, 38, 44, 0xA49CA1, false),
        ("tongs_end_east", 0, 44, 37, 0xA59DA2, false),
        ("tongs_end_east", 0, 39, 37, 0xC2B9BE, false),
        ("tongs_end_east", 0, 42, 41, 0xC4854C, true),
        ("tongs_end_east", 0, 39, 53, 0xC66659, false),
        ("tongs_end_east", 0, 41, 43, 0xD7D7DB, false),
        ("tongs_end_east", 0, 41, 42, 0xE4BD7C, true),
        ("tongs_end_east", 0, 44, 38, 0xECF0E9, false),
        ("tongs_end_east", 0, 42, 37, 0xF2D7A7, true),
        ("tongs_end_east", 0, 38, 43, 0xF4F4F4, false),
        ("tongs_start_east", 0, 37, 49, 0x000000, false),
        ("tongs_start_east", 0, 38, 51, 0x1C2613, false),
        ("tongs_start_east", 0, 44, 49, 0x2E3D43, false),
        ("tongs_start_east", 0, 38, 33, 0x342323, false),
        ("tongs_start_east", 0, 40, 48, 0x4A3429, false),
        ("tongs_start_east", 0, 42, 30, 0x4E3F3F, false),
        ("tongs_start_east", 0, 41, 50, 0x505938, false),
        ("tongs_start_east", 0, 47, 47, 0x606C83, false),
        ("tongs_start_east", 0, 42, 45, 0x662B2F, false),
        ("tongs_start_east", 0, 38, 47, 0x665347, false),
        ("tongs_start_east", 0, 39, 40, 0x684810, true),
        ("tongs_start_east", 0, 44, 31, 0x746161, false),
        ("tongs_start_east", 0, 43, 50, 0x798459, false),
        ("tongs_start_east", 0, 42, 44, 0x9B3F3F, false),
        ("tongs_start_east", 0, 41, 47, 0xA08A6F, false),
        ("tongs_start_east", 0, 38, 44, 0xA49CA1, false),
        ("tongs_start_east", 0, 44, 37, 0xA59DA2, false),
        ("tongs_start_east", 0, 39, 37, 0xC2B9BE, false),
        ("tongs_start_east", 0, 42, 41, 0xC4854C, true),
        ("tongs_start_east", 0, 39, 53, 0xC66659, false),
        ("tongs_start_east", 0, 41, 43, 0xD7D7DB, false),
        ("tongs_start_east", 0, 41, 42, 0xE4BD7C, true),
        ("tongs_start_east", 0, 44, 38, 0xECF0E9, false),
        ("tongs_start_east", 0, 42, 37, 0xF2D7A7, true),
        ("tongs_start_east", 0, 38, 43, 0xF4F4F4, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("hammer_east", &[47, 35, 43, 57, 47, 49, 49]),
        ("pickaxe_east", &[47, 36, 45, 59, 49, 49, 49]),
        ("tongs_end_east", &[46]),
        ("tongs_loop_east", &[45]),
        ("tongs_start_east", &[46]),
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
            let prefix = "spr_npc_olric_specialanimation_spring";
            let asset = format!("assets/animations/NPCs/Olric/Sprites/Spring/{prefix}_{case}.png");
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
                // All reviewed world shades are skin in these Spring strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Olric Spring material mismatch: {id} {case} [{x},{y}]"
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
