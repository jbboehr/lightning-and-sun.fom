use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Beach blink/action/kiss corpus in extracted/ryis-beach-swim-study"]
fn ryis_beach_actions_cover_closed_features_and_limbs_without_material_spills() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-beach-swim-study");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("blink_east", &[95, 99, 95]),
        ("blink_south", &[106, 110, 106]),
        ("action_east", &[81, 74, 74, 74, 74, 81, 89]),
        ("action_north", &[74, 73, 73, 73, 73, 77, 79]),
        ("action_south", &[93, 94, 94, 94, 94, 93, 102]),
        ("kiss_east", &[83, 86, 93, 90]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // Actual Beach frames include closed eyelids, exposed chest/back,
    // gesturing bare arms and hands, legs and toes. Hair, tank, shorts,
    // sandal straps and wristbands retain their separate material colors.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates are within the frame.
    let landmarks = [
        ("blink_south", 1, 38, 35, 0xB06C57, true), // closed eyelid
        ("blink_south", 1, 37, 36, 0x000000, false), // closed eye line
        ("blink_south", 1, 39, 41, 0xB06C57, true), // exposed chest
        ("blink_south", 1, 37, 40, 0xFF948B, false), // tank strap
        ("blink_south", 1, 45, 44, 0x354647, false), // wristband
        ("blink_south", 1, 45, 45, 0xB06C57, true), // hand below wristband
        ("blink_south", 1, 38, 49, 0xB06C57, true), // shin
        ("blink_south", 1, 37, 52, 0x72B2D9, false), // sandal strap
        ("blink_south", 1, 37, 53, 0xB06C57, true), // toes
        ("action_north", 0, 39, 35, 0x5E423B, false), // short rear hair
        ("action_north", 0, 35, 35, 0xB06C57, true), // ear beside hair
        ("action_north", 0, 39, 41, 0xB06C57, true), // upper back
        ("action_north", 0, 39, 44, 0xFF5C64, false), // tank fabric
        ("action_north", 0, 33, 42, 0x354647, false), // turned wristband
        ("action_north", 0, 32, 43, 0xB06C57, true), // hand by wristband
        ("action_north", 0, 46, 40, 0x63342A, true), // raised finger
        ("action_east", 1, 43, 41, 0xB06C57, true), // upper arm
        ("action_east", 1, 48, 42, 0xB06C57, true), // extended forearm
        ("action_east", 1, 48, 44, 0x491F1B, true), // extended finger shadow
        ("action_east", 1, 41, 40, 0xFF948B, false), // moving tank strap
        ("action_east", 1, 41, 44, 0xFF5C64, false), // moving tank fabric
        ("action_east", 1, 43, 50, 0xB06C57, true), // bare leg
        ("action_east", 1, 43, 52, 0x72B2D9, false), // sandal strap
        ("action_east", 1, 44, 53, 0xB06C57, true), // extended toe
        ("kiss_east", 2, 41, 35, 0x000000, false),  // closed eye
        ("kiss_east", 2, 46, 37, 0x854D3C, true),   // lip/cheek shading
        ("kiss_east", 2, 47, 37, 0x000000, false),  // kissing mouth outline
        ("kiss_east", 2, 42, 41, 0xB06C57, true),   // bare chest
        ("kiss_east", 2, 37, 53, 0xB06C57, true),   // raised foot
        ("kiss_east", 2, 36, 51, 0x6482AA, false),  // raised sandal strap
        ("kiss_east", 2, 40, 47, 0x72B2D9, false),  // shorts hem
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
        assert_eq!(changed, 2696);
    }

    // Practical controls for the two easy Beach mistakes: dropping a leg
    // component, or broadening the palette into shorts and sandal straps.
    let asset = "assets/animations/NPCs/Ryis/Sprites/Beach/spr_npc_ryis_beach_blink_south.png";
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
        .retain(|s| *s != json!([37, 48]));
    let omitted = apply_control("missing-shin", missing, map.clone());
    assert_eq!(omitted.get_pixel(38, 49).0, rgba(0xB06C57));
    assert_ne!(omitted.get_pixel(38, 49), correct.get_pixel(38, 49));
    let mut too_broad = map;
    too_broad.insert("#72B2D9".into(), json!("#FFFFFF"));
    let spilled = apply_control("clothing-spill", region, too_broad);
    assert_eq!(correct.get_pixel(37, 52).0, rgba(0x72B2D9));
    assert_eq!(spilled.get_pixel(37, 52).0, rgba(0xFFFFFF));
}
