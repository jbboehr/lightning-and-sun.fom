use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-beach-actions-study and the retained Winter-actions Juniper bundle"]
fn juniper_winter_standard_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-beach-actions-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 5] = [
        ("action_north", &[0, 0, 0, 0, 0, 0, 0]),
        ("action_south", &[22, 22, 22, 22, 22, 22, 22]),
        ("action_east", &[19, 19, 19, 19, 19, 19, 19]),
        ("sleep_east", &[18]),
        ("kiss_east", &[19, 19, 23, 23]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        ("action_north", 0, &[(33, 43)]),
        ("action_north", 1, &[(34, 44)]),
        ("action_north", 2, &[(34, 44)]),
        ("action_north", 3, &[(34, 44)]),
        ("action_north", 4, &[(34, 44)]),
        ("action_north", 5, &[(33, 45)]),
        ("action_north", 6, &[(33, 45), (46, 45)]),
        (
            "action_south",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (34, 46),
                (44, 45),
                (44, 46),
                (45, 46),
            ],
        ),
        (
            "action_south",
            1,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (36, 42),
                (35, 43),
                (36, 43),
                (35, 45),
                (44, 45),
            ],
        ),
        (
            "action_south",
            2,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (36, 42),
                (37, 42),
                (36, 43),
                (37, 43),
                (36, 45),
                (44, 45),
            ],
        ),
        (
            "action_south",
            3,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (36, 42),
                (35, 43),
                (36, 43),
                (35, 45),
                (44, 45),
            ],
        ),
        (
            "action_south",
            4,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (36, 42),
                (37, 42),
                (36, 43),
                (37, 43),
                (36, 45),
                (44, 45),
            ],
        ),
        (
            "action_south",
            5,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (34, 46),
                (44, 45),
                (44, 46),
                (45, 46),
            ],
        ),
        (
            "action_south",
            6,
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
            "action_east",
            0,
            &[
                (40, 34),
                (43, 34),
                (39, 35),
                (44, 35),
                (37, 41),
                (45, 41),
                (38, 44),
                (39, 44),
                (38, 45),
                (38, 46),
                (40, 46),
            ],
        ),
        (
            "action_east",
            1,
            &[
                (42, 33),
                (45, 33),
                (41, 34),
                (46, 34),
                (39, 40),
                (47, 40),
                (42, 42),
                (43, 42),
                (44, 42),
                (43, 43),
                (44, 43),
                (47, 43),
                (45, 44),
            ],
        ),
        (
            "action_east",
            2,
            &[
                (42, 33),
                (45, 33),
                (41, 34),
                (46, 34),
                (39, 40),
                (47, 40),
                (41, 42),
                (42, 42),
                (41, 43),
                (42, 43),
                (42, 44),
                (43, 45),
            ],
        ),
        (
            "action_east",
            3,
            &[
                (42, 33),
                (45, 33),
                (41, 34),
                (46, 34),
                (39, 40),
                (47, 40),
                (42, 42),
                (43, 42),
                (44, 42),
                (43, 43),
                (44, 43),
                (47, 43),
                (45, 44),
            ],
        ),
        (
            "action_east",
            4,
            &[
                (42, 33),
                (45, 33),
                (41, 34),
                (46, 34),
                (39, 40),
                (47, 40),
                (41, 42),
                (42, 42),
                (41, 43),
                (42, 43),
                (42, 44),
                (43, 45),
            ],
        ),
        (
            "action_east",
            5,
            &[
                (40, 34),
                (43, 34),
                (39, 35),
                (44, 35),
                (37, 41),
                (45, 41),
                (38, 44),
                (39, 44),
                (38, 45),
                (38, 46),
                (40, 46),
            ],
        ),
        (
            "action_east",
            6,
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
            "sleep_east",
            0,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (37, 40),
                (40, 41),
                (41, 41),
                (44, 41),
                (40, 42),
                (41, 42),
            ],
        ),
        (
            "kiss_east",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (43, 41),
                (35, 44),
                (36, 44),
                (34, 46),
            ],
        ),
        (
            "kiss_east",
            1,
            &[
                (40, 34),
                (43, 34),
                (39, 35),
                (44, 35),
                (37, 41),
                (45, 41),
                (37, 44),
                (38, 44),
                (38, 45),
                (35, 46),
            ],
        ),
        (
            "kiss_east",
            2,
            &[
                (42, 33),
                (45, 33),
                (41, 34),
                (46, 34),
                (38, 40),
                (46, 40),
                (38, 43),
                (35, 44),
                (38, 44),
            ],
        ),
        (
            "kiss_east",
            3,
            &[
                (40, 34),
                (43, 34),
                (39, 35),
                (44, 35),
                (37, 41),
                (45, 41),
                (37, 44),
                (38, 44),
                (38, 45),
                (35, 46),
            ],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("action_north", 0, 33, 43, 0xBC8B43, false), // cuff beyond covered hand
        ("action_north", 2, 34, 44, 0xBC8B43, false), // moving cuff
        ("action_north", 6, 46, 45, 0xBC8B43, false), // opposite cuff
        ("action_south", 0, 35, 37, 0xEFD89A, true),  // shifted ear
        ("action_south", 0, 37, 40, 0x763F21, true),  // cheek shadow
        ("action_south", 0, 38, 34, 0xE3BF7F, false), // circlet corner
        ("action_south", 0, 35, 41, 0xBC8B43, false), // shoulder decoration
        ("action_south", 0, 34, 46, 0xBC8B43, false), // low cuff
        ("action_south", 1, 36, 43, 0xEFD89A, false), // bright moving cuff
        ("action_south", 1, 36, 42, 0xBC8B43, false), // cuff border
        ("action_south", 1, 37, 39, 0x763F21, true),  // restored cheek
        ("action_south", 2, 37, 42, 0xEFD89A, false), // raised cuff highlight
        ("action_south", 2, 36, 43, 0xE3BF7F, false), // cuff midtone
        ("action_south", 2, 35, 36, 0xEFD89A, true),  // ear above moving arm
        ("action_south", 3, 36, 43, 0xEFD89A, false), // return cuff highlight
        ("action_south", 4, 37, 43, 0xEFD89A, false), // repeated raised cuff
        ("action_south", 5, 37, 40, 0x763F21, true),  // lowered face shadow
        ("action_south", 6, 33, 45, 0xBC8B43, false), // resting cuff
        ("action_south", 6, 39, 40, 0xBC8B43, true),  // resting chin
        ("action_east", 0, 37, 37, 0xEFD89A, true),   // shifted ear
        ("action_east", 0, 39, 40, 0x763F21, true),   // jaw
        ("action_east", 0, 39, 44, 0xEFD89A, false),  // bright cuff
        ("action_east", 0, 38, 45, 0xE3BF7F, false),  // cuff midtone
        ("action_east", 1, 39, 36, 0xEFD89A, true),   // moving ear
        ("action_east", 1, 41, 39, 0x763F21, true),   // moving jaw
        ("action_east", 1, 43, 42, 0xEFD89A, false),  // raised cuff highlight
        ("action_east", 1, 44, 43, 0xE3BF7F, false),  // raised cuff midtone
        ("action_east", 1, 47, 43, 0xEFD89A, false),  // opposite cuff highlight
        ("action_east", 2, 42, 42, 0xEFD89A, false),  // bent cuff highlight
        ("action_east", 2, 43, 45, 0xE3BF7F, false),  // bent cuff edge
        ("action_east", 3, 45, 44, 0xBC8B43, false),  // cuff underside
        ("action_east", 4, 42, 43, 0xEFD89A, false),  // repeated cuff highlight
        ("action_east", 5, 41, 41, 0xBC8B43, true),   // lowered chin
        ("action_east", 6, 36, 43, 0xBC8B43, false),  // resting cuff
        ("action_east", 6, 38, 39, 0x763F21, true),   // resting jaw
        ("sleep_east", 0, 38, 35, 0xE3BF7F, true),    // skin above closed eye
        ("sleep_east", 0, 36, 36, 0xEFD89A, true),    // ear
        ("sleep_east", 0, 38, 38, 0xBC8B43, true),    // cheek above glove
        ("sleep_east", 0, 41, 39, 0xE3BF7F, true),    // face beside glove
        ("sleep_east", 0, 38, 36, 0xFC639B, false),   // cosmetics
        ("sleep_east", 0, 43, 39, 0xDD426C, false),   // pink glove by mouth
        ("sleep_east", 0, 40, 42, 0xE3BF7F, false),   // cuff under face
        ("sleep_east", 0, 41, 42, 0xEFD89A, false),   // bright cuff edge
        ("sleep_east", 0, 39, 33, 0xE3BF7F, false),   // circlet
        ("kiss_east", 0, 35, 37, 0xEFD89A, true),     // ear
        ("kiss_east", 0, 37, 40, 0x763F21, true),     // jaw
        ("kiss_east", 0, 36, 44, 0xEFD89A, false),    // cuff highlight
        ("kiss_east", 0, 37, 37, 0xC2B9BE, false),    // eye
        ("kiss_east", 1, 39, 36, 0xFC639B, false),    // raised cosmetics
        ("kiss_east", 1, 39, 40, 0x763F21, true),     // shifted jaw
        ("kiss_east", 1, 38, 44, 0xEFD89A, false),    // shifted cuff
        ("kiss_east", 2, 41, 37, 0xE3BF7F, true),     // closed-eye skin
        ("kiss_east", 2, 46, 37, 0xEFD89A, true),     // skin under far eye
        ("kiss_east", 2, 41, 35, 0xFC639B, false),    // cosmetics above closed eye
        ("kiss_east", 2, 38, 40, 0xBC8B43, false),    // shoulder ornament
        ("kiss_east", 2, 38, 44, 0xEFD89A, false),    // cuff above pink glove
        ("kiss_east", 2, 35, 45, 0xDD426C, false),    // pink glove
        ("kiss_east", 2, 43, 40, 0xBC8B43, true),     // chin
        ("kiss_east", 3, 39, 38, 0xE3BF7F, true),     // returning closed-eye skin
        ("kiss_east", 3, 44, 38, 0xEFD89A, true),     // far cheek
        ("kiss_east", 3, 38, 45, 0xBC8B43, false),    // returning cuff edge
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
        {
            let asset = "assets/animations/NPCs/Juniper/Sprites/Winter/spr_npc_juniper_winter_action_north.png";
            assert_eq!(
                fs::read(original.join(asset)).unwrap(),
                fs::read(output.join(asset)).unwrap(),
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
        assert_eq!(changed, 389);

        for r in &profile["regions"].as_array().unwrap()[..256] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-winter-actions-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: missing cheek/ear components and unrestricted color matching.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Winter/spr_npc_juniper_winter_action_south.png";
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
        ("missing-cheek-shadow", [37, 40], 0x763F21),
        ("missing-ear", [35, 37], 0xEFD89A),
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
        ([38, 34], 0xE3BF7F, 0x7F9FBD), // circlet
        ([35, 41], 0xBC8B43, 0x6687AD), // shoulder decoration
        ([34, 46], 0xBC8B43, 0x6687AD), // glove cuff
    ] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(source));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(target));
    }
}
