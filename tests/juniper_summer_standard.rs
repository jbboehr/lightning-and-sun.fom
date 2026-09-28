use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-winter-specials-study and the retained Summer-actions Juniper bundle"]
fn juniper_summer_standard_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-winter-specials-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 5] = [
        ("action_north", &[14, 14, 14, 14, 14, 16, 19]),
        ("action_south", &[58, 53, 52, 53, 52, 58, 62]),
        ("action_east", &[43, 54, 45, 54, 45, 43, 51]),
        ("sleep_east", &[51]),
        ("kiss_east", &[45, 45, 51, 49]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        ("action_north", 0, &[]),
        ("action_north", 1, &[]),
        ("action_north", 2, &[]),
        ("action_north", 3, &[]),
        ("action_north", 4, &[]),
        ("action_north", 5, &[]),
        ("action_north", 6, &[]),
        (
            "action_south",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (36, 45),
                (33, 46),
                (35, 46),
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
            ],
        ),
        (
            "action_south",
            2,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (44, 45), (45, 45)],
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
            ],
        ),
        (
            "action_south",
            4,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (44, 45), (45, 45)],
        ),
        (
            "action_south",
            5,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (36, 45),
                (33, 46),
                (35, 46),
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
            ],
        ),
        ("action_east", 0, &[(40, 34), (43, 34), (39, 35), (44, 35)]),
        ("action_east", 1, &[(42, 33), (45, 33), (41, 34), (46, 34)]),
        (
            "action_east",
            2,
            &[(42, 33), (45, 33), (41, 34), (46, 34), (42, 44), (43, 45)],
        ),
        ("action_east", 3, &[(42, 33), (45, 33), (41, 34), (46, 34)]),
        (
            "action_east",
            4,
            &[(42, 33), (45, 33), (41, 34), (46, 34), (42, 44), (43, 45)],
        ),
        ("action_east", 5, &[(40, 34), (43, 34), (39, 35), (44, 35)]),
        (
            "action_east",
            6,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (37, 44), (34, 45)],
        ),
        ("sleep_east", 0, &[(39, 33), (42, 33), (38, 34), (43, 34)]),
        (
            "kiss_east",
            0,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (34, 46)],
        ),
        (
            "kiss_east",
            1,
            &[(40, 34), (43, 34), (39, 35), (44, 35), (38, 45), (35, 46)],
        ),
        (
            "kiss_east",
            2,
            &[(42, 33), (45, 33), (41, 34), (46, 34), (38, 45)],
        ),
        (
            "kiss_east",
            3,
            &[(40, 34), (43, 34), (39, 35), (44, 35), (38, 45), (35, 46)],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("action_north", 0, 33, 43, 0xBC8B43, true), // rear fingers
        ("action_north", 0, 37, 51, 0xD6CDF4, false), // sandal strap
        ("action_north", 0, 37, 52, 0xBC8B43, true), // ankle under strap
        ("action_north", 1, 35, 46, 0x763F21, true), // moving finger shadow
        ("action_north", 3, 39, 42, 0x9E77B3, false), // hair covering the back
        ("action_south", 0, 38, 34, 0xE3BF7F, false), // circlet corner
        ("action_south", 0, 37, 35, 0xBC8B43, false), // circlet side
        ("action_south", 0, 35, 37, 0xEFD89A, true), // ear
        ("action_south", 0, 39, 43, 0xE3BF7F, true), // chest through blouse
        ("action_south", 0, 38, 43, 0xD6CDF4, false), // blouse beside chest
        ("action_south", 0, 36, 45, 0xBC8B43, false), // separable bracer upper corner
        ("action_south", 0, 33, 46, 0xBC8B43, false), // separable bracer lower corner
        ("action_south", 0, 35, 46, 0xBC8B43, false), // separable bracer inner corner
        ("action_south", 0, 45, 47, 0xBC8B43, true), // hand joined to bracer border
        ("action_south", 0, 34, 48, 0x763F21, true), // near finger shadow
        ("action_south", 1, 44, 45, 0xBC8B43, false), // far cuff separable in this pose
        ("action_south", 1, 45, 45, 0xBC8B43, false), // far cuff inner border
        ("action_south", 1, 45, 46, 0x763F21, true), // finger below cuff
        ("action_south", 2, 38, 46, 0xEFD89A, true), // moving hand below coupled corner
        ("action_south", 2, 42, 48, 0xBC8B43, true), // shin through skirt slit
        ("action_south", 2, 42, 49, 0x9793DC, false), // sandal wrap through slit
        ("action_south", 6, 33, 45, 0xBC8B43, false), // returning cuff
        ("action_east", 0, 40, 34, 0xE3BF7F, false), // shifted circlet
        ("action_east", 0, 40, 47, 0xEFD89A, true),  // hand below coupled cuff
        ("action_east", 1, 42, 33, 0xE3BF7F, false), // moved circlet
        ("action_east", 1, 47, 44, 0xEFD89A, true),  // extended hand
        ("action_east", 1, 46, 43, 0xD264AF, false), // cuff gem
        ("action_east", 2, 42, 44, 0xBC8B43, false), // angled bracer left corner
        ("action_east", 2, 43, 45, 0xBC8B43, false), // angled bracer bottom corner
        ("action_east", 2, 45, 44, 0xEFD89A, true),  // exposed hand beside bracer
        ("action_east", 6, 37, 44, 0xBC8B43, false), // returning side cuff
        ("action_east", 6, 44, 45, 0xEFD89A, true),  // far hand beside hem
        ("sleep_east", 0, 38, 35, 0xE3BF7F, true),   // closed eyelid skin
        ("sleep_east", 0, 38, 36, 0xB789D5, false),  // eyelid cosmetics
        ("sleep_east", 0, 39, 41, 0xBC8B43, true),   // upper arm beside coupled cuff
        ("sleep_east", 0, 40, 42, 0xE3BF7F, true),   // bent forearm
        ("sleep_east", 0, 41, 41, 0xD264AF, false),  // bent cuff gem
        ("sleep_east", 0, 42, 42, 0xFFF45D, false),  // bent cuff gold
        ("sleep_east", 0, 38, 49, 0xBC8B43, true),   // bare ankle
        ("kiss_east", 0, 34, 46, 0xBC8B43, false),   // starting cuff edge
        ("kiss_east", 1, 38, 45, 0xBC8B43, false),   // moving cuff inner edge
        ("kiss_east", 2, 38, 45, 0xBC8B43, false),   // angled cuff bottom edge
        ("kiss_east", 2, 39, 43, 0xEFD89A, true),    // upper arm beside cuff
        ("kiss_east", 2, 36, 44, 0xEFD89A, true),    // extended hand
        // Known art exceptions: these bracer-border pixels share a component
        // with exposed hand or arm pixels. Retaining full skin coverage changes them.
        ("action_south", 0, 44, 46, 0xBC8B43, true),
        ("action_south", 0, 45, 46, 0xBC8B43, true),
        ("action_south", 0, 46, 46, 0xBC8B43, true),
        ("action_south", 2, 38, 45, 0xE3BF7F, true),
        ("action_south", 4, 38, 45, 0xE3BF7F, true),
        ("action_south", 5, 44, 46, 0xBC8B43, true),
        ("action_south", 5, 45, 46, 0xBC8B43, true),
        ("action_south", 5, 46, 46, 0xBC8B43, true),
        ("action_east", 0, 40, 46, 0xE3BF7F, true),
        ("action_east", 2, 44, 45, 0xE3BF7F, true),
        ("action_east", 4, 44, 45, 0xE3BF7F, true),
        ("action_east", 5, 40, 46, 0xE3BF7F, true),
        ("sleep_east", 0, 40, 41, 0xBC8B43, true),
        ("sleep_east", 0, 41, 42, 0xE3BF7F, true),
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
            let prefix = "summer";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Summer/spr_npc_juniper_{prefix}_{name}.png"
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
        assert_eq!(changed, 1069);

        for r in &profile["regions"].as_array().unwrap()[..190] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-summer-actions-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping a finger shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Summer/spr_npc_juniper_summer_action_south.png";
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
        .retain(|s| *s != json!([34, 48]));
    let omitted = apply_control(
        "missing-finger-shadow",
        missing,
        map.clone(),
        profile["color_groups"].clone(),
    );
    assert_eq!(omitted.get_pixel(34, 48).0, rgba(0x763F21));
    assert_ne!(omitted.get_pixel(34, 48), correct.get_pixel(34, 48));
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(33, 46).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(33, 46).0, rgba(0x6687AD));
}
