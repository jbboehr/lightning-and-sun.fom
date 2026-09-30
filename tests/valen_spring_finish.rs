use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/valen-summer-world-study and the retained Spring-writing Valen bundle"]
fn valen_spring_finish_covers_skin_and_preserves_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-summer-world-study");
    let profile_path = root.join("palettes/profiles/valen-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/valen-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("heal_start_east", &[63]),
        ("heal_loop_east", &[47, 43, 46, 47]),
        ("heal_end_east", &[63]),
        ("charm_start_south", &[48, 50, 48]),
        ("charm_loop_south", &[47, 47, 47, 48]),
        ("charm_end_south", &[60, 67]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xFBD3A7, 0xEFA67A, 0xC37555, 0x762E21, 0xB65932];
    // Frame-local coordinates are zero-based. The independent material inventory
    // separates hands, including a rare existing shadow role, from held gold and blue props.
    let landmarks = [
        ("heal_start_east", 0, 38, 34, 0xEFA67A, true), // forehead below goggles
        ("heal_start_east", 0, 34, 29, 0xBD8E19, false), // gold goggles
        ("heal_start_east", 0, 35, 29, 0xC2D5E4, false), // glass lens
        ("heal_start_east", 0, 34, 28, 0x6D4C12, false), // gold shadow
        ("heal_start_east", 0, 32, 47, 0xFBD3A7, true), // hand beside healing prop
        ("heal_start_east", 0, 33, 48, 0x762E21, true), // hand outline
        ("heal_start_east", 0, 37, 48, 0xFFC962, false), // amber prop highlight
        ("heal_start_east", 0, 37, 47, 0xCF8039, false), // amber prop midtone
        ("heal_start_east", 0, 36, 48, 0x934D00, false), // prop shadow
        ("heal_start_east", 0, 36, 49, 0x7D391E, false), // darkest prop edge
        ("heal_start_east", 0, 33, 50, 0xA6B7E5, false), // blue prop portion
        ("heal_start_east", 0, 32, 50, 0x596CA1, false), // shaded blue portion
        ("heal_start_east", 0, 42, 46, 0xEFA67A, true), // other hand
        ("heal_start_east", 0, 43, 48, 0x762E21, true), // other fingertip edge
        ("heal_loop_east", 0, 42, 45, 0xFBD3A7, true),  // hand gripping prop
        ("heal_loop_east", 0, 43, 46, 0x762E21, true),  // fingertip outline
        ("heal_loop_east", 0, 40, 47, 0xC37555, true),  // wrist shadow
        ("heal_loop_east", 0, 45, 47, 0xFFC962, false), // amber prop tip
        ("heal_loop_east", 0, 44, 48, 0x7D391E, false), // prop outline differs from skin
        ("heal_loop_east", 0, 41, 49, 0xA6B7E5, false), // blue section below hand
        ("heal_loop_east", 1, 46, 39, 0xFBD3A7, true),  // raised fingers
        ("heal_loop_east", 1, 47, 39, 0xC37555, true),  // raised fingertip shadow
        ("heal_loop_east", 1, 48, 40, 0x762E21, true),  // raised fingertip outline
        ("heal_loop_east", 1, 49, 38, 0xFFC962, false), // raised prop amber
        ("heal_loop_east", 1, 50, 39, 0xCF8039, false), // raised prop midtone
        ("heal_loop_east", 1, 51, 41, 0x7D391E, false), // raised prop edge
        ("heal_loop_east", 1, 50, 42, 0xA6B7E5, false), // raised blue section
        ("heal_loop_east", 1, 43, 40, 0xDCDBED, false), // sleeve beside wrist
        ("heal_loop_east", 1, 37, 50, 0xEFA67A, true),  // small exposed rear ankle
        ("heal_loop_east", 1, 38, 51, 0xEFA67A, true),  // isolated ankle corner
        ("heal_loop_east", 1, 42, 50, 0xFBD3A7, true),  // front ankle
        ("heal_loop_east", 2, 44, 44, 0xFBD3A7, true),  // returning hand
        ("heal_loop_east", 2, 47, 46, 0xFFC962, false), // moving amber prop
        ("heal_end_east", 0, 32, 47, 0xFBD3A7, true),   // lowered hand
        ("heal_end_east", 0, 37, 48, 0xFFC962, false),  // lowered prop
        ("charm_start_south", 0, 32, 46, 0xBD8E19, false), // held goggles
        ("charm_start_south", 0, 33, 46, 0xC2D5E4, false), // held lens
        ("charm_start_south", 0, 32, 45, 0x6D4C12, false), // gold edge
        ("charm_start_south", 0, 44, 48, 0xFBD3A7, true), // free hand
        ("charm_start_south", 1, 46, 44, 0xFBD3A7, true), // hand holding pale cloth
        ("charm_start_south", 1, 47, 43, 0xC37555, true), // wrist shadow
        ("charm_start_south", 1, 48, 44, 0x547BB0, false), // cloth edge
        ("charm_start_south", 1, 49, 44, 0xC3D1DD, false), // cloth shade
        ("charm_start_south", 1, 49, 45, 0xF5F5F5, false), // cloth highlight
        ("charm_start_south", 1, 36, 48, 0xEFA67A, true), // fingers below goggles
        ("charm_start_south", 1, 37, 48, 0x762E21, true), // gripping finger outline
        ("charm_start_south", 2, 47, 44, 0xC37555, true), // turning wrist
        ("charm_start_south", 2, 48, 45, 0xEFA67A, true), // small exposed hand corner
        ("charm_start_south", 2, 45, 46, 0xF5F5F5, false), // flowing cloth
        ("charm_loop_south", 0, 45, 46, 0xFBD3A7, true), // narrow visible hand
        ("charm_loop_south", 0, 46, 45, 0xC37555, true), // hand shade next to cuff
        ("charm_loop_south", 0, 40, 45, 0x547BB0, false), // cloth blue edge
        ("charm_loop_south", 0, 41, 45, 0xC3D1DD, false), // cloth shade
        ("charm_loop_south", 0, 41, 46, 0xF5F5F5, false), // cloth highlight
        ("charm_loop_south", 0, 35, 45, 0xBD8E19, false), // gold lens rim
        ("charm_loop_south", 0, 36, 48, 0xEFA67A, true), // other gripping hand
        ("charm_end_south", 0, 33, 48, 0xB65932, true), // rare hand shadow, existing portrait role
        ("charm_end_south", 0, 34, 47, 0xEFA67A, true), // hand below goggles
        ("charm_end_south", 0, 37, 48, 0x762E21, true), // finger outline
        ("charm_end_south", 0, 38, 48, 0xFBD3A7, true), // visible fingertip
        ("charm_end_south", 0, 31, 45, 0xBD8E19, false), // held gold rim remains distinct
        ("charm_end_south", 0, 33, 46, 0xBD8E19, false), // gold next to rare hand shade
        ("charm_end_south", 0, 46, 44, 0xFBD3A7, true), // free hand
        ("charm_end_south", 0, 43, 46, 0x848484, false), // clothing detail
        ("charm_end_south", 1, 32, 47, 0xFBD3A7, true), // return to relaxed hand
        ("charm_end_south", 1, 35, 29, 0xBD8E19, false), // goggles return to head
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
        let targets = [8, 9, 10, 4, 3].map(|i| {
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
        assert_eq!(changed, 771);

        for r in &profile["regions"].as_array().unwrap()[..126] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-balor-valen-eiland-spring-specials-trial/characters/valen/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls omit small gripping-hand components, including the rare shadow, and spill into gold.
    let asset = "assets/animations/NPCs/Valen/Sprites/Spring/spr_npc_valen_specialanimation_spring_charm_end_south.png";
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
        ("missing-rare-hand-shadow", [33, 48], 0xB65932),
        ("missing-free-hand", [46, 44], 0xFBD3A7),
        ("missing-gripping-hand", [34, 47], 0xEFA67A),
        ("missing-finger-outline", [37, 48], 0x762E21),
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
    too_broad.insert("#BD8E19".into(), json!("#FF0000"));
    let mut broad_groups = profile["color_groups"].clone();
    broad_groups
        .as_array_mut()
        .unwrap()
        .push(json!(["#BD8E19"]));
    let spilled = apply_control("gold-spill", Value::Null, too_broad, broad_groups);
    for point in [[31, 45], [33, 46]] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(0xBD8E19));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(0xFF0000));
    }
}
