use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/juniper and the retained Wedding-pilot Juniper bundle"]
fn juniper_wedding_finish_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/juniper");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 9] = [
        ("blink_south", &[59, 63, 59]),
        ("blink_east", &[57, 61, 57]),
        ("sit_north", &[0]),
        ("sit_south", &[54]),
        ("sit_east", &[50]),
        ("action_north", &[0, 4, 4, 4, 4, 0, 2]),
        ("action_south", &[57, 60, 60, 60, 60, 57, 59]),
        ("action_east", &[57, 61, 54, 61, 54, 57, 57]),
        ("kiss_east", &[52, 56, 63, 60]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // Literal protected gold hairpin, arm-band and anklet borders.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "blink_south",
            0,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (34, 44),
                (36, 44),
                (43, 44),
                (45, 44),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "blink_south",
            1,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (34, 44),
                (36, 44),
                (43, 44),
                (45, 44),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "blink_south",
            2,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (34, 44),
                (36, 44),
                (43, 44),
                (45, 44),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "blink_east",
            0,
            &[(37, 30), (34, 32), (35, 32), (37, 44), (41, 52), (42, 52)],
        ),
        (
            "blink_east",
            1,
            &[(37, 30), (34, 32), (35, 32), (37, 44), (41, 52), (42, 52)],
        ),
        (
            "blink_east",
            2,
            &[(37, 30), (34, 32), (35, 32), (37, 44), (41, 52), (42, 52)],
        ),
        (
            "sit_south",
            0,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (35, 44),
                (44, 44),
                (37, 49),
                (38, 49),
            ],
        ),
        (
            "sit_east",
            0,
            &[(37, 30), (34, 32), (35, 32), (38, 44), (40, 47)],
        ),
        (
            "action_south",
            0,
            &[
                (35, 31),
                (44, 31),
                (33, 33),
                (46, 33),
                (34, 45),
                (45, 45),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "action_south",
            1,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "action_south",
            2,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (36, 44),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "action_south",
            3,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "action_south",
            4,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (36, 44),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "action_south",
            5,
            &[
                (35, 31),
                (44, 31),
                (33, 33),
                (46, 33),
                (34, 45),
                (45, 45),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "action_south",
            6,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (34, 44),
                (36, 44),
                (43, 44),
                (45, 44),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        ("action_east", 0, &[(38, 31), (35, 33), (36, 33)]),
        (
            "action_east",
            1,
            &[(40, 30), (37, 32), (38, 32), (43, 43), (39, 52)],
        ),
        (
            "action_east",
            2,
            &[(40, 30), (37, 32), (38, 32), (38, 51), (39, 52)],
        ),
        (
            "action_east",
            3,
            &[(40, 30), (37, 32), (38, 32), (43, 43), (39, 52)],
        ),
        (
            "action_east",
            4,
            &[(40, 30), (37, 32), (38, 32), (38, 51), (39, 52)],
        ),
        ("action_east", 5, &[(38, 31), (35, 33), (36, 33)]),
        (
            "action_east",
            6,
            &[(37, 30), (34, 32), (35, 32), (37, 44), (41, 52), (42, 52)],
        ),
        ("kiss_east", 0, &[(36, 31), (33, 33), (34, 33), (34, 46)]),
        ("kiss_east", 1, &[(38, 31), (35, 33), (36, 33), (38, 45)]),
        (
            "kiss_east",
            2,
            &[(40, 30), (37, 32), (38, 32), (39, 44), (38, 52)],
        ),
        ("kiss_east", 3, &[(38, 31), (35, 33), (36, 33), (38, 45)]),
    ];
    // Fifty accepted anklet/cuff-edge occurrences remain connected to foot/hand skin.
    let exceptions: &[MaterialCase<'_>] = &[
        ("blink_east", 0, &[(38, 52), (39, 52)]),
        ("blink_east", 1, &[(38, 52), (39, 52)]),
        ("blink_east", 2, &[(38, 52), (39, 52)]),
        ("sit_east", 0, &[(41, 48), (42, 48)]),
        ("action_south", 1, &[(44, 45)]),
        ("action_south", 2, &[(44, 45)]),
        ("action_south", 3, &[(44, 45)]),
        ("action_south", 4, &[(44, 45)]),
        ("action_east", 0, &[(38, 52), (39, 52), (42, 52), (43, 52)]),
        ("action_east", 1, &[(37, 51), (38, 51), (42, 52), (43, 52)]),
        ("action_east", 2, &[(42, 52), (43, 52)]),
        ("action_east", 3, &[(37, 51), (38, 51), (42, 52), (43, 52)]),
        ("action_east", 4, &[(42, 52), (43, 52)]),
        ("action_east", 5, &[(38, 52), (39, 52), (42, 52), (43, 52)]),
        ("action_east", 6, &[(38, 52), (39, 52)]),
        ("kiss_east", 0, &[(37, 52), (38, 52), (41, 52), (42, 52)]),
        ("kiss_east", 1, &[(37, 52), (38, 52), (41, 52), (42, 52)]),
        ("kiss_east", 2, &[(36, 51), (37, 51), (41, 52), (42, 52)]),
        ("kiss_east", 3, &[(37, 52), (38, 52), (41, 52), (42, 52)]),
    ];
    assert_eq!(
        exceptions.iter().map(|(_, _, p)| p.len()).sum::<usize>(),
        50
    );
    // Zero-based frames, frame-local literal landmarks from freshly inspected art.
    let landmarks = [
        ("blink_south", 1, 37, 35, 0xE3BF7F, true), // lid-adjacent exposed face
        ("blink_south", 1, 37, 36, 0xFF6C89, false), // eyelid cosmetics
        ("blink_south", 1, 35, 30, 0xBC8B43, false), // gold hairpin edge
        ("blink_south", 1, 39, 42, 0xBC8B43, true), // dress keyhole
        ("blink_south", 1, 34, 44, 0x763F21, false), // band edge
        ("blink_south", 1, 32, 46, 0xEFD89A, true), // hand
        ("blink_south", 1, 37, 52, 0xBC8B43, false), // anklet border
        ("blink_south", 1, 37, 53, 0xEFD89A, true), // toes
        ("blink_east", 1, 38, 35, 0xE3BF7F, true),  // exposed lid-adjacent face
        ("blink_east", 1, 38, 36, 0xFF6C89, false), // side eyelid cosmetics
        ("blink_east", 1, 37, 30, 0xBC8B43, false), // side hairpin
        ("blink_east", 1, 38, 53, 0xBC8B43, true),  // foot shadow below connected anklet
        ("blink_east", 1, 41, 52, 0xBC8B43, false), // separable opposite anklet
        ("sit_north", 0, 39, 34, 0xFFFFFF, false),  // back hair-band highlight
        ("sit_south", 0, 38, 33, 0xBC8B43, true),   // forehead
        ("sit_south", 0, 39, 42, 0xBC8B43, true),   // keyhole
        ("sit_south", 0, 35, 44, 0x763F21, false),  // shifted arm-band edge
        ("sit_south", 0, 33, 46, 0xE3BF7F, true),   // shaded seated hand
        ("sit_south", 0, 44, 46, 0xE3BF7F, true),   // opposite hand
        ("sit_south", 0, 34, 47, 0x763F21, true),   // hand outline
        ("sit_south", 0, 37, 48, 0xFFF45D, false),  // raised anklet highlight
        ("sit_south", 0, 37, 49, 0xBC8B43, false),  // raised anklet lower edge
        ("sit_south", 0, 37, 50, 0xEFD89A, true),   // forward toes
        ("sit_south", 0, 41, 50, 0xEFD89A, true),   // opposite toes
        ("sit_east", 0, 38, 44, 0x763F21, false),   // seated band edge
        ("sit_east", 0, 34, 46, 0xEFD89A, true),    // near hand
        ("sit_east", 0, 40, 47, 0xBC8B43, false),   // separable ankle edge
        ("sit_east", 0, 41, 47, 0xFFF45D, false),   // gold ankle center
        ("sit_east", 0, 41, 49, 0xBC8B43, true),    // shaded foot, connected edge above
        ("sit_east", 0, 45, 48, 0xEFD89A, true),    // far seated toes
        ("action_north", 1, 37, 53, 0xBC8B43, true), // shaded feet under hem
        ("action_north", 4, 42, 53, 0xBC8B43, true),
        ("action_north", 6, 32, 47, 0xEFD89A, true), // hand reappears in final frame
        ("action_south", 0, 44, 44, 0x763F21, true), // exposed dark upper arm above band
        ("action_south", 0, 45, 45, 0x763F21, false), // adjacent band edge
        ("action_south", 0, 45, 47, 0xBC8B43, true), // shaded hand below connected cuff
        ("action_south", 1, 35, 43, 0xBC8B43, true), // upper arm
        ("action_south", 1, 35, 44, 0xFFF45D, false), // near gold band
        ("action_south", 1, 44, 44, 0xB65932, false), // opposite gold shadow
        ("action_south", 1, 44, 46, 0xBC8B43, true), // hand connected to cuff-edge exception
        ("action_south", 2, 36, 44, 0x763F21, false), // moving band border
        ("action_south", 2, 36, 45, 0xE3BF7F, true), // raised palm
        ("action_south", 2, 39, 46, 0xEFD89A, true), // fingertip crossing dress
        ("action_south", 2, 37, 47, 0xBC8B43, true), // lower palm shadow
        ("action_south", 4, 37, 52, 0xBC8B43, false), // anklet border remains excluded
        ("action_south", 5, 44, 44, 0x763F21, true), // exposed upper arm on return
        ("action_south", 6, 33, 45, 0xBC8B43, true), // restored wrist
        ("action_east", 0, 38, 43, 0xBC8B43, true),  // upper arm
        ("action_east", 0, 39, 45, 0xFFF45D, false), // gold center
        ("action_east", 0, 38, 46, 0xE3BF7F, true),  // palm
        ("action_east", 0, 42, 53, 0xBC8B43, true),  // foot shadow beside anklet
        ("action_east", 1, 43, 43, 0xBC8B43, false), // separable bracelet corner
        ("action_east", 1, 49, 43, 0xE3BF7F, true),  // extended fingertips
        ("action_east", 1, 49, 45, 0xBC8B43, true),  // far hand edge
        ("action_east", 1, 39, 52, 0xBC8B43, false), // separable angled anklet edge
        ("action_east", 2, 38, 51, 0xBC8B43, false), // alternate angled anklet edge
        ("action_east", 2, 37, 52, 0xBC8B43, true),  // shaded bare heel remains covered
        ("action_east", 2, 44, 45, 0xE3BF7F, true),  // moving hand by fabric
        ("action_east", 4, 46, 46, 0xE3BF7F, true),  // moving hand tip
        ("action_east", 6, 41, 52, 0xBC8B43, false), // opposite standing anklet
        ("kiss_east", 0, 34, 46, 0xBC8B43, false),   // cuff corner
        ("kiss_east", 0, 34, 47, 0xEFD89A, true),    // hand directly below corner
        ("kiss_east", 0, 37, 53, 0xBC8B43, true),    // foot below connected anklet
        ("kiss_east", 1, 38, 45, 0x763F21, false),   // shifted gold cuff border
        ("kiss_east", 1, 35, 46, 0xBC8B43, true),    // wrist alongside cuff
        ("kiss_east", 2, 41, 35, 0xFF6C89, false),   // kiss eyelid cosmetics
        ("kiss_east", 2, 41, 37, 0xE3BF7F, true),    // exposed skin below closed eye
        ("kiss_east", 2, 45, 38, 0xEFD89A, true),    // projecting cheek/mouth contour
        ("kiss_east", 2, 39, 44, 0x763F21, false),   // angled gold band edge
        ("kiss_east", 2, 36, 45, 0xEFD89A, true),    // raised palm
        ("kiss_east", 2, 38, 52, 0xBC8B43, false),   // separable angled ankle edge
        ("kiss_east", 2, 36, 52, 0xBC8B43, true),    // connected bare heel
        ("kiss_east", 3, 39, 36, 0xFF6C89, false),   // closed-eye cosmetics on return
        ("kiss_east", 3, 39, 38, 0xE3BF7F, true),    // below-eye skin on return
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
            let prefix = "wedding";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Wedding/spr_npc_juniper_{prefix}_{name}.png"
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
                        if !clothing.iter().any(|(case, frame, points)| {
                            *case == name && *frame == x / 80 && points.contains(&(x % 80, y))
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
            for (case, frame, points) in exceptions {
                if *case == name {
                    for (x, y) in *points {
                        assert_eq!(before.get_pixel(frame * 80 + x, *y).0, rgba(0xBC8B43));
                        assert_eq!(after.get_pixel(frame * 80 + x, *y).0, targets[2]);
                    }
                }
            }
            changed += actual.iter().sum::<usize>();
            for &(case, frame, x, y, color, changes) in &landmarks {
                if case == name {
                    let x = frame * 80 + x;
                    assert_eq!(before.get_pixel(x, y).0, rgba(color));
                    assert_eq!(before.get_pixel(x, y) != after.get_pixel(x, y), changes);
                }
            }
        }
        assert_eq!(changed, 1523);

        for r in &profile["regions"].as_array().unwrap()[..296] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-wedding-pilot-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: omitted forehead, keyhole, hand and toes; unrestricted material spill.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Wedding/spr_npc_juniper_wedding_blink_south.png";
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
        ("missing-forehead", [38, 33], 0xBC8B43),
        ("missing-keyhole", [39, 42], 0xBC8B43),
        ("missing-hand", [32, 46], 0xEFD89A),
        ("missing-toes", [37, 53], 0xEFD89A),
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
        "unrestricted-spill",
        Value::Null,
        map,
        profile["color_groups"].clone(),
    );
    for (point, color) in [
        ([35, 30], 0xBC8B43),
        ([34, 44], 0x763F21),
        ([37, 52], 0xBC8B43),
    ] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(color));
        assert_ne!(
            spilled.get_pixel(point[0], point[1]),
            correct.get_pixel(point[0], point[1])
        );
    }
}
