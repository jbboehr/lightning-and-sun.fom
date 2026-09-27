use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-autumn-reading-study and the retained Spring-actions Juniper bundle"]
fn juniper_spring_standard_covers_skin_and_preserves_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-autumn-reading-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 5] = [
        ("action_east", &[34, 39, 35, 39, 35, 34, 41]),
        ("action_north", &[6, 6, 6, 6, 6, 8, 11]),
        ("action_south", &[46, 45, 44, 45, 44, 46, 50]),
        ("kiss_east", &[38, 37, 42, 41]),
        ("sleep_east", &[41]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171];
    // Literal source-material exclusions: circlet, bracers and skirt trim.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "action_east",
            0,
            &[
                (40, 34),
                (43, 34),
                (39, 35),
                (44, 35),
                (40, 46),
                (38, 49),
                (37, 50),
            ],
        ),
        (
            "action_east",
            1,
            &[
                (42, 33),
                (45, 33),
                (41, 34),
                (46, 34),
                (45, 44),
                (39, 45),
                (39, 46),
                (39, 47),
                (38, 48),
                (37, 49),
            ],
        ),
        (
            "action_east",
            2,
            &[
                (42, 33),
                (45, 33),
                (41, 34),
                (46, 34),
                (42, 44),
                (43, 45),
                (39, 45),
                (39, 46),
                (39, 47),
                (38, 48),
                (37, 49),
            ],
        ),
        (
            "action_east",
            3,
            &[
                (42, 33),
                (45, 33),
                (41, 34),
                (46, 34),
                (45, 44),
                (39, 45),
                (39, 46),
                (39, 47),
                (38, 48),
                (37, 49),
            ],
        ),
        (
            "action_east",
            4,
            &[
                (42, 33),
                (45, 33),
                (41, 34),
                (46, 34),
                (42, 44),
                (43, 45),
                (39, 45),
                (39, 46),
                (39, 47),
                (38, 48),
                (37, 49),
            ],
        ),
        (
            "action_east",
            5,
            &[
                (40, 34),
                (43, 34),
                (39, 35),
                (44, 35),
                (40, 46),
                (38, 49),
                (37, 50),
            ],
        ),
        (
            "action_east",
            6,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (37, 44),
                (34, 45),
                (37, 48),
                (36, 49),
            ],
        ),
        ("action_north", 0, &[(37, 49)]),
        ("action_north", 1, &[(37, 48)]),
        ("action_north", 2, &[(37, 48)]),
        ("action_north", 3, &[(37, 48)]),
        ("action_north", 4, &[(37, 48)]),
        ("action_north", 5, &[(37, 49)]),
        ("action_north", 6, &[(37, 49)]),
        (
            "action_south",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (33, 46),
                (44, 46),
                (41, 48),
            ],
        ),
        (
            "action_south",
            1,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (34, 45),
                (44, 45),
                (45, 45),
                (41, 47),
            ],
        ),
        (
            "action_south",
            2,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (36, 45),
                (38, 45),
                (44, 45),
                (45, 45),
                (41, 47),
            ],
        ),
        (
            "action_south",
            3,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (34, 45),
                (44, 45),
                (45, 45),
                (41, 47),
            ],
        ),
        (
            "action_south",
            4,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (36, 45),
                (38, 45),
                (44, 45),
                (45, 45),
                (41, 47),
            ],
        ),
        (
            "action_south",
            5,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (33, 46),
                (44, 46),
                (41, 48),
            ],
        ),
        (
            "action_south",
            6,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (33, 45),
                (35, 45),
                (44, 45),
                (46, 45),
                (41, 47),
            ],
        ),
        (
            "kiss_east",
            0,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (34, 46), (35, 50)],
        ),
        (
            "kiss_east",
            1,
            &[
                (40, 34),
                (43, 34),
                (39, 35),
                (44, 35),
                (38, 45),
                (35, 46),
                (36, 50),
            ],
        ),
        (
            "kiss_east",
            2,
            &[(42, 33), (45, 33), (41, 34), (46, 34), (37, 48)],
        ),
        (
            "kiss_east",
            3,
            &[
                (40, 34),
                (43, 34),
                (39, 35),
                (44, 35),
                (38, 45),
                (35, 46),
                (36, 50),
            ],
        ),
        (
            "sleep_east",
            0,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (42, 42),
                (37, 48),
                (36, 49),
            ],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("action_east", 0, 38, 43, 0xBC8B43, true), // upper arm shadow
        ("action_east", 0, 39, 43, 0xEFD89A, true), // arm highlight
        ("action_east", 0, 40, 46, 0xBC8B43, false), // gold bracer shadow
        ("action_east", 0, 40, 48, 0x763F21, true), // fingers below bracer
        ("action_east", 0, 37, 50, 0xBC8B43, false), // trailing hem point
        ("action_east", 1, 45, 44, 0xBC8B43, false), // extended bracer
        ("action_east", 1, 49, 45, 0xBC8B43, true), // extended hand edge
        ("action_east", 1, 39, 45, 0x763F21, false), // dark skirt border
        ("action_east", 1, 40, 48, 0xE3BF7F, true), // exposed leg above boot
        ("action_east", 1, 40, 49, 0xBC8B43, true), // exposed leg shadow
        ("action_east", 2, 42, 44, 0xBC8B43, false), // bent bracer border
        ("action_east", 2, 43, 45, 0xBC8B43, false), // bracer lower edge
        ("action_east", 2, 44, 46, 0xBC8B43, true), // bent hand edge
        ("action_east", 6, 38, 49, 0xE3BF7F, true), // settling leg
        ("action_north", 0, 33, 43, 0xBC8B43, true), // rear hand
        ("action_north", 0, 37, 49, 0xBC8B43, false), // rear hem
        ("action_north", 1, 39, 42, 0x9E77B3, false), // hair highlight
        ("action_south", 0, 38, 34, 0xE3BF7F, false), // circlet corner
        ("action_south", 0, 37, 39, 0xBC8B43, true), // cheek shadow
        ("action_south", 0, 44, 46, 0x763F21, false), // far bracer point
        ("action_south", 0, 45, 47, 0xBC8B43, true), // far hand
        ("action_south", 1, 44, 45, 0xBC8B43, false), // far bracer edge
        ("action_south", 1, 45, 46, 0x763F21, true), // far finger
        ("action_south", 2, 36, 45, 0xE3BF7F, false), // bracer highlight
        ("action_south", 2, 36, 46, 0xBC8B43, true), // hand below bracer
        ("action_south", 2, 41, 48, 0xE3BF7F, true), // exposed leg
        ("action_south", 6, 41, 47, 0xBC8B43, false), // clasp between gold trim
        ("kiss_east", 0, 35, 50, 0x763F21, false),  // skirt point
        ("kiss_east", 0, 37, 50, 0xBC8B43, true),   // leg beside skirt
        ("kiss_east", 2, 35, 44, 0xBC8B43, true),   // raised hand
        ("kiss_east", 2, 40, 44, 0xBC8B43, true),   // side midriff
        ("kiss_east", 2, 37, 48, 0x763F21, false),  // trailing skirt edge
        ("sleep_east", 0, 44, 39, 0x763F21, true),  // folded hand detail
        ("sleep_east", 0, 42, 42, 0xBC8B43, false), // folded bracer
        ("sleep_east", 0, 39, 42, 0xEFD89A, true),  // folded arm
        ("sleep_east", 0, 41, 44, 0xBC8B43, true),  // exposed midriff
        ("sleep_east", 0, 38, 49, 0xE3BF7F, true),  // exposed leg
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
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Spring/spr_npc_juniper_spring_{name}.png"
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
        assert_eq!(changed, 825);

        for r in &profile["regions"].as_array().unwrap()[..149] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-actions-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping an upper-arm shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Spring/spr_npc_juniper_spring_action_east.png";
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
        .retain(|s| *s != json!([38, 43]));
    let omitted = apply_control(
        "missing-arm-shadow",
        missing,
        map.clone(),
        profile["color_groups"].clone(),
    );
    assert_eq!(omitted.get_pixel(38, 43).0, rgba(0xBC8B43));
    assert_ne!(omitted.get_pixel(38, 43), correct.get_pixel(38, 43));
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(40, 46).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(40, 46).0, rgba(0x6687AD));
}
