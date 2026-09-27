use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-autumn-specials-study and the retained Spring-standard Juniper bundle"]
fn juniper_spring_reactions_cover_skin_and_preserve_books_and_jewelry() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-autumn-specials-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("shocked_start_south", &[52]),
        ("shocked_loop_south", &[66]),
        ("shocked_end_south", &[52]),
        ("read_sit_start_south", &[30, 24, 28]),
        ("read_sit_loop_south", &[20, 28, 20, 28]),
        ("read_sit_end_south", &[32, 20, 30]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171];
    // Literal source-material exclusions: circlet, bracers and skirt trim.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "shocked_start_south",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (33, 46),
                (35, 46),
                (44, 46),
                (46, 46),
                (41, 48),
                (44, 49),
                (34, 50),
            ],
        ),
        (
            "shocked_loop_south",
            0,
            &[
                (38, 31),
                (41, 31),
                (33, 37),
                (46, 37),
                (33, 39),
                (46, 39),
                (41, 45),
                (32, 46),
                (33, 47),
            ],
        ),
        (
            "shocked_end_south",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (33, 46),
                (35, 46),
                (44, 46),
                (46, 46),
                (41, 48),
                (44, 49),
                (34, 50),
            ],
        ),
        (
            "read_sit_start_south",
            0,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (34, 45),
                (44, 45),
                (41, 45),
                (40, 46),
                (42, 46),
            ],
        ),
        (
            "read_sit_start_south",
            1,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (34, 45), (45, 45)],
        ),
        (
            "read_sit_start_south",
            2,
            &[(38, 34), (41, 34), (37, 35), (42, 35)],
        ),
        (
            "read_sit_loop_south",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34)],
        ),
        (
            "read_sit_loop_south",
            1,
            &[(38, 34), (41, 34), (37, 35), (42, 35)],
        ),
        (
            "read_sit_loop_south",
            2,
            &[(37, 33), (40, 33), (36, 34), (41, 34)],
        ),
        (
            "read_sit_loop_south",
            3,
            &[(38, 34), (41, 34), (37, 35), (42, 35)],
        ),
        (
            "read_sit_end_south",
            0,
            &[(38, 34), (41, 34), (37, 35), (42, 35)],
        ),
        (
            "read_sit_end_south",
            1,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (34, 45), (45, 45)],
        ),
        (
            "read_sit_end_south",
            2,
            &[
                (38, 33),
                (41, 33),
                (37, 34),
                (42, 34),
                (34, 45),
                (44, 45),
                (41, 45),
                (40, 46),
                (42, 46),
            ],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("shocked_start_south", 0, 38, 34, 0xE3BF7F, false), // circlet corner
        ("shocked_start_south", 0, 37, 36, 0xE3BF7F, true),  // forehead above closed eye
        ("shocked_start_south", 0, 37, 37, 0xB789D5, false), // cosmetic eyelid
        ("shocked_start_south", 0, 35, 44, 0xBC8B43, true),  // upper arm
        ("shocked_start_south", 0, 33, 46, 0xBC8B43, false), // gold bracer
        ("shocked_start_south", 0, 33, 48, 0x763F21, true),  // hand detail
        ("shocked_start_south", 0, 41, 48, 0xBC8B43, false), // clasp
        ("shocked_start_south", 0, 42, 49, 0xBC8B43, true),  // exposed leg
        ("shocked_start_south", 0, 44, 49, 0xBC8B43, false), // opposite hem
        ("shocked_start_south", 0, 34, 50, 0xBC8B43, false), // trailing hem
        ("shocked_loop_south", 0, 32, 33, 0xBC8B43, true),   // raised hand shadow
        ("shocked_loop_south", 0, 30, 34, 0xE3BF7F, true),   // raised fingertips
        ("shocked_loop_south", 0, 33, 37, 0xE3BF7F, false),  // raised bracer highlight
        ("shocked_loop_south", 0, 34, 38, 0xE3BF7F, true),   // upper arm beside bracer
        ("shocked_loop_south", 0, 33, 39, 0xBC8B43, false),  // raised bracer lower edge
        ("shocked_loop_south", 0, 37, 33, 0xC2B9BE, false),  // enlarged eye upper edge
        ("shocked_loop_south", 0, 37, 35, 0xECF0E9, false),  // enlarged eye white
        ("shocked_loop_south", 0, 39, 36, 0x410808, false),  // mouth interior
        ("shocked_loop_south", 0, 39, 37, 0x9E2626, false),  // tongue
        ("shocked_loop_south", 0, 43, 46, 0xBC8B43, true),   // leg above bent boot
        ("shocked_end_south", 0, 44, 44, 0xBC8B43, true),    // other arm settling
        ("read_sit_start_south", 0, 41, 45, 0xBC8B43, false), // seated clasp
        ("read_sit_start_south", 0, 33, 46, 0xE3BF7F, true), // hand beside skirt
        ("read_sit_start_south", 1, 34, 45, 0xBC8B43, false), // bracer beside closed book
        ("read_sit_start_south", 1, 34, 46, 0xEFD89A, true), // hand below bracer
        ("read_sit_start_south", 1, 38, 41, 0xC9AF9C, false), // warm page shadow
        ("read_sit_start_south", 1, 39, 41, 0xF6E4D7, false), // pale page
        ("read_sit_start_south", 2, 39, 41, 0xBC8B43, true), // neck above opening book
        ("read_sit_start_south", 2, 34, 46, 0xEFD89A, true), // fingers behind open book
        ("read_sit_loop_south", 0, 39, 33, 0xE3BF7F, false), // tilted circlet
        ("read_sit_loop_south", 0, 39, 42, 0xBC8B43, true),  // chest above pages
        ("read_sit_loop_south", 0, 32, 42, 0xF6E4D7, false), // open page corner
        ("read_sit_loop_south", 2, 41, 38, 0xBC8B43, true),  // opposite tilted cheek
        ("read_sit_loop_south", 2, 46, 44, 0xF174B3, false), // book cover
        ("read_sit_end_south", 0, 35, 47, 0x763F21, true),   // fingers while closing book
        ("read_sit_end_south", 1, 44, 47, 0x763F21, true),   // opposite fingers
        ("read_sit_end_south", 2, 40, 46, 0xBC8B43, false),  // returned skirt trim
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
            let prefix = if name.starts_with("read_") {
                "specialanimation_spring"
            } else {
                "spring"
            };
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Spring/spr_npc_juniper_{prefix}_{name}.png"
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
        assert_eq!(changed, 430);

        for r in &profile["regions"].as_array().unwrap()[..154] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-standard-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping an upper-arm shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Spring/spr_npc_juniper_spring_shocked_start_south.png";
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
        .retain(|s| *s != json!([35, 44]));
    let omitted = apply_control(
        "missing-arm-shadow",
        missing,
        map.clone(),
        profile["color_groups"].clone(),
    );
    assert_eq!(omitted.get_pixel(35, 44).0, rgba(0xBC8B43));
    assert_ne!(omitted.get_pixel(35, 44), correct.get_pixel(35, 44));
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(33, 46).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(33, 46).0, rgba(0x6687AD));
}
