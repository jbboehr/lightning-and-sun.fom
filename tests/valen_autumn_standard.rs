use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/valen-winter-actions-study and the local accepted Valen world baseline"]
fn valen_autumn_standard_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-winter-actions-study");
    let baseline = root.join(
        "generated/characters-balor-autumn-finish-valen-actions-eiland-summer-finish-trial/characters/valen",
    );
    let set = std::env::var_os("FOM_VALEN_AUTUMN_STANDARD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/valen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_VALEN_AUTUMN_STANDARD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/valen-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..183];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..183], prior);
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
        ("action_north", 0, 32, 44, 0xFBD3A7, true),
        ("action_east", 0, 42, 48, 0x000000, false),
        ("action_east", 0, 42, 43, 0x23263D, false),
        ("action_east", 0, 43, 45, 0x393E6C, false),
        ("action_east", 0, 42, 53, 0x57516F, false),
        ("action_east", 0, 43, 44, 0x5C62A9, false),
        ("action_east", 0, 37, 48, 0x61514D, false),
        ("action_east", 0, 40, 28, 0x6D4C12, false),
        ("action_east", 0, 45, 40, 0x6E578A, false),
        ("action_east", 0, 40, 48, 0x762E21, true),
        ("action_east", 0, 43, 53, 0x837CA0, false),
        ("action_east", 0, 44, 47, 0x8F8073, false),
        ("action_east", 0, 36, 34, 0xA385B9, false),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 40, 30, 0xBD8E19, false),
        ("action_east", 0, 39, 50, 0xC0B8A2, false),
        ("action_east", 0, 40, 36, 0xC2B9BE, false),
        ("action_east", 0, 39, 29, 0xC2D5E4, false),
        ("action_east", 0, 42, 41, 0xC37555, true),
        ("action_east", 0, 36, 33, 0xD6C1DD, false),
        ("action_east", 0, 39, 38, 0xECF0E9, false),
        ("action_east", 0, 43, 40, 0xEFA67A, true),
        ("action_east", 0, 43, 49, 0xF4F3EB, false),
        ("action_east", 0, 37, 33, 0xF5F5F5, false),
        ("action_east", 0, 41, 47, 0xFBD3A7, true),
        ("action_east", 1, 48, 45, 0x762E21, true),
        ("action_east", 1, 44, 40, 0xC37555, true),
        ("action_east", 1, 47, 45, 0xEFA67A, true),
        ("action_east", 1, 47, 44, 0xFBD3A7, true),
        ("action_south", 0, 34, 49, 0x000000, false),
        ("action_south", 0, 37, 45, 0x23263D, false),
        ("action_south", 0, 38, 45, 0x393E6C, false),
        ("action_south", 0, 34, 45, 0x5C62A9, false),
        ("action_south", 0, 44, 44, 0x61514D, false),
        ("action_south", 0, 41, 28, 0x6D4C12, false),
        ("action_south", 0, 36, 41, 0x6E578A, false),
        ("action_south", 0, 46, 47, 0x762E21, true),
        ("action_south", 0, 41, 53, 0x837CA0, false),
        ("action_south", 0, 41, 46, 0x8F8073, false),
        ("action_south", 0, 34, 33, 0xA385B9, false),
        ("action_south", 0, 35, 30, 0xBD8E19, false),
        ("action_south", 0, 39, 48, 0xC0B8A2, false),
        ("action_south", 0, 41, 36, 0xC2B9BE, false),
        ("action_south", 0, 37, 29, 0xC2D5E4, false),
        ("action_south", 0, 33, 46, 0xC37555, true),
        ("action_south", 0, 44, 32, 0xD6C1DD, false),
        ("action_south", 0, 37, 38, 0xECF0E9, false),
        ("action_south", 0, 39, 42, 0xEFA67A, true),
        ("action_south", 0, 38, 48, 0xF4F3EB, false),
        ("action_south", 0, 36, 33, 0xF5F5F5, false),
        ("action_south", 0, 35, 47, 0xFBD3A7, true),
        ("action_south", 1, 35, 47, 0x762E21, true),
        ("action_south", 1, 34, 45, 0xC37555, true),
        ("action_south", 1, 40, 41, 0xEFA67A, true),
        ("action_south", 1, 35, 46, 0xFBD3A7, true),
        ("kiss_east", 1, 36, 48, 0x762E21, true),
        ("kiss_east", 1, 42, 41, 0xC37555, true),
        ("kiss_east", 1, 43, 40, 0xEFA67A, true),
        ("kiss_east", 1, 37, 47, 0xFBD3A7, true),
        ("sleep_east", 0, 37, 47, 0x000000, false),
        ("sleep_east", 0, 38, 44, 0x23263D, false),
        ("sleep_east", 0, 42, 42, 0x393E6C, false),
        ("sleep_east", 0, 38, 53, 0x57516F, false),
        ("sleep_east", 0, 40, 42, 0x5C62A9, false),
        ("sleep_east", 0, 42, 45, 0x61514D, false),
        ("sleep_east", 0, 39, 27, 0x6D4C12, false),
        ("sleep_east", 0, 37, 40, 0x6E578A, false),
        ("sleep_east", 0, 37, 38, 0x762E21, true),
        ("sleep_east", 0, 42, 53, 0x837CA0, false),
        ("sleep_east", 0, 41, 45, 0x8F8073, false),
        ("sleep_east", 0, 35, 33, 0xA385B9, false),
        ("sleep_east", 0, 39, 29, 0xBD8E19, false),
        ("sleep_east", 0, 41, 47, 0xC0B8A2, false),
        ("sleep_east", 0, 38, 28, 0xC2D5E4, false),
        ("sleep_east", 0, 44, 36, 0xC37555, true),
        ("sleep_east", 0, 35, 32, 0xD6C1DD, false),
        ("sleep_east", 0, 43, 35, 0xEFA67A, true),
        ("sleep_east", 0, 42, 48, 0xF4F3EB, false),
        ("sleep_east", 0, 36, 32, 0xF5F5F5, false),
        ("sleep_east", 0, 44, 40, 0xFBD3A7, true),
        ("sleep_east", 0, 36, 42, 0xFCF5F1, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[39, 40, 39, 40, 39, 39, 41]),
        ("action_north", &[1, 0, 0, 0, 0, 0, 0]),
        ("action_south", &[47, 49, 46, 49, 46, 47, 51]),
        ("kiss_east", &[41, 47, 53, 53]),
        ("sleep_east", &[46]),
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
            let prefix = "spr_npc_valen_autumn";
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
            if case == "action_north" {
                assert_eq!(per_frame, [1, 0, 0, 0, 0, 0, 0]);
            } else {
                assert!(per_frame.iter().all(|n| *n > 0), "empty frame in {case}");
            }
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
