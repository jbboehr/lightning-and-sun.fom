use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/valen and the local accepted Valen world baseline"]
fn valen_summer_finish_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/valen");
    let baseline = root.join(
        "generated/characters-balor-autumn-valen-writing-eiland-standard-trial/characters/valen",
    );
    let set = std::env::var_os("FOM_VALEN_SUMMER_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/valen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_VALEN_SUMMER_FINISH_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/valen-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..163];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..163], prior);
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
    let source = [0xFBD3A7, 0xEFA67A, 0xC37555, 0x762E21];
    // Literal source-grid landmarks cover skin and protected materials independently
    // of recipe seeds. Per-frame counts guard moving and briefly exposed skin.
    let landmarks = [
        ("heal_end_east", 0, 33, 49, 0x000000, false),
        ("heal_end_east", 0, 40, 46, 0x3F3935, false),
        ("heal_end_east", 0, 35, 44, 0x542F3A, false),
        ("heal_end_east", 0, 32, 50, 0x596CA1, false),
        ("heal_end_east", 0, 41, 52, 0x6264A0, false),
        ("heal_end_east", 0, 37, 28, 0x6D4C12, false),
        ("heal_end_east", 0, 42, 46, 0x6D70AF, false),
        ("heal_end_east", 0, 34, 40, 0x6E578A, false),
        ("heal_end_east", 0, 39, 48, 0x72665E, false),
        ("heal_end_east", 0, 33, 48, 0x762E21, true),
        ("heal_end_east", 0, 36, 50, 0x7D391E, false),
        ("heal_end_east", 0, 36, 48, 0x934D00, false),
        ("heal_end_east", 0, 33, 34, 0xA385B9, false),
        ("heal_end_east", 0, 34, 50, 0xA6B7E5, false),
        ("heal_end_east", 0, 36, 44, 0xAB615F, false),
        ("heal_end_east", 0, 33, 46, 0xAEB0DF, false),
        ("heal_end_east", 0, 41, 51, 0xAFA190, false),
        ("heal_end_east", 0, 37, 30, 0xBD8E19, false),
        ("heal_end_east", 0, 36, 29, 0xC2D5E4, false),
        ("heal_end_east", 0, 42, 45, 0xC37555, true),
        ("heal_end_east", 0, 38, 44, 0xCD8D8B, false),
        ("heal_end_east", 0, 37, 49, 0xCF8039, false),
        ("heal_end_east", 0, 33, 33, 0xD6C1DD, false),
        ("heal_end_east", 0, 40, 48, 0xE3DACA, false),
        ("heal_end_east", 0, 39, 42, 0xEFA67A, true),
        ("heal_end_east", 0, 34, 33, 0xF5F5F5, false),
        ("heal_end_east", 0, 34, 47, 0xFBD3A7, true),
        ("heal_end_east", 0, 37, 48, 0xFFC962, false),
        ("heal_loop_east", 0, 39, 48, 0x000000, false),
        ("heal_loop_east", 0, 42, 41, 0x542F3A, false),
        ("heal_loop_east", 0, 40, 49, 0x596CA1, false),
        ("heal_loop_east", 0, 39, 52, 0x6264A0, false),
        ("heal_loop_east", 0, 39, 27, 0x6D4C12, false),
        ("heal_loop_east", 0, 35, 35, 0x6E578A, false),
        ("heal_loop_east", 0, 43, 46, 0x762E21, true),
        ("heal_loop_east", 0, 44, 49, 0x7D391E, false),
        ("heal_loop_east", 0, 45, 49, 0x934D00, false),
        ("heal_loop_east", 0, 35, 33, 0xA385B9, false),
        ("heal_loop_east", 0, 43, 35, 0xA59DA2, false),
        ("heal_loop_east", 0, 42, 49, 0xA6B7E5, false),
        ("heal_loop_east", 0, 38, 43, 0xAB615F, false),
        ("heal_loop_east", 0, 40, 46, 0xAEB0DF, false),
        ("heal_loop_east", 0, 38, 49, 0xAFA190, false),
        ("heal_loop_east", 0, 39, 29, 0xBD8E19, false),
        ("heal_loop_east", 0, 39, 35, 0xC2B9BE, false),
        ("heal_loop_east", 0, 38, 28, 0xC2D5E4, false),
        ("heal_loop_east", 0, 43, 47, 0xC37555, true),
        ("heal_loop_east", 0, 39, 43, 0xCD8D8B, false),
        ("heal_loop_east", 0, 45, 48, 0xCF8039, false),
        ("heal_loop_east", 0, 35, 32, 0xD6C1DD, false),
        ("heal_loop_east", 0, 39, 50, 0xE3DACA, false),
        ("heal_loop_east", 0, 38, 37, 0xECF0E9, false),
        ("heal_loop_east", 0, 41, 41, 0xEFA67A, true),
        ("heal_loop_east", 0, 36, 32, 0xF5F5F5, false),
        ("heal_loop_east", 0, 42, 46, 0xFBD3A7, true),
        ("heal_loop_east", 0, 45, 47, 0xFFC962, false),
        ("heal_start_east", 0, 33, 49, 0x000000, false),
        ("heal_start_east", 0, 40, 46, 0x3F3935, false),
        ("heal_start_east", 0, 35, 44, 0x542F3A, false),
        ("heal_start_east", 0, 32, 50, 0x596CA1, false),
        ("heal_start_east", 0, 41, 52, 0x6264A0, false),
        ("heal_start_east", 0, 37, 28, 0x6D4C12, false),
        ("heal_start_east", 0, 42, 46, 0x6D70AF, false),
        ("heal_start_east", 0, 34, 40, 0x6E578A, false),
        ("heal_start_east", 0, 39, 48, 0x72665E, false),
        ("heal_start_east", 0, 33, 48, 0x762E21, true),
        ("heal_start_east", 0, 36, 50, 0x7D391E, false),
        ("heal_start_east", 0, 36, 48, 0x934D00, false),
        ("heal_start_east", 0, 33, 34, 0xA385B9, false),
        ("heal_start_east", 0, 34, 50, 0xA6B7E5, false),
        ("heal_start_east", 0, 36, 44, 0xAB615F, false),
        ("heal_start_east", 0, 33, 46, 0xAEB0DF, false),
        ("heal_start_east", 0, 41, 51, 0xAFA190, false),
        ("heal_start_east", 0, 37, 30, 0xBD8E19, false),
        ("heal_start_east", 0, 36, 29, 0xC2D5E4, false),
        ("heal_start_east", 0, 42, 45, 0xC37555, true),
        ("heal_start_east", 0, 38, 44, 0xCD8D8B, false),
        ("heal_start_east", 0, 37, 49, 0xCF8039, false),
        ("heal_start_east", 0, 33, 33, 0xD6C1DD, false),
        ("heal_start_east", 0, 40, 48, 0xE3DACA, false),
        ("heal_start_east", 0, 39, 42, 0xEFA67A, true),
        ("heal_start_east", 0, 34, 33, 0xF5F5F5, false),
        ("heal_start_east", 0, 34, 47, 0xFBD3A7, true),
        ("heal_start_east", 0, 37, 48, 0xFFC962, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("heal_end_east", &[70]),
        ("heal_loop_east", &[59, 57, 57, 59]),
        ("heal_start_east", &[70]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [8, 9, 10, 4]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_valen_specialanimation_summer";
            let asset = format!("assets/animations/NPCs/Valen/Sprites/Summer/{prefix}_{case}.png");
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
                    "Valen Summer material mismatch: {id} {case} [{x},{y}]"
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
