use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/hayden-autumn-actions-study and the local accepted Summer standard baseline"]
fn hayden_summer_special_masks_preserve_book_tools_and_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-autumn-actions-study");
    let baseline =
        root.join("generated/characters-world-seasonal-standard-trial/characters/hayden");
    let set = root.join("palettes/sets/hayden-world-trial.json");
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..208];
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
    // Tiny fingers and low harvesting forearms change; shirt seams, book pages,
    // covers, tool heads, handles and swing effects must remain untouched.
    let landmarks = [
        ("hammer_east", 0, 39, 43, 0x815A2E, true),
        ("hammer_east", 0, 53, 39, 0x85CED4, false),
        ("hammer_east", 1, 35, 33, 0xAB7E3F, true),
        ("hammer_east", 1, 37, 33, 0x815A2E, true),
        ("hammer_east", 1, 38, 44, 0x523C26, false),
        ("hammer_east", 2, 51, 28, 0xC5E3E4, false),
        ("hammer_east", 2, 47, 43, 0x815A2E, true),
        ("harvest_east", 2, 42, 48, 0xE7B172, true),
        ("harvest_east", 2, 47, 52, 0x815A2E, true),
        ("harvest_east", 3, 48, 53, 0x815A2E, true),
        ("harvest_east", 3, 43, 46, 0xEEA440, false),
        ("harvest_east", 3, 39, 43, 0xDFC6A1, false),
        ("read_sit_start_south", 0, 32, 43, 0x815A2E, true),
        ("read_sit_start_south", 0, 36, 43, 0x523C26, false),
        ("read_sit_start_south", 1, 39, 39, 0xF6E4D7, false),
        ("read_sit_start_south", 1, 44, 43, 0xAB7E3F, true),
        ("read_sit_start_south", 2, 46, 44, 0x815A2E, true),
        ("read_sit_loop_south", 0, 38, 39, 0xE7B172, true),
        ("read_sit_loop_south", 0, 39, 40, 0xE7B172, true),
        ("read_sit_loop_south", 0, 34, 39, 0xC9AF9C, false),
        ("read_sit_loop_south", 0, 33, 40, 0x693F22, false),
        ("read_sit_loop_south", 0, 34, 41, 0x9A5F1B, false),
        ("read_sit_loop_south", 2, 42, 38, 0xE7B172, true),
        ("read_sit_end_south", 0, 46, 44, 0x815A2E, true),
        ("read_sit_end_south", 0, 35, 38, 0xF6E4D7, false),
        ("read_sit_end_south", 1, 38, 45, 0x9A5F1B, false),
        ("read_sit_end_south", 2, 43, 43, 0x523C26, false),
        ("till_east", 0, 41, 47, 0x815A2E, true),
        ("till_east", 1, 50, 43, 0x815A2E, true),
        ("till_east", 1, 51, 47, 0x7C4A53, false),
        ("till_east", 1, 56, 47, 0xDDEAF6, false),
        ("water_east", 0, 38, 44, 0x815A2E, true),
        ("water_east", 0, 47, 39, 0xBE6D44, false),
        ("water_east", 1, 39, 44, 0x523C26, false),
        ("water_east", 1, 51, 37, 0xDD9D3E, false),
        ("water_east", 1, 52, 37, 0xFFF672, false),
        ("wipebrow_south", 0, 39, 31, 0xE7B172, true),
        ("wipebrow_south", 0, 40, 35, 0x815A2E, true),
        ("wipebrow_south", 0, 40, 28, 0x66534A, false),
        ("wipebrow_south", 1, 43, 35, 0x815A2E, true),
        ("wipebrow_south", 2, 44, 35, 0x815A2E, true),
        ("wipebrow_south", 2, 36, 45, 0x523C26, false),
        ("wipebrow_south", 4, 36, 44, 0x523C26, false),
        ("wipebrow_south", 5, 45, 46, 0x815A2E, true),
    ];
    let cases = [
        ("hammer_east", 6, 236),
        ("harvest_east", 10, 461),
        ("read_sit_start_south", 3, 128),
        ("read_sit_loop_south", 4, 90),
        ("read_sit_end_south", 3, 128),
        ("till_east", 5, 192),
        ("water_east", 4, 136),
        ("wipebrow_south", 6, 373),
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
                "assets/animations/NPCs/Hayden/Sprites/Summer/spr_npc_hayden_specialanimation_summer_{case}.png"
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
