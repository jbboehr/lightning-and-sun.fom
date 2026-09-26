use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/hayden-wedding-finish-study and the local accepted Winter finish baseline"]
fn hayden_beach_masks_cover_exposed_skin_and_preserve_hat_and_swimwear() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-wedding-finish-study");
    let baseline = root.join("generated/characters-world-winter-finish-trial/characters/hayden");
    let set = std::env::var_os("FOM_HAYDEN_BEACH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/hayden-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..276];
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
    // Exposed torso, forearms and bare feet change, including the darkest forearm
    // detail already accepted on Spring sprites. The straw hat, its edging,
    // swimsuit, drawstrings, head hair and beard remain their original materials.
    let landmarks = [
        ("idle_east", 0, 39, 33, 0x000000, false),
        ("idle_east", 0, 37, 38, 0xE7B172, true),
        ("idle_east", 0, 40, 40, 0xE7B172, true),
        ("idle_east", 0, 36, 42, 0x000000, false),
        ("idle_east", 0, 34, 44, 0x523C26, true),
        ("idle_east", 0, 34, 46, 0xE7B172, true),
        ("idle_east", 0, 38, 49, 0x815A2E, true),
        ("idle_east", 0, 39, 52, 0xAB7E3F, true),
        ("idle_east", 0, 32, 40, 0xDFC6A1, false),
        ("idle_east", 0, 43, 38, 0x000000, false),
        ("idle_east", 0, 39, 44, 0xFFDC87, false),
        ("idle_east", 0, 40, 45, 0xEA9829, false),
        ("idle_north", 0, 32, 42, 0xE7B172, true),
        ("idle_north", 0, 46, 44, 0x523C26, true),
        ("idle_north", 0, 34, 46, 0x815A2E, true),
        ("idle_north", 0, 37, 49, 0xAB7E3F, true),
        ("idle_north", 0, 42, 53, 0xAB7E3F, true),
        ("idle_north", 0, 39, 38, 0xDFC6A1, false),
        ("idle_north", 0, 33, 37, 0xB68D54, false),
        ("idle_north", 0, 36, 38, 0x84533E, false),
        ("idle_north", 0, 38, 46, 0x84533E, false),
        ("idle_north", 0, 40, 28, 0x9E7C6E, false),
        ("idle_south", 0, 38, 31, 0xAB7E3F, true),
        ("idle_south", 0, 35, 33, 0xE7B172, true),
        ("idle_south", 0, 40, 28, 0x9E7C6E, false),
        ("idle_south", 0, 39, 36, 0x3F332D, false),
        ("idle_south", 0, 39, 40, 0xE7B172, true),
        ("idle_south", 0, 36, 42, 0x815A2E, true),
        ("idle_south", 0, 42, 42, 0x815A2E, true),
        ("idle_south", 0, 32, 44, 0x523C26, true),
        ("idle_south", 0, 45, 44, 0xAB7E3F, true),
        ("idle_south", 0, 38, 49, 0xE7B172, true),
        ("idle_south", 0, 42, 52, 0x815A2E, true),
        ("idle_south", 0, 41, 53, 0xE7B172, true),
        ("idle_south", 0, 33, 37, 0xDFC6A1, false),
        ("idle_south", 0, 38, 44, 0xFFDC87, false),
        ("idle_south", 0, 39, 45, 0xEA9829, false),
        ("walk_east", 1, 32, 44, 0x523C26, true),
        ("walk_east", 1, 39, 49, 0xAB7E3F, true),
        ("walk_east", 1, 43, 52, 0xE7B172, true),
        ("walk_east", 1, 34, 47, 0x000000, false),
        ("walk_east", 3, 39, 41, 0xE7B172, true),
        ("walk_east", 3, 35, 45, 0x523C26, true),
        ("walk_east", 3, 37, 51, 0xE7B172, true),
        ("walk_east", 3, 44, 52, 0xE7B172, true),
        ("walk_east", 3, 33, 45, 0xB68D54, false),
        ("walk_north", 1, 34, 46, 0xAB7E3F, true),
        ("walk_north", 1, 46, 45, 0x815A2E, true),
        ("walk_north", 1, 41, 54, 0xE7B172, true),
        ("walk_north", 1, 37, 45, 0xDFC6A1, false),
        ("walk_north", 3, 33, 45, 0x815A2E, true),
        ("walk_north", 3, 45, 47, 0xAB7E3F, true),
        ("walk_north", 3, 38, 54, 0xE7B172, true),
        ("walk_north", 3, 41, 40, 0xDFC6A1, false),
        ("walk_south", 1, 39, 41, 0xE7B172, true),
        ("walk_south", 1, 34, 44, 0x523C26, true),
        ("walk_south", 1, 46, 45, 0x815A2E, true),
        ("walk_south", 1, 38, 51, 0x815A2E, true),
        ("walk_south", 1, 41, 54, 0xE7B172, true),
        ("walk_south", 1, 38, 45, 0xFFDC87, false),
        ("walk_south", 3, 33, 43, 0x523C26, true),
        ("walk_south", 3, 45, 45, 0xE7B172, true),
        ("walk_south", 3, 42, 51, 0x815A2E, true),
        ("walk_south", 3, 38, 54, 0xE7B172, true),
        ("walk_south", 3, 42, 45, 0x000000, false),
    ];
    let cases = [
        ("idle_east", 1, 104),
        ("idle_north", 1, 40),
        ("idle_south", 1, 122),
        ("walk_east", 4, 397),
        ("walk_north", 4, 124),
        ("walk_south", 4, 452),
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
