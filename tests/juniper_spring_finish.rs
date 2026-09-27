use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-summer-standard-study and the retained Spring-specials Juniper bundle"]
fn juniper_spring_finish_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-summer-standard-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 7] = [
        ("pose_south", &[49]),
        ("hair_flip_south", &[43, 58, 59, 53, 53, 53]),
        ("spell_cast_start_south", &[52, 60]),
        ("spell_cast_loop_south", &[76, 72, 68, 68]),
        ("spell_cast_end_south", &[52]),
        ("gremlin_east", &[31, 27, 31, 28, 31, 30]),
        ("snooze_south", &[62, 58, 62, 58, 62, 45, 45, 41, 55]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171];
    // Literal source-material exclusions: circlet, bracers and skirt trim.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "pose_south",
            0,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (36, 43),
                (36, 44),
                (46, 42),
                (42, 47),
            ],
        ),
        (
            "hair_flip_south",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (43, 41),
                (41, 42),
                (35, 44),
                (41, 48),
                (36, 51),
            ],
        ),
        (
            "hair_flip_south",
            1,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (46, 40),
                (34, 44),
                (41, 47),
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
                (45, 40),
                (46, 41),
                (35, 43),
                (41, 47),
                (34, 50),
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
                (43, 41),
                (45, 41),
                (44, 42),
                (30, 44),
                (32, 44),
                (38, 48),
                (41, 49),
                (42, 49),
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
                (43, 41),
                (45, 41),
                (44, 42),
                (30, 44),
                (32, 44),
                (38, 48),
                (41, 49),
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
                (44, 43),
                (43, 44),
                (45, 44),
                (40, 47),
                (40, 48),
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
                (33, 46),
                (35, 46),
                (44, 46),
                (46, 46),
                (41, 48),
                (44, 49),
                (34, 50),
            ],
        ),
        (
            "spell_cast_start_south",
            1,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 43),
                (34, 44),
                (44, 43),
                (45, 44),
                (41, 47),
            ],
        ),
        (
            "spell_cast_loop_south",
            0,
            &[
                (38, 32),
                (41, 32),
                (37, 33),
                (42, 33),
                (35, 43),
                (44, 43),
                (41, 47),
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
                (35, 43),
                (44, 43),
                (41, 47),
            ],
        ),
        (
            "spell_cast_loop_south",
            2,
            &[(41, 32), (42, 33), (35, 43), (44, 43), (41, 47)],
        ),
        (
            "spell_cast_loop_south",
            3,
            &[(41, 32), (42, 33), (35, 43), (44, 43), (41, 47)],
        ),
        (
            "spell_cast_end_south",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (33, 46),
                (35, 46),
                (44, 46),
                (46, 46),
                (41, 48),
                (44, 49),
                (34, 50),
            ],
        ),
        (
            "gremlin_east",
            0,
            &[
                (39, 34),
                (42, 34),
                (38, 35),
                (43, 35),
                (37, 44),
                (44, 45),
                (41, 46),
                (39, 47),
            ],
        ),
        (
            "gremlin_east",
            1,
            &[(37, 33), (40, 33), (36, 45), (45, 43), (43, 44), (39, 47)],
        ),
        (
            "gremlin_east",
            2,
            &[
                (39, 34),
                (42, 34),
                (38, 35),
                (43, 35),
                (37, 44),
                (44, 45),
                (41, 46),
                (39, 47),
            ],
        ),
        (
            "gremlin_east",
            3,
            &[
                (41, 33),
                (44, 33),
                (40, 34),
                (45, 34),
                (45, 45),
                (46, 45),
                (43, 46),
                (39, 47),
            ],
        ),
        (
            "gremlin_east",
            4,
            &[
                (39, 34),
                (42, 34),
                (38, 35),
                (43, 35),
                (37, 44),
                (44, 45),
                (41, 46),
                (39, 47),
            ],
        ),
        (
            "gremlin_east",
            5,
            &[(42, 38), (45, 38), (41, 39), (46, 39), (35, 47), (42, 48)],
        ),
        (
            "snooze_south",
            0,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (33, 45),
                (35, 45),
                (41, 47),
            ],
        ),
        (
            "snooze_south",
            1,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (33, 45),
                (35, 45),
                (41, 47),
            ],
        ),
        (
            "snooze_south",
            2,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (33, 45),
                (35, 45),
                (41, 47),
            ],
        ),
        (
            "snooze_south",
            3,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (33, 45),
                (35, 45),
                (41, 47),
            ],
        ),
        (
            "snooze_south",
            4,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (33, 45),
                (35, 45),
                (41, 47),
            ],
        ),
        (
            "snooze_south",
            5,
            &[
                (39, 36),
                (42, 36),
                (38, 37),
                (43, 37),
                (33, 46),
                (35, 46),
                (41, 47),
            ],
        ),
        (
            "snooze_south",
            6,
            &[
                (39, 37),
                (42, 37),
                (38, 38),
                (43, 38),
                (33, 46),
                (35, 46),
                (41, 48),
            ],
        ),
        ("snooze_south", 7, &[(33, 46), (35, 46), (41, 48)]),
        (
            "snooze_south",
            8,
            &[(38, 32), (41, 32), (37, 33), (42, 33), (33, 44), (35, 44)],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("pose_south", 0, 38, 33, 0xE3BF7F, false), // circlet corner
        ("pose_south", 0, 36, 42, 0xE3BF7F, true),  // upper arm above bracer
        ("pose_south", 0, 36, 43, 0xBC8B43, false), // bracer edge
        ("pose_south", 0, 36, 44, 0x763F21, false), // darkest bracer edge
        ("pose_south", 0, 34, 47, 0x763F21, true),  // fingers below bracer
        ("pose_south", 0, 41, 48, 0xBC8B43, true),  // leg beside hem
        ("hair_flip_south", 0, 43, 41, 0xBC8B43, false), // raised cuff
        ("hair_flip_south", 0, 41, 42, 0xE3BF7F, false), // separable cuff corner
        ("hair_flip_south", 1, 46, 40, 0xE3BF7F, false), // outer cuff corner
        ("hair_flip_south", 1, 34, 44, 0xE3BF7F, false), // opposite cuff edge
        ("hair_flip_south", 2, 45, 40, 0xBC8B43, false), // raised cuff shadow
        ("hair_flip_south", 2, 45, 42, 0xBC8B43, true), // elbow below cuff
        ("hair_flip_south", 3, 30, 44, 0xE3BF7F, false), // swinging cuff corner
        ("hair_flip_south", 3, 32, 44, 0x763F21, false), // cuff darkest edge
        ("hair_flip_south", 4, 30, 45, 0xBC8B43, true), // hand below cuff
        ("hair_flip_south", 5, 44, 43, 0xBC8B43, false), // rotated cuff edge
        ("hair_flip_south", 5, 43, 42, 0xBC8B43, true), // arm above rotated cuff
        ("spell_cast_start_south", 0, 35, 44, 0xBC8B43, true), // lowered arm
        ("spell_cast_start_south", 1, 44, 43, 0xBC8B43, false), // extended cuff edge
        ("spell_cast_start_south", 1, 49, 45, 0xE3BF7F, true), // extended fingers
        ("spell_cast_loop_south", 0, 38, 31, 0x763F21, true), // forehead exposed above circlet
        ("spell_cast_loop_south", 0, 39, 31, 0x763F21, true), // adjoining forehead
        ("spell_cast_loop_south", 0, 37, 32, 0x763F21, true), // forehead along circlet
        ("spell_cast_loop_south", 1, 36, 33, 0x763F21, true), // changing forehead edge
        ("spell_cast_loop_south", 1, 36, 34, 0xBC8B43, true), // exposed temple
        ("spell_cast_loop_south", 2, 41, 32, 0xE3BF7F, false), // circlet under moving hair
        ("spell_cast_loop_south", 3, 44, 43, 0xBC8B43, false), // cuff beside open palm
        ("spell_cast_end_south", 0, 33, 46, 0xBC8B43, false), // restored cuff edge
        ("gremlin_east", 0, 37, 44, 0xBC8B43, false), // crouching bracer
        ("gremlin_east", 1, 36, 42, 0xBC8B43, true), // crouching upper arm
        ("gremlin_east", 1, 36, 45, 0xBC8B43, false), // cuff below upper arm
        ("gremlin_east", 1, 36, 35, 0xC2B9BE, false), // expression detail
        ("gremlin_east", 3, 44, 41, 0xBC8B43, true), // raised fingers
        ("gremlin_east", 3, 46, 45, 0xBC8B43, false), // skirt tail
        ("gremlin_east", 5, 43, 47, 0xBC8B43, true), // hand beside cuff
        ("gremlin_east", 5, 42, 48, 0xBC8B43, false), // rotated cuff
        ("snooze_south", 0, 45, 36, 0xBC8B43, true), // far ear
        ("snooze_south", 0, 44, 39, 0x763F21, true), // raised hand shadow
        ("snooze_south", 4, 35, 43, 0xBC8B43, true), // relaxed upper arm
        ("snooze_south", 5, 39, 36, 0xE3BF7F, false), // drooping circlet
        ("snooze_south", 6, 43, 38, 0xBC8B43, false), // lower circlet edge
        ("snooze_south", 7, 45, 40, 0xBC8B43, true), // ear beneath tilted head
        ("snooze_south", 7, 48, 39, 0xEFD89A, true), // raised fingertips
        ("snooze_south", 8, 33, 44, 0xBC8B43, false), // startled cuff
        ("snooze_south", 8, 42, 45, 0x763F21, true), // hand over chest
        // Known art exceptions: these seven bracer-edge midtones share a
        // component with exposed wrist/arm pixels. Preserve full skin coverage.
        ("hair_flip_south", 0, 43, 42, 0xE3BF7F, true),
        ("hair_flip_south", 0, 34, 45, 0xE3BF7F, true),
        ("hair_flip_south", 1, 44, 40, 0xE3BF7F, true),
        ("hair_flip_south", 1, 35, 43, 0xE3BF7F, true),
        ("hair_flip_south", 2, 33, 43, 0xE3BF7F, true),
        ("hair_flip_south", 5, 32, 44, 0xE3BF7F, true),
        ("hair_flip_south", 5, 46, 43, 0xE3BF7F, true),
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
        let targets = [0, 1, 12, 4, 13].map(|i| {
            rgba(u32::from_str_radix(&preset["colors"][i].as_str().unwrap()[1..7], 16).unwrap())
        });
        let mut changed = 0;

        for (name, counts) in cases {
            let prefix = "specialanimation_spring";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Spring/spr_npc_juniper_{prefix}_{name}.png"
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
        assert_eq!(changed, 1482);

        for r in &profile["regions"].as_array().unwrap()[..166] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-specials-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping an upper-arm shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Spring/spr_npc_juniper_specialanimation_spring_spell_cast_end_south.png";
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
    let apply_control = |id: &str,
                         region: Value,
                         map: serde_json::Map<String, Value>,
                         groups: Value| {
        let recipe = temp.path().join(format!("control-{id}.json"));
        fs::write(
            &recipe,
            serde_json::to_vec(&json!({"regions":[region],"rgba_map":map,"color_groups":groups}))
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
    let mut missing = region.clone();
    missing["seeds"]
        .as_array_mut()
        .unwrap()
        .retain(|s| *s != json!([35, 44]));
    let omitted = apply_control(
        "missing-arm-shadow",
        missing,
        map.clone(),
        profile["color_groups"].clone(),
    );
    assert_eq!(omitted.get_pixel(35, 44).0, rgba(0xBC8B43));
    assert_ne!(omitted.get_pixel(35, 44), correct.get_pixel(35, 44));
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(33, 46).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(33, 46).0, rgba(0x6687AD));
}
