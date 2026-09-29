use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-winter-finish-study and the retained Autumn-injured Juniper bundle"]
fn juniper_winter_world_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-winter-finish-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("idle_north", &[0]),
        ("idle_south", &[22]),
        ("idle_east", &[19]),
        ("walk_north", &[0, 0, 0, 0]),
        ("walk_south", &[22, 21, 22, 21]),
        ("walk_east", &[19, 19, 19, 19]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        ("idle_north", 0, &[(33, 45), (46, 45)]),
        (
            "idle_south",
            0,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (33, 45),
                (46, 45),
            ],
        ),
        (
            "idle_east",
            0,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (36, 40),
                (44, 40),
                (36, 43),
                (34, 45),
                (44, 45),
            ],
        ),
        ("walk_north", 0, &[(33, 45), (46, 45)]),
        ("walk_north", 1, &[(33, 46)]),
        ("walk_north", 2, &[(33, 45), (46, 45)]),
        ("walk_north", 3, &[(46, 46)]),
        (
            "walk_south",
            0,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (33, 45),
                (46, 45),
            ],
        ),
        (
            "walk_south",
            1,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (33, 46),
                (44, 45),
                (44, 46),
                (45, 46),
            ],
        ),
        (
            "walk_south",
            2,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (33, 45),
                (46, 45),
            ],
        ),
        (
            "walk_south",
            3,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (35, 45),
                (34, 46),
                (35, 46),
                (46, 46),
            ],
        ),
        (
            "walk_east",
            0,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (36, 40),
                (44, 40),
                (36, 43),
                (34, 45),
                (44, 45),
            ],
        ),
        (
            "walk_east",
            1,
            &[
                (39, 34),
                (42, 34),
                (38, 35),
                (43, 35),
                (36, 41),
                (44, 41),
                (33, 45),
                (35, 45),
                (44, 45),
                (45, 45),
                (46, 45),
            ],
        ),
        (
            "walk_east",
            2,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (36, 40),
                (44, 40),
                (36, 43),
                (34, 45),
                (44, 45),
            ],
        ),
        (
            "walk_east",
            3,
            &[
                (39, 34),
                (42, 34),
                (38, 35),
                (43, 35),
                (36, 41),
                (44, 41),
                (35, 46),
                (44, 46),
            ],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("idle_north", 0, 33, 45, 0xBC8B43, false), // cuff edge, no exposed skin
        ("idle_north", 0, 46, 45, 0xBC8B43, false), // opposite cuff edge
        ("idle_north", 0, 32, 46, 0xDD426C, false), // glove
        ("idle_south", 0, 38, 33, 0xE3BF7F, false), // circlet corner
        ("idle_south", 0, 37, 34, 0xBC8B43, false), // circlet border
        ("idle_south", 0, 40, 33, 0x3CB9D8, false), // blue circlet gem
        ("idle_south", 0, 37, 36, 0xC2B9BE, false), // eye detail
        ("idle_south", 0, 35, 36, 0xEFD89A, true),  // ear beside hair
        ("idle_south", 0, 39, 36, 0xE3BF7F, true),  // nose
        ("idle_south", 0, 37, 39, 0x763F21, true),  // cheek shadow
        ("idle_south", 0, 39, 40, 0xBC8B43, true),  // chin
        ("idle_south", 0, 35, 40, 0xBC8B43, false), // shoulder decoration
        ("idle_south", 0, 44, 40, 0xBC8B43, false), // opposite shoulder decoration
        ("idle_south", 0, 39, 42, 0x645049, false), // warm bodice detail
        ("idle_south", 0, 39, 43, 0x836C64, false), // warm bodice highlight
        ("idle_south", 0, 37, 47, 0x645049, false), // leg clothing
        ("idle_south", 0, 37, 48, 0x836C64, false), // leg clothing highlight
        ("idle_south", 0, 33, 45, 0xBC8B43, false), // cuff border
        ("idle_south", 0, 33, 47, 0x60285E, false), // glove shadow
        ("idle_east", 0, 39, 33, 0xE3BF7F, false),  // circlet corner
        ("idle_east", 0, 38, 34, 0xBC8B43, false),  // circlet border
        ("idle_east", 0, 36, 36, 0xEFD89A, true),   // ear
        ("idle_east", 0, 43, 36, 0xA59DA2, false),  // far eye
        ("idle_east", 0, 43, 38, 0xE3BF7F, true),   // face at silhouette
        ("idle_east", 0, 38, 39, 0x763F21, true),   // jaw shadow
        ("idle_east", 0, 36, 40, 0xBC8B43, false),  // shoulder decoration
        ("idle_east", 0, 36, 43, 0xBC8B43, false),  // upper cuff border
        ("idle_east", 0, 44, 45, 0xBC8B43, false),  // opposite cuff border
        ("walk_north", 0, 33, 45, 0xBC8B43, false), // near cuff
        ("walk_north", 1, 33, 46, 0xBC8B43, false), // moving near cuff
        ("walk_north", 2, 46, 45, 0xBC8B43, false), // far cuff
        ("walk_north", 3, 46, 46, 0xBC8B43, false), // moving far cuff
        ("walk_south", 0, 35, 36, 0xEFD89A, true),  // ear
        ("walk_south", 0, 35, 40, 0xBC8B43, false), // shoulder trim
        ("walk_south", 1, 35, 37, 0xEFD89A, true),  // moving ear
        ("walk_south", 1, 42, 40, 0x763F21, true),  // moving cheek shadow
        ("walk_south", 1, 35, 41, 0xBC8B43, false), // shifted shoulder trim
        ("walk_south", 1, 44, 45, 0xBC8B43, false), // raised cuff corner
        ("walk_south", 1, 44, 46, 0xBC8B43, false), // raised cuff underside
        ("walk_south", 2, 44, 36, 0xEFD89A, true),  // opposite ear
        ("walk_south", 2, 44, 40, 0xBC8B43, false), // opposite shoulder trim
        ("walk_south", 3, 37, 40, 0x763F21, true),  // opposite cheek shadow
        ("walk_south", 3, 35, 45, 0xBC8B43, false), // opposite raised cuff
        ("walk_south", 3, 34, 46, 0xBC8B43, false), // opposite cuff underside
        ("walk_east", 0, 38, 39, 0x763F21, true),   // jaw
        ("walk_east", 0, 36, 43, 0xBC8B43, false),  // upper cuff
        ("walk_east", 1, 36, 37, 0xEFD89A, true),   // moving ear
        ("walk_east", 1, 38, 40, 0x763F21, true),   // moving jaw
        ("walk_east", 1, 44, 45, 0x763F21, false),  // same dark shade in cuff
        ("walk_east", 1, 46, 45, 0x763F21, false),  // opposite cuff corner
        ("walk_east", 1, 45, 45, 0xBC8B43, false),  // cuff between dark corners
        ("walk_east", 1, 35, 45, 0xBC8B43, false),  // near cuff corner
        ("walk_east", 2, 43, 38, 0xE3BF7F, true),   // outer face
        ("walk_east", 2, 44, 45, 0xBC8B43, false),  // far cuff
        ("walk_east", 3, 36, 37, 0xEFD89A, true),   // final ear
        ("walk_east", 3, 35, 46, 0xBC8B43, false),  // final near cuff
        ("walk_east", 3, 44, 46, 0xBC8B43, false),  // final far cuff
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
        let targets = [0, 1, 12, 4, 13, 14, 15, 16].map(|i| {
            rgba(u32::from_str_radix(&preset["colors"][i].as_str().unwrap()[1..7], 16).unwrap())
        });
        let mut changed = 0;
        for name in ["idle_north", "walk_north"] {
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Winter/spr_npc_juniper_winter_{name}.png"
            );
            assert_eq!(
                fs::read(original.join(&asset)).unwrap(),
                fs::read(output.join(&asset)).unwrap(),
                "covered North art must remain byte-identical"
            );
            let region = profile["regions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["asset"] == asset)
                .unwrap();
            assert_eq!(region["seeds"], json!([]));
        }

        for (name, counts) in cases {
            let prefix = "winter";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Winter/spr_npc_juniper_{prefix}_{name}.png"
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
                    Some(i)
                        if !clothing.iter().any(|(case, f, pts)| {
                            *case == name && *f == x / 80 && pts.contains(&(x % 80, y))
                        }) =>
                    {
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
        assert_eq!(changed, 203);

        for r in &profile["regions"].as_array().unwrap()[..239] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-march-autumn-injured-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: missing cheek/ear components and unrestricted color matching.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Winter/spr_npc_juniper_winter_idle_south.png";
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
        ("missing-cheek-shadow", [37, 39], 0x763F21),
        ("missing-ear", [35, 36], 0xEFD89A),
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
    let spilled = apply_control(
        "unrestricted-colors",
        Value::Null,
        map,
        profile["color_groups"].clone(),
    );
    for (point, source, target) in [
        ([38, 33], 0xE3BF7F, 0x7F9FBD), // circlet
        ([35, 40], 0xBC8B43, 0x6687AD), // shoulder decoration
        ([33, 45], 0xBC8B43, 0x6687AD), // glove cuff
    ] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(source));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(target));
    }
}
