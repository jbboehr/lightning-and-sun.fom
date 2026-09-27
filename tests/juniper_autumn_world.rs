use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
fn juniper_autumn_highlight_alias_preserves_every_previous_palette_role() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let read =
        |s: &str| -> Value { serde_json::from_slice(&fs::read(root.join(s)).unwrap()).unwrap() };
    let portrait = read("palettes/profiles/juniper-portraits.json");
    let world = read("palettes/profiles/juniper-world-trial.json");
    let mut colors = portrait["source_colors"].as_array().unwrap().clone();
    colors.extend([json!("#BC8B43"), json!("#E8B171"), json!("#F1E791")]);
    assert_eq!(world["source_colors"], json!(colors));
    let mut groups = portrait["color_groups"].as_array().unwrap().clone();
    groups.extend([json!(["#BC8B43"]), json!(["#E8B171"]), json!(["#F1E791"])]);
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
        expected["colors"] = json!(colors);
        assert_eq!(*b, expected);
    }
    let mut expected = read("palettes/stylized/juniper-portraits.json")["rgba_map"].clone();
    expected["#BC8B43"] = expected["#D2AB66"].clone();
    expected["#E8B171"] = expected["#E3BF7F"].clone();
    expected["#F1E791"] = expected["#EFD89A"].clone();
    assert_eq!(
        read("palettes/stylized/juniper-world-trial.json")["rgba_map"],
        expected
    );
}

