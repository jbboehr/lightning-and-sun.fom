use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/eiland and the local accepted Eiland world baseline"]
fn eiland_beach_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/eiland");
    let baseline = root.join("generated/slice-010-build/characters/eiland");
    let set = std::env::var_os("FOM_EILAND_BEACH_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_BEACH_WORLD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
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
    let source = [0xE9A980, 0xDE8F5D, 0xBA6A4C, 0x9C5241, 0x7D3B14];
    // Reviewed landmarks distinguish moving skin from protected materials.
    // Literal per-frame counts guard brief and occluded skin exposure.
    let landmarks = [
        ("idle_east", 0, 45, 48, 0x000000, false),
        ("idle_east", 0, 43, 48, 0x461F3B, false),
        ("idle_east", 0, 37, 36, 0x6C2859, false),
        ("idle_east", 0, 40, 48, 0x71295C, false),
        ("idle_east", 0, 43, 45, 0x7D3B14, true),
        ("idle_east", 0, 41, 49, 0x927D96, false),
        ("idle_east", 0, 41, 33, 0x9C5241, true),
        ("idle_east", 0, 44, 45, 0xA2294C, false),
        ("idle_east", 0, 39, 32, 0xA54E7F, false),
        ("idle_east", 0, 39, 47, 0xAE4791, false),
        ("idle_east", 0, 37, 44, 0xBA6A4C, true),
        ("idle_east", 0, 42, 49, 0xBBB5C7, false),
        ("idle_east", 0, 39, 35, 0xC2B9BE, false),
        ("idle_east", 0, 37, 48, 0xC9785A, false),
        ("idle_east", 0, 41, 52, 0xD8D0DF, false),
        ("idle_east", 0, 41, 41, 0xDE8F5D, true),
        ("idle_east", 0, 37, 32, 0xE797AC, false),
        ("idle_east", 0, 36, 45, 0xE9A980, true),
        ("idle_east", 0, 38, 37, 0xECF0E9, false),
        ("idle_east", 0, 39, 48, 0xF0BC70, false),
        ("idle_east", 0, 42, 52, 0xF8ECF9, false),
        ("idle_east", 0, 42, 32, 0xFCDAE0, false),
        ("idle_east", 0, 44, 31, 0xFFFFFF, false),
        ("walk_east", 0, 45, 48, 0x000000, false),
        ("walk_east", 0, 43, 48, 0x461F3B, false),
        ("walk_east", 0, 37, 36, 0x6C2859, false),
        ("walk_east", 0, 40, 48, 0x71295C, false),
        ("walk_east", 0, 43, 45, 0x7D3B14, true),
        ("walk_east", 0, 41, 49, 0x927D96, false),
        ("walk_east", 0, 41, 33, 0x9C5241, true),
        ("walk_east", 0, 44, 45, 0xA2294C, false),
        ("walk_east", 0, 39, 32, 0xA54E7F, false),
        ("walk_east", 0, 39, 47, 0xAE4791, false),
        ("walk_east", 0, 37, 44, 0xBA6A4C, true),
        ("walk_east", 0, 42, 49, 0xBBB5C7, false),
        ("walk_east", 0, 39, 35, 0xC2B9BE, false),
        ("walk_east", 0, 37, 48, 0xC9785A, false),
        ("walk_east", 0, 41, 52, 0xD8D0DF, false),
        ("walk_east", 0, 41, 41, 0xDE8F5D, true),
        ("walk_east", 0, 37, 32, 0xE797AC, false),
        ("walk_east", 0, 36, 45, 0xE9A980, true),
        ("walk_east", 0, 38, 37, 0xECF0E9, false),
        ("walk_east", 0, 39, 48, 0xF0BC70, false),
        ("walk_east", 0, 42, 52, 0xF8ECF9, false),
        ("walk_east", 0, 42, 32, 0xFCDAE0, false),
        ("walk_east", 0, 44, 31, 0xFFFFFF, false),
        ("walk_east", 1, 33, 47, 0x7D3B14, true),
        ("walk_east", 1, 36, 49, 0x9C5241, false),
        ("walk_east", 1, 38, 44, 0xBA6A4C, true),
        ("walk_east", 1, 44, 43, 0xDE8F5D, true),
        ("walk_east", 1, 41, 45, 0xE9A980, true),
        ("walk_north", 1, 46, 47, 0x7D3B14, true),
        ("walk_north", 1, 43, 49, 0x9C5241, false),
        ("walk_north", 1, 37, 45, 0xBA6A4C, true),
        ("walk_north", 1, 44, 44, 0xDE8F5D, true),
        ("walk_north", 1, 38, 45, 0xE9A980, true),
        ("walk_south", 0, 46, 48, 0x000000, false),
        ("walk_south", 0, 41, 47, 0x461F3B, false),
        ("walk_south", 0, 34, 37, 0x6C2859, false),
        ("walk_south", 0, 43, 47, 0x71295C, false),
        ("walk_south", 0, 33, 47, 0x7D3B14, true),
        ("walk_south", 0, 41, 48, 0x927D96, false),
        ("walk_south", 0, 44, 48, 0x9C5241, false),
        ("walk_south", 0, 46, 45, 0xA2294C, false),
        ("walk_south", 0, 38, 32, 0xA54E7F, false),
        ("walk_south", 0, 38, 47, 0xAE4791, false),
        ("walk_south", 0, 42, 43, 0xBA6A4C, true),
        ("walk_south", 0, 41, 49, 0xBBB5C7, false),
        ("walk_south", 0, 38, 35, 0xC2B9BE, false),
        ("walk_south", 0, 37, 48, 0xC9785A, false),
        ("walk_south", 0, 42, 52, 0xD8D0DF, false),
        ("walk_south", 0, 42, 50, 0xDE8F5D, true),
        ("walk_south", 0, 36, 32, 0xE797AC, false),
        ("walk_south", 0, 39, 45, 0xE9A980, true),
        ("walk_south", 0, 37, 37, 0xECF0E9, false),
        ("walk_south", 0, 38, 48, 0xF0BC70, false),
        ("walk_south", 0, 41, 52, 0xF8ECF9, false),
        ("walk_south", 0, 41, 32, 0xFCDAE0, false),
        ("walk_south", 0, 45, 45, 0xFF487D, false),
        ("walk_south", 0, 43, 31, 0xFFFFFF, false),
        ("walk_south", 1, 46, 47, 0x7D3B14, true),
        ("walk_south", 1, 36, 45, 0xBA6A4C, true),
        ("walk_south", 1, 35, 44, 0xDE8F5D, true),
        ("walk_south", 1, 40, 45, 0xE9A980, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[84]),
        ("idle_north", &[67]),
        ("idle_south", &[95]),
        ("walk_east", &[84, 89, 84, 81]),
        ("walk_north", &[67, 60, 67, 61]),
        ("walk_south", &[95, 87, 95, 87]),
    ];
    // Swimwear trim shares a skin shadow; protect its reviewed coordinates.
    let trim: &[(&str, u32, u32, u32)] = &[
        ("idle_south", 0, 44, 48),
        ("walk_east", 1, 41, 48),
        ("walk_east", 1, 36, 49),
        ("walk_north", 1, 36, 49),
        ("walk_north", 1, 43, 49),
        ("walk_north", 3, 36, 49),
        ("walk_north", 3, 43, 49),
        ("walk_south", 0, 44, 48),
        ("walk_south", 2, 44, 48),
        ("walk_south", 3, 43, 49),
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
