use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis action corpus in extracted/ryis-outfit-special-study"]
fn ryis_actions_cover_raised_fingers_and_preserve_hair_gloves_cups_and_mouth() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-outfit-special-study");
    let profile_path = std::env::var_os("FOM_RYIS_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/ryis-world-trial.json"));
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 11] = [
        ("blink_south", &[62, 66, 62]),
        ("blink_east", &[55, 59, 55]),
        ("sit_north", &[30]),
        ("sit_south", &[56]),
        ("sit_east", &[48]),
        ("eat_north", &[30, 26, 30]),
        ("eat_south", &[56, 59, 47, 63, 56]),
        ("eat_east", &[45, 49, 45, 49, 47]),
        ("drink_north", &[30, 26, 30]),
        ("drink_south", &[54, 63, 54]),
        ("drink_east", &[47, 50, 47]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // The reviewed action corpus uses these shades only on skin, including
    // disconnected fingers. The 5E423B rear head area is short hair.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates here are within the frame.
    let landmarks = [
        ("sit_north", 0, 39, 36, 0x5E423B, false),
        ("sit_north", 0, 34, 46, 0xECC45E, false),
        ("sit_north", 0, 40, 44, 0xF4877C, false),
        ("eat_south", 2, 38, 35, 0x410808, false),
        ("eat_south", 2, 38, 36, 0xC83E37, false),
        ("eat_south", 2, 36, 40, 0x491F1B, true),
        ("eat_south", 2, 38, 40, 0xB06C57, true),
        ("eat_south", 2, 37, 41, 0xECC45E, false),
        ("drink_south", 1, 35, 40, 0xECC45E, false),
        ("drink_south", 1, 36, 40, 0xB06C57, true),
        ("drink_east", 0, 40, 44, 0xECC45E, false),
        ("drink_east", 0, 42, 42, 0x854D3C, true),
        ("drink_east", 1, 40, 40, 0xB06C57, true),
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
                "assets/animations/NPCs/Ryis/Sprites/Spring/spr_npc_ryis_spring_{name}.png"
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
        assert_eq!(changed, 1496);
    }
}