#[test]
#[ignore = "requires extracted/juniper-autumn-actions-study and the retained Summer-injured Juniper bundle"]
fn juniper_autumn_world_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-autumn-actions-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("idle_north", &[12]),
        ("idle_south", &[44]),
        ("idle_east", &[37]),
        ("walk_north", &[12, 6, 12, 7]),
        ("walk_south", &[44, 39, 44, 39]),
        ("walk_east", &[37, 40, 37, 33]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        ("idle_north", 0, &[(33, 45), (46, 45)]),
        (
            "idle_south",
            0,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (33, 45), (46, 45)],
        ),
        (
            "idle_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (34, 45)],
        ),
        ("walk_north", 0, &[(33, 45), (46, 45)]),
        ("walk_north", 1, &[(33, 46)]),
        ("walk_north", 2, &[(33, 45), (46, 45)]),
        ("walk_north", 3, &[(46, 46)]),
        (
            "walk_south",
            0,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (33, 45), (46, 45)],
        ),
        (
            "walk_south",
            1,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (33, 46),
                (45, 46),
                (46, 46),
            ],
        ),
        (
            "walk_south",
            2,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (33, 45), (46, 45)],
        ),
        (
            "walk_south",
            3,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (33, 46),
                (34, 46),
                (46, 46),
            ],
        ),
        (
            "walk_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (34, 45)],
        ),
        (
            "walk_east",
            1,
            &[(39, 34), (42, 34), (38, 35), (43, 35), (46, 45)],
        ),
        (
            "walk_east",
            2,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (34, 45)],
        ),
        (
            "walk_east",
            3,
            &[(39, 34), (42, 34), (38, 35), (43, 35), (35, 46)],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("idle_north", 0, 33, 45, 0xBC8B43, false), // warm cuff border above hand
        ("idle_north", 0, 46, 45, 0xBC8B43, false), // matching far cuff behind hair
        ("idle_north", 0, 34, 45, 0xF8F960, false), // Autumn gold cuff
        ("idle_north", 0, 33, 47, 0x763F21, true),  // fingers
        ("idle_north", 0, 37, 52, 0x3A2B5B, false), // Autumn boot replaces Summer skin
        ("idle_north", 0, 39, 42, 0x9E77B3, false), // hair over back
        ("idle_south", 0, 38, 33, 0xE3BF7F, false), // circlet corner
        ("idle_south", 0, 40, 33, 0xCA3561, false), // Autumn gemstone
        ("idle_south", 0, 39, 36, 0xE3BF7F, true),  // nose between eyes
        ("idle_south", 0, 35, 36, 0xEFD89A, true),  // ear
        ("idle_south", 0, 37, 36, 0xC2B9BE, false), // eye cosmetics
        ("idle_south", 0, 36, 42, 0x1C1627, false), // covered upper arm
        ("idle_south", 0, 38, 42, 0x675F91, false), // blouse beside neckline
        ("idle_south", 0, 39, 42, 0xBC8B43, true),  // neckline shadow
        ("idle_south", 0, 40, 42, 0xE3BF7F, true),  // neckline light
        ("idle_south", 0, 39, 44, 0xBC8B43, true),  // upper midriff shadow
        ("idle_south", 0, 38, 45, 0xE3BF7F, true),  // exposed waist next to new highlight
        ("idle_south", 0, 39, 45, 0xF1E791, true),  // new highlight alias
        ("idle_south", 0, 40, 45, 0xF1E791, true),  // second alias pixel
        ("idle_south", 0, 39, 46, 0xCA3561, false), // red fabric below waist
        ("idle_south", 0, 45, 45, 0xF8F960, false), // far gold cuff
        ("idle_south", 0, 46, 47, 0x763F21, true),  // far fingers below cuff
        ("idle_east", 0, 34, 45, 0xBC8B43, false),  // side cuff border
        ("idle_east", 0, 35, 45, 0xFFF45D, false),  // side gold cuff
        ("idle_east", 0, 44, 45, 0xFBCC5A, false),  // shaded gold cuff
        ("idle_east", 0, 44, 46, 0xE3BF7F, true),   // hand below shaded cuff
        ("idle_east", 0, 34, 47, 0xBC8B43, true),   // hand outline below near cuff
        ("idle_east", 0, 40, 45, 0xEFD89A, true), // existing highlight in equivalent waist geometry
        ("idle_east", 0, 40, 44, 0xBC8B43, true), // waist shadow
        ("idle_east", 0, 39, 52, 0x3D3C66, false), // boot
        ("walk_north", 1, 33, 46, 0xBC8B43, false), // swinging cuff
        ("walk_north", 1, 33, 47, 0xEFD89A, true), // swinging hand
        ("walk_north", 3, 46, 46, 0xBC8B43, false), // opposite cuff
        ("walk_north", 3, 33, 47, 0x763F21, true), // far finger visible below hair
        ("walk_south", 1, 45, 46, 0xE3BF7F, false), // turned cuff uses a skin-matching color
        ("walk_south", 1, 46, 47, 0x763F21, true), // hand below turned cuff
        ("walk_south", 3, 34, 46, 0xE3BF7F, false), // opposite turned cuff
        ("walk_south", 3, 33, 47, 0x763F21, true), // opposite hand shadow
        ("walk_east", 1, 46, 45, 0xBC8B43, false), // far moving cuff border
        ("walk_east", 1, 32, 46, 0xEFD89A, true), // reaching fingertips
        ("walk_east", 3, 35, 46, 0xBC8B43, false), // backward cuff edge
        ("walk_east", 3, 35, 47, 0xEFD89A, true), // backward hand
        ("walk_south", 0, 39, 45, 0xF1E791, true), // new highlight throughout walk
        ("walk_south", 0, 40, 45, 0xF1E791, true), // new highlight throughout walk
        ("walk_south", 1, 39, 46, 0xF1E791, true), // new highlight throughout walk
        ("walk_south", 1, 40, 46, 0xF1E791, true), // new highlight throughout walk
        ("walk_south", 2, 39, 45, 0xF1E791, true), // new highlight throughout walk
        ("walk_south", 2, 40, 45, 0xF1E791, true), // new highlight throughout walk
        ("walk_south", 3, 39, 46, 0xF1E791, true), // new highlight throughout walk
        ("walk_south", 3, 40, 46, 0xF1E791, true), // new highlight throughout walk
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
        assert_eq!(changed, 443);

        for r in &profile["regions"].as_array().unwrap()[..206] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-march-summer-injured-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping an finger shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Autumn/spr_npc_juniper_autumn_idle_south.png";
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
