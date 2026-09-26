use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/hayden-beach-actions-study and the local accepted Spring baseline"]
fn hayden_summer_masks_preserve_the_plaid_shirt_hat_and_previous_outputs() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-beach-actions-study");
    let baseline = root.join("generated/characters-world-outfit-special-trial/characters/hayden");
    let set = root.join("palettes/sets/hayden-world-trial.json");
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..186];
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
    // Shared browns on skin must change while neighboring sleeve folds stay original.
    let landmarks = [
        ("idle_east", 0, 43, 44, 0x523C26, false),
        ("idle_east", 0, 37, 44, 0x523C26, false),
        ("idle_east", 0, 34, 40, 0xE7B172, true),
        ("idle_east", 0, 41, 40, 0xE7B172, true),
        ("idle_east", 0, 45, 41, 0x815A2E, true),
        ("idle_east", 0, 40, 29, 0x3F332D, false),
        ("idle_north", 0, 32, 42, 0xE7B172, true),
        ("idle_north", 0, 33, 44, 0xAB7E3F, true),
        ("idle_north", 0, 40, 37, 0xDFC6A1, false),
        ("idle_south", 0, 36, 44, 0x523C26, false),
        ("idle_south", 0, 43, 44, 0x523C26, false),
        ("idle_south", 0, 45, 44, 0xAB7E3F, true),
        ("idle_south", 0, 39, 40, 0xE7B172, true),
        ("idle_south", 0, 40, 42, 0xFFC16A, false),
        ("idle_south", 0, 40, 37, 0x66534A, false),
        ("walk_east", 1, 35, 43, 0x523C26, true),
        ("walk_east", 1, 43, 45, 0x523C26, false),
        ("walk_east", 1, 45, 45, 0x815A2E, true),
        ("walk_east", 1, 33, 44, 0x585654, false),
        ("walk_east", 3, 44, 42, 0x523C26, true),
        ("walk_east", 3, 43, 45, 0x523C26, false),
        ("walk_east", 3, 37, 47, 0x815A2E, true),
        ("walk_north", 1, 46, 45, 0x815A2E, true),
        ("walk_north", 1, 34, 46, 0xAB7E3F, true),
        ("walk_north", 3, 33, 45, 0x815A2E, true),
        ("walk_north", 3, 45, 46, 0xAB7E3F, true),
        ("walk_north", 1, 40, 38, 0xDFC6A1, false),
        ("walk_south", 1, 45, 40, 0x523C26, true),
        ("walk_south", 1, 44, 41, 0x523C26, true),
        ("walk_south", 3, 34, 40, 0x523C26, true),
        ("walk_south", 3, 35, 41, 0x523C26, true),
        ("walk_south", 3, 43, 47, 0x815A2E, true),
        ("walk_south", 3, 40, 42, 0x585654, false),
    ];
    let cases = [
        ("idle_east", 1, 39),
        ("idle_north", 1, 19),
        ("idle_south", 1, 55),
        ("walk_east", 4, 157),
        ("walk_north", 4, 46),
        ("walk_south", 4, 203),
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
                // The exposed hands end at row 47; trousers and boots stay original.
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
