use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/eiland-summer-magnify-study and the local accepted Eiland world baseline"]
fn eiland_summer_standard_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/eiland-summer-magnify-study");
    let baseline = root.join(
        "generated/characters-balor-summer-finish-valen-eiland-actions-trial/characters/eiland",
    );
    let set = std::env::var_os("FOM_EILAND_SUMMER_STANDARD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_SUMMER_STANDARD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..142];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..142], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 162);
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
    // Literal source-grid landmarks cover skin and protected materials independently
    // of recipe seeds. Per-frame counts guard moving and briefly exposed skin.
    let landmarks = [
        ("action_east", 0, 38, 49, 0x000000, false),
        ("action_east", 0, 43, 47, 0x533061, false),
        ("action_east", 0, 38, 37, 0x6C2859, false),
        ("action_east", 0, 42, 44, 0x724E80, false),
        ("action_east", 0, 42, 51, 0x756279, false),
        ("action_east", 0, 40, 48, 0x7D3B14, true),
        ("action_east", 0, 43, 51, 0x927D96, false),
        ("action_east", 0, 42, 34, 0x9C5241, true),
        ("action_east", 0, 38, 44, 0xA54E7F, false),
        ("action_east", 0, 42, 45, 0xB475BA, false),
        ("action_east", 0, 42, 41, 0xBA6A4C, true),
        ("action_east", 0, 40, 50, 0xBBB5C7, false),
        ("action_east", 0, 38, 53, 0xC1BDC8, false),
        ("action_east", 0, 40, 36, 0xC2B9BE, false),
        ("action_east", 0, 43, 44, 0xDA8B36, false),
        ("action_east", 0, 39, 44, 0xDB5C81, false),
        ("action_east", 0, 42, 42, 0xDE8F5D, true),
        ("action_east", 0, 38, 33, 0xE797AC, false),
        ("action_east", 0, 41, 47, 0xE9A980, true),
        ("action_east", 0, 39, 38, 0xECF0E9, false),
        ("action_east", 0, 39, 52, 0xEDE0EF, false),
        ("action_east", 0, 43, 45, 0xF9C94D, false),
        ("action_east", 0, 43, 33, 0xFCDAE0, false),
        ("action_east", 0, 45, 32, 0xFFFFFF, false),
        ("action_east", 1, 48, 45, 0x7D3B14, true),
        ("action_east", 1, 44, 33, 0x9C5241, true),
        ("action_east", 1, 45, 44, 0xBA6A4C, true),
        ("action_east", 1, 46, 44, 0xDE8F5D, true),
        ("action_east", 1, 47, 44, 0xE9A980, true),
        ("action_east", 2, 45, 46, 0x7D3B14, true),
        ("action_east", 2, 44, 33, 0x9C5241, true),
        ("action_east", 2, 43, 45, 0xBA6A4C, true),
        ("action_east", 2, 44, 44, 0xDE8F5D, true),
        ("action_east", 2, 45, 45, 0xE9A980, true),
        ("action_east", 3, 48, 45, 0x7D3B14, true),
        ("action_east", 3, 44, 33, 0x9C5241, true),
        ("action_east", 3, 45, 44, 0xBA6A4C, true),
        ("action_east", 3, 46, 44, 0xDE8F5D, true),
        ("action_east", 3, 47, 44, 0xE9A980, true),
        ("action_east", 4, 45, 46, 0x7D3B14, true),
        ("action_east", 4, 44, 33, 0x9C5241, true),
        ("action_east", 4, 43, 45, 0xBA6A4C, true),
        ("action_east", 4, 44, 44, 0xDE8F5D, true),
        ("action_east", 4, 45, 45, 0xE9A980, true),
        ("action_east", 5, 40, 48, 0x7D3B14, true),
        ("action_east", 5, 42, 34, 0x9C5241, true),
        ("action_east", 5, 42, 41, 0xBA6A4C, true),
        ("action_east", 5, 42, 42, 0xDE8F5D, true),
        ("action_east", 5, 41, 47, 0xE9A980, true),
        ("action_east", 6, 35, 47, 0x7D3B14, true),
        ("action_east", 6, 41, 33, 0x9C5241, true),
        ("action_east", 6, 34, 45, 0xBA6A4C, true),
        ("action_east", 6, 35, 45, 0xDE8F5D, true),
        ("action_east", 6, 35, 46, 0xE9A980, true),
        ("action_north", 0, 35, 46, 0x000000, false),
        ("action_north", 0, 41, 46, 0x533061, false),
        ("action_north", 0, 44, 38, 0x6C2859, false),
        ("action_north", 0, 37, 45, 0x724E80, false),
        ("action_north", 0, 42, 50, 0x756279, false),
        ("action_north", 0, 45, 43, 0x7D3B14, true),
        ("action_north", 0, 38, 50, 0x927D96, false),
        ("action_north", 0, 38, 43, 0xA54E7F, false),
        ("action_north", 0, 40, 45, 0xB475BA, false),
        ("action_north", 0, 35, 44, 0xBA6A4C, true),
        ("action_north", 0, 44, 44, 0xBBB5C7, false),
        ("action_north", 0, 37, 52, 0xC1BDC8, false),
        ("action_north", 0, 38, 42, 0xDB5C81, false),
        ("action_north", 0, 34, 44, 0xDE8F5D, true),
        ("action_north", 0, 44, 34, 0xE797AC, false),
        ("action_north", 0, 32, 45, 0xE9A980, true),
        ("action_north", 0, 44, 43, 0xEDE0EF, false),
        ("action_north", 0, 34, 35, 0xFCDAE0, false),
        ("action_north", 0, 37, 33, 0xFFFFFF, false),
        ("action_north", 1, 35, 46, 0x7D3B14, true),
        ("action_north", 1, 34, 44, 0xBA6A4C, true),
        ("action_north", 1, 36, 44, 0xDE8F5D, true),
        ("action_north", 1, 36, 45, 0xE9A980, true),
        ("action_north", 2, 35, 46, 0x7D3B14, true),
        ("action_north", 2, 34, 44, 0xBA6A4C, true),
        ("action_north", 2, 36, 44, 0xDE8F5D, true),
        ("action_north", 2, 36, 45, 0xE9A980, true),
        ("action_north", 3, 35, 46, 0x7D3B14, true),
        ("action_north", 3, 34, 44, 0xBA6A4C, true),
        ("action_north", 3, 36, 44, 0xDE8F5D, true),
        ("action_north", 3, 36, 45, 0xE9A980, true),
        ("action_north", 4, 35, 46, 0x7D3B14, true),
        ("action_north", 4, 34, 44, 0xBA6A4C, true),
        ("action_north", 4, 36, 44, 0xDE8F5D, true),
        ("action_north", 4, 36, 45, 0xE9A980, true),
        ("action_north", 5, 45, 43, 0x7D3B14, true),
        ("action_north", 5, 46, 44, 0xBA6A4C, true),
        ("action_north", 5, 34, 45, 0xDE8F5D, true),
        ("action_north", 5, 35, 46, 0xE9A980, true),
        ("action_north", 6, 46, 47, 0x7D3B14, true),
        ("action_north", 6, 44, 45, 0xBA6A4C, true),
        ("action_north", 6, 45, 45, 0xDE8F5D, true),
        ("action_north", 6, 47, 46, 0xE9A980, true),
        ("action_south", 0, 46, 48, 0x000000, false),
        ("action_south", 0, 41, 47, 0x533061, false),
        ("action_south", 0, 34, 38, 0x6C2859, false),
        ("action_south", 0, 42, 45, 0x724E80, false),
        ("action_south", 0, 42, 51, 0x756279, false),
        ("action_south", 0, 44, 46, 0x7D3B14, true),
        ("action_south", 0, 38, 51, 0x927D96, false),
        ("action_south", 0, 40, 34, 0x9C5241, true),
        ("action_south", 0, 42, 44, 0xA54E7F, false),
        ("action_south", 0, 38, 46, 0xB475BA, false),
        ("action_south", 0, 45, 45, 0xBA6A4C, true),
        ("action_south", 0, 44, 45, 0xBBB5C7, false),
        ("action_south", 0, 42, 52, 0xC1BDC8, false),
        ("action_south", 0, 38, 36, 0xC2B9BE, false),
        ("action_south", 0, 41, 44, 0xDA8B36, false),
        ("action_south", 0, 36, 44, 0xDB5C81, false),
        ("action_south", 0, 40, 42, 0xDE8F5D, true),
        ("action_south", 0, 36, 33, 0xE797AC, false),
        ("action_south", 0, 35, 47, 0xE9A980, true),
        ("action_south", 0, 37, 38, 0xECF0E9, false),
        ("action_south", 0, 38, 52, 0xEDE0EF, false),
        ("action_south", 0, 41, 45, 0xF9C94D, false),
        ("action_south", 0, 41, 33, 0xFCDAE0, false),
        ("action_south", 0, 43, 32, 0xFFFFFF, false),
        ("action_south", 1, 35, 47, 0x7D3B14, true),
        ("action_south", 1, 40, 33, 0x9C5241, true),
        ("action_south", 1, 34, 45, 0xBA6A4C, true),
        ("action_south", 1, 35, 45, 0xDE8F5D, true),
        ("action_south", 1, 36, 46, 0xE9A980, true),
        ("action_south", 2, 38, 47, 0x7D3B14, true),
        ("action_south", 2, 40, 33, 0x9C5241, true),
        ("action_south", 2, 44, 45, 0xBA6A4C, true),
        ("action_south", 2, 37, 45, 0xDE8F5D, true),
        ("action_south", 2, 39, 46, 0xE9A980, true),
        ("action_south", 3, 35, 47, 0x7D3B14, true),
        ("action_south", 3, 40, 33, 0x9C5241, true),
        ("action_south", 3, 34, 45, 0xBA6A4C, true),
        ("action_south", 3, 35, 45, 0xDE8F5D, true),
        ("action_south", 3, 36, 46, 0xE9A980, true),
        ("action_south", 4, 38, 47, 0x7D3B14, true),
        ("action_south", 4, 40, 33, 0x9C5241, true),
        ("action_south", 4, 44, 45, 0xBA6A4C, true),
        ("action_south", 4, 37, 45, 0xDE8F5D, true),
        ("action_south", 4, 39, 46, 0xE9A980, true),
        ("action_south", 5, 44, 46, 0x7D3B14, true),
        ("action_south", 5, 40, 34, 0x9C5241, true),
        ("action_south", 5, 45, 45, 0xBA6A4C, true),
        ("action_south", 5, 40, 42, 0xDE8F5D, true),
        ("action_south", 5, 35, 47, 0xE9A980, true),
        ("action_south", 6, 46, 47, 0x7D3B14, true),
        ("action_south", 6, 40, 33, 0x9C5241, true),
        ("action_south", 6, 35, 45, 0xBA6A4C, true),
        ("action_south", 6, 34, 45, 0xDE8F5D, true),
        ("action_south", 6, 47, 46, 0xE9A980, true),
        ("kiss_east", 0, 44, 48, 0x000000, false),
        ("kiss_east", 0, 40, 47, 0x533061, false),
        ("kiss_east", 0, 36, 37, 0x6C2859, false),
        ("kiss_east", 0, 40, 44, 0x724E80, false),
        ("kiss_east", 0, 41, 51, 0x756279, false),
        ("kiss_east", 0, 35, 48, 0x7D3B14, true),
        ("kiss_east", 0, 38, 51, 0x927D96, false),
        ("kiss_east", 0, 40, 34, 0x9C5241, true),
        ("kiss_east", 0, 37, 44, 0xA54E7F, false),
        ("kiss_east", 0, 38, 46, 0xB475BA, false),
        ("kiss_east", 0, 40, 41, 0xBA6A4C, true),
        ("kiss_east", 0, 43, 45, 0xBBB5C7, false),
        ("kiss_east", 0, 37, 53, 0xC1BDC8, false),
        ("kiss_east", 0, 38, 36, 0xC2B9BE, false),
        ("kiss_east", 0, 41, 44, 0xDA8B36, false),
        ("kiss_east", 0, 42, 43, 0xDB5C81, false),
        ("kiss_east", 0, 39, 42, 0xDE8F5D, true),
        ("kiss_east", 0, 36, 33, 0xE797AC, false),
        ("kiss_east", 0, 35, 47, 0xE9A980, true),
        ("kiss_east", 0, 37, 38, 0xECF0E9, false),
        ("kiss_east", 0, 38, 52, 0xEDE0EF, false),
        ("kiss_east", 0, 41, 45, 0xF9C94D, false),
        ("kiss_east", 0, 41, 33, 0xFCDAE0, false),
        ("kiss_east", 0, 43, 32, 0xFFFFFF, false),
        ("kiss_east", 1, 36, 48, 0x7D3B14, true),
        ("kiss_east", 1, 42, 34, 0x9C5241, true),
        ("kiss_east", 1, 42, 41, 0xBA6A4C, true),
        ("kiss_east", 1, 41, 42, 0xDE8F5D, true),
        ("kiss_east", 1, 36, 47, 0xE9A980, true),
        ("kiss_east", 2, 36, 46, 0x7D3B14, true),
        ("kiss_east", 2, 44, 33, 0x9C5241, true),
        ("kiss_east", 2, 35, 44, 0xBA6A4C, true),
        ("kiss_east", 2, 43, 41, 0xDE8F5D, true),
        ("kiss_east", 2, 35, 45, 0xE9A980, true),
        ("kiss_east", 3, 36, 48, 0x7D3B14, true),
        ("kiss_east", 3, 42, 34, 0x9C5241, true),
        ("kiss_east", 3, 42, 41, 0xBA6A4C, true),
        ("kiss_east", 3, 41, 42, 0xDE8F5D, true),
        ("kiss_east", 3, 36, 47, 0xE9A980, true),
        ("sleep_east", 0, 40, 48, 0x000000, false),
        ("sleep_east", 0, 41, 46, 0x533061, false),
        ("sleep_east", 0, 37, 35, 0x6C2859, false),
        ("sleep_east", 0, 43, 47, 0x724E80, false),
        ("sleep_east", 0, 41, 50, 0x756279, false),
        ("sleep_east", 0, 44, 39, 0x7D3B14, true),
        ("sleep_east", 0, 41, 49, 0x927D96, false),
        ("sleep_east", 0, 41, 33, 0x9C5241, true),
        ("sleep_east", 0, 38, 43, 0xA54E7F, false),
        ("sleep_east", 0, 39, 45, 0xB475BA, false),
        ("sleep_east", 0, 42, 40, 0xBA6A4C, true),
        ("sleep_east", 0, 41, 42, 0xBBB5C7, false),
        ("sleep_east", 0, 38, 52, 0xC1BDC8, false),
        ("sleep_east", 0, 42, 48, 0xDA8B36, false),
        ("sleep_east", 0, 39, 42, 0xDB5C81, false),
        ("sleep_east", 0, 43, 41, 0xDE8F5D, true),
        ("sleep_east", 0, 37, 32, 0xE797AC, false),
        ("sleep_east", 0, 45, 40, 0xE9A980, true),
        ("sleep_east", 0, 42, 51, 0xEDE0EF, false),
        ("sleep_east", 0, 40, 45, 0xF9C94D, false),
        ("sleep_east", 0, 42, 32, 0xFCDAE0, false),
        ("sleep_east", 0, 44, 31, 0xFFFFFF, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[37, 38, 36, 38, 36, 37, 38]),
        ("action_north", &[15, 9, 9, 9, 9, 16, 18]),
        ("action_south", &[43, 40, 40, 40, 40, 43, 46]),
        ("kiss_east", &[37, 40, 45, 44]),
        ("sleep_east", &[39]),
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
            let prefix = "spr_npc_eiland_summer";
            let asset = format!("assets/animations/NPCs/Eiland/Sprites/Summer/{prefix}_{case}.png");
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
                    "Eiland Summer material mismatch: {id} {case} [{x},{y}]"
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
