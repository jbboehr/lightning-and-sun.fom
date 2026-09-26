use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/hayden-wedding-pilot-study and the local accepted Beach actions baseline"]
fn hayden_beach_swim_covers_face_and_neck_and_preserves_water() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-wedding-pilot-study");
    let baseline = root.join("generated/characters-world-beach-actions-trial/characters/hayden");
    let set = std::env::var_os("FOM_HAYDEN_BEACH_SWIM_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/hayden-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..288];
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
    // Swimming exposes the face and two small neck patches in the raised frames.
    // Bobbing hides those patches; water, spray, hair and beard stay original.
    let landmarks = [
        ("bath_swim_east", 0, 36, 49, 0xE7B172, true),
        ("bath_swim_east", 0, 40, 49, 0xE7B172, true),
        ("bath_swim_east", 0, 38, 51, 0xAB7E3F, true),
        ("bath_swim_east", 0, 40, 44, 0x9E7C6E, false),
        ("bath_swim_east", 0, 40, 53, 0x66534A, false),
        ("bath_swim_east", 0, 40, 55, 0x9DEBFC, false),
        ("bath_swim_east", 0, 34, 55, 0x328BC9, false),
        ("bath_swim_east", 1, 36, 49, 0xE7B172, true),
        ("bath_swim_east", 1, 40, 49, 0xE7B172, true),
        ("bath_swim_east", 1, 38, 51, 0xAB7E3F, true),
        ("bath_swim_east", 1, 40, 44, 0x9E7C6E, false),
        ("bath_swim_east", 1, 40, 53, 0x66534A, false),
        ("bath_swim_east", 1, 40, 55, 0x9DEBFC, false),
        ("bath_swim_east", 1, 34, 55, 0x328BC9, false),
        ("bath_swim_east", 2, 36, 50, 0xE7B172, true),
        ("bath_swim_east", 2, 40, 50, 0xE7B172, true),
        ("bath_swim_east", 2, 38, 52, 0xAB7E3F, true),
        ("bath_swim_east", 2, 40, 45, 0x9E7C6E, false),
        ("bath_swim_east", 2, 40, 54, 0x66534A, false),
        ("bath_swim_east", 2, 40, 55, 0x9DEBFC, false),
        ("bath_swim_east", 2, 34, 55, 0x328BC9, false),
        ("bath_swim_east", 3, 36, 50, 0xE7B172, true),
        ("bath_swim_east", 3, 40, 50, 0xE7B172, true),
        ("bath_swim_east", 3, 38, 52, 0xAB7E3F, true),
        ("bath_swim_east", 3, 40, 45, 0x9E7C6E, false),
        ("bath_swim_east", 3, 40, 54, 0x66534A, false),
        ("bath_swim_east", 3, 40, 55, 0x9DEBFC, false),
        ("bath_swim_east", 3, 34, 55, 0x328BC9, false),
        ("bath_swim_south", 0, 35, 49, 0xE7B172, true),
        ("bath_swim_south", 0, 39, 49, 0xE7B172, true),
        ("bath_swim_south", 0, 38, 51, 0x3F332D, false),
        ("bath_swim_south", 0, 40, 44, 0x9E7C6E, false),
        ("bath_swim_south", 0, 40, 53, 0x66534A, false),
        ("bath_swim_south", 0, 40, 55, 0x9DEBFC, false),
        ("bath_swim_south", 0, 34, 55, 0x328BC9, false),
        ("bath_swim_south", 1, 35, 49, 0xE7B172, true),
        ("bath_swim_south", 1, 39, 49, 0xE7B172, true),
        ("bath_swim_south", 1, 38, 51, 0x3F332D, false),
        ("bath_swim_south", 1, 40, 44, 0x9E7C6E, false),
        ("bath_swim_south", 1, 40, 53, 0x66534A, false),
        ("bath_swim_south", 1, 40, 55, 0x9DEBFC, false),
        ("bath_swim_south", 1, 34, 55, 0x328BC9, false),
        ("bath_swim_south", 2, 35, 50, 0xE7B172, true),
        ("bath_swim_south", 2, 39, 50, 0xE7B172, true),
        ("bath_swim_south", 2, 38, 52, 0x3F332D, false),
        ("bath_swim_south", 2, 40, 45, 0x9E7C6E, false),
        ("bath_swim_south", 2, 40, 54, 0x66534A, false),
        ("bath_swim_south", 2, 40, 55, 0x9DEBFC, false),
        ("bath_swim_south", 2, 34, 55, 0x328BC9, false),
        ("bath_swim_south", 3, 35, 50, 0xE7B172, true),
        ("bath_swim_south", 3, 39, 50, 0xE7B172, true),
        ("bath_swim_south", 3, 38, 52, 0x3F332D, false),
        ("bath_swim_south", 3, 40, 45, 0x9E7C6E, false),
        ("bath_swim_south", 3, 40, 54, 0x66534A, false),
        ("bath_swim_south", 3, 40, 55, 0x9DEBFC, false),
        ("bath_swim_south", 3, 34, 55, 0x328BC9, false),
        ("bath_swim_east", 0, 36, 54, 0xE7B172, true),
        ("bath_swim_east", 1, 35, 54, 0xAB7E3F, true),
        ("bath_swim_south", 0, 35, 54, 0xE7B172, true),
        ("bath_swim_south", 1, 44, 54, 0xE7B172, true),
    ];
    let cases = [("bath_swim_east", 4, 62), ("bath_swim_south", 4, 76)];
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
                // Inspection established that these two swimming strips use this ramp
                // exclusively for skin. Check unchanged pixels too, so an omitted
                // disconnected neck patch cannot pass as an intentional exclusion.
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
            let expected_per_frame = if case == "bath_swim_east" {
                [17, 17, 14, 14]
            } else {
                [22, 22, 16, 16]
            };
            assert_eq!(
                per_frame, expected_per_frame,
                "bobbing skin coverage: {case}"
            );
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
