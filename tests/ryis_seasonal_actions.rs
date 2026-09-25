use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Autumn idle/walk corpus in extracted/ryis-autumn-special-study"]
fn ryis_autumn_pilot_preserves_coat_gloves_boots_and_short_hair() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-autumn-special-study");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("idle_east", &[47]),
        ("idle_north", &[26]),
        ("idle_south", &[52]),
        ("walk_east", &[47, 49, 47, 47]),
        ("walk_north", &[26, 25, 26, 25]),
        ("walk_south", &[52, 51, 52, 51]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // Autumn skin is restricted to the face, ears, neck and detached fingers.
    // The blue scarf and coat, yellow gloves, covered legs and brown boots
    // have their own shades; the broad rear hair patch also stays original.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates here are within the frame.
    let landmarks = [
        ("idle_north", 0, 39, 35, 0x5E423B, false), // short rear hair
        ("idle_north", 0, 35, 35, 0xB06C57, true),  // adjacent ear
        ("idle_south", 0, 38, 31, 0x63342A, true),  // forehead at hairline
        ("idle_south", 0, 40, 31, 0x1B1717, false), // adjacent hair
        ("idle_south", 0, 37, 35, 0xC2B9BE, false), // shaded eye white
        ("idle_south", 0, 37, 36, 0xECF0E9, false), // eye white
        ("idle_south", 0, 39, 40, 0x854D3C, true),  // neck above scarf
        ("idle_south", 0, 39, 41, 0x274286, false), // scarf
        ("idle_south", 0, 35, 44, 0x6482AA, false), // blue sleeve
        ("idle_south", 0, 34, 44, 0xDCEEF8, false), // light cuff
        ("idle_south", 0, 34, 45, 0xF4CD86, false), // yellow glove
        ("idle_south", 0, 34, 46, 0xB06C57, true),  // detached fingertip
        ("idle_south", 0, 33, 46, 0x63342A, true),  // fingertip shadow
        ("idle_south", 0, 39, 44, 0xF27E55, false), // orange shirt
        ("idle_south", 0, 39, 45, 0xFF9E6D, false), // shirt highlight
        ("idle_south", 0, 38, 48, 0xA78A79, false), // belt
        ("idle_south", 0, 37, 51, 0x111315, false), // covered lower leg
        ("idle_south", 0, 37, 52, 0x5A2E2B, false), // boot shadow
        ("idle_south", 0, 37, 53, 0x805E54, false), // boot highlight
        ("walk_east", 3, 36, 47, 0x63342A, true),   // moving finger
        ("walk_east", 3, 35, 47, 0xF4CD86, false),  // neighboring glove
        ("walk_north", 1, 41, 52, 0x111315, false), // moving trouser leg
        ("walk_north", 1, 41, 54, 0x805E54, false), // moving boot
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
        assert_eq!(changed, 623);
    }
}
