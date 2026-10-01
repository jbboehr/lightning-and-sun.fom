use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/hayden and the local accepted Beach pilot baseline"]
fn hayden_beach_actions_cover_moving_skin_and_preserve_hat_and_swimwear() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/hayden");
    let baseline = root.join("generated/characters-world-beach-pilot-trial/characters/hayden");
    let set = std::env::var_os("FOM_HAYDEN_BEACH_ACTION_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/hayden-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..282];
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
    let source = [0xE7B172, 0xAB7E3F, 0x815A2E, 0x523C26];
    // Independently reviewed, literal frame-local boundaries. Frames are zero-based.
    // Exposed torso, moving fingers, kissing cheek and bare feet change, including
    // accepted darkest forearm detail. The straw hat, its edging,
    // swimsuit, drawstrings, head hair and beard remain their original materials.
    let landmarks = [
        ("action_east", 0, 38, 43, 0x815A2E, true),
        ("action_east", 0, 37, 40, 0xE7B172, true),
        ("action_east", 1, 47, 43, 0x815A2E, true),
        ("action_east", 1, 43, 40, 0xE7B172, true),
        ("action_east", 2, 45, 44, 0x815A2E, true),
        ("action_east", 2, 40, 38, 0xE7B172, true),
        ("action_north", 0, 32, 45, 0x815A2E, true),
        ("action_north", 0, 33, 39, 0xDFC6A1, false),
        ("action_north", 2, 35, 45, 0x815A2E, true),
        ("action_north", 2, 43, 38, 0x84533E, false),
        ("action_north", 2, 39, 26, 0x66534A, false),
        ("action_south", 0, 35, 47, 0x815A2E, true),
        ("action_south", 0, 34, 40, 0xE7B172, true),
        ("action_south", 1, 36, 43, 0x815A2E, true),
        ("action_south", 1, 36, 38, 0xE7B172, true),
        ("action_south", 2, 38, 43, 0x815A2E, true),
        ("action_south", 2, 44, 38, 0x815A2E, true),
        ("kiss_east", 0, 43, 45, 0x815A2E, true),
        ("kiss_east", 0, 43, 41, 0x815A2E, true),
        ("kiss_east", 2, 41, 35, 0xAB7E3F, true),
        ("kiss_east", 2, 39, 38, 0xAB7E3F, true),
        ("kiss_east", 2, 38, 39, 0xE7B172, true),
        ("kiss_east", 2, 37, 46, 0x815A2E, true),
        ("kiss_east", 3, 37, 47, 0x815A2E, true),
        ("action_north", 0, 47, 38, 0x000000, false),
        ("action_north", 0, 48, 39, 0xAB7E3F, true),
        ("action_north", 0, 48, 42, 0x815A2E, true),
        ("action_north", 2, 47, 36, 0x000000, false),
        ("action_south", 2, 36, 41, 0xE7B172, true),
        ("action_south", 2, 37, 42, 0x815A2E, true),
        ("action_south", 2, 36, 45, 0xAB7E3F, true),
        ("action_south", 2, 39, 44, 0x523C26, true),
        ("action_south", 2, 39, 40, 0xE7B172, true),
        ("kiss_east", 2, 38, 42, 0xAB7E3F, true),
        ("kiss_east", 2, 44, 40, 0xE7B172, true),
        ("action_east", 0, 39, 45, 0x523C26, true),
        ("action_east", 0, 42, 41, 0xE7B172, true),
        ("blink_east", 0, 40, 33, 0xE7B172, true),
        ("blink_east", 1, 40, 34, 0xE7B172, true),
        ("blink_east", 1, 34, 44, 0x523C26, true),
        ("blink_east", 2, 39, 52, 0xAB7E3F, true),
        ("blink_east", 1, 32, 40, 0xDFC6A1, false),
        ("blink_east", 1, 39, 44, 0xFFDC87, false),
        ("blink_south", 0, 38, 31, 0xAB7E3F, true),
        ("blink_south", 1, 40, 34, 0xE7B172, true),
        ("blink_south", 1, 36, 42, 0x815A2E, true),
        ("blink_south", 2, 41, 53, 0xE7B172, true),
        ("blink_south", 1, 33, 37, 0xDFC6A1, false),
        ("blink_south", 1, 39, 45, 0xEA9829, false),
        ("action_east", 3, 38, 50, 0xAB7E3F, true),
        ("action_east", 4, 38, 50, 0xAB7E3F, true),
        ("action_east", 5, 40, 28, 0x3F332D, false),
        ("action_east", 6, 38, 49, 0x815A2E, true),
        ("action_north", 0, 39, 38, 0xDFC6A1, false),
        ("action_north", 1, 38, 50, 0xE7B172, true),
        ("action_north", 3, 38, 50, 0xE7B172, true),
        ("action_north", 4, 38, 50, 0xE7B172, true),
        ("action_north", 5, 38, 50, 0xE7B172, true),
        ("action_north", 6, 38, 50, 0xE7B172, true),
        ("action_south", 3, 38, 50, 0xE7B172, true),
        ("action_south", 4, 38, 50, 0xE7B172, true),
        ("action_south", 5, 38, 50, 0xE7B172, true),
        ("action_south", 6, 38, 50, 0xE7B172, true),
        ("kiss_east", 1, 39, 50, 0xE7B172, true),
        ("kiss_east", 2, 39, 50, 0x000000, false),
        ("kiss_east", 3, 38, 49, 0x9E4A26, false),
    ];
    let cases = [
        ("action_east", 7, 556),
        ("action_north", 7, 230),
        ("action_south", 7, 764),
        ("blink_east", 3, 328),
        ("blink_south", 3, 382),
        ("kiss_east", 4, 363),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = preset["colors"].as_array().unwrap()[11..15]
            .iter()
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for (case, frames, expected_changed) in cases {
            let asset = format!(
                "assets/animations/NPCs/Hayden/Sprites/Beach/spr_npc_hayden_beach_{case}.png"
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
                // Inspection established that these six Beach strips use this ramp
                // exclusively for skin. Check unchanged pixels too, so an omitted
                // disconnected leg or finger cannot pass as an intentional exclusion.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Beach material mismatch: {id} {case} [{x},{y}]"
                );
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
