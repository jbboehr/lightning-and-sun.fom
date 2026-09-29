use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-beach-actions-study and the retained Winter-reading Juniper bundle"]
fn juniper_winter_specials_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-beach-actions-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 3] = [
        ("laugh_start_south", &[26]),
        ("laugh_loop_south", &[21, 17]),
        ("laugh_end_south", &[21, 26]),
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
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (35, 45),
                (46, 46),
            ],
        ),
        (
            "laugh_loop_south",
            0,
            &[(38, 32), (41, 32), (37, 33), (42, 33), (44, 41), (46, 44)],
        ),
        (
            "laugh_loop_south",
            1,
            &[(38, 32), (41, 32), (37, 33), (42, 33), (44, 40), (46, 43)],
        ),
        (
            "laugh_end_south",
            0,
            &[(38, 32), (41, 32), (37, 33), (42, 33), (44, 41), (46, 44)],
        ),
        (
            "laugh_end_south",
            1,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (35, 45),
                (46, 46),
            ],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("laugh_start_south", 0, 38, 34, 0xE3BF7F, false), // circlet corner
        ("laugh_start_south", 0, 40, 34, 0x3CB9D8, false), // cyan gemstone
        ("laugh_start_south", 0, 37, 36, 0xE3BF7F, true),  // skin above closed eye
        ("laugh_start_south", 0, 37, 37, 0xFC639B, false), // cosmetics
        ("laugh_start_south", 0, 35, 37, 0xEFD89A, true),  // exposed ear
        ("laugh_start_south", 0, 37, 40, 0x763F21, true),  // cheek shadow
        ("laugh_start_south", 0, 39, 41, 0xBC8B43, true),  // chin
        ("laugh_start_south", 0, 35, 41, 0xBC8B43, false), // shoulder decoration
        ("laugh_start_south", 0, 35, 45, 0xBC8B43, false), // raised cuff
        ("laugh_start_south", 0, 46, 46, 0xBC8B43, false), // opposite cuff
        ("laugh_start_south", 0, 32, 43, 0xDD426C, false), // pink glove
        ("laugh_start_south", 0, 39, 43, 0x645049, false), // warm bodice
        ("laugh_loop_south", 0, 38, 32, 0xE3BF7F, false),  // raised circlet
        ("laugh_loop_south", 0, 37, 34, 0xFC639B, false),  // raised cosmetics
        ("laugh_loop_south", 0, 37, 36, 0xEFD89A, true),   // cheek above glove
        ("laugh_loop_south", 0, 35, 37, 0xEFD89A, true),   // raised ear
        ("laugh_loop_south", 0, 41, 38, 0xEFD89A, true),   // opposite cheek
        ("laugh_loop_south", 0, 42, 40, 0x763F21, true),   // jaw shadow beside glove
        ("laugh_loop_south", 0, 40, 41, 0xBC8B43, true),   // chin beside glove
        ("laugh_loop_south", 0, 38, 39, 0xDD426C, false),  // glove in front of mouth
        ("laugh_loop_south", 0, 37, 39, 0x60285E, false),  // glove shadow
        ("laugh_loop_south", 0, 36, 41, 0x3CB9D8, false),  // raised wrist gemstone
        ("laugh_loop_south", 0, 44, 41, 0xBC8B43, false),  // opposite shoulder decoration
        ("laugh_loop_south", 0, 46, 44, 0xBC8B43, false),  // opposite cuff
        ("laugh_loop_south", 1, 35, 36, 0xEFD89A, true),   // lowered ear
        ("laugh_loop_south", 1, 37, 36, 0xEFD89A, true),   // small cheek above glove
        ("laugh_loop_south", 1, 41, 38, 0xE3BF7F, true),   // opposite lower face
        ("laugh_loop_south", 1, 42, 39, 0x763F21, true),   // lowered jaw shadow
        ("laugh_loop_south", 1, 38, 38, 0xDD426C, false),  // moving glove in front of mouth
        ("laugh_loop_south", 1, 37, 39, 0x8A2C5F, false),  // glove underside
        ("laugh_loop_south", 1, 44, 40, 0xBC8B43, false),  // lowered shoulder decoration
        ("laugh_loop_south", 1, 46, 43, 0xBC8B43, false),  // lowered cuff
        ("laugh_end_south", 0, 38, 37, 0xEFD89A, true),    // cheek while glove remains raised
        ("laugh_end_south", 0, 42, 40, 0x763F21, true),    // returning jaw
        ("laugh_end_south", 0, 39, 39, 0xDD426C, false),   // returning glove
        ("laugh_end_south", 0, 46, 44, 0xBC8B43, false),   // returning cuff
        ("laugh_end_south", 1, 37, 36, 0xE3BF7F, true),    // restored closed-eye skin
        ("laugh_end_south", 1, 35, 37, 0xEFD89A, true),    // restored ear
        ("laugh_end_south", 1, 37, 37, 0xFC639B, false),   // restored cosmetics
        ("laugh_end_south", 1, 35, 45, 0xBC8B43, false),   // restored cuff
        ("laugh_end_south", 1, 46, 47, 0xDD426C, false),   // opposite glove
        ("laugh_end_south", 1, 39, 44, 0x836C64, false),   // warm clothing
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
        assert_eq!(changed, 111);

        for r in &profile["regions"].as_array().unwrap()[..264] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-winter-reading-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: missing cheek/ear components and unrestricted color matching.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Winter/spr_npc_juniper_specialanimation_winter_laugh_start_south.png";
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
        ("missing-cheek-shadow", [37, 40], 0x763F21),
        ("missing-ear", [35, 37], 0xEFD89A),
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
        ([38, 34], 0xE3BF7F, 0x7F9FBD), // circlet
        ([35, 41], 0xBC8B43, 0x6687AD), // shoulder decoration
        ([35, 45], 0xBC8B43, 0x6687AD), // glove cuff
    ] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(source));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(target));
    }
}
