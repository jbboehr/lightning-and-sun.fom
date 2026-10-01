use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/eiland and the local accepted Eiland world baseline"]
fn eiland_autumn_finish_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/eiland");
    let baseline = root.join("generated/slice-003-build/characters/eiland");
    let set = std::env::var_os("FOM_EILAND_AUTUMN_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_AUTUMN_FINISH_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..203];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..203], prior);
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
        ("axe_east", 0, 43, 48, 0x000000, false),
        ("axe_east", 0, 43, 26, 0x255894, false),
        ("axe_east", 0, 42, 49, 0x2B2432, false),
        ("axe_east", 0, 43, 27, 0x3E80BA, false),
        ("axe_east", 0, 38, 49, 0x473C52, false),
        ("axe_east", 0, 41, 28, 0x58A8E1, false),
        ("axe_east", 0, 38, 44, 0x5D457D, false),
        ("axe_east", 0, 42, 29, 0x623B31, false),
        ("axe_east", 0, 41, 52, 0x685979, false),
        ("axe_east", 0, 36, 35, 0x6C2859, false),
        ("axe_east", 0, 39, 47, 0x735E90, false),
        ("axe_east", 0, 41, 36, 0x7D3B14, true),
        ("axe_east", 0, 44, 28, 0x853317, false),
        ("axe_east", 0, 42, 30, 0x8E6146, false),
        ("axe_east", 0, 42, 46, 0x927D96, false),
        ("axe_east", 0, 42, 27, 0x95D9F9, false),
        ("axe_east", 0, 39, 33, 0x9C5241, true),
        ("axe_east", 0, 36, 43, 0xA54E7F, false),
        ("axe_east", 0, 36, 48, 0xAA93C8, false),
        ("axe_east", 0, 38, 45, 0xAF97B4, false),
        ("axe_east", 0, 42, 32, 0xB88C54, false),
        ("axe_east", 0, 38, 47, 0xBA6A4C, false),
        ("axe_east", 0, 42, 45, 0xBBB5C7, false),
        ("axe_east", 0, 48, 27, 0xBEF9FD, false),
        ("axe_east", 0, 42, 50, 0xC1BDC8, false),
        ("axe_east", 0, 45, 26, 0xC4633D, false),
        ("axe_east", 0, 36, 44, 0xC9785A, false),
        ("axe_east", 0, 36, 42, 0xD2CCDE, false),
        ("axe_east", 0, 40, 43, 0xDB5C81, false),
        ("axe_east", 0, 37, 37, 0xDE8F5D, true),
        ("axe_east", 0, 36, 32, 0xE797AC, false),
        ("axe_east", 0, 41, 27, 0xE9A482, false),
        ("axe_east", 0, 40, 36, 0xE9A980, true),
        ("axe_east", 0, 39, 53, 0xEDE0EF, false),
        ("axe_east", 0, 35, 48, 0xF0BC70, false),
        ("axe_east", 0, 45, 31, 0xFCDAE0, false),
        ("axe_east", 0, 37, 32, 0xFFFFFF, false),
        ("axe_east", 1, 45, 42, 0x7D3B14, true),
        ("axe_east", 1, 48, 43, 0xBA6A4C, true),
        ("axe_east", 1, 53, 44, 0xDE8F5D, true),
        ("axe_east", 1, 48, 41, 0xE9A980, true),
        ("brush_east", 1, 48, 45, 0x7D3B14, true),
        ("brush_east", 1, 47, 45, 0xDE8F5D, true),
        ("brush_east", 1, 47, 44, 0xE9A980, true),
        ("pickaxe_east", 0, 43, 48, 0x000000, false),
        ("pickaxe_east", 0, 42, 49, 0x2B2432, false),
        ("pickaxe_east", 0, 38, 49, 0x473C52, false),
        ("pickaxe_east", 0, 38, 44, 0x5D457D, false),
        ("pickaxe_east", 0, 41, 52, 0x685979, false),
        ("pickaxe_east", 0, 36, 35, 0x6C2859, false),
        ("pickaxe_east", 0, 39, 47, 0x735E90, false),
        ("pickaxe_east", 0, 41, 36, 0x7D3B14, true),
        ("pickaxe_east", 0, 42, 46, 0x927D96, false),
        ("pickaxe_east", 0, 40, 33, 0x9C5241, true),
        ("pickaxe_east", 0, 36, 43, 0xA54E7F, false),
        ("pickaxe_east", 0, 36, 48, 0xAA93C8, false),
        ("pickaxe_east", 0, 38, 45, 0xAF97B4, false),
        ("pickaxe_east", 0, 38, 47, 0xBA6A4C, false),
        ("pickaxe_east", 0, 42, 45, 0xBBB5C7, false),
        ("pickaxe_east", 0, 41, 23, 0xBE6D44, false),
        ("pickaxe_east", 0, 42, 50, 0xC1BDC8, false),
        ("pickaxe_east", 0, 36, 44, 0xC9785A, false),
        ("pickaxe_east", 0, 36, 42, 0xD2CCDE, false),
        ("pickaxe_east", 0, 40, 43, 0xDB5C81, false),
        ("pickaxe_east", 0, 37, 37, 0xDE8F5D, true),
        ("pickaxe_east", 0, 34, 32, 0xE797AC, false),
        ("pickaxe_east", 0, 40, 36, 0xE9A980, true),
        ("pickaxe_east", 0, 39, 53, 0xEDE0EF, false),
        ("pickaxe_east", 0, 35, 48, 0xF0BC70, false),
        ("pickaxe_east", 0, 41, 32, 0xFCDAE0, false),
        ("pickaxe_east", 0, 50, 25, 0xFFCF36, false),
        ("pickaxe_east", 0, 44, 23, 0xFFF672, false),
        ("pickaxe_east", 0, 50, 24, 0xFFFFFF, false),
        ("pickaxe_east", 1, 45, 42, 0x7D3B14, true),
        ("pickaxe_east", 1, 48, 43, 0xBA6A4C, true),
        ("pickaxe_east", 1, 49, 42, 0xDE8F5D, true),
        ("pickaxe_east", 1, 48, 41, 0xE9A980, true),
        ("trowel_east", 0, 46, 48, 0x000000, false),
        ("trowel_east", 0, 40, 50, 0x2B2432, false),
        ("trowel_east", 0, 39, 50, 0x473C52, false),
        ("trowel_east", 0, 37, 43, 0x6C2859, false),
        ("trowel_east", 0, 44, 52, 0x70797D, false),
        ("trowel_east", 0, 42, 49, 0x735E90, false),
        ("trowel_east", 0, 40, 53, 0x7D3B14, true),
        ("trowel_east", 0, 38, 49, 0x927D96, false),
        ("trowel_east", 0, 42, 38, 0x9C5241, true),
        ("trowel_east", 0, 43, 53, 0x9EB0BB, false),
        ("trowel_east", 0, 40, 47, 0xA54E7F, false),
        ("trowel_east", 0, 42, 45, 0xBA6A4C, true),
        ("trowel_east", 0, 42, 47, 0xBBB5C7, false),
        ("trowel_east", 0, 39, 49, 0xC1BDC8, false),
        ("trowel_east", 0, 40, 40, 0xC2B9BE, false),
        ("trowel_east", 0, 45, 53, 0xC5CECF, false),
        ("trowel_east", 0, 44, 46, 0xC9785A, false),
        ("trowel_east", 0, 38, 47, 0xD2CCDE, false),
        ("trowel_east", 0, 42, 48, 0xDB5C81, false),
        ("trowel_east", 0, 42, 44, 0xDE8F5D, true),
        ("trowel_east", 0, 47, 41, 0xE797AC, false),
        ("trowel_east", 0, 42, 43, 0xE9A980, true),
        ("trowel_east", 0, 39, 42, 0xECF0E9, false),
        ("trowel_east", 0, 42, 46, 0xEDE0EF, false),
        ("trowel_east", 0, 40, 46, 0xF0BC70, false),
        ("trowel_east", 0, 39, 47, 0xF4F4F4, false),
        ("trowel_east", 0, 47, 40, 0xFCDAE0, false),
        ("trowel_east", 0, 45, 36, 0xFFFFFF, false),
        ("trowel_east", 1, 42, 52, 0x7D3B14, true),
        ("trowel_east", 1, 42, 45, 0xBA6A4C, true),
        ("trowel_east", 1, 42, 44, 0xDE8F5D, true),
        ("trowel_east", 1, 43, 42, 0xE9A980, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("axe_east", &[18, 47, 45, 37, 35, 18]),
        ("brush_east", &[31, 34, 31, 34, 31, 31, 34]),
        ("pickaxe_east", &[19, 46, 44, 36, 35, 19]),
        ("trowel_east", &[38, 31, 31, 31, 31, 31, 31, 31, 31, 38]),
    ];
    // Thirty-three individually reviewed clothing pixels share the skin shadow color.
    let clothing: &[(&str, u32, u32, u32)] = &[
        ("axe_east", 0, 38, 47),
        ("axe_east", 1, 40, 47),
        ("axe_east", 2, 40, 47),
        ("axe_east", 3, 40, 47),
        ("axe_east", 5, 38, 47),
        ("brush_east", 1, 38, 44),
        ("brush_east", 1, 38, 45),
        ("brush_east", 1, 40, 47),
        ("brush_east", 1, 35, 48),
        ("brush_east", 2, 38, 44),
        ("brush_east", 2, 38, 45),
        ("brush_east", 2, 40, 47),
        ("brush_east", 2, 35, 48),
        ("brush_east", 3, 38, 44),
        ("brush_east", 3, 38, 45),
        ("brush_east", 3, 40, 47),
        ("brush_east", 3, 35, 48),
        ("brush_east", 4, 38, 44),
        ("brush_east", 4, 38, 45),
        ("brush_east", 4, 40, 47),
        ("brush_east", 4, 35, 48),
        ("brush_east", 6, 38, 47),
        ("pickaxe_east", 0, 38, 47),
        ("pickaxe_east", 1, 40, 47),
        ("pickaxe_east", 2, 40, 47),
        ("pickaxe_east", 3, 40, 47),
        ("pickaxe_east", 5, 38, 47),
        ("trowel_east", 2, 41, 51),
        ("trowel_east", 3, 41, 51),
        ("trowel_east", 4, 41, 51),
        ("trowel_east", 5, 41, 51),
        ("trowel_east", 6, 41, 51),
        ("trowel_east", 7, 41, 51),
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
            let prefix = "spr_npc_eiland_specialanimation_autumn";
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
                // The shared shadow shade also occurs in the thirty-three clothing pixels above.
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
