use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-beach-swim-study and the retained Winter-world Juniper bundle"]
fn juniper_winter_actions_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-beach-swim-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 11] = [
        ("blink_south", &[22, 26, 22]),
        ("blink_east", &[19, 23, 19]),
        ("sit_north", &[0]),
        ("sit_south", &[22]),
        ("sit_east", &[19]),
        ("eat_north", &[0, 0, 0]),
        ("eat_south", &[22, 20, 13, 26, 22]),
        ("eat_east", &[19, 15, 12, 19, 19]),
        ("drink_north", &[0, 0, 0]),
        ("drink_south", &[22, 25, 22]),
        ("drink_east", &[19, 19, 19]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "blink_south",
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
            "blink_south",
            1,
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
            "blink_south",
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
            "blink_east",
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
            "blink_east",
            1,
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
            "blink_east",
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
        ("sit_north", 0, &[(34, 45)]),
        (
            "sit_south",
            0,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (34, 45),
                (45, 45),
            ],
        ),
        (
            "sit_east",
            0,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (36, 40),
                (44, 40),
                (35, 45),
            ],
        ),
        ("eat_north", 0, &[(34, 45), (46, 45)]),
        ("eat_north", 1, &[]),
        ("eat_north", 2, &[(34, 45), (46, 45)]),
        (
            "eat_south",
            0,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (35, 43),
                (36, 45),
                (45, 45),
            ],
        ),
        (
            "eat_south",
            1,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (36, 44),
                (37, 44),
                (36, 45),
                (36, 46),
                (45, 45),
            ],
        ),
        ("eat_south", 2, &[(38, 32), (41, 32), (44, 40), (45, 45)]),
        (
            "eat_south",
            3,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (45, 45),
            ],
        ),
        (
            "eat_south",
            4,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (34, 45),
                (45, 45),
            ],
        ),
        (
            "eat_east",
            0,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (36, 40),
                (44, 40),
                (39, 42),
                (40, 42),
                (41, 42),
                (40, 43),
            ],
        ),
        (
            "eat_east",
            1,
            &[
                (40, 33),
                (43, 33),
                (39, 34),
                (44, 34),
                (37, 40),
                (41, 41),
                (42, 41),
                (43, 41),
                (40, 42),
                (41, 42),
                (42, 42),
            ],
        ),
        (
            "eat_east",
            2,
            &[
                (39, 32),
                (42, 32),
                (36, 40),
                (41, 41),
                (43, 41),
                (40, 42),
                (41, 42),
                (42, 42),
            ],
        ),
        (
            "eat_east",
            3,
            &[
                (39, 34),
                (42, 34),
                (38, 35),
                (43, 35),
                (36, 41),
                (44, 41),
                (40, 43),
                (40, 44),
                (41, 44),
            ],
        ),
        (
            "eat_east",
            4,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (36, 40),
                (44, 40),
                (39, 43),
                (40, 43),
                (39, 44),
                (40, 45),
            ],
        ),
        ("drink_north", 0, &[(34, 45), (46, 45)]),
        ("drink_north", 1, &[]),
        ("drink_north", 2, &[(34, 45), (46, 45)]),
        (
            "drink_south",
            0,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (45, 45),
            ],
        ),
        (
            "drink_south",
            1,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (44, 41),
                (35, 43),
                (45, 45),
            ],
        ),
        (
            "drink_south",
            2,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (45, 45),
            ],
        ),
        (
            "drink_east",
            0,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (36, 40),
                (44, 40),
                (38, 44),
            ],
        ),
        (
            "drink_east",
            1,
            &[
                (37, 33),
                (40, 33),
                (36, 34),
                (41, 34),
                (42, 40),
                (34, 41),
                (36, 43),
                (36, 44),
                (37, 44),
            ],
        ),
        (
            "drink_east",
            2,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (36, 40),
                (44, 40),
                (38, 44),
            ],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("blink_south", 0, 38, 33, 0xE3BF7F, false), // circlet corner
        ("blink_south", 0, 40, 33, 0x3CB9D8, false), // cyan gemstone
        ("blink_south", 0, 39, 36, 0xE3BF7F, true),  // nose
        ("blink_south", 1, 37, 35, 0xE3BF7F, true),  // exposed skin above closed eye
        ("blink_south", 1, 35, 36, 0xEFD89A, true),  // ear beside cosmetics
        ("blink_south", 2, 33, 45, 0xBC8B43, false), // cuff
        ("blink_south", 2, 39, 42, 0x645049, false), // warm bodice
        ("blink_east", 0, 36, 40, 0xBC8B43, false),  // shoulder decoration
        ("blink_east", 1, 38, 35, 0xE3BF7F, true),   // eyelid skin
        ("blink_east", 1, 36, 36, 0xEFD89A, true),   // ear
        ("blink_east", 2, 36, 43, 0xBC8B43, false),  // raised cuff edge
        ("sit_north", 0, 34, 45, 0xBC8B43, false),   // cuff above covered hand
        ("sit_south", 0, 35, 40, 0xBC8B43, false),   // shoulder jewelry
        ("sit_south", 0, 37, 39, 0x763F21, true),    // cheek shadow
        ("sit_south", 0, 34, 45, 0xBC8B43, false),   // seated cuff
        ("sit_east", 0, 38, 39, 0x763F21, true),     // jaw shadow
        ("sit_east", 0, 35, 45, 0xBC8B43, false),    // side cuff
        ("eat_north", 0, 46, 45, 0xBC8B43, false),   // cuff beyond hair
        ("eat_north", 2, 34, 45, 0xBC8B43, false),   // opposite cuff
        ("eat_south", 0, 35, 43, 0xBC8B43, false),   // upper cuff corner
        ("eat_south", 0, 36, 45, 0xBC8B43, false),   // lower cuff corner
        ("eat_south", 0, 35, 36, 0xEFD89A, true),    // ear
        ("eat_south", 1, 37, 44, 0xEFD89A, false),   // bright cuff highlight
        ("eat_south", 1, 36, 45, 0xE3BF7F, false),   // cuff midtone
        ("eat_south", 1, 37, 40, 0x763F21, true),    // moving cheek
        ("eat_south", 2, 38, 32, 0xE3BF7F, false),   // circlet above open mouth
        ("eat_south", 2, 39, 36, 0x9E2626, false),   // mouth interior
        ("eat_south", 2, 41, 35, 0x410808, false),   // mouth rim
        ("eat_south", 2, 37, 35, 0xEFD89A, true),    // cheek alongside mouth
        ("eat_south", 2, 44, 40, 0xBC8B43, false),   // shoulder gem edge
        ("eat_south", 3, 37, 38, 0xE3BF7F, true),    // cheek after swallowing
        ("eat_south", 3, 35, 41, 0xBC8B43, false),   // moving shoulder decoration
        ("eat_south", 4, 34, 45, 0xBC8B43, false),   // restored cuff
        ("eat_south", 4, 39, 40, 0xBC8B43, true),    // restored chin
        ("eat_east", 0, 39, 42, 0xE3BF7F, false),    // cuff highlight beside glove
        ("eat_east", 0, 40, 42, 0xEFD89A, false),    // brightest cuff highlight
        ("eat_east", 0, 38, 39, 0x763F21, true),     // jaw above cuff
        ("eat_east", 1, 41, 41, 0xEFD89A, false),    // moving cuff highlight
        ("eat_east", 1, 40, 42, 0xE3BF7F, false),    // moving cuff lower edge
        ("eat_east", 1, 39, 39, 0x763F21, true),     // shifted jaw
        ("eat_east", 2, 40, 36, 0x9E2626, false),    // open mouth interior
        ("eat_east", 2, 38, 36, 0xEFD89A, true),     // face beside mouth
        ("eat_east", 2, 41, 41, 0xE3BF7F, false),    // tilted cuff beside gold
        ("eat_east", 2, 41, 42, 0xEFD89A, false),    // tilted cuff highlight
        ("eat_east", 3, 40, 43, 0xEFD89A, false),    // cuff during return
        ("eat_east", 3, 41, 44, 0xBC8B43, false),    // cuff underside
        ("eat_east", 3, 38, 40, 0x763F21, true),     // jaw during return
        ("eat_east", 4, 39, 43, 0xEFD89A, false),    // resting cuff highlight
        ("eat_east", 4, 39, 44, 0xE3BF7F, false),    // resting cuff midtone
        ("eat_east", 4, 40, 45, 0xBC8B43, false),    // resting cuff border
        ("drink_north", 0, 46, 45, 0xBC8B43, false), // cuff beyond hair
        ("drink_north", 2, 34, 45, 0xBC8B43, false), // opposite cuff
        ("drink_south", 0, 35, 40, 0xBC8B43, false), // shoulder decoration
        ("drink_south", 0, 37, 39, 0x763F21, true),  // cheek
        ("drink_south", 1, 37, 36, 0xE3BF7F, true),  // closed-eye skin
        ("drink_south", 1, 35, 43, 0xBC8B43, false), // raised cuff corner
        ("drink_south", 2, 45, 45, 0xBC8B43, false), // opposite cuff
        ("drink_south", 2, 35, 36, 0xEFD89A, true),  // restored ear
        ("drink_east", 0, 38, 44, 0xBC8B43, false),  // cuff below glove
        ("drink_east", 0, 36, 36, 0xEFD89A, true),   // ear
        ("drink_east", 1, 34, 36, 0xEFD89A, true),   // tilted ear
        ("drink_east", 1, 36, 39, 0x763F21, true),   // tilted jaw
        ("drink_east", 1, 36, 43, 0x763F21, false),  // same dark shade on cuff
        ("drink_east", 1, 36, 44, 0xBC8B43, false),  // cuff lower edge
        ("drink_east", 1, 37, 44, 0xE8B171, false),  // established skin alias used in cuff
        ("drink_east", 2, 38, 44, 0xBC8B43, false),  // restored cuff
        ("drink_east", 2, 38, 39, 0x763F21, true),   // restored jaw
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
        for name in ["sit_north", "eat_north", "drink_north"] {
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
        assert_eq!(changed, 485);

        for r in &profile["regions"].as_array().unwrap()[..245] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-winter-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: missing cheek/ear components and unrestricted color matching.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Winter/spr_npc_juniper_winter_blink_south.png";
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
