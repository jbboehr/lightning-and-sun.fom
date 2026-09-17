use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/hayden-seasonal-expansion-study and the local accepted Summer actions baseline"]
fn hayden_summer_standard_masks_cover_kissing_cheek_and_preserve_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-seasonal-expansion-study");
    let baseline = root.join("generated/characters-world-seasonal-actions-trial/characters/hayden");
    let set = root.join("palettes/sets/hayden-world-trial.json");
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..203];
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
    let source = [0xE7B172, 0xAB7E3F, 0x815A2E, 0x523C26, 0xE8B271];
    // Independently reviewed, literal frame-local boundaries. Frames are zero-based.
    // Shared browns on skin must change while neighboring sleeve folds stay original.
    let landmarks = [
        ("action_east", 0, 39, 44, 0x815A2E, true),
        ("action_east", 0, 40, 47, 0x815A2E, true),
        ("action_east", 1, 41, 41, 0xAB7E3F, true),
        ("action_east", 1, 39, 44, 0x523C26, false),
        ("action_east", 2, 43, 44, 0xAB7E3F, true),
        ("action_east", 2, 39, 44, 0x523C26, false),
        ("action_east", 6, 43, 44, 0x523C26, false),
        ("action_north", 0, 48, 42, 0x815A2E, true),
        ("action_north", 2, 33, 43, 0xE7B172, true),
        ("action_north", 2, 35, 45, 0x815A2E, true),
        ("action_north", 2, 40, 36, 0xDFC6A1, false),
        ("action_south", 1, 36, 43, 0x815A2E, true),
        ("action_south", 1, 43, 44, 0x523C26, false),
        ("action_south", 2, 38, 43, 0x815A2E, true),
        ("action_south", 2, 40, 44, 0xAB7E3F, true),
        ("action_south", 5, 43, 45, 0x523C26, false),
        ("kiss_east", 0, 42, 45, 0x523C26, false),
        ("kiss_east", 0, 43, 45, 0x815A2E, true),
        ("kiss_east", 2, 41, 35, 0xE8B271, true),
        ("kiss_east", 2, 41, 38, 0xE7B172, true),
        ("kiss_east", 2, 39, 44, 0x523C26, false),
        ("kiss_east", 2, 45, 44, 0x523C26, false),
        ("kiss_east", 2, 37, 46, 0x815A2E, true),
        ("sleep_east", 0, 44, 36, 0x815A2E, true),
        ("sleep_east", 0, 44, 38, 0xAB7E3F, true),
        ("sleep_east", 0, 37, 44, 0x523C26, false),
    ];
    let cases = [
        ("action_east", 7, 247),
        ("action_north", 7, 70),
        ("action_south", 7, 341),
        ("kiss_east", 4, 155),
        ("sleep_east", 1, 39),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let mut target: Vec<_> = preset["colors"].as_array().unwrap()[11..15]
            .iter()
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        // The isolated kissing cheek uses the existing portrait highlight.
        target.push(u32::from_str_radix(&preset["colors"][0].as_str().unwrap()[1..7], 16).unwrap());
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for (case, frames, expected_changed) in cases {
            let asset = format!(
                "assets/animations/NPCs/Hayden/Sprites/Summer/spr_npc_hayden_summer_{case}.png"
            );
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
                // The lowered hands end at row 47; trousers and boots below stay original.
                if y >= 48 {
                    assert_eq!(p, q, "lower clothing changed: {id} {case} [{x},{y}]");
                }
                mask.push(p != q);
                if p != q {
                    let index = source
                        .iter()
                        .position(|c| rgba(*c) == p.0)
                        .expect("hat, hair, clothing or another non-skin color changed");
                    assert_eq!(q.0, rgba(target[index]));
                    per_frame[x as usize / 80] += 1;
                }
            }
            assert!(per_frame.iter().all(|n| *n > 0), "empty frame in {case}");
            assert_eq!(per_frame.iter().sum::<usize>(), expected_changed, "{case}");
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
        if let Some(first) = &common_mask {
            assert_eq!(first, &mask);
        } else {
            common_mask = Some(mask);
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
