use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Summer idle/walk corpus in extracted/ryis-wedding-pilot-study"]
fn ryis_summer_pilot_covers_exposed_skin_but_preserves_outfit_and_short_hair() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-wedding-pilot-study");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("idle_east", &[60]),
        ("idle_north", &[42]),
        ("idle_south", &[69]),
        ("walk_east", &[60, 61, 60, 58]),
        ("walk_north", &[42, 38, 42, 38]),
        ("walk_south", &[69, 63, 69, 63]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // Summer's exposed lower legs use the same four skin shades as the face
    // and detached fingers. Dark gloves, pink footwear and the broad short
    // hair patch at the back of the head use different colors and stay original.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates here are within the frame.
    let landmarks = [
        ("idle_north", 0, 39, 35, 0x5E423B, false), // short rear hair
        ("idle_north", 0, 35, 35, 0xB06C57, true),  // adjacent ear
        ("idle_south", 0, 38, 31, 0x63342A, true),  // forehead at hairline
        ("idle_south", 0, 39, 31, 0x322724, false), // adjacent hair
        ("idle_south", 0, 34, 44, 0xB06C57, true),  // forearm
        ("idle_south", 0, 34, 45, 0x353A50, false), // dark glove
        ("idle_south", 0, 34, 46, 0xB06C57, true),  // detached finger
        ("idle_south", 0, 39, 41, 0xB06C57, true),  // chest
        ("idle_south", 0, 39, 42, 0xF5ECE6, false), // undershirt
        ("idle_south", 0, 37, 50, 0x121221, false), // shorts hem
        ("idle_south", 0, 37, 51, 0x854D3C, true),  // exposed leg
        ("idle_south", 0, 37, 52, 0x63342A, true),  // leg shading
        ("idle_south", 0, 37, 53, 0xFFA799, false), // footwear
        ("walk_east", 3, 36, 47, 0x491F1B, true),   // moving finger
        ("walk_east", 3, 35, 47, 0x353A50, false),  // neighboring glove
        ("walk_north", 1, 41, 52, 0xB06C57, true),  // moving lower leg
        ("walk_north", 1, 41, 54, 0xFFA799, false), // moving footwear
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
        assert_eq!(changed, 834);
    }
}
