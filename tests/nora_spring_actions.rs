use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/nora and the local accepted Nora world baseline"]
fn nora_spring_actions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/nora");
    let baseline = root.join("local/fixtures/slice-061-build/characters/nora");
    let set = std::env::var_os("FOM_NORA_SPRING_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/nora-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/nora-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..38];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..38], prior);
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
    // Source grids separate skin from hair, blouse, overalls, shoes and mouth interiors.
    // In these pinned strips, the deepest skin shade belongs to hands from row 42.
    // The same shade above that row is hair; the other world shades are skin.
    let landmarks = [
        ("blink_east", 0, 34, 48, 0x000000, false),
        ("blink_east", 0, 38, 48, 0x163253, false),
        ("blink_east", 0, 38, 50, 0x254397, false),
        ("blink_east", 0, 40, 48, 0x3B5370, false),
        ("blink_east", 0, 40, 45, 0x5078E7, false),
        ("blink_east", 0, 34, 31, 0x762E21, false),
        ("blink_east", 0, 35, 47, 0x762E21, true),
        ("blink_east", 0, 42, 42, 0x91AEFF, false),
        ("blink_east", 0, 43, 36, 0xA59DA2, false),
        ("blink_east", 0, 39, 33, 0xB65932, false),
        ("blink_east", 0, 43, 43, 0xBEBC91, false),
        ("blink_east", 0, 38, 36, 0xC2B9BE, false),
        ("blink_east", 0, 41, 31, 0xCC9155, false),
        ("blink_east", 0, 34, 47, 0xD37A57, true),
        ("blink_east", 0, 43, 37, 0xECF0E9, false),
        ("blink_east", 0, 40, 31, 0xEFCC86, false),
        ("blink_east", 0, 45, 46, 0xF0B988, true),
        ("blink_east", 0, 38, 32, 0xFAECB2, false),
        ("blink_east", 0, 35, 46, 0xFCD9B3, true),
        ("blink_east", 0, 40, 42, 0xFFFDD3, false),
        ("blink_east", 1, 34, 31, 0x762E21, false),
        ("blink_east", 1, 35, 47, 0x762E21, true),
        ("blink_east", 1, 34, 47, 0xD37A57, true),
        ("blink_east", 1, 45, 46, 0xF0B988, true),
        ("blink_east", 1, 35, 46, 0xFCD9B3, true),
        ("blink_south", 0, 34, 48, 0x000000, false),
        ("blink_south", 0, 37, 48, 0x163253, false),
        ("blink_south", 0, 42, 45, 0x254397, false),
        ("blink_south", 0, 39, 48, 0x3B5370, false),
        ("blink_south", 0, 39, 45, 0x5078E7, false),
        ("blink_south", 0, 40, 29, 0x762E21, false),
        ("blink_south", 0, 46, 47, 0x762E21, true),
        ("blink_south", 0, 41, 42, 0x91AEFF, false),
        ("blink_south", 0, 41, 32, 0xB65932, false),
        ("blink_south", 0, 36, 44, 0xBEBC91, false),
        ("blink_south", 0, 42, 36, 0xC2B9BE, false),
        ("blink_south", 0, 35, 31, 0xCC9155, false),
        ("blink_south", 0, 46, 45, 0xD37A57, true),
        ("blink_south", 0, 42, 37, 0xECF0E9, false),
        ("blink_south", 0, 41, 31, 0xEFCC86, false),
        ("blink_south", 0, 38, 52, 0xF0B988, true),
        ("blink_south", 0, 42, 32, 0xFAECB2, false),
        ("blink_south", 0, 46, 46, 0xFCD9B3, true),
        ("blink_south", 0, 40, 42, 0xFFFDD3, false),
        ("blink_south", 1, 40, 29, 0x762E21, false),
        ("blink_south", 1, 46, 47, 0x762E21, true),
        ("blink_south", 1, 46, 45, 0xD37A57, true),
        ("blink_south", 1, 38, 52, 0xF0B988, true),
        ("blink_south", 1, 46, 46, 0xFCD9B3, true),
        ("blink_south", 2, 38, 52, 0xF0B988, true),
        ("drink_east", 1, 32, 32, 0x762E21, false),
        ("drink_east", 1, 41, 48, 0xD37A57, true),
        ("drink_east", 1, 39, 42, 0xF0B988, true),
        ("drink_east", 1, 39, 41, 0xFCD9B3, true),
        ("drink_north", 0, 36, 46, 0x000000, false),
        ("drink_north", 0, 38, 47, 0x163253, false),
        ("drink_north", 0, 37, 44, 0x254397, false),
        ("drink_north", 0, 41, 46, 0x3B5370, false),
        ("drink_north", 0, 40, 43, 0x5078E7, false),
        ("drink_north", 0, 40, 34, 0x762E21, false),
        ("drink_north", 0, 34, 47, 0x762E21, true),
        ("drink_north", 0, 40, 41, 0x91AEFF, false),
        ("drink_north", 0, 45, 32, 0xB65932, false),
        ("drink_north", 0, 34, 44, 0xBEBC91, false),
        ("drink_north", 0, 43, 34, 0xCC9155, false),
        ("drink_north", 0, 34, 45, 0xD37A57, true),
        ("drink_north", 0, 42, 32, 0xEFCC86, false),
        ("drink_north", 0, 33, 46, 0xF0B988, true),
        ("drink_north", 0, 40, 31, 0xFAECB2, false),
        ("drink_north", 0, 44, 37, 0xFCD9B3, true),
        ("drink_north", 0, 43, 42, 0xFFFDD3, false),
        ("drink_north", 1, 42, 40, 0x762E21, false),
        ("drink_north", 1, 34, 47, 0x762E21, true),
        ("drink_north", 1, 45, 42, 0xD37A57, true),
        ("drink_north", 1, 34, 46, 0xF0B988, true),
        ("drink_north", 1, 44, 38, 0xFCD9B3, true),
        ("drink_south", 1, 42, 40, 0x762E21, false),
        ("drink_south", 1, 45, 47, 0x762E21, true),
        ("drink_south", 1, 36, 42, 0xD37A57, true),
        ("drink_south", 1, 39, 42, 0xF0B988, true),
        ("drink_south", 1, 35, 41, 0xFCD9B3, true),
        ("eat_east", 0, 37, 45, 0x000000, false),
        ("eat_east", 0, 38, 47, 0x163253, false),
        ("eat_east", 0, 44, 45, 0x254397, false),
        ("eat_east", 0, 45, 48, 0x3B5370, false),
        ("eat_east", 0, 44, 46, 0x5078E7, false),
        ("eat_east", 0, 34, 31, 0x762E21, false),
        ("eat_east", 0, 43, 35, 0xA59DA2, false),
        ("eat_east", 0, 42, 32, 0xB65932, false),
        ("eat_east", 0, 42, 42, 0xBEBC91, false),
        ("eat_east", 0, 39, 35, 0xC2B9BE, false),
        ("eat_east", 0, 35, 31, 0xCC9155, false),
        ("eat_east", 0, 41, 40, 0xD37A57, true),
        ("eat_east", 0, 38, 37, 0xECF0E9, false),
        ("eat_east", 0, 40, 31, 0xEFCC86, false),
        ("eat_east", 0, 44, 47, 0xF0B988, true),
        ("eat_east", 0, 38, 32, 0xFAECB2, false),
        ("eat_east", 0, 43, 43, 0xFCD9B3, true),
        ("eat_east", 0, 40, 42, 0xFFFDD3, false),
        ("eat_east", 1, 35, 31, 0x762E21, false),
        ("eat_east", 1, 46, 42, 0x762E21, true),
        ("eat_east", 1, 45, 42, 0xD37A57, true),
        ("eat_east", 1, 44, 47, 0xF0B988, true),
        ("eat_east", 1, 45, 41, 0xFCD9B3, true),
        ("eat_north", 1, 42, 40, 0x762E21, false),
        ("eat_north", 1, 34, 47, 0x762E21, true),
        ("eat_north", 1, 45, 42, 0xD37A57, true),
        ("eat_north", 1, 34, 46, 0xF0B988, true),
        ("eat_north", 1, 44, 38, 0xFCD9B3, true),
        ("eat_south", 1, 42, 40, 0x762E21, false),
        ("eat_south", 1, 45, 47, 0x762E21, true),
        ("eat_south", 1, 40, 41, 0xD37A57, true),
        ("eat_south", 1, 46, 46, 0xF0B988, true),
        ("eat_south", 1, 37, 48, 0xFCD9B3, true),
        ("sit_north", 0, 33, 47, 0x000000, false),
        ("sit_north", 0, 38, 47, 0x163253, false),
        ("sit_north", 0, 37, 44, 0x254397, false),
        ("sit_north", 0, 41, 46, 0x3B5370, false),
        ("sit_north", 0, 40, 43, 0x5078E7, false),
        ("sit_north", 0, 40, 34, 0x762E21, false),
        ("sit_north", 0, 45, 47, 0x762E21, true),
        ("sit_north", 0, 40, 41, 0x91AEFF, false),
        ("sit_north", 0, 45, 32, 0xB65932, false),
        ("sit_north", 0, 36, 44, 0xBEBC91, false),
        ("sit_north", 0, 43, 34, 0xCC9155, false),
        ("sit_north", 0, 35, 46, 0xD37A57, true),
        ("sit_north", 0, 42, 32, 0xEFCC86, false),
        ("sit_north", 0, 46, 46, 0xF0B988, true),
        ("sit_north", 0, 40, 31, 0xFAECB2, false),
        ("sit_north", 0, 44, 37, 0xFCD9B3, true),
        ("sit_north", 0, 43, 42, 0xFFFDD3, false),
        ("drink_north", 0, 45, 42, 0x762E21, true),
        ("drink_north", 2, 45, 42, 0x762E21, true),
        ("eat_east", 1, 46, 42, 0x762E21, true),
        ("eat_north", 0, 45, 42, 0x762E21, true),
        ("eat_north", 2, 45, 42, 0x762E21, true),
        ("eat_south", 3, 35, 43, 0x762E21, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[42, 50, 42]),
        ("blink_south", &[47, 55, 47]),
        ("drink_east", &[32, 36, 32]),
        ("drink_north", &[15, 11, 15]),
        ("drink_south", &[42, 49, 42]),
        ("eat_east", &[29, 34, 29, 34, 31]),
        ("eat_north", &[15, 11, 15]),
        ("eat_south", &[42, 42, 42, 52, 42]),
        ("sit_east", &[34]),
        ("sit_north", &[14]),
        ("sit_south", &[42]),
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
            let prefix = "spr_npc_nora_spring";
            let asset = format!("assets/animations/NPCs/Nora/Sprites/Spring/{prefix}_{case}.png");
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
                // Include raised and lowered hands while preserving the dark hair tips.
                // Exposed ankles change; blue shoes and mouth interiors stay original.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 42))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Nora Spring material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin separately from changed pixels.
                if source
                    .iter()
                    .any(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 42))
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
