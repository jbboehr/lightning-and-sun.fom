use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-wedding-finish-study and the retained Beach-pilot Juniper bundle"]
fn juniper_beach_actions_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-wedding-finish-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("blink_east", &[74, 78, 74]),
        ("blink_south", &[82, 86, 82]),
        ("action_north", &[59, 58, 58, 58, 58, 60, 62]),
        ("action_south", &[74, 75, 74, 75, 74, 76, 82]),
        ("action_east", &[64, 75, 68, 75, 68, 64, 74]),
        ("kiss_east", &[66, 67, 73, 71]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // Literal source-material exclusions: separable bangle borders.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        ("blink_east", 0, &[(34, 45), (36, 45)]),
        ("blink_east", 1, &[(34, 45), (36, 45)]),
        ("blink_east", 2, &[(34, 45), (36, 45)]),
        ("blink_south", 0, &[(33, 45), (35, 45), (44, 45), (46, 45)]),
        ("blink_south", 1, &[(33, 45), (35, 45), (44, 45), (46, 45)]),
        ("blink_south", 2, &[(33, 45), (35, 45), (44, 45), (46, 45)]),
        ("action_north", 0, &[(33, 43), (35, 43)]),
        ("action_north", 1, &[(34, 44), (36, 44)]),
        ("action_north", 2, &[(34, 44), (36, 44)]),
        ("action_north", 3, &[(34, 44), (36, 44)]),
        ("action_north", 4, &[(34, 44), (36, 44)]),
        ("action_north", 5, &[(33, 45), (35, 45)]),
        ("action_north", 6, &[(33, 45), (35, 45), (44, 45), (46, 45)]),
        ("action_south", 0, &[(33, 46), (35, 46), (44, 46), (46, 46)]),
        ("action_south", 1, &[(34, 45), (36, 45), (45, 45)]),
        ("action_south", 2, &[(38, 45), (45, 45)]),
        ("action_south", 3, &[(34, 45), (36, 45), (45, 45)]),
        ("action_south", 4, &[(38, 45), (45, 45)]),
        ("action_south", 5, &[(33, 46), (35, 46)]),
        ("action_south", 6, &[(33, 45), (35, 45), (44, 45), (46, 45)]),
        ("action_east", 0, &[(40, 46)]),
        ("action_east", 1, &[(47, 43), (49, 43)]),
        ("action_east", 2, &[(42, 44), (45, 44)]),
        ("action_east", 3, &[(47, 43), (49, 43)]),
        ("action_east", 4, &[(42, 44), (45, 44)]),
        ("action_east", 5, &[(40, 46)]),
        ("action_east", 6, &[(34, 45), (36, 45)]),
        ("kiss_east", 0, &[(34, 46), (36, 46)]),
        ("kiss_east", 1, &[(35, 46), (37, 46)]),
        ("kiss_east", 2, &[(35, 44), (37, 44)]),
        ("kiss_east", 3, &[(35, 46), (37, 46)]),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("blink_east", 0, 40, 36, 0xEFD89A, true), // face between eyes
        ("blink_east", 0, 38, 39, 0x763F21, true), // jaw shadow
        ("blink_east", 0, 34, 45, 0xBC8B43, false), // near bangle border
        ("blink_east", 1, 38, 35, 0xE3BF7F, true), // skin above closed eyelid
        ("blink_east", 1, 38, 36, 0xB789D5, false), // eyelid cosmetics
        ("blink_east", 1, 36, 36, 0xEFD89A, true), // ear
        ("blink_east", 2, 44, 46, 0xE3BF7F, true), // far hand
        ("blink_east", 2, 44, 45, 0xFFF45D, false), // far bangle center
        ("blink_south", 0, 39, 42, 0xEFD89A, true), // chest between wraps
        ("blink_south", 0, 35, 27, 0xDF8D4B, false), // hair tie
        ("blink_south", 0, 36, 27, 0xFFD565, false), // hair tie highlight
        ("blink_south", 1, 37, 35, 0xE3BF7F, true), // skin above closed eyelid
        ("blink_south", 1, 37, 36, 0xB789D5, false), // eyelid cosmetics
        ("blink_south", 1, 44, 45, 0xBC8B43, false), // opposite bangle border
        ("blink_south", 2, 37, 53, 0xEFD89A, true), // toes
        ("blink_south", 2, 33, 45, 0xBC8B43, false), // restored near bangle border
        ("action_north", 0, 39, 41, 0xE3BF7F, true), // exposed back
        ("action_north", 0, 32, 44, 0xEFD89A, true), // forward hand
        ("action_north", 0, 33, 43, 0xBC8B43, false), // forward bangle border
        ("action_north", 0, 45, 41, 0x763F21, true), // raised rear fingers
        ("action_north", 1, 44, 38, 0xBC8B43, true), // raised rear hand
        ("action_north", 1, 36, 44, 0xBC8B43, false), // near bangle border
        ("action_north", 2, 44, 39, 0xBC8B43, true), // returning raised hand
        ("action_north", 2, 36, 45, 0xEFD89A, true), // opposite hand below bangle
        ("action_north", 3, 43, 39, 0x763F21, true), // raised finger shadow
        ("action_north", 3, 35, 44, 0xFFF45D, false), // near gold center
        ("action_north", 4, 45, 40, 0x763F21, true), // hand shadow follows gesture
        ("action_north", 4, 34, 44, 0xBC8B43, false), // near bangle edge
        ("action_north", 5, 33, 46, 0xEFD89A, true), // lowered hand
        ("action_north", 5, 35, 45, 0xBC8B43, false), // lowered bangle edge
        ("action_north", 6, 35, 36, 0xEFD89A, true), // ear at rest
        ("action_north", 6, 44, 45, 0xBC8B43, false), // restored opposite bangle
        ("action_south", 0, 39, 43, 0xEFD89A, true), // lowered chest
        ("action_south", 0, 44, 46, 0x763F21, false), // dark separable bangle corner
        ("action_south", 0, 45, 47, 0xBC8B43, true), // opposite hand below bangle
        ("action_south", 1, 39, 44, 0xBC8B43, true), // midriff shadow below top
        ("action_south", 1, 45, 45, 0xBC8B43, false), // far bangle border
        ("action_south", 2, 36, 46, 0xBC8B43, true), // hand joined to bangle edge
        ("action_south", 2, 38, 45, 0xBC8B43, false), // separable opposite edge
        ("action_south", 3, 36, 46, 0xEFD89A, true), // hand as gesture returns
        ("action_south", 3, 34, 45, 0xBC8B43, false), // bangle edge as gesture returns
        ("action_south", 4, 38, 47, 0x763F21, true), // fingers below joined edge
        ("action_south", 4, 45, 45, 0xBC8B43, false), // opposite bangle border
        ("action_south", 5, 45, 47, 0xBC8B43, true), // far hand joined to edge
        ("action_south", 5, 33, 46, 0xBC8B43, false), // near border remains separable
        ("action_south", 6, 39, 45, 0xEFD89A, true), // exposed waist at rest
        ("action_south", 6, 46, 45, 0xBC8B43, false), // restored opposite border
        ("action_east", 0, 38, 47, 0xBC8B43, true), // hand joined to edge
        ("action_east", 0, 40, 46, 0xBC8B43, false), // separable opposite edge
        ("action_east", 1, 43, 42, 0xEFD89A, true), // outstretched forearm
        ("action_east", 1, 49, 44, 0xEFD89A, true), // fingertips beyond bangle
        ("action_east", 1, 47, 43, 0xBC8B43, false), // outstretched bangle edge
        ("action_east", 2, 43, 45, 0xBC8B43, true), // hand during return
        ("action_east", 2, 42, 44, 0xBC8B43, false), // near bangle edge
        ("action_east", 3, 48, 45, 0x763F21, true), // extended finger shadow
        ("action_east", 3, 49, 43, 0xBC8B43, false), // outer bangle edge
        ("action_east", 4, 46, 46, 0xE3BF7F, true), // returning fingertips
        ("action_east", 4, 45, 44, 0xBC8B43, false), // opposite bangle edge
        ("action_east", 5, 41, 48, 0xEFD89A, true), // lowered hand
        ("action_east", 5, 39, 46, 0xFFF45D, false), // lowered gold center
        ("action_east", 6, 44, 44, 0xBC8B43, true), // far arm at rest
        ("action_east", 6, 36, 45, 0xBC8B43, false), // restored near bangle
        ("kiss_east", 0, 39, 39, 0xEFD89A, true),  // face above mouth
        ("kiss_east", 0, 37, 40, 0x763F21, true),  // jaw shadow
        ("kiss_east", 0, 36, 46, 0xBC8B43, false), // near bangle edge
        ("kiss_east", 1, 43, 40, 0xE3BF7F, true),  // mouth edge while leaning
        ("kiss_east", 1, 35, 46, 0xBC8B43, false), // shifted outer bangle
        ("kiss_east", 2, 46, 38, 0xE3BF7F, true),  // cheek beside mouth
        ("kiss_east", 2, 41, 35, 0xB789D5, false), // closed eye cosmetics
        ("kiss_east", 2, 39, 36, 0xEFD89A, true),  // ear during kiss
        ("kiss_east", 2, 37, 44, 0xBC8B43, false), // raised bangle border
        ("kiss_east", 3, 44, 38, 0xEFD89A, true),  // face during recovery
        ("kiss_east", 3, 39, 36, 0xB789D5, false), // recovering eyelid
        ("kiss_east", 3, 37, 46, 0xBC8B43, false), // recovering inner bangle
        // Accepted connected bangle edges: keep attached hand skin fully covered.
        ("action_south", 2, 36, 45, 0xBC8B43, true),
        ("action_south", 4, 36, 45, 0xBC8B43, true),
        ("action_south", 5, 45, 45, 0xBC8B43, true),
        ("action_east", 0, 38, 46, 0xBC8B43, true),
        ("action_east", 5, 38, 46, 0xBC8B43, true),
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
            let prefix = "beach";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Beach/spr_npc_juniper_{prefix}_{name}.png"
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
        assert_eq!(changed, 2184);

        for r in &profile["regions"].as_array().unwrap()[..278] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-beach-pilot-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: missing ear/hand/waist components and unrestricted color matching.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Beach/spr_npc_juniper_beach_blink_south.png";
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
        ("missing-ear", [35, 36], 0xEFD89A),
        ("missing-hand", [32, 46], 0xEFD89A),
        ("missing-waist", [38, 45], 0xE3BF7F),
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
        ([33, 45], 0xBC8B43, 0x6687AD), // outer bangle border
        ([35, 45], 0xBC8B43, 0x6687AD), // inner bangle border
    ] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(source));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(target));
    }
}
