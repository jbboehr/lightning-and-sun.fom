use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/eiland and the local accepted Eiland world baseline"]
fn eiland_wedding_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/eiland");
    let baseline = root.join("generated/slice-013-build/characters/eiland");
    let set = std::env::var_os("FOM_EILAND_WEDDING_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_WEDDING_WORLD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..262];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..262], prior);
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
        ("idle_east", 0, 34, 49, 0x000000, false),
        ("idle_east", 0, 42, 48, 0x685979, false),
        ("idle_east", 0, 42, 42, 0x6A3126, false),
        ("idle_east", 0, 35, 36, 0x6C2859, false),
        ("idle_east", 0, 40, 43, 0x8B2240, false),
        ("idle_east", 0, 42, 47, 0x927D96, false),
        ("idle_east", 0, 37, 43, 0x9C5241, false),
        ("idle_east", 0, 40, 33, 0x9C5241, true),
        ("idle_east", 0, 46, 32, 0xA54E7F, false),
        ("idle_east", 0, 41, 40, 0xBA6A4C, true),
        ("idle_east", 0, 34, 47, 0xBBB5C7, false),
        ("idle_east", 0, 39, 35, 0xC2B9BE, false),
        ("idle_east", 0, 44, 45, 0xC9785A, false),
        ("idle_east", 0, 40, 39, 0xDE8F5D, true),
        ("idle_east", 0, 44, 46, 0xE4D5E6, false),
        ("idle_east", 0, 41, 42, 0xE64372, false),
        ("idle_east", 0, 38, 32, 0xE797AC, false),
        ("idle_east", 0, 36, 37, 0xE9A980, true),
        ("idle_east", 0, 38, 37, 0xECF0E9, false),
        ("idle_east", 0, 41, 45, 0xF0BC70, false),
        ("idle_east", 0, 41, 47, 0xF4F4F4, false),
        ("idle_east", 0, 42, 32, 0xFCDAE0, false),
        ("idle_east", 0, 44, 31, 0xFFFFFF, false),
        ("walk_east", 0, 34, 49, 0x000000, false),
        ("walk_east", 0, 42, 48, 0x685979, false),
        ("walk_east", 0, 42, 42, 0x6A3126, false),
        ("walk_east", 0, 35, 36, 0x6C2859, false),
        ("walk_east", 0, 40, 43, 0x8B2240, false),
        ("walk_east", 0, 42, 47, 0x927D96, false),
        ("walk_east", 0, 37, 43, 0x9C5241, false),
        ("walk_east", 0, 40, 33, 0x9C5241, true),
        ("walk_east", 0, 46, 32, 0xA54E7F, false),
        ("walk_east", 0, 41, 40, 0xBA6A4C, true),
        ("walk_east", 0, 34, 47, 0xBBB5C7, false),
        ("walk_east", 0, 39, 35, 0xC2B9BE, false),
        ("walk_east", 0, 44, 45, 0xC9785A, false),
        ("walk_east", 0, 40, 39, 0xDE8F5D, true),
        ("walk_east", 0, 44, 46, 0xE4D5E6, false),
        ("walk_east", 0, 41, 42, 0xE64372, false),
        ("walk_east", 0, 38, 32, 0xE797AC, false),
        ("walk_east", 0, 36, 37, 0xE9A980, true),
        ("walk_east", 0, 38, 37, 0xECF0E9, false),
        ("walk_east", 0, 41, 45, 0xF0BC70, false),
        ("walk_east", 0, 41, 47, 0xF4F4F4, false),
        ("walk_east", 0, 42, 32, 0xFCDAE0, false),
        ("walk_east", 0, 44, 31, 0xFFFFFF, false),
        ("walk_east", 1, 38, 40, 0x7D3B14, true),
        ("walk_east", 1, 40, 41, 0xBA6A4C, true),
        ("walk_east", 1, 41, 40, 0xDE8F5D, true),
        ("walk_south", 0, 40, 49, 0x000000, false),
        ("walk_south", 0, 37, 49, 0x685979, false),
        ("walk_south", 0, 41, 42, 0x6A3126, false),
        ("walk_south", 0, 36, 36, 0x6C2859, false),
        ("walk_south", 0, 39, 43, 0x8B2240, false),
        ("walk_south", 0, 43, 47, 0x927D96, false),
        ("walk_south", 0, 43, 43, 0x9C5241, false),
        ("walk_south", 0, 39, 33, 0x9C5241, true),
        ("walk_south", 0, 45, 32, 0xA54E7F, false),
        ("walk_south", 0, 40, 40, 0xBA6A4C, true),
        ("walk_south", 0, 37, 46, 0xBBB5C7, false),
        ("walk_south", 0, 38, 35, 0xC2B9BE, false),
        ("walk_south", 0, 44, 46, 0xC9785A, false),
        ("walk_south", 0, 39, 39, 0xDE8F5D, true),
        ("walk_south", 0, 37, 47, 0xE4D5E6, false),
        ("walk_south", 0, 40, 42, 0xE64372, false),
        ("walk_south", 0, 37, 32, 0xE797AC, false),
        ("walk_south", 0, 35, 37, 0xE9A980, true),
        ("walk_south", 0, 37, 37, 0xECF0E9, false),
        ("walk_south", 0, 40, 45, 0xF0BC70, false),
        ("walk_south", 0, 39, 47, 0xF4F4F4, false),
        ("walk_south", 0, 41, 32, 0xFCDAE0, false),
        ("walk_south", 0, 43, 31, 0xFFFFFF, false),
        ("walk_south", 1, 39, 41, 0xBA6A4C, true),
        ("walk_south", 1, 40, 40, 0xDE8F5D, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[23]),
        ("idle_north", &[0]),
        ("idle_south", &[23]),
        ("walk_east", &[23, 24, 23, 23]),
        ("walk_north", &[0, 0, 0, 0]),
        ("walk_south", &[23, 23, 23, 23]),
    ];
    // Wedding gold trim shares a skin shadow; preserve each reviewed coordinate.
    let trim: &[(&str, u32, u32, u32)] = &[
        ("idle_east", 0, 36, 43),
        ("idle_east", 0, 37, 43),
        ("idle_east", 0, 44, 43),
        ("idle_north", 0, 37, 41),
        ("idle_north", 0, 42, 41),
        ("idle_north", 0, 36, 43),
        ("idle_north", 0, 37, 43),
        ("idle_north", 0, 42, 43),
        ("idle_north", 0, 43, 43),
        ("idle_north", 0, 38, 44),
        ("idle_north", 0, 39, 44),
        ("idle_north", 0, 40, 44),
        ("idle_north", 0, 41, 44),
        ("idle_south", 0, 35, 43),
        ("idle_south", 0, 36, 43),
        ("idle_south", 0, 43, 43),
        ("idle_south", 0, 44, 43),
        ("walk_east", 0, 36, 43),
        ("walk_east", 0, 37, 43),
        ("walk_east", 0, 44, 43),
        ("walk_east", 1, 35, 43),
        ("walk_east", 2, 36, 43),
        ("walk_east", 2, 37, 43),
        ("walk_east", 2, 44, 43),
        ("walk_east", 1, 36, 44),
        ("walk_east", 1, 37, 44),
        ("walk_east", 1, 44, 44),
        ("walk_east", 3, 36, 44),
        ("walk_east", 3, 37, 44),
        ("walk_east", 3, 44, 44),
        ("walk_north", 0, 37, 41),
        ("walk_north", 0, 42, 41),
        ("walk_north", 2, 37, 41),
        ("walk_north", 2, 42, 41),
        ("walk_north", 1, 37, 42),
        ("walk_north", 1, 42, 42),
        ("walk_north", 3, 37, 42),
        ("walk_north", 3, 42, 42),
        ("walk_north", 0, 36, 43),
        ("walk_north", 0, 37, 43),
        ("walk_north", 0, 42, 43),
        ("walk_north", 0, 43, 43),
        ("walk_north", 2, 36, 43),
        ("walk_north", 2, 37, 43),
        ("walk_north", 2, 42, 43),
        ("walk_north", 2, 43, 43),
        ("walk_north", 0, 38, 44),
        ("walk_north", 0, 39, 44),
        ("walk_north", 0, 40, 44),
        ("walk_north", 0, 41, 44),
        ("walk_north", 1, 36, 44),
        ("walk_north", 1, 37, 44),
        ("walk_north", 1, 42, 44),
        ("walk_north", 1, 43, 44),
        ("walk_north", 2, 38, 44),
        ("walk_north", 2, 39, 44),
        ("walk_north", 2, 40, 44),
        ("walk_north", 2, 41, 44),
        ("walk_north", 3, 36, 44),
        ("walk_north", 3, 37, 44),
        ("walk_north", 3, 42, 44),
        ("walk_north", 3, 43, 44),
        ("walk_north", 1, 38, 45),
        ("walk_north", 1, 39, 45),
        ("walk_north", 1, 40, 45),
        ("walk_north", 1, 41, 45),
        ("walk_north", 3, 38, 45),
        ("walk_north", 3, 39, 45),
        ("walk_north", 3, 40, 45),
        ("walk_north", 3, 41, 45),
        ("walk_south", 0, 35, 43),
        ("walk_south", 0, 36, 43),
        ("walk_south", 0, 43, 43),
        ("walk_south", 0, 44, 43),
        ("walk_south", 2, 35, 43),
        ("walk_south", 2, 36, 43),
        ("walk_south", 2, 43, 43),
        ("walk_south", 2, 44, 43),
        ("walk_south", 1, 35, 44),
        ("walk_south", 1, 36, 44),
        ("walk_south", 1, 43, 44),
        ("walk_south", 1, 44, 44),
        ("walk_south", 3, 35, 44),
        ("walk_south", 3, 36, 44),
        ("walk_south", 3, 43, 44),
        ("walk_south", 3, 44, 44),
        ("walk_south", 1, 44, 46),
        ("walk_south", 3, 35, 46),
        ("walk_south", 1, 37, 51),
        ("walk_south", 1, 38, 51),
        ("walk_south", 3, 41, 51),
        ("walk_south", 3, 42, 51),
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
            let prefix = "spr_npc_eiland_wedding";
            let asset =
                format!("assets/animations/NPCs/Eiland/Sprites/Wedding/{prefix}_{case}.png");
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
                // Preserve the reviewed gold-trim pixels that share a skin shade.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !trim.contains(&(case, x / 80, x % 80, y)))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Eiland Wedding material mismatch: {id} {case} [{x},{y}]"
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
            // North shows only hair and clothing; retain the original PNG bytes.
            if case.ends_with("_north") {
                assert_eq!(
                    fs::read(original.join(&asset)).unwrap(),
                    fs::read(variant.join(&asset)).unwrap()
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
