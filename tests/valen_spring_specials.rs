use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/valen-spring-finish-study and the retained Spring-reactions Valen bundle"]
fn valen_spring_specials_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-spring-finish-study");
    let profile_path = root.join("palettes/profiles/valen-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/valen-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("write_start_south", &[58, 45]),
        ("write_loop_south", &[47, 51, 48, 48]),
        ("write_end_south", &[45, 58]),
        ("write_sit_start_south", &[57, 43]),
        ("write_sit_loop_south", &[44, 45, 45, 45]),
        ("write_sit_end_south", &[43, 57]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xFBD3A7, 0xEFA67A, 0xC37555, 0x762E21];
    // Frame-local coordinates are zero-based. This source-material inventory distinguishes
    // fingers and wrists from the blue quill, pale writing surface and blue/red prop details.
    let landmarks = [
        ("write_start_south", 0, 39, 42, 0xFBD3A7, true), // neck above vest
        ("write_start_south", 0, 33, 42, 0x3F3F74, false), // dark blue quill
        ("write_start_south", 0, 34, 42, 0x181829, false), // quill shaft
        ("write_start_south", 0, 35, 42, 0xB6CBF7, false), // feather highlight
        ("write_start_south", 0, 35, 43, 0x799ADD, false), // feather blue
        ("write_start_south", 0, 32, 47, 0xFBD3A7, true), // hand outside upright quill
        ("write_start_south", 0, 32, 48, 0xFBD3A7, true),
        ("write_start_south", 0, 34, 48, 0x799ADD, false), // quill tip beside hand
        ("write_start_south", 0, 46, 44, 0xC3D1DD, false), // writing surface edge
        ("write_start_south", 0, 47, 44, 0xF5F5F5, false), // pale page
        ("write_start_south", 0, 46, 45, 0x6E8FBA, false), // blue prop edge
        ("write_start_south", 0, 48, 46, 0x6E8FBA, false), // blue prop body
        ("write_start_south", 0, 45, 47, 0x933D52, false), // red prop detail beside hand
        ("write_start_south", 0, 45, 48, 0xFBD3A7, true),  // supporting hand
        ("write_start_south", 1, 31, 44, 0xFBD3A7, true),  // raised near hand
        ("write_start_south", 1, 33, 45, 0x799ADD, false), // raised quill tip
        ("write_start_south", 1, 33, 46, 0xB6CBF7, false), // quill highlight near hand
        ("write_start_south", 1, 40, 40, 0xC37555, true),  // chin
        ("write_start_south", 1, 40, 41, 0xFBD3A7, true),  // neck
        ("write_start_south", 1, 43, 44, 0xC3D1DD, false), // moving page edge
        ("write_start_south", 1, 44, 44, 0xF5F5F5, false), // moving pale page
        ("write_start_south", 1, 42, 47, 0x933D52, false), // red detail above hand
        ("write_start_south", 1, 42, 48, 0xFBD3A7, true),  // moving support hand
        ("write_loop_south", 0, 38, 39, 0x762E21, true),   // jaw outline beside quill
        ("write_loop_south", 0, 36, 39, 0xB6CBF7, false),  // quill highlight by face
        ("write_loop_south", 0, 35, 40, 0x3F3F74, false),  // quill feather shadow
        ("write_loop_south", 0, 37, 40, 0x799ADD, false),  // quill feather highlight
        ("write_loop_south", 0, 35, 41, 0x181829, false),  // diagonal shaft
        ("write_loop_south", 0, 40, 40, 0xC37555, true),   // chin
        ("write_loop_south", 0, 40, 41, 0xFBD3A7, true),   // neck beside quill
        ("write_loop_south", 0, 36, 44, 0xDCDBED, false),  // cuff over writing hand
        ("write_loop_south", 0, 36, 45, 0xC37555, true),   // writing hand shadow
        ("write_loop_south", 0, 37, 45, 0xFBD3A7, true),   // writing finger
        ("write_loop_south", 0, 38, 46, 0xFBD3A7, true),   // moving fingertip
        ("write_loop_south", 0, 39, 45, 0x3F3F74, false),  // quill beside fingertip
        ("write_loop_south", 0, 41, 46, 0x933D52, false),  // red detail over support hand
        ("write_loop_south", 0, 41, 47, 0xEFA67A, true),   // small support-hand edge
        ("write_loop_south", 0, 41, 48, 0xFBD3A7, true),   // support hand
        ("write_loop_south", 0, 42, 43, 0xC3D1DD, false),  // page edge
        ("write_loop_south", 0, 43, 43, 0xF5F5F5, false),  // paper
        ("write_loop_south", 0, 44, 44, 0xC3D1DD, false),  // page shade
        ("write_loop_south", 0, 44, 45, 0x6E8FBA, false),  // blue stationery
        ("write_loop_south", 1, 35, 45, 0xC37555, true),   // shifted writing wrist
        ("write_loop_south", 1, 35, 46, 0xEFA67A, true),   // isolated wrist midtone
        ("write_loop_south", 1, 36, 47, 0xFBD3A7, true),   // shifted palm
        ("write_loop_south", 1, 38, 45, 0x799ADD, false),  // shifted blue quill
        ("write_loop_south", 1, 41, 47, 0xEFA67A, true),   // returning support hand
        ("write_sit_start_south", 0, 32, 47, 0xFBD3A7, true), // seated hand at quill
        ("write_sit_start_south", 0, 37, 48, 0xEFA67A, true), // exposed seated ankle
        ("write_sit_start_south", 0, 38, 48, 0xFBD3A7, true), // ankle highlight
        ("write_sit_start_south", 0, 37, 50, 0xFCF5F1, false), // shoe
        ("write_sit_start_south", 0, 34, 49, 0xB6CBF7, false), // quill tip beside ankle
        ("write_sit_start_south", 0, 45, 48, 0xFBD3A7, true), // seated supporting hand
        ("write_sit_start_south", 0, 46, 49, 0xEFA67A, true), // supporting fingertip shade
        ("write_sit_loop_south", 2, 34, 45, 0xC37555, true), // seated writing wrist
        ("write_sit_loop_south", 2, 35, 45, 0xFBD3A7, true), // seated writing finger
        ("write_sit_loop_south", 2, 36, 46, 0xFBD3A7, true), // seated fingertip
        ("write_sit_loop_south", 2, 38, 45, 0xB6CBF7, false), // quill highlight beside hand
        ("write_sit_loop_south", 2, 41, 47, 0x933D52, false), // red prop detail
        ("write_sit_loop_south", 2, 41, 48, 0xEFA67A, true), // support-hand shade
        ("write_sit_loop_south", 2, 41, 49, 0xFBD3A7, true), // support-hand highlight
        ("write_sit_loop_south", 2, 37, 48, 0xEFA67A, true), // visible ankle under crossed arm
        ("write_sit_loop_south", 2, 33, 48, 0xFCF5F1, false), // coat edge
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
            let prefix = "specialanimation_spring";
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
        assert_eq!(changed, 779);

        for r in &profile["regions"].as_array().unwrap()[..120] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-balor-valen-eiland-spring-reactions-trial/characters/valen/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: omit writing-hand and jaw components and deliberately pollute the page palette.
    let asset = "assets/animations/NPCs/Valen/Sprites/Spring/spr_npc_valen_specialanimation_spring_write_loop_south.png";
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
        ("missing-writing-finger", [37, 45], 0xFBD3A7),
        ("missing-writing-hand-shadow", [36, 45], 0xC37555),
        ("missing-support-hand", [121, 47], 0xEFA67A),
        ("missing-jaw", [38, 39], 0x762E21),
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
    too_broad.insert("#C3D1DD".into(), json!("#FF0000"));
    let mut broad_groups = profile["color_groups"].clone();
    broad_groups
        .as_array_mut()
        .unwrap()
        .push(json!(["#C3D1DD"]));
    let spilled = apply_control("page-spill", Value::Null, too_broad, broad_groups);
    for point in [[42, 43], [44, 44]] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(0xC3D1DD));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(0xFF0000));
    }
}
