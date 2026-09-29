use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/valen-spring-reactions-study and the retained first-world Valen bundle"]
fn valen_spring_actions_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-spring-reactions-study");
    let profile_path = root.join("palettes/profiles/valen-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/valen-world-trial.json"));
    let cases: [(&str, &[usize]); 11] = [
        ("blink_east", &[51, 61, 51]),
        ("blink_south", &[58, 67, 58]),
        ("sit_north", &[0]),
        ("sit_south", &[51]),
        ("sit_east", &[41]),
        ("eat_north", &[2, 2, 2]),
        ("eat_south", &[50, 55, 55, 70, 51]),
        ("eat_east", &[38, 43, 45, 49, 40]),
        ("drink_north", &[2, 2, 2]),
        ("drink_south", &[52, 62, 52]),
        ("drink_east", &[41, 48, 41]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xFBD3A7, 0xEFA67A, 0xC37555, 0x762E21];
    // Frame-local coordinates are zero-based. North eating/drinking expose two hand pixels,
    // while North sitting has no exposed skin. The mouths' red shades are not skin.
    let landmarks = [
        ("blink_east", 1, 40, 34, 0xFBD3A7, true), // expanded forehead highlight
        ("blink_east", 1, 37, 35, 0xC37555, true), // closed eyelid shade
        ("blink_east", 1, 38, 35, 0xEFA67A, true), // eyelid midtone
        ("blink_east", 1, 44, 35, 0x762E21, true), // far face outline
        ("blink_east", 1, 38, 37, 0x000000, false), // closed eye line
        ("blink_south", 1, 37, 36, 0xFBD3A7, true), // left closed eyelid
        ("blink_south", 1, 42, 36, 0xFBD3A7, true), // right closed eyelid
        ("blink_south", 1, 37, 37, 0x000000, false), // closed eye line
        ("blink_south", 1, 43, 35, 0xC37555, true), // far eyelid edge
        ("sit_south", 0, 33, 46, 0xEFA67A, true),  // near seated hand
        ("sit_south", 0, 45, 47, 0x762E21, true),  // opposite hand outline
        ("sit_south", 0, 37, 48, 0xEFA67A, true),  // ankle below pants
        ("sit_south", 0, 38, 48, 0xFBD3A7, true),  // ankle highlight
        ("sit_south", 0, 37, 49, 0xA8A5B5, false), // shoe top
        ("sit_south", 0, 37, 50, 0xFCF5F1, false), // shoe
        ("sit_south", 0, 37, 45, 0x635659, false), // belt
        ("sit_east", 0, 34, 46, 0xFBD3A7, true),   // near hand
        ("sit_east", 0, 35, 47, 0x762E21, true),   // hand outline
        ("sit_east", 0, 44, 46, 0xFBD3A7, true),   // far ankle
        ("sit_east", 0, 42, 47, 0xFBD3A7, true),   // near ankle
        ("sit_east", 0, 44, 48, 0xFCF5F1, false),  // shoe
        ("sit_east", 0, 42, 46, 0xA8A5B5, false),  // pants over ankle
        ("sit_north", 0, 38, 40, 0x6E578A, false), // hair covers neck
        ("sit_north", 0, 44, 28, 0xBD8E19, false), // gold goggles are not skin
        ("eat_north", 0, 46, 42, 0xC37555, true),  // hand beyond coat
        ("eat_north", 0, 46, 43, 0xC37555, true),
        ("eat_north", 0, 45, 42, 0x837CA0, false), // adjacent sleeve
        ("eat_north", 1, 44, 41, 0xC37555, true),  // lifted hand next to hair
        ("eat_north", 1, 45, 41, 0xC37555, true),
        ("eat_north", 1, 44, 42, 0x837CA0, false), // adjacent coat
        ("eat_north", 2, 46, 42, 0xC37555, true),  // returning hand
        ("drink_north", 0, 46, 43, 0xC37555, true),
        ("drink_north", 1, 44, 41, 0xC37555, true),
        ("drink_north", 2, 46, 42, 0xC37555, true),
        ("eat_south", 0, 36, 46, 0xFBD3A7, true), // near hand reaching down
        ("eat_south", 0, 37, 46, 0xEFA67A, true), // isolated hand edge
        ("eat_south", 0, 35, 47, 0xC37555, true), // hand edge near ankle
        ("eat_south", 0, 35, 46, 0x9F86A6, false), // cuff beside hand
        ("eat_south", 1, 39, 40, 0x9E2626, false), // small open mouth
        ("eat_south", 1, 38, 47, 0xFBD3A7, true), // moving hand above ankle
        ("eat_south", 1, 36, 46, 0xDCDBED, false), // cuff above moving hand
        ("eat_south", 2, 36, 34, 0xC37555, true), // isolated eyelid corner
        ("eat_south", 2, 37, 35, 0xFBD3A7, true), // skin at mouth corner
        ("eat_south", 2, 38, 35, 0x410808, false), // mouth outline
        ("eat_south", 2, 39, 37, 0x9E2626, false), // open mouth interior
        ("eat_south", 2, 43, 37, 0xEFA67A, true), // cheek against mouth
        ("eat_south", 2, 36, 40, 0x762E21, true), // raised hand outline
        ("eat_south", 2, 37, 40, 0xFBD3A7, true), // raised finger
        ("eat_south", 2, 38, 42, 0xC37555, true), // hand shadow
        ("eat_south", 2, 35, 43, 0x9F86A6, false), // coat below raised hand
        ("eat_east", 0, 43, 42, 0xFBD3A7, true),  // extended hand
        ("eat_east", 0, 44, 42, 0xEFA67A, true),  // fingertip shade
        ("eat_east", 0, 41, 42, 0xDCDBED, false), // extended cuff
        ("eat_east", 0, 42, 43, 0x9F86A6, false), // wrist cloth
        ("drink_south", 0, 34, 42, 0xFBD3A7, true), // lowered hand
        ("drink_south", 0, 35, 44, 0xC37555, true), // hand shadow
        ("drink_south", 1, 35, 40, 0xFBD3A7, true), // raised hand beside chin
        ("drink_south", 1, 36, 42, 0xC37555, true), // wrist above cuff
        ("drink_south", 1, 34, 43, 0xDCDBED, false), // cuff
        ("drink_south", 1, 34, 42, 0x9F86A6, false), // cloth beside hand
        ("drink_east", 1, 34, 36, 0xFBD3A7, true), // ear in turned head
        ("drink_east", 1, 35, 36, 0xC37555, true), // shaded ear
        ("drink_east", 1, 42, 37, 0x762E21, true), // far cheek outline
        ("drink_east", 1, 39, 40, 0xFBD3A7, true), // raised palm
        ("drink_east", 1, 40, 42, 0xC37555, true), // wrist shade
        ("drink_east", 1, 37, 40, 0x010101, false), // near-black outline
        ("drink_east", 1, 38, 42, 0xDCDBED, false), // light cuff beside wrist
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
        assert_eq!(changed, 1242);

        for r in &profile["regions"].as_array().unwrap()[..98] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-balor-valen-eiland-world-trial/characters/valen/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: omit small skin components; prove a polluted mouth map changes protected red.
    let asset = "assets/animations/NPCs/Valen/Sprites/Spring/spr_npc_valen_spring_eat_south.png";
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
        ("missing-finger", [197, 40], 0xFBD3A7),
        ("missing-eyelid-corner", [196, 34], 0xC37555),
        ("missing-ankle", [38, 48], 0xFBD3A7),
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
    too_broad.insert("#9E2626".into(), json!("#FFFFFF"));
    let mut broad_groups = profile["color_groups"].clone();
    broad_groups
        .as_array_mut()
        .unwrap()
        .push(json!(["#9E2626"]));
    let spilled = apply_control("mouth-spill", Value::Null, too_broad, broad_groups);
    for point in [[199, 37], [198, 36]] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(0x9E2626));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(0xFFFFFF));
    }
}
