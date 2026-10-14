use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/nora and the local accepted Nora world baseline"]
fn nora_winter_general_actions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/nora");
    let baseline = root.join("local/fixtures/slice-080-build/characters/nora");
    let set = std::env::var_os("FOM_NORA_WINTER_GENERAL_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/nora-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/nora-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..139];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..139], prior);
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
    let source = [0xFCD9B3, 0xF0B988, 0xD37A57, 0x762E21];
    // Source grids separate exposed skin from hair, scarf, coat, skirt, boots and earrings.
    // Deep contours from row 42 are skin. At row 41 only North-facing hands change;
    // East/South hair tips retain their original color throughout the motion.
    let landmarks = [
        ("action_east", 0, 46, 48, 0x000000, false),
        ("action_east", 0, 44, 43, 0x1C2744, false),
        ("action_east", 0, 43, 43, 0x293B6B, false),
        ("action_east", 0, 42, 43, 0x3F599E, false),
        ("action_east", 0, 42, 51, 0x441732, false),
        ("action_east", 0, 44, 50, 0x4B1F15, false),
        ("action_east", 0, 43, 51, 0x752D58, false),
        ("action_east", 0, 43, 50, 0x75453A, false),
        ("action_east", 0, 44, 40, 0x762E21, false),
        ("action_east", 0, 40, 48, 0x762E21, true),
        ("action_east", 0, 37, 48, 0x7D6956, false),
        ("action_east", 0, 39, 52, 0x996953, false),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 40, 53, 0xB18262, false),
        ("action_east", 0, 43, 33, 0xB65932, false),
        ("action_east", 0, 40, 46, 0xB6AA8A, false),
        ("action_east", 0, 43, 46, 0xC2B9BE, false),
        ("action_east", 0, 45, 40, 0xCC9155, false),
        ("action_east", 0, 42, 41, 0xD37A57, true),
        ("action_east", 0, 37, 39, 0xE06C88, false),
        ("action_east", 0, 39, 38, 0xECF0E9, false),
        ("action_east", 0, 38, 40, 0xEFCC86, false),
        ("action_east", 0, 42, 40, 0xF0B988, true),
        ("action_east", 0, 44, 47, 0xF2E4BD, false),
        ("action_east", 0, 39, 33, 0xFAECB2, false),
        ("action_east", 0, 41, 47, 0xFCD9B3, true),
        ("action_east", 1, 37, 31, 0x762E21, false),
        ("action_east", 1, 48, 45, 0x762E21, true),
        ("action_east", 1, 44, 40, 0xD37A57, true),
        ("action_east", 1, 47, 45, 0xF0B988, true),
        ("action_east", 1, 48, 44, 0xFCD9B3, true),
        ("action_east", 2, 37, 31, 0x762E21, false),
        ("action_east", 2, 45, 46, 0x762E21, true),
        ("action_east", 2, 44, 40, 0xD37A57, true),
        ("action_east", 2, 46, 46, 0xF0B988, true),
        ("action_east", 2, 46, 45, 0xFCD9B3, true),
        ("action_east", 3, 37, 31, 0x762E21, false),
        ("action_east", 3, 48, 45, 0x762E21, true),
        ("action_east", 3, 44, 40, 0xD37A57, true),
        ("action_east", 3, 47, 45, 0xF0B988, true),
        ("action_east", 3, 48, 44, 0xFCD9B3, true),
        ("action_east", 4, 37, 31, 0x762E21, false),
        ("action_east", 4, 45, 46, 0x762E21, true),
        ("action_east", 4, 44, 40, 0xD37A57, true),
        ("action_east", 4, 46, 46, 0xF0B988, true),
        ("action_east", 4, 46, 45, 0xFCD9B3, true),
        ("action_east", 5, 44, 40, 0x762E21, false),
        ("action_east", 5, 40, 48, 0x762E21, true),
        ("action_east", 5, 42, 41, 0xD37A57, true),
        ("action_east", 5, 42, 40, 0xF0B988, true),
        ("action_east", 5, 41, 47, 0xFCD9B3, true),
        ("action_east", 6, 34, 31, 0x762E21, false),
        ("action_east", 6, 35, 47, 0x762E21, true),
        ("action_east", 6, 41, 40, 0xD37A57, true),
        ("action_east", 6, 45, 46, 0xF0B988, true),
        ("action_east", 6, 36, 46, 0xFCD9B3, true),
        ("action_north", 0, 35, 46, 0x000000, false),
        ("action_north", 0, 42, 41, 0x1C2744, false),
        ("action_north", 0, 40, 41, 0x293B6B, false),
        ("action_north", 0, 39, 42, 0x3F599E, false),
        ("action_north", 0, 42, 50, 0x441732, false),
        ("action_north", 0, 41, 50, 0x752D58, false),
        ("action_north", 0, 40, 34, 0x762E21, false),
        ("action_north", 0, 33, 45, 0x762E21, true),
        ("action_north", 0, 44, 44, 0x7D6956, false),
        ("action_north", 0, 42, 52, 0x996953, false),
        ("action_north", 0, 41, 52, 0xB18262, false),
        ("action_north", 0, 45, 32, 0xB65932, false),
        ("action_north", 0, 37, 44, 0xB6AA8A, false),
        ("action_north", 0, 44, 38, 0xCB4D6B, false),
        ("action_north", 0, 43, 34, 0xCC9155, false),
        ("action_north", 0, 40, 40, 0xD37A57, true),
        ("action_north", 0, 42, 32, 0xEFCC86, false),
        ("action_north", 0, 41, 46, 0xF2E4BD, false),
        ("action_north", 0, 40, 31, 0xFAECB2, false),
        ("action_north", 0, 34, 44, 0xFCD9B3, true),
        ("action_north", 1, 40, 33, 0x762E21, false),
        ("action_north", 1, 35, 46, 0x762E21, true),
        ("action_north", 1, 40, 39, 0xD37A57, true),
        ("action_north", 1, 36, 45, 0xFCD9B3, true),
        ("action_north", 2, 40, 33, 0x762E21, false),
        ("action_north", 2, 35, 46, 0x762E21, true),
        ("action_north", 2, 40, 39, 0xD37A57, true),
        ("action_north", 2, 36, 45, 0xFCD9B3, true),
        ("action_north", 3, 40, 33, 0x762E21, false),
        ("action_north", 3, 35, 46, 0x762E21, true),
        ("action_north", 3, 40, 39, 0xD37A57, true),
        ("action_north", 3, 36, 45, 0xFCD9B3, true),
        ("action_north", 4, 40, 33, 0x762E21, false),
        ("action_north", 4, 35, 46, 0x762E21, true),
        ("action_north", 4, 40, 39, 0xD37A57, true),
        ("action_north", 4, 36, 45, 0xFCD9B3, true),
        ("action_north", 5, 40, 34, 0x762E21, false),
        ("action_north", 5, 34, 47, 0x762E21, true),
        ("action_north", 5, 46, 41, 0xD37A57, true),
        ("action_north", 5, 35, 46, 0xFCD9B3, true),
        ("action_north", 6, 40, 34, 0x762E21, false),
        ("action_north", 6, 46, 47, 0x762E21, true),
        ("action_north", 6, 40, 40, 0xD37A57, true),
        ("action_north", 6, 47, 46, 0xFCD9B3, true),
        ("action_south", 0, 45, 48, 0x000000, false),
        ("action_south", 0, 39, 44, 0x1C2744, false),
        ("action_south", 0, 40, 44, 0x293B6B, false),
        ("action_south", 0, 38, 44, 0x3F599E, false),
        ("action_south", 0, 42, 51, 0x441732, false),
        ("action_south", 0, 42, 50, 0x4B1F15, false),
        ("action_south", 0, 41, 51, 0x752D58, false),
        ("action_south", 0, 39, 50, 0x75453A, false),
        ("action_south", 0, 42, 40, 0x762E21, false),
        ("action_south", 0, 34, 48, 0x762E21, true),
        ("action_south", 0, 45, 45, 0x7D6956, false),
        ("action_south", 0, 41, 49, 0x996953, false),
        ("action_south", 0, 37, 53, 0xB18262, false),
        ("action_south", 0, 40, 33, 0xB65932, false),
        ("action_south", 0, 33, 46, 0xB6AA8A, false),
        ("action_south", 0, 41, 46, 0xC2B9BE, false),
        ("action_south", 0, 35, 32, 0xCC9155, false),
        ("action_south", 0, 45, 46, 0xD37A57, true),
        ("action_south", 0, 44, 39, 0xE06C88, false),
        ("action_south", 0, 37, 38, 0xECF0E9, false),
        ("action_south", 0, 43, 40, 0xEFCC86, false),
        ("action_south", 0, 40, 40, 0xF0B988, true),
        ("action_south", 0, 37, 48, 0xF2E4BD, false),
        ("action_south", 0, 42, 33, 0xFAECB2, false),
        ("action_south", 0, 35, 47, 0xFCD9B3, true),
        ("action_south", 1, 40, 29, 0x762E21, false),
        ("action_south", 1, 35, 47, 0x762E21, true),
        ("action_south", 1, 44, 45, 0xD37A57, true),
        ("action_south", 1, 41, 38, 0xF0B988, true),
        ("action_south", 1, 36, 46, 0xFCD9B3, true),
        ("action_south", 2, 40, 29, 0x762E21, false),
        ("action_south", 2, 38, 47, 0x762E21, true),
        ("action_south", 2, 45, 45, 0xD37A57, true),
        ("action_south", 2, 41, 38, 0xF0B988, true),
        ("action_south", 2, 39, 46, 0xFCD9B3, true),
        ("action_south", 3, 40, 29, 0x762E21, false),
        ("action_south", 3, 35, 47, 0x762E21, true),
        ("action_south", 3, 44, 45, 0xD37A57, true),
        ("action_south", 3, 41, 38, 0xF0B988, true),
        ("action_south", 3, 36, 46, 0xFCD9B3, true),
        ("action_south", 4, 40, 29, 0x762E21, false),
        ("action_south", 4, 38, 47, 0x762E21, true),
        ("action_south", 4, 45, 45, 0xD37A57, true),
        ("action_south", 4, 41, 38, 0xF0B988, true),
        ("action_south", 4, 39, 46, 0xFCD9B3, true),
        ("action_south", 5, 42, 40, 0x762E21, false),
        ("action_south", 5, 34, 48, 0x762E21, true),
        ("action_south", 5, 45, 46, 0xD37A57, true),
        ("action_south", 5, 40, 40, 0xF0B988, true),
        ("action_south", 5, 35, 47, 0xFCD9B3, true),
        ("action_south", 6, 40, 29, 0x762E21, false),
        ("action_south", 6, 46, 47, 0x762E21, true),
        ("action_south", 6, 40, 40, 0xD37A57, true),
        ("action_south", 6, 41, 38, 0xF0B988, true),
        ("action_south", 6, 47, 46, 0xFCD9B3, true),
        ("action_east", 0, 45, 39, 0x762E21, false),
        ("action_east", 0, 37, 40, 0x762E21, false),
        ("action_east", 1, 39, 39, 0x762E21, false),
        ("action_east", 1, 46, 39, 0x762E21, false),
        ("action_east", 2, 39, 39, 0x762E21, false),
        ("action_east", 2, 46, 39, 0x762E21, false),
        ("action_east", 3, 39, 39, 0x762E21, false),
        ("action_east", 3, 46, 39, 0x762E21, false),
        ("action_east", 4, 39, 39, 0x762E21, false),
        ("action_east", 4, 46, 39, 0x762E21, false),
        ("action_east", 5, 45, 39, 0x762E21, false),
        ("action_east", 5, 37, 40, 0x762E21, false),
        ("action_east", 6, 36, 39, 0x762E21, false),
        ("action_east", 6, 43, 39, 0x762E21, false),
        ("action_north", 0, 37, 39, 0x762E21, false),
        ("action_north", 0, 42, 39, 0x762E21, false),
        ("action_north", 0, 45, 41, 0x762E21, true),
        ("action_north", 5, 37, 39, 0x762E21, false),
        ("action_north", 5, 42, 39, 0x762E21, false),
        ("action_north", 5, 45, 41, 0x762E21, true),
        ("action_north", 6, 37, 39, 0x762E21, false),
        ("action_north", 6, 42, 39, 0x762E21, false),
        ("action_south", 0, 37, 40, 0x762E21, false),
        ("action_south", 1, 37, 39, 0x762E21, false),
        ("action_south", 1, 42, 39, 0x762E21, false),
        ("action_south", 2, 37, 39, 0x762E21, false),
        ("action_south", 2, 42, 39, 0x762E21, false),
        ("action_south", 3, 37, 39, 0x762E21, false),
        ("action_south", 3, 42, 39, 0x762E21, false),
        ("action_south", 4, 37, 39, 0x762E21, false),
        ("action_south", 4, 42, 39, 0x762E21, false),
        ("action_south", 5, 37, 40, 0x762E21, false),
        ("action_south", 6, 37, 39, 0x762E21, false),
        ("action_south", 6, 42, 39, 0x762E21, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[28, 30, 28, 30, 28, 28, 29]),
        ("action_north", &[12, 10, 10, 10, 10, 12, 16]),
        ("action_south", &[34, 34, 34, 34, 34, 34, 36]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [10, 11, 12, 3]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_nora_winter";
            let asset = format!("assets/animations/NPCs/Nora/Sprites/Winter/{prefix}_{case}.png");
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
                // Face, neck and hands change; hair, clothing, boots and earrings do not.
                let expected = source
                    .iter()
                    .position(|c| {
                        rgba(*c) == p.0
                            && (*c != 0x762E21 || (y >= 42 || (case == "action_north" && y == 41)))
                    })
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Nora Winter material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin separately from changed pixels.
                if source.iter().any(|c| {
                    rgba(*c) == p.0
                        && (*c != 0x762E21 || (y >= 42 || (case == "action_north" && y == 41)))
                }) {
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
