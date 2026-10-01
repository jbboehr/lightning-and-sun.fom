use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/valen and the local accepted Valen world baseline"]
fn valen_beach_actions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/valen");
    let baseline = root.join("generated/slice-005-build/characters/valen");
    let set = std::env::var_os("FOM_VALEN_BEACH_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/valen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_VALEN_BEACH_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/valen-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..234];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..234], prior);
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
        ("action_east", 0, 37, 49, 0x000000, false),
        ("action_east", 0, 43, 44, 0x2E2D38, false),
        ("action_east", 0, 41, 45, 0x4D4C53, false),
        ("action_east", 0, 42, 52, 0x6264A0, false),
        ("action_east", 0, 45, 40, 0x6E578A, false),
        ("action_east", 0, 40, 48, 0x762E21, true),
        ("action_east", 0, 38, 49, 0x874B9A, false),
        ("action_east", 0, 42, 50, 0x8BBDBE, false),
        ("action_east", 0, 43, 42, 0x8D9FB8, false),
        ("action_east", 0, 41, 32, 0xA385B9, false),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 43, 46, 0xB571AF, false),
        ("action_east", 0, 40, 36, 0xC2B9BE, false),
        ("action_east", 0, 39, 48, 0xC37555, true),
        ("action_east", 0, 40, 32, 0xD6C1DD, false),
        ("action_east", 0, 41, 44, 0xD6DCE5, false),
        ("action_east", 0, 39, 38, 0xECF0E9, false),
        ("action_east", 0, 42, 42, 0xEFA67A, true),
        ("action_east", 0, 42, 49, 0xF0B1DE, false),
        ("action_east", 0, 38, 33, 0xF5F5F5, false),
        ("action_east", 0, 41, 48, 0xFBD3A7, true),
        ("action_east", 1, 48, 45, 0x762E21, true),
        ("action_east", 1, 39, 48, 0xC37555, true),
        ("action_east", 1, 47, 45, 0xEFA67A, true),
        ("action_east", 1, 49, 44, 0xFBD3A7, true),
        ("action_north", 1, 35, 46, 0x762E21, true),
        ("action_north", 1, 37, 53, 0xC37555, true),
        ("action_north", 1, 41, 49, 0xEFA67A, true),
        ("action_north", 1, 36, 45, 0xFBD3A7, true),
        ("action_south", 1, 35, 47, 0x762E21, true),
        ("action_south", 1, 43, 43, 0xC37555, true),
        ("action_south", 1, 37, 49, 0xEFA67A, true),
        ("action_south", 1, 36, 47, 0xFBD3A7, true),
        ("blink_east", 0, 34, 48, 0x000000, false),
        ("blink_east", 0, 38, 44, 0x2E2D38, false),
        ("blink_east", 0, 40, 44, 0x4D4C53, false),
        ("blink_east", 0, 41, 52, 0x6264A0, false),
        ("blink_east", 0, 37, 40, 0x6E578A, false),
        ("blink_east", 0, 35, 47, 0x762E21, true),
        ("blink_east", 0, 37, 48, 0x874B9A, false),
        ("blink_east", 0, 41, 47, 0x8BBDBE, false),
        ("blink_east", 0, 42, 41, 0x8D9FB8, false),
        ("blink_east", 0, 40, 31, 0xA385B9, false),
        ("blink_east", 0, 43, 36, 0xA59DA2, false),
        ("blink_east", 0, 42, 45, 0xB571AF, false),
        ("blink_east", 0, 38, 36, 0xC2B9BE, false),
        ("blink_east", 0, 44, 44, 0xC37555, true),
        ("blink_east", 0, 39, 31, 0xD6C1DD, false),
        ("blink_east", 0, 40, 43, 0xD6DCE5, false),
        ("blink_east", 0, 43, 37, 0xECF0E9, false),
        ("blink_east", 0, 44, 45, 0xEFA67A, true),
        ("blink_east", 0, 41, 46, 0xF0B1DE, false),
        ("blink_east", 0, 37, 32, 0xF5F5F5, false),
        ("blink_east", 0, 36, 47, 0xFBD3A7, true),
        ("blink_east", 1, 35, 47, 0x762E21, true),
        ("blink_east", 1, 44, 44, 0xC37555, true),
        ("blink_east", 1, 44, 45, 0xEFA67A, true),
        ("blink_east", 1, 36, 47, 0xFBD3A7, true),
        ("blink_south", 1, 33, 47, 0x762E21, true),
        ("blink_south", 1, 34, 44, 0xC37555, true),
        ("blink_south", 1, 42, 49, 0xEFA67A, true),
        ("blink_south", 1, 47, 46, 0xFBD3A7, true),
        ("kiss_east", 0, 43, 48, 0x000000, false),
        ("kiss_east", 0, 41, 44, 0x2E2D38, false),
        ("kiss_east", 0, 39, 45, 0x4D4C53, false),
        ("kiss_east", 0, 41, 52, 0x6264A0, false),
        ("kiss_east", 0, 43, 40, 0x6E578A, false),
        ("kiss_east", 0, 35, 48, 0x762E21, true),
        ("kiss_east", 0, 42, 48, 0x874B9A, false),
        ("kiss_east", 0, 40, 50, 0x8BBDBE, false),
        ("kiss_east", 0, 41, 42, 0x8D9FB8, false),
        ("kiss_east", 0, 39, 32, 0xA385B9, false),
        ("kiss_east", 0, 42, 36, 0xA59DA2, false),
        ("kiss_east", 0, 41, 46, 0xB571AF, false),
        ("kiss_east", 0, 38, 36, 0xC2B9BE, false),
        ("kiss_east", 0, 34, 46, 0xC37555, true),
        ("kiss_east", 0, 38, 32, 0xD6C1DD, false),
        ("kiss_east", 0, 39, 44, 0xD6DCE5, false),
        ("kiss_east", 0, 37, 38, 0xECF0E9, false),
        ("kiss_east", 0, 41, 40, 0xEFA67A, true),
        ("kiss_east", 0, 40, 47, 0xF0B1DE, false),
        ("kiss_east", 0, 36, 33, 0xF5F5F5, false),
        ("kiss_east", 0, 36, 48, 0xFBD3A7, true),
        ("kiss_east", 1, 36, 48, 0x762E21, true),
        ("kiss_east", 1, 35, 46, 0xC37555, true),
        ("kiss_east", 1, 43, 40, 0xEFA67A, true),
        ("kiss_east", 1, 37, 47, 0xFBD3A7, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[62, 66, 67, 66, 67, 62, 72]),
        ("action_north", &[38, 34, 36, 34, 36, 40, 42]),
        ("action_south", &[75, 76, 76, 76, 76, 75, 86]),
        ("blink_east", &[75, 85, 75]),
        ("blink_south", &[89, 98, 89]),
        ("kiss_east", &[64, 70, 80, 76]),
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
            let prefix = "spr_npc_valen_beach";
            let asset = format!("assets/animations/NPCs/Valen/Sprites/Beach/{prefix}_{case}.png");
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
                // All reviewed world shades are skin in these Beach strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Valen Beach material mismatch: {id} {case} [{x},{y}]"
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
