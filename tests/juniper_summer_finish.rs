use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-winter-standard-study and the retained Summer-specials Juniper bundle"]
fn juniper_summer_finish_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-winter-standard-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 5] = [
        ("spell_cast_start_south", &[65, 70]),
        ("spell_cast_loop_south", &[82, 81, 78, 76]),
        ("spell_cast_end_south", &[65]),
        ("hair_flip_south", &[55, 68, 70, 65, 65, 66]),
        ("snooze_south", &[69, 65, 69, 65, 69, 52, 53, 48, 65]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
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
                (33, 43),
                (35, 43),
                (44, 43),
                (46, 43),
            ],
        ),
        (
            "spell_cast_loop_south",
            0,
            &[(38, 32), (41, 32), (37, 33), (42, 33), (35, 43), (44, 43)],
        ),
        (
            "spell_cast_loop_south",
            1,
            &[(38, 32), (41, 32), (37, 33), (42, 33), (35, 43), (44, 43)],
        ),
        (
            "spell_cast_loop_south",
            2,
            &[(41, 32), (42, 33), (35, 43), (44, 43)],
        ),
        (
            "spell_cast_loop_south",
            3,
            &[(41, 32), (42, 33), (35, 43), (44, 43)],
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
            ],
        ),
        (
            "hair_flip_south",
            0,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (33, 45), (42, 42)],
        ),
        (
            "hair_flip_south",
            1,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (34, 45), (45, 39)],
        ),
        (
            "hair_flip_south",
            2,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (34, 44)],
        ),
        (
            "hair_flip_south",
            3,
            &[(35, 34), (38, 34), (34, 35), (39, 35), (42, 43)],
        ),
        (
            "hair_flip_south",
            4,
            &[(35, 34), (38, 34), (34, 35), (39, 35), (42, 43)],
        ),
        (
            "hair_flip_south",
            5,
            &[(37, 33), (40, 33), (36, 34), (41, 34), (43, 44), (45, 44)],
        ),
        (
            "snooze_south",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (33, 45), (35, 45)],
        ),
        (
            "snooze_south",
            1,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (33, 45), (35, 45)],
        ),
        (
            "snooze_south",
            2,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (33, 45), (35, 45)],
        ),
        (
            "snooze_south",
            3,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (33, 45), (35, 45)],
        ),
        (
            "snooze_south",
            4,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (33, 45), (35, 45)],
        ),
        (
            "snooze_south",
            5,
            &[(39, 36), (42, 36), (38, 37), (43, 37), (33, 46)],
        ),
        (
            "snooze_south",
            6,
            &[(39, 37), (42, 37), (38, 38), (43, 38), (33, 46), (35, 46)],
        ),
        ("snooze_south", 7, &[(33, 46), (35, 46)]),
        (
            "snooze_south",
            8,
            &[
                (38, 32),
                (41, 32),
                (37, 33),
                (42, 33),
                (33, 44),
                (35, 44),
                (46, 44),
            ],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("spell_cast_start_south", 0, 38, 34, 0xE3BF7F, false), // circlet corner
        ("spell_cast_start_south", 0, 37, 35, 0xBC8B43, false), // circlet side
        ("spell_cast_start_south", 0, 36, 44, 0xEFD89A, true),  // upper arm
        ("spell_cast_start_south", 0, 35, 46, 0xBC8B43, false), // cuff corner
        ("spell_cast_start_south", 0, 33, 46, 0xBC8B43, false), // cuff outer edge
        ("spell_cast_start_south", 0, 34, 46, 0xFFF45D, false), // gold cuff
        ("spell_cast_start_south", 0, 33, 48, 0x763F21, true),  // finger shadow
        ("spell_cast_start_south", 0, 39, 46, 0xEFD89A, true),  // midriff
        ("spell_cast_start_south", 0, 38, 43, 0xD6CDF4, false), // blouse
        ("spell_cast_start_south", 0, 38, 47, 0x84DAE8, false), // skirt
        ("spell_cast_start_south", 1, 33, 43, 0xBC8B43, false), // outstretched cuff edge
        ("spell_cast_start_south", 1, 34, 44, 0xBC8B43, true),  // hand edge below cuff
        ("spell_cast_start_south", 1, 49, 44, 0xE3BF7F, true),  // far fingertips
        ("spell_cast_loop_south", 0, 38, 31, 0x763F21, true),   // exposed forehead above circlet
        ("spell_cast_loop_south", 0, 39, 31, 0x763F21, true),   // neighboring forehead
        ("spell_cast_loop_south", 1, 36, 33, 0x763F21, true),   // forehead beside shifting hair
        ("spell_cast_loop_south", 1, 36, 34, 0xBC8B43, true),   // exposed temple
        ("spell_cast_loop_south", 2, 39, 32, 0xECF0E9, false),  // circlet highlight
        ("spell_cast_loop_south", 3, 38, 33, 0x5A3668, false),  // hair covers prior circlet edge
        ("spell_cast_end_south", 0, 46, 46, 0xBC8B43, false),   // restored cuff edge
        ("spell_cast_end_south", 0, 46, 48, 0x763F21, true),    // restored fingers
        ("hair_flip_south", 0, 42, 42, 0xBC8B43, false),        // rotated cuff corner
        ("hair_flip_south", 0, 42, 40, 0xBC8B43, true),         // raised wrist outline above cuff
        ("hair_flip_south", 0, 43, 43, 0xBC8B43, true),         // upper-arm outline below cuff
        ("hair_flip_south", 0, 33, 45, 0xBC8B43, false),        // lowered cuff edge
        ("hair_flip_south", 1, 45, 39, 0xBC8B43, false),        // higher rotated cuff corner
        ("hair_flip_south", 1, 43, 35, 0xEFD89A, true),         // high fingers above cuff
        ("hair_flip_south", 1, 46, 40, 0xBC8B43, true),         // bare elbow below cuff
        ("hair_flip_south", 2, 48, 35, 0xBC8B43, true),         // fingertips beside flying hair
        ("hair_flip_south", 2, 50, 36, 0x9E77B3, false),        // flying hair
        ("hair_flip_south", 3, 42, 43, 0xBC8B43, false),        // angled cuff
        ("hair_flip_south", 3, 47, 39, 0xEFD89A, true),         // outstretched fingers
        ("hair_flip_south", 4, 44, 42, 0xBC8B43, true),         // wrist outside cuff gem
        ("hair_flip_south", 5, 43, 44, 0xBC8B43, false),        // returning cuff near corner
        ("hair_flip_south", 5, 45, 44, 0xBC8B43, false),        // returning cuff far corner
        ("hair_flip_south", 5, 46, 41, 0x763F21, true),         // raised finger shadow
        ("snooze_south", 0, 33, 45, 0xBC8B43, false),           // lowered cuff outer border
        ("snooze_south", 5, 33, 46, 0xBC8B43, false),           // cuff during head droop
        ("snooze_south", 6, 35, 46, 0xBC8B43, false),           // cuff inner border
        ("snooze_south", 7, 36, 40, 0xEFD89A, true),            // ear under drooped hair
        ("snooze_south", 8, 46, 44, 0xBC8B43, false),           // raised cuff outer border
        // Known exceptions: cuff corners coupled to the adjoining hand/forearm.
        ("spell_cast_loop_south", 0, 33, 43, 0xE3BF7F, true),
        ("spell_cast_loop_south", 0, 46, 43, 0xE3BF7F, true),
        ("spell_cast_loop_south", 1, 33, 43, 0xE3BF7F, true),
        ("spell_cast_loop_south", 1, 46, 43, 0xE3BF7F, true),
        ("spell_cast_loop_south", 2, 33, 43, 0xE3BF7F, true),
        ("spell_cast_loop_south", 2, 46, 43, 0xE3BF7F, true),
        ("spell_cast_loop_south", 3, 33, 43, 0xE3BF7F, true),
        ("spell_cast_loop_south", 3, 46, 43, 0xE3BF7F, true),
        ("hair_flip_south", 2, 45, 41, 0xE3BF7F, true),
        ("hair_flip_south", 3, 31, 45, 0xE3BF7F, true),
        ("hair_flip_south", 4, 31, 45, 0xE3BF7F, true),
        ("hair_flip_south", 5, 33, 45, 0xEFD89A, true),
        ("snooze_south", 8, 44, 44, 0xE3BF7F, true),
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
            let prefix = "specialanimation_summer";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Summer/spr_npc_juniper_{prefix}_{name}.png"
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
        assert_eq!(changed, 1461);

        for r in &profile["regions"].as_array().unwrap()[..201] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-summer-specials-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping a finger shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Summer/spr_npc_juniper_specialanimation_summer_spell_cast_start_south.png";
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
        .retain(|s| *s != json!([46, 48]));
    let omitted = apply_control(
        "missing-finger-shadow",
        missing,
        map.clone(),
        profile["color_groups"].clone(),
    );
    assert_eq!(omitted.get_pixel(46, 48).0, rgba(0x763F21));
    assert_ne!(omitted.get_pixel(46, 48), correct.get_pixel(46, 48));
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(46, 46).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(46, 46).0, rgba(0x6687AD));
}
