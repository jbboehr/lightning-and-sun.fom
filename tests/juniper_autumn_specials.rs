use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
fn juniper_autumn_laugh_aliases_preserve_every_previous_palette_role() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let read =
        |s: &str| -> Value { serde_json::from_slice(&fs::read(root.join(s)).unwrap()).unwrap() };
    let portrait = read("palettes/profiles/juniper-portraits.json");
    let world = read("palettes/profiles/juniper-world-trial.json");
    let mut colors = portrait["source_colors"].as_array().unwrap().clone();
    colors.extend([json!("#BC8B43"), json!("#E8B171"), json!("#F1E791")]);
    colors.extend([json!("#B58E45"), json!("#E0B572")]);
    assert_eq!(world["source_colors"], json!(colors));
    let mut groups = portrait["color_groups"].as_array().unwrap().clone();
    groups.extend([json!(["#BC8B43"]), json!(["#E8B171"]), json!(["#F1E791"])]);
    groups.extend([json!(["#B58E45"]), json!(["#E0B572"])]);
    assert_eq!(world["color_groups"], json!(groups));
    let old = read("palettes/sets/juniper-portraits-trial.json");
    let set = read("palettes/sets/juniper-world-trial.json");
    assert_eq!(set["presets"].as_array().unwrap().len(), 4);
    for (a, b) in old["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(set["presets"].as_array().unwrap())
    {
        let mut expected = a.clone();
        let mut colors = a["colors"].as_array().unwrap().clone();
        colors.extend([
            a["colors"][2].clone(),
            a["colors"][1].clone(),
            a["colors"][0].clone(),
        ]);
        colors.extend([a["colors"][2].clone(), a["colors"][1].clone()]);
        expected["colors"] = json!(colors);
        assert_eq!(*b, expected);
    }
    let mut expected = read("palettes/stylized/juniper-portraits.json")["rgba_map"].clone();
    expected["#BC8B43"] = expected["#D2AB66"].clone();
    expected["#E8B171"] = expected["#E3BF7F"].clone();
    expected["#F1E791"] = expected["#EFD89A"].clone();
    expected["#B58E45"] = expected["#BC8B43"].clone();
    expected["#E0B572"] = expected["#E3BF7F"].clone();
    assert_eq!(
        read("palettes/stylized/juniper-world-trial.json")["rgba_map"],
        expected
    );
}

