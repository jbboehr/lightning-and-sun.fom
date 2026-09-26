use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Winter action corpus in extracted/ryis-beach-actions-study"]
fn ryis_winter_actions_cover_fingers_but_preserve_gloves_boots_and_mouth() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-beach-actions-study");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 11] = [
        ("blink_east", &[49, 53, 49]),
        ("blink_south", &[54, 58, 54]),
        ("drink_east", &[45, 48, 45]),
        ("drink_north", &[24, 24, 24]),
        ("drink_south", &[50, 56, 50]),
        ("eat_east", &[43, 44, 41, 47, 43]),
        ("eat_north", &[24, 24, 24]),
        ("eat_south", &[50, 52, 44, 59, 50]),
        ("sit_east", &[44]),
        ("sit_north", &[26]),
        ("sit_south", &[50]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // Skin includes eyelids, neck and exposed fingertips. Winter dark gloves,
    // orange sleeves, covered legs and boots use separate shades. Rear short hair
    // and eating mouth details are pinned independently at material boundaries.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates here are within the frame.
    let landmarks = [
        ("blink_south", 1, 38, 35, 0xB06C57, true), // closed eyelid
        ("blink_south", 1, 38, 36, 0x000000, false), // closed eye line
        ("blink_south", 0, 36, 41, 0x5F221E, false), // warm coat shadow
        ("blink_south", 0, 36, 42, 0xE08F4E, false), // orange coat highlight
        ("sit_north", 0, 39, 36, 0x5E423B, false),  // short rear hair
        ("sit_north", 0, 34, 46, 0x353A50, false),  // dark glove
        ("sit_north", 0, 40, 44, 0xC16639, false),  // orange coat
        ("sit_south", 0, 34, 46, 0x353A50, false),  // glove above finger
        ("sit_south", 0, 34, 47, 0x491F1B, true),   // detached finger
        ("sit_south", 0, 37, 49, 0x0E1E34, false),  // trouser hem
        ("sit_south", 0, 37, 50, 0x5A2E2B, false),  // boot shadow
        ("sit_south", 0, 37, 51, 0x805E54, false),  // boot highlight
        ("eat_south", 2, 38, 35, 0x410808, false),  // mouth interior
        ("eat_south", 2, 38, 36, 0xC83E37, false),  // red mouth detail
        ("eat_south", 2, 36, 40, 0x63342A, true),   // raised finger shadow
        ("eat_south", 2, 38, 40, 0x63342A, true),   // finger beside glove
        ("eat_south", 2, 37, 41, 0x353A50, false),  // adjacent glove
        ("eat_east", 2, 42, 35, 0x410808, false),   // side mouth interior
        ("eat_east", 2, 42, 36, 0xC83E37, false),   // side mouth detail
        ("eat_east", 2, 44, 40, 0x353A50, false),   // raised glove
        ("drink_south", 1, 35, 40, 0x353A50, false), // glove at cup
        ("drink_south", 1, 36, 41, 0x491F1B, true), // adjacent finger shadow
        ("drink_east", 0, 42, 42, 0x854D3C, true),  // finger beside glove
        ("drink_east", 0, 40, 44, 0x353A50, false), // dark glove
        ("drink_east", 1, 39, 40, 0x353A50, false), // raised glove at cup
        ("drink_east", 1, 40, 41, 0x491F1B, true),  // raised finger shadow
    ];
    let temp = tempfile::tempdir().unwrap();
    for preset in set["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let recipe = json!({
            "regions": profile["regions"],
            "rgba_map": profile["source_colors"].as_array().unwrap().iter()
                .zip(preset["colors"].as_array().unwrap())
                .map(|(from, to)| (from.as_str().unwrap().to_owned(), to.clone()))
                .collect::<serde_json::Map<_, _>>()
        });
        let recipe_path = temp.path().join(format!("{id}.json"));
        fs::write(&recipe_path, serde_json::to_vec(&recipe).unwrap()).unwrap();
        let output = temp.path().join(id);
        let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["apply", "--input"])
            .arg(&original)
            .arg("--palette")
            .arg(&recipe_path)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let targets = [0, 8, 2, 3].map(|i| {
            rgba(u32::from_str_radix(&preset["colors"][i].as_str().unwrap()[1..7], 16).unwrap())
        });
        let mut changed = 0;
        for (name, counts) in cases {
            let asset = format!(
                "assets/animations/NPCs/Ryis/Sprites/Winter/spr_npc_ryis_winter_{name}.png"
            );
            let before = image::open(original.join(&asset)).unwrap().to_rgba8();
            let after = image::open(output.join(&asset)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), (counts.len() as u32 * 80, 80));
            assert_eq!(before.dimensions(), after.dimensions());
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(output.join(meta)).unwrap()
            );
            let mut actual = vec![0; counts.len()];
            for (x, y, pixel) in before.enumerate_pixels() {
                let expected = match skin.iter().position(|c| rgba(*c) == pixel.0) {
                    Some(i) => {
                        actual[x as usize / 80] += 1;
                        targets[i]
                    }
                    None => pixel.0,
                };
                assert_eq!(
                    after.get_pixel(x, y).0,
                    expected,
                    "skin/material boundary {id} {name} [{x},{y}]"
                );
            }
            assert_eq!(actual, counts, "per-frame coverage {name}");
            changed += actual.iter().sum::<usize>();
            for &(case, frame, x, y, color, changes) in &landmarks {
                if case == name {
                    let x = frame * 80 + x;
                    assert_eq!(before.get_pixel(x, y).0, rgba(color));
                    assert_eq!(before.get_pixel(x, y) != after.get_pixel(x, y), changes);
                }
            }
        }
        assert_eq!(changed, 1348);
    }
}
