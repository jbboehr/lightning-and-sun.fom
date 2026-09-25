use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/hayden-winter-actions-study and the local accepted Winter expansion baseline"]
fn hayden_winter_actions_masks_preserve_sleeves_mouth_and_previous_outputs() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-winter-actions-study");
    let baseline = root.join("generated/characters-world-winter-expansion-trial/characters/hayden");
    let set = root.join("palettes/sets/hayden-world-trial.json");
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..252];
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
    // Tiny exposed necklines and the raised eating finger change beside preserved
    // cream cuffs, gold sleeves, coat seams and both red mouth shades.
    let landmarks = [
        ("blink_east", 1, 35, 46, 0x815A2E, true),
        ("blink_east", 1, 45, 43, 0xC6994D, false),
        ("blink_east", 1, 36, 38, 0xFFF5DA, false),
        ("blink_south", 1, 34, 46, 0x815A2E, true),
        ("blink_south", 1, 37, 42, 0x384C92, false),
        ("blink_south", 1, 39, 27, 0x66534A, false),
        ("drink_east", 1, 35, 43, 0xA57333, false),
        ("drink_east", 1, 36, 43, 0xC6994D, false),
        ("drink_east", 1, 35, 39, 0xFFF5DA, false),
        ("drink_north", 1, 31, 44, 0xAB7E3F, true),
        ("drink_north", 1, 33, 46, 0x815A2E, true),
        ("drink_north", 1, 46, 39, 0xE5BA5D, false),
        ("drink_south", 1, 47, 43, 0xFFF5DA, false),
        ("drink_south", 1, 45, 43, 0xFFF5DA, false),
        ("drink_south", 1, 43, 40, 0xFFF5DA, false),
        ("eat_east", 0, 40, 43, 0xA57333, false),
        ("eat_east", 1, 41, 36, 0x9E2626, false),
        ("eat_east", 2, 40, 32, 0x410808, false),
        ("eat_east", 2, 40, 33, 0x9E2626, false),
        ("eat_east", 2, 38, 38, 0xFFF5DA, false),
        ("eat_east", 3, 37, 44, 0xC6994D, false),
        ("eat_north", 1, 31, 44, 0xAB7E3F, true),
        ("eat_north", 1, 33, 46, 0x815A2E, true),
        ("eat_north", 1, 46, 38, 0xD8BB9C, false),
        ("eat_south", 1, 39, 37, 0x9E2626, false),
        ("eat_south", 2, 38, 32, 0x410808, false),
        ("eat_south", 2, 38, 33, 0x9E2626, false),
        ("eat_south", 2, 44, 38, 0xFFF5DA, false),
        ("eat_south", 3, 37, 43, 0x815A2E, true),
        ("sit_east", 0, 32, 44, 0xAB7E3F, true),
        ("sit_east", 0, 35, 43, 0xD8BB9C, false),
        ("sit_east", 0, 35, 39, 0xD8BB9C, false),
        ("sit_north", 0, 47, 43, 0xFFF5DA, false),
        ("sit_north", 0, 33, 46, 0x815A2E, true),
        ("sit_north", 0, 35, 38, 0xD8BB9C, false),
        ("sit_south", 0, 47, 43, 0xFFF5DA, false),
        ("sit_south", 0, 34, 43, 0xFFF5DA, false),
        ("sit_south", 0, 34, 38, 0xFFF5DA, false),
        ("blink_east", 1, 41, 40, 0xE7B172, true),
        ("blink_south", 1, 39, 40, 0xE7B172, true),
        ("drink_south", 1, 39, 41, 0xE7B172, true),
        ("eat_east", 4, 41, 40, 0xE7B172, true),
        ("eat_south", 2, 36, 37, 0xAB7E3F, true),
        ("eat_south", 2, 40, 39, 0xE7B172, true),
        ("sit_east", 0, 41, 40, 0xE7B172, true),
        ("sit_south", 0, 39, 40, 0xE7B172, true),
        ("drink_north", 1, 35, 43, 0x6E4922, false),
        ("eat_north", 1, 44, 42, 0x6E4922, false),
        ("drink_east", 1, 40, 39, 0xE7B172, true),
        ("eat_east", 2, 44, 36, 0xE7B172, true),
        ("drink_south", 1, 33, 40, 0xFFF5DA, false),
        ("sit_north", 0, 47, 44, 0xE7B172, true),
        ("sit_south", 0, 47, 44, 0xE7B172, true),
        ("blink_east", 1, 40, 33, 0xE7B172, true),
    ];
    let cases = [
        ("blink_east", 3, 82),
        ("blink_south", 3, 106),
        ("drink_east", 3, 78),
        ("drink_north", 3, 34),
        ("drink_south", 3, 114),
        ("eat_east", 5, 117),
        ("eat_north", 3, 34),
        ("eat_south", 5, 192),
        ("sit_east", 1, 24),
        ("sit_north", 1, 20),
        ("sit_south", 1, 36),
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
                "assets/animations/NPCs/Hayden/Sprites/Winter/spr_npc_hayden_winter_{case}.png"
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
                // The lowered hand reaches row 48; trousers and boots below stay original.
                if y >= 49 {
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
