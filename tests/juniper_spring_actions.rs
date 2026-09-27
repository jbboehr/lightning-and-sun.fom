use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
fn juniper_drink_alias_appends_a_midtone_role_without_changing_prior_roles() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let read =
        |s: &str| -> Value { serde_json::from_slice(&fs::read(root.join(s)).unwrap()).unwrap() };
    let portrait = read("palettes/profiles/juniper-portraits.json");
    let world = read("palettes/profiles/juniper-world-trial.json");
    let mut colors = portrait["source_colors"].as_array().unwrap().clone();
    colors.extend([json!("#BC8B43"), json!("#E8B171")]);
    // Autumn midriff highlight; previous source roles retain their indices.
    colors.push(json!("#F1E791"));
    assert_eq!(world["source_colors"], json!(colors));
    let mut groups = portrait["color_groups"].as_array().unwrap().clone();
    groups.extend([json!(["#BC8B43"]), json!(["#E8B171"])]);
    groups.push(json!(["#F1E791"]));
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
        colors.extend([a["colors"][2].clone(), a["colors"][1].clone()]);
        colors.push(a["colors"][0].clone());
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
#[ignore = "requires extracted/juniper-autumn-standard-study and the retained first-world Juniper bundle"]
fn juniper_spring_actions_cover_raised_arms_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-autumn-standard-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 11] = [
        ("blink_east", &[41, 45, 41]),
        ("blink_south", &[50, 54, 50]),
        ("drink_east", &[30, 32, 30]),
        ("drink_north", &[7, 4, 7]),
        ("drink_south", &[35, 39, 35]),
        ("eat_east", &[27, 31, 29, 27, 32]),
        ("eat_north", &[7, 4, 7]),
        ("eat_south", &[36, 38, 31, 40, 32]),
        ("sit_east", &[28]),
        ("sit_north", &[10]),
        ("sit_south", &[32]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171];
    // Literal source-material exclusions: circlet, bracers and skirt trim.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "blink_east",
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
            "blink_east",
            1,
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
            "blink_east",
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
            "blink_south",
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
            "blink_south",
            1,
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
            "blink_south",
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
            "sit_east",
            0,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (37, 44),
                (44, 45),
                (41, 46),
                (39, 47),
            ],
        ),
        ("sit_north", 0, &[]),
        (
            "sit_south",
            0,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (34, 45),
                (45, 45),
                (41, 45),
                (40, 46),
                (42, 46),
            ],
        ),
        (
            "drink_east",
            0,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (44, 45),
                (41, 46),
                (37, 47),
                (39, 47),
            ],
        ),
        (
            "drink_east",
            1,
            &[
                (37, 33),
                (40, 33),
                (36, 34),
                (41, 34),
                (44, 45),
                (41, 46),
                (37, 47),
                (39, 47),
            ],
        ),
        (
            "drink_east",
            2,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (44, 45),
                (41, 46),
                (37, 47),
                (39, 47),
            ],
        ),
        ("drink_north", 0, &[]),
        ("drink_north", 1, &[]),
        ("drink_north", 2, &[]),
        (
            "drink_south",
            0,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (45, 45),
                (41, 45),
                (40, 46),
                (42, 46),
            ],
        ),
        (
            "drink_south",
            1,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (45, 45),
                (37, 46),
                (40, 46),
                (42, 46),
            ],
        ),
        (
            "drink_south",
            2,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (45, 45),
                (41, 45),
                (40, 46),
                (42, 46),
            ],
        ),
        (
            "eat_east",
            0,
            &[
                (39, 33),
                (42, 33),
                (38, 34),
                (43, 34),
                (40, 43),
                (44, 45),
                (41, 46),
                (37, 47),
                (39, 47),
            ],
        ),
        (
            "eat_east",
            1,
            &[
                (40, 33),
                (43, 33),
                (39, 34),
                (44, 34),
                (44, 45),
                (41, 46),
                (37, 47),
                (39, 47),
            ],
        ),
        (
            "eat_east",
            2,
            &[
                (39, 31),
                (42, 31),
                (38, 32),
                (43, 32),
                (44, 45),
                (41, 46),
                (37, 47),
                (39, 47),
            ],
        ),
        (
            "eat_east",
            3,
            &[
                (39, 34),
                (42, 34),
                (38, 35),
                (43, 35),
                (40, 44),
                (41, 46),
                (37, 47),
                (39, 47),
            ],
        ),
        (
            "eat_east",
            4,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (37, 47), (39, 47)],
        ),
        ("eat_north", 0, &[]),
        ("eat_north", 1, &[]),
        ("eat_north", 2, &[]),
        (
            "eat_south",
            0,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (45, 45),
                (41, 45),
                (40, 46),
                (42, 46),
            ],
        ),
        (
            "eat_south",
            1,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (45, 45), (42, 46)],
        ),
        (
            "eat_south",
            2,
            &[(38, 31), (41, 31), (42, 32), (45, 45), (40, 46), (42, 46)],
        ),
        (
            "eat_south",
            3,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (45, 45),
                (40, 46),
                (42, 46),
            ],
        ),
        (
            "eat_south",
            4,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (34, 45),
                (45, 45),
                (41, 45),
                (40, 46),
                (42, 46),
            ],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("blink_south", 1, 37, 35, 0xE3BF7F, true), // closed eyelid skin
        ("blink_south", 1, 38, 33, 0xE3BF7F, false), // same shade on circlet
        ("blink_south", 0, 37, 34, 0xBC8B43, false), // circlet border
        ("blink_south", 0, 37, 38, 0xBC8B43, true), // cheek shadow
        ("blink_east", 1, 38, 35, 0xE3BF7F, true),  // side closed eyelid
        ("blink_east", 0, 36, 49, 0x763F21, false), // trailing skirt point
        ("sit_south", 0, 39, 42, 0xBC8B43, true),   // chest shading
        ("sit_south", 0, 38, 42, 0x646392, false),  // bodice beside chest
        ("sit_south", 0, 39, 44, 0xBC8B43, true),   // exposed midriff
        ("sit_south", 0, 34, 45, 0xBC8B43, false),  // bracer shadow
        ("sit_south", 0, 33, 46, 0xE3BF7F, true),   // seated hand
        ("sit_south", 0, 41, 45, 0xBC8B43, false),  // seated clasp
        ("sit_south", 0, 40, 46, 0xBC8B43, false),  // seated hem shadow
        ("sit_south", 0, 42, 46, 0xBC8B43, false),  // opposite hem shadow
        ("sit_east", 0, 37, 44, 0xBC8B43, false),   // side bracer
        ("sit_east", 0, 34, 46, 0xEFD89A, true),    // side hand highlight
        ("sit_east", 0, 44, 45, 0xBC8B43, false),   // side skirt border
        ("sit_north", 0, 34, 45, 0xBC8B43, true),   // rear hand edge
        ("sit_north", 0, 39, 42, 0x9E77B3, false),  // rear hair highlight
        ("drink_south", 0, 34, 42, 0xEFD89A, true), // raised arm highlight
        ("drink_south", 0, 35, 42, 0xE8B171, true), // new arm midtone alias
        ("drink_south", 1, 36, 40, 0xE8B171, true), // lifted hand midtone
        ("drink_south", 1, 34, 43, 0xE8B171, true), // forearm midtone
        ("drink_south", 0, 45, 45, 0xBC8B43, false), // far bracer
        ("drink_east", 0, 41, 43, 0xE8B171, true),  // side forearm midtone
        ("drink_east", 0, 36, 43, 0x010101, false), // side arm outline
        ("drink_east", 1, 40, 40, 0xE8B171, true),  // lifted side hand
        ("drink_east", 1, 41, 43, 0xE8B171, true),  // exposed chest alias
        ("drink_north", 0, 46, 45, 0xBC8B43, true), // far raised hand edge
        ("eat_north", 1, 34, 47, 0x763F21, true),   // rear finger detail
        ("eat_south", 1, 39, 40, 0x9E2626, false),  // mouth interior
        ("eat_east", 1, 41, 39, 0x9E2626, false),   // side mouth interior
        ("eat_east", 0, 40, 43, 0xBC8B43, false),   // bent bracer shadow
        ("eat_east", 1, 45, 42, 0xBC8B43, true),    // extended hand shading
        // Known art exceptions: these material-edge pixels touch exposed
        // skin in the same source-color component, so they follow that skin.
        ("eat_south", 1, 40, 46, 0xBC8B43, true), // touching hem edge
        ("eat_south", 2, 41, 45, 0xBC8B43, true), // touching clasp edge
        ("eat_east", 4, 39, 44, 0xE3BF7F, true),  // touching bracer edge
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
        let mut alias_pixels = 0;
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
                        if i == 4 {
                            alias_pixels += 1;
                        }
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
        assert_eq!(changed, 911);
        assert_eq!(alias_pixels, 23);
        for r in &profile["regions"].as_array().unwrap()[..138] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-world-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior portrait output {id} {path}");
            }
        }
    }

    // Practical controls: dropping the new arm-midtone component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Spring/spr_npc_juniper_spring_drink_south.png";
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
        .retain(|s| *s != json!([35, 42]));
    let omitted = apply_control(
        "missing-arm-midtone",
        missing,
        map.clone(),
        profile["color_groups"].clone(),
    );
    assert_eq!(omitted.get_pixel(35, 42).0, rgba(0xE8B171));
    assert_ne!(omitted.get_pixel(35, 42), correct.get_pixel(35, 42));
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(45, 45).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(45, 45).0, rgba(0x6687AD));
}
