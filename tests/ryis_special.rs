use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Spring special corpus in extracted/ryis-beach-pilot-study"]
fn ryis_special_cover_tool_and_writing_fingers_but_preserve_props_gloves_and_hair() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-beach-pilot-study");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 8] = [
        ("hammer_east", &[50, 48, 48, 46, 48, 50, 50]),
        ("saw_east", &[46, 49, 51, 49]),
        ("siteyesclosed_east", &[56]),
        ("siteyesclosed_south", &[64]),
        ("wipebrow_south", &[36, 48, 54, 65, 59, 58]),
        ("write_start_south", &[57, 56]),
        ("write_loop_south", &[54, 54, 54, 54]),
        ("write_end_south", &[56, 57]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // These four shades occur only on reviewed skin in the eight pinned strips.
    // Brown tools and writing props use separate colors. Tiny detached fingers
    // at the brow, hammer and clipboard need their own component seeds.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates here are within the frame.
    let landmarks = [
        ("hammer_east", 0, 52, 43, 0x85CED4, false),
        ("hammer_east", 0, 48, 44, 0x936244, false),
        ("hammer_east", 3, 50, 44, 0x63342A, true),
        ("saw_east", 0, 60, 44, 0xC0BDD7, false),
        ("saw_east", 0, 50, 46, 0x491F1B, true),
        ("wipebrow_south", 0, 42, 33, 0x63342A, true),
        ("wipebrow_south", 0, 41, 33, 0xECC45E, false),
        ("wipebrow_south", 0, 36, 33, 0x322724, false),
        ("write_loop_south", 0, 36, 43, 0x854D3C, true),
        ("write_loop_south", 0, 41, 47, 0xB06C57, true),
        ("write_loop_south", 0, 43, 47, 0xECC45E, false),
        ("write_loop_south", 0, 44, 43, 0x7D4F3D, false),
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
                "assets/animations/NPCs/Ryis/Sprites/Spring/spr_npc_ryis_specialanimation_spring_{name}.png"
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
        assert_eq!(changed, 1417);
    }
}
