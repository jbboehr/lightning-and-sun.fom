use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-wedding-finish-study and the retained Autumn-specials Juniper bundle"]
fn juniper_autumn_finish_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-wedding-finish-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 5] = [
        ("spell_cast_start_south", &[48, 52]),
        ("spell_cast_loop_south", &[64, 63, 60, 58]),
        ("spell_cast_end_south", &[48]),
        ("hair_flip_south", &[32, 44, 44, 41, 41, 41]),
        ("snooze_south", &[45, 41, 45, 41, 45, 29, 27, 24, 45]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "spell_cast_start_south",
            0,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (33, 46), (46, 46)],
        ),
        (
            "spell_cast_start_south",
            1,
            &[(38, 33), (41, 33), (37, 34), (42, 34)],
        ),
        (
            "spell_cast_loop_south",
            0,
            &[(38, 32), (41, 32), (37, 33), (42, 33)],
        ),
        (
            "spell_cast_loop_south",
            1,
            &[(38, 32), (41, 32), (37, 33), (42, 33)],
        ),
        ("spell_cast_loop_south", 2, &[(41, 32), (42, 33)]),
        ("spell_cast_loop_south", 3, &[(41, 32), (42, 33)]),
        (
            "spell_cast_end_south",
            0,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (33, 46), (46, 46)],
        ),
        (
            "hair_flip_south",
            0,
            &[(38, 34), (41, 34), (37, 35), (42, 35)],
        ),
        (
            "hair_flip_south",
            1,
            &[(38, 33), (41, 33), (37, 34), (42, 34)],
        ),
        (
            "hair_flip_south",
            2,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (33, 46)],
        ),
        (
            "hair_flip_south",
            3,
            &[(35, 34), (38, 34), (34, 35), (39, 35), (31, 46), (45, 41)],
        ),
        (
            "hair_flip_south",
            4,
            &[(35, 34), (38, 34), (34, 35), (39, 35), (31, 46), (45, 41)],
        ),
        (
            "hair_flip_south",
            5,
            &[(37, 33), (40, 33), (36, 34), (41, 34)],
        ),
        (
            "snooze_south",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (33, 45), (43, 40)],
        ),
        (
            "snooze_south",
            1,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (33, 45), (43, 40)],
        ),
        (
            "snooze_south",
            2,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (33, 45), (43, 40)],
        ),
        (
            "snooze_south",
            3,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (33, 45), (43, 40)],
        ),
        (
            "snooze_south",
            4,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (33, 45), (43, 40)],
        ),
        (
            "snooze_south",
            5,
            &[(39, 36), (42, 36), (38, 37), (43, 37), (33, 46), (44, 40)],
        ),
        (
            "snooze_south",
            6,
            &[(39, 37), (42, 37), (38, 38), (43, 38), (33, 46), (44, 40)],
        ),
        ("snooze_south", 7, &[(33, 46)]),
        (
            "snooze_south",
            8,
            &[(38, 32), (41, 32), (37, 33), (42, 33), (33, 44)],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("spell_cast_start_south", 0, 38, 34, 0xE3BF7F, false), // circlet corner
        ("spell_cast_start_south", 0, 40, 34, 0xCA3561, false), // gemstone
        ("spell_cast_start_south", 0, 37, 36, 0xE3BF7F, true),  // forehead below circlet
        ("spell_cast_start_south", 0, 39, 43, 0xB58E45, true),  // keyhole shadow
        ("spell_cast_start_south", 0, 40, 43, 0xE0B572, true),  // keyhole midtone
        ("spell_cast_start_south", 0, 39, 46, 0xF1E791, true),  // midriff highlight
        ("spell_cast_start_south", 0, 46, 46, 0xBC8B43, false), // separable cuff edge
        ("spell_cast_start_south", 0, 46, 48, 0x763F21, true),  // fingers
        ("spell_cast_start_south", 1, 31, 44, 0xBC8B43, true),  // moving hand shadow
        ("spell_cast_start_south", 1, 33, 44, 0xF8F960, false), // raised cuff gold
        ("spell_cast_start_south", 1, 48, 44, 0xBC8B43, true),  // opposite hand shadow
        ("spell_cast_loop_south", 0, 38, 31, 0x763F21, true),   // exposed forehead above circlet
        ("spell_cast_loop_south", 0, 37, 32, 0x763F21, true),   // forehead beside shifting hair
        ("spell_cast_loop_south", 0, 38, 32, 0xE3BF7F, false),  // circlet beside forehead
        ("spell_cast_loop_south", 1, 36, 33, 0x763F21, true),   // exposed forehead after hair moves
        ("spell_cast_loop_south", 1, 36, 34, 0xBC8B43, true),   // adjacent skin shadow
        ("spell_cast_loop_south", 2, 41, 32, 0xE3BF7F, false),  // remaining circlet corner
        ("spell_cast_loop_south", 3, 42, 33, 0xBC8B43, false),  // remaining circlet border
        // Known art compromise: each BC8B43 cuff edge shares a component with
        // its neighboring hand shadow; preserve full wrist coverage in all four loop frames.
        ("spell_cast_loop_south", 0, 33, 44, 0xBC8B43, true), // coupled left cuff edge
        ("spell_cast_loop_south", 0, 32, 44, 0xBC8B43, true), // adjacent left hand
        ("spell_cast_loop_south", 0, 46, 44, 0xBC8B43, true), // coupled right cuff edge
        ("spell_cast_loop_south", 0, 47, 44, 0xBC8B43, true), // adjacent right hand
        ("spell_cast_loop_south", 0, 33, 43, 0xF8F960, false), // left gold border
        ("spell_cast_loop_south", 0, 46, 43, 0xF8F960, false), // right gold border
        ("spell_cast_loop_south", 1, 33, 44, 0xBC8B43, true), // coupled left cuff edge
        ("spell_cast_loop_south", 1, 32, 44, 0xBC8B43, true), // adjacent left hand
        ("spell_cast_loop_south", 1, 46, 44, 0xBC8B43, true), // coupled right cuff edge
        ("spell_cast_loop_south", 1, 47, 44, 0xBC8B43, true), // adjacent right hand
        ("spell_cast_loop_south", 1, 33, 43, 0xF8F960, false), // left gold border
        ("spell_cast_loop_south", 1, 46, 43, 0xF8F960, false), // right gold border
        ("spell_cast_loop_south", 2, 33, 44, 0xBC8B43, true), // coupled left cuff edge
        ("spell_cast_loop_south", 2, 32, 44, 0xBC8B43, true), // adjacent left hand
        ("spell_cast_loop_south", 2, 46, 44, 0xBC8B43, true), // coupled right cuff edge
        ("spell_cast_loop_south", 2, 47, 44, 0xBC8B43, true), // adjacent right hand
        ("spell_cast_loop_south", 2, 33, 43, 0xF8F960, false), // left gold border
        ("spell_cast_loop_south", 2, 46, 43, 0xF8F960, false), // right gold border
        ("spell_cast_loop_south", 3, 33, 44, 0xBC8B43, true), // coupled left cuff edge
        ("spell_cast_loop_south", 3, 32, 44, 0xBC8B43, true), // adjacent left hand
        ("spell_cast_loop_south", 3, 46, 44, 0xBC8B43, true), // coupled right cuff edge
        ("spell_cast_loop_south", 3, 47, 44, 0xBC8B43, true), // adjacent right hand
        ("spell_cast_loop_south", 3, 33, 43, 0xF8F960, false), // left gold border
        ("spell_cast_loop_south", 3, 46, 43, 0xF8F960, false), // right gold border
        ("spell_cast_end_south", 0, 33, 46, 0xBC8B43, false), // restored separable cuff
        ("spell_cast_end_south", 0, 33, 48, 0x763F21, true),  // restored fingers
        ("hair_flip_south", 0, 35, 46, 0xE3BF7F, true),       // diagonal hand tip beside cuff
        ("hair_flip_south", 0, 34, 46, 0xF8F960, false),      // gold beside hand tip
        ("hair_flip_south", 1, 35, 46, 0xE3BF7F, true),       // moving hand tip
        ("hair_flip_south", 1, 46, 37, 0xBC8B43, true),       // lifted fingers
        ("hair_flip_south", 2, 33, 46, 0xBC8B43, false),      // lower cuff border
        ("hair_flip_south", 2, 46, 38, 0xE3BF7F, true),       // opposite wrist above gold
        ("hair_flip_south", 2, 46, 39, 0xF8F960, false),      // opposite gold cuff
        ("hair_flip_south", 3, 31, 46, 0xBC8B43, false),      // moving lower cuff border
        ("hair_flip_south", 3, 45, 41, 0xBC8B43, false),      // raised cuff border
        ("hair_flip_south", 3, 44, 40, 0xBC8B43, true),       // raised hand beside border
        ("hair_flip_south", 4, 31, 46, 0xBC8B43, false),      // held lower cuff border
        ("hair_flip_south", 4, 45, 41, 0xBC8B43, false),      // held raised cuff border
        ("hair_flip_south", 4, 44, 40, 0xBC8B43, true),       // held hand beside border
        ("hair_flip_south", 5, 34, 45, 0xE3BF7F, true),       // final lower hand tip
        ("hair_flip_south", 5, 46, 41, 0x763F21, true),       // final raised fingers
        ("hair_flip_south", 5, 46, 43, 0xF8F960, false),      // final raised cuff
        ("snooze_south", 0, 43, 40, 0xBC8B43, false),         // cuff edge beside cheek
        ("snooze_south", 0, 45, 40, 0xBC8B43, true),          // hand edge beside cuff
        ("snooze_south", 1, 43, 40, 0xBC8B43, false),         // cuff edge beside cheek
        ("snooze_south", 1, 45, 40, 0xBC8B43, true),          // hand edge beside cuff
        ("snooze_south", 2, 43, 40, 0xBC8B43, false),         // cuff edge beside cheek
        ("snooze_south", 2, 45, 40, 0xBC8B43, true),          // hand edge beside cuff
        ("snooze_south", 3, 43, 40, 0xBC8B43, false),         // cuff edge beside cheek
        ("snooze_south", 3, 45, 40, 0xBC8B43, true),          // hand edge beside cuff
        ("snooze_south", 4, 43, 40, 0xBC8B43, false),         // cuff edge beside cheek
        ("snooze_south", 4, 45, 40, 0xBC8B43, true),          // hand edge beside cuff
        ("snooze_south", 5, 44, 40, 0xBC8B43, false),         // moved cuff edge
        ("snooze_south", 5, 46, 40, 0xBC8B43, true),          // moved hand edge
        ("snooze_south", 6, 44, 40, 0xBC8B43, false),         // held cuff edge
        ("snooze_south", 6, 46, 40, 0xBC8B43, true),          // held hand edge
        ("snooze_south", 7, 45, 40, 0xBC8B43, true),          // ear under hair, away from cuff
        ("snooze_south", 7, 45, 42, 0xF8F960, false),         // cuff below ear
        ("snooze_south", 7, 33, 46, 0xBC8B43, false),         // opposite cuff border
        ("snooze_south", 8, 33, 44, 0xBC8B43, false),         // restored lower cuff
        ("snooze_south", 8, 44, 45, 0xBC8B43, true),          // lower hand outline
        ("snooze_south", 8, 45, 44, 0xF8F960, false),         // cuff beside lower hand
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
            let prefix = "autumn";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Autumn/spr_npc_juniper_specialanimation_{prefix}_{name}.png"
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
        assert_eq!(changed, 978);

        for r in &profile["regions"].as_array().unwrap()[..234] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-autumn-specials-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping a finger shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Autumn/spr_npc_juniper_specialanimation_autumn_spell_cast_start_south.png";
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
    for (id, point, color) in [
        ("missing-keyhole-shadow", [39, 43], 0xB58E45),
        ("missing-keyhole-midtone", [40, 43], 0xE0B572),
    ] {
        let mut missing_alias = region.clone();
        missing_alias["seeds"]
            .as_array_mut()
            .unwrap()
            .retain(|s| *s != json!(point));
        let omitted = apply_control(
            id,
            missing_alias,
            map.clone(),
            profile["color_groups"].clone(),
        );
        assert_eq!(omitted.get_pixel(point[0], point[1]).0, rgba(color));
        assert_ne!(
            omitted.get_pixel(point[0], point[1]),
            correct.get_pixel(point[0], point[1])
        );
    }
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(46, 46).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(46, 46).0, rgba(0x6687AD));
}
