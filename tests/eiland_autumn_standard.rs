use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/eiland-autumn-writing-study and the local accepted Eiland world baseline"]
fn eiland_autumn_standard_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/eiland-autumn-writing-study");
    let baseline = root.join(
        "generated/characters-balor-winter-actions-valen-finish-eiland-autumn-actions-trial/characters/eiland",
    );
    let set = std::env::var_os("FOM_EILAND_AUTUMN_STANDARD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_AUTUMN_STANDARD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..183];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..183], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 194);
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
        ("action_east", 0, 34, 49, 0x000000, false),
        ("action_east", 0, 43, 50, 0x2B2432, false),
        ("action_east", 0, 40, 46, 0x473C52, false),
        ("action_east", 0, 42, 52, 0x685979, false),
        ("action_east", 0, 38, 37, 0x6C2859, false),
        ("action_east", 0, 43, 48, 0x735E90, false),
        ("action_east", 0, 40, 48, 0x7D3B14, true),
        ("action_east", 0, 43, 47, 0x927D96, false),
        ("action_east", 0, 42, 34, 0x9C5241, true),
        ("action_east", 0, 39, 44, 0xA54E7F, false),
        ("action_east", 0, 36, 50, 0xAA93C8, false),
        ("action_east", 0, 42, 41, 0xBA6A4C, true),
        ("action_east", 0, 42, 43, 0xBBB5C7, false),
        ("action_east", 0, 39, 51, 0xC1BDC8, false),
        ("action_east", 0, 40, 36, 0xC2B9BE, false),
        ("action_east", 0, 36, 48, 0xC9785A, false),
        ("action_east", 0, 38, 43, 0xD2CCDE, false),
        ("action_east", 0, 42, 44, 0xDB5C81, false),
        ("action_east", 0, 42, 40, 0xDE8F5D, true),
        ("action_east", 0, 38, 33, 0xE797AC, false),
        ("action_east", 0, 41, 47, 0xE9A980, true),
        ("action_east", 0, 39, 38, 0xECF0E9, false),
        ("action_east", 0, 39, 53, 0xEDE0EF, false),
        ("action_east", 0, 41, 50, 0xF0BC70, false),
        ("action_east", 0, 39, 43, 0xF4F4F4, false),
        ("action_east", 0, 43, 33, 0xFCDAE0, false),
        ("action_east", 0, 45, 32, 0xFFFFFF, false),
        ("action_east", 1, 48, 45, 0x7D3B14, true),
        ("action_east", 1, 47, 45, 0xDE8F5D, true),
        ("action_east", 1, 47, 44, 0xE9A980, true),
        ("action_south", 0, 46, 48, 0x000000, false),
        ("action_south", 0, 42, 49, 0x2B2432, false),
        ("action_south", 0, 37, 47, 0x473C52, false),
        ("action_south", 0, 42, 45, 0x5D457D, false),
        ("action_south", 0, 42, 52, 0x685979, false),
        ("action_south", 0, 34, 38, 0x6C2859, false),
        ("action_south", 0, 39, 48, 0x735E90, false),
        ("action_south", 0, 46, 47, 0x7D3B14, true),
        ("action_south", 0, 44, 45, 0x927D96, false),
        ("action_south", 0, 40, 34, 0x9C5241, true),
        ("action_south", 0, 38, 44, 0xA54E7F, false),
        ("action_south", 0, 44, 50, 0xAA93C8, false),
        ("action_south", 0, 42, 46, 0xAF97B4, false),
        ("action_south", 0, 46, 46, 0xBA6A4C, true),
        ("action_south", 0, 38, 46, 0xBBB5C7, false),
        ("action_south", 0, 38, 51, 0xC1BDC8, false),
        ("action_south", 0, 38, 36, 0xC2B9BE, false),
        ("action_south", 0, 43, 42, 0xC9785A, false),
        ("action_south", 0, 35, 43, 0xD2CCDE, false),
        ("action_south", 0, 39, 44, 0xDB5C81, false),
        ("action_south", 0, 40, 40, 0xDE8F5D, true),
        ("action_south", 0, 36, 33, 0xE797AC, false),
        ("action_south", 0, 35, 47, 0xE9A980, true),
        ("action_south", 0, 37, 38, 0xECF0E9, false),
        ("action_south", 0, 37, 53, 0xEDE0EF, false),
        ("action_south", 0, 34, 50, 0xF0BC70, false),
        ("action_south", 0, 36, 43, 0xF4F4F4, false),
        ("action_south", 0, 41, 33, 0xFCDAE0, false),
        ("action_south", 0, 43, 32, 0xFFFFFF, false),
        ("action_south", 1, 35, 47, 0x7D3B14, true),
        ("action_south", 1, 45, 45, 0xBA6A4C, true),
        ("action_south", 1, 36, 46, 0xE9A980, true),
        ("kiss_east", 1, 36, 48, 0x7D3B14, true),
        ("kiss_east", 1, 42, 41, 0xBA6A4C, true),
        ("kiss_east", 1, 42, 40, 0xDE8F5D, true),
        ("kiss_east", 1, 37, 47, 0xE9A980, true),
        ("sleep_east", 0, 37, 48, 0x000000, false),
        ("sleep_east", 0, 39, 49, 0x2B2432, false),
        ("sleep_east", 0, 38, 46, 0x473C52, false),
        ("sleep_east", 0, 38, 44, 0x5D457D, false),
        ("sleep_east", 0, 41, 52, 0x685979, false),
        ("sleep_east", 0, 37, 35, 0x6C2859, false),
        ("sleep_east", 0, 39, 47, 0x735E90, false),
        ("sleep_east", 0, 44, 39, 0x7D3B14, true),
        ("sleep_east", 0, 38, 50, 0x927D96, false),
        ("sleep_east", 0, 41, 33, 0x9C5241, true),
        ("sleep_east", 0, 38, 43, 0xA54E7F, false),
        ("sleep_east", 0, 36, 48, 0xAA93C8, false),
        ("sleep_east", 0, 38, 45, 0xAF97B4, false),
        ("sleep_east", 0, 35, 47, 0xBA6A4C, false),
        ("sleep_east", 0, 40, 46, 0xBBB5C7, false),
        ("sleep_east", 0, 42, 50, 0xC1BDC8, false),
        ("sleep_east", 0, 38, 41, 0xC9785A, false),
        ("sleep_east", 0, 39, 42, 0xD2CCDE, false),
        ("sleep_east", 0, 40, 39, 0xDE8F5D, true),
        ("sleep_east", 0, 37, 32, 0xE797AC, false),
        ("sleep_east", 0, 44, 40, 0xE9A980, true),
        ("sleep_east", 0, 40, 53, 0xEDE0EF, false),
        ("sleep_east", 0, 34, 48, 0xF0BC70, false),
        ("sleep_east", 0, 38, 42, 0xF4F4F4, false),
        ("sleep_east", 0, 42, 32, 0xFCDAE0, false),
        ("sleep_east", 0, 44, 31, 0xFFFFFF, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[31, 34, 31, 34, 31, 31, 34]),
        ("action_north", &[6, 0, 1, 0, 1, 3, 4]),
        ("action_south", &[36, 36, 35, 36, 35, 36, 38]),
        ("kiss_east", &[32, 35, 41, 39]),
        ("sleep_east", &[35]),
    ];
    // Thirty-four individually reviewed clothing pixels share the skin shadow color.
    let clothing: &[(&str, u32, u32, u32)] = &[
        ("action_east", 1, 38, 44),
        ("action_east", 1, 38, 45),
        ("action_east", 1, 40, 47),
        ("action_east", 1, 35, 48),
        ("action_east", 2, 38, 44),
        ("action_east", 2, 38, 45),
        ("action_east", 2, 40, 47),
        ("action_east", 2, 35, 48),
        ("action_east", 3, 38, 44),
        ("action_east", 3, 38, 45),
        ("action_east", 3, 40, 47),
        ("action_east", 3, 35, 48),
        ("action_east", 4, 38, 44),
        ("action_east", 4, 38, 45),
        ("action_east", 4, 40, 47),
        ("action_east", 4, 35, 48),
        ("action_east", 6, 38, 47),
        ("action_south", 0, 37, 48),
        ("action_south", 0, 42, 48),
        ("action_south", 1, 42, 47),
        ("action_south", 2, 37, 47),
        ("action_south", 2, 42, 47),
        ("action_south", 3, 42, 47),
        ("action_south", 4, 37, 47),
        ("action_south", 4, 42, 47),
        ("action_south", 5, 37, 48),
        ("action_south", 5, 42, 48),
        ("action_south", 6, 37, 47),
        ("action_south", 6, 42, 47),
        ("kiss_east", 2, 39, 47),
        ("sleep_east", 0, 36, 43),
        ("sleep_east", 0, 36, 44),
        ("sleep_east", 0, 35, 47),
        ("sleep_east", 0, 38, 47),
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
            let prefix = "spr_npc_eiland_autumn";
            let asset = format!("assets/animations/NPCs/Eiland/Sprites/Autumn/{prefix}_{case}.png");
            let before = image::open(original.join(&asset)).unwrap().to_rgba8();
            let after = image::open(variant.join(&asset)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), (80 * frames, 80));
            assert_eq!(before.dimensions(), after.dimensions());
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(variant.join(meta)).unwrap()
            );
            for &(name, frame, x, y) in clothing {
                if name == case {
                    assert_eq!(before.get_pixel(frame * 80 + x, y).0, rgba(0xBA6A4C));
                }
            }
            let mut per_frame = vec![0; frames as usize];
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                // The shared shadow shade also occurs in the thirty-four clothing pixels above.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !clothing.contains(&(case, x / 80, x % 80, y)))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Eiland Autumn material mismatch: {id} {case} [{x},{y}]"
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
                assert_eq!(per_frame, [6, 0, 1, 0, 1, 3, 4]);
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
