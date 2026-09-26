use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Summer action corpus in extracted/ryis-wedding-pilot-study"]
fn ryis_summer_actions_cover_fingers_and_legs_but_preserve_props_hair_and_mouth() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-wedding-pilot-study");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 11] = [
        ("blink_east", &[64, 68, 64]),
        ("blink_south", &[73, 77, 73]),
        ("drink_east", &[51, 57, 50]),
        ("drink_north", &[31, 28, 31]),
        ("drink_south", &[60, 65, 60]),
        ("eat_east", &[47, 51, 40, 51, 51]),
        ("eat_north", &[31, 28, 31]),
        ("eat_south", &[61, 57, 58, 65, 61]),
        ("sit_east", &[53]),
        ("sit_north", &[30]),
        ("sit_south", &[61]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // Summer's reviewed skin ramp covers detached fingers and exposed lower
    // legs. Dark gloves/cups and pink shoes use separate shades; the rear head
    // patch remains short hair. Eating mouth details are independently pinned.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates here are within the frame.
    let landmarks = [
        ("sit_north", 0, 39, 36, 0x5E423B, false), // short rear hair
        ("sit_north", 0, 34, 46, 0x353A50, false), // dark glove
        ("sit_north", 0, 40, 44, 0xFF5C64, false), // pink shirt
        ("sit_south", 0, 37, 49, 0x121221, false), // shorts hem
        ("sit_south", 0, 37, 50, 0x63342A, true),  // seated lower leg
        ("sit_south", 0, 37, 51, 0xFFA799, false), // pink footwear
        ("eat_south", 2, 38, 35, 0x410808, false), // mouth interior
        ("eat_south", 2, 38, 36, 0xC83E37, false), // red mouth detail
        ("eat_south", 2, 36, 40, 0x491F1B, true),  // raised hand outline
        ("eat_south", 2, 38, 40, 0x63342A, true),  // raised finger shading
        ("eat_south", 2, 37, 41, 0xB06C57, true),  // raised finger highlight
        ("eat_east", 2, 42, 35, 0x410808, false),  // side mouth interior
        ("eat_east", 2, 42, 36, 0xC83E37, false),  // side mouth detail
        ("eat_east", 2, 44, 40, 0x353A50, false),  // glove beside raised hand
        ("drink_south", 1, 35, 40, 0x353A50, false), // raised cup
        ("drink_south", 1, 36, 40, 0xB06C57, true), // adjacent finger
        ("drink_east", 0, 42, 42, 0x353A50, false), // resting cup
        ("drink_east", 0, 40, 44, 0x854D3C, true), // hand under cup
        ("drink_east", 1, 39, 40, 0x353A50, false), // raised cup side
        ("drink_east", 1, 40, 40, 0x854D3C, true), // raised finger
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
                "assets/animations/NPCs/Ryis/Sprites/Summer/spr_npc_ryis_summer_{name}.png"
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
        assert_eq!(changed, 1628);
    }
}
