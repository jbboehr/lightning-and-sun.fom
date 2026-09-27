use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
fn juniper_world_retains_all_portrait_regions_groups_and_target_roles() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let read =
        |s: &str| -> Value { serde_json::from_slice(&fs::read(root.join(s)).unwrap()).unwrap() };
    let old = read("palettes/profiles/juniper-portraits.json");
    let world = read("palettes/profiles/juniper-world-trial.json");
    assert_eq!(old["regions"].as_array().unwrap().len(), 132);
    assert_eq!(world["regions"].as_array().unwrap().len(), 201);
    assert_eq!(
        &world["regions"].as_array().unwrap()[..132],
        old["regions"].as_array().unwrap()
    );
    assert_eq!(
        &world["source_colors"].as_array().unwrap()[..12],
        old["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &world["color_groups"].as_array().unwrap()[..4],
        old["color_groups"].as_array().unwrap()
    );
    assert_eq!(world["source_colors"][12], "#BC8B43");
    assert_eq!(world["color_groups"][4], json!(["#BC8B43"]));
    let old = read("palettes/sets/juniper-portraits-trial.json");
    let world = read("palettes/sets/juniper-world-trial.json");
    assert_eq!(world["profile"], "../profiles/juniper-world-trial.json");
    assert_eq!(world["presets"].as_array().unwrap().len(), 4);
    for (a, b) in old["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(world["presets"].as_array().unwrap())
    {
        assert_eq!(a["id"], b["id"]);
        assert_eq!(a["label"], b["label"]);
        assert_eq!(
            &b["colors"].as_array().unwrap()[..12],
            a["colors"].as_array().unwrap()
        );
        assert_eq!(b["colors"][12], a["colors"][2]);
    }
}

#[test]
#[ignore = "requires extracted/juniper-summer-specials-study and the retained Wedding-finish Juniper bundle"]
fn juniper_world_covers_skin_without_recoloring_shared_jewelry_shades() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-summer-specials-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("idle_east", &[41]),
        ("idle_north", &[11]),
        ("idle_south", &[50]),
        ("walk_east", &[41, 46, 41, 37]),
        ("walk_north", &[11, 9, 11, 8]),
        ("walk_south", &[50, 48, 50, 42]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21];
    // The circlet, bracers and trailing skirt trim reuse skin shades.
    // These literal material positions are excluded from the expected skin map.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "idle_south",
            0,
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
            "idle_east",
            0,
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
        ("idle_north", 0, &[(37, 49)]),
        (
            "walk_south",
            0,
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
            "walk_south",
            1,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (33, 46),
                (44, 45),
                (41, 48),
            ],
        ),
        (
            "walk_south",
            2,
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
            "walk_south",
            3,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 45),
                (46, 46),
                (41, 48),
            ],
        ),
        (
            "walk_east",
            0,
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
        (
            "walk_east",
            1,
            &[
                (39, 34),
                (42, 34),
                (38, 35),
                (43, 35),
                (37, 44),
                (33, 45),
                (36, 45),
                (44, 45),
                (46, 45),
                (37, 48),
                (36, 49),
            ],
        ),
        (
            "walk_east",
            2,
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
        (
            "walk_east",
            3,
            &[
                (39, 34),
                (42, 34),
                (38, 35),
                (43, 35),
                (35, 46),
                (38, 49),
                (40, 49),
                (43, 49),
            ],
        ),
        ("walk_north", 0, &[(37, 49)]),
        ("walk_north", 1, &[(36, 48), (38, 49), (45, 49), (44, 50)]),
        ("walk_north", 2, &[(37, 49)]),
        ("walk_north", 3, &[(34, 49), (43, 49), (36, 50), (43, 50)]),
    ];
    // Frame numbers are zero-based; x coordinates are within the frame.
    let landmarks = [
        ("idle_south", 0, 38, 33, 0xE3BF7F, false), // circlet upper border
        ("idle_south", 0, 37, 34, 0xBC8B43, false), // circlet side shadow
        ("idle_south", 0, 38, 34, 0xFFF45D, false), // gold highlight
        ("idle_south", 0, 39, 34, 0x3CB9D8, false), // gemstone
        ("idle_south", 0, 39, 36, 0xE3BF7F, true),  // nose bridge
        ("idle_south", 0, 39, 37, 0xEFD89A, true),  // face highlight
        ("idle_south", 0, 37, 38, 0xBC8B43, true),  // cheek shadow
        ("idle_south", 0, 37, 39, 0x763F21, true),  // dark jaw edge
        ("idle_south", 0, 36, 38, 0x715E8E, false), // hair beside cheek
        ("idle_south", 0, 39, 40, 0xBC8B43, true),  // neck
        ("idle_south", 0, 39, 42, 0xBC8B43, true),  // chest shading
        ("idle_south", 0, 38, 42, 0x646392, false), // bodice edge
        ("idle_south", 0, 35, 43, 0xBC8B43, true),  // exposed upper arm
        ("idle_south", 0, 39, 44, 0xBC8B43, true),  // midriff shadow
        ("idle_south", 0, 33, 45, 0xBC8B43, false), // bracer shadow
        ("idle_south", 0, 32, 46, 0xEFD89A, true),  // hand below bracer
        ("idle_south", 0, 33, 47, 0x763F21, true),  // finger detail
        ("idle_south", 0, 41, 47, 0xBC8B43, false), // gold skirt clasp
        ("idle_south", 0, 42, 48, 0xBC8B43, true),  // thigh shadow beside trim
        ("idle_south", 0, 41, 49, 0xEFD89A, true),  // exposed thigh
        ("idle_south", 0, 38, 53, 0x4C4B74, false), // boot
        ("idle_east", 0, 37, 44, 0xBC8B43, false),  // side bracer shadow
        ("idle_east", 0, 34, 47, 0xBC8B43, true),   // side hand shadow
        ("idle_east", 0, 36, 49, 0x763F21, false),  // trailing skirt point
        ("idle_north", 0, 33, 45, 0xBC8B43, true),  // rear hand edge
        ("idle_north", 0, 37, 49, 0xBC8B43, false), // rear hem
        ("idle_north", 0, 39, 42, 0x9E77B3, false), // hair highlight
        ("walk_south", 1, 44, 45, 0xE3BF7F, false), // turned bracer
        ("walk_south", 1, 45, 46, 0xE3BF7F, true),  // turned hand
        ("walk_south", 3, 35, 45, 0xE3BF7F, false), // opposite turned bracer
        ("walk_south", 3, 34, 46, 0xE3BF7F, true),  // opposite turned hand
        ("walk_north", 1, 38, 49, 0xBC8B43, false), // moving rear hem
        ("walk_north", 1, 33, 48, 0xBC8B43, true),  // moving hand edge
        ("walk_east", 3, 40, 49, 0xBC8B43, false),  // folded hem
        ("walk_east", 3, 39, 50, 0xBC8B43, true),   // leg above boot
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
        let targets = [0, 1, 12, 4].map(|i| {
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
        assert_eq!(changed, 496);
        for r in &profile["regions"].as_array().unwrap()[..132] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-world-wedding-finish-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior portrait output {id} {path}");
            }
        }
    }

    // Practical controls: dropping a detached hand, or merging color groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Spring/spr_npc_juniper_spring_idle_south.png";
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
        .retain(|s| *s != json!([32, 46]));
    let omitted = apply_control(
        "missing-hand",
        missing,
        map.clone(),
        profile["color_groups"].clone(),
    );
    assert_eq!(omitted.get_pixel(32, 46).0, rgba(0xEFD89A));
    assert_ne!(omitted.get_pixel(32, 46), correct.get_pixel(32, 46));
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(33, 45).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(33, 45).0, rgba(0x6687AD));
}
