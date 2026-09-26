use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the local Ryis Beach idle/walk corpus in extracted/ryis-wedding-finish-study"]
fn ryis_beach_skin_covers_bare_limbs_and_preserves_clothing_and_wristbands() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-wedding-finish-study");
    let profile_path = root.join("palettes/profiles/ryis-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/ryis-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("idle_east", &[91]),
        ("idle_north", &[79]),
        ("idle_south", &[102]),
        ("walk_east", &[91, 94, 91, 88]),
        ("walk_north", &[79, 70, 79, 71]),
        ("walk_south", &[102, 94, 102, 93]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // Beach exposes chest, back, shoulders, arms, hands, legs and toes.
    // All four reviewed shades are skin in these strips; the rear hair,
    // pink tank, blue shorts/sandal straps and dark wristbands are separate.
    let skin = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    // Frame numbers are zero-based; x coordinates are within the frame.
    let landmarks = [
        ("idle_south", 0, 38, 31, 0x63342A, true), // forehead under hair
        ("idle_south", 0, 37, 36, 0xECF0E9, false), // eye white
        ("idle_south", 0, 39, 41, 0xB06C57, true), // exposed chest
        ("idle_south", 0, 37, 40, 0xFF948B, false), // tank strap
        ("idle_south", 0, 39, 44, 0xFF5C64, false), // tank fabric
        ("idle_south", 0, 34, 44, 0xB06C57, true), // bare forearm
        ("idle_south", 0, 45, 44, 0x354647, false), // wristband highlight
        ("idle_south", 0, 44, 44, 0x332727, false), // wristband shadow
        ("idle_south", 0, 45, 45, 0xB06C57, true), // hand below wristband
        ("idle_south", 0, 46, 46, 0x491F1B, true), // finger shadow
        ("idle_south", 0, 38, 47, 0x72B2D9, false), // shorts hem
        ("idle_south", 0, 38, 49, 0xB06C57, true), // shin
        ("idle_south", 0, 37, 52, 0x72B2D9, false), // sandal strap
        ("idle_south", 0, 37, 53, 0xB06C57, true), // exposed toes
        ("idle_north", 0, 39, 35, 0x5E423B, false), // short rear hair
        ("idle_north", 0, 35, 35, 0xB06C57, true), // ear beside hair
        ("idle_north", 0, 39, 41, 0xB06C57, true), // exposed upper back
        ("idle_north", 0, 39, 44, 0xFF5C64, false), // back of tank
        ("idle_north", 0, 32, 44, 0x354647, false), // reversed wristband
        ("idle_north", 0, 32, 45, 0xB06C57, true), // hand below wristband
        ("idle_east", 0, 44, 45, 0x854D3C, true),  // far hand
        ("idle_east", 0, 45, 44, 0x354647, false), // far wristband
        ("idle_east", 0, 38, 50, 0x63342A, true),  // side shin shadow
        ("idle_east", 0, 40, 53, 0xB06C57, true),  // extended toe
        ("walk_south", 1, 44, 44, 0x354647, false), // moving wristband
        ("walk_south", 1, 44, 45, 0x491F1B, true), // adjacent moving hand
        ("walk_south", 1, 37, 51, 0x6482AA, false), // lifted sandal strap
        ("walk_south", 1, 37, 52, 0x854D3C, true), // lifted toes
        ("walk_south", 1, 41, 54, 0xB06C57, true), // planted toes
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
        assert_eq!(changed, 1326);
    }

    // Practical controls for the two easy Beach mistakes: dropping a leg
    // component, or broadening the palette into shorts and sandal straps.
    let asset = "assets/animations/NPCs/Ryis/Sprites/Beach/spr_npc_ryis_beach_idle_south.png";
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
