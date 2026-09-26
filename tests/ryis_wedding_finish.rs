use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Wedding finish corpus in extracted/ryis-wedding-finish-study"]
fn ryis_wedding_finish_covers_skin_and_preserves_suit_and_neckwear() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-wedding-finish-study");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 9] = [
        ("blink_east", &[53, 57, 53]),
        ("blink_south", &[62, 66, 62]),
        ("sit_east", &[47]),
        ("sit_north", &[32]),
        ("sit_south", &[56]),
        ("action_east", &[48, 48, 51, 48, 51, 48, 49]),
        ("action_north", &[30, 28, 28, 28, 28, 30, 34]),
        ("action_south", &[56, 56, 56, 56, 56, 56, 58]),
        ("kiss_east", &[49, 51, 56, 55]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // Warm red neckwear belongs to the Wedding outfit; it is not neck skin.
    // Cream fabric, dark cuffs, shoes and short rear hair also stay original.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates are within the frame.
    let landmarks = [
        ("blink_south", 1, 38, 35, 0xB06C57, true),  // eyelid skin
        ("blink_south", 1, 37, 36, 0x000000, false), // closed eye line
        ("blink_south", 0, 39, 39, 0x63342A, true),  // neck above neckwear
        ("blink_south", 0, 39, 40, 0xF49F7D, false), // neckwear highlight
        ("blink_south", 0, 39, 41, 0xD06B53, false), // neckwear shading
        ("blink_south", 0, 38, 40, 0x30302C, false), // dark collar
        ("blink_south", 0, 37, 41, 0xFFEFDC, false), // cream lapel
        ("blink_south", 0, 36, 42, 0xC39B7A, false), // lapel shadow
        ("blink_south", 0, 36, 43, 0x986B57, false), // deep sleeve shadow
        ("blink_south", 0, 34, 43, 0xE8C8B0, false), // light sleeve shading
        ("blink_south", 0, 34, 44, 0x3F3F3A, false), // cuff highlight
        ("blink_south", 0, 35, 44, 0x21211E, false), // cuff shadow
        ("blink_south", 0, 32, 45, 0xB06C57, true),  // hand under cuff
        ("blink_south", 0, 33, 46, 0x491F1B, true),  // finger detail
        ("blink_south", 0, 45, 45, 0xB06C57, true),  // other detached hand
        ("sit_south", 0, 33, 46, 0x854D3C, true),    // seated hand
        ("sit_south", 0, 34, 47, 0x491F1B, true),    // seated finger detail
        ("sit_south", 0, 35, 45, 0x30302C, false),   // seated cuff
        ("sit_south", 0, 39, 41, 0xF49F7D, false),   // seated neckwear
        ("sit_south", 0, 38, 49, 0xFFEFDC, false),   // trouser leg
        ("sit_south", 0, 37, 50, 0x111315, false),   // shoe shadow
        ("sit_south", 0, 38, 51, 0x313436, false),   // shoe highlight
        ("sit_north", 0, 39, 35, 0x5E423B, false),   // short rear hair
        ("sit_north", 0, 35, 36, 0xB06C57, true),    // ear beside hair
        ("sit_north", 0, 39, 41, 0xC39B7A, false),   // back collar
        ("sit_east", 0, 34, 46, 0xB06C57, true),     // hand beside lap
        ("sit_east", 0, 36, 45, 0x3F3F3A, false),    // cuff beside lap
        ("action_east", 1, 47, 43, 0xB06C57, true),  // extended hand highlight
        ("action_east", 1, 48, 44, 0x491F1B, true),  // extended finger detail
        ("action_east", 1, 49, 44, 0x63342A, true),  // fingertip shadow
        ("action_east", 1, 48, 42, 0x3F3F3A, false), // raised cuff
        ("action_east", 1, 45, 42, 0xFFEFDC, false), // raised sleeve
        ("action_north", 0, 45, 40, 0x491F1B, true), // far fingertip
        ("action_north", 0, 46, 41, 0x21211E, false), // far cuff
        ("kiss_east", 2, 44, 36, 0xB06C57, true),    // mouth-area skin
        ("kiss_east", 2, 45, 35, 0x000000, false),   // closed eye
        ("kiss_east", 2, 42, 40, 0xF49F7D, false),   // kissing neckwear
        ("kiss_east", 2, 35, 44, 0xB06C57, true),    // swinging hand
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
                "assets/animations/NPCs/Ryis/Sprites/Wedding/spr_npc_ryis_wedding_{name}.png"
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
        assert_eq!(changed, 1642);
    }

    // Practical controls for the two easy Wedding mistakes: dropping a hand
    // component, or mistaking the warm neckwear for another skin shade.
    let asset = "assets/animations/NPCs/Ryis/Sprites/Wedding/spr_npc_ryis_wedding_blink_south.png";
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
        .retain(|s| *s != json!([45, 45]));
    let omitted = apply_control("missing-hand", missing, map.clone());
    assert_eq!(omitted.get_pixel(45, 45).0, rgba(0xB06C57));
    assert_ne!(omitted.get_pixel(45, 45), correct.get_pixel(45, 45));
    let mut too_broad = map;
    too_broad.insert("#F49F7D".into(), json!("#FFFFFF"));
    let spilled = apply_control("neckwear-spill", region, too_broad);
    assert_eq!(correct.get_pixel(39, 40).0, rgba(0xF49F7D));
    assert_eq!(spilled.get_pixel(39, 40).0, rgba(0xFFFFFF));
}
