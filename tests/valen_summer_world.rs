use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/valen-summer-world-study and the retained completed-Spring Valen bundle"]
fn valen_summer_world_covers_skin_and_preserves_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-summer-world-study");
    let profile_path = root.join("palettes/profiles/valen-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/valen-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("idle_north", &[27]),
        ("idle_south", &[68]),
        ("idle_east", &[57]),
        ("walk_north", &[27, 22, 27, 21]),
        ("walk_south", &[68, 62, 68, 63]),
        ("walk_east", &[57, 59, 57, 55]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xFBD3A7, 0xEFA67A, 0xC37555, 0x762E21];
    // Frame-local coordinates are zero-based. Exposed arms and sandal skin are
    // independently distinguished from the pink shirt, tan trousers and blue accessories.
    let landmarks = [
        ("idle_north", 0, 35, 44, 0xFBD3A7, true), // exposed arm below sleeve
        ("idle_north", 0, 34, 44, 0xC37555, true), // arm shadow
        ("idle_north", 0, 32, 46, 0xFBD3A7, true), // near hand
        ("idle_north", 0, 33, 47, 0x762E21, true), // finger outline
        ("idle_north", 0, 44, 44, 0xFBD3A7, true), // other arm above bracelet
        ("idle_north", 0, 44, 45, 0x6D70AF, false), // blue bracelet edge
        ("idle_north", 0, 45, 45, 0xAEB0DF, false), // bracelet highlight
        ("idle_north", 0, 45, 46, 0xFBD3A7, true), // hand below bracelet
        ("idle_north", 0, 38, 42, 0xCD8D8B, false), // shirt, not skin
        ("idle_north", 0, 38, 41, 0xAB615F, false), // shirt shade
        ("idle_north", 0, 37, 41, 0x542F3A, false), // collar shadow
        ("idle_north", 0, 37, 45, 0x3F3935, false), // belt edge
        ("idle_north", 0, 38, 45, 0x72665E, false), // belt body
        ("idle_north", 0, 38, 49, 0xE3DACA, false), // trouser highlight
        ("idle_north", 0, 37, 49, 0xAFA190, false), // trouser shade
        ("idle_north", 0, 37, 51, 0xEFA67A, true), // ankle above strap
        ("idle_north", 0, 38, 51, 0xFBD3A7, true),
        ("idle_north", 0, 37, 52, 0x6264A0, false), // sandal strap
        ("idle_north", 0, 37, 53, 0xEFA67A, true),  // heel below strap
        ("idle_north", 0, 38, 53, 0xFBD3A7, true),
        ("idle_south", 0, 39, 33, 0xEFA67A, true), // forehead
        ("idle_south", 0, 35, 28, 0xBD8E19, false), // gold goggles
        ("idle_south", 0, 36, 28, 0xC2D5E4, false), // goggles glass
        ("idle_south", 0, 37, 35, 0xC2B9BE, false), // eye shade
        ("idle_south", 0, 37, 36, 0xECF0E9, false), // eye white
        ("idle_south", 0, 38, 41, 0xFBD3A7, true), // neck above collar
        ("idle_south", 0, 39, 42, 0xFBD3A7, true), // small neckline opening
        ("idle_south", 0, 38, 42, 0xCD8D8B, false), // shirt beside neckline
        ("idle_south", 0, 36, 44, 0x542F3A, false), // sleeve boundary
        ("idle_south", 0, 35, 44, 0xFBD3A7, true), // arm beside sleeve
        ("idle_south", 0, 34, 45, 0xAEB0DF, false), // wristband remains pale blue
        ("idle_south", 0, 32, 46, 0xFBD3A7, true), // hand below band
        ("idle_south", 0, 37, 52, 0x6264A0, false), // sandal band
        ("idle_south", 0, 37, 53, 0xFBD3A7, true), // toes
        ("idle_east", 0, 39, 41, 0xFBD3A7, true),  // side neck
        ("idle_east", 0, 40, 42, 0xFBD3A7, true),  // side neckline opening
        ("idle_east", 0, 35, 44, 0xC37555, true),  // near arm shade
        ("idle_east", 0, 35, 45, 0xAEB0DF, false), // wristband
        ("idle_east", 0, 34, 46, 0xFBD3A7, true),  // near palm
        ("idle_east", 0, 44, 45, 0xEFA67A, true),  // small far hand
        ("idle_east", 0, 38, 51, 0xC37555, true),  // ankle shadow
        ("idle_east", 0, 38, 52, 0x6264A0, false), // foot strap
        ("idle_east", 0, 40, 53, 0xFBD3A7, true),  // toe at silhouette
        ("walk_north", 1, 35, 45, 0xFBD3A7, true), // swinging arm
        ("walk_north", 1, 45, 46, 0xC37555, true), // far hand shade
        ("walk_north", 1, 44, 46, 0x6D70AF, false), // turning wristband
        ("walk_north", 1, 37, 51, 0x6264A0, false), // lifted foot strap
        ("walk_north", 1, 37, 52, 0xC37555, true), // lifted heel
        ("walk_north", 1, 41, 54, 0xFBD3A7, true), // planted foot below strap
        ("walk_north", 3, 34, 45, 0xEFA67A, true), // returning arm edge
        ("walk_north", 3, 35, 46, 0x762E21, true), // returning hand outline
        ("walk_north", 3, 45, 46, 0xAEB0DF, false), // other bracelet highlight
        ("walk_north", 3, 44, 48, 0xFBD3A7, true), // other fingertips
        ("walk_south", 1, 39, 43, 0xFBD3A7, true), // moving neckline
        ("walk_south", 1, 38, 43, 0xCD8D8B, false), // shirt beside moving neckline
        ("walk_south", 1, 34, 46, 0xAEB0DF, false), // moving bracelet
        ("walk_south", 1, 35, 48, 0xFBD3A7, true), // finger around bracelet
        ("walk_south", 3, 34, 45, 0x6D70AF, false), // narrow bracelet edge
        ("walk_south", 3, 35, 45, 0xC37555, true), // arm beside bracelet
        ("walk_south", 3, 35, 46, 0x6D70AF, false), // far bracelet edge
        ("walk_east", 1, 35, 45, 0xFBD3A7, true),  // exposed arm above wristband
        ("walk_east", 1, 34, 46, 0xAEB0DF, false), // angled wristband
        ("walk_east", 1, 34, 47, 0xFBD3A7, true),  // fingertip below band
        ("walk_east", 1, 37, 50, 0x762E21, true),  // lifted rear heel outline
        ("walk_east", 1, 37, 51, 0xC37555, true),  // lifted rear heel shade
        ("walk_east", 1, 41, 52, 0x6D70AF, false), // angled sandal strap
        ("walk_east", 1, 43, 52, 0xFBD3A7, true),  // front toe beside strap
        ("walk_east", 3, 36, 51, 0x6D70AF, false), // opposite foot strap
        ("walk_east", 3, 37, 51, 0xFBD3A7, true),  // ankle beside strap
        ("walk_east", 3, 36, 52, 0xFBD3A7, true),  // heel beside strap
        ("walk_east", 3, 38, 52, 0x6D70AF, false), // diagonal strap
        ("walk_east", 3, 45, 52, 0xFBD3A7, true),  // toe edge
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
            let prefix = "summer";
            let asset = format!(
                "assets/animations/NPCs/Valen/Sprites/Summer/spr_npc_valen_{prefix}_{name}.png"
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
        assert_eq!(changed, 738);

        for r in &profile["regions"].as_array().unwrap()[..132] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-balor-summer-valen-heal-eiland-magnify-trial/characters/valen/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls omit exposed arm, hand and sandal skin, then deliberately spill into the pink shirt.
    let asset = "assets/animations/NPCs/Valen/Sprites/Summer/spr_npc_valen_summer_idle_north.png";
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
        ("missing-upper-arm", [35, 44], 0xFBD3A7),
        ("missing-hand", [34, 45], 0xFBD3A7),
        ("missing-heel", [37, 53], 0xEFA67A),
        ("missing-finger-outline", [33, 47], 0x762E21),
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
    too_broad.insert("#CD8D8B".into(), json!("#FF0000"));
    let mut broad_groups = profile["color_groups"].clone();
    broad_groups
        .as_array_mut()
        .unwrap()
        .push(json!(["#CD8D8B"]));
    let spilled = apply_control("shirt-spill", Value::Null, too_broad, broad_groups);
    for point in [[38, 42], [40, 44]] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(0xCD8D8B));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(0xFF0000));
    }
}
