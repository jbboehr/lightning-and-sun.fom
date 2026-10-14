use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/nora and the local accepted Nora world baseline"]
fn nora_autumn_tapping_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/nora");
    let baseline = root.join("local/fixtures/slice-076-build/characters/nora");
    let set = std::env::var_os("FOM_NORA_AUTUMN_TAPPING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/nora-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/nora-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..116];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..116], prior);
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
    let source = [0xFCD9B3, 0xF0B988, 0xD37A57];
    // These pinned tapping strips use three distinct shades for exposed skin.
    // The shared deep shade is entirely hair here; the board and pen stay original.
    let landmarks = [
        ("tap_end_south", 0, 42, 47, 0x000000, false),
        ("tap_end_south", 0, 39, 43, 0x1A2748, false),
        ("tap_end_south", 0, 40, 42, 0x30447A, false),
        ("tap_end_south", 0, 37, 46, 0x343973, false),
        ("tap_end_south", 0, 33, 46, 0x373357, false),
        ("tap_end_south", 0, 42, 51, 0x3C1F2F, false),
        ("tap_end_south", 0, 37, 45, 0x5263B7, false),
        ("tap_end_south", 0, 46, 47, 0x57342B, false),
        ("tap_end_south", 0, 42, 50, 0x5B3249, false),
        ("tap_end_south", 0, 41, 51, 0x6A475B, false),
        ("tap_end_south", 0, 33, 45, 0x6B5C7C, false),
        ("tap_end_south", 0, 41, 44, 0x7089D8, false),
        ("tap_end_south", 0, 44, 39, 0x7482B0, false),
        ("tap_end_south", 0, 38, 41, 0x762E21, false),
        ("tap_end_south", 0, 37, 47, 0x7B6A55, false),
        ("tap_end_south", 0, 49, 46, 0x7D4F3D, false),
        ("tap_end_south", 0, 47, 45, 0x837CA0, false),
        ("tap_end_south", 0, 48, 45, 0xA8A5B5, false),
        ("tap_end_south", 0, 39, 48, 0xB09E87, false),
        ("tap_end_south", 0, 45, 33, 0xB65932, false),
        ("tap_end_south", 0, 42, 42, 0xB8BABB, false),
        ("tap_end_south", 0, 32, 42, 0xBA7F3E, false),
        ("tap_end_south", 0, 44, 46, 0xC5895C, false),
        ("tap_end_south", 0, 35, 32, 0xCC9155, false),
        ("tap_end_south", 0, 41, 53, 0xD1DCFF, false),
        ("tap_end_south", 0, 40, 41, 0xD37A57, true),
        ("tap_end_south", 0, 41, 43, 0xE2F4F8, false),
        ("tap_end_south", 0, 34, 47, 0xE3779D, false),
        ("tap_end_south", 0, 39, 49, 0xE8E2D1, false),
        ("tap_end_south", 0, 43, 40, 0xEFCC86, false),
        ("tap_end_south", 0, 46, 49, 0xF0B988, true),
        ("tap_end_south", 0, 49, 44, 0xF5F5F5, false),
        ("tap_end_south", 0, 42, 33, 0xFAECB2, false),
        ("tap_end_south", 0, 45, 48, 0xFCD9B3, true),
        ("tap_end_south", 0, 33, 42, 0xFFDA6A, false),
        ("tap_loop_south", 0, 35, 47, 0x000000, false),
        ("tap_loop_south", 0, 40, 41, 0x1A2748, false),
        ("tap_loop_south", 0, 35, 44, 0x343973, false),
        ("tap_loop_south", 0, 38, 43, 0x373357, false),
        ("tap_loop_south", 0, 40, 51, 0x3C1F2F, false),
        ("tap_loop_south", 0, 36, 43, 0x5263B7, false),
        ("tap_loop_south", 0, 42, 46, 0x57342B, false),
        ("tap_loop_south", 0, 36, 50, 0x5B3249, false),
        ("tap_loop_south", 0, 37, 51, 0x6A475B, false),
        ("tap_loop_south", 0, 40, 43, 0x7089D8, false),
        ("tap_loop_south", 0, 35, 38, 0x7482B0, false),
        ("tap_loop_south", 0, 41, 40, 0x762E21, false),
        ("tap_loop_south", 0, 45, 45, 0x7D4F3D, false),
        ("tap_loop_south", 0, 43, 44, 0x837CA0, false),
        ("tap_loop_south", 0, 42, 35, 0xA59DA2, false),
        ("tap_loop_south", 0, 44, 44, 0xA8A5B5, false),
        ("tap_loop_south", 0, 37, 49, 0xB09E87, false),
        ("tap_loop_south", 0, 41, 32, 0xB65932, false),
        ("tap_loop_south", 0, 41, 41, 0xB8BABB, false),
        ("tap_loop_south", 0, 38, 41, 0xBA7F3E, false),
        ("tap_loop_south", 0, 38, 35, 0xC2B9BE, false),
        ("tap_loop_south", 0, 40, 45, 0xC5895C, false),
        ("tap_loop_south", 0, 34, 31, 0xCC9155, false),
        ("tap_loop_south", 0, 41, 53, 0xD1DCFF, false),
        ("tap_loop_south", 0, 36, 52, 0xD37A57, true),
        ("tap_loop_south", 0, 40, 42, 0xE2F4F8, false),
        ("tap_loop_south", 0, 39, 49, 0xE8E2D1, false),
        ("tap_loop_south", 0, 38, 37, 0xECF0E9, false),
        ("tap_loop_south", 0, 39, 31, 0xEFCC86, false),
        ("tap_loop_south", 0, 42, 48, 0xF0B988, true),
        ("tap_loop_south", 0, 45, 43, 0xF5F5F5, false),
        ("tap_loop_south", 0, 37, 32, 0xFAECB2, false),
        ("tap_loop_south", 0, 41, 47, 0xFCD9B3, true),
        ("tap_loop_south", 0, 38, 40, 0xFFDA6A, false),
        ("tap_loop_south", 1, 41, 40, 0x762E21, false),
        ("tap_loop_south", 1, 35, 47, 0xD37A57, true),
        ("tap_loop_south", 1, 42, 48, 0xF0B988, true),
        ("tap_loop_south", 1, 41, 47, 0xFCD9B3, true),
        ("tap_loop_south", 2, 38, 40, 0x762E21, false),
        ("tap_loop_south", 2, 36, 52, 0xD37A57, true),
        ("tap_loop_south", 2, 42, 48, 0xF0B988, true),
        ("tap_loop_south", 2, 41, 47, 0xFCD9B3, true),
        ("tap_start_south", 0, 42, 47, 0x000000, false),
        ("tap_start_south", 0, 39, 43, 0x1A2748, false),
        ("tap_start_south", 0, 40, 42, 0x30447A, false),
        ("tap_start_south", 0, 37, 46, 0x343973, false),
        ("tap_start_south", 0, 33, 46, 0x373357, false),
        ("tap_start_south", 0, 42, 51, 0x3C1F2F, false),
        ("tap_start_south", 0, 37, 45, 0x5263B7, false),
        ("tap_start_south", 0, 46, 47, 0x57342B, false),
        ("tap_start_south", 0, 42, 50, 0x5B3249, false),
        ("tap_start_south", 0, 41, 51, 0x6A475B, false),
        ("tap_start_south", 0, 33, 45, 0x6B5C7C, false),
        ("tap_start_south", 0, 41, 44, 0x7089D8, false),
        ("tap_start_south", 0, 44, 39, 0x7482B0, false),
        ("tap_start_south", 0, 38, 41, 0x762E21, false),
        ("tap_start_south", 0, 37, 47, 0x7B6A55, false),
        ("tap_start_south", 0, 49, 46, 0x7D4F3D, false),
        ("tap_start_south", 0, 47, 45, 0x837CA0, false),
        ("tap_start_south", 0, 48, 45, 0xA8A5B5, false),
        ("tap_start_south", 0, 39, 48, 0xB09E87, false),
        ("tap_start_south", 0, 45, 33, 0xB65932, false),
        ("tap_start_south", 0, 42, 42, 0xB8BABB, false),
        ("tap_start_south", 0, 32, 42, 0xBA7F3E, false),
        ("tap_start_south", 0, 44, 46, 0xC5895C, false),
        ("tap_start_south", 0, 35, 32, 0xCC9155, false),
        ("tap_start_south", 0, 41, 53, 0xD1DCFF, false),
        ("tap_start_south", 0, 40, 41, 0xD37A57, true),
        ("tap_start_south", 0, 41, 43, 0xE2F4F8, false),
        ("tap_start_south", 0, 34, 47, 0xE3779D, false),
        ("tap_start_south", 0, 39, 49, 0xE8E2D1, false),
        ("tap_start_south", 0, 43, 40, 0xEFCC86, false),
        ("tap_start_south", 0, 46, 49, 0xF0B988, true),
        ("tap_start_south", 0, 49, 44, 0xF5F5F5, false),
        ("tap_start_south", 0, 42, 33, 0xFAECB2, false),
        ("tap_start_south", 0, 45, 48, 0xFCD9B3, true),
        ("tap_start_south", 0, 33, 42, 0xFFDA6A, false),
        ("tap_loop_south", 1, 40, 43, 0xBA7F3E, false),
        ("tap_loop_south", 1, 40, 42, 0xFFDA6A, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("tap_end_south", &[43]),
        ("tap_loop_south", &[33, 38, 35]),
        ("tap_start_south", &[43]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [10, 11, 12]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_nora_specialanimation_autumn";
            let asset = format!("assets/animations/NPCs/Nora/Sprites/Autumn/{prefix}_{case}.png");
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
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Nora Autumn material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin separately from changed pixels.
                if source.iter().any(|c| rgba(*c) == p.0) {
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
