use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/nora and the local accepted Nora world baseline"]
fn nora_summer_writing_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/nora");
    let baseline = root.join("local/fixtures/slice-069-build/characters/nora");
    let set = std::env::var_os("FOM_NORA_SUMMER_WRITING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/nora-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/nora-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..84];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..84], prior);
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
    // These pinned strips separate exposed skin from hair and writing tools.
    // The deepest shade is hair except two lowered hand contours at row 47.
    let landmarks = [
        ("write_sit_end_east", 0, 40, 45, 0x000000, false),
        ("write_sit_end_east", 0, 35, 44, 0x010101, false),
        ("write_sit_end_east", 0, 38, 43, 0x23325B, false),
        ("write_sit_end_east", 0, 38, 42, 0x354981, false),
        ("write_sit_end_east", 0, 41, 46, 0x373357, false),
        ("write_sit_end_east", 0, 44, 48, 0x3F599E, false),
        ("write_sit_end_east", 0, 40, 44, 0x6B5C7C, false),
        ("write_sit_end_east", 0, 34, 31, 0x762E21, false),
        ("write_sit_end_east", 0, 43, 36, 0xA59DA2, false),
        ("write_sit_end_east", 0, 47, 44, 0xB28159, false),
        ("write_sit_end_east", 0, 39, 33, 0xB65932, false),
        ("write_sit_end_east", 0, 43, 45, 0xB8A68F, false),
        ("write_sit_end_east", 0, 40, 41, 0xBA7F3E, false),
        ("write_sit_end_east", 0, 38, 36, 0xC2B9BE, false),
        ("write_sit_end_east", 0, 45, 45, 0xC3D1DD, false),
        ("write_sit_end_east", 0, 41, 31, 0xCC9155, false),
        ("write_sit_end_east", 0, 36, 45, 0xD37A57, true),
        ("write_sit_end_east", 0, 43, 37, 0xECF0E9, false),
        ("write_sit_end_east", 0, 40, 31, 0xEFCC86, false),
        ("write_sit_end_east", 0, 39, 46, 0xF0B988, true),
        ("write_sit_end_east", 0, 46, 44, 0xF5F5F5, false),
        ("write_sit_end_east", 0, 38, 32, 0xFAECB2, false),
        ("write_sit_end_east", 0, 38, 45, 0xFCD9B3, true),
        ("write_sit_end_east", 0, 39, 41, 0xFFDA6A, false),
        ("write_sit_end_east", 1, 43, 40, 0x762E21, false),
        ("write_sit_end_east", 1, 35, 47, 0x762E21, true),
        ("write_sit_end_east", 1, 41, 41, 0xD37A57, true),
        ("write_sit_end_east", 1, 35, 45, 0xF0B988, true),
        ("write_sit_end_east", 1, 35, 46, 0xFCD9B3, true),
        ("write_sit_loop_east", 0, 36, 45, 0x000000, false),
        ("write_sit_loop_east", 0, 39, 42, 0x23325B, false),
        ("write_sit_loop_east", 0, 37, 43, 0x354981, false),
        ("write_sit_loop_east", 0, 40, 41, 0x373357, false),
        ("write_sit_loop_east", 0, 45, 48, 0x3F599E, false),
        ("write_sit_loop_east", 0, 34, 31, 0x762E21, false),
        ("write_sit_loop_east", 0, 40, 47, 0x807467, false),
        ("write_sit_loop_east", 0, 43, 36, 0xA59DA2, false),
        ("write_sit_loop_east", 0, 45, 44, 0xB28159, false),
        ("write_sit_loop_east", 0, 39, 33, 0xB65932, false),
        ("write_sit_loop_east", 0, 42, 47, 0xB8A68F, false),
        ("write_sit_loop_east", 0, 39, 40, 0xBA7F3E, false),
        ("write_sit_loop_east", 0, 38, 36, 0xC2B9BE, false),
        ("write_sit_loop_east", 0, 42, 45, 0xC3D1DD, false),
        ("write_sit_loop_east", 0, 41, 31, 0xCC9155, false),
        ("write_sit_loop_east", 0, 37, 45, 0xD37A57, true),
        ("write_sit_loop_east", 0, 43, 37, 0xECF0E9, false),
        ("write_sit_loop_east", 0, 39, 31, 0xEFCC86, false),
        ("write_sit_loop_east", 0, 38, 45, 0xF0B988, true),
        ("write_sit_loop_east", 0, 43, 44, 0xF5F5F5, false),
        ("write_sit_loop_east", 0, 38, 32, 0xFAECB2, false),
        ("write_sit_loop_east", 0, 38, 44, 0xFCD9B3, true),
        ("write_sit_loop_east", 0, 38, 40, 0xFFDA6A, false),
        ("write_sit_loop_east", 1, 34, 31, 0x762E21, false),
        ("write_sit_loop_east", 1, 37, 45, 0xD37A57, true),
        ("write_sit_loop_east", 1, 40, 45, 0xF0B988, true),
        ("write_sit_loop_east", 1, 40, 44, 0xFCD9B3, true),
        ("write_sit_loop_east", 2, 34, 31, 0x762E21, false),
        ("write_sit_loop_east", 2, 36, 45, 0xD37A57, true),
        ("write_sit_loop_east", 2, 39, 44, 0xF0B988, true),
        ("write_sit_loop_east", 2, 37, 44, 0xFCD9B3, true),
        ("write_sit_loop_east", 3, 34, 31, 0x762E21, false),
        ("write_sit_loop_east", 3, 36, 45, 0xD37A57, true),
        ("write_sit_loop_east", 3, 39, 45, 0xF0B988, true),
        ("write_sit_loop_east", 3, 39, 44, 0xFCD9B3, true),
        ("write_sit_start_east", 0, 45, 45, 0x000000, false),
        ("write_sit_start_east", 0, 38, 44, 0x23325B, false),
        ("write_sit_start_east", 0, 39, 43, 0x354981, false),
        ("write_sit_start_east", 0, 38, 46, 0x3C1F2F, false),
        ("write_sit_start_east", 0, 42, 44, 0x3F599E, false),
        ("write_sit_start_east", 0, 41, 43, 0x516FBE, false),
        ("write_sit_start_east", 0, 40, 45, 0x6A475B, false),
        ("write_sit_start_east", 0, 43, 40, 0x762E21, false),
        ("write_sit_start_east", 0, 35, 47, 0x762E21, true),
        ("write_sit_start_east", 0, 39, 47, 0x807467, false),
        ("write_sit_start_east", 0, 45, 43, 0xB28159, false),
        ("write_sit_start_east", 0, 43, 34, 0xB65932, false),
        ("write_sit_start_east", 0, 44, 46, 0xB8A68F, false),
        ("write_sit_start_east", 0, 44, 44, 0xC3D1DD, false),
        ("write_sit_start_east", 0, 44, 40, 0xCC9155, false),
        ("write_sit_start_east", 0, 41, 41, 0xD37A57, true),
        ("write_sit_start_east", 0, 36, 39, 0xECF0E9, false),
        ("write_sit_start_east", 0, 37, 40, 0xEFCC86, false),
        ("write_sit_start_east", 0, 35, 45, 0xF0B988, true),
        ("write_sit_start_east", 0, 44, 43, 0xF5F5F5, false),
        ("write_sit_start_east", 0, 38, 33, 0xFAECB2, false),
        ("write_sit_start_east", 0, 35, 46, 0xFCD9B3, true),
        ("write_sit_start_east", 1, 34, 31, 0x762E21, false),
        ("write_sit_start_east", 1, 36, 45, 0xD37A57, true),
        ("write_sit_start_east", 1, 39, 46, 0xF0B988, true),
        ("write_sit_start_east", 1, 38, 45, 0xFCD9B3, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("write_sit_end_east", &[31, 41]),
        ("write_sit_loop_east", &[34, 35, 36, 36]),
        ("write_sit_start_east", &[41, 31]),
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
                // Moving hands and exposed ankles change; hair and writing tools do not.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y == 47))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Nora Summer material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin separately from changed pixels.
                if source
                    .iter()
                    .any(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y == 47))
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
