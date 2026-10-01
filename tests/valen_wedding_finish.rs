use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/valen and the local accepted Valen world baseline"]
fn valen_wedding_finish_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/valen");
    let baseline = root.join("generated/slice-008-build/characters/valen");
    let set = std::env::var_os("FOM_VALEN_WEDDING_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/valen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_VALEN_WEDDING_FINISH_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/valen-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..248];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..248], prior);
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
        ("action_east", 0, 45, 48, 0x000000, false),
        ("action_east", 0, 41, 43, 0x3B3F5D, false),
        ("action_east", 0, 38, 49, 0x49495B, false),
        ("action_east", 0, 37, 40, 0x6E578A, false),
        ("action_east", 0, 40, 48, 0x762E21, true),
        ("action_east", 0, 42, 42, 0x787CA5, false),
        ("action_east", 0, 38, 46, 0x9696A8, false),
        ("action_east", 0, 44, 43, 0x9E528C, false),
        ("action_east", 0, 41, 32, 0xA385B9, false),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 42, 49, 0xB3B3C1, false),
        ("action_east", 0, 40, 36, 0xC2B9BE, false),
        ("action_east", 0, 39, 48, 0xC37555, true),
        ("action_east", 0, 38, 32, 0xD6C1DD, false),
        ("action_east", 0, 43, 49, 0xD9D4D8, false),
        ("action_east", 0, 39, 38, 0xECF0E9, false),
        ("action_east", 0, 43, 40, 0xEFA67A, true),
        ("action_east", 0, 43, 43, 0xF2A7E0, false),
        ("action_east", 0, 38, 33, 0xF5F5F5, false),
        ("action_east", 0, 41, 47, 0xFBD3A7, true),
        ("action_east", 0, 39, 46, 0xFFFAF6, false),
        ("action_east", 1, 44, 52, 0x762E21, true),
        ("action_east", 1, 49, 45, 0xC37555, true),
        ("action_east", 1, 38, 51, 0xEFA67A, true),
        ("action_east", 1, 48, 44, 0xFBD3A7, true),
        ("action_north", 1, 35, 46, 0x762E21, true),
        ("action_north", 1, 42, 52, 0xEFA67A, true),
        ("action_north", 1, 34, 46, 0xFBD3A7, true),
        ("action_south", 1, 35, 47, 0x762E21, true),
        ("action_south", 1, 45, 45, 0xC37555, true),
        ("action_south", 1, 41, 52, 0xEFA67A, true),
        ("action_south", 1, 36, 46, 0xFBD3A7, true),
        ("blink_east", 1, 35, 47, 0x762E21, true),
        ("blink_east", 1, 34, 47, 0xC37555, true),
        ("blink_east", 1, 39, 52, 0xEFA67A, true),
        ("blink_east", 1, 36, 46, 0xFBD3A7, true),
        ("blink_south", 0, 40, 48, 0x000000, false),
        ("blink_south", 0, 39, 42, 0x3B3F5D, false),
        ("blink_south", 0, 38, 46, 0x49495B, false),
        ("blink_south", 0, 36, 34, 0x6E578A, false),
        ("blink_south", 0, 46, 47, 0x762E21, true),
        ("blink_south", 0, 40, 41, 0x787CA5, false),
        ("blink_south", 0, 40, 46, 0x9696A8, false),
        ("blink_south", 0, 42, 42, 0x9E528C, false),
        ("blink_south", 0, 42, 31, 0xA385B9, false),
        ("blink_south", 0, 38, 48, 0xB3B3C1, false),
        ("blink_south", 0, 42, 36, 0xC2B9BE, false),
        ("blink_south", 0, 37, 52, 0xC37555, true),
        ("blink_south", 0, 35, 31, 0xD6C1DD, false),
        ("blink_south", 0, 42, 46, 0xD9D4D8, false),
        ("blink_south", 0, 42, 37, 0xECF0E9, false),
        ("blink_south", 0, 41, 52, 0xEFA67A, true),
        ("blink_south", 0, 41, 42, 0xF2A7E0, false),
        ("blink_south", 0, 36, 32, 0xF5F5F5, false),
        ("blink_south", 0, 47, 46, 0xFBD3A7, true),
        ("blink_south", 0, 34, 45, 0xFFFAF6, false),
        ("blink_south", 1, 46, 47, 0x762E21, true),
        ("blink_south", 1, 37, 52, 0xC37555, true),
        ("blink_south", 1, 41, 52, 0xEFA67A, true),
        ("blink_south", 1, 47, 46, 0xFBD3A7, true),
        ("kiss_east", 1, 36, 48, 0x762E21, true),
        ("kiss_east", 1, 35, 48, 0xC37555, true),
        ("kiss_east", 1, 43, 40, 0xEFA67A, true),
        ("kiss_east", 1, 37, 47, 0xFBD3A7, true),
        ("sit_south", 0, 40, 47, 0x000000, false),
        ("sit_south", 0, 39, 42, 0x3B3F5D, false),
        ("sit_south", 0, 40, 45, 0x49495B, false),
        ("sit_south", 0, 36, 34, 0x6E578A, false),
        ("sit_south", 0, 45, 47, 0x762E21, true),
        ("sit_south", 0, 40, 41, 0x787CA5, false),
        ("sit_south", 0, 44, 44, 0x9696A8, false),
        ("sit_south", 0, 42, 42, 0x9E528C, false),
        ("sit_south", 0, 42, 31, 0xA385B9, false),
        ("sit_south", 0, 37, 45, 0xB3B3C1, false),
        ("sit_south", 0, 41, 35, 0xC2B9BE, false),
        ("sit_south", 0, 37, 49, 0xC37555, true),
        ("sit_south", 0, 35, 31, 0xD6C1DD, false),
        ("sit_south", 0, 44, 45, 0xD9D4D8, false),
        ("sit_south", 0, 37, 37, 0xECF0E9, false),
        ("sit_south", 0, 46, 46, 0xEFA67A, true),
        ("sit_south", 0, 41, 42, 0xF2A7E0, false),
        ("sit_south", 0, 36, 32, 0xF5F5F5, false),
        ("sit_south", 0, 44, 36, 0xFBD3A7, true),
        ("sit_south", 0, 34, 45, 0xFFFAF6, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[42, 42, 42, 42, 42, 42, 43]),
        ("action_north", &[12, 10, 10, 10, 10, 12, 16]),
        ("action_south", &[49, 48, 48, 48, 48, 49, 52]),
        ("blink_east", &[46, 55, 46]),
        ("blink_south", &[55, 64, 55]),
        ("kiss_east", &[42, 48, 53, 54]),
        ("sit_east", &[38]),
        ("sit_north", &[6]),
        ("sit_south", &[46]),
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
            let prefix = "spr_npc_valen_wedding";
            let asset = format!("assets/animations/NPCs/Valen/Sprites/Wedding/{prefix}_{case}.png");
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
                // All reviewed world shades are skin in these Wedding strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Valen Wedding material mismatch: {id} {case} [{x},{y}]"
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
