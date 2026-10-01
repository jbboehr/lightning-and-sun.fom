use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/balor-winter-standard-study and the local accepted Balor world baseline"]
fn balor_winter_standard_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-winter-standard-study");
    let baseline = root.join(
        "generated/characters-balor-winter-actions-valen-finish-eiland-autumn-actions-trial/characters/balor",
    );
    let set = std::env::var_os("FOM_BALOR_WINTER_STANDARD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_BALOR_WINTER_STANDARD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/balor-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..218];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..218], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 223);
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
        ("action_east", 0, 37, 45, 0x1A1F31, false),
        ("action_east", 0, 39, 50, 0x262C49, false),
        ("action_east", 0, 38, 40, 0x281846, false),
        ("action_east", 0, 39, 51, 0x343F52, false),
        ("action_east", 0, 40, 34, 0x4A3D66, false),
        ("action_east", 0, 39, 49, 0x627390, false),
        ("action_east", 0, 42, 41, 0x672115, true),
        ("action_east", 0, 39, 33, 0x686589, false),
        ("action_east", 0, 44, 45, 0x877E6D, false),
        ("action_east", 0, 42, 45, 0x893F61, false),
        ("action_east", 0, 42, 31, 0x9B83B7, false),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 39, 36, 0xC2B9BE, false),
        ("action_east", 0, 43, 46, 0xC6BEAF, false),
        ("action_east", 0, 42, 40, 0xD37A57, true),
        ("action_east", 0, 44, 37, 0xECF0E9, false),
        ("action_east", 0, 44, 38, 0xF0B988, true),
        ("action_east", 0, 41, 46, 0xF8F0E2, false),
        ("action_east", 0, 41, 38, 0xFCD9B3, true),
        ("action_east", 1, 44, 40, 0x672115, true),
        ("action_south", 0, 45, 47, 0x000000, false),
        ("action_south", 0, 39, 44, 0x1A1F31, false),
        ("action_south", 0, 41, 51, 0x262C49, false),
        ("action_south", 0, 43, 40, 0x281846, false),
        ("action_south", 0, 36, 49, 0x343F52, false),
        ("action_south", 0, 40, 33, 0x4A3D66, false),
        ("action_south", 0, 44, 45, 0x5F534D, false),
        ("action_south", 0, 37, 49, 0x627390, false),
        ("action_south", 0, 40, 41, 0x672115, true),
        ("action_south", 0, 41, 33, 0x686589, false),
        ("action_south", 0, 37, 46, 0x877E6D, false),
        ("action_south", 0, 40, 45, 0x893F61, false),
        ("action_south", 0, 40, 31, 0x9B83B7, false),
        ("action_south", 0, 42, 36, 0xC2B9BE, false),
        ("action_south", 0, 41, 46, 0xC6BEAF, false),
        ("action_south", 0, 40, 40, 0xD37A57, true),
        ("action_south", 0, 42, 37, 0xECF0E9, false),
        ("action_south", 0, 38, 38, 0xF0B988, true),
        ("action_south", 0, 35, 46, 0xF8F0E2, false),
        ("action_south", 0, 40, 37, 0xFCD9B3, true),
        ("action_south", 1, 40, 40, 0x672115, true),
        ("kiss_east", 1, 42, 41, 0x672115, true),
        ("kiss_east", 1, 42, 40, 0xD37A57, true),
        ("sleep_east", 0, 36, 48, 0x000000, false),
        ("sleep_east", 0, 36, 45, 0x1A1F31, false),
        ("sleep_east", 0, 39, 50, 0x262C49, false),
        ("sleep_east", 0, 36, 37, 0x281846, false),
        ("sleep_east", 0, 39, 49, 0x343F52, false),
        ("sleep_east", 0, 42, 33, 0x4A3D66, false),
        ("sleep_east", 0, 43, 45, 0x5F534D, false),
        ("sleep_east", 0, 42, 47, 0x627390, false),
        ("sleep_east", 0, 38, 38, 0x672115, true),
        ("sleep_east", 0, 39, 32, 0x686589, false),
        ("sleep_east", 0, 42, 41, 0x877E6D, false),
        ("sleep_east", 0, 41, 44, 0x893F61, false),
        ("sleep_east", 0, 41, 30, 0x9B83B7, false),
        ("sleep_east", 0, 39, 45, 0xC6BEAF, false),
        ("sleep_east", 0, 38, 37, 0xD37A57, true),
        ("sleep_east", 0, 40, 35, 0xF0B988, true),
        ("sleep_east", 0, 44, 39, 0xF8F0E2, false),
        ("sleep_east", 0, 41, 36, 0xFCD9B3, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[23, 23, 23, 23, 23, 23, 23]),
        ("action_north", &[0, 0, 0, 0, 0, 0, 0]),
        ("action_south", &[27, 27, 27, 27, 27, 27, 27]),
        ("kiss_east", &[23, 25, 29, 29]),
        ("sleep_east", &[23]),
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
            let prefix = "spr_npc_balor_winter";
            let asset = format!("assets/animations/NPCs/Balor/Sprites/Winter/{prefix}_{case}.png");
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
                    "Balor Winter material mismatch: {id} {case} [{x},{y}]"
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
            if case.ends_with("_north") {
                assert!(per_frame.iter().all(|n| *n == 0));
                let region = candidate["regions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|r| r["asset"] == asset)
                    .unwrap();
                assert!(region["seeds"].as_array().unwrap().is_empty());
                assert_eq!(
                    fs::read(original.join(&asset)).unwrap(),
                    fs::read(variant.join(&asset)).unwrap(),
                    "covered North PNG must remain byte-identical"
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
