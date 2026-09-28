use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-winter-actions-study and the retained Autumn-world Juniper bundle"]
fn juniper_autumn_actions_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-winter-actions-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 11] = [
        ("blink_south", &[44, 48, 44]),
        ("blink_east", &[37, 41, 37]),
        ("sit_north", &[10]),
        ("sit_south", &[42]),
        ("sit_east", &[28]),
        ("eat_north", &[7, 4, 7]),
        ("eat_south", &[42, 38, 36, 44, 42]),
        ("eat_east", &[23, 27, 18, 23, 24]),
        ("drink_north", &[7, 4, 7]),
        ("drink_south", &[43, 40, 43]),
        ("drink_east", &[26, 25, 26]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "blink_south",
            0,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (33, 45), (46, 45)],
        ),
        (
            "blink_south",
            1,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (33, 45), (46, 45)],
        ),
        (
            "blink_south",
            2,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (33, 45), (46, 45)],
        ),
        (
            "blink_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (34, 45)],
        ),
        (
            "blink_east",
            1,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (34, 45)],
        ),
        (
            "blink_east",
            2,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (34, 45)],
        ),
        ("sit_north", 0, &[]),
        (
            "sit_south",
            0,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (34, 45), (45, 45)],
        ),
        (
            "sit_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (35, 45)],
        ),
        ("eat_north", 0, &[]),
        ("eat_north", 1, &[]),
        ("eat_north", 2, &[]),
        (
            "eat_south",
            0,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (45, 45)],
        ),
        (
            "eat_south",
            1,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (45, 45)],
        ),
        ("eat_south", 2, &[(38, 32), (41, 32), (45, 45)]),
        (
            "eat_south",
            3,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (45, 45)],
        ),
        (
            "eat_south",
            4,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (34, 45), (45, 45)],
        ),
        (
            "eat_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (41, 43)],
        ),
        (
            "eat_east",
            1,
            &[(40, 33), (43, 33), (39, 34), (44, 34), (43, 42)],
        ),
        ("eat_east", 2, &[(39, 32), (42, 32)]),
        (
            "eat_east",
            3,
            &[(39, 34), (42, 34), (38, 35), (43, 35), (41, 44)],
        ),
        (
            "eat_east",
            4,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (40, 45)],
        ),
        ("drink_north", 0, &[]),
        ("drink_north", 1, &[]),
        ("drink_north", 2, &[]),
        (
            "drink_south",
            0,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (45, 45)],
        ),
        (
            "drink_south",
            1,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (35, 43), (45, 45)],
        ),
        (
            "drink_south",
            2,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (45, 45)],
        ),
        (
            "drink_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (38, 44), (40, 44)],
        ),
        (
            "drink_east",
            1,
            &[(37, 33), (40, 33), (36, 34), (41, 34), (37, 43), (39, 43)],
        ),
        (
            "drink_east",
            2,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (38, 44), (40, 44)],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("blink_south", 0, 38, 33, 0xE3BF7F, false), // circlet corner
        ("blink_south", 0, 40, 33, 0xCA3561, false), // Autumn gemstone
        ("blink_south", 0, 39, 36, 0xE3BF7F, true),  // nose
        ("blink_south", 1, 35, 36, 0xEFD89A, true),  // ear during closed blink
        ("blink_south", 1, 39, 45, 0xF1E791, true),  // reviewed midriff alias
        ("blink_south", 2, 33, 45, 0xBC8B43, false), // gold cuff border
        ("blink_south", 2, 34, 45, 0xF8F960, false), // cuff gold
        ("blink_south", 2, 33, 47, 0x763F21, true),  // finger shadow below cuff
        ("blink_east", 1, 44, 45, 0xFBCC5A, false),  // shaded gold
        ("blink_east", 1, 44, 46, 0xE3BF7F, true),   // hand below gold
        ("blink_east", 2, 37, 42, 0x1C1627, false),  // sleeve covers Summer arm
        ("sit_north", 0, 34, 45, 0xBC8B43, true), // exposed back hand shadow, no gold in this pose
        ("sit_north", 0, 34, 47, 0x763F21, true), // seated fingers
        ("sit_north", 0, 39, 42, 0x9E77B3, false), // hair over back
        ("sit_south", 0, 34, 45, 0xBC8B43, false), // visible seated cuff border
        ("sit_south", 0, 45, 45, 0xBC8B43, false), // opposite cuff border
        ("sit_south", 0, 39, 45, 0xF1E791, true), // midriff highlight
        ("sit_south", 0, 39, 46, 0x84285F, false), // red skirt below midriff
        ("sit_east", 0, 35, 45, 0xBC8B43, false), // side cuff border
        ("sit_east", 0, 35, 47, 0x763F21, true),  // side finger shadow
        ("sit_east", 0, 39, 45, 0x84285F, false), // seated fabric covers standing midriff
        ("eat_north", 0, 46, 45, 0xBC8B43, true), // small hand visible beyond hair
        ("eat_north", 1, 34, 47, 0x763F21, true), // remaining finger shadow during bob
        ("eat_south", 0, 36, 45, 0xFBCC5A, false), // lowered cuff gold
        ("eat_south", 0, 36, 46, 0xEFD89A, true), // hand below lowered cuff
        ("eat_south", 1, 36, 46, 0xFFF45D, false), // moving cuff gold
        ("eat_south", 1, 36, 47, 0xEFD89A, true), // moving hand
        ("eat_south", 2, 37, 33, 0x8D80C7, false), // eyelid/circlet overlay
        ("eat_south", 2, 39, 36, 0x9E2626, false), // mouth interior
        ("eat_south", 2, 41, 35, 0x410808, false), // mouth rim
        ("eat_south", 2, 36, 40, 0x763F21, true), // raised finger shadow
        ("eat_south", 2, 37, 41, 0xEFD89A, true), // raised hand, sleeve edge clear
        ("eat_south", 2, 39, 45, 0xF1E791, true), // midriff beneath raised hand
        ("eat_south", 3, 35, 43, 0x763F21, true), // fingers as hand lowers
        ("eat_south", 3, 37, 45, 0xBC8B43, true), // hand outline
        ("eat_south", 4, 34, 45, 0xBC8B43, false), // restored cuff border
        ("eat_east", 0, 41, 43, 0xBC8B43, false), // cuff corner
        ("eat_east", 0, 43, 43, 0xEFD89A, true),  // fingers beyond cuff
        ("eat_east", 1, 43, 42, 0xBC8B43, false), // moving cuff corner
        ("eat_east", 1, 45, 42, 0xBC8B43, true),  // hand outline beyond cuff
        ("eat_east", 1, 46, 42, 0x763F21, true),  // fingertips
        ("eat_east", 2, 40, 36, 0x9E2626, false), // open mouth interior
        ("eat_east", 2, 43, 40, 0xEFD89A, true),  // raised hand by mouth
        ("eat_east", 2, 43, 41, 0xFBCC5A, false), // cuff under raised hand
        ("eat_east", 3, 41, 44, 0xBC8B43, false), // lowered cuff corner
        ("eat_east", 4, 40, 45, 0xBC8B43, false), // resting cuff corner
        ("drink_north", 0, 34, 45, 0xBC8B43, true), // exposed back hand edge
        ("drink_north", 1, 33, 46, 0xE3BF7F, true), // back fingers during bob
        ("drink_south", 0, 35, 42, 0xE8B171, true), // established drink-hand alias
        ("drink_south", 0, 35, 44, 0xBC8B43, true), // raised hand outline
        ("drink_south", 1, 35, 43, 0xBC8B43, false), // cuff corner when hand lifts
        ("drink_south", 1, 36, 41, 0xE8B171, true), // hand shading above cuff
        ("drink_south", 2, 39, 45, 0xF1E791, true), // exposed midriff returns
        ("drink_east", 0, 38, 44, 0xBC8B43, false), // cuff outer edge
        ("drink_east", 0, 40, 44, 0xE8B171, false), // same alias used on separable cuff corner
        ("drink_east", 0, 42, 43, 0xE8B171, true), // hand alias alongside cuff
        ("drink_east", 1, 37, 43, 0xBC8B43, false), // tilted cuff outer edge
        ("drink_east", 1, 39, 43, 0xBC8B43, false), // tilted cuff inner edge
        ("drink_east", 1, 39, 42, 0xE8B171, true), // hand above tilted cuff
        ("drink_east", 2, 40, 44, 0xE8B171, false), // returning cuff corner
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
        let targets = [0, 1, 12, 4, 13, 14].map(|i| {
            rgba(u32::from_str_radix(&preset["colors"][i].as_str().unwrap()[1..7], 16).unwrap())
        });
        let mut changed = 0;

        for (name, counts) in cases {
            let prefix = "autumn";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Autumn/spr_npc_juniper_{prefix}_{name}.png"
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
        assert_eq!(changed, 887);

        for r in &profile["regions"].as_array().unwrap()[..212] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-autumn-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping an finger shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Autumn/spr_npc_juniper_autumn_blink_south.png";
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
        .retain(|s| *s != json!([33, 47]));
    let omitted = apply_control(
        "missing-finger-shadow",
        missing,
        map.clone(),
        profile["color_groups"].clone(),
    );
    assert_eq!(omitted.get_pixel(33, 47).0, rgba(0x763F21));
    assert_ne!(omitted.get_pixel(33, 47), correct.get_pixel(33, 47));
    let mut missing_alias = region.clone();
    missing_alias["seeds"]
        .as_array_mut()
        .unwrap()
        .retain(|s| *s != json!([39, 45]));
    let omitted_alias = apply_control(
        "missing-midriff-highlight",
        missing_alias,
        map.clone(),
        profile["color_groups"].clone(),
    );
    assert_eq!(omitted_alias.get_pixel(39, 45).0, rgba(0xF1E791));
    assert_ne!(omitted_alias.get_pixel(39, 45), correct.get_pixel(39, 45));
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(33, 45).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(33, 45).0, rgba(0x6687AD));
}
