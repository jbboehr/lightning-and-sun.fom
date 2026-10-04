use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/nora and the local accepted Nora world baseline"]
fn nora_summer_world_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/nora");
    let baseline = root.join("generated/slice-066-build/characters/nora");
    let set = std::env::var_os("FOM_NORA_SUMMER_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/nora-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/nora-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..64];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..64], prior);
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
    // Source grids separate exposed skin from hair, shirt, apron, trousers and shoes.
    // The deepest shade is hair through row 40 and skin contours from row 46.
    let landmarks = [
        ("idle_east", 0, 46, 47, 0x000000, false),
        ("idle_east", 0, 43, 43, 0x23325B, false),
        ("idle_east", 0, 37, 43, 0x354981, false),
        ("idle_east", 0, 41, 47, 0x3C1F2F, false),
        ("idle_east", 0, 42, 44, 0x3F599E, false),
        ("idle_east", 0, 41, 43, 0x516FBE, false),
        ("idle_east", 0, 40, 48, 0x6A475B, false),
        ("idle_east", 0, 34, 31, 0x762E21, false),
        ("idle_east", 0, 35, 47, 0x762E21, true),
        ("idle_east", 0, 38, 51, 0x807467, false),
        ("idle_east", 0, 43, 35, 0xA59DA2, false),
        ("idle_east", 0, 42, 32, 0xB65932, false),
        ("idle_east", 0, 39, 51, 0xB8A68F, false),
        ("idle_east", 0, 39, 35, 0xC2B9BE, false),
        ("idle_east", 0, 35, 31, 0xCC9155, false),
        ("idle_east", 0, 34, 45, 0xD37A57, true),
        ("idle_east", 0, 42, 50, 0xE8E2D1, false),
        ("idle_east", 0, 38, 37, 0xECF0E9, false),
        ("idle_east", 0, 40, 31, 0xEFCC86, false),
        ("idle_east", 0, 45, 46, 0xF0B988, true),
        ("idle_east", 0, 38, 32, 0xFAECB2, false),
        ("idle_east", 0, 35, 46, 0xFCD9B3, true),
        ("idle_north", 0, 34, 48, 0x000000, false),
        ("idle_north", 0, 44, 43, 0x23325B, false),
        ("idle_north", 0, 41, 42, 0x354981, false),
        ("idle_north", 0, 42, 46, 0x3C1F2F, false),
        ("idle_north", 0, 38, 45, 0x3F599E, false),
        ("idle_north", 0, 36, 42, 0x516FBE, false),
        ("idle_north", 0, 40, 46, 0x6A475B, false),
        ("idle_north", 0, 40, 34, 0x762E21, false),
        ("idle_north", 0, 46, 47, 0x762E21, true),
        ("idle_north", 0, 40, 50, 0x807467, false),
        ("idle_north", 0, 45, 32, 0xB65932, false),
        ("idle_north", 0, 41, 49, 0xB8A68F, false),
        ("idle_north", 0, 44, 38, 0xC2B9BE, false),
        ("idle_north", 0, 43, 34, 0xCC9155, false),
        ("idle_north", 0, 42, 47, 0xCCC5AF, false),
        ("idle_north", 0, 35, 45, 0xD37A57, true),
        ("idle_north", 0, 38, 48, 0xE8E2D1, false),
        ("idle_north", 0, 42, 32, 0xEFCC86, false),
        ("idle_north", 0, 37, 52, 0xF0B988, true),
        ("idle_north", 0, 40, 31, 0xFAECB2, false),
        ("idle_north", 0, 47, 46, 0xFCD9B3, true),
        ("idle_south", 0, 48, 47, 0x000000, false),
        ("idle_south", 0, 37, 44, 0x23325B, false),
        ("idle_south", 0, 44, 42, 0x354981, false),
        ("idle_south", 0, 40, 47, 0x3C1F2F, false),
        ("idle_south", 0, 44, 43, 0x3F599E, false),
        ("idle_south", 0, 40, 43, 0x516FBE, false),
        ("idle_south", 0, 39, 48, 0x6A475B, false),
        ("idle_south", 0, 40, 29, 0x762E21, false),
        ("idle_south", 0, 46, 47, 0x762E21, true),
        ("idle_south", 0, 40, 50, 0x807467, false),
        ("idle_south", 0, 40, 32, 0xB65932, false),
        ("idle_south", 0, 37, 51, 0xB8A68F, false),
        ("idle_south", 0, 41, 35, 0xC2B9BE, false),
        ("idle_south", 0, 35, 31, 0xCC9155, false),
        ("idle_south", 0, 35, 45, 0xD37A57, true),
        ("idle_south", 0, 37, 50, 0xE8E2D1, false),
        ("idle_south", 0, 42, 37, 0xECF0E9, false),
        ("idle_south", 0, 41, 31, 0xEFCC86, false),
        ("idle_south", 0, 38, 52, 0xF0B988, true),
        ("idle_south", 0, 42, 32, 0xFAECB2, false),
        ("idle_south", 0, 46, 46, 0xFCD9B3, true),
        ("walk_east", 0, 46, 47, 0x000000, false),
        ("walk_east", 0, 43, 43, 0x23325B, false),
        ("walk_east", 0, 37, 43, 0x354981, false),
        ("walk_east", 0, 41, 47, 0x3C1F2F, false),
        ("walk_east", 0, 42, 44, 0x3F599E, false),
        ("walk_east", 0, 41, 43, 0x516FBE, false),
        ("walk_east", 0, 40, 48, 0x6A475B, false),
        ("walk_east", 0, 34, 31, 0x762E21, false),
        ("walk_east", 0, 35, 47, 0x762E21, true),
        ("walk_east", 0, 38, 51, 0x807467, false),
        ("walk_east", 0, 43, 35, 0xA59DA2, false),
        ("walk_east", 0, 42, 32, 0xB65932, false),
        ("walk_east", 0, 39, 51, 0xB8A68F, false),
        ("walk_east", 0, 39, 35, 0xC2B9BE, false),
        ("walk_east", 0, 35, 31, 0xCC9155, false),
        ("walk_east", 0, 34, 45, 0xD37A57, true),
        ("walk_east", 0, 42, 50, 0xE8E2D1, false),
        ("walk_east", 0, 38, 37, 0xECF0E9, false),
        ("walk_east", 0, 40, 31, 0xEFCC86, false),
        ("walk_east", 0, 45, 46, 0xF0B988, true),
        ("walk_east", 0, 38, 32, 0xFAECB2, false),
        ("walk_east", 0, 35, 46, 0xFCD9B3, true),
        ("walk_east", 1, 43, 40, 0x762E21, false),
        ("walk_east", 1, 46, 47, 0x762E21, true),
        ("walk_east", 1, 44, 45, 0xD37A57, true),
        ("walk_east", 1, 34, 45, 0xF0B988, true),
        ("walk_east", 1, 45, 46, 0xFCD9B3, true),
        ("walk_east", 2, 34, 31, 0x762E21, false),
        ("walk_east", 2, 35, 47, 0x762E21, true),
        ("walk_east", 2, 34, 45, 0xD37A57, true),
        ("walk_east", 2, 45, 46, 0xF0B988, true),
        ("walk_east", 2, 35, 46, 0xFCD9B3, true),
        ("walk_east", 3, 43, 40, 0x762E21, false),
        ("walk_east", 3, 44, 51, 0x762E21, true),
        ("walk_east", 3, 35, 46, 0xD37A57, true),
        ("walk_east", 3, 36, 46, 0xF0B988, true),
        ("walk_east", 3, 37, 47, 0xFCD9B3, true),
        ("walk_north", 0, 34, 48, 0x000000, false),
        ("walk_north", 0, 44, 43, 0x23325B, false),
        ("walk_north", 0, 41, 42, 0x354981, false),
        ("walk_north", 0, 42, 46, 0x3C1F2F, false),
        ("walk_north", 0, 38, 45, 0x3F599E, false),
        ("walk_north", 0, 36, 42, 0x516FBE, false),
        ("walk_north", 0, 40, 46, 0x6A475B, false),
        ("walk_north", 0, 40, 34, 0x762E21, false),
        ("walk_north", 0, 46, 47, 0x762E21, true),
        ("walk_north", 0, 40, 50, 0x807467, false),
        ("walk_north", 0, 45, 32, 0xB65932, false),
        ("walk_north", 0, 41, 49, 0xB8A68F, false),
        ("walk_north", 0, 44, 38, 0xC2B9BE, false),
        ("walk_north", 0, 43, 34, 0xCC9155, false),
        ("walk_north", 0, 42, 47, 0xCCC5AF, false),
        ("walk_north", 0, 35, 45, 0xD37A57, true),
        ("walk_north", 0, 38, 48, 0xE8E2D1, false),
        ("walk_north", 0, 42, 32, 0xEFCC86, false),
        ("walk_north", 0, 37, 52, 0xF0B988, true),
        ("walk_north", 0, 40, 31, 0xFAECB2, false),
        ("walk_north", 0, 47, 46, 0xFCD9B3, true),
        ("walk_north", 1, 42, 40, 0x762E21, false),
        ("walk_north", 1, 46, 47, 0x762E21, true),
        ("walk_north", 1, 46, 46, 0xD37A57, true),
        ("walk_north", 1, 45, 45, 0xF0B988, true),
        ("walk_north", 1, 34, 47, 0xFCD9B3, true),
        ("walk_north", 2, 40, 34, 0x762E21, false),
        ("walk_north", 2, 46, 47, 0x762E21, true),
        ("walk_north", 2, 35, 45, 0xD37A57, true),
        ("walk_north", 2, 37, 52, 0xF0B988, true),
        ("walk_north", 2, 47, 46, 0xFCD9B3, true),
        ("walk_north", 3, 42, 40, 0x762E21, false),
        ("walk_north", 3, 33, 47, 0x762E21, true),
        ("walk_north", 3, 46, 46, 0xD37A57, true),
        ("walk_north", 3, 44, 45, 0xF0B988, true),
        ("walk_north", 3, 46, 47, 0xFCD9B3, true),
        ("walk_south", 0, 48, 47, 0x000000, false),
        ("walk_south", 0, 37, 44, 0x23325B, false),
        ("walk_south", 0, 44, 42, 0x354981, false),
        ("walk_south", 0, 40, 47, 0x3C1F2F, false),
        ("walk_south", 0, 44, 43, 0x3F599E, false),
        ("walk_south", 0, 40, 43, 0x516FBE, false),
        ("walk_south", 0, 39, 48, 0x6A475B, false),
        ("walk_south", 0, 40, 29, 0x762E21, false),
        ("walk_south", 0, 46, 47, 0x762E21, true),
        ("walk_south", 0, 40, 50, 0x807467, false),
        ("walk_south", 0, 40, 32, 0xB65932, false),
        ("walk_south", 0, 37, 51, 0xB8A68F, false),
        ("walk_south", 0, 41, 35, 0xC2B9BE, false),
        ("walk_south", 0, 35, 31, 0xCC9155, false),
        ("walk_south", 0, 35, 45, 0xD37A57, true),
        ("walk_south", 0, 37, 50, 0xE8E2D1, false),
        ("walk_south", 0, 42, 37, 0xECF0E9, false),
        ("walk_south", 0, 41, 31, 0xEFCC86, false),
        ("walk_south", 0, 38, 52, 0xF0B988, true),
        ("walk_south", 0, 42, 32, 0xFAECB2, false),
        ("walk_south", 0, 46, 46, 0xFCD9B3, true),
        ("walk_south", 1, 42, 40, 0x762E21, false),
        ("walk_south", 1, 34, 48, 0x762E21, true),
        ("walk_south", 1, 33, 46, 0xD37A57, true),
        ("walk_south", 1, 45, 45, 0xF0B988, true),
        ("walk_south", 1, 34, 47, 0xFCD9B3, true),
        ("walk_south", 2, 40, 29, 0x762E21, false),
        ("walk_south", 2, 46, 47, 0x762E21, true),
        ("walk_south", 2, 35, 45, 0xD37A57, true),
        ("walk_south", 2, 38, 52, 0xF0B988, true),
        ("walk_south", 2, 46, 46, 0xFCD9B3, true),
        ("walk_south", 3, 42, 40, 0x762E21, false),
        ("walk_south", 3, 45, 48, 0x762E21, true),
        ("walk_south", 3, 33, 46, 0xD37A57, true),
        ("walk_south", 3, 34, 45, 0xF0B988, true),
        ("walk_south", 3, 46, 47, 0xFCD9B3, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[42]),
        ("idle_north", &[28]),
        ("idle_south", &[50]),
        ("walk_east", &[42, 45, 42, 37]),
        ("walk_north", &[28, 22, 28, 22]),
        ("walk_south", &[50, 46, 50, 46]),
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
            let prefix = "spr_npc_nora_summer";
            let asset = format!("assets/animations/NPCs/Nora/Sprites/Summer/{prefix}_{case}.png");
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
                // Moving hands and exposed ankles change; hair, shirt, apron, trousers and shoes do not.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 46))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Nora Summer material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin separately from changed pixels.
                if source
                    .iter()
                    .any(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 46))
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
