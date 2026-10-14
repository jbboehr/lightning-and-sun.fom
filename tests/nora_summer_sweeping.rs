use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/nora and the local accepted Nora world baseline"]
fn nora_summer_sweeping_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/nora");
    let baseline = root.join("local/fixtures/slice-071-build/characters/nora");
    let set = std::env::var_os("FOM_NORA_SUMMER_SWEEPING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/nora-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/nora-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..90];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..90], prior);
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
    // These pinned strips separate skin from hair, clothing, broom and dust.
    // The deepest shade is hair through row 40 and arm/hand contours from row 45.
    let landmarks = [
        ("sweep_end_south", 0, 43, 48, 0x000000, false),
        ("sweep_end_south", 0, 42, 44, 0x23325B, false),
        ("sweep_end_south", 0, 42, 43, 0x354981, false),
        ("sweep_end_south", 0, 40, 47, 0x3C1F2F, false),
        ("sweep_end_south", 0, 39, 44, 0x3F599E, false),
        ("sweep_end_south", 0, 40, 44, 0x516FBE, false),
        ("sweep_end_south", 0, 39, 48, 0x6A475B, false),
        ("sweep_end_south", 0, 42, 40, 0x762E21, false),
        ("sweep_end_south", 0, 46, 48, 0x762E21, true),
        ("sweep_end_south", 0, 40, 50, 0x807467, false),
        ("sweep_end_south", 0, 45, 33, 0xB65932, false),
        ("sweep_end_south", 0, 37, 51, 0xB8A68F, false),
        ("sweep_end_south", 0, 35, 32, 0xCC9155, false),
        ("sweep_end_south", 0, 33, 46, 0xD37A57, true),
        ("sweep_end_south", 0, 37, 50, 0xE8E2D1, false),
        ("sweep_end_south", 0, 44, 39, 0xECF0E9, false),
        ("sweep_end_south", 0, 43, 40, 0xEFCC86, false),
        ("sweep_end_south", 0, 44, 45, 0xF0B988, true),
        ("sweep_end_south", 0, 42, 33, 0xFAECB2, false),
        ("sweep_end_south", 0, 46, 47, 0xFCD9B3, true),
        ("sweep_loop_south", 0, 40, 49, 0x000000, false),
        ("sweep_loop_south", 0, 38, 43, 0x23325B, false),
        ("sweep_loop_south", 0, 37, 43, 0x354981, false),
        ("sweep_loop_south", 0, 42, 47, 0x3C1F2F, false),
        ("sweep_loop_south", 0, 37, 44, 0x3F599E, false),
        ("sweep_loop_south", 0, 41, 43, 0x516FBE, false),
        ("sweep_loop_south", 0, 42, 53, 0x653D25, false),
        ("sweep_loop_south", 0, 42, 48, 0x6A475B, false),
        ("sweep_loop_south", 0, 43, 40, 0x762E21, false),
        ("sweep_loop_south", 0, 39, 48, 0x762E21, true),
        ("sweep_loop_south", 0, 37, 51, 0x807467, false),
        ("sweep_loop_south", 0, 46, 53, 0x936244, false),
        ("sweep_loop_south", 0, 46, 33, 0xB65932, false),
        ("sweep_loop_south", 0, 38, 51, 0xB8A68F, false),
        ("sweep_loop_south", 0, 44, 51, 0xBB8151, false),
        ("sweep_loop_south", 0, 41, 31, 0xCC9155, false),
        ("sweep_loop_south", 0, 37, 47, 0xD37A57, true),
        ("sweep_loop_south", 0, 38, 50, 0xE8E2D1, false),
        ("sweep_loop_south", 0, 44, 55, 0xE9B03A, false),
        ("sweep_loop_south", 0, 43, 38, 0xECF0E9, false),
        ("sweep_loop_south", 0, 37, 40, 0xEFCC86, false),
        ("sweep_loop_south", 0, 37, 45, 0xF0B988, true),
        ("sweep_loop_south", 0, 43, 33, 0xFAECB2, false),
        ("sweep_loop_south", 0, 38, 46, 0xFCD9B3, true),
        ("sweep_loop_south", 0, 42, 57, 0xFFE36F, false),
        ("sweep_loop_south", 1, 39, 40, 0x762E21, false),
        ("sweep_loop_south", 1, 43, 45, 0xD37A57, true),
        ("sweep_loop_south", 1, 41, 45, 0xF0B988, true),
        ("sweep_loop_south", 1, 42, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 2, 44, 40, 0x762E21, false),
        ("sweep_loop_south", 2, 41, 44, 0xD37A57, true),
        ("sweep_loop_south", 2, 46, 44, 0xF0B988, true),
        ("sweep_loop_south", 2, 41, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 3, 42, 29, 0x762E21, false),
        ("sweep_loop_south", 3, 40, 44, 0xD37A57, true),
        ("sweep_loop_south", 3, 43, 45, 0xF0B988, true),
        ("sweep_loop_south", 3, 40, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 4, 42, 29, 0x762E21, false),
        ("sweep_loop_south", 4, 40, 44, 0xD37A57, true),
        ("sweep_loop_south", 4, 43, 45, 0xF0B988, true),
        ("sweep_loop_south", 4, 40, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 5, 42, 29, 0x762E21, false),
        ("sweep_loop_south", 5, 40, 44, 0xD37A57, true),
        ("sweep_loop_south", 5, 43, 45, 0xF0B988, true),
        ("sweep_loop_south", 5, 40, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 6, 42, 29, 0x762E21, false),
        ("sweep_loop_south", 6, 41, 44, 0xD37A57, true),
        ("sweep_loop_south", 6, 44, 45, 0xF0B988, true),
        ("sweep_loop_south", 6, 41, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 7, 39, 40, 0x762E21, false),
        ("sweep_loop_south", 7, 43, 45, 0xD37A57, true),
        ("sweep_loop_south", 7, 41, 45, 0xF0B988, true),
        ("sweep_loop_south", 7, 42, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 8, 44, 40, 0x762E21, false),
        ("sweep_loop_south", 8, 41, 44, 0xD37A57, true),
        ("sweep_loop_south", 8, 46, 44, 0xF0B988, true),
        ("sweep_loop_south", 8, 41, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 9, 42, 29, 0x762E21, false),
        ("sweep_loop_south", 9, 40, 44, 0xD37A57, true),
        ("sweep_loop_south", 9, 43, 45, 0xF0B988, true),
        ("sweep_loop_south", 9, 40, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 10, 42, 29, 0x762E21, false),
        ("sweep_loop_south", 10, 40, 44, 0xD37A57, true),
        ("sweep_loop_south", 10, 43, 45, 0xF0B988, true),
        ("sweep_loop_south", 10, 40, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 11, 42, 29, 0x762E21, false),
        ("sweep_loop_south", 11, 40, 44, 0xD37A57, true),
        ("sweep_loop_south", 11, 43, 45, 0xF0B988, true),
        ("sweep_loop_south", 11, 40, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 12, 42, 29, 0x762E21, false),
        ("sweep_loop_south", 12, 40, 47, 0x762E21, true),
        ("sweep_loop_south", 12, 37, 44, 0xD37A57, true),
        ("sweep_loop_south", 12, 38, 45, 0xF0B988, true),
        ("sweep_loop_south", 12, 39, 45, 0xFCD9B3, true),
        ("sweep_loop_south", 13, 43, 40, 0x762E21, false),
        ("sweep_loop_south", 13, 36, 47, 0x762E21, true),
        ("sweep_loop_south", 13, 36, 44, 0xD37A57, true),
        ("sweep_loop_south", 13, 45, 44, 0xF0B988, true),
        ("sweep_loop_south", 13, 35, 46, 0xFCD9B3, true),
        ("sweep_loop_south", 14, 40, 29, 0x762E21, false),
        ("sweep_loop_south", 14, 33, 47, 0x762E21, true),
        ("sweep_loop_south", 14, 33, 45, 0xD37A57, true),
        ("sweep_loop_south", 14, 47, 44, 0xF0B988, true),
        ("sweep_loop_south", 14, 32, 46, 0xFCD9B3, true),
        ("sweep_start_south", 0, 48, 48, 0x000000, false),
        ("sweep_start_south", 0, 37, 44, 0x23325B, false),
        ("sweep_start_south", 0, 36, 43, 0x354981, false),
        ("sweep_start_south", 0, 40, 47, 0x3C1F2F, false),
        ("sweep_start_south", 0, 44, 43, 0x3F599E, false),
        ("sweep_start_south", 0, 40, 43, 0x516FBE, false),
        ("sweep_start_south", 0, 39, 48, 0x6A475B, false),
        ("sweep_start_south", 0, 42, 40, 0x762E21, false),
        ("sweep_start_south", 0, 33, 48, 0x762E21, true),
        ("sweep_start_south", 0, 40, 50, 0x807467, false),
        ("sweep_start_south", 0, 41, 33, 0xB65932, false),
        ("sweep_start_south", 0, 37, 51, 0xB8A68F, false),
        ("sweep_start_south", 0, 35, 32, 0xCC9155, false),
        ("sweep_start_south", 0, 35, 46, 0xD37A57, true),
        ("sweep_start_south", 0, 37, 50, 0xE8E2D1, false),
        ("sweep_start_south", 0, 42, 38, 0xECF0E9, false),
        ("sweep_start_south", 0, 43, 40, 0xEFCC86, false),
        ("sweep_start_south", 0, 44, 44, 0xF0B988, true),
        ("sweep_start_south", 0, 42, 33, 0xFAECB2, false),
        ("sweep_start_south", 0, 45, 47, 0xFCD9B3, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("sweep_end_south", &[59]),
        (
            "sweep_loop_south",
            &[47, 53, 49, 52, 52, 52, 53, 53, 49, 52, 52, 52, 52, 45, 42],
        ),
        ("sweep_start_south", &[53]),
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
            let prefix = "spr_npc_nora_specialanimation_summer";
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
                // Moving hands and exposed ankles change; hair, broom and dust do not.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 45))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Nora Summer material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin separately from changed pixels.
                if source
                    .iter()
                    .any(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 45))
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
