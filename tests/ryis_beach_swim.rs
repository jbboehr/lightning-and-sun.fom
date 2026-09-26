use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Beach swimming corpus in extracted/ryis-beach-swim-study"]
fn ryis_beach_swimming_covers_face_edges_and_preserves_water_and_hair() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-beach-swim-study");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 2] = [
        ("bath_swim_east", &[38, 38, 36, 36]),
        ("bath_swim_south", &[41, 41, 39, 39]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // The lower face meets animated water. No body pixels are visible.
    // All four candidate shades are face skin; hair, eyes, water and foam
    // retain their original colors, even when touching the selected chin.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates are within the frame.
    let landmarks = [
        ("bath_swim_east", 0, 39, 46, 0x63342A, true), // forehead below hair
        ("bath_swim_east", 0, 40, 43, 0x322724, false), // hair highlight
        ("bath_swim_east", 0, 37, 49, 0x5E423B, false), // short side hair
        ("bath_swim_east", 0, 39, 50, 0x000000, false), // pupil
        ("bath_swim_east", 0, 38, 50, 0xC2B9BE, false), // eye shading
        ("bath_swim_east", 0, 38, 51, 0xECF0E9, false), // eye white
        ("bath_swim_east", 0, 40, 50, 0xB06C57, true), // cheek
        ("bath_swim_east", 0, 43, 52, 0x854D3C, true), // lower cheek shade
        ("bath_swim_east", 0, 40, 54, 0x63342A, true), // chin at waterline
        ("bath_swim_east", 0, 40, 55, 0x9DEBFC, false), // foam below chin
        ("bath_swim_east", 0, 34, 54, 0x328BC9, false), // dark water
        ("bath_swim_east", 0, 49, 49, 0x9DEBFC, false), // detached splash
        ("bath_swim_south", 0, 42, 47, 0x63342A, true), // isolated forehead patch
        ("bath_swim_south", 0, 42, 48, 0x854D3C, true), // same patch below hair
        ("bath_swim_south", 0, 39, 54, 0x63342A, true), // front chin
        ("bath_swim_south", 0, 39, 55, 0x9DEBFC, false), // adjacent foam
        ("bath_swim_south", 0, 33, 53, 0x328BC9, false), // surrounding water
        ("bath_swim_south", 2, 39, 53, 0xB06C57, true), // bobbing cheek
        ("bath_swim_south", 2, 39, 54, 0x854D3C, true), // submerged pose lower face
        ("bath_swim_south", 2, 39, 55, 0x9DEBFC, false), // foam below lowered face
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
            let asset =
                format!("assets/animations/NPCs/Ryis/Sprites/Beach/spr_npc_ryis_beach_{name}.png");
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
        assert_eq!(changed, 308);
    }

    // Practical controls for the two easy Beach mistakes: dropping an isolated forehead
    // component, or broadening the palette into the surrounding foam.
    let asset = "assets/animations/NPCs/Ryis/Sprites/Beach/spr_npc_ryis_beach_bath_swim_south.png";
    let source = temp.path().join("controls-source");
    fs::create_dir_all(source.join(asset).parent().unwrap()).unwrap();
    for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
        fs::copy(original.join(&path), source.join(path)).unwrap();
    }
    let region = profile["regions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["asset"] == asset)
        .unwrap()
        .clone();
    let map = profile["source_colors"]
        .as_array()
        .unwrap()
        .iter()
        .zip(set["presets"][0]["colors"].as_array().unwrap())
        .map(|(from, to)| (from.as_str().unwrap().to_owned(), to.clone()))
        .collect::<serde_json::Map<_, _>>();
    let apply_control = |id: &str, region: Value, map: serde_json::Map<String, Value>| {
        let recipe = temp.path().join(format!("control-{id}.json"));
        fs::write(
            &recipe,
            serde_json::to_vec(&json!({"regions":[region],"rgba_map":map})).unwrap(),
        )
        .unwrap();
        let output = temp.path().join(format!("control-{id}"));
        let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["apply", "--input"])
            .arg(&source)
            .arg("--palette")
            .arg(recipe)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        image::open(output.join(asset)).unwrap().to_rgba8()
    };
    let correct = apply_control("correct", region.clone(), map.clone());
    let mut missing = region.clone();
    missing["seeds"]
        .as_array_mut()
        .unwrap()
        .retain(|s| *s != json!([42, 47]));
    let omitted = apply_control("missing-forehead", missing, map.clone());
    assert_eq!(omitted.get_pixel(42, 48).0, rgba(0x854D3C));
    assert_ne!(omitted.get_pixel(42, 48), correct.get_pixel(42, 48));
    let mut too_broad = map;
    too_broad.insert("#9DEBFC".into(), json!("#FFFFFF"));
    let spilled = apply_control("water-spill", region, too_broad);
    assert_eq!(correct.get_pixel(39, 55).0, rgba(0x9DEBFC));
    assert_eq!(spilled.get_pixel(39, 55).0, rgba(0xFFFFFF));
}
