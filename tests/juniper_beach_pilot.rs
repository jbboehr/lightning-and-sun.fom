use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-beach-pilot-study and the retained March Winter-injured Juniper bundle"]
fn juniper_beach_pilot_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-beach-pilot-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("idle_north", &[62]),
        ("idle_south", &[82]),
        ("idle_east", &[74]),
        ("walk_north", &[62, 58, 62, 57]),
        ("walk_south", &[82, 78, 82, 78]),
        ("walk_east", &[74, 77, 74, 68]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // Literal source-material exclusions: separable bangle borders.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        ("idle_north", 0, &[(33, 45), (35, 45), (44, 45), (46, 45)]),
        ("idle_south", 0, &[(33, 45), (35, 45), (44, 45), (46, 45)]),
        ("idle_east", 0, &[(34, 45), (36, 45)]),
        ("walk_north", 0, &[(33, 45), (35, 45), (44, 45), (46, 45)]),
        ("walk_north", 1, &[(33, 46)]),
        ("walk_north", 2, &[(33, 45), (35, 45), (44, 45), (46, 45)]),
        ("walk_north", 3, &[(46, 46)]),
        ("walk_south", 0, &[(33, 45), (35, 45), (44, 45), (46, 45)]),
        ("walk_south", 1, &[(33, 46), (44, 46)]),
        ("walk_south", 2, &[(33, 45), (35, 45), (44, 45), (46, 45)]),
        ("walk_south", 3, &[(35, 46), (46, 46)]),
        ("walk_east", 0, &[(34, 45), (36, 45)]),
        ("walk_east", 1, &[(33, 45), (36, 45), (44, 45), (46, 45)]),
        ("walk_east", 2, &[(34, 45), (36, 45)]),
        ("walk_east", 3, &[(35, 46)]),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("idle_north", 0, 35, 36, 0xEFD89A, true), // ear outside hair
        ("idle_north", 0, 39, 41, 0xE3BF7F, true), // exposed back
        ("idle_north", 0, 36, 43, 0xEFD89A, true), // upper arm
        ("idle_north", 0, 32, 46, 0xEFD89A, true), // hand below bangle
        ("idle_north", 0, 39, 45, 0xEFD89A, true), // exposed waist
        ("idle_north", 0, 37, 48, 0xE3BF7F, true), // leg beside wrap
        ("idle_north", 0, 37, 53, 0xE3BF7F, true), // heel
        ("idle_north", 0, 33, 45, 0xBC8B43, false), // outer bangle border
        ("idle_north", 0, 35, 45, 0xBC8B43, false), // inner bangle border
        ("idle_north", 0, 34, 45, 0xFFF45D, false), // gold bangle center
        ("idle_north", 0, 41, 25, 0x5A3668, false), // hair
        ("idle_south", 0, 39, 36, 0xEFD89A, true), // face
        ("idle_south", 0, 37, 39, 0x763F21, true), // jaw
        ("idle_south", 0, 39, 41, 0xE3BF7F, true), // neck
        ("idle_south", 0, 39, 42, 0xEFD89A, true), // chest between wraps
        ("idle_south", 0, 39, 45, 0xEFD89A, true), // midriff
        ("idle_south", 0, 37, 50, 0xE3BF7F, true), // lower leg
        ("idle_south", 0, 37, 53, 0xEFD89A, true), // toes
        ("idle_south", 0, 35, 27, 0xDF8D4B, false), // new warm hair tie shade, not skin
        ("idle_south", 0, 36, 27, 0xFFD565, false), // hair tie highlight
        ("idle_south", 0, 44, 45, 0xBC8B43, false), // opposite bangle border
        ("idle_east", 0, 40, 36, 0xEFD89A, true),  // profile face
        ("idle_east", 0, 38, 39, 0x763F21, true),  // profile jaw
        ("idle_east", 0, 36, 44, 0xEFD89A, true),  // near arm
        ("idle_east", 0, 44, 44, 0xBC8B43, true),  // far arm above gold bangle
        ("idle_east", 0, 44, 46, 0xE3BF7F, true),  // far hand
        ("idle_east", 0, 38, 53, 0xBC8B43, true),  // shaded heel
        ("idle_east", 0, 41, 44, 0xBC8B43, true),  // skin at lower top edge
        ("idle_east", 0, 38, 44, 0x443963, false), // dark wrap edge
        ("idle_east", 0, 39, 43, 0xE8E1FF, false), // pale wrap
        ("idle_east", 0, 44, 45, 0xFFF45D, false), // far bangle center
        ("idle_east", 0, 34, 45, 0xBC8B43, false), // near bangle border
        ("walk_north", 0, 43, 43, 0xEFD89A, true), // opposite shoulder
        ("walk_north", 1, 35, 47, 0xBC8B43, true), // hand joined to bangle edge
        ("walk_north", 1, 45, 47, 0xBC8B43, true), // far hand joined to shaded bangle
        ("walk_north", 1, 33, 46, 0xBC8B43, false), // separable opposite border
        ("walk_north", 2, 35, 45, 0xBC8B43, false), // restored bangle edge
        ("walk_north", 3, 34, 47, 0xBC8B43, true), // far hand after swing reverses
        ("walk_north", 3, 44, 47, 0xBC8B43, true), // near hand after swing reverses
        ("walk_north", 3, 46, 46, 0xBC8B43, false), // separable outer border
        ("walk_south", 0, 39, 44, 0xBC8B43, true), // midriff shadow
        ("walk_south", 1, 44, 46, 0x763F21, false), // separable shaded bangle edge
        ("walk_south", 1, 45, 47, 0xBC8B43, true), // far hand below jewelry
        ("walk_south", 2, 46, 45, 0xBC8B43, false), // restored opposite bangle
        ("walk_south", 3, 35, 46, 0x763F21, false), // opposite shaded bangle edge
        ("walk_south", 3, 34, 47, 0xBC8B43, true), // far hand after swing reverses
        ("walk_east", 0, 34, 47, 0xBC8B43, true),  // near hand shadow
        ("walk_east", 1, 32, 46, 0xEFD89A, true),  // forward hand
        ("walk_east", 1, 47, 47, 0xBC8B43, true),  // rear hand shadow
        ("walk_east", 1, 36, 45, 0xBC8B43, false), // near bangle border
        ("walk_east", 1, 46, 45, 0xBC8B43, false), // far bangle border
        ("walk_east", 2, 36, 45, 0xBC8B43, false), // restored near bangle
        ("walk_east", 3, 37, 47, 0xEFD89A, true),  // hand after swing reverses
        ("walk_east", 3, 44, 47, 0x763F21, true),  // far hand shadow
        ("walk_east", 3, 35, 46, 0xBC8B43, false), // separable moving bangle
        // Accepted exceptions: fourteen bangle-edge pixels inseparable from skin components.
        ("walk_north", 1, 35, 46, 0xBC8B43, true),
        ("walk_north", 1, 44, 46, 0xBC8B43, true),
        ("walk_north", 1, 45, 46, 0xBC8B43, true),
        ("walk_north", 1, 46, 46, 0xBC8B43, true),
        ("walk_north", 3, 33, 46, 0xBC8B43, true),
        ("walk_north", 3, 34, 46, 0xBC8B43, true),
        ("walk_north", 3, 35, 46, 0xBC8B43, true),
        ("walk_north", 3, 44, 46, 0xBC8B43, true),
        ("walk_south", 1, 35, 46, 0xBC8B43, true),
        ("walk_south", 1, 45, 46, 0xBC8B43, true),
        ("walk_south", 1, 46, 46, 0xBC8B43, true),
        ("walk_south", 3, 33, 46, 0xBC8B43, true),
        ("walk_south", 3, 34, 46, 0xBC8B43, true),
        ("walk_south", 3, 44, 46, 0xBC8B43, true),
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
        assert_eq!(changed, 1070);

        for r in &profile["regions"].as_array().unwrap()[..272] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-march-winter-injured-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: missing ear/hand/waist components and unrestricted color matching.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Beach/spr_npc_juniper_beach_idle_south.png";
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
