use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-beach-finish-study and the retained Winter-specials Juniper bundle"]
fn juniper_winter_finish_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-beach-finish-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 5] = [
        ("hair_flip_south", &[8, 21, 23, 23, 23, 22]),
        ("snooze_south", &[24, 20, 24, 20, 24, 14, 14, 11, 22]),
        ("spell_cast_start_south", &[26, 26]),
        ("spell_cast_loop_south", &[34, 33, 30, 28]),
        ("spell_cast_end_south", &[26]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "hair_flip_south",
            0,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (33, 45)],
        ),
        (
            "hair_flip_south",
            1,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (36, 40),
                (43, 40),
                (46, 38),
                (34, 45),
            ],
        ),
        (
            "hair_flip_south",
            2,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (36, 40),
                (43, 40),
                (34, 44),
            ],
        ),
        (
            "hair_flip_south",
            3,
            &[
                (35, 34),
                (38, 34),
                (34, 35),
                (39, 35),
                (32, 41),
                (40, 41),
                (31, 45),
            ],
        ),
        (
            "hair_flip_south",
            4,
            &[
                (35, 34),
                (38, 34),
                (34, 35),
                (39, 35),
                (32, 41),
                (40, 41),
                (31, 45),
            ],
        ),
        (
            "hair_flip_south",
            5,
            &[
                (37, 33),
                (40, 33),
                (36, 34),
                (41, 34),
                (34, 40),
                (43, 40),
                (33, 45),
            ],
        ),
        (
            "snooze_south",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (35, 40), (33, 45)],
        ),
        (
            "snooze_south",
            1,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (35, 40), (33, 45)],
        ),
        (
            "snooze_south",
            2,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (35, 40), (33, 45)],
        ),
        (
            "snooze_south",
            3,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (35, 40), (33, 45)],
        ),
        (
            "snooze_south",
            4,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (35, 40), (33, 45)],
        ),
        (
            "snooze_south",
            5,
            &[(39, 36), (42, 36), (38, 37), (43, 37), (35, 41), (33, 46)],
        ),
        (
            "snooze_south",
            6,
            &[(39, 37), (42, 37), (38, 38), (43, 38), (35, 42), (33, 46)],
        ),
        ("snooze_south", 7, &[(35, 42), (33, 46)]),
        (
            "snooze_south",
            8,
            &[
                (38, 32),
                (41, 32),
                (37, 33),
                (42, 33),
                (35, 39),
                (44, 39),
                (33, 44),
            ],
        ),
        (
            "spell_cast_start_south",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (33, 46),
                (46, 46),
            ],
        ),
        (
            "spell_cast_start_south",
            1,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (35, 40), (44, 40)],
        ),
        (
            "spell_cast_loop_south",
            0,
            &[
                (38, 32),
                (41, 32),
                (37, 33),
                (42, 33),
                (35, 40),
                (44, 40),
                (33, 44),
                (46, 44),
            ],
        ),
        (
            "spell_cast_loop_south",
            1,
            &[
                (38, 32),
                (41, 32),
                (37, 33),
                (42, 33),
                (35, 40),
                (43, 40),
                (33, 44),
                (46, 44),
            ],
        ),
        (
            "spell_cast_loop_south",
            2,
            &[(41, 32), (42, 33), (35, 40), (44, 40), (33, 44), (46, 44)],
        ),
        (
            "spell_cast_loop_south",
            3,
            &[(41, 32), (42, 33), (35, 40), (44, 40), (33, 44), (46, 44)],
        ),
        (
            "spell_cast_end_south",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (33, 46),
                (46, 46),
            ],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("hair_flip_south", 0, 35, 37, 0xEFD89A, true), // ear beyond raised glove
        ("hair_flip_south", 0, 37, 40, 0x763F21, true), // small cheek shadow
        ("hair_flip_south", 0, 39, 41, 0xBC8B43, true), // chin below glove
        ("hair_flip_south", 0, 39, 37, 0xDD426C, false), // glove over face
        ("hair_flip_south", 0, 38, 34, 0xE3BF7F, false), // circlet
        ("hair_flip_south", 0, 33, 45, 0xBC8B43, false), // opposite cuff
        ("hair_flip_south", 1, 37, 37, 0xEFD89A, true), // revealed cheek
        ("hair_flip_south", 1, 42, 39, 0x763F21, true), // turned jaw
        ("hair_flip_south", 1, 46, 38, 0xBC8B43, false), // cuff next to head
        ("hair_flip_south", 1, 36, 40, 0xBC8B43, false), // shoulder decoration
        ("hair_flip_south", 2, 44, 36, 0xEFD89A, true), // revealed opposite ear
        ("hair_flip_south", 2, 34, 44, 0xBC8B43, false), // moving cuff
        ("hair_flip_south", 3, 41, 37, 0xEFD89A, true), // shifted ear
        ("hair_flip_south", 3, 39, 40, 0x763F21, true), // shifted jaw
        ("hair_flip_south", 3, 35, 34, 0xE3BF7F, false), // shifted circlet
        ("hair_flip_south", 3, 31, 45, 0xBC8B43, false), // shifted cuff
        ("hair_flip_south", 4, 34, 38, 0xEFD89A, true), // face during hair return
        ("hair_flip_south", 4, 40, 41, 0xBC8B43, false), // opposite shoulder
        ("hair_flip_south", 5, 34, 36, 0xEFD89A, true), // restored ear
        ("hair_flip_south", 5, 36, 39, 0x763F21, true), // restored jaw
        ("hair_flip_south", 5, 33, 45, 0xBC8B43, false), // restored cuff
        ("snooze_south", 0, 45, 36, 0xBC8B43, true),    // tiny shaded ear above pink glove
        ("snooze_south", 0, 36, 36, 0xEFD89A, true),    // opposite ear
        ("snooze_south", 0, 38, 35, 0xE3BF7F, true),    // skin above closed eye
        ("snooze_south", 0, 38, 36, 0xFC639B, false),   // cosmetics
        ("snooze_south", 0, 45, 38, 0xDD426C, false),   // glove against head
        ("snooze_south", 0, 35, 40, 0xBC8B43, false),   // shoulder decoration
        ("snooze_south", 0, 33, 45, 0xBC8B43, false),   // low cuff
        ("snooze_south", 1, 45, 36, 0xBC8B43, true),    // shaded ear while eyes open
        ("snooze_south", 1, 38, 35, 0xFC639B, false),   // cosmetics shift above eye
        ("snooze_south", 2, 38, 39, 0x763F21, true),    // blinking jaw
        ("snooze_south", 3, 41, 40, 0xBC8B43, true),    // chin during hold
        ("snooze_south", 4, 45, 36, 0xBC8B43, true),    // same shaded ear on final blink
        ("snooze_south", 5, 36, 38, 0xEFD89A, true),    // lowered ear
        ("snooze_south", 5, 38, 41, 0x763F21, true),    // lowered cheek shadow
        ("snooze_south", 5, 39, 36, 0xE3BF7F, false),   // lowered circlet
        ("snooze_south", 5, 35, 41, 0xBC8B43, false),   // shoulder beside lowered face
        ("snooze_south", 6, 36, 39, 0xEFD89A, true),    // further lowered ear
        ("snooze_south", 6, 38, 42, 0x763F21, true),    // further lowered cheek
        ("snooze_south", 6, 35, 42, 0xBC8B43, false),   // covered shoulder
        ("snooze_south", 7, 45, 40, 0xBC8B43, true),    // shaded ear in lowest pose
        ("snooze_south", 7, 43, 43, 0x763F21, true),    // small exposed jaw
        ("snooze_south", 7, 33, 46, 0xBC8B43, false),   // low cuff beside glove
        ("snooze_south", 8, 35, 35, 0xEFD89A, true),    // recovered ear
        ("snooze_south", 8, 37, 38, 0x763F21, true),    // recovered jaw
        ("snooze_south", 8, 44, 39, 0xBC8B43, false),   // recovered shoulder
        ("spell_cast_start_south", 0, 37, 36, 0xE3BF7F, true), // skin above closed eye
        ("spell_cast_start_south", 0, 37, 37, 0xFC639B, false), // cosmetics
        ("spell_cast_start_south", 0, 33, 46, 0xBC8B43, false), // cuff above covered hand
        ("spell_cast_start_south", 1, 35, 36, 0xEFD89A, true), // ear as arms rise
        ("spell_cast_start_south", 1, 37, 39, 0x763F21, true), // moving jaw
        ("spell_cast_start_south", 1, 44, 40, 0xBC8B43, false), // shoulder beside raised arms
        ("spell_cast_loop_south", 0, 38, 31, 0x763F21, true), // exposed forehead above circlet
        ("spell_cast_loop_south", 0, 39, 31, 0x763F21, true), // second forehead pixel
        ("spell_cast_loop_south", 0, 37, 32, 0x763F21, true), // separate forehead corner
        ("spell_cast_loop_south", 0, 38, 32, 0xE3BF7F, false), // adjacent circlet retained
        ("spell_cast_loop_south", 0, 35, 37, 0xEFD89A, true), // ear below flying hair
        ("spell_cast_loop_south", 0, 42, 39, 0x763F21, true), // jaw under opposite hair
        ("spell_cast_loop_south", 0, 33, 44, 0xBC8B43, false), // wrist jewelry below glove
        ("spell_cast_loop_south", 0, 32, 42, 0xDD426C, false), // casting glove
        ("spell_cast_loop_south", 0, 39, 42, 0x645049, false), // warm bodice
        ("spell_cast_loop_south", 1, 37, 32, 0x763F21, true), // forehead follows hair movement
        ("spell_cast_loop_south", 1, 36, 33, 0x763F21, true), // newly revealed forehead edge
        ("spell_cast_loop_south", 1, 36, 34, 0xBC8B43, true), // skin between hair and circlet
        ("spell_cast_loop_south", 1, 37, 33, 0xBC8B43, false), // adjacent circlet corner
        ("spell_cast_loop_south", 1, 43, 40, 0xBC8B43, false), // shoulder shifted under hair
        ("spell_cast_loop_south", 1, 46, 44, 0xBC8B43, false), // opposite cuff
        ("spell_cast_loop_south", 2, 38, 34, 0xE3BF7F, true), // skin under returning hair
        ("spell_cast_loop_south", 2, 42, 33, 0xBC8B43, false), // exposed circlet border
        ("spell_cast_loop_south", 2, 33, 44, 0xBC8B43, false), // unchanged cuff edge
        ("spell_cast_loop_south", 3, 42, 37, 0xE3BF7F, true), // opposite cheek during loop
        ("spell_cast_loop_south", 3, 44, 38, 0xEFD89A, true), // ear beside hair
        ("spell_cast_loop_south", 3, 46, 44, 0xBC8B43, false), // covered wrist border
        ("spell_cast_end_south", 0, 37, 40, 0x763F21, true), // restored cheek
        ("spell_cast_end_south", 0, 38, 34, 0xE3BF7F, false), // restored circlet
        ("spell_cast_end_south", 0, 46, 46, 0xBC8B43, false), // restored opposite cuff
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

        for (name, counts) in cases {
            let prefix = "winter";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Winter/spr_npc_juniper_specialanimation_{prefix}_{name}.png"
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
        assert_eq!(changed, 496);

        for r in &profile["regions"].as_array().unwrap()[..267] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-winter-specials-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: missing cheek/ear/forehead components and unrestricted color matching.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Winter/spr_npc_juniper_specialanimation_winter_spell_cast_loop_south.png";
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
        ("missing-cheek-shadow", [42, 39], 0x763F21),
        ("missing-ear", [35, 37], 0xEFD89A),
        ("missing-forehead", [38, 31], 0x763F21),
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
        ([38, 32], 0xE3BF7F, 0x7F9FBD), // circlet
        ([35, 40], 0xBC8B43, 0x6687AD), // shoulder decoration
        ([33, 44], 0xBC8B43, 0x6687AD), // glove cuff
    ] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(source));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(target));
    }
}
