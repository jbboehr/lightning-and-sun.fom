use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/holt and the local accepted Holt world baseline"]
fn holt_autumn_actions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/holt");
    let baseline = root.join("generated/slice-097-build/characters/holt");
    let set = std::env::var_os("FOM_HOLT_AUTUMN_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/holt-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/holt-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..95];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..95], prior);
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
    let source = [0xF0CC8E, 0xCD9067, 0x96523A, 0xAD6945, 0x762E21];
    // Separate lowered hands from footwear that shares the first three skin shades.
    // Three East blinking hand contours reuse the portrait shadow; hair stays original.
    // Hair, eyes, green shirt, brown apron, pink trim, boots and mouth interiors stay original.
    let landmarks = [
        ("blink_east", 0, 34, 48, 0x000000, false),
        ("blink_east", 0, 45, 42, 0x1C3521, false),
        ("blink_east", 0, 45, 43, 0x335132, false),
        ("blink_east", 0, 41, 42, 0x384926, false),
        ("blink_east", 0, 41, 47, 0x412D23, false),
        ("blink_east", 0, 34, 43, 0x576A51, false),
        ("blink_east", 0, 37, 48, 0x5C683E, false),
        ("blink_east", 0, 42, 50, 0x6B4D3E, false),
        ("blink_east", 0, 39, 30, 0x762E21, false),
        ("blink_east", 0, 45, 45, 0x762E21, true),
        ("blink_east", 0, 41, 52, 0x7D3629, false),
        ("blink_east", 0, 42, 45, 0x8D473C, false),
        ("blink_east", 0, 42, 42, 0x94934C, false),
        ("blink_east", 0, 41, 51, 0x96523A, false),
        ("blink_east", 0, 35, 46, 0x96523A, true),
        ("blink_east", 0, 42, 52, 0x9B594E, false),
        ("blink_east", 0, 37, 45, 0xA57270, false),
        ("blink_east", 0, 43, 35, 0xA59DA2, false),
        ("blink_east", 0, 44, 35, 0xAD6945, true),
        ("blink_east", 0, 38, 32, 0xB65932, false),
        ("blink_east", 0, 39, 41, 0xBCB765, false),
        ("blink_east", 0, 38, 35, 0xC2B9BE, false),
        ("blink_east", 0, 33, 47, 0xCD9067, true),
        ("blink_east", 0, 44, 30, 0xD2793F, false),
        ("blink_east", 0, 37, 44, 0xDB9F9E, false),
        ("blink_east", 0, 43, 36, 0xECF0E9, false),
        ("blink_east", 0, 40, 53, 0xF0CC8E, false),
        ("blink_east", 0, 34, 46, 0xF0CC8E, true),
        ("blink_east", 0, 37, 30, 0xF1A354, false),
        ("blink_east", 0, 41, 44, 0xF7D0C5, false),
        ("blink_east", 1, 39, 30, 0x762E21, false),
        ("blink_east", 1, 45, 45, 0x762E21, true),
        ("blink_east", 1, 35, 46, 0x96523A, true),
        ("blink_east", 1, 44, 35, 0xAD6945, true),
        ("blink_east", 1, 33, 47, 0xCD9067, true),
        ("blink_east", 1, 34, 46, 0xF0CC8E, true),
        ("blink_east", 2, 45, 45, 0x762E21, true),
        ("blink_east", 2, 33, 47, 0xCD9067, true),
        ("blink_south", 0, 33, 47, 0x96523A, true),
        ("blink_south", 1, 39, 30, 0x762E21, false),
        ("blink_south", 1, 33, 47, 0x96523A, true),
        ("blink_south", 1, 34, 46, 0xCD9067, true),
        ("blink_south", 1, 47, 46, 0xF0CC8E, true),
        ("blink_south", 2, 33, 47, 0x96523A, true),
        ("drink_east", 1, 36, 30, 0x762E21, false),
        ("drink_east", 1, 42, 36, 0x96523A, true),
        ("drink_east", 1, 42, 35, 0xAD6945, true),
        ("drink_east", 1, 40, 41, 0xCD9067, true),
        ("drink_east", 1, 40, 42, 0xF0CC8E, true),
        ("drink_north", 0, 46, 45, 0x000000, false),
        ("drink_north", 0, 45, 42, 0x1C3521, false),
        ("drink_north", 0, 45, 43, 0x335132, false),
        ("drink_north", 0, 42, 43, 0x384926, false),
        ("drink_north", 0, 46, 43, 0x576A51, false),
        ("drink_north", 0, 42, 46, 0x5C683E, false),
        ("drink_north", 0, 43, 37, 0x762E21, false),
        ("drink_north", 0, 45, 41, 0x94934C, false),
        ("drink_north", 0, 47, 39, 0x96523A, true),
        ("drink_north", 0, 36, 45, 0xA57270, false),
        ("drink_north", 0, 38, 36, 0xB65932, false),
        ("drink_north", 0, 40, 41, 0xBCB765, false),
        ("drink_north", 0, 31, 45, 0xCD9067, true),
        ("drink_north", 0, 36, 33, 0xD2793F, false),
        ("drink_north", 0, 42, 44, 0xDB9F9E, false),
        ("drink_north", 0, 32, 46, 0xF0CC8E, true),
        ("drink_north", 0, 41, 30, 0xF1A354, false),
        ("drink_north", 0, 40, 45, 0xF7D0C5, false),
        ("drink_north", 1, 40, 40, 0x762E21, false),
        ("drink_north", 1, 31, 45, 0xCD9067, true),
        ("drink_north", 1, 32, 46, 0xF0CC8E, true),
        ("drink_south", 1, 39, 31, 0x762E21, false),
        ("drink_south", 1, 47, 44, 0x96523A, true),
        ("drink_south", 1, 36, 43, 0xCD9067, true),
        ("drink_south", 1, 45, 45, 0xF0CC8E, true),
        ("eat_east", 0, 45, 45, 0x000000, false),
        ("eat_east", 0, 38, 43, 0x1C3521, false),
        ("eat_east", 0, 42, 42, 0x335132, false),
        ("eat_east", 0, 37, 47, 0x384926, false),
        ("eat_east", 0, 41, 47, 0x412D23, false),
        ("eat_east", 0, 40, 43, 0x576A51, false),
        ("eat_east", 0, 39, 45, 0x5C683E, false),
        ("eat_east", 0, 44, 46, 0x6B4D3E, false),
        ("eat_east", 0, 39, 30, 0x762E21, false),
        ("eat_east", 0, 41, 49, 0x7D3629, false),
        ("eat_east", 0, 38, 46, 0x94934C, false),
        ("eat_east", 0, 41, 48, 0x96523A, false),
        ("eat_east", 0, 45, 42, 0x96523A, true),
        ("eat_east", 0, 42, 49, 0x9B594E, false),
        ("eat_east", 0, 37, 45, 0xA57270, false),
        ("eat_east", 0, 43, 34, 0xA59DA2, false),
        ("eat_east", 0, 44, 35, 0xAD6945, true),
        ("eat_east", 0, 38, 32, 0xB65932, false),
        ("eat_east", 0, 37, 41, 0xBCB765, false),
        ("eat_east", 0, 39, 34, 0xC2B9BE, false),
        ("eat_east", 0, 45, 44, 0xCD9067, true),
        ("eat_east", 0, 44, 30, 0xD2793F, false),
        ("eat_east", 0, 37, 44, 0xDB9F9E, false),
        ("eat_east", 0, 38, 36, 0xECF0E9, false),
        ("eat_east", 0, 45, 49, 0xF0CC8E, false),
        ("eat_east", 0, 45, 43, 0xF0CC8E, true),
        ("eat_east", 0, 37, 30, 0xF1A354, false),
        ("eat_east", 1, 40, 30, 0x762E21, false),
        ("eat_east", 1, 46, 41, 0x96523A, true),
        ("eat_east", 1, 45, 35, 0xAD6945, true),
        ("eat_east", 1, 47, 40, 0xCD9067, true),
        ("eat_east", 1, 46, 40, 0xF0CC8E, true),
        ("eat_north", 1, 40, 40, 0x762E21, false),
        ("eat_north", 1, 31, 45, 0xCD9067, true),
        ("eat_north", 1, 32, 46, 0xF0CC8E, true),
        ("eat_south", 0, 36, 48, 0x96523A, true),
        ("eat_south", 1, 39, 31, 0x762E21, false),
        ("eat_south", 1, 37, 49, 0x96523A, true),
        ("eat_south", 1, 46, 46, 0xCD9067, true),
        ("eat_south", 1, 35, 47, 0xF0CC8E, true),
        ("sit_north", 0, 49, 46, 0x000000, false),
        ("sit_north", 0, 32, 43, 0x1C3521, false),
        ("sit_north", 0, 34, 43, 0x335132, false),
        ("sit_north", 0, 42, 43, 0x384926, false),
        ("sit_north", 0, 46, 43, 0x576A51, false),
        ("sit_north", 0, 42, 46, 0x5C683E, false),
        ("sit_north", 0, 43, 37, 0x762E21, false),
        ("sit_north", 0, 45, 41, 0x94934C, false),
        ("sit_north", 0, 36, 45, 0xA57270, false),
        ("sit_north", 0, 38, 36, 0xB65932, false),
        ("sit_north", 0, 40, 41, 0xBCB765, false),
        ("sit_north", 0, 48, 45, 0xCD9067, true),
        ("sit_north", 0, 36, 33, 0xD2793F, false),
        ("sit_north", 0, 42, 44, 0xDB9F9E, false),
        ("sit_north", 0, 33, 46, 0xF0CC8E, true),
        ("sit_north", 0, 41, 30, 0xF1A354, false),
        ("sit_north", 0, 40, 45, 0xF7D0C5, false),
        ("sit_south", 0, 44, 47, 0x000000, false),
        ("sit_south", 0, 32, 43, 0x1C3521, false),
        ("sit_south", 0, 34, 43, 0x335132, false),
        ("sit_south", 0, 43, 42, 0x384926, false),
        ("sit_south", 0, 40, 46, 0x412D23, false),
        ("sit_south", 0, 46, 43, 0x576A51, false),
        ("sit_south", 0, 37, 46, 0x5C683E, false),
        ("sit_south", 0, 41, 47, 0x6B4D3E, false),
        ("sit_south", 0, 39, 30, 0x762E21, false),
        ("sit_south", 0, 42, 49, 0x7D3629, false),
        ("sit_south", 0, 41, 44, 0x8D473C, false),
        ("sit_south", 0, 40, 41, 0x94934C, false),
        ("sit_south", 0, 47, 44, 0x96523A, true),
        ("sit_south", 0, 41, 49, 0x9B594E, false),
        ("sit_south", 0, 36, 45, 0xA57270, false),
        ("sit_south", 0, 37, 32, 0xB65932, false),
        ("sit_south", 0, 38, 41, 0xBCB765, false),
        ("sit_south", 0, 41, 34, 0xC2B9BE, false),
        ("sit_south", 0, 42, 48, 0xCD9067, false),
        ("sit_south", 0, 46, 46, 0xCD9067, true),
        ("sit_south", 0, 43, 30, 0xD2793F, false),
        ("sit_south", 0, 36, 44, 0xDB9F9E, false),
        ("sit_south", 0, 37, 36, 0xECF0E9, false),
        ("sit_south", 0, 38, 50, 0xF0CC8E, false),
        ("sit_south", 0, 31, 46, 0xF0CC8E, true),
        ("sit_south", 0, 36, 30, 0xF1A354, false),
        ("sit_south", 0, 42, 44, 0xF7D0C5, false),
        ("blink_east", 1, 39, 35, 0xF0CC8E, true),
        ("eat_south", 0, 36, 48, 0x96523A, true),
        ("eat_south", 1, 37, 49, 0x96523A, true),
        ("eat_south", 1, 38, 50, 0xF0CC8E, false),
        ("sit_east", 0, 41, 48, 0x96523A, false),
        ("eat_east", 1, 41, 38, 0x9E2626, false),
        ("eat_east", 1, 38, 37, 0x762E21, false),
        ("sit_north", 0, 32, 44, 0xCD9067, true),
        ("eat_south", 1, 34, 46, 0x335132, false),
        ("blink_east", 0, 45, 45, 0x762E21, true),
        ("blink_east", 1, 45, 45, 0x762E21, true),
        ("blink_east", 2, 45, 45, 0x762E21, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[41, 49, 41]),
        ("blink_south", &[48, 56, 48]),
        ("drink_east", &[35, 45, 35]),
        ("drink_north", &[18, 17, 18]),
        ("drink_south", &[45, 57, 45]),
        ("eat_east", &[34, 39, 44, 42, 34]),
        ("eat_north", &[18, 17, 18]),
        ("eat_south", &[46, 51, 55, 60, 47]),
        ("sit_east", &[36]),
        ("sit_north", &[26]),
        ("sit_south", &[47]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [9, 1, 10, 11, 3]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_holt_autumn";
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
                let local_x = x % 80;
                let in_skin_area = if p.0 == rgba(0x762E21) {
                    case == "blink_east" && local_x == 45 && y == 45
                } else if case.starts_with("blink_") {
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
