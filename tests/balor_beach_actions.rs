use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/balor and the local accepted Balor world baseline"]
fn balor_beach_actions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/balor");
    let baseline = root.join("generated/slice-003-build/characters/balor");
    let set = std::env::var_os("FOM_BALOR_BEACH_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_BALOR_BEACH_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/balor-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..235];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..235], prior);
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
    // Reviewed landmarks distinguish moving skin from protected materials.
    // Literal per-frame counts guard brief and occluded skin exposure.
    let landmarks = [
        ("action_east", 0, 40, 48, 0x000000, false),
        ("action_east", 0, 38, 40, 0x281846, false),
        ("action_east", 0, 43, 49, 0x2E2E64, false),
        ("action_east", 0, 38, 46, 0x374050, false),
        ("action_east", 0, 40, 49, 0x424E88, false),
        ("action_east", 0, 38, 49, 0x461839, false),
        ("action_east", 0, 40, 34, 0x4A3D66, false),
        ("action_east", 0, 39, 45, 0x525A62, false),
        ("action_east", 0, 42, 50, 0x6472B6, false),
        ("action_east", 0, 40, 47, 0x672115, true),
        ("action_east", 0, 44, 44, 0x686589, false),
        ("action_east", 0, 39, 49, 0x77405D, false),
        ("action_east", 0, 42, 31, 0x9B83B7, false),
        ("action_east", 0, 39, 50, 0xA56B81, false),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 40, 42, 0xB395D6, false),
        ("action_east", 0, 39, 36, 0xC2B9BE, false),
        ("action_east", 0, 38, 51, 0xD37A57, true),
        ("action_east", 0, 40, 41, 0xDEDAE9, false),
        ("action_east", 0, 44, 37, 0xECF0E9, false),
        ("action_east", 0, 43, 45, 0xF0B988, true),
        ("action_east", 0, 41, 46, 0xFCD9B3, true),
        ("action_east", 1, 44, 52, 0x672115, true),
        ("action_east", 1, 37, 51, 0xD37A57, true),
        ("action_east", 1, 44, 45, 0xF0B988, true),
        ("action_east", 1, 43, 45, 0xFCD9B3, true),
        ("action_north", 1, 35, 45, 0x672115, true),
        ("action_north", 1, 41, 53, 0xD37A57, true),
        ("action_north", 1, 37, 50, 0xF0B988, true),
        ("action_north", 1, 38, 49, 0xFCD9B3, true),
        ("action_south", 1, 35, 46, 0x672115, true),
        ("action_south", 1, 45, 44, 0xD37A57, true),
        ("action_south", 1, 41, 44, 0xF0B988, true),
        ("action_south", 1, 36, 46, 0xFCD9B3, true),
        ("blink_east", 1, 35, 46, 0x672115, true),
        ("blink_east", 1, 41, 50, 0xD37A57, true),
        ("blink_east", 1, 42, 45, 0xF0B988, true),
        ("blink_east", 1, 41, 45, 0xFCD9B3, true),
        ("blink_south", 0, 47, 47, 0x000000, false),
        ("blink_south", 0, 44, 38, 0x281846, false),
        ("blink_south", 0, 42, 47, 0x2E2E64, false),
        ("blink_south", 0, 46, 44, 0x374050, false),
        ("blink_south", 0, 41, 46, 0x424E88, false),
        ("blink_south", 0, 36, 46, 0x461839, false),
        ("blink_south", 0, 38, 33, 0x4A3D66, false),
        ("blink_south", 0, 45, 45, 0x525A62, false),
        ("blink_south", 0, 41, 48, 0x6472B6, false),
        ("blink_south", 0, 46, 46, 0x672115, true),
        ("blink_south", 0, 44, 42, 0x686589, false),
        ("blink_south", 0, 37, 46, 0x77405D, false),
        ("blink_south", 0, 40, 30, 0x9B83B7, false),
        ("blink_south", 0, 40, 45, 0xA56B81, false),
        ("blink_south", 0, 35, 43, 0xB395D6, false),
        ("blink_south", 0, 42, 36, 0xC2B9BE, false),
        ("blink_south", 0, 45, 43, 0xD37A57, true),
        ("blink_south", 0, 43, 41, 0xDEDAE9, false),
        ("blink_south", 0, 41, 44, 0xF0B988, true),
        ("blink_south", 0, 34, 46, 0xFCD9B3, true),
        ("blink_south", 1, 46, 46, 0x672115, true),
        ("blink_south", 1, 45, 43, 0xD37A57, true),
        ("blink_south", 1, 41, 44, 0xF0B988, true),
        ("blink_south", 1, 34, 46, 0xFCD9B3, true),
        ("kiss_east", 1, 36, 47, 0x672115, true),
        ("kiss_east", 1, 35, 47, 0xD37A57, true),
        ("kiss_east", 1, 43, 45, 0xF0B988, true),
        ("kiss_east", 1, 41, 46, 0xFCD9B3, true),
        ("sit_south", 0, 47, 47, 0x000000, false),
        ("sit_south", 0, 43, 40, 0x281846, false),
        ("sit_south", 0, 42, 46, 0x2E2E64, false),
        ("sit_south", 0, 44, 45, 0x374050, false),
        ("sit_south", 0, 40, 46, 0x424E88, false),
        ("sit_south", 0, 36, 47, 0x461839, false),
        ("sit_south", 0, 40, 33, 0x4A3D66, false),
        ("sit_south", 0, 45, 45, 0x525A62, false),
        ("sit_south", 0, 41, 47, 0x6472B6, false),
        ("sit_south", 0, 45, 47, 0x672115, true),
        ("sit_south", 0, 36, 44, 0x686589, false),
        ("sit_south", 0, 37, 46, 0x77405D, false),
        ("sit_south", 0, 40, 31, 0x9B83B7, false),
        ("sit_south", 0, 40, 45, 0xA56B81, false),
        ("sit_south", 0, 37, 43, 0xB395D6, false),
        ("sit_south", 0, 42, 36, 0xC2B9BE, false),
        ("sit_south", 0, 35, 44, 0xD37A57, true),
        ("sit_south", 0, 36, 42, 0xDEDAE9, false),
        ("sit_south", 0, 42, 37, 0xECF0E9, false),
        ("sit_south", 0, 34, 46, 0xF0B988, true),
        ("sit_south", 0, 42, 48, 0xFCD9B3, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[55, 57, 50, 57, 50, 55, 64]),
        ("action_north", &[33, 32, 34, 32, 34, 33, 36]),
        ("action_south", &[71, 74, 68, 74, 68, 71, 79]),
        ("blink_east", &[66, 70, 66]),
        ("blink_south", &[81, 85, 81]),
        ("kiss_east", &[58, 59, 66, 63]),
        ("sit_east", &[50]),
        ("sit_north", &[10]),
        ("sit_south", &[65]),
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
            let prefix = "spr_npc_balor_beach";
            let asset = format!("assets/animations/NPCs/Balor/Sprites/Beach/{prefix}_{case}.png");
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
                    "Balor Beach material mismatch: {id} {case} [{x},{y}]"
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
