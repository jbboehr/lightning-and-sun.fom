use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/hayden-seasonal-special-study and the local outfit pilot baseline"]
fn hayden_alternate_riding_masks_preserve_horse_animation_and_shirt_edges() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-seasonal-special-study");
    let baseline = root.join("generated/characters-world-outfit-pilots-trial/characters/hayden");
    let set = root.join("palettes/sets/hayden-world-trial.json");
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..174];
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
        ("ride_blink_east", 1, 36, 29, 0x815A2E, true),
        ("ride_blink_east", 1, 33, 29, 0xAB7E3F, false),
        ("ride_blink_east", 1, 41, 34, 0x815A2E, true),
        ("ride_blink_east", 1, 50, 28, 0x825F62, false),
        ("ride_blink_south", 1, 36, 28, 0x523C26, true),
        ("ride_blink_south", 1, 37, 27, 0xAB7E3F, false),
        ("ride_blink_south", 1, 37, 29, 0xE7B172, true),
        ("ride_blink_south", 1, 39, 32, 0xF7E7DF, false),
        ("ride_blink_south", 1, 36, 37, 0x825F62, false),
        ("ride_idle_2_east", 8, 34, 30, 0x523C26, true),
        ("ride_idle_2_east", 8, 34, 27, 0xAB7E3F, false),
        ("ride_idle_2_east", 8, 39, 29, 0xE7B172, true),
        ("ride_idle_2_east", 8, 48, 31, 0x967C7C, false),
        ("ride_idle_2_north", 7, 35, 26, 0x523C26, true),
        ("ride_idle_2_north", 7, 36, 26, 0xAB7E3F, false),
        ("ride_idle_2_north", 7, 36, 27, 0x815A2E, false),
        ("ride_idle_2_north", 7, 34, 27, 0xE7B172, true),
        ("ride_idle_2_north", 7, 39, 33, 0xE7BA83, false),
        ("ride_idle_2_north", 7, 41, 48, 0xF7E7DF, false),
        ("ride_idle_2_south", 0, 35, 29, 0x523C26, true),
        ("ride_idle_2_south", 0, 36, 29, 0xE7B172, true),
        ("ride_idle_2_south", 0, 37, 27, 0xAB7E3F, false),
        ("ride_idle_2_south", 0, 35, 23, 0x815A2E, false),
        ("ride_idle_2_south", 1, 34, 30, 0x815A2E, true),
        ("ride_idle_2_south", 1, 36, 29, 0x000000, false),
        ("ride_idle_2_south", 1, 37, 29, 0xE7B172, true),
        ("ride_idle_3_east", 6, 39, 32, 0x523C26, true),
        ("ride_idle_3_east", 6, 33, 33, 0xAB7E3F, false),
        ("ride_idle_3_east", 6, 41, 34, 0x815A2E, true),
        ("ride_idle_3_east", 6, 54, 32, 0x958676, false),
        ("ride_idle_3_east", 6, 50, 27, 0xC4A998, false),
    ];
    let cases = [
        ("ride_blink_east", 3, 123),
        ("ride_blink_south", 3, 138),
        ("ride_idle_2_east", 9, 369),
        ("ride_idle_2_north", 8, 96),
        ("ride_idle_2_south", 4, 186),
        ("ride_idle_3_east", 8, 328),
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
                "assets/animations/NPCs/Hayden/Sprites/Spring/spr_npc_hayden_specialanimation_spring_{case}.png"
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
                // The rider's lowest exposed skin ends at row 34. The complete
                // lower horse, tail, saddle, boots and East horse head must remain byte-identical.
                if y >= 35 || (case.ends_with("_east") && x % 80 >= 43) {
                    assert_eq!(p, q, "horse or gear changed: {id} {case} [{x},{y}]");
                }
                mask.push(p != q);
                if p != q {
                    let index = source
                        .iter()
                        .position(|c| rgba(*c) == p.0)
                        .expect("horse, gear, hair, clothing or another non-skin color changed");
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
