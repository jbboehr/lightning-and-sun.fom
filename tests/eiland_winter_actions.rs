use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/eiland and the local accepted Eiland world baseline"]
fn eiland_winter_actions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/eiland");
    let baseline = root.join("generated/slice-005-build/characters/eiland");
    let set = std::env::var_os("FOM_EILAND_WINTER_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_WINTER_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..213];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..213], prior);
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
        ("blink_east", 0, 44, 48, 0x000000, false),
        ("blink_east", 0, 42, 48, 0x2B2432, false),
        ("blink_east", 0, 41, 48, 0x473C52, false),
        ("blink_east", 0, 36, 49, 0x585061, false),
        ("blink_east", 0, 38, 44, 0x685979, false),
        ("blink_east", 0, 37, 36, 0x6C2859, false),
        ("blink_east", 0, 38, 50, 0x927D96, false),
        ("blink_east", 0, 41, 33, 0x9C5241, true),
        ("blink_east", 0, 39, 32, 0xA54E7F, false),
        ("blink_east", 0, 43, 36, 0xA59DA2, false),
        ("blink_east", 0, 36, 43, 0xBA6A4C, true),
        ("blink_east", 0, 42, 45, 0xBBB5C7, false),
        ("blink_east", 0, 38, 36, 0xC2B9BE, false),
        ("blink_east", 0, 36, 42, 0xD2CCDE, false),
        ("blink_east", 0, 40, 39, 0xDE8F5D, true),
        ("blink_east", 0, 37, 32, 0xE797AC, false),
        ("blink_east", 0, 36, 37, 0xE9A980, true),
        ("blink_east", 0, 43, 37, 0xECF0E9, false),
        ("blink_east", 0, 38, 46, 0xEDE0EF, false),
        ("blink_east", 0, 41, 46, 0xF0BC70, false),
        ("blink_east", 0, 37, 42, 0xF4F4F4, false),
        ("blink_east", 0, 42, 32, 0xFCDAE0, false),
        ("blink_east", 0, 44, 31, 0xFFFFFF, false),
        ("blink_east", 1, 36, 43, 0xBA6A4C, true),
        ("blink_south", 1, 35, 43, 0xBA6A4C, true),
        ("drink_east", 1, 40, 45, 0xBA6A4C, true),
        ("drink_north", 1, 45, 45, 0xBA6A4C, true),
        ("drink_south", 1, 40, 41, 0xBA6A4C, true),
        ("drink_south", 1, 40, 40, 0xDE8F5D, true),
        ("eat_east", 0, 45, 45, 0x000000, false),
        ("eat_east", 0, 40, 47, 0x2B2432, false),
        ("eat_east", 0, 40, 46, 0x473C52, false),
        ("eat_east", 0, 40, 43, 0x685979, false),
        ("eat_east", 0, 37, 36, 0x6C2859, false),
        ("eat_east", 0, 44, 47, 0x927D96, false),
        ("eat_east", 0, 41, 33, 0x9C5241, true),
        ("eat_east", 0, 39, 32, 0xA54E7F, false),
        ("eat_east", 0, 38, 42, 0xBA6A4C, true),
        ("eat_east", 0, 37, 44, 0xBBB5C7, false),
        ("eat_east", 0, 39, 35, 0xC2B9BE, false),
        ("eat_east", 0, 44, 42, 0xD2CCDE, false),
        ("eat_east", 0, 40, 39, 0xDE8F5D, true),
        ("eat_east", 0, 37, 32, 0xE797AC, false),
        ("eat_east", 0, 36, 37, 0xE9A980, true),
        ("eat_east", 0, 38, 37, 0xECF0E9, false),
        ("eat_east", 0, 45, 48, 0xEDE0EF, false),
        ("eat_east", 0, 36, 46, 0xF0BC70, false),
        ("eat_east", 0, 43, 42, 0xF4F4F4, false),
        ("eat_east", 0, 42, 32, 0xFCDAE0, false),
        ("eat_east", 0, 44, 31, 0xFFFFFF, false),
        ("eat_east", 1, 40, 45, 0xBA6A4C, true),
        ("eat_north", 1, 45, 45, 0xBA6A4C, true),
        ("eat_south", 1, 36, 42, 0xBA6A4C, true),
        ("sit_south", 0, 40, 47, 0x000000, false),
        ("sit_south", 0, 38, 47, 0x2B2432, false),
        ("sit_south", 0, 45, 45, 0x473C52, false),
        ("sit_south", 0, 42, 44, 0x685979, false),
        ("sit_south", 0, 34, 37, 0x6C2859, false),
        ("sit_south", 0, 34, 47, 0x927D96, false),
        ("sit_south", 0, 40, 33, 0x9C5241, true),
        ("sit_south", 0, 38, 32, 0xA54E7F, false),
        ("sit_south", 0, 43, 41, 0xBA6A4C, true),
        ("sit_south", 0, 41, 43, 0xBBB5C7, false),
        ("sit_south", 0, 38, 35, 0xC2B9BE, false),
        ("sit_south", 0, 44, 46, 0xD2CCDE, false),
        ("sit_south", 0, 39, 39, 0xDE8F5D, true),
        ("sit_south", 0, 36, 32, 0xE797AC, false),
        ("sit_south", 0, 35, 37, 0xE9A980, true),
        ("sit_south", 0, 37, 37, 0xECF0E9, false),
        ("sit_south", 0, 37, 46, 0xEDE0EF, false),
        ("sit_south", 0, 39, 44, 0xF0BC70, false),
        ("sit_south", 0, 36, 42, 0xF4F4F4, false),
        ("sit_south", 0, 41, 32, 0xFCDAE0, false),
        ("sit_south", 0, 43, 31, 0xFFFFFF, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[32, 39, 32]),
        ("blink_south", &[32, 39, 32]),
        ("drink_east", &[29, 35, 29]),
        ("drink_north", &[8, 6, 8]),
        ("drink_south", &[28, 35, 28]),
        ("eat_east", &[31, 30, 24, 32, 29]),
        ("eat_north", &[8, 6, 8]),
        ("eat_south", &[29, 31, 20, 35, 29]),
        ("sit_east", &[28]),
        ("sit_north", &[8]),
        ("sit_south", &[29]),
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
            let prefix = "spr_npc_eiland_winter";
            let asset = format!("assets/animations/NPCs/Eiland/Sprites/Winter/{prefix}_{case}.png");
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
                // All reviewed world shades are skin in these Winter strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Eiland Winter material mismatch: {id} {case} [{x},{y}]"
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
