use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Autumn action/sleep/kiss corpus in extracted/test-corpus/ryis"]
fn ryis_autumn_special_preserves_gloves_sleeves_boots_and_closed_features() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/ryis");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 5] = [
        ("action_east", &[46, 44, 46, 44, 46, 46, 47]),
        ("action_north", &[23, 24, 24, 24, 24, 24, 26]),
        ("action_south", &[51, 51, 51, 51, 51, 51, 52]),
        ("kiss_east", &[46, 49, 54, 53]),
        ("sleep_east", &[48]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // Reviewed Autumn skin includes detached fingertips and closed eyelids.
    // Yellow gloves, blue sleeves, covered legs and boots retain their shades.
    // Short rear hair, black closed eyes and the kissing mouth stay original.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates here are within the frame.
    let landmarks = [
        ("action_north", 0, 39, 35, 0x5E423B, false), // short rear hair
        ("action_north", 0, 35, 35, 0xB06C57, true),  // nearby ear
        ("action_north", 0, 33, 43, 0xF4CD86, false), // yellow glove
        ("action_north", 1, 37, 50, 0x111315, false), // moving trouser hem
        ("action_north", 1, 37, 52, 0x5C413D, false), // boot shading
        ("action_east", 1, 48, 42, 0xE7A063, false),  // extended glove shading
        ("action_east", 1, 48, 43, 0xF4CD86, false),  // glove beside finger
        ("action_east", 1, 48, 44, 0x63342A, true),   // detached fingertip
        ("action_south", 6, 37, 50, 0x1E2124, false), // trousers
        ("action_south", 6, 37, 51, 0x111315, false), // covered lower leg
        ("action_south", 6, 37, 53, 0x805E54, false), // brown footwear
        ("sleep_east", 0, 43, 38, 0xE7A063, false),   // glove at cheek
        ("sleep_east", 0, 44, 38, 0x63342A, true),    // finger beside glove
        ("sleep_east", 0, 42, 40, 0x6482AA, false),   // raised blue sleeve
        ("sleep_east", 0, 38, 36, 0x000000, false),   // closed eye
        ("kiss_east", 2, 47, 37, 0x000000, false),    // kissing mouth outline
        ("kiss_east", 2, 46, 37, 0x854D3C, true),     // lip/cheek skin
        ("kiss_east", 2, 38, 35, 0x000000, false),    // closed eye
        ("kiss_east", 2, 39, 35, 0xB06C57, true),     // skin beside closed eye
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
                "assets/animations/NPCs/Ryis/Sprites/Autumn/spr_npc_ryis_autumn_{name}.png"
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
        assert_eq!(changed, 1096);
    }
}
