use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/valen-winter-actions-study and the local accepted Valen world baseline"]
fn valen_autumn_finish_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-winter-actions-study");
    let baseline = root.join(
        "generated/characters-balor-winter-valen-standard-eiland-autumn-trial/characters/valen",
    );
    let set = std::env::var_os("FOM_VALEN_AUTUMN_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/valen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_VALEN_AUTUMN_FINISH_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/valen-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..188];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..188], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 214);
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
    // Reviewed landmarks distinguish moving skin from protected materials.
    // Literal per-frame counts guard brief and occluded skin exposure.
    let landmarks = [
        ("heal_end_east", 0, 32, 49, 0x000000, false),
        ("heal_end_east", 0, 39, 43, 0x23263D, false),
        ("heal_end_east", 0, 39, 44, 0x393E6C, false),
        ("heal_end_east", 0, 41, 53, 0x57516F, false),
        ("heal_end_east", 0, 32, 50, 0x596CA1, false),
        ("heal_end_east", 0, 38, 44, 0x5C62A9, false),
        ("heal_end_east", 0, 33, 44, 0x61514D, false),
        ("heal_end_east", 0, 37, 28, 0x6D4C12, false),
        ("heal_end_east", 0, 34, 40, 0x6E578A, false),
        ("heal_end_east", 0, 33, 48, 0x762E21, true),
        ("heal_end_east", 0, 36, 50, 0x7D391E, false),
        ("heal_end_east", 0, 42, 53, 0x837CA0, false),
        ("heal_end_east", 0, 42, 44, 0x8F8073, false),
        ("heal_end_east", 0, 36, 48, 0x934D00, false),
        ("heal_end_east", 0, 33, 34, 0xA385B9, false),
        ("heal_end_east", 0, 34, 50, 0xA6B7E5, false),
        ("heal_end_east", 0, 37, 30, 0xBD8E19, false),
        ("heal_end_east", 0, 39, 48, 0xC0B8A2, false),
        ("heal_end_east", 0, 36, 29, 0xC2D5E4, false),
        ("heal_end_east", 0, 32, 46, 0xC37555, true),
        ("heal_end_east", 0, 37, 49, 0xCF8039, false),
        ("heal_end_east", 0, 33, 33, 0xD6C1DD, false),
        ("heal_end_east", 0, 34, 46, 0xEFA67A, true),
        ("heal_end_east", 0, 40, 47, 0xF4F3EB, false),
        ("heal_end_east", 0, 34, 33, 0xF5F5F5, false),
        ("heal_end_east", 0, 33, 47, 0xFBD3A7, true),
        ("heal_end_east", 0, 37, 48, 0xFFC962, false),
        ("heal_loop_east", 1, 48, 40, 0x762E21, true),
        ("heal_loop_east", 1, 47, 42, 0xC37555, true),
        ("heal_loop_east", 1, 44, 41, 0xEFA67A, true),
        ("heal_loop_east", 1, 45, 41, 0xFBD3A7, true),
        ("read_sit_end_south", 1, 44, 47, 0x762E21, true),
        ("read_sit_end_south", 1, 45, 46, 0xFBD3A7, true),
        ("read_sit_loop_south", 0, 47, 44, 0x000000, false),
        ("read_sit_loop_south", 0, 40, 42, 0x23263D, false),
        ("read_sit_loop_south", 0, 44, 38, 0x26031D, false),
        ("read_sit_loop_south", 0, 41, 42, 0x393E6C, false),
        ("read_sit_loop_south", 0, 39, 43, 0x5C62A9, false),
        ("read_sit_loop_south", 0, 41, 41, 0x61514D, false),
        ("read_sit_loop_south", 0, 39, 27, 0x6D4C12, false),
        ("read_sit_loop_south", 0, 35, 35, 0x6E578A, false),
        ("read_sit_loop_south", 0, 37, 38, 0x762E21, true),
        ("read_sit_loop_south", 0, 46, 45, 0x76404E, false),
        ("read_sit_loop_south", 0, 42, 50, 0x837CA0, false),
        ("read_sit_loop_south", 0, 45, 41, 0x8F8073, false),
        ("read_sit_loop_south", 0, 41, 46, 0xA16775, false),
        ("read_sit_loop_south", 0, 35, 33, 0xA385B9, false),
        ("read_sit_loop_south", 0, 43, 36, 0xA59DA2, false),
        ("read_sit_loop_south", 0, 39, 29, 0xBD8E19, false),
        ("read_sit_loop_south", 0, 43, 40, 0xC0B8A2, false),
        ("read_sit_loop_south", 0, 38, 36, 0xC2B9BE, false),
        ("read_sit_loop_south", 0, 38, 28, 0xC2D5E4, false),
        ("read_sit_loop_south", 0, 41, 40, 0xC37555, true),
        ("read_sit_loop_south", 0, 38, 47, 0xC98597, false),
        ("read_sit_loop_south", 0, 41, 45, 0xC9AF9C, false),
        ("read_sit_loop_south", 0, 35, 32, 0xD6C1DD, false),
        ("read_sit_loop_south", 0, 45, 46, 0xDFB1BD, false),
        ("read_sit_loop_south", 0, 43, 37, 0xECF0E9, false),
        ("read_sit_loop_south", 0, 40, 41, 0xEFA67A, true),
        ("read_sit_loop_south", 0, 43, 41, 0xF4F3EB, false),
        ("read_sit_loop_south", 0, 36, 32, 0xF5F5F5, false),
        ("read_sit_loop_south", 0, 42, 44, 0xF6E4D7, false),
        ("read_sit_loop_south", 0, 36, 37, 0xFBD3A7, true),
        ("read_sit_loop_south", 1, 42, 40, 0x762E21, true),
        ("read_sit_loop_south", 1, 39, 41, 0xC37555, true),
        ("read_sit_loop_south", 1, 39, 42, 0xEFA67A, true),
        ("read_sit_start_south", 1, 44, 47, 0x762E21, true),
        ("read_sit_start_south", 1, 45, 46, 0xFBD3A7, true),
        ("write_end_south", 1, 42, 40, 0x762E21, true),
        ("write_end_south", 1, 39, 41, 0xC37555, true),
        ("write_end_south", 1, 39, 42, 0xEFA67A, true),
        ("write_end_south", 1, 45, 48, 0xFBD3A7, true),
        ("write_loop_south", 1, 35, 47, 0xC37555, true),
        ("write_loop_south", 1, 36, 45, 0xEFA67A, true),
        ("write_loop_south", 1, 37, 47, 0xFBD3A7, true),
        ("write_start_south", 0, 35, 48, 0x000000, false),
        ("write_start_south", 0, 34, 44, 0x181829, false),
        ("write_start_south", 0, 37, 45, 0x23263D, false),
        ("write_start_south", 0, 42, 44, 0x393E6C, false),
        ("write_start_south", 0, 33, 45, 0x3F3F74, false),
        ("write_start_south", 0, 39, 44, 0x5C62A9, false),
        ("write_start_south", 0, 42, 43, 0x61514D, false),
        ("write_start_south", 0, 41, 28, 0x6D4C12, false),
        ("write_start_south", 0, 44, 40, 0x6E578A, false),
        ("write_start_south", 0, 48, 47, 0x6E8FBA, false),
        ("write_start_south", 0, 42, 40, 0x762E21, true),
        ("write_start_south", 0, 35, 44, 0x799ADD, false),
        ("write_start_south", 0, 41, 53, 0x837CA0, false),
        ("write_start_south", 0, 41, 46, 0x8F8073, false),
        ("write_start_south", 0, 44, 48, 0x933D52, false),
        ("write_start_south", 0, 34, 33, 0xA385B9, false),
        ("write_start_south", 0, 34, 49, 0xB6CBF7, false),
        ("write_start_south", 0, 35, 30, 0xBD8E19, false),
        ("write_start_south", 0, 39, 48, 0xC0B8A2, false),
        ("write_start_south", 0, 37, 29, 0xC2D5E4, false),
        ("write_start_south", 0, 39, 41, 0xC37555, true),
        ("write_start_south", 0, 47, 46, 0xC3D1DD, false),
        ("write_start_south", 0, 44, 32, 0xD6C1DD, false),
        ("write_start_south", 0, 39, 42, 0xEFA67A, true),
        ("write_start_south", 0, 41, 48, 0xF4F3EB, false),
        ("write_start_south", 0, 50, 44, 0xF5F5F5, false),
        ("write_start_south", 0, 45, 48, 0xFBD3A7, true),
        ("write_start_south", 1, 40, 40, 0xC37555, true),
        ("write_start_south", 1, 40, 41, 0xEFA67A, true),
        ("write_start_south", 1, 42, 48, 0xFBD3A7, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("heal_end_east", &[63]),
        ("heal_loop_east", &[45, 43, 43, 45]),
        ("heal_start_east", &[63]),
        ("read_sit_end_south", &[51, 34, 44]),
        ("read_sit_loop_south", &[37, 49, 37, 49]),
        ("read_sit_start_south", &[44, 43, 42]),
        ("write_end_south", &[41, 54]),
        ("write_loop_south", &[44, 47, 45, 45]),
        ("write_start_south", &[54, 41]),
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
            let prefix = "spr_npc_valen_specialanimation_autumn";
            let asset = format!("assets/animations/NPCs/Valen/Sprites/Autumn/{prefix}_{case}.png");
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
                    "Valen Autumn material mismatch: {id} {case} [{x},{y}]"
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
