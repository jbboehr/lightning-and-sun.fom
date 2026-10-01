use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/eiland and the local accepted Eiland world baseline"]
fn eiland_beach_actions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/eiland");
    let baseline = root.join("generated/slice-011-build/characters/eiland");
    let set = std::env::var_os("FOM_EILAND_BEACH_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_BEACH_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..254];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..254], prior);
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
    let source = [0xE9A980, 0xDE8F5D, 0xBA6A4C, 0x9C5241, 0x7D3B14];
    // Reviewed landmarks distinguish moving skin from protected materials.
    // Literal per-frame counts guard brief and occluded skin exposure.
    let landmarks = [
        ("action_east", 0, 36, 49, 0x000000, false),
        ("action_east", 0, 44, 49, 0x461F3B, false),
        ("action_east", 0, 38, 37, 0x6C2859, false),
        ("action_east", 0, 39, 50, 0x71295C, false),
        ("action_east", 0, 40, 48, 0x7D3B14, true),
        ("action_east", 0, 42, 50, 0x927D96, false),
        ("action_east", 0, 37, 49, 0x9C5241, false),
        ("action_east", 0, 42, 34, 0x9C5241, true),
        ("action_east", 0, 40, 33, 0xA54E7F, false),
        ("action_east", 0, 44, 48, 0xAE4791, false),
        ("action_east", 0, 38, 47, 0xBA6A4C, true),
        ("action_east", 0, 43, 50, 0xBBB5C7, false),
        ("action_east", 0, 40, 36, 0xC2B9BE, false),
        ("action_east", 0, 38, 49, 0xC9785A, false),
        ("action_east", 0, 42, 52, 0xD8D0DF, false),
        ("action_east", 0, 42, 42, 0xDE8F5D, true),
        ("action_east", 0, 38, 33, 0xE797AC, false),
        ("action_east", 0, 43, 45, 0xE9A980, true),
        ("action_east", 0, 39, 38, 0xECF0E9, false),
        ("action_east", 0, 43, 52, 0xF8ECF9, false),
        ("action_east", 0, 43, 33, 0xFCDAE0, false),
        ("action_east", 0, 45, 32, 0xFFFFFF, false),
        ("action_east", 1, 48, 45, 0x7D3B14, true),
        ("action_east", 1, 45, 44, 0xBA6A4C, true),
        ("action_east", 1, 44, 45, 0xDE8F5D, true),
        ("action_east", 1, 48, 44, 0xE9A980, true),
        ("action_north", 1, 35, 46, 0x7D3B14, true),
        ("action_north", 1, 42, 42, 0xBA6A4C, true),
        ("action_north", 1, 35, 43, 0xDE8F5D, true),
        ("action_north", 1, 40, 43, 0xE9A980, true),
        ("action_south", 1, 45, 46, 0x7D3B14, true),
        ("action_south", 1, 42, 43, 0xBA6A4C, true),
        ("action_south", 1, 43, 44, 0xDE8F5D, true),
        ("action_south", 1, 36, 45, 0xE9A980, true),
        ("blink_east", 0, 45, 48, 0x000000, false),
        ("blink_east", 0, 43, 48, 0x461F3B, false),
        ("blink_east", 0, 37, 36, 0x6C2859, false),
        ("blink_east", 0, 40, 48, 0x71295C, false),
        ("blink_east", 0, 43, 45, 0x7D3B14, true),
        ("blink_east", 0, 41, 49, 0x927D96, false),
        ("blink_east", 0, 41, 33, 0x9C5241, true),
        ("blink_east", 0, 44, 45, 0xA2294C, false),
        ("blink_east", 0, 39, 32, 0xA54E7F, false),
        ("blink_east", 0, 43, 36, 0xA59DA2, false),
        ("blink_east", 0, 39, 47, 0xAE4791, false),
        ("blink_east", 0, 37, 44, 0xBA6A4C, true),
        ("blink_east", 0, 42, 49, 0xBBB5C7, false),
        ("blink_east", 0, 38, 36, 0xC2B9BE, false),
        ("blink_east", 0, 37, 48, 0xC9785A, false),
        ("blink_east", 0, 41, 52, 0xD8D0DF, false),
        ("blink_east", 0, 41, 41, 0xDE8F5D, true),
        ("blink_east", 0, 37, 32, 0xE797AC, false),
        ("blink_east", 0, 36, 45, 0xE9A980, true),
        ("blink_east", 0, 43, 37, 0xECF0E9, false),
        ("blink_east", 0, 39, 48, 0xF0BC70, false),
        ("blink_east", 0, 42, 52, 0xF8ECF9, false),
        ("blink_east", 0, 42, 32, 0xFCDAE0, false),
        ("blink_east", 0, 44, 31, 0xFFFFFF, false),
        ("blink_east", 1, 43, 45, 0x7D3B14, true),
        ("blink_east", 1, 37, 44, 0xBA6A4C, true),
        ("blink_east", 1, 41, 41, 0xDE8F5D, true),
        ("blink_east", 1, 36, 45, 0xE9A980, true),
        ("blink_south", 1, 33, 47, 0x7D3B14, true),
        ("blink_south", 1, 42, 43, 0xBA6A4C, true),
        ("blink_south", 1, 42, 50, 0xDE8F5D, true),
        ("blink_south", 1, 39, 45, 0xE9A980, true),
        ("kiss_east", 0, 34, 49, 0x000000, false),
        ("kiss_east", 0, 42, 49, 0x461F3B, false),
        ("kiss_east", 0, 36, 37, 0x6C2859, false),
        ("kiss_east", 0, 39, 49, 0x71295C, false),
        ("kiss_east", 0, 42, 46, 0x7D3B14, true),
        ("kiss_east", 0, 40, 50, 0x927D96, false),
        ("kiss_east", 0, 40, 34, 0x9C5241, true),
        ("kiss_east", 0, 38, 33, 0xA54E7F, false),
        ("kiss_east", 0, 42, 48, 0xAE4791, false),
        ("kiss_east", 0, 35, 45, 0xBA6A4C, true),
        ("kiss_east", 0, 41, 50, 0xBBB5C7, false),
        ("kiss_east", 0, 38, 36, 0xC2B9BE, false),
        ("kiss_east", 0, 37, 49, 0xC9785A, false),
        ("kiss_east", 0, 41, 52, 0xD8D0DF, false),
        ("kiss_east", 0, 41, 40, 0xDE8F5D, true),
        ("kiss_east", 0, 36, 33, 0xE797AC, false),
        ("kiss_east", 0, 35, 46, 0xE9A980, true),
        ("kiss_east", 0, 37, 38, 0xECF0E9, false),
        ("kiss_east", 0, 38, 49, 0xF0BC70, false),
        ("kiss_east", 0, 42, 52, 0xF8ECF9, false),
        ("kiss_east", 0, 41, 33, 0xFCDAE0, false),
        ("kiss_east", 0, 43, 32, 0xFFFFFF, false),
        ("kiss_east", 1, 36, 48, 0x7D3B14, true),
        ("kiss_east", 1, 36, 45, 0xBA6A4C, true),
        ("kiss_east", 1, 41, 42, 0xDE8F5D, true),
        ("kiss_east", 1, 36, 46, 0xE9A980, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[68, 70, 64, 70, 64, 68, 82]),
        ("action_north", &[62, 58, 59, 58, 59, 65, 67]),
        ("action_south", &[86, 85, 81, 85, 81, 86, 95]),
        ("blink_east", &[85, 92, 85]),
        ("blink_south", &[96, 103, 96]),
        ("kiss_east", &[75, 79, 84, 83]),
    ];
    // Swimwear trim shares a skin shadow; protect its reviewed coordinates.
    let trim: &[(&str, u32, u32, u32)] = &[
        ("action_east", 1, 38, 48),
        ("action_east", 2, 38, 48),
        ("action_east", 3, 38, 48),
        ("action_east", 4, 38, 48),
        ("action_east", 0, 37, 49),
        ("action_east", 5, 37, 49),
        ("action_south", 1, 44, 48),
        ("action_south", 2, 44, 48),
        ("action_south", 3, 44, 48),
        ("action_south", 4, 44, 48),
        ("action_south", 6, 44, 48),
        ("action_south", 0, 44, 49),
        ("action_south", 5, 44, 49),
        ("blink_south", 0, 44, 48),
        ("blink_south", 1, 44, 48),
        ("blink_south", 2, 44, 48),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [0, 9, 10, 3, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_eiland_beach";
            let asset = format!("assets/animations/NPCs/Eiland/Sprites/Beach/{prefix}_{case}.png");
            let before = image::open(original.join(&asset)).unwrap().to_rgba8();
            let after = image::open(variant.join(&asset)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), (80 * frames, 80));
            assert_eq!(before.dimensions(), after.dimensions());
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(variant.join(meta)).unwrap()
            );
            for &(name, frame, x, y) in trim {
                if name == case {
                    assert_eq!(before.get_pixel(frame * 80 + x, y).0, rgba(0x9C5241));
                }
            }
            let mut per_frame = vec![0; frames as usize];
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                // Preserve reviewed swimwear trim that shares a skin shade.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !trim.contains(&(case, x / 80, x % 80, y)))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Eiland Beach material mismatch: {id} {case} [{x},{y}]"
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
