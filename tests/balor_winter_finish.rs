use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/balor and the local accepted Balor world baseline"]
fn balor_winter_finish_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/balor");
    let baseline = root.join("generated/slice-001-build/characters/balor");
    let set = std::env::var_os("FOM_BALOR_WINTER_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_BALOR_WINTER_FINISH_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/balor-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..223];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..223], prior);
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
        ("inspect_gem_end_south", 0, 37, 45, 0x000000, false),
        ("inspect_gem_end_south", 0, 41, 44, 0x1A1F31, false),
        ("inspect_gem_end_south", 0, 41, 51, 0x262C49, false),
        ("inspect_gem_end_south", 0, 35, 40, 0x281846, false),
        ("inspect_gem_end_south", 0, 36, 49, 0x343F52, false),
        ("inspect_gem_end_south", 0, 38, 34, 0x4A3D66, false),
        ("inspect_gem_end_south", 0, 37, 49, 0x627390, false),
        ("inspect_gem_end_south", 0, 40, 41, 0x672115, true),
        ("inspect_gem_end_south", 0, 41, 33, 0x686589, false),
        ("inspect_gem_end_south", 0, 36, 45, 0x877E6D, false),
        ("inspect_gem_end_south", 0, 40, 45, 0x893F61, false),
        ("inspect_gem_end_south", 0, 40, 31, 0x9B83B7, false),
        ("inspect_gem_end_south", 0, 38, 46, 0xC6BEAF, false),
        ("inspect_gem_end_south", 0, 40, 40, 0xD37A57, true),
        ("inspect_gem_end_south", 0, 42, 36, 0xF0B988, true),
        ("inspect_gem_end_south", 0, 36, 44, 0xF8F0E2, false),
        ("inspect_gem_end_south", 0, 39, 37, 0xFCD9B3, true),
        ("inspect_gem_end_south", 1, 40, 40, 0x672115, true),
        ("inspect_gem_start_south", 1, 40, 40, 0x672115, true),
        ("read_sit_end_south", 0, 44, 46, 0x000000, false),
        ("read_sit_end_south", 0, 41, 50, 0x262C49, false),
        ("read_sit_end_south", 0, 42, 30, 0x281846, false),
        ("read_sit_end_south", 0, 38, 51, 0x343F52, false),
        ("read_sit_end_south", 0, 38, 35, 0x4A3D66, false),
        ("read_sit_end_south", 0, 37, 45, 0x606C76, false),
        ("read_sit_end_south", 0, 42, 40, 0x672115, true),
        ("read_sit_end_south", 0, 41, 34, 0x686589, false),
        ("read_sit_end_south", 0, 44, 47, 0x877E6D, false),
        ("read_sit_end_south", 0, 38, 46, 0x98A2A6, false),
        ("read_sit_end_south", 0, 40, 32, 0x9B83B7, false),
        ("read_sit_end_south", 0, 38, 44, 0xC9AF9C, false),
        ("read_sit_end_south", 0, 39, 41, 0xD37A57, true),
        ("read_sit_end_south", 0, 42, 38, 0xECF0E9, false),
        ("read_sit_end_south", 0, 40, 40, 0xF0B988, true),
        ("read_sit_end_south", 0, 38, 42, 0xF6E4D7, false),
        ("read_sit_end_south", 0, 34, 47, 0xF8F0E2, false),
        ("read_sit_end_south", 0, 40, 38, 0xFCD9B3, true),
        ("read_sit_loop_south", 1, 39, 42, 0x672115, true),
        ("read_sit_loop_south", 1, 39, 41, 0xD37A57, true),
        ("read_sit_loop_south", 1, 40, 40, 0xF0B988, true),
        ("read_sit_start_south", 0, 43, 47, 0x000000, false),
        ("read_sit_start_south", 0, 37, 44, 0x1A1F31, false),
        ("read_sit_start_south", 0, 42, 48, 0x262C49, false),
        ("read_sit_start_south", 0, 43, 40, 0x281846, false),
        ("read_sit_start_south", 0, 39, 47, 0x343F52, false),
        ("read_sit_start_south", 0, 40, 33, 0x4A3D66, false),
        ("read_sit_start_south", 0, 43, 45, 0x5F534D, false),
        ("read_sit_start_south", 0, 42, 43, 0x627390, false),
        ("read_sit_start_south", 0, 40, 41, 0x672115, true),
        ("read_sit_start_south", 0, 41, 33, 0x686589, false),
        ("read_sit_start_south", 0, 44, 45, 0x877E6D, false),
        ("read_sit_start_south", 0, 40, 31, 0x9B83B7, false),
        ("read_sit_start_south", 0, 42, 36, 0xC2B9BE, false),
        ("read_sit_start_south", 0, 38, 46, 0xC6BEAF, false),
        ("read_sit_start_south", 0, 40, 40, 0xD37A57, true),
        ("read_sit_start_south", 0, 42, 37, 0xECF0E9, false),
        ("read_sit_start_south", 0, 38, 38, 0xF0B988, true),
        ("read_sit_start_south", 0, 35, 46, 0xF8F0E2, false),
        ("read_sit_start_south", 0, 40, 37, 0xFCD9B3, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("inspect_gem_end_south", &[33, 33, 33, 33]),
        ("inspect_gem_loop_south", &[21, 21, 21, 21, 21, 21, 21]),
        ("inspect_gem_start_south", &[33, 33, 33, 33]),
        ("read_sit_end_south", &[25, 19, 27]),
        ("read_sit_loop_south", &[23, 33, 23, 33]),
        ("read_sit_start_south", &[27, 19, 25]),
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
            let prefix = "spr_npc_balor_specialanimation_winter";
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
