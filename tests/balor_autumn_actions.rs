use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/balor-winter-world-study and the local accepted Balor world baseline"]
fn balor_autumn_actions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-winter-world-study");
    let baseline = root.join(
        "generated/characters-balor-autumn-valen-writing-eiland-standard-trial/characters/balor",
    );
    let set = std::env::var_os("FOM_BALOR_AUTUMN_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_BALOR_AUTUMN_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/balor-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..179];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..179], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 207);
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
    let source = [0xFCD9B3, 0xF0B988, 0xD37A57, 0x672115];
    // Literal source-grid landmarks cover skin and protected materials independently
    // of recipe seeds. Per-frame counts guard moving and briefly exposed skin.
    let landmarks = [
        ("blink_east", 0, 34, 48, 0x000000, false),
        ("blink_east", 0, 39, 50, 0x262C49, false),
        ("blink_east", 0, 36, 40, 0x281846, false),
        ("blink_east", 0, 38, 43, 0x380F1C, false),
        ("blink_east", 0, 42, 48, 0x3A4863, false),
        ("blink_east", 0, 41, 33, 0x4A3D66, false),
        ("blink_east", 0, 38, 47, 0x51303E, false),
        ("blink_east", 0, 43, 41, 0x561635, false),
        ("blink_east", 0, 35, 46, 0x672115, true),
        ("blink_east", 0, 40, 42, 0x67243B, false),
        ("blink_east", 0, 42, 45, 0x686589, false),
        ("blink_east", 0, 41, 40, 0x6F5A54, false),
        ("blink_east", 0, 37, 47, 0x815065, false),
        ("blink_east", 0, 41, 30, 0x9B83B7, false),
        ("blink_east", 0, 40, 41, 0xA19085, false),
        ("blink_east", 0, 43, 36, 0xA59DA2, false),
        ("blink_east", 0, 38, 36, 0xC2B9BE, false),
        ("blink_east", 0, 34, 46, 0xD37A57, true),
        ("blink_east", 0, 41, 46, 0xECF0E9, false),
        ("blink_east", 0, 45, 45, 0xF0B988, true),
        ("blink_east", 0, 35, 45, 0xFCD9B3, true),
        ("eat_east", 0, 43, 46, 0x000000, false),
        ("eat_east", 0, 40, 47, 0x262C49, false),
        ("eat_east", 0, 37, 40, 0x281846, false),
        ("eat_east", 0, 39, 44, 0x380F1C, false),
        ("eat_east", 0, 44, 49, 0x3A4863, false),
        ("eat_east", 0, 39, 34, 0x4A3D66, false),
        ("eat_east", 0, 38, 47, 0x51303E, false),
        ("eat_east", 0, 39, 42, 0x561635, false),
        ("eat_east", 0, 38, 39, 0x672115, true),
        ("eat_east", 0, 38, 42, 0x67243B, false),
        ("eat_east", 0, 42, 48, 0x686589, false),
        ("eat_east", 0, 40, 43, 0x6F5A54, false),
        ("eat_east", 0, 37, 48, 0x815065, false),
        ("eat_east", 0, 41, 31, 0x9B83B7, false),
        ("eat_east", 0, 41, 42, 0xA19085, false),
        ("eat_east", 0, 43, 36, 0xA59DA2, false),
        ("eat_east", 0, 38, 36, 0xC2B9BE, false),
        ("eat_east", 0, 41, 40, 0xD37A57, true),
        ("eat_east", 0, 40, 45, 0xECF0E9, false),
        ("eat_east", 0, 44, 42, 0xF0B988, true),
        ("eat_east", 0, 42, 43, 0xFCD9B3, true),
        ("sit_south", 0, 33, 48, 0x000000, false),
        ("sit_south", 0, 42, 48, 0x262C49, false),
        ("sit_south", 0, 43, 40, 0x281846, false),
        ("sit_south", 0, 36, 44, 0x380F1C, false),
        ("sit_south", 0, 42, 47, 0x3A4863, false),
        ("sit_south", 0, 40, 33, 0x4A3D66, false),
        ("sit_south", 0, 42, 46, 0x51303E, false),
        ("sit_south", 0, 42, 42, 0x561635, false),
        ("sit_south", 0, 45, 47, 0x672115, true),
        ("sit_south", 0, 37, 43, 0x67243B, false),
        ("sit_south", 0, 41, 45, 0x686589, false),
        ("sit_south", 0, 42, 41, 0x6F5A54, false),
        ("sit_south", 0, 41, 46, 0x815065, false),
        ("sit_south", 0, 40, 31, 0x9B83B7, false),
        ("sit_south", 0, 40, 42, 0xA19085, false),
        ("sit_south", 0, 42, 36, 0xC2B9BE, false),
        ("sit_south", 0, 40, 40, 0xD37A57, true),
        ("sit_south", 0, 40, 46, 0xECF0E9, false),
        ("sit_south", 0, 35, 46, 0xF0B988, true),
        ("sit_south", 0, 40, 37, 0xFCD9B3, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[36, 40, 36]),
        ("blink_south", &[45, 49, 45]),
        ("drink_east", &[29, 35, 29]),
        ("drink_north", &[6, 5, 6]),
        ("drink_south", &[38, 41, 38]),
        ("eat_east", &[27, 30, 25, 29, 26]),
        ("eat_north", &[6, 5, 6]),
        ("eat_south", &[40, 39, 38, 46, 39]),
        ("sit_east", &[28]),
        ("sit_north", &[8]),
        ("sit_south", &[39]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [7, 8, 9, 10]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_balor_autumn";
            let asset = format!("assets/animations/NPCs/Balor/Sprites/Autumn/{prefix}_{case}.png");
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
                    "Balor Autumn material mismatch: {id} {case} [{x},{y}]"
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
