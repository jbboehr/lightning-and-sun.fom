use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/hayden-autumn-special-study and the local reactions baseline"]
fn hayden_special_masks_cover_moving_hands_without_recoloring_shirts_or_tools() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-autumn-special-study");
    let baseline = root.join("generated/characters-spring-reactions-trial/characters/hayden");
    let set = root.join("palettes/sets/hayden-world-trial.json");
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..161];
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
        ("hammer_east", 0, 40, 38, 0xAB7E3F, false),
        ("hammer_east", 0, 40, 41, 0x815A2E, true),
        ("hammer_east", 0, 49, 44, 0xBB8151, false),
        ("hammer_east", 0, 55, 43, 0x6099A8, false),
        ("hammer_east", 1, 41, 40, 0x815A2E, true),
        ("hammer_east", 1, 42, 41, 0xAB7E3F, false),
        ("hammer_east", 2, 58, 36, 0xC5E3E4, false),
        ("harvest_east", 3, 49, 48, 0x815A2E, true),
        ("harvest_east", 3, 45, 48, 0x815A2E, false),
        ("harvest_east", 3, 42, 46, 0xAB7E3F, false),
        ("harvest_east", 3, 47, 51, 0xE7B172, true),
        ("harvest_east", 3, 46, 45, 0x66534A, false),
        ("pet_east", 0, 38, 43, 0x815A2E, false),
        ("pet_east", 0, 43, 43, 0x815A2E, true),
        ("pet_east", 2, 41, 41, 0x815A2E, true),
        ("pet_east", 2, 39, 43, 0x815A2E, false),
        ("pet_east", 3, 40, 44, 0xAB7E3F, false),
        ("pet_east", 3, 42, 44, 0xAB7E3F, true),
        ("sigh_south", 0, 39, 36, 0x9E2626, false),
        ("sigh_south", 0, 39, 35, 0x410808, false),
        ("sigh_south", 0, 32, 40, 0xE7B172, true),
        ("till_east", 1, 43, 44, 0x815A2E, true),
        ("till_east", 1, 42, 41, 0xAB7E3F, false),
        ("till_east", 1, 52, 47, 0xA45759, false),
        ("till_east", 1, 56, 47, 0xDDEAF6, false),
        ("water_east", 1, 43, 38, 0x815A2E, true),
        ("water_east", 1, 42, 40, 0xAB7E3F, false),
        ("water_east", 1, 52, 39, 0xFFFFFF, false),
        ("water_east", 1, 52, 41, 0xFFF672, false),
        ("wipebrow_south", 2, 43, 39, 0x815A2E, true),
        ("wipebrow_south", 2, 35, 39, 0x815A2E, false),
        ("wipebrow_south", 2, 44, 40, 0x815A2E, true),
        ("wipebrow_south", 2, 39, 34, 0xE7B172, true),
        ("wipebrow_south", 2, 40, 28, 0x66534A, false),
    ];
    let cases = [
        ("hammer_east", 6, 237),
        ("harvest_east", 10, 475),
        ("pet_east", 7, 328),
        ("sigh_south", 4, 250),
        ("till_east", 5, 197),
        ("water_east", 4, 141),
        ("wipebrow_south", 6, 380),
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
                mask.push(p != q);
                if p != q {
                    let index = source.iter().position(|c| rgba(*c) == p.0).expect(
                        "hair, clothing, tools, effects, mouth or another non-skin color changed",
                    );
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
