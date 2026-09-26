use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Autumn special corpus in extracted/ryis-wedding-finish-study"]
fn ryis_autumn_specials_preserve_tools_books_gloves_and_coat() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-wedding-finish-study");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 11] = [
        ("hammer_east", &[52, 44, 44, 51, 51, 51, 51]),
        ("read_sit_end_south", &[54, 40, 51]),
        ("read_sit_loop_south", &[44, 56, 44, 56]),
        ("read_sit_start_south", &[51, 44, 50]),
        ("saw_east", &[43, 47, 47, 47]),
        ("siteyesclosed_east", &[54]),
        ("siteyesclosed_south", &[60]),
        ("wipebrow_south", &[32, 44, 50, 59, 53, 52]),
        ("write_end_south", &[55, 59]),
        ("write_loop_south", &[54, 54, 54, 54]),
        ("write_start_south", &[59, 55]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // Reviewed skin includes detached fingertips beside yellow gloves and
    // props. Blue covers, cream paper, brown handles/clipboard and the coat
    // use separate shades. Covered legs, boots and mouth details stay original.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates here are within the frame.
    let landmarks = [
        ("hammer_east", 0, 52, 43, 0x85CED4, false), // hammer head
        ("hammer_east", 0, 48, 44, 0x936244, false), // brown handle
        ("hammer_east", 3, 50, 44, 0xE7A063, false), // glove beside hammer
        ("hammer_east", 3, 47, 45, 0xB06C57, true),  // gripping fingertip
        ("saw_east", 0, 60, 44, 0xC0BDD7, false),    // saw blade
        ("saw_east", 0, 50, 46, 0x491F1B, true),     // detached finger
        ("saw_east", 0, 49, 45, 0xF4CD86, false),    // adjacent yellow glove
        ("wipebrow_south", 0, 42, 33, 0x63342A, true), // finger shading
        ("wipebrow_south", 0, 41, 33, 0xB06C57, true), // finger at brow
        ("wipebrow_south", 0, 43, 35, 0xF4CD86, false), // raised glove
        ("wipebrow_south", 4, 40, 38, 0x9E2626, false), // mouth detail
        ("write_loop_south", 0, 36, 43, 0xDCEEF8, false), // light cuff
        ("write_loop_south", 0, 37, 45, 0xB06C57, true), // writing finger
        ("write_loop_south", 0, 41, 47, 0xB06C57, true), // clipboard finger
        ("write_loop_south", 0, 43, 47, 0x854D3C, true), // finger shading
        ("write_loop_south", 0, 43, 46, 0xF4CD86, false), // glove above finger
        ("write_loop_south", 0, 44, 43, 0x7D4F3D, false), // brown clipboard
        ("write_loop_south", 0, 37, 51, 0x111315, false), // covered lower leg
        ("write_loop_south", 0, 37, 53, 0x805E54, false), // brown boot
        ("read_sit_start_south", 1, 39, 41, 0xF6E4D7, false), // cream pages
        ("read_sit_start_south", 1, 38, 48, 0x699CC1, false), // blue cover
        ("read_sit_start_south", 1, 34, 46, 0xF4CD86, false), // glove by book
        ("read_sit_start_south", 1, 35, 47, 0x491F1B, true), // detached finger
        ("read_sit_start_south", 2, 40, 45, 0xC9AF9C, false), // shaded page
        ("read_sit_loop_south", 0, 35, 46, 0x4E7E9F, false), // cover shading
        ("read_sit_loop_south", 0, 36, 47, 0x2E4D69, false), // cover edge
        ("siteyesclosed_south", 0, 37, 50, 0x5A2E2B, false), // boot shadow
        ("siteyesclosed_south", 0, 37, 51, 0x805E54, false), // boot highlight
        ("siteyesclosed_south", 0, 37, 36, 0x000000, false), // closed eye
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
                "assets/animations/NPCs/Ryis/Sprites/Autumn/spr_npc_ryis_specialanimation_autumn_{name}.png"
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
        assert_eq!(changed, 1866);
    }
}
