use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/nora and the local accepted Nora world baseline"]
fn nora_winter_world_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/nora");
    let baseline = root.join("local/fixtures/slice-078-build/characters/nora");
    let set = std::env::var_os("FOM_NORA_WINTER_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/nora-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/nora-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..122];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..122], prior);
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
    // These pinned strips separate exposed skin from hair, scarf, coat, skirt, boots and earrings.
    // Hair ends by row 40; deep hand contours start at row 47. Boots cover the ankles.
    let landmarks = [
        ("idle_east", 0, 36, 48, 0x000000, false),
        ("idle_east", 0, 42, 43, 0x1C2744, false),
        ("idle_east", 0, 41, 43, 0x293B6B, false),
        ("idle_east", 0, 39, 43, 0x3F599E, false),
        ("idle_east", 0, 41, 50, 0x441732, false),
        ("idle_east", 0, 43, 49, 0x4B1F15, false),
        ("idle_east", 0, 42, 50, 0x752D58, false),
        ("idle_east", 0, 41, 49, 0x75453A, false),
        ("idle_east", 0, 34, 31, 0x762E21, false),
        ("idle_east", 0, 35, 47, 0x762E21, true),
        ("idle_east", 0, 44, 45, 0x7D6956, false),
        ("idle_east", 0, 42, 48, 0x996953, false),
        ("idle_east", 0, 43, 35, 0xA59DA2, false),
        ("idle_east", 0, 42, 51, 0xB18262, false),
        ("idle_east", 0, 42, 32, 0xB65932, false),
        ("idle_east", 0, 36, 45, 0xB6AA8A, false),
        ("idle_east", 0, 42, 45, 0xC2B9BE, false),
        ("idle_east", 0, 35, 31, 0xCC9155, false),
        ("idle_east", 0, 41, 40, 0xD37A57, true),
        ("idle_east", 0, 36, 38, 0xE06C88, false),
        ("idle_east", 0, 38, 37, 0xECF0E9, false),
        ("idle_east", 0, 40, 31, 0xEFCC86, false),
        ("idle_east", 0, 45, 46, 0xF0B988, true),
        ("idle_east", 0, 38, 47, 0xF2E4BD, false),
        ("idle_east", 0, 38, 32, 0xFAECB2, false),
        ("idle_east", 0, 36, 46, 0xFCD9B3, true),
        ("idle_north", 0, 46, 48, 0x000000, false),
        ("idle_north", 0, 42, 41, 0x1C2744, false),
        ("idle_north", 0, 40, 41, 0x293B6B, false),
        ("idle_north", 0, 39, 42, 0x3F599E, false),
        ("idle_north", 0, 42, 50, 0x441732, false),
        ("idle_north", 0, 41, 50, 0x752D58, false),
        ("idle_north", 0, 40, 34, 0x762E21, false),
        ("idle_north", 0, 46, 47, 0x762E21, true),
        ("idle_north", 0, 36, 46, 0x7D6956, false),
        ("idle_north", 0, 42, 52, 0x996953, false),
        ("idle_north", 0, 41, 52, 0xB18262, false),
        ("idle_north", 0, 45, 32, 0xB65932, false),
        ("idle_north", 0, 33, 45, 0xB6AA8A, false),
        ("idle_north", 0, 44, 38, 0xCB4D6B, false),
        ("idle_north", 0, 43, 34, 0xCC9155, false),
        ("idle_north", 0, 40, 40, 0xD37A57, true),
        ("idle_north", 0, 42, 32, 0xEFCC86, false),
        ("idle_north", 0, 41, 46, 0xF2E4BD, false),
        ("idle_north", 0, 40, 31, 0xFAECB2, false),
        ("idle_north", 0, 47, 46, 0xFCD9B3, true),
        ("idle_south", 0, 34, 48, 0x000000, false),
        ("idle_south", 0, 39, 43, 0x1C2744, false),
        ("idle_south", 0, 40, 43, 0x293B6B, false),
        ("idle_south", 0, 38, 43, 0x3F599E, false),
        ("idle_south", 0, 42, 50, 0x441732, false),
        ("idle_south", 0, 42, 49, 0x4B1F15, false),
        ("idle_south", 0, 41, 50, 0x752D58, false),
        ("idle_south", 0, 39, 49, 0x75453A, false),
        ("idle_south", 0, 40, 29, 0x762E21, false),
        ("idle_south", 0, 46, 47, 0x762E21, true),
        ("idle_south", 0, 36, 46, 0x7D6956, false),
        ("idle_south", 0, 37, 51, 0x996953, false),
        ("idle_south", 0, 41, 51, 0xB18262, false),
        ("idle_south", 0, 40, 32, 0xB65932, false),
        ("idle_south", 0, 35, 45, 0xB6AA8A, false),
        ("idle_south", 0, 41, 45, 0xC2B9BE, false),
        ("idle_south", 0, 35, 31, 0xCC9155, false),
        ("idle_south", 0, 40, 40, 0xD37A57, true),
        ("idle_south", 0, 44, 38, 0xE06C88, false),
        ("idle_south", 0, 37, 37, 0xECF0E9, false),
        ("idle_south", 0, 41, 31, 0xEFCC86, false),
        ("idle_south", 0, 41, 38, 0xF0B988, true),
        ("idle_south", 0, 42, 46, 0xF2E4BD, false),
        ("idle_south", 0, 42, 32, 0xFAECB2, false),
        ("idle_south", 0, 47, 46, 0xFCD9B3, true),
        ("walk_east", 0, 36, 48, 0x000000, false),
        ("walk_east", 0, 42, 43, 0x1C2744, false),
        ("walk_east", 0, 41, 43, 0x293B6B, false),
        ("walk_east", 0, 39, 43, 0x3F599E, false),
        ("walk_east", 0, 41, 50, 0x441732, false),
        ("walk_east", 0, 43, 49, 0x4B1F15, false),
        ("walk_east", 0, 42, 50, 0x752D58, false),
        ("walk_east", 0, 41, 49, 0x75453A, false),
        ("walk_east", 0, 34, 31, 0x762E21, false),
        ("walk_east", 0, 35, 47, 0x762E21, true),
        ("walk_east", 0, 44, 45, 0x7D6956, false),
        ("walk_east", 0, 42, 48, 0x996953, false),
        ("walk_east", 0, 43, 35, 0xA59DA2, false),
        ("walk_east", 0, 42, 51, 0xB18262, false),
        ("walk_east", 0, 42, 32, 0xB65932, false),
        ("walk_east", 0, 36, 45, 0xB6AA8A, false),
        ("walk_east", 0, 42, 45, 0xC2B9BE, false),
        ("walk_east", 0, 35, 31, 0xCC9155, false),
        ("walk_east", 0, 41, 40, 0xD37A57, true),
        ("walk_east", 0, 36, 38, 0xE06C88, false),
        ("walk_east", 0, 38, 37, 0xECF0E9, false),
        ("walk_east", 0, 40, 31, 0xEFCC86, false),
        ("walk_east", 0, 45, 46, 0xF0B988, true),
        ("walk_east", 0, 38, 47, 0xF2E4BD, false),
        ("walk_east", 0, 38, 32, 0xFAECB2, false),
        ("walk_east", 0, 36, 46, 0xFCD9B3, true),
        ("walk_east", 1, 43, 40, 0x762E21, false),
        ("walk_east", 1, 46, 47, 0x762E21, true),
        ("walk_east", 1, 41, 41, 0xD37A57, true),
        ("walk_east", 1, 42, 40, 0xF0B988, true),
        ("walk_east", 1, 46, 46, 0xFCD9B3, true),
        ("walk_east", 2, 34, 31, 0x762E21, false),
        ("walk_east", 2, 35, 47, 0x762E21, true),
        ("walk_east", 2, 41, 40, 0xD37A57, true),
        ("walk_east", 2, 45, 46, 0xF0B988, true),
        ("walk_east", 2, 36, 46, 0xFCD9B3, true),
        ("walk_east", 3, 43, 40, 0x762E21, false),
        ("walk_east", 3, 36, 48, 0x762E21, true),
        ("walk_east", 3, 41, 41, 0xD37A57, true),
        ("walk_east", 3, 41, 40, 0xF0B988, true),
        ("walk_east", 3, 37, 47, 0xFCD9B3, true),
        ("walk_north", 0, 46, 48, 0x000000, false),
        ("walk_north", 0, 42, 41, 0x1C2744, false),
        ("walk_north", 0, 40, 41, 0x293B6B, false),
        ("walk_north", 0, 39, 42, 0x3F599E, false),
        ("walk_north", 0, 42, 50, 0x441732, false),
        ("walk_north", 0, 41, 50, 0x752D58, false),
        ("walk_north", 0, 40, 34, 0x762E21, false),
        ("walk_north", 0, 46, 47, 0x762E21, true),
        ("walk_north", 0, 36, 46, 0x7D6956, false),
        ("walk_north", 0, 42, 52, 0x996953, false),
        ("walk_north", 0, 41, 52, 0xB18262, false),
        ("walk_north", 0, 45, 32, 0xB65932, false),
        ("walk_north", 0, 33, 45, 0xB6AA8A, false),
        ("walk_north", 0, 44, 38, 0xCB4D6B, false),
        ("walk_north", 0, 43, 34, 0xCC9155, false),
        ("walk_north", 0, 40, 40, 0xD37A57, true),
        ("walk_north", 0, 42, 32, 0xEFCC86, false),
        ("walk_north", 0, 41, 46, 0xF2E4BD, false),
        ("walk_north", 0, 40, 31, 0xFAECB2, false),
        ("walk_north", 0, 47, 46, 0xFCD9B3, true),
        ("walk_north", 1, 42, 40, 0x762E21, false),
        ("walk_north", 1, 34, 48, 0x762E21, true),
        ("walk_north", 1, 46, 46, 0xD37A57, true),
        ("walk_north", 1, 35, 47, 0xFCD9B3, true),
        ("walk_north", 2, 40, 34, 0x762E21, false),
        ("walk_north", 2, 46, 47, 0x762E21, true),
        ("walk_north", 2, 40, 40, 0xD37A57, true),
        ("walk_north", 2, 47, 46, 0xFCD9B3, true),
        ("walk_north", 3, 42, 40, 0x762E21, false),
        ("walk_north", 3, 45, 48, 0x762E21, true),
        ("walk_north", 3, 34, 46, 0xD37A57, true),
        ("walk_north", 3, 46, 47, 0xFCD9B3, true),
        ("walk_south", 0, 34, 48, 0x000000, false),
        ("walk_south", 0, 39, 43, 0x1C2744, false),
        ("walk_south", 0, 40, 43, 0x293B6B, false),
        ("walk_south", 0, 38, 43, 0x3F599E, false),
        ("walk_south", 0, 42, 50, 0x441732, false),
        ("walk_south", 0, 42, 49, 0x4B1F15, false),
        ("walk_south", 0, 41, 50, 0x752D58, false),
        ("walk_south", 0, 39, 49, 0x75453A, false),
        ("walk_south", 0, 40, 29, 0x762E21, false),
        ("walk_south", 0, 46, 47, 0x762E21, true),
        ("walk_south", 0, 36, 46, 0x7D6956, false),
        ("walk_south", 0, 37, 51, 0x996953, false),
        ("walk_south", 0, 41, 51, 0xB18262, false),
        ("walk_south", 0, 40, 32, 0xB65932, false),
        ("walk_south", 0, 35, 45, 0xB6AA8A, false),
        ("walk_south", 0, 41, 45, 0xC2B9BE, false),
        ("walk_south", 0, 35, 31, 0xCC9155, false),
        ("walk_south", 0, 40, 40, 0xD37A57, true),
        ("walk_south", 0, 44, 38, 0xE06C88, false),
        ("walk_south", 0, 37, 37, 0xECF0E9, false),
        ("walk_south", 0, 41, 31, 0xEFCC86, false),
        ("walk_south", 0, 41, 38, 0xF0B988, true),
        ("walk_south", 0, 42, 46, 0xF2E4BD, false),
        ("walk_south", 0, 42, 32, 0xFAECB2, false),
        ("walk_south", 0, 47, 46, 0xFCD9B3, true),
        ("walk_south", 1, 42, 40, 0x762E21, false),
        ("walk_south", 1, 34, 48, 0x762E21, true),
        ("walk_south", 1, 45, 46, 0xD37A57, true),
        ("walk_south", 1, 40, 40, 0xF0B988, true),
        ("walk_south", 1, 35, 47, 0xFCD9B3, true),
        ("walk_south", 2, 40, 29, 0x762E21, false),
        ("walk_south", 2, 46, 47, 0x762E21, true),
        ("walk_south", 2, 40, 40, 0xD37A57, true),
        ("walk_south", 2, 41, 38, 0xF0B988, true),
        ("walk_south", 2, 47, 46, 0xFCD9B3, true),
        ("walk_south", 3, 42, 40, 0x762E21, false),
        ("walk_south", 3, 45, 48, 0x762E21, true),
        ("walk_south", 3, 33, 46, 0xD37A57, true),
        ("walk_south", 3, 40, 40, 0xF0B988, true),
        ("walk_south", 3, 46, 47, 0xFCD9B3, true),
        ("walk_east", 1, 36, 40, 0x762E21, false),
        ("walk_south", 1, 42, 40, 0x762E21, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[29]),
        ("idle_north", &[16]),
        ("idle_south", &[36]),
        ("walk_east", &[29, 33, 29, 28]),
        ("walk_north", &[16, 14, 16, 14]),
        ("walk_south", &[36, 34, 36, 34]),
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
                    .position(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 47))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Nora Winter material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin separately from changed pixels.
                if source
                    .iter()
                    .any(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 47))
                {
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
