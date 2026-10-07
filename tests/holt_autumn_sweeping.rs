use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/holt and the local accepted Holt world baseline"]
fn holt_autumn_sweeping_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/holt");
    let baseline = root.join("generated/slice-101-build/characters/holt");
    let set = std::env::var_os("FOM_HOLT_AUTUMN_SWEEPING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/holt-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/holt-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..113];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..113], prior);
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
    // The four skin shades are above row 51; footwear reuses them below.
    // Autumn's gripping hands use these shades, with green sleeves alongside them.
    // The dark hair/moustache shade is absent from these hands. Hair, eyes, shirt,
    // apron, pink trim, boots, broom and dust remain original.
    let landmarks = [
        ("sweep_end_south", 0, 31, 47, 0x000000, false),
        ("sweep_end_south", 0, 32, 45, 0x1C3521, false),
        ("sweep_end_south", 0, 34, 44, 0x335132, false),
        ("sweep_end_south", 0, 37, 44, 0x384926, false),
        ("sweep_end_south", 0, 39, 48, 0x412D23, false),
        ("sweep_end_south", 0, 46, 44, 0x576A51, false),
        ("sweep_end_south", 0, 43, 48, 0x5C683E, false),
        ("sweep_end_south", 0, 39, 31, 0x762E21, false),
        ("sweep_end_south", 0, 42, 52, 0x7D3629, false),
        ("sweep_end_south", 0, 41, 46, 0x8D473C, false),
        ("sweep_end_south", 0, 38, 43, 0x94934C, false),
        ("sweep_end_south", 0, 34, 46, 0x96523A, true),
        ("sweep_end_south", 0, 41, 52, 0x9B594E, false),
        ("sweep_end_south", 0, 36, 46, 0xA57270, false),
        ("sweep_end_south", 0, 41, 33, 0xB65932, false),
        ("sweep_end_south", 0, 34, 42, 0xBCB765, false),
        ("sweep_end_south", 0, 42, 36, 0xC2B9BE, false),
        ("sweep_end_south", 0, 42, 51, 0xCD9067, false),
        ("sweep_end_south", 0, 45, 45, 0xCD9067, true),
        ("sweep_end_south", 0, 43, 31, 0xD2793F, false),
        ("sweep_end_south", 0, 36, 45, 0xDB9F9E, false),
        ("sweep_end_south", 0, 42, 37, 0xECF0E9, false),
        ("sweep_end_south", 0, 38, 53, 0xF0CC8E, false),
        ("sweep_end_south", 0, 44, 45, 0xF0CC8E, true),
        ("sweep_end_south", 0, 36, 31, 0xF1A354, false),
        ("sweep_end_south", 0, 40, 45, 0xF7D0C5, false),
        ("sweep_loop_south", 0, 44, 48, 0x000000, false),
        ("sweep_loop_south", 0, 41, 44, 0x1C3521, false),
        ("sweep_loop_south", 0, 42, 44, 0x335132, false),
        ("sweep_loop_south", 0, 36, 42, 0x384926, false),
        ("sweep_loop_south", 0, 37, 50, 0x412D23, false),
        ("sweep_loop_south", 0, 43, 43, 0x576A51, false),
        ("sweep_loop_south", 0, 37, 49, 0x5C683E, false),
        ("sweep_loop_south", 0, 46, 51, 0x653D25, false),
        ("sweep_loop_south", 0, 38, 50, 0x6B4D3E, false),
        ("sweep_loop_south", 0, 39, 30, 0x762E21, false),
        ("sweep_loop_south", 0, 37, 52, 0x7D3629, false),
        ("sweep_loop_south", 0, 41, 51, 0x936244, false),
        ("sweep_loop_south", 0, 43, 41, 0x94934C, false),
        ("sweep_loop_south", 0, 38, 45, 0x96523A, true),
        ("sweep_loop_south", 0, 38, 52, 0x9B594E, false),
        ("sweep_loop_south", 0, 35, 33, 0xB65932, false),
        ("sweep_loop_south", 0, 45, 50, 0xBB8151, false),
        ("sweep_loop_south", 0, 34, 41, 0xBCB765, false),
        ("sweep_loop_south", 0, 37, 51, 0xCD9067, false),
        ("sweep_loop_south", 0, 40, 44, 0xCD9067, true),
        ("sweep_loop_south", 0, 43, 30, 0xD2793F, false),
        ("sweep_loop_south", 0, 48, 53, 0xE9B03A, false),
        ("sweep_loop_south", 0, 37, 53, 0xF0CC8E, false),
        ("sweep_loop_south", 0, 39, 43, 0xF0CC8E, true),
        ("sweep_loop_south", 0, 36, 30, 0xF1A354, false),
        ("sweep_loop_south", 0, 41, 55, 0xFFE36F, false),
        ("sweep_loop_south", 1, 39, 31, 0x762E21, false),
        ("sweep_loop_south", 1, 39, 46, 0x96523A, true),
        ("sweep_loop_south", 1, 38, 45, 0xCD9067, true),
        ("sweep_loop_south", 1, 40, 44, 0xF0CC8E, true),
        ("sweep_loop_south", 2, 39, 31, 0x762E21, false),
        ("sweep_loop_south", 2, 39, 46, 0x96523A, true),
        ("sweep_loop_south", 2, 38, 45, 0xCD9067, true),
        ("sweep_loop_south", 2, 40, 44, 0xF0CC8E, true),
        ("sweep_loop_south", 3, 39, 30, 0x762E21, false),
        ("sweep_loop_south", 3, 38, 45, 0x96523A, true),
        ("sweep_loop_south", 3, 40, 44, 0xCD9067, true),
        ("sweep_loop_south", 3, 39, 43, 0xF0CC8E, true),
        ("sweep_loop_south", 4, 39, 30, 0x762E21, false),
        ("sweep_loop_south", 4, 38, 44, 0x96523A, true),
        ("sweep_loop_south", 4, 41, 44, 0xCD9067, true),
        ("sweep_loop_south", 4, 40, 43, 0xF0CC8E, true),
        ("sweep_loop_south", 5, 39, 30, 0x762E21, false),
        ("sweep_loop_south", 5, 38, 44, 0x96523A, true),
        ("sweep_loop_south", 5, 41, 44, 0xCD9067, true),
        ("sweep_loop_south", 5, 40, 43, 0xF0CC8E, true),
        ("sweep_loop_south", 6, 39, 31, 0x762E21, false),
        ("sweep_loop_south", 6, 40, 45, 0x96523A, true),
        ("sweep_loop_south", 6, 39, 45, 0xCD9067, true),
        ("sweep_loop_south", 6, 42, 44, 0xF0CC8E, true),
        ("sweep_loop_south", 7, 39, 31, 0x762E21, false),
        ("sweep_loop_south", 7, 40, 45, 0x96523A, true),
        ("sweep_loop_south", 7, 39, 45, 0xCD9067, true),
        ("sweep_loop_south", 7, 42, 44, 0xF0CC8E, true),
        ("sweep_loop_south", 8, 39, 30, 0x762E21, false),
        ("sweep_loop_south", 8, 38, 44, 0x96523A, true),
        ("sweep_loop_south", 8, 41, 44, 0xCD9067, true),
        ("sweep_loop_south", 8, 40, 43, 0xF0CC8E, true),
        ("sweep_loop_south", 9, 39, 30, 0x762E21, false),
        ("sweep_loop_south", 9, 38, 44, 0x96523A, true),
        ("sweep_loop_south", 9, 41, 44, 0xCD9067, true),
        ("sweep_loop_south", 9, 40, 43, 0xF0CC8E, true),
        ("sweep_start_south", 0, 44, 48, 0x000000, false),
        ("sweep_start_south", 0, 32, 45, 0x1C3521, false),
        ("sweep_start_south", 0, 34, 44, 0x335132, false),
        ("sweep_start_south", 0, 37, 44, 0x384926, false),
        ("sweep_start_south", 0, 39, 48, 0x412D23, false),
        ("sweep_start_south", 0, 46, 44, 0x576A51, false),
        ("sweep_start_south", 0, 43, 48, 0x5C683E, false),
        ("sweep_start_south", 0, 49, 50, 0x653D25, false),
        ("sweep_start_south", 0, 39, 31, 0x762E21, false),
        ("sweep_start_south", 0, 42, 52, 0x7D3629, false),
        ("sweep_start_south", 0, 41, 46, 0x8D473C, false),
        ("sweep_start_south", 0, 45, 50, 0x936244, false),
        ("sweep_start_south", 0, 41, 43, 0x94934C, false),
        ("sweep_start_south", 0, 34, 46, 0x96523A, true),
        ("sweep_start_south", 0, 41, 52, 0x9B594E, false),
        ("sweep_start_south", 0, 36, 46, 0xA57270, false),
        ("sweep_start_south", 0, 41, 33, 0xB65932, false),
        ("sweep_start_south", 0, 46, 50, 0xBB8151, false),
        ("sweep_start_south", 0, 34, 42, 0xBCB765, false),
        ("sweep_start_south", 0, 42, 36, 0xC2B9BE, false),
        ("sweep_start_south", 0, 42, 51, 0xCD9067, false),
        ("sweep_start_south", 0, 45, 45, 0xCD9067, true),
        ("sweep_start_south", 0, 41, 31, 0xD2793F, false),
        ("sweep_start_south", 0, 36, 45, 0xDB9F9E, false),
        ("sweep_start_south", 0, 51, 52, 0xE9B03A, false),
        ("sweep_start_south", 0, 42, 37, 0xECF0E9, false),
        ("sweep_start_south", 0, 38, 53, 0xF0CC8E, false),
        ("sweep_start_south", 0, 44, 45, 0xF0CC8E, true),
        ("sweep_start_south", 0, 36, 31, 0xF1A354, false),
        ("sweep_start_south", 0, 40, 45, 0xF7D0C5, false),
        ("sweep_start_south", 0, 44, 54, 0xFFE36F, false),
        ("sweep_end_south", 0, 33, 48, 0x96523A, true),
        ("sweep_end_south", 0, 37, 51, 0xCD9067, false),
        ("sweep_loop_south", 0, 37, 44, 0x1C3521, false),
        ("sweep_loop_south", 0, 40, 40, 0x5C683E, false),
        ("sweep_loop_south", 0, 36, 37, 0x762E21, false),
        ("sweep_loop_south", 1, 38, 47, 0xCD9067, true),
        ("sweep_loop_south", 1, 55, 53, 0xA0A0A0, false),
        ("sweep_loop_south", 5, 40, 44, 0xF0CC8E, true),
        ("sweep_loop_south", 6, 39, 47, 0xCD9067, true),
        ("sweep_start_south", 0, 34, 48, 0xF0CC8E, true),
        ("sweep_loop_south", 0, 40, 42, 0x96523A, true),
        ("sweep_loop_south", 0, 42, 42, 0x1C3521, false),
        ("sweep_loop_south", 1, 40, 43, 0xF0CC8E, true),
        ("sweep_loop_south", 1, 42, 43, 0x1C3521, false),
        ("sweep_loop_south", 2, 40, 43, 0xF0CC8E, true),
        ("sweep_loop_south", 2, 42, 43, 0x1C3521, false),
        ("sweep_loop_south", 3, 40, 42, 0x96523A, true),
        ("sweep_loop_south", 3, 42, 42, 0x1C3521, false),
        ("sweep_loop_south", 4, 41, 42, 0x96523A, true),
        ("sweep_loop_south", 4, 43, 42, 0x384926, false),
        ("sweep_loop_south", 5, 41, 42, 0x96523A, true),
        ("sweep_loop_south", 5, 43, 42, 0x384926, false),
        ("sweep_loop_south", 6, 43, 43, 0x96523A, true),
        ("sweep_loop_south", 6, 45, 43, 0x1C3521, false),
        ("sweep_loop_south", 7, 43, 43, 0x96523A, true),
        ("sweep_loop_south", 7, 45, 43, 0x1C3521, false),
        ("sweep_loop_south", 8, 41, 42, 0x96523A, true),
        ("sweep_loop_south", 8, 43, 42, 0x384926, false),
        ("sweep_loop_south", 9, 41, 42, 0x96523A, true),
        ("sweep_loop_south", 9, 43, 42, 0x384926, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("sweep_end_south", &[48]),
        (
            "sweep_loop_south",
            &[48, 56, 52, 48, 49, 49, 56, 52, 49, 49],
        ),
        ("sweep_start_south", &[43]),
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
            let prefix = "spr_npc_holt_specialanimation_autumn";
            let asset = format!("assets/animations/NPCs/Holt/Sprites/Autumn/{prefix}_{case}.png");
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
                let in_skin_area = y < 51;
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && in_skin_area)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Holt Autumn material mismatch: {id} {case} [{x},{y}]"
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
