use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/holt and the local accepted Holt world baseline"]
fn holt_spring_actions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/holt");
    let baseline = root.join("local/fixtures/slice-085-build/characters/holt");
    let set = std::env::var_os("FOM_HOLT_SPRING_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/holt-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/holt-world-trial.json")).unwrap(),
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
    let source = [0xF0CC8E, 0xCD9067, 0x96523A, 0xAD6945];
    // Reviewed source landmarks separate hands from boots that reuse skin shades.
    // Eating South lowers one hand across the left boot in its second frame.
    // Hair, moustache, scarf, clothes and mouth interiors remain original.
    let landmarks = [
        ("blink_east", 0, 35, 48, 0x000000, false),
        ("blink_east", 0, 37, 49, 0x163253, false),
        ("blink_east", 0, 39, 48, 0x3B5370, false),
        ("blink_east", 0, 44, 43, 0x55908C, false),
        ("blink_east", 0, 41, 52, 0x762E21, false),
        ("blink_east", 0, 34, 43, 0x7AB99F, false),
        ("blink_east", 0, 41, 50, 0x837D60, false),
        ("blink_east", 0, 41, 51, 0x96523A, false),
        ("blink_east", 0, 35, 46, 0x96523A, true),
        ("blink_east", 0, 34, 44, 0x9FDDCF, false),
        ("blink_east", 0, 43, 35, 0xA59DA2, false),
        ("blink_east", 0, 44, 35, 0xAD6945, true),
        ("blink_east", 0, 42, 43, 0xB65932, false),
        ("blink_east", 0, 42, 50, 0xBEBC91, false),
        ("blink_east", 0, 38, 35, 0xC2B9BE, false),
        ("blink_east", 0, 33, 47, 0xCD9067, true),
        ("blink_east", 0, 44, 30, 0xD2793F, false),
        ("blink_east", 0, 43, 36, 0xECF0E9, false),
        ("blink_east", 0, 40, 53, 0xF0CC8E, false),
        ("blink_east", 0, 34, 45, 0xF0CC8E, true),
        ("blink_east", 0, 39, 45, 0xF1A354, false),
        ("blink_east", 0, 42, 41, 0xFCF2DE, false),
        ("blink_east", 1, 35, 46, 0x96523A, true),
        ("blink_east", 1, 44, 35, 0xAD6945, true),
        ("blink_east", 1, 33, 47, 0xCD9067, true),
        ("blink_east", 1, 34, 45, 0xF0CC8E, true),
        ("blink_east", 2, 33, 47, 0xCD9067, true),
        ("blink_south", 0, 33, 47, 0x96523A, true),
        ("blink_south", 1, 33, 47, 0x96523A, true),
        ("blink_south", 1, 45, 46, 0xCD9067, true),
        ("blink_south", 1, 33, 46, 0xF0CC8E, true),
        ("blink_south", 2, 33, 47, 0x96523A, true),
        ("drink_east", 1, 42, 36, 0x96523A, true),
        ("drink_east", 1, 42, 35, 0xAD6945, true),
        ("drink_east", 1, 40, 41, 0xCD9067, true),
        ("drink_east", 1, 40, 42, 0xF0CC8E, true),
        ("drink_north", 0, 47, 45, 0x000000, false),
        ("drink_north", 0, 43, 46, 0x163253, false),
        ("drink_north", 0, 48, 44, 0x3B5370, false),
        ("drink_north", 0, 47, 43, 0x55908C, false),
        ("drink_north", 0, 43, 37, 0x762E21, false),
        ("drink_north", 0, 45, 43, 0x7AB99F, false),
        ("drink_north", 0, 47, 39, 0x96523A, true),
        ("drink_north", 0, 32, 44, 0x9FDDCF, false),
        ("drink_north", 0, 37, 42, 0xB65932, false),
        ("drink_north", 0, 33, 47, 0xCD9067, true),
        ("drink_north", 0, 36, 33, 0xD2793F, false),
        ("drink_north", 0, 32, 46, 0xF0CC8E, true),
        ("drink_north", 0, 36, 41, 0xF1A354, false),
        ("drink_north", 0, 45, 40, 0xFCF2DE, false),
        ("drink_north", 1, 33, 47, 0xCD9067, true),
        ("drink_north", 1, 32, 46, 0xF0CC8E, true),
        ("drink_north", 2, 33, 47, 0xCD9067, true),
        ("drink_south", 1, 36, 41, 0x96523A, true),
        ("drink_south", 1, 36, 43, 0xCD9067, true),
        ("drink_south", 1, 35, 43, 0xF0CC8E, true),
        ("eat_east", 0, 45, 45, 0x000000, false),
        ("eat_east", 0, 37, 47, 0x163253, false),
        ("eat_east", 0, 40, 44, 0x3B5370, false),
        ("eat_east", 0, 40, 42, 0x55908C, false),
        ("eat_east", 0, 42, 49, 0x762E21, false),
        ("eat_east", 0, 39, 42, 0x7AB99F, false),
        ("eat_east", 0, 41, 47, 0x837D60, false),
        ("eat_east", 0, 41, 49, 0x96523A, false),
        ("eat_east", 0, 45, 42, 0x96523A, true),
        ("eat_east", 0, 42, 43, 0x9FDDCF, false),
        ("eat_east", 0, 43, 34, 0xA59DA2, false),
        ("eat_east", 0, 44, 35, 0xAD6945, true),
        ("eat_east", 0, 36, 41, 0xB65932, false),
        ("eat_east", 0, 44, 46, 0xBEBC91, false),
        ("eat_east", 0, 39, 34, 0xC2B9BE, false),
        ("eat_east", 0, 45, 44, 0xCD9067, true),
        ("eat_east", 0, 44, 30, 0xD2793F, false),
        ("eat_east", 0, 38, 36, 0xECF0E9, false),
        ("eat_east", 0, 44, 49, 0xF0CC8E, false),
        ("eat_east", 0, 44, 43, 0xF0CC8E, true),
        ("eat_east", 0, 40, 40, 0xF1A354, false),
        ("eat_east", 0, 38, 40, 0xFCF2DE, false),
        ("eat_east", 1, 46, 41, 0x96523A, true),
        ("eat_east", 1, 45, 35, 0xAD6945, true),
        ("eat_east", 1, 47, 40, 0xCD9067, true),
        ("eat_east", 1, 46, 40, 0xF0CC8E, true),
        ("eat_north", 0, 33, 47, 0xCD9067, true),
        ("eat_north", 1, 33, 47, 0xCD9067, true),
        ("eat_north", 1, 32, 46, 0xF0CC8E, true),
        ("eat_north", 2, 33, 47, 0xCD9067, true),
        ("eat_south", 0, 36, 48, 0x96523A, true),
        ("eat_south", 1, 37, 49, 0x96523A, true),
        ("eat_south", 1, 46, 46, 0xCD9067, true),
        ("eat_south", 1, 48, 46, 0xF0CC8E, true),
        ("sit_north", 0, 30, 47, 0x000000, false),
        ("sit_north", 0, 43, 46, 0x163253, false),
        ("sit_north", 0, 37, 46, 0x3B5370, false),
        ("sit_north", 0, 45, 43, 0x55908C, false),
        ("sit_north", 0, 43, 37, 0x762E21, false),
        ("sit_north", 0, 46, 43, 0x7AB99F, false),
        ("sit_north", 0, 45, 44, 0x9FDDCF, false),
        ("sit_north", 0, 37, 42, 0xB65932, false),
        ("sit_north", 0, 33, 47, 0xCD9067, true),
        ("sit_north", 0, 36, 33, 0xD2793F, false),
        ("sit_north", 0, 33, 46, 0xF0CC8E, true),
        ("sit_north", 0, 36, 41, 0xF1A354, false),
        ("sit_north", 0, 45, 40, 0xFCF2DE, false),
        ("sit_south", 0, 44, 47, 0x000000, false),
        ("sit_south", 0, 36, 46, 0x163253, false),
        ("sit_south", 0, 44, 43, 0x3B5370, false),
        ("sit_south", 0, 39, 43, 0x55908C, false),
        ("sit_south", 0, 41, 49, 0x762E21, false),
        ("sit_south", 0, 44, 42, 0x7AB99F, false),
        ("sit_south", 0, 42, 47, 0x837D60, false),
        ("sit_south", 0, 42, 38, 0x96523A, true),
        ("sit_south", 0, 45, 44, 0x9FDDCF, false),
        ("sit_south", 0, 37, 43, 0xB65932, false),
        ("sit_south", 0, 41, 47, 0xBEBC91, false),
        ("sit_south", 0, 41, 34, 0xC2B9BE, false),
        ("sit_south", 0, 42, 48, 0xCD9067, false),
        ("sit_south", 0, 46, 46, 0xCD9067, true),
        ("sit_south", 0, 43, 30, 0xD2793F, false),
        ("sit_south", 0, 37, 36, 0xECF0E9, false),
        ("sit_south", 0, 38, 50, 0xF0CC8E, false),
        ("sit_south", 0, 47, 45, 0xF0CC8E, true),
        ("sit_south", 0, 41, 42, 0xF1A354, false),
        ("sit_south", 0, 38, 41, 0xFCF2DE, false),
        ("eat_south", 0, 36, 48, 0x96523A, true),
        ("eat_south", 1, 37, 49, 0x96523A, true),
        ("eat_south", 1, 38, 50, 0xF0CC8E, false),
        ("sit_east", 0, 41, 48, 0x96523A, false),
        ("eat_east", 1, 41, 38, 0x9E2626, false),
        ("eat_east", 1, 38, 37, 0x762E21, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[43, 51, 43]),
        ("blink_south", &[50, 58, 50]),
        ("drink_east", &[37, 45, 37]),
        ("drink_north", &[16, 16, 16]),
        ("drink_south", &[47, 58, 47]),
        ("eat_east", &[36, 37, 44, 43, 37]),
        ("eat_north", &[16, 16, 16]),
        ("eat_south", &[47, 52, 55, 62, 47]),
        ("sit_east", &[37]),
        ("sit_north", &[24]),
        ("sit_south", &[47]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [9, 1, 10, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_holt_spring";
            let asset = format!("assets/animations/NPCs/Holt/Sprites/Spring/{prefix}_{case}.png");
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
                // Literal bounds describe these pinned strips, independently of mask seeds.
                let local_x = x % 80;
                let in_skin_area = if case.starts_with("blink_") {
                    y < 49
                } else if case.ends_with("_north") {
                    true
                } else if case.ends_with("_east") {
                    y < 47 || !(41..=45).contains(&local_x)
                } else {
                    y < 48
                        || !(37..=42).contains(&local_x)
                        || (case == "eat_south" && x / 80 == 1 && local_x <= 37 && y <= 49)
                };
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && in_skin_area)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Holt Spring material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin even when a Ryis target equals its source.
                if in_skin_area && source.iter().any(|c| rgba(*c) == p.0) {
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