#[test]
#[ignore = "requires extracted/juniper-wedding-finish-study and the retained Autumn-reading Juniper bundle"]
fn juniper_autumn_specials_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-wedding-finish-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 3] = [
        ("laugh_start_south", &[50]),
        ("laugh_loop_south", &[43, 39]),
        ("laugh_end_south", &[43, 50]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "laugh_start_south",
            0,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (46, 46)],
        ),
        (
            "laugh_loop_south",
            0,
            &[(38, 32), (41, 32), (37, 33), (42, 33)],
        ),
        (
            "laugh_loop_south",
            1,
            &[(38, 32), (41, 32), (37, 33), (42, 33)],
        ),
        (
            "laugh_end_south",
            0,
            &[(38, 32), (41, 32), (37, 33), (42, 33)],
        ),
        (
            "laugh_end_south",
            1,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (46, 46)],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("laugh_start_south", 0, 38, 34, 0xE3BF7F, false), // circlet corner
        ("laugh_start_south", 0, 40, 34, 0xCA3561, false), // circlet gemstone
        ("laugh_start_south", 0, 37, 37, 0x8D80C7, false), // eye cosmetics
        ("laugh_start_south", 0, 37, 36, 0xE3BF7F, true),  // skin above closed eyelid
        ("laugh_start_south", 0, 39, 43, 0xB58E45, true),  // new keyhole shadow alias
        ("laugh_start_south", 0, 40, 43, 0xE0B572, true),  // new keyhole midtone alias
        ("laugh_start_south", 0, 39, 45, 0xB58E45, true),  // midriff upper shadow alias
        ("laugh_start_south", 0, 37, 46, 0xB58E45, true),  // midriff side shadow alias
        ("laugh_start_south", 0, 38, 46, 0xE0B572, true),  // midriff midtone alias
        ("laugh_start_south", 0, 39, 46, 0xF1E791, true), // established highlight between new aliases
        ("laugh_start_south", 0, 32, 43, 0xEFD89A, true), // lifted hand highlight
        ("laugh_start_south", 0, 34, 43, 0x763F21, true), // lifted finger shadow
        ("laugh_start_south", 0, 34, 45, 0xCA3561, false), // wrist gemstone
        ("laugh_start_south", 0, 46, 46, 0xBC8B43, false), // opposite cuff border
        ("laugh_start_south", 0, 45, 46, 0xF8F960, false), // opposite cuff gold
        ("laugh_start_south", 0, 46, 48, 0x763F21, true), // fingers below cuff
        ("laugh_loop_south", 0, 38, 32, 0xE3BF7F, false), // raised circlet
        ("laugh_loop_south", 0, 39, 36, 0x000000, false), // open mouth interior
        ("laugh_loop_south", 0, 41, 38, 0xEFD89A, true),  // cheek next to mouth
        ("laugh_loop_south", 0, 37, 39, 0x763F21, true),  // raised fingertip
        ("laugh_loop_south", 0, 37, 41, 0xEFD89A, true),  // hand above cuff
        ("laugh_loop_south", 0, 38, 41, 0xBC8B43, true),  // hand outline beside wrist
        ("laugh_loop_south", 0, 36, 41, 0xCA3561, false), // wrist gemstone
        ("laugh_loop_south", 0, 36, 42, 0xF8F960, false), // cuff below raised hand
        ("laugh_loop_south", 0, 39, 43, 0xB58E45, true),  // keyhole shadow through loop
        ("laugh_loop_south", 0, 40, 43, 0xE0B572, true),  // keyhole midtone through loop
        ("laugh_loop_south", 0, 41, 46, 0xE0B572, true),  // opposite midriff alias
        ("laugh_loop_south", 0, 45, 46, 0xBC8B43, true),  // far hand outline beside cuff
        ("laugh_loop_south", 0, 46, 45, 0xF8F960, false), // far cuff gold
        ("laugh_loop_south", 1, 37, 34, 0x8D80C7, false), // eyelid cosmetics during bob
        ("laugh_loop_south", 1, 39, 37, 0x000000, false), // mouth interior during bob
        ("laugh_loop_south", 1, 37, 38, 0x763F21, true),  // fingertip during bob
        ("laugh_loop_south", 1, 36, 41, 0xF8F960, false), // moving cuff gold
        ("laugh_loop_south", 1, 39, 42, 0xB58E45, true),  // shifted keyhole shadow
        ("laugh_loop_south", 1, 40, 42, 0xE0B572, true),  // shifted keyhole midtone
        ("laugh_loop_south", 1, 42, 45, 0xB58E45, true),  // shifted midriff shadow
        ("laugh_loop_south", 1, 41, 45, 0xE0B572, true),  // shifted midriff midtone
        ("laugh_end_south", 0, 37, 39, 0x763F21, true),   // finger as laugh ends
        ("laugh_end_south", 0, 39, 43, 0xB58E45, true),   // keyhole alias as laugh ends
        ("laugh_end_south", 0, 40, 43, 0xE0B572, true),   // paired keyhole alias
        ("laugh_end_south", 0, 39, 47, 0xCA3561, false),  // skirt below midriff
        ("laugh_end_south", 1, 38, 34, 0xE3BF7F, false),  // restored circlet
        ("laugh_end_south", 1, 46, 46, 0xBC8B43, false),  // restored cuff border
        ("laugh_end_south", 1, 39, 43, 0xB58E45, true),   // restored keyhole shadow
        ("laugh_end_south", 1, 40, 43, 0xE0B572, true),   // restored keyhole midtone
        ("laugh_end_south", 1, 39, 46, 0xF1E791, true),   // restored midriff highlight
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
        assert_eq!(changed, 225);

        for r in &profile["regions"].as_array().unwrap()[..231] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-autumn-reading-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping a finger shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Autumn/spr_npc_juniper_specialanimation_autumn_laugh_start_south.png";
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
