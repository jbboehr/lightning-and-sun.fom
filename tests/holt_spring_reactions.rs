use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/holt and the local accepted Holt world baseline"]
fn holt_spring_reactions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/holt");
    let baseline = root.join("generated/slice-086-build/characters/holt");
    let set = std::env::var_os("FOM_HOLT_SPRING_REACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/holt-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/holt-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..49];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..49], prior);
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
    // Skin remains above row 49 in these pinned action and reaction strips.
    // Raised fingers change; boots reuse skin shades below that boundary.
    // Hair, moustache, scarf, clothes and mouth interiors remain original.
    let landmarks = [
        ("action_east", 0, 37, 49, 0x000000, false),
        ("action_east", 0, 40, 50, 0x163253, false),
        ("action_east", 0, 42, 49, 0x3B5370, false),
        ("action_east", 0, 42, 44, 0x55908C, false),
        ("action_east", 0, 42, 52, 0x762E21, false),
        ("action_east", 0, 38, 45, 0x7AB99F, false),
        ("action_east", 0, 42, 51, 0x837D60, false),
        ("action_east", 0, 42, 53, 0x96523A, false),
        ("action_east", 0, 41, 46, 0x96523A, true),
        ("action_east", 0, 40, 45, 0x9FDDCF, false),
        ("action_east", 0, 44, 35, 0xA59DA2, false),
        ("action_east", 0, 45, 36, 0xAD6945, true),
        ("action_east", 0, 41, 44, 0xB65932, false),
        ("action_east", 0, 43, 51, 0xBEBC91, false),
        ("action_east", 0, 40, 35, 0xC2B9BE, false),
        ("action_east", 0, 42, 40, 0xCD9067, true),
        ("action_east", 0, 45, 31, 0xD2793F, false),
        ("action_east", 0, 39, 37, 0xECF0E9, false),
        ("action_east", 0, 43, 53, 0xF0CC8E, false),
        ("action_east", 0, 39, 47, 0xF0CC8E, true),
        ("action_east", 0, 42, 42, 0xF1A354, false),
        ("action_east", 0, 41, 42, 0xFCF2DE, false),
        ("action_east", 1, 48, 42, 0x96523A, true),
        ("action_east", 1, 47, 35, 0xAD6945, true),
        ("action_east", 1, 48, 44, 0xCD9067, true),
        ("action_east", 1, 47, 43, 0xF0CC8E, true),
        ("action_north", 0, 35, 47, 0x000000, false),
        ("action_north", 0, 41, 47, 0x163253, false),
        ("action_north", 0, 39, 46, 0x3B5370, false),
        ("action_north", 0, 46, 43, 0x55908C, false),
        ("action_north", 0, 41, 52, 0x762E21, false),
        ("action_north", 0, 45, 43, 0x7AB99F, false),
        ("action_north", 0, 42, 50, 0x837D60, false),
        ("action_north", 0, 34, 45, 0x96523A, true),
        ("action_north", 0, 33, 44, 0x9FDDCF, false),
        ("action_north", 0, 37, 42, 0xB65932, false),
        ("action_north", 0, 41, 50, 0xBEBC91, false),
        ("action_north", 0, 37, 53, 0xCD9067, false),
        ("action_north", 0, 31, 46, 0xCD9067, true),
        ("action_north", 0, 36, 33, 0xD2793F, false),
        ("action_north", 0, 38, 53, 0xF0CC8E, false),
        ("action_north", 0, 32, 45, 0xF0CC8E, true),
        ("action_north", 0, 36, 41, 0xF1A354, false),
        ("action_north", 0, 43, 40, 0xFCF2DE, false),
        ("action_north", 1, 36, 46, 0x96523A, true),
        ("action_north", 1, 35, 46, 0xCD9067, true),
        ("action_north", 1, 34, 45, 0xF0CC8E, true),
        ("action_south", 0, 44, 48, 0x000000, false),
        ("action_south", 0, 43, 49, 0x163253, false),
        ("action_south", 0, 38, 49, 0x3B5370, false),
        ("action_south", 0, 40, 44, 0x55908C, false),
        ("action_south", 0, 41, 52, 0x762E21, false),
        ("action_south", 0, 33, 44, 0x7AB99F, false),
        ("action_south", 0, 42, 51, 0x837D60, false),
        ("action_south", 0, 35, 48, 0x96523A, true),
        ("action_south", 0, 35, 45, 0x9FDDCF, false),
        ("action_south", 0, 40, 43, 0xB65932, false),
        ("action_south", 0, 41, 51, 0xBEBC91, false),
        ("action_south", 0, 41, 35, 0xC2B9BE, false),
        ("action_south", 0, 46, 47, 0xCD9067, true),
        ("action_south", 0, 43, 31, 0xD2793F, false),
        ("action_south", 0, 37, 37, 0xECF0E9, false),
        ("action_south", 0, 41, 53, 0xF0CC8E, false),
        ("action_south", 0, 34, 46, 0xF0CC8E, true),
        ("action_south", 0, 41, 43, 0xF1A354, false),
        ("action_south", 0, 45, 41, 0xFCF2DE, false),
        ("action_south", 1, 45, 46, 0x96523A, true),
        ("action_south", 1, 46, 46, 0xCD9067, true),
        ("action_south", 1, 35, 45, 0xF0CC8E, true),
        ("shocked_end_south", 0, 48, 48, 0x000000, false),
        ("shocked_end_south", 0, 36, 49, 0x163253, false),
        ("shocked_end_south", 0, 37, 49, 0x3B5370, false),
        ("shocked_end_south", 0, 43, 44, 0x55908C, false),
        ("shocked_end_south", 0, 41, 52, 0x762E21, false),
        ("shocked_end_south", 0, 33, 44, 0x7AB99F, false),
        ("shocked_end_south", 0, 42, 51, 0x837D60, false),
        ("shocked_end_south", 0, 33, 48, 0x96523A, true),
        ("shocked_end_south", 0, 45, 45, 0x9FDDCF, false),
        ("shocked_end_south", 0, 40, 43, 0xB65932, false),
        ("shocked_end_south", 0, 41, 51, 0xBEBC91, false),
        ("shocked_end_south", 0, 34, 47, 0xCD9067, true),
        ("shocked_end_south", 0, 43, 31, 0xD2793F, false),
        ("shocked_end_south", 0, 41, 53, 0xF0CC8E, false),
        ("shocked_end_south", 0, 33, 47, 0xF0CC8E, true),
        ("shocked_end_south", 0, 41, 43, 0xF1A354, false),
        ("shocked_end_south", 0, 45, 41, 0xFCF2DE, false),
        ("shocked_loop_south", 0, 41, 48, 0x000000, false),
        ("shocked_loop_south", 0, 44, 46, 0x163253, false),
        ("shocked_loop_south", 0, 37, 46, 0x3B5370, false),
        ("shocked_loop_south", 0, 40, 35, 0x410808, false),
        ("shocked_loop_south", 0, 36, 42, 0x55908C, false),
        ("shocked_loop_south", 0, 34, 50, 0x762E21, false),
        ("shocked_loop_south", 0, 38, 42, 0x7AB99F, false),
        ("shocked_loop_south", 0, 45, 48, 0x837D60, false),
        ("shocked_loop_south", 0, 35, 33, 0x96523A, true),
        ("shocked_loop_south", 0, 40, 36, 0x9E2626, false),
        ("shocked_loop_south", 0, 33, 38, 0x9FDDCF, false),
        ("shocked_loop_south", 0, 38, 44, 0xB65932, false),
        ("shocked_loop_south", 0, 44, 48, 0xBEBC91, false),
        ("shocked_loop_south", 0, 41, 32, 0xC2B9BE, false),
        ("shocked_loop_south", 0, 34, 51, 0xCD9067, false),
        ("shocked_loop_south", 0, 49, 33, 0xCD9067, true),
        ("shocked_loop_south", 0, 36, 28, 0xD2793F, false),
        ("shocked_loop_south", 0, 38, 34, 0xECF0E9, false),
        ("shocked_loop_south", 0, 47, 50, 0xF0CC8E, false),
        ("shocked_loop_south", 0, 46, 34, 0xF0CC8E, true),
        ("shocked_loop_south", 0, 40, 43, 0xF1A354, false),
        ("shocked_loop_south", 0, 42, 40, 0xFCF2DE, false),
        ("shocked_start_south", 0, 48, 48, 0x000000, false),
        ("shocked_start_south", 0, 36, 49, 0x163253, false),
        ("shocked_start_south", 0, 37, 49, 0x3B5370, false),
        ("shocked_start_south", 0, 43, 44, 0x55908C, false),
        ("shocked_start_south", 0, 41, 52, 0x762E21, false),
        ("shocked_start_south", 0, 33, 44, 0x7AB99F, false),
        ("shocked_start_south", 0, 42, 51, 0x837D60, false),
        ("shocked_start_south", 0, 33, 48, 0x96523A, true),
        ("shocked_start_south", 0, 45, 45, 0x9FDDCF, false),
        ("shocked_start_south", 0, 40, 43, 0xB65932, false),
        ("shocked_start_south", 0, 41, 51, 0xBEBC91, false),
        ("shocked_start_south", 0, 34, 47, 0xCD9067, true),
        ("shocked_start_south", 0, 43, 31, 0xD2793F, false),
        ("shocked_start_south", 0, 41, 53, 0xF0CC8E, false),
        ("shocked_start_south", 0, 33, 47, 0xF0CC8E, true),
        ("shocked_start_south", 0, 41, 43, 0xF1A354, false),
        ("shocked_start_south", 0, 45, 41, 0xFCF2DE, false),
        ("shocked_loop_south", 0, 32, 32, 0x96523A, true),
        ("shocked_loop_south", 0, 47, 32, 0x96523A, true),
        ("shocked_loop_south", 0, 34, 49, 0xF0CC8E, false),
        ("shocked_loop_south", 0, 39, 35, 0x410808, false),
        ("shocked_loop_south", 0, 39, 36, 0x9E2626, false),
        ("action_east", 1, 47, 35, 0xAD6945, true),
        ("action_east", 1, 40, 37, 0x762E21, false),
        ("action_east", 1, 48, 42, 0x96523A, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[38, 37, 36, 37, 36, 38, 40]),
        ("action_north", &[16, 12, 14, 12, 14, 16, 22]),
        ("action_south", &[47, 41, 43, 41, 43, 47, 47]),
        ("shocked_end_south", &[58]),
        ("shocked_loop_south", &[70]),
        ("shocked_start_south", &[58]),
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
                // Literal source bounds are independent of mask seeds.
                let in_skin_area = y < 49;
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
