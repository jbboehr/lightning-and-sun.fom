use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/valen-spring-finish-study and the retained portrait Valen bundle"]
fn valen_spring_world_covers_skin_and_preserves_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-spring-finish-study");
    let profile_path = root.join("palettes/profiles/valen-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/valen-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("idle_north", &[0]),
        ("idle_south", &[55]),
        ("idle_east", &[48]),
        ("walk_north", &[0, 0, 0, 0]),
        ("walk_south", &[55, 55, 55, 55]),
        ("walk_east", &[48, 49, 48, 46]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xFBD3A7, 0xEFA67A, 0xC37555, 0x762E21];
    // Literal source landmarks distinguish skin from goggles, hair, eyes, coat and shoes.
    // Frame numbers and local coordinates are zero-based. North views have no exposed skin.
    let landmarks = [
        ("idle_south", 0, 35, 27, 0x6D4C12, false), // goggles dark rim
        ("idle_south", 0, 35, 28, 0xBD8E19, false), // gold goggles
        ("idle_south", 0, 36, 28, 0xC2D5E4, false), // blue lens
        ("idle_south", 0, 35, 31, 0xD6C1DD, false), // hair
        ("idle_south", 0, 39, 33, 0xEFA67A, true),  // forehead midtone
        ("idle_south", 0, 41, 33, 0xC37555, true),  // forehead shadow
        ("idle_south", 0, 39, 34, 0xFBD3A7, true),  // face highlight
        ("idle_south", 0, 37, 35, 0xC2B9BE, false), // eye white shade
        ("idle_south", 0, 37, 36, 0xECF0E9, false), // eye white
        ("idle_south", 0, 38, 36, 0x000000, false), // pupil
        ("idle_south", 0, 35, 36, 0xFBD3A7, true),  // left ear
        ("idle_south", 0, 44, 36, 0xFBD3A7, true),  // opposite ear
        ("idle_south", 0, 36, 38, 0x762E21, true),  // deep jaw outline
        ("idle_south", 0, 39, 40, 0xC37555, true),  // chin
        ("idle_south", 0, 39, 41, 0xEFA67A, true),  // neck above vest
        ("idle_south", 0, 34, 41, 0xFCF5F1, false), // white coat
        ("idle_south", 0, 38, 41, 0x473C52, false), // dark vest
        ("idle_south", 0, 34, 44, 0xDCDBED, false), // pale cuff
        ("idle_south", 0, 33, 45, 0xC37555, true),  // wrist next to cuff
        ("idle_south", 0, 32, 46, 0xFBD3A7, true),  // hand
        ("idle_south", 0, 33, 47, 0x762E21, true),  // hand outline
        ("idle_south", 0, 37, 45, 0x635659, false), // belt
        ("idle_south", 0, 38, 46, 0xA8A5B5, false), // pants
        ("idle_south", 0, 43, 46, 0x848484, false), // accessory
        ("idle_south", 0, 37, 51, 0xEFA67A, true),  // ankle under pants
        ("idle_south", 0, 38, 51, 0xFBD3A7, true),  // ankle highlight
        ("idle_south", 0, 37, 52, 0xA8A5B5, false), // shoe top
        ("idle_south", 0, 37, 53, 0xFCF5F1, false), // shoe
        ("idle_east", 0, 40, 33, 0xEFA67A, true),   // forehead
        ("idle_east", 0, 36, 36, 0xFBD3A7, true),   // ear beside hair
        ("idle_east", 0, 44, 36, 0xC37555, true),   // far face edge
        ("idle_east", 0, 44, 37, 0x762E21, true),   // far jaw outline
        ("idle_east", 0, 40, 41, 0xEFA67A, true),   // neck
        ("idle_east", 0, 34, 46, 0xFBD3A7, true),   // near hand
        ("idle_east", 0, 44, 46, 0xEFA67A, true),   // far hand
        ("idle_east", 0, 38, 51, 0xC37555, true),   // ankle shade
        ("idle_east", 0, 39, 51, 0xFBD3A7, true),   // ankle highlight
        ("idle_east", 0, 39, 53, 0xFCF5F1, false),  // shoe
        ("walk_south", 1, 35, 37, 0xFBD3A7, true),  // bobbing ear
        ("walk_south", 1, 33, 46, 0xC37555, true),  // moving wrist
        ("walk_south", 1, 44, 46, 0x762E21, true),  // far hand
        ("walk_south", 1, 37, 50, 0xC37555, true),  // raised ankle
        ("walk_south", 1, 41, 52, 0xFBD3A7, true),  // lower ankle
        ("walk_south", 1, 41, 54, 0xFCF5F1, false), // lowered shoe
        ("walk_south", 3, 35, 46, 0x762E21, true),  // returning hand outline
        ("walk_south", 3, 44, 46, 0xFBD3A7, true),  // opposite hand
        ("walk_south", 3, 41, 50, 0xEFA67A, true),  // lifted opposite ankle
        ("walk_south", 3, 38, 52, 0xFBD3A7, true),  // planted ankle
        ("walk_east", 1, 32, 46, 0xFBD3A7, true),   // outstretched near hand
        ("walk_east", 1, 44, 46, 0x762E21, true),   // far hand outline
        ("walk_east", 1, 34, 45, 0xDCDBED, false),  // moving cuff
        ("walk_east", 1, 42, 51, 0x762E21, true),   // lifted ankle outline
        ("walk_east", 1, 41, 52, 0xEFA67A, true),   // lower ankle
        ("walk_east", 1, 43, 52, 0xFCF5F1, false),  // shoe beyond ankle
        ("walk_east", 3, 35, 47, 0xFBD3A7, true),   // returning near hand
        ("walk_east", 3, 44, 47, 0x762E21, true),   // far hand edge
        ("walk_east", 3, 37, 51, 0xFBD3A7, true),   // forward ankle
        ("walk_east", 3, 42, 51, 0xC37555, true),   // trailing ankle
        ("walk_east", 3, 43, 51, 0xEFA67A, true),   // trailing ankle light
        ("walk_east", 3, 36, 52, 0xFCF5F1, false),  // forward shoe
        ("idle_north", 0, 35, 27, 0x6D4C12, false), // exposed goggles are not skin
        ("walk_north", 0, 44, 28, 0xBD8E19, false), // gold rim behind head
        ("walk_north", 0, 38, 40, 0x6E578A, false), // hair covers neck
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
        assert_eq!(changed, 514);

        for r in &profile["regions"].as_array().unwrap()[..92] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-wedding-finish-trial/characters/valen/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: omitted forehead, ear, wrist and ankle components, plus polluted gold.
    let asset = "assets/animations/NPCs/Valen/Sprites/Spring/spr_npc_valen_spring_idle_south.png";
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
        ("missing-forehead", [39, 33], 0xEFA67A),
        ("missing-ear", [35, 36], 0xFBD3A7),
        ("missing-wrist", [33, 45], 0xC37555),
        ("missing-ankle", [38, 51], 0xFBD3A7),
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
    too_broad.insert("#BD8E19".into(), json!("#FFFFFF"));
    let mut broad_groups = profile["color_groups"].clone();
    broad_groups
        .as_array_mut()
        .unwrap()
        .push(json!(["#BD8E19"]));
    let spilled = apply_control("goggle-spill", Value::Null, too_broad, broad_groups);
    for point in [[35, 28], [35, 29]] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(0xBD8E19));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(0xFFFFFF));
    }
}

