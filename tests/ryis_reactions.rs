use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Spring reaction and reading corpus in extracted/ryis-wedding-pilot-study"]
fn ryis_reactions_cover_isolated_fingers_and_preserve_book_gloves_hair_and_shocked_mouth() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-wedding-pilot-study");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("shocked_start", &[66]),
        ("shocked_loop", &[64]),
        ("shocked_end", &[66]),
        ("read_sit_start", &[54, 48, 50]),
        ("read_sit_loop", &[44, 56, 44, 56]),
        ("read_sit_end", &[54, 44, 54]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // These shades occur only on reviewed skin in these six pinned strips.
    // The blue book and cream pages use separate colors, despite resembling
    // the Debug Blue skin target. Detached fingers still need their own seeds.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates here are within the frame.
    let landmarks = [
        ("shocked_loop", 0, 39, 36, 0x410808, false),
        ("shocked_loop", 0, 39, 37, 0xC83E37, false),
        ("shocked_loop", 0, 34, 35, 0xB06C57, true),
        ("shocked_loop", 0, 33, 35, 0xECC45E, false),
        ("shocked_loop", 0, 35, 38, 0x491F1B, true),
        ("read_sit_start", 1, 39, 43, 0xF6E4D7, false),
        ("read_sit_start", 1, 34, 46, 0xECC45E, false),
        ("read_sit_start", 1, 35, 47, 0x491F1B, true),
        ("read_sit_start", 2, 40, 45, 0xC9AF9C, false),
        ("read_sit_start", 2, 35, 47, 0x491F1B, true),
        ("read_sit_loop", 0, 34, 45, 0x699CC1, false),
        ("read_sit_loop", 0, 35, 46, 0x4E7E9F, false),
        ("read_sit_loop", 0, 36, 47, 0x2E4D69, false),
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
            let special = if name.starts_with("read_sit_") {
                "specialanimation_"
            } else {
                ""
            };
            let asset = format!(
                "assets/animations/NPCs/Ryis/Sprites/Spring/spr_npc_ryis_{special}spring_{name}_south.png"
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
        assert_eq!(changed, 700);
    }
}
