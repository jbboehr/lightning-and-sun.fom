use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/valen-autumn-world-study and the retained Spring-everyday Valen bundle"]
fn valen_spring_standard_covers_skin_and_preserves_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-autumn-world-study");
    let profile_path = root.join("palettes/profiles/valen-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/valen-world-trial.json"));
    let cases: [(&str, &[usize]); 5] = [
        ("action_north", &[3, 0, 0, 0, 0, 2, 0]),
        ("action_south", &[52, 50, 50, 50, 50, 52, 55]),
        ("action_east", &[43, 43, 42, 43, 42, 43, 48]),
        ("sleep_east", &[50]),
        ("kiss_east", &[45, 51, 56, 57]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xFBD3A7, 0xEFA67A, 0xC37555, 0x762E21];
    // Frame-local coordinates are zero-based. Reaching hands, closed eyelids and ankles
    // are skin; goggles, eye lines, cuffs and coat folds remain their original materials.
    let landmarks = [
        ("action_north", 0, 46, 41, 0xC37555, true), // far hand above coat
        ("action_north", 0, 46, 42, 0xC37555, true),
        ("action_north", 0, 32, 44, 0xFBD3A7, true), // single near fingertip
        ("action_north", 0, 45, 41, 0x837CA0, false), // sleeve beside far hand
        ("action_north", 0, 34, 44, 0xFCF5F1, false), // coat beside near finger
        ("action_north", 0, 38, 40, 0x6E578A, false), // hair covers neck
        ("action_north", 5, 46, 41, 0xC37555, true), // returning far hand
        ("action_north", 5, 46, 42, 0xC37555, true),
        ("action_south", 0, 39, 42, 0xEFA67A, true), // neck over vest
        ("action_south", 0, 34, 45, 0xDCDBED, false), // cuff above wrist
        ("action_south", 0, 33, 46, 0xC37555, true), // near wrist
        ("action_south", 0, 34, 46, 0x9F86A6, false), // sleeve beside wrist
        ("action_south", 0, 33, 47, 0xFBD3A7, true), // near hand
        ("action_south", 0, 34, 48, 0x762E21, true), // near hand outline
        ("action_south", 0, 45, 46, 0xC37555, true), // far hand
        ("action_south", 0, 46, 47, 0x762E21, true), // far hand outline
        ("action_south", 0, 37, 46, 0x635659, false), // belt
        ("action_south", 0, 43, 47, 0x848484, false), // accessory
        ("action_south", 0, 37, 51, 0xEFA67A, true), // ankle
        ("action_south", 2, 37, 46, 0xFBD3A7, true), // moving hand across torso
        ("action_south", 2, 38, 47, 0x762E21, true), // moving hand outline
        ("action_south", 2, 39, 47, 0xFBD3A7, true), // moving fingertip
        ("action_south", 2, 37, 45, 0x9F86A6, false), // cuff over moving hand
        ("action_south", 2, 45, 46, 0x762E21, true), // far hand edge
        ("action_south", 2, 41, 51, 0xFBD3A7, true), // opposite ankle
        ("action_east", 1, 47, 43, 0xFBD3A7, true),  // extended palm
        ("action_east", 1, 49, 43, 0xEFA67A, true),  // isolated fingertip
        ("action_east", 1, 48, 45, 0x762E21, true),  // hand outline
        ("action_east", 1, 49, 45, 0xC37555, true),  // hand shade
        ("action_east", 1, 46, 43, 0xDCDBED, false), // cuff next to palm
        ("action_east", 1, 43, 42, 0xDCDBED, false), // sleeve highlight
        ("action_east", 1, 45, 44, 0x9F86A6, false), // sleeve midtone
        ("action_east", 1, 38, 51, 0xEFA67A, true),  // near ankle
        ("action_east", 1, 42, 51, 0xC37555, true),  // isolated far ankle shadow
        ("action_east", 1, 43, 51, 0xFBD3A7, true),  // far ankle highlight
        ("action_east", 1, 43, 52, 0xA8A5B5, false), // shoe below ankle
        ("action_east", 2, 43, 41, 0xEFA67A, true),  // neck during return
        ("action_east", 2, 44, 45, 0xEFA67A, true),  // returning wrist
        ("action_east", 2, 45, 45, 0xFBD3A7, true),  // returning palm
        ("action_east", 2, 45, 46, 0x762E21, true),  // returning hand outline
        ("action_east", 2, 43, 45, 0xDCDBED, false), // returning cuff
        ("sleep_east", 0, 40, 34, 0xFBD3A7, true),   // forehead
        ("sleep_east", 0, 37, 35, 0xC37555, true),   // eyelid corner
        ("sleep_east", 0, 38, 37, 0x000000, false),  // closed eye line
        ("sleep_east", 0, 36, 36, 0xFBD3A7, true),   // ear
        ("sleep_east", 0, 43, 39, 0xEFA67A, true),   // supporting hand edge beside face
        ("sleep_east", 0, 44, 39, 0x762E21, true),   // supporting hand outline
        ("sleep_east", 0, 44, 40, 0xFBD3A7, true),   // supporting palm
        ("sleep_east", 0, 42, 40, 0x9F86A6, false),  // sleeve beside hand
        ("sleep_east", 0, 42, 41, 0xDCDBED, false),  // light cuff
        ("kiss_east", 2, 43, 34, 0xFBD3A7, true),    // tilted forehead
        ("kiss_east", 2, 40, 35, 0xC37555, true),    // closed eyelid shade
        ("kiss_east", 2, 41, 36, 0x000000, false),   // closed eye line
        ("kiss_east", 2, 47, 35, 0x762E21, true),    // far face outline
        ("kiss_east", 2, 39, 36, 0xFBD3A7, true),    // ear
        ("kiss_east", 2, 43, 40, 0xC37555, true),    // chin
        ("kiss_east", 2, 42, 41, 0xEFA67A, true),    // neck
        ("kiss_east", 2, 35, 44, 0xC37555, true),    // near wrist
        ("kiss_east", 2, 35, 45, 0xFBD3A7, true),    // near hand
        ("kiss_east", 2, 36, 46, 0x762E21, true),    // hand outline
        ("kiss_east", 2, 37, 44, 0xDCDBED, false),   // cuff beside wrist
        ("kiss_east", 2, 39, 45, 0x635659, false),   // belt
        ("kiss_east", 2, 41, 51, 0xC37555, true),    // far ankle
        ("kiss_east", 2, 42, 51, 0xFBD3A7, true),    // far ankle highlight
        ("kiss_east", 3, 45, 38, 0x762E21, true),    // final face edge
        ("kiss_east", 3, 44, 39, 0xEFA67A, true),    // final cheek near mouth
        ("kiss_east", 3, 41, 41, 0xC37555, true),    // final chin
        ("kiss_east", 3, 43, 37, 0x000000, false),   // final closed eye
    ];
    let temp = tempfile::tempdir().unwrap();
    for preset in set["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let recipe = json!({
            "regions": profile["regions"],
            "color_groups": profile["color_groups"],
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
        let targets = [8, 9, 10, 4].map(|i| {
            rgba(u32::from_str_radix(&preset["colors"][i].as_str().unwrap()[1..7], 16).unwrap())
        });
        let mut changed = 0;

        for (name, counts) in cases {
            let prefix = "spring";
            let asset = format!(
                "assets/animations/NPCs/Valen/Sprites/Spring/spr_npc_valen_{prefix}_{name}.png"
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
                    _ => pixel.0,
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
        assert_eq!(changed, 927);

        for r in &profile["regions"].as_array().unwrap()[..109] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-balor-valen-eiland-spring-actions-trial/characters/valen/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: omit isolated hand/ankle components and deliberately pollute the cuff palette.
    let asset = "assets/animations/NPCs/Valen/Sprites/Spring/spr_npc_valen_spring_action_east.png";
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
    let apply_control =
        |id: &str, region: Value, map: serde_json::Map<String, Value>, groups: Value| {
            let recipe = temp.path().join(format!("control-{id}.json"));
            fs::write(
                &recipe,
                serde_json::to_vec(&if region.is_null() {
                    json!({"rgba_map":map,"color_groups":groups})
                } else {
                    json!({"regions":[region],"rgba_map":map,"color_groups":groups})
                })
                .unwrap(),
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
    let correct = apply_control(
        "correct",
        region.clone(),
        map.clone(),
        profile["color_groups"].clone(),
    );
    for (id, point, color) in [
        ("missing-palm", [127, 43], 0xFBD3A7),
        ("missing-fingertip", [129, 43], 0xEFA67A),
        ("missing-hand-outline", [128, 45], 0x762E21),
        ("missing-ankle", [122, 51], 0xC37555),
    ] {
        let mut missing = region.clone();
        missing["seeds"]
            .as_array_mut()
            .unwrap()
            .retain(|s| *s != json!(point));
        let omitted = apply_control(id, missing, map.clone(), profile["color_groups"].clone());
        assert_eq!(omitted.get_pixel(point[0], point[1]).0, rgba(color));
        assert_ne!(
            omitted.get_pixel(point[0], point[1]),
            correct.get_pixel(point[0], point[1])
        );
    }
    let mut too_broad = map;
    too_broad.insert("#DCDBED".into(), json!("#FF0000"));
    let mut broad_groups = profile["color_groups"].clone();
    broad_groups
        .as_array_mut()
        .unwrap()
        .push(json!(["#DCDBED"]));
    let spilled = apply_control("cuff-spill", Value::Null, too_broad, broad_groups);
    for point in [[126, 43], [123, 42]] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(0xDCDBED));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(0xFF0000));
    }
}
