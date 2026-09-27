use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-autumn-reading-study and the retained Spring-reactions Juniper bundle"]
fn juniper_spring_specials_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-autumn-reading-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("laugh_start_south", &[53]),
        ("laugh_loop_south", &[52, 50]),
        ("laugh_end_south", &[52, 53]),
        ("charm_start_south", &[59, 70, 55, 54]),
        ("charm_loop_south", &[52, 52, 56, 52]),
        ("charm_end_south", &[52]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171];
    // Literal source-material exclusions: circlet, bracers and skirt trim.
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
                (34, 45),
                (44, 46),
                (46, 46),
                (41, 48),
                (44, 49),
                (34, 50),
            ],
        ),
        (
            "laugh_loop_south",
            0,
            &[(38, 32), (41, 32), (37, 33), (42, 33), (46, 43), (42, 48)],
        ),
        (
            "laugh_loop_south",
            1,
            &[(38, 32), (41, 32), (37, 33), (42, 33), (46, 42), (42, 47)],
        ),
        (
            "laugh_end_south",
            0,
            &[(38, 32), (41, 32), (37, 33), (42, 33), (46, 43), (42, 48)],
        ),
        (
            "laugh_end_south",
            1,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (34, 45),
                (44, 46),
                (46, 46),
                (41, 48),
                (44, 49),
                (34, 50),
            ],
        ),
        (
            "charm_start_south",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (35, 44),
                (34, 45),
                (45, 45),
                (41, 48),
            ],
        ),
        (
            "charm_start_south",
            1,
            &[(38, 32), (41, 32), (37, 33), (42, 33), (41, 47)],
        ),
        (
            "charm_start_south",
            2,
            &[
                (37, 34),
                (40, 34),
                (36, 35),
                (41, 35),
                (38, 45),
                (41, 48),
                (34, 50),
            ],
        ),
        (
            "charm_start_south",
            3,
            &[(37, 33), (40, 33), (36, 34), (41, 34), (38, 46), (41, 47)],
        ),
        (
            "charm_loop_south",
            0,
            &[(37, 33), (40, 33), (36, 34), (41, 34), (38, 45), (41, 47)],
        ),
        (
            "charm_loop_south",
            1,
            &[(37, 33), (40, 33), (36, 34), (41, 34), (38, 45), (41, 47)],
        ),
        (
            "charm_loop_south",
            2,
            &[(37, 33), (40, 33), (36, 34), (41, 34), (38, 45), (41, 47)],
        ),
        (
            "charm_loop_south",
            3,
            &[(37, 33), (40, 33), (36, 34), (41, 34), (38, 45), (41, 47)],
        ),
        (
            "charm_end_south",
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
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("laugh_start_south", 0, 38, 34, 0xE3BF7F, false), // circlet corner
        ("laugh_start_south", 0, 37, 36, 0xE3BF7F, true),  // forehead above eyelid
        ("laugh_start_south", 0, 34, 43, 0x763F21, true),  // raised hand detail
        ("laugh_start_south", 0, 34, 45, 0xBC8B43, false), // raised bracer edge
        ("laugh_start_south", 0, 44, 46, 0xBC8B43, false), // opposite bracer
        ("laugh_start_south", 0, 42, 49, 0xBC8B43, true),  // exposed leg
        ("laugh_start_south", 0, 44, 49, 0xBC8B43, false), // skirt edge beside leg
        ("laugh_loop_south", 0, 38, 39, 0xEFD89A, true),   // hand over mouth
        ("laugh_loop_south", 0, 37, 41, 0xEFD89A, true),   // wrist above bracer
        ("laugh_loop_south", 0, 36, 43, 0xE3BF7F, true),   // bent forearm
        ("laugh_loop_south", 0, 46, 43, 0xBC8B43, false),  // far bracer edge
        ("laugh_loop_south", 0, 46, 45, 0xBC8B43, true),   // far hand shadow
        ("laugh_loop_south", 0, 42, 48, 0xBC8B43, false),  // skirt clasp
        ("laugh_loop_south", 1, 37, 40, 0xEFD89A, true),   // rising wrist
        ("laugh_loop_south", 1, 42, 49, 0xE3BF7F, true),   // settling leg
        ("laugh_end_south", 1, 33, 45, 0x763F21, true),    // lowered fingers
        ("charm_start_south", 0, 35, 44, 0xBC8B43, false), // outstretched bracer edge
        ("charm_start_south", 0, 34, 45, 0xBC8B43, false), // bracer lower edge
        ("charm_start_south", 0, 31, 45, 0xBC8B43, true),  // open palm detail
        ("charm_start_south", 0, 49, 46, 0xE3BF7F, true),  // opposite fingertips
        ("charm_start_south", 1, 35, 43, 0xBC8B43, true),  // upper arm beyond rotated bracer
        ("charm_start_south", 1, 29, 43, 0x763F21, true),  // extended hand outline shade
        ("charm_start_south", 1, 41, 47, 0xBC8B43, false), // skirt clasp
        ("charm_start_south", 2, 38, 45, 0xBC8B43, false), // crossed bracer bottom
        ("charm_start_south", 2, 43, 43, 0x763F21, true),  // crossed fingers
        ("charm_start_south", 2, 34, 50, 0xBC8B43, false), // trailing skirt point
        ("charm_start_south", 3, 38, 46, 0xBC8B43, false), // lowered crossed bracer
        ("charm_loop_south", 0, 36, 43, 0x763F21, true),   // tucked elbow shade
        ("charm_loop_south", 0, 38, 45, 0xBC8B43, false),  // resting bracer
        ("charm_loop_south", 2, 36, 35, 0xE3BF7F, true),   // eyelid skin
        ("charm_loop_south", 2, 37, 33, 0xE3BF7F, false),  // matching circlet corner
        ("charm_end_south", 0, 35, 44, 0xBC8B43, true),    // lowered upper arm
        ("charm_end_south", 0, 33, 46, 0xBC8B43, false),   // lowered bracer
        // Known art exceptions: each bracer-edge midtone is inseparable from
        // the wrist highlight immediately above within the existing group.
        ("laugh_loop_south", 0, 37, 42, 0xE3BF7F, true), // touching bracer edge
        ("laugh_loop_south", 1, 37, 41, 0xE3BF7F, true), // touching bracer edge
        ("laugh_end_south", 0, 37, 42, 0xE3BF7F, true),  // touching bracer edge
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
            let prefix = "specialanimation_spring";
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
        assert_eq!(changed, 762);

        for r in &profile["regions"].as_array().unwrap()[..160] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-reactions-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping an upper-arm shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Spring/spr_npc_juniper_specialanimation_spring_charm_end_south.png";
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
