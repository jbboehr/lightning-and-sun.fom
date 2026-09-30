use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/balor-autumn-actions-study and the local accepted Balor world baseline"]
fn balor_autumn_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-autumn-actions-study");
    let baseline = root.join(
        "generated/characters-balor-summer-finish-valen-eiland-actions-trial/characters/balor",
    );
    let set = std::env::var_os("FOM_BALOR_AUTUMN_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_BALOR_AUTUMN_WORLD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/balor-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..173];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..173], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 190);
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
        ("idle_east", 0, 34, 48, 0x000000, false),
        ("idle_east", 0, 39, 50, 0x262C49, false),
        ("idle_east", 0, 36, 40, 0x281846, false),
        ("idle_east", 0, 38, 43, 0x380F1C, false),
        ("idle_east", 0, 42, 48, 0x3A4863, false),
        ("idle_east", 0, 39, 33, 0x4A3D66, false),
        ("idle_east", 0, 38, 47, 0x51303E, false),
        ("idle_east", 0, 43, 41, 0x561635, false),
        ("idle_east", 0, 35, 46, 0x672115, true),
        ("idle_east", 0, 40, 42, 0x67243B, false),
        ("idle_east", 0, 42, 45, 0x686589, false),
        ("idle_east", 0, 41, 40, 0x6F5A54, false),
        ("idle_east", 0, 37, 47, 0x815065, false),
        ("idle_east", 0, 41, 30, 0x9B83B7, false),
        ("idle_east", 0, 40, 41, 0xA19085, false),
        ("idle_east", 0, 43, 35, 0xA59DA2, false),
        ("idle_east", 0, 38, 35, 0xC2B9BE, false),
        ("idle_east", 0, 34, 46, 0xD37A57, true),
        ("idle_east", 0, 41, 46, 0xECF0E9, false),
        ("idle_east", 0, 45, 45, 0xF0B988, true),
        ("idle_east", 0, 35, 45, 0xFCD9B3, true),
        ("idle_north", 0, 43, 48, 0x000000, false),
        ("idle_north", 0, 42, 50, 0x262C49, false),
        ("idle_north", 0, 37, 38, 0x281846, false),
        ("idle_north", 0, 43, 43, 0x380F1C, false),
        ("idle_north", 0, 42, 51, 0x3A4863, false),
        ("idle_north", 0, 44, 36, 0x4A3D66, false),
        ("idle_north", 0, 42, 44, 0x561635, false),
        ("idle_north", 0, 46, 46, 0x672115, true),
        ("idle_north", 0, 40, 45, 0x67243B, false),
        ("idle_north", 0, 41, 51, 0x686589, false),
        ("idle_north", 0, 41, 31, 0x9B83B7, false),
        ("idle_north", 0, 45, 43, 0xA19085, false),
        ("idle_north", 0, 46, 44, 0xD37A57, true),
        ("idle_north", 0, 46, 45, 0xFCD9B3, true),
        ("idle_south", 0, 43, 48, 0x000000, false),
        ("idle_south", 0, 38, 50, 0x262C49, false),
        ("idle_south", 0, 44, 38, 0x281846, false),
        ("idle_south", 0, 36, 43, 0x380F1C, false),
        ("idle_south", 0, 38, 48, 0x3A4863, false),
        ("idle_south", 0, 40, 32, 0x4A3D66, false),
        ("idle_south", 0, 42, 46, 0x51303E, false),
        ("idle_south", 0, 36, 42, 0x561635, false),
        ("idle_south", 0, 46, 46, 0x672115, true),
        ("idle_south", 0, 37, 42, 0x67243B, false),
        ("idle_south", 0, 41, 45, 0x686589, false),
        ("idle_south", 0, 42, 40, 0x6F5A54, false),
        ("idle_south", 0, 35, 47, 0x815065, false),
        ("idle_south", 0, 40, 30, 0x9B83B7, false),
        ("idle_south", 0, 40, 41, 0xA19085, false),
        ("idle_south", 0, 42, 35, 0xC2B9BE, false),
        ("idle_south", 0, 44, 44, 0xD37A57, true),
        ("idle_south", 0, 40, 46, 0xECF0E9, false),
        ("idle_south", 0, 38, 37, 0xF0B988, true),
        ("idle_south", 0, 46, 45, 0xFCD9B3, true),
        ("walk_east", 0, 34, 48, 0x000000, false),
        ("walk_east", 0, 39, 50, 0x262C49, false),
        ("walk_east", 0, 36, 40, 0x281846, false),
        ("walk_east", 0, 38, 43, 0x380F1C, false),
        ("walk_east", 0, 42, 48, 0x3A4863, false),
        ("walk_east", 0, 39, 33, 0x4A3D66, false),
        ("walk_east", 0, 38, 47, 0x51303E, false),
        ("walk_east", 0, 43, 41, 0x561635, false),
        ("walk_east", 0, 35, 46, 0x672115, true),
        ("walk_east", 0, 40, 42, 0x67243B, false),
        ("walk_east", 0, 42, 45, 0x686589, false),
        ("walk_east", 0, 41, 40, 0x6F5A54, false),
        ("walk_east", 0, 37, 47, 0x815065, false),
        ("walk_east", 0, 41, 30, 0x9B83B7, false),
        ("walk_east", 0, 40, 41, 0xA19085, false),
        ("walk_east", 0, 43, 35, 0xA59DA2, false),
        ("walk_east", 0, 38, 35, 0xC2B9BE, false),
        ("walk_east", 0, 34, 46, 0xD37A57, true),
        ("walk_east", 0, 41, 46, 0xECF0E9, false),
        ("walk_east", 0, 45, 45, 0xF0B988, true),
        ("walk_east", 0, 35, 45, 0xFCD9B3, true),
        ("walk_east", 1, 33, 46, 0x672115, true),
        ("walk_east", 1, 46, 44, 0xD37A57, true),
        ("walk_east", 1, 45, 46, 0xF0B988, true),
        ("walk_east", 1, 45, 45, 0xFCD9B3, true),
        ("walk_east", 2, 35, 46, 0x672115, true),
        ("walk_east", 2, 34, 46, 0xD37A57, true),
        ("walk_east", 2, 45, 45, 0xF0B988, true),
        ("walk_east", 2, 35, 45, 0xFCD9B3, true),
        ("walk_east", 3, 36, 47, 0x672115, true),
        ("walk_east", 3, 35, 45, 0xD37A57, true),
        ("walk_east", 3, 44, 45, 0xF0B988, true),
        ("walk_east", 3, 36, 46, 0xFCD9B3, true),
        ("walk_north", 0, 43, 48, 0x000000, false),
        ("walk_north", 0, 42, 50, 0x262C49, false),
        ("walk_north", 0, 37, 38, 0x281846, false),
        ("walk_north", 0, 43, 43, 0x380F1C, false),
        ("walk_north", 0, 42, 51, 0x3A4863, false),
        ("walk_north", 0, 44, 36, 0x4A3D66, false),
        ("walk_north", 0, 42, 44, 0x561635, false),
        ("walk_north", 0, 46, 46, 0x672115, true),
        ("walk_north", 0, 40, 45, 0x67243B, false),
        ("walk_north", 0, 41, 51, 0x686589, false),
        ("walk_north", 0, 41, 31, 0x9B83B7, false),
        ("walk_north", 0, 45, 43, 0xA19085, false),
        ("walk_north", 0, 46, 44, 0xD37A57, true),
        ("walk_north", 0, 46, 45, 0xFCD9B3, true),
        ("walk_north", 1, 34, 47, 0x672115, true),
        ("walk_north", 1, 46, 45, 0xD37A57, true),
        ("walk_north", 1, 33, 46, 0xFCD9B3, true),
        ("walk_north", 2, 46, 46, 0x672115, true),
        ("walk_north", 2, 46, 44, 0xD37A57, true),
        ("walk_north", 2, 46, 45, 0xFCD9B3, true),
        ("walk_north", 3, 45, 47, 0x672115, true),
        ("walk_north", 3, 46, 45, 0xD37A57, true),
        ("walk_north", 3, 45, 46, 0xFCD9B3, true),
        ("walk_south", 0, 43, 48, 0x000000, false),
        ("walk_south", 0, 38, 50, 0x262C49, false),
        ("walk_south", 0, 44, 38, 0x281846, false),
        ("walk_south", 0, 36, 43, 0x380F1C, false),
        ("walk_south", 0, 38, 48, 0x3A4863, false),
        ("walk_south", 0, 40, 32, 0x4A3D66, false),
        ("walk_south", 0, 42, 46, 0x51303E, false),
        ("walk_south", 0, 36, 42, 0x561635, false),
        ("walk_south", 0, 46, 46, 0x672115, true),
        ("walk_south", 0, 37, 42, 0x67243B, false),
        ("walk_south", 0, 41, 45, 0x686589, false),
        ("walk_south", 0, 42, 40, 0x6F5A54, false),
        ("walk_south", 0, 35, 47, 0x815065, false),
        ("walk_south", 0, 40, 30, 0x9B83B7, false),
        ("walk_south", 0, 40, 41, 0xA19085, false),
        ("walk_south", 0, 42, 35, 0xC2B9BE, false),
        ("walk_south", 0, 44, 44, 0xD37A57, true),
        ("walk_south", 0, 40, 46, 0xECF0E9, false),
        ("walk_south", 0, 38, 37, 0xF0B988, true),
        ("walk_south", 0, 46, 45, 0xFCD9B3, true),
        ("walk_south", 1, 46, 46, 0x672115, true),
        ("walk_south", 1, 45, 45, 0xD37A57, true),
        ("walk_south", 1, 38, 38, 0xF0B988, true),
        ("walk_south", 1, 34, 46, 0xFCD9B3, true),
        ("walk_south", 2, 46, 46, 0x672115, true),
        ("walk_south", 2, 44, 44, 0xD37A57, true),
        ("walk_south", 2, 38, 37, 0xF0B988, true),
        ("walk_south", 2, 46, 45, 0xFCD9B3, true),
        ("walk_south", 3, 33, 46, 0x672115, true),
        ("walk_south", 3, 34, 45, 0xD37A57, true),
        ("walk_south", 3, 38, 38, 0xF0B988, true),
        ("walk_south", 3, 45, 46, 0xFCD9B3, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[34]),
        ("idle_north", &[16]),
        ("idle_south", &[43]),
        ("walk_east", &[34, 37, 34, 32]),
        ("walk_north", &[16, 7, 16, 8]),
        ("walk_south", &[43, 39, 43, 39]),
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
