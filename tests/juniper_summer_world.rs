use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-beach-actions-study and the retained Spring-injured Juniper bundle"]
fn juniper_summer_world_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-beach-actions-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("idle_north", &[19]),
        ("idle_south", &[62]),
        ("idle_east", &[51]),
        ("walk_north", &[19, 17, 19, 11]),
        ("walk_south", &[62, 56, 62, 57]),
        ("walk_east", &[51, 53, 51, 51]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        ("idle_north", 0, &[]),
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
            ],
        ),
        (
            "idle_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (37, 44), (34, 45)],
        ),
        ("walk_north", 0, &[]),
        ("walk_north", 1, &[]),
        ("walk_north", 2, &[]),
        ("walk_north", 3, &[]),
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
            ],
        ),
        (
            "walk_south",
            1,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (33, 46), (44, 45)],
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
            ],
        ),
        (
            "walk_south",
            3,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (35, 45), (46, 46)],
        ),
        (
            "walk_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (37, 44), (34, 45)],
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
            ],
        ),
        (
            "walk_east",
            2,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (37, 44), (34, 45)],
        ),
        (
            "walk_east",
            3,
            &[(39, 34), (42, 34), (38, 35), (43, 35), (35, 46)],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("idle_north", 0, 33, 45, 0xBC8B43, true), // rear hand edge
        ("idle_north", 0, 33, 47, 0x763F21, true), // fingers
        ("idle_north", 0, 37, 52, 0xBC8B43, true), // heel above sandal sole
        ("idle_north", 0, 37, 51, 0xD6CDF4, false), // sandal strap
        ("idle_north", 0, 39, 42, 0x9E77B3, false), // hair over back
        ("idle_south", 0, 38, 33, 0xE3BF7F, false), // circlet corner
        ("idle_south", 0, 40, 33, 0xD264AF, false), // Summer gemstone
        ("idle_south", 0, 39, 36, 0xE3BF7F, true), // nose between eyes
        ("idle_south", 0, 36, 42, 0xBC8B43, true), // bare shoulder
        ("idle_south", 0, 36, 43, 0xEFD89A, true), // bare upper arm
        ("idle_south", 0, 36, 44, 0x763F21, true), // exposed underarm shadow
        ("idle_south", 0, 43, 44, 0x763F21, true), // opposite underarm shadow
        ("idle_south", 0, 39, 42, 0xE3BF7F, true), // neckline
        ("idle_south", 0, 39, 44, 0xE3BF7F, true), // midriff above navel
        ("idle_south", 0, 38, 42, 0xD6CDF4, false), // blouse highlight
        ("idle_south", 0, 33, 45, 0xBC8B43, false), // bracer below gemstone
        ("idle_south", 0, 34, 44, 0xD264AF, false), // bracer gemstone
        ("idle_south", 0, 42, 48, 0xBC8B43, true), // exposed leg through slit
        ("idle_south", 0, 42, 49, 0x9793DC, false), // sandal wrap within slit
        ("idle_south", 0, 42, 50, 0xE3BF7F, true), // shin below wrap
        ("idle_south", 0, 37, 52, 0xBC8B43, true), // bare foot
        ("idle_east", 0, 37, 42, 0xBC8B43, true),  // side shoulder
        ("idle_east", 0, 37, 44, 0xBC8B43, false), // cuff corner
        ("idle_east", 0, 38, 49, 0xBC8B43, true),  // leg above wrap
        ("idle_east", 0, 38, 50, 0x9793DC, false), // side sandal wrap
        ("idle_east", 0, 38, 51, 0xBC8B43, true),  // skin between sandal straps
        ("idle_east", 0, 38, 52, 0x9793DC, false), // lower sandal strap
        ("walk_north", 1, 35, 44, 0xE3BF7F, true), // swinging upper arm
        ("walk_north", 1, 37, 51, 0x763F21, true), // raised heel shadow
        ("walk_north", 3, 37, 54, 0xE3BF7F, true), // stepping heel
        ("walk_south", 1, 36, 45, 0xBC8B43, true), // moving underarm
        ("walk_south", 1, 44, 45, 0xE3BF7F, false), // turned bracer
        ("walk_south", 1, 45, 46, 0xE3BF7F, true), // hand beneath bracer
        ("walk_south", 1, 37, 52, 0xE3BF7F, true), // stepping foot
        ("walk_south", 3, 35, 45, 0xE3BF7F, false), // opposite turned bracer
        ("walk_south", 3, 34, 46, 0xE3BF7F, true), // opposite hand
        ("walk_south", 3, 41, 49, 0xBC8B43, true), // knee in moving slit
        ("walk_east", 1, 36, 43, 0xE3BF7F, true),  // moving bare shoulder
        ("walk_east", 1, 36, 45, 0xBC8B43, false), // swinging cuff corner
        ("walk_east", 1, 44, 45, 0xBC8B43, false), // far cuff corner
        ("walk_east", 1, 37, 50, 0x763F21, true),  // raised foot shadow
        ("walk_east", 3, 35, 46, 0xBC8B43, false), // backward cuff edge
        ("walk_east", 3, 38, 51, 0xBC8B43, true),  // ankle beside strap
        ("walk_east", 3, 43, 52, 0xD6CDF4, false), // opposite sandal strap
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
        assert_eq!(changed, 641);

        for r in &profile["regions"].as_array().unwrap()[..173] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-march-spring-injured-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping an finger shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Summer/spr_npc_juniper_summer_idle_south.png";
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
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(33, 45).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(33, 45).0, rgba(0x6687AD));
}
