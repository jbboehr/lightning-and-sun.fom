use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/hayden and the local outfit actions baseline"]
fn hayden_run_jump_masks_follow_rider_motion_and_preserve_horse_gear() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/hayden");
    let baseline = root.join("generated/characters-world-outfit-actions-trial/characters/hayden");
    let set = root.join("palettes/sets/hayden-world-trial.json");
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..180];
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
        ("ride_jump_east", 3, 41, 35, 0x815A2E, true),
        ("ride_jump_east", 3, 33, 34, 0xAB7E3F, false),
        ("ride_jump_east", 3, 39, 35, 0xAB7E3F, true),
        ("ride_jump_east", 3, 47, 30, 0x967C7C, false),
        ("ride_jump_north", 1, 35, 22, 0x523C26, true),
        ("ride_jump_north", 1, 36, 22, 0xAB7E3F, false),
        ("ride_jump_north", 1, 34, 23, 0xE7B172, true),
        ("ride_jump_north", 1, 39, 29, 0xBB8151, false),
        ("ride_jump_south", 0, 37, 13, 0x815A2E, true),
        ("ride_jump_south", 0, 35, 20, 0x815A2E, false),
        ("ride_jump_south", 0, 35, 23, 0x523C26, true),
        ("ride_jump_south", 1, 35, 25, 0x523C26, true),
        ("ride_jump_south", 1, 37, 23, 0xAB7E3F, false),
        ("ride_jump_south", 1, 37, 26, 0xE7B172, true),
        ("ride_jump_south", 1, 44, 26, 0xAB7E3F, true),
        ("ride_jump_south", 1, 39, 30, 0xF7E7DF, false),
        ("ride_jump_south", 2, 36, 27, 0xE7B172, true),
        ("ride_jump_south", 2, 44, 28, 0xAB7E3F, true),
        ("ride_run_east", 0, 42, 35, 0x815A2E, true),
        ("ride_run_east", 0, 34, 34, 0xAB7E3F, false),
        ("ride_run_east", 0, 40, 30, 0xE7B172, true),
        ("ride_run_north", 2, 35, 24, 0x523C26, true),
        ("ride_run_north", 2, 36, 24, 0xAB7E3F, false),
        ("ride_run_north", 2, 34, 25, 0xE7B172, true),
        ("ride_run_north", 2, 39, 31, 0x936244, false),
        ("ride_run_south", 2, 37, 15, 0x815A2E, true),
        ("ride_run_south", 2, 35, 22, 0x815A2E, false),
        ("ride_run_south", 2, 35, 28, 0x523C26, true),
        ("ride_run_south", 2, 36, 30, 0xAB7E3F, true),
        ("ride_run_south", 2, 43, 30, 0xAB7E3F, true),
        ("ride_run_south", 2, 37, 26, 0xAB7E3F, false),
        ("ride_run_south", 2, 42, 26, 0xAB7E3F, false),
        ("ride_run_south", 2, 39, 35, 0xC4A998, false),
    ];
    let cases = [
        ("ride_jump_east", 4, 164),
        ("ride_jump_north", 4, 48),
        ("ride_jump_south", 4, 216),
        ("ride_run_east", 4, 164),
        ("ride_run_north", 4, 48),
        ("ride_run_south", 4, 202),
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
                // The rider's lowest exposed skin ends at row 35. The complete
                // lower horse, tail, saddle, boots and East horse head must remain byte-identical.
                if y >= 36 || (case.ends_with("_east") && x % 80 >= 44) {
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
