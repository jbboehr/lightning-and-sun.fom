use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/valen and the local accepted Valen world baseline"]
fn valen_autumn_actions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/valen");
    let baseline = root.join(
        "generated/characters-balor-valen-autumn-eiland-summer-magnify-trial/characters/valen",
    );
    let set = std::env::var_os("FOM_VALEN_AUTUMN_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/valen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_VALEN_AUTUMN_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/valen-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..172];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..172], prior);
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
        ("drink_north", 0, 46, 42, 0xC37555, true),
        ("drink_north", 2, 46, 42, 0xC37555, true),
        ("eat_north", 0, 46, 42, 0xC37555, true),
        ("eat_north", 2, 46, 42, 0xC37555, true),
        ("blink_east", 0, 35, 48, 0x000000, false),
        ("blink_east", 0, 37, 44, 0x23263D, false),
        ("blink_east", 0, 40, 44, 0x393E6C, false),
        ("blink_east", 0, 38, 53, 0x57516F, false),
        ("blink_east", 0, 35, 44, 0x5C62A9, false),
        ("blink_east", 0, 38, 45, 0x61514D, false),
        ("blink_east", 0, 39, 27, 0x6D4C12, false),
        ("blink_east", 0, 35, 35, 0x6E578A, false),
        ("blink_east", 0, 35, 47, 0x762E21, true),
        ("blink_east", 0, 42, 53, 0x837CA0, false),
        ("blink_east", 0, 40, 45, 0x8F8073, false),
        ("blink_east", 0, 35, 33, 0xA385B9, false),
        ("blink_east", 0, 43, 36, 0xA59DA2, false),
        ("blink_east", 0, 39, 29, 0xBD8E19, false),
        ("blink_east", 0, 38, 48, 0xC0B8A2, false),
        ("blink_east", 0, 38, 36, 0xC2B9BE, false),
        ("blink_east", 0, 38, 28, 0xC2D5E4, false),
        ("blink_east", 0, 34, 45, 0xC37555, true),
        ("blink_east", 0, 35, 32, 0xD6C1DD, false),
        ("blink_east", 0, 43, 37, 0xECF0E9, false),
        ("blink_east", 0, 41, 41, 0xEFA67A, true),
        ("blink_east", 0, 42, 47, 0xF4F3EB, false),
        ("blink_east", 0, 36, 32, 0xF5F5F5, false),
        ("blink_east", 0, 36, 46, 0xFBD3A7, true),
        ("blink_east", 1, 35, 47, 0x762E21, true),
        ("blink_east", 1, 34, 45, 0xC37555, true),
        ("blink_east", 1, 41, 41, 0xEFA67A, true),
        ("blink_east", 1, 36, 46, 0xFBD3A7, true),
        ("blink_south", 1, 46, 47, 0x762E21, true),
        ("blink_south", 1, 33, 45, 0xC37555, true),
        ("blink_south", 1, 40, 41, 0xEFA67A, true),
        ("blink_south", 1, 47, 46, 0xFBD3A7, true),
        ("drink_east", 1, 40, 42, 0xC37555, true),
        ("drink_east", 1, 40, 41, 0xEFA67A, true),
        ("drink_east", 1, 39, 41, 0xFBD3A7, true),
        ("drink_north", 1, 45, 41, 0xC37555, true),
        ("drink_south", 1, 45, 47, 0x762E21, true),
        ("drink_south", 1, 39, 41, 0xC37555, true),
        ("drink_south", 1, 45, 46, 0xEFA67A, true),
        ("drink_south", 1, 35, 41, 0xFBD3A7, true),
        ("eat_east", 0, 37, 45, 0x000000, false),
        ("eat_east", 0, 41, 42, 0x23263D, false),
        ("eat_east", 0, 39, 43, 0x393E6C, false),
        ("eat_east", 0, 41, 49, 0x57516F, false),
        ("eat_east", 0, 40, 43, 0x5C62A9, false),
        ("eat_east", 0, 39, 42, 0x61514D, false),
        ("eat_east", 0, 39, 27, 0x6D4C12, false),
        ("eat_east", 0, 35, 35, 0x6E578A, false),
        ("eat_east", 0, 37, 38, 0x762E21, true),
        ("eat_east", 0, 42, 49, 0x837CA0, false),
        ("eat_east", 0, 44, 46, 0x8F8073, false),
        ("eat_east", 0, 35, 33, 0xA385B9, false),
        ("eat_east", 0, 43, 35, 0xA59DA2, false),
        ("eat_east", 0, 39, 29, 0xBD8E19, false),
        ("eat_east", 0, 38, 46, 0xC0B8A2, false),
        ("eat_east", 0, 39, 35, 0xC2B9BE, false),
        ("eat_east", 0, 38, 28, 0xC2D5E4, false),
        ("eat_east", 0, 41, 40, 0xC37555, true),
        ("eat_east", 0, 35, 32, 0xD6C1DD, false),
        ("eat_east", 0, 38, 37, 0xECF0E9, false),
        ("eat_east", 0, 42, 43, 0xEFA67A, true),
        ("eat_east", 0, 40, 46, 0xF4F3EB, false),
        ("eat_east", 0, 36, 32, 0xF5F5F5, false),
        ("eat_east", 0, 43, 43, 0xFBD3A7, true),
        ("eat_east", 1, 46, 42, 0x762E21, true),
        ("eat_east", 1, 45, 42, 0xC37555, true),
        ("eat_east", 1, 45, 41, 0xFBD3A7, true),
        ("eat_north", 1, 45, 41, 0xC37555, true),
        ("eat_south", 1, 42, 40, 0x762E21, true),
        ("eat_south", 1, 39, 41, 0xC37555, true),
        ("eat_south", 1, 46, 46, 0xEFA67A, true),
        ("eat_south", 1, 37, 48, 0xFBD3A7, true),
        ("sit_south", 0, 36, 47, 0x000000, false),
        ("sit_south", 0, 37, 44, 0x23263D, false),
        ("sit_south", 0, 38, 44, 0x393E6C, false),
        ("sit_south", 0, 39, 43, 0x5C62A9, false),
        ("sit_south", 0, 35, 43, 0x61514D, false),
        ("sit_south", 0, 41, 27, 0x6D4C12, false),
        ("sit_south", 0, 45, 35, 0x6E578A, false),
        ("sit_south", 0, 45, 47, 0x762E21, true),
        ("sit_south", 0, 41, 50, 0x837CA0, false),
        ("sit_south", 0, 45, 43, 0x8F8073, false),
        ("sit_south", 0, 34, 32, 0xA385B9, false),
        ("sit_south", 0, 35, 29, 0xBD8E19, false),
        ("sit_south", 0, 39, 46, 0xC0B8A2, false),
        ("sit_south", 0, 41, 35, 0xC2B9BE, false),
        ("sit_south", 0, 37, 28, 0xC2D5E4, false),
        ("sit_south", 0, 40, 40, 0xC37555, true),
        ("sit_south", 0, 44, 31, 0xD6C1DD, false),
        ("sit_south", 0, 37, 37, 0xECF0E9, false),
        ("sit_south", 0, 44, 46, 0xEFA67A, true),
        ("sit_south", 0, 41, 46, 0xF4F3EB, false),
        ("sit_south", 0, 36, 32, 0xF5F5F5, false),
        ("sit_south", 0, 44, 36, 0xFBD3A7, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[44, 54, 44]),
        ("blink_south", &[54, 63, 54]),
        ("drink_east", &[37, 51, 37]),
        ("drink_north", &[1, 2, 1]),
        ("drink_south", &[48, 56, 48]),
        ("eat_east", &[37, 42, 46, 45, 39]),
        ("eat_north", &[1, 2, 1]),
        ("eat_south", &[50, 55, 51, 63, 47]),
        ("sit_east", &[38]),
        ("sit_north", &[0]),
        ("sit_south", &[47]),
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
            if case == "sit_north" {
                assert!(per_frame.iter().all(|n| *n == 0));
                let region = candidate["regions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|r| r["asset"] == asset)
                    .unwrap();
                assert!(region["seeds"].as_array().unwrap().is_empty());
                assert_eq!(
                    fs::read(original.join(&asset)).unwrap(),
                    fs::read(variant.join(&asset)).unwrap(),
                    "covered North PNG must remain byte-identical"
                );
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
