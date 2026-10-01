use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/hayden and the local accepted Winter actions baseline"]
fn hayden_winter_standard_masks_preserve_coat_shadows_and_cover_skin() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/hayden");
    let baseline = root.join("generated/characters-world-winter-actions-trial/characters/hayden");
    let set = root.join("palettes/sets/hayden-world-trial.json");
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..263];
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
    // Winter coat shadows reuse skin shades; preserve them beside cream cuffs,
    // while tiny necklines, exposed fingers and the kissing cheek change.
    let landmarks = [
        ("action_east", 0, 38, 43, 0xE5BA5D, false),
        ("action_east", 0, 37, 40, 0xFFF5DA, false),
        ("action_east", 1, 47, 43, 0x815A2E, true),
        ("action_east", 1, 43, 40, 0xA57333, false),
        ("action_east", 2, 45, 44, 0x815A2E, true),
        ("action_east", 2, 40, 38, 0xFFF5DA, false),
        ("action_north", 0, 32, 45, 0x815A2E, true),
        ("action_north", 0, 33, 39, 0xA57333, false),
        ("action_north", 2, 35, 45, 0x815A2E, true),
        ("action_north", 2, 43, 38, 0xC6994D, false),
        ("action_north", 2, 39, 26, 0x66534A, false),
        ("action_south", 0, 35, 47, 0x815A2E, true),
        ("action_south", 0, 34, 40, 0xD8BB9C, false),
        ("action_south", 1, 36, 43, 0xA57333, false),
        ("action_south", 1, 36, 38, 0xD8BB9C, false),
        ("action_south", 2, 38, 43, 0xAB7E3F, false),
        ("action_south", 2, 44, 38, 0xFFF5DA, false),
        ("kiss_east", 0, 43, 45, 0xFFF5DA, false),
        ("kiss_east", 0, 43, 41, 0xD8BB9C, false),
        ("kiss_east", 2, 41, 35, 0xE8B271, true),
        ("kiss_east", 2, 39, 38, 0xFFF5DA, false),
        ("kiss_east", 2, 38, 39, 0xD8BB9C, false),
        ("kiss_east", 2, 37, 46, 0x815A2E, true),
        ("kiss_east", 3, 37, 47, 0x815A2E, true),
        ("sleep_east", 0, 38, 32, 0x815A2E, true),
        ("sleep_east", 0, 44, 36, 0x815A2E, true),
        ("sleep_east", 0, 37, 38, 0xE5BA5D, false),
        ("sleep_east", 0, 38, 39, 0xE5BA5D, false),
        ("sleep_east", 0, 43, 41, 0x4570A3, false),
        ("action_north", 0, 47, 38, 0xAB7E3F, false),
        ("action_north", 0, 48, 39, 0xAB7E3F, false),
        ("action_north", 0, 48, 42, 0x815A2E, false),
        ("action_north", 2, 47, 36, 0x6E4922, false),
        ("action_south", 2, 36, 41, 0xAB7E3F, false),
        ("action_south", 2, 37, 42, 0x6E4922, false),
        ("action_south", 2, 36, 45, 0xAB7E3F, false),
        ("action_south", 2, 39, 44, 0xE7B172, true),
        ("action_south", 2, 39, 40, 0xE7B172, true),
        ("kiss_east", 2, 38, 42, 0xAB7E3F, false),
        ("kiss_east", 2, 44, 40, 0xE7B172, true),
        ("action_east", 0, 39, 45, 0xAB7E3F, true),
        ("action_east", 0, 42, 41, 0xE7B172, true),
        ("sleep_east", 0, 36, 41, 0x6E4922, false),
    ];
    let cases = [
        ("action_east", 7, 162),
        ("action_north", 7, 48),
        ("action_south", 7, 190),
        ("kiss_east", 4, 106),
        ("sleep_east", 1, 27),
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
                // The lowered hands end at row 47; trousers and boots below stay original.
                if y >= 48 {
                    assert_eq!(p, q, "lower clothing changed: {id} {case} [{x},{y}]");
                }
                // His raised North-facing arm is covered by the Winter coat;
                // only the lowered hands below the cuffs are exposed.
                if case == "action_north" && y < 44 {
                    assert_eq!(p, q, "raised Winter sleeve changed: {id} [{x},{y}]");
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
