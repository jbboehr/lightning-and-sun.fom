use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-winter-study and the retained Summer-pilot Juniper bundle"]
fn juniper_summer_actions_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-winter-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 11] = [
        ("blink_south", &[62, 66, 62]),
        ("blink_east", &[51, 55, 51]),
        ("sit_north", &[10]),
        ("sit_south", &[53]),
        ("sit_east", &[33]),
        ("eat_north", &[7, 4, 7]),
        ("eat_south", &[56, 45, 46, 53, 53]),
        ("eat_east", &[34, 38, 32, 34, 38]),
        ("drink_north", &[7, 4, 7]),
        ("drink_south", &[53, 49, 53]),
        ("drink_east", &[33, 33, 33]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
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
            ],
        ),
        (
            "blink_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (37, 44), (34, 45)],
        ),
        (
            "blink_east",
            1,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (37, 44), (34, 45)],
        ),
        (
            "blink_east",
            2,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (37, 44), (34, 45)],
        ),
        ("sit_north", 0, &[]),
        ("sit_south", 0, &[(38, 33), (41, 33), (37, 34), (42, 34)]),
        (
            "sit_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (37, 44)],
        ),
        ("eat_north", 0, &[]),
        ("eat_north", 1, &[]),
        ("eat_north", 2, &[]),
        ("eat_south", 0, &[(38, 33), (41, 33), (37, 34), (42, 34)]),
        ("eat_south", 1, &[(38, 34), (41, 34), (37, 35), (42, 35)]),
        ("eat_south", 2, &[(38, 32), (41, 32)]),
        ("eat_south", 3, &[(38, 34), (41, 34), (37, 35), (42, 35)]),
        ("eat_south", 4, &[(38, 33), (41, 33), (37, 34), (42, 34)]),
        (
            "eat_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (40, 43)],
        ),
        ("eat_east", 1, &[(40, 33), (43, 33), (39, 34), (44, 34)]),
        ("eat_east", 2, &[(39, 32), (42, 32)]),
        (
            "eat_east",
            3,
            &[(39, 34), (42, 34), (38, 35), (43, 35), (40, 44)],
        ),
        ("eat_east", 4, &[(39, 33), (42, 33), (38, 34), (43, 34)]),
        ("drink_north", 0, &[]),
        ("drink_north", 1, &[]),
        ("drink_north", 2, &[]),
        ("drink_south", 0, &[(38, 33), (41, 33), (37, 34), (42, 34)]),
        (
            "drink_south",
            1,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (35, 43)],
        ),
        ("drink_south", 2, &[(38, 33), (41, 33), (37, 34), (42, 34)]),
        (
            "drink_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (37, 44), (40, 44)],
        ),
        (
            "drink_east",
            1,
            &[(37, 33), (40, 33), (36, 34), (41, 34), (39, 43), (37, 44)],
        ),
        (
            "drink_east",
            2,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (37, 44), (40, 44)],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("blink_south", 1, 37, 35, 0xE3BF7F, true), // closed eyelid skin
        ("blink_south", 1, 38, 33, 0xE3BF7F, false), // matching circlet corner
        ("blink_south", 0, 36, 44, 0x763F21, true), // bare underarm shadow
        ("blink_south", 0, 33, 45, 0xBC8B43, false), // cuff below gem
        ("blink_south", 0, 42, 50, 0xE3BF7F, true), // shin through skirt slit
        ("blink_south", 0, 42, 49, 0x9793DC, false), // sandal wrap in slit
        ("blink_east", 1, 38, 35, 0xE3BF7F, true),  // closed side eyelid
        ("blink_east", 0, 37, 44, 0xBC8B43, false), // side cuff edge
        ("blink_east", 0, 38, 51, 0xBC8B43, true),  // ankle between wraps
        ("sit_north", 0, 34, 47, 0x763F21, true),   // rear fingertips
        ("sit_north", 0, 39, 42, 0x9E77B3, false),  // hair over back
        ("sit_south", 0, 36, 42, 0xBC8B43, true),   // bare shoulder
        ("sit_south", 0, 36, 43, 0xE3BF7F, true),   // upper arm
        ("sit_south", 0, 39, 44, 0xE3BF7F, true),   // midriff
        ("sit_south", 0, 35, 44, 0xD264AF, false),  // Summer bracer gem
        ("sit_south", 0, 37, 49, 0xBC8B43, true),   // foot above sole
        ("sit_south", 0, 37, 48, 0xD6CDF4, false),  // sandal strap
        ("sit_east", 0, 37, 44, 0xBC8B43, false),   // side cuff corner
        ("sit_east", 0, 41, 49, 0xBC8B43, true),    // seated ankle
        ("sit_east", 0, 44, 47, 0xD6CDF4, false),   // far sandal strap
        ("eat_north", 1, 34, 47, 0x763F21, true),   // rear finger detail
        ("eat_south", 0, 36, 43, 0x763F21, true),   // bent upper arm
        ("eat_south", 1, 39, 40, 0x9E2626, false),  // mouth interior
        ("eat_south", 1, 37, 43, 0xBC8B43, true),   // moving shoulder
        ("eat_south", 2, 38, 32, 0xE3BF7F, false),  // shifted circlet corner
        ("eat_south", 2, 38, 33, 0xB789D5, false),  // eyelid cosmetics
        ("eat_south", 2, 36, 40, 0x763F21, true),   // lifted hand detail
        ("eat_south", 2, 37, 35, 0xEFD89A, true),   // cheek beside open mouth
        ("eat_south", 3, 37, 45, 0xBC8B43, true),   // raised fist edge
        ("eat_east", 0, 40, 43, 0xBC8B43, false),   // rotated cuff shadow
        ("eat_east", 0, 39, 43, 0x763F21, true),    // forearm beside cuff
        ("eat_east", 1, 41, 39, 0x9E2626, false),   // side mouth interior
        ("eat_east", 1, 45, 42, 0xBC8B43, true),    // extended fingers
        ("eat_east", 2, 39, 32, 0xE3BF7F, false),   // moving side circlet
        ("eat_east", 3, 40, 44, 0xBC8B43, false),   // lowering cuff shadow
        ("drink_north", 0, 46, 45, 0xBC8B43, true), // far raised hand
        ("drink_south", 0, 35, 42, 0xE8B171, true), // existing arm midtone alias
        ("drink_south", 1, 36, 40, 0xE8B171, true), // lifted hand alias
        ("drink_south", 1, 35, 43, 0x763F21, false), // separable rotated bracer edge
        ("drink_east", 0, 41, 43, 0xE8B171, true),  // hand midtone
        ("drink_east", 0, 37, 44, 0xBC8B43, false), // cuff left edge
        ("drink_east", 0, 40, 44, 0xE8B171, false), // separable cuff right edge
        ("drink_east", 2, 40, 44, 0xE8B171, false), // returning cuff right edge
        ("drink_east", 1, 40, 40, 0xE8B171, true),  // lifted side hand
        ("drink_east", 1, 39, 43, 0xBC8B43, false), // raised cuff right edge
        ("drink_east", 1, 37, 44, 0xBC8B43, false), // raised cuff lower edge
        ("drink_east", 1, 36, 44, 0xE8B171, true),  // forearm beneath coupled cuff edge
        ("drink_east", 0, 36, 43, 0x010101, false), // arm outline
        // Known art exceptions: these bracer corners share a component with
        // exposed hands or forearm. Keeping full skin coverage also changes them.
        ("sit_south", 0, 34, 45, 0xE3BF7F, true),
        ("sit_south", 0, 45, 45, 0xE3BF7F, true),
        ("eat_south", 0, 36, 45, 0xE3BF7F, true),
        ("eat_south", 0, 45, 45, 0xE3BF7F, true),
        ("eat_south", 1, 45, 45, 0xE3BF7F, true),
        ("eat_south", 1, 37, 46, 0xE3BF7F, true),
        ("eat_south", 2, 45, 45, 0xE3BF7F, true),
        ("eat_south", 3, 45, 45, 0xE3BF7F, true),
        ("eat_south", 4, 34, 45, 0xE3BF7F, true),
        ("eat_south", 4, 45, 45, 0xE3BF7F, true),
        ("eat_east", 4, 39, 44, 0xE3BF7F, true),
        ("drink_south", 0, 45, 45, 0xE3BF7F, true),
        ("drink_south", 1, 45, 45, 0xE3BF7F, true),
        ("drink_south", 2, 45, 45, 0xE3BF7F, true),
        ("drink_east", 1, 36, 43, 0xE8B171, true),
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
        assert_eq!(changed, 1162);

        for r in &profile["regions"].as_array().unwrap()[..179] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-summer-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping an finger shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Summer/spr_npc_juniper_summer_blink_south.png";
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