#[test]
fn valen_world_extends_portrait_roles_without_changing_them() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let read =
        |p: &str| -> Value { serde_json::from_slice(&fs::read(root.join(p)).unwrap()).unwrap() };
    let old = read("palettes/profiles/valen-portraits.json");
    let profile = read("palettes/profiles/valen-world-trial.json");
    assert_eq!(old["regions"].as_array().unwrap().len(), 92);
    assert_eq!(profile["regions"].as_array().unwrap().len(), 132);
    assert_eq!(
        &profile["regions"].as_array().unwrap()[..92],
        old["regions"].as_array().unwrap()
    );
    assert_eq!(
        &profile["source_colors"].as_array().unwrap()[..8],
        old["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &profile["color_groups"].as_array().unwrap()[..3],
        old["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        profile["source_colors"].as_array().unwrap()[8..],
        json!(["#FBD3A7", "#EFA67A", "#C37555"]).as_array().unwrap()[..]
    );
    assert_eq!(
        profile["color_groups"].as_array().unwrap()[3..],
        json!([["#FBD3A7"], ["#EFA67A"], ["#C37555"]])
            .as_array()
            .unwrap()[..]
    );
    let old_set = read("palettes/sets/valen-portraits-trial.json");
    let set = read("palettes/sets/valen-world-trial.json");
    let old_style = read("palettes/stylized/valen-portraits.json");
    let style = read("palettes/stylized/valen-world-trial.json");
    assert_eq!(set["profile"], "../profiles/valen-world-trial.json");
    assert_eq!(style["profile"], set["profile"]);
    assert_eq!(style["rgba_map"].as_object().unwrap().len(), 11);
    for (key, value) in old_style["rgba_map"].as_object().unwrap() {
        assert_eq!(style["rgba_map"][key], *value);
    }
    assert_eq!(set["presets"].as_array().unwrap().len(), 4);
    for i in 0..4 {
        assert_eq!(set["presets"][i]["id"], old_set["presets"][i]["id"]);
        assert_eq!(set["presets"][i]["label"], old_set["presets"][i]["label"]);
        assert_eq!(
            &set["presets"][i]["colors"].as_array().unwrap()[..8],
            old_set["presets"][i]["colors"].as_array().unwrap()
        );
        assert_eq!(set["presets"][i]["colors"].as_array().unwrap().len(), 11);
        for (alias, role) in [("#FBD3A7", 0), ("#EFA67A", 1), ("#C37555", 2)] {
            assert_eq!(
                set["presets"][i]["colors"][8 + role],
                old_set["presets"][i]["colors"][role]
            );
            assert_eq!(
                style["rgba_map"][alias],
                set["presets"][0]["colors"][8 + role]
            );
        }
    }
}
