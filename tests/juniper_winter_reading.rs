use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-beach-finish-study and the retained Winter-standard Juniper bundle"]
fn juniper_winter_reading_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-beach-finish-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 3] = [
        ("read_sit_start_south", &[22, 16, 22]),
        ("read_sit_loop_south", &[18, 26, 18, 26]),
        ("read_sit_end_south", &[26, 12, 22]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "read_sit_start_south",
            0,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (34, 45),
                (45, 45),
            ],
        ),
        (
            "read_sit_start_south",
            1,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (34, 45),
                (45, 45),
            ],
        ),
        (
            "read_sit_start_south",
            2,
            &[(38, 34), (41, 34), (37, 35), (42, 35)],
        ),
        (
            "read_sit_loop_south",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (35, 40), (44, 40)],
        ),
        (
            "read_sit_loop_south",
            1,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (35, 41), (44, 41)],
        ),
        (
            "read_sit_loop_south",
            2,
            &[(37, 33), (40, 33), (36, 34), (41, 34), (35, 40), (44, 40)],
        ),
        (
            "read_sit_loop_south",
            3,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (35, 41), (44, 41)],
        ),
        (
            "read_sit_end_south",
            0,
            &[(38, 34), (41, 34), (37, 35), (42, 35)],
        ),
        (
            "read_sit_end_south",
            1,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 41),
                (44, 41),
                (34, 45),
                (45, 45),
            ],
        ),
        (
            "read_sit_end_south",
            2,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (35, 40),
                (44, 40),
                (34, 45),
                (45, 45),
            ],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("read_sit_start_south", 0, 38, 33, 0xE3BF7F, false), // circlet corner
        ("read_sit_start_south", 0, 40, 33, 0x3CB9D8, false), // cyan gemstone
        ("read_sit_start_south", 0, 35, 36, 0xEFD89A, true),  // exposed ear
        ("read_sit_start_south", 0, 37, 39, 0x763F21, true),  // cheek shadow
        ("read_sit_start_south", 0, 39, 40, 0xBC8B43, true),  // chin
        ("read_sit_start_south", 0, 35, 40, 0xBC8B43, false), // shoulder decoration
        ("read_sit_start_south", 0, 34, 45, 0xBC8B43, false), // cuff above glove
        ("read_sit_start_south", 0, 34, 46, 0x8A2C5F, false), // covered hand
        ("read_sit_start_south", 0, 39, 42, 0x645049, false), // warm bodice
        ("read_sit_start_south", 1, 37, 36, 0xE3BF7F, true),  // skin above closed eye
        ("read_sit_start_south", 1, 37, 37, 0xFC639B, false), // cosmetics
        ("read_sit_start_south", 1, 38, 39, 0xE3BF7F, true),  // cheek above raised book
        ("read_sit_start_south", 1, 38, 41, 0xC9AF9C, false), // warm page shadow
        ("read_sit_start_south", 1, 39, 41, 0xF6E4D7, false), // page highlight
        ("read_sit_start_south", 1, 37, 43, 0xBF54A3, false), // book cover
        ("read_sit_start_south", 1, 45, 45, 0xBC8B43, false), // opposite cuff
        ("read_sit_start_south", 2, 35, 40, 0xF6E4D7, false), // raised page beside face
        ("read_sit_start_south", 2, 37, 40, 0x763F21, true),  // jaw beside page
        ("read_sit_start_south", 2, 39, 41, 0xBC8B43, true),  // chin above pages
        ("read_sit_start_south", 2, 45, 46, 0x8A2C5F, false), // small glove beyond book
        ("read_sit_loop_south", 0, 39, 33, 0xE3BF7F, false),  // turned circlet corner
        ("read_sit_loop_south", 0, 36, 36, 0xEFD89A, true),   // turned ear
        ("read_sit_loop_south", 0, 38, 39, 0x763F21, true),   // turned jaw
        ("read_sit_loop_south", 0, 40, 40, 0xBC8B43, true),   // small chin beside hair
        ("read_sit_loop_south", 0, 35, 40, 0xBC8B43, false),  // shoulder decoration
        ("read_sit_loop_south", 0, 32, 42, 0xF6E4D7, false),  // wide left page
        ("read_sit_loop_south", 0, 38, 46, 0xC9AF9C, false),  // page fold
        ("read_sit_loop_south", 1, 37, 36, 0xE3BF7F, true),   // blink skin
        ("read_sit_loop_south", 1, 37, 37, 0xFC639B, false),  // blink cosmetics
        ("read_sit_loop_south", 1, 39, 41, 0xBC8B43, true),   // chin above book
        ("read_sit_loop_south", 1, 44, 41, 0xBC8B43, false),  // shoulder corner
        ("read_sit_loop_south", 1, 39, 46, 0xF6E4D7, false),  // central page highlight
        ("read_sit_loop_south", 2, 43, 36, 0xEFD89A, true),   // opposite exposed ear
        ("read_sit_loop_south", 2, 41, 39, 0x763F21, true),   // opposite cheek shadow
        ("read_sit_loop_south", 2, 40, 33, 0xE3BF7F, false),  // opposite circlet edge
        ("read_sit_loop_south", 2, 45, 42, 0xC9AF9C, false),  // lifted right page
        ("read_sit_loop_south", 3, 39, 40, 0xE3BF7F, true),   // returning lower face
        ("read_sit_loop_south", 3, 40, 47, 0xC9AF9C, false),  // returning page fold
        ("read_sit_end_south", 0, 37, 36, 0xE3BF7F, true),    // skin above closed eye
        ("read_sit_end_south", 0, 35, 40, 0xF6E4D7, false),   // closing page beside face
        ("read_sit_end_south", 0, 45, 46, 0x8A2C5F, false),   // covered opposite hand
        ("read_sit_end_south", 1, 37, 36, 0xFC639B, false),   // cosmetics above open eye
        ("read_sit_end_south", 1, 35, 37, 0xEFD89A, true),    // ear above book
        ("read_sit_end_south", 1, 45, 45, 0xBC8B43, false),   // closing cuff corner
        ("read_sit_end_south", 1, 38, 47, 0x9F3E7B, false),   // book binding
        ("read_sit_end_south", 2, 34, 45, 0xBC8B43, false),   // restored cuff
        ("read_sit_end_south", 2, 33, 46, 0x8A2C5F, false),   // restored glove
        ("read_sit_end_south", 2, 37, 39, 0x763F21, true),    // restored jaw
        ("read_sit_end_south", 2, 39, 43, 0x836C64, false),   // warm clothing
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
        assert_eq!(changed, 208);

        for r in &profile["regions"].as_array().unwrap()[..261] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-winter-standard-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: missing cheek/ear components and unrestricted color matching.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Winter/spr_npc_juniper_specialanimation_winter_read_sit_start_south.png";
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
        ("missing-cheek-shadow", [37, 39], 0x763F21),
        ("missing-ear", [35, 36], 0xEFD89A),
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
        ([38, 33], 0xE3BF7F, 0x7F9FBD), // circlet
        ([35, 40], 0xBC8B43, 0x6687AD), // shoulder decoration
        ([34, 45], 0xBC8B43, 0x6687AD), // glove cuff
    ] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(source));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(target));
    }
}
