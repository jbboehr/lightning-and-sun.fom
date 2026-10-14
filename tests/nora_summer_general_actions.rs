use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/nora and the local accepted Nora world baseline"]
fn nora_summer_general_actions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/nora");
    let baseline = root.join("local/fixtures/slice-068-build/characters/nora");
    let set = std::env::var_os("FOM_NORA_SUMMER_GENERAL_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/nora-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/nora-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..81];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..81], prior);
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
    // Source grids separate skin from hair, clothing and shoes.
    // In these pinned strips, the deepest skin shade belongs to hands from row 41.
    // North-facing frames 3 and 5 also have a hand edge at [45,40].
    // Other occurrences above row 41 are hair; the other world shades are skin.
    let landmarks = [
        ("action_east", 0, 42, 48, 0x000000, false),
        ("action_east", 0, 40, 44, 0x23325B, false),
        ("action_east", 0, 39, 44, 0x354981, false),
        ("action_east", 0, 43, 47, 0x3C1F2F, false),
        ("action_east", 0, 43, 45, 0x3F599E, false),
        ("action_east", 0, 42, 44, 0x516FBE, false),
        ("action_east", 0, 42, 49, 0x6A475B, false),
        ("action_east", 0, 44, 40, 0x762E21, false),
        ("action_east", 0, 40, 48, 0x762E21, true),
        ("action_east", 0, 39, 51, 0x807467, false),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 43, 33, 0xB65932, false),
        ("action_east", 0, 40, 51, 0xB8A68F, false),
        ("action_east", 0, 40, 36, 0xC2B9BE, false),
        ("action_east", 0, 45, 40, 0xCC9155, false),
        ("action_east", 0, 38, 47, 0xD37A57, true),
        ("action_east", 0, 43, 50, 0xE8E2D1, false),
        ("action_east", 0, 39, 38, 0xECF0E9, false),
        ("action_east", 0, 38, 40, 0xEFCC86, false),
        ("action_east", 0, 38, 46, 0xF0B988, true),
        ("action_east", 0, 39, 33, 0xFAECB2, false),
        ("action_east", 0, 40, 47, 0xFCD9B3, true),
        ("action_east", 1, 37, 31, 0x762E21, false),
        ("action_east", 1, 44, 52, 0x762E21, true),
        ("action_east", 1, 45, 44, 0xD37A57, true),
        ("action_east", 1, 47, 45, 0xF0B988, true),
        ("action_east", 1, 46, 44, 0xFCD9B3, true),
        ("action_east", 2, 37, 31, 0x762E21, false),
        ("action_east", 2, 44, 52, 0x762E21, true),
        ("action_east", 2, 43, 45, 0xD37A57, true),
        ("action_east", 2, 38, 51, 0xF0B988, true),
        ("action_east", 2, 45, 44, 0xFCD9B3, true),
        ("action_east", 3, 37, 31, 0x762E21, false),
        ("action_east", 3, 44, 52, 0x762E21, true),
        ("action_east", 3, 45, 44, 0xD37A57, true),
        ("action_east", 3, 47, 45, 0xF0B988, true),
        ("action_east", 3, 46, 44, 0xFCD9B3, true),
        ("action_east", 4, 37, 31, 0x762E21, false),
        ("action_east", 4, 44, 52, 0x762E21, true),
        ("action_east", 4, 43, 45, 0xD37A57, true),
        ("action_east", 4, 38, 51, 0xF0B988, true),
        ("action_east", 4, 45, 44, 0xFCD9B3, true),
        ("action_east", 5, 44, 40, 0x762E21, false),
        ("action_east", 5, 40, 48, 0x762E21, true),
        ("action_east", 5, 38, 47, 0xD37A57, true),
        ("action_east", 5, 38, 46, 0xF0B988, true),
        ("action_east", 5, 40, 47, 0xFCD9B3, true),
        ("action_east", 6, 34, 31, 0x762E21, false),
        ("action_east", 6, 35, 47, 0x762E21, true),
        ("action_east", 6, 34, 45, 0xD37A57, true),
        ("action_east", 6, 45, 46, 0xF0B988, true),
        ("action_east", 6, 35, 46, 0xFCD9B3, true),
        ("action_north", 0, 36, 46, 0x000000, false),
        ("action_north", 0, 44, 43, 0x23325B, false),
        ("action_north", 0, 41, 42, 0x354981, false),
        ("action_north", 0, 42, 46, 0x3C1F2F, false),
        ("action_north", 0, 38, 45, 0x3F599E, false),
        ("action_north", 0, 36, 42, 0x516FBE, false),
        ("action_north", 0, 40, 46, 0x6A475B, false),
        ("action_north", 0, 40, 34, 0x762E21, false),
        ("action_north", 0, 33, 45, 0x762E21, true),
        ("action_north", 0, 40, 50, 0x807467, false),
        ("action_north", 0, 45, 32, 0xB65932, false),
        ("action_north", 0, 41, 49, 0xB8A68F, false),
        ("action_north", 0, 44, 38, 0xC2B9BE, false),
        ("action_north", 0, 43, 34, 0xCC9155, false),
        ("action_north", 0, 42, 47, 0xCCC5AF, false),
        ("action_north", 0, 33, 43, 0xD37A57, true),
        ("action_north", 0, 38, 48, 0xE8E2D1, false),
        ("action_north", 0, 42, 32, 0xEFCC86, false),
        ("action_north", 0, 45, 44, 0xF0B988, true),
        ("action_north", 0, 40, 31, 0xFAECB2, false),
        ("action_north", 0, 34, 44, 0xFCD9B3, true),
        ("action_north", 1, 40, 33, 0x762E21, false),
        ("action_north", 1, 35, 46, 0x762E21, true),
        ("action_north", 1, 34, 44, 0xD37A57, true),
        ("action_north", 1, 37, 51, 0xF0B988, true),
        ("action_north", 1, 34, 46, 0xFCD9B3, true),
        ("action_north", 2, 45, 40, 0x762E21, true),
        ("action_north", 2, 35, 46, 0x762E21, true),
        ("action_north", 2, 34, 44, 0xD37A57, true),
        ("action_north", 2, 37, 51, 0xF0B988, true),
        ("action_north", 2, 34, 46, 0xFCD9B3, true),
        ("action_north", 3, 40, 33, 0x762E21, false),
        ("action_north", 3, 35, 46, 0x762E21, true),
        ("action_north", 3, 34, 44, 0xD37A57, true),
        ("action_north", 3, 37, 51, 0xF0B988, true),
        ("action_north", 3, 34, 46, 0xFCD9B3, true),
        ("action_north", 4, 45, 40, 0x762E21, true),
        ("action_north", 4, 35, 46, 0x762E21, true),
        ("action_north", 4, 34, 44, 0xD37A57, true),
        ("action_north", 4, 37, 51, 0xF0B988, true),
        ("action_north", 4, 34, 46, 0xFCD9B3, true),
        ("action_north", 5, 40, 34, 0x762E21, false),
        ("action_north", 5, 34, 47, 0x762E21, true),
        ("action_north", 5, 46, 44, 0xD37A57, true),
        ("action_north", 5, 45, 44, 0xF0B988, true),
        ("action_north", 5, 34, 46, 0xFCD9B3, true),
        ("action_north", 6, 40, 34, 0x762E21, false),
        ("action_north", 6, 46, 47, 0x762E21, true),
        ("action_north", 6, 35, 45, 0xD37A57, true),
        ("action_north", 6, 37, 52, 0xF0B988, true),
        ("action_north", 6, 47, 46, 0xFCD9B3, true),
        ("action_south", 0, 36, 48, 0x000000, false),
        ("action_south", 0, 42, 44, 0x23325B, false),
        ("action_south", 0, 36, 44, 0x354981, false),
        ("action_south", 0, 40, 48, 0x3C1F2F, false),
        ("action_south", 0, 38, 45, 0x3F599E, false),
        ("action_south", 0, 40, 44, 0x516FBE, false),
        ("action_south", 0, 39, 49, 0x6A475B, false),
        ("action_south", 0, 42, 40, 0x762E21, false),
        ("action_south", 0, 46, 47, 0x762E21, true),
        ("action_south", 0, 38, 51, 0x807467, false),
        ("action_south", 0, 40, 33, 0xB65932, false),
        ("action_south", 0, 41, 51, 0xB8A68F, false),
        ("action_south", 0, 41, 36, 0xC2B9BE, false),
        ("action_south", 0, 35, 32, 0xCC9155, false),
        ("action_south", 0, 45, 46, 0xD37A57, true),
        ("action_south", 0, 42, 50, 0xE8E2D1, false),
        ("action_south", 0, 42, 38, 0xECF0E9, false),
        ("action_south", 0, 43, 40, 0xEFCC86, false),
        ("action_south", 0, 44, 45, 0xF0B988, true),
        ("action_south", 0, 42, 33, 0xFAECB2, false),
        ("action_south", 0, 34, 47, 0xFCD9B3, true),
        ("action_south", 1, 40, 29, 0x762E21, false),
        ("action_south", 1, 35, 47, 0x762E21, true),
        ("action_south", 1, 45, 45, 0xD37A57, true),
        ("action_south", 1, 38, 52, 0xF0B988, true),
        ("action_south", 1, 35, 46, 0xFCD9B3, true),
        ("action_south", 2, 40, 29, 0x762E21, false),
        ("action_south", 2, 38, 47, 0x762E21, true),
        ("action_south", 2, 36, 46, 0xD37A57, true),
        ("action_south", 2, 38, 45, 0xF0B988, true),
        ("action_south", 2, 38, 46, 0xFCD9B3, true),
        ("action_south", 3, 40, 29, 0x762E21, false),
        ("action_south", 3, 35, 47, 0x762E21, true),
        ("action_south", 3, 45, 45, 0xD37A57, true),
        ("action_south", 3, 38, 52, 0xF0B988, true),
        ("action_south", 3, 35, 46, 0xFCD9B3, true),
        ("action_south", 4, 40, 29, 0x762E21, false),
        ("action_south", 4, 38, 47, 0x762E21, true),
        ("action_south", 4, 36, 46, 0xD37A57, true),
        ("action_south", 4, 38, 45, 0xF0B988, true),
        ("action_south", 4, 38, 46, 0xFCD9B3, true),
        ("action_south", 5, 42, 40, 0x762E21, false),
        ("action_south", 5, 46, 47, 0x762E21, true),
        ("action_south", 5, 45, 46, 0xD37A57, true),
        ("action_south", 5, 44, 45, 0xF0B988, true),
        ("action_south", 5, 34, 47, 0xFCD9B3, true),
        ("action_south", 6, 40, 29, 0x762E21, false),
        ("action_south", 6, 46, 47, 0x762E21, true),
        ("action_south", 6, 35, 45, 0xD37A57, true),
        ("action_south", 6, 38, 52, 0xF0B988, true),
        ("action_south", 6, 46, 46, 0xFCD9B3, true),
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
        ("action_east", &[38, 38, 35, 38, 35, 38, 42]),
        ("action_north", &[23, 16, 18, 16, 18, 25, 28]),
        ("action_south", &[46, 44, 45, 44, 45, 46, 50]),
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
                // Moving hands and exposed ankles change; hair, clothing, shoes and mouth interiors do not.
                let role = source.iter().position(|c| {
                    rgba(*c) == p.0
                        && (*c != 0x762E21
                            || y >= 41
                            || (case == "action_north"
                                && y == 40
                                && x % 80 == 45
                                && matches!(x / 80, 2 | 4)))
                });
                let expected = role.map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Nora Summer material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin separately from changed pixels.
                if role.is_some() {
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
