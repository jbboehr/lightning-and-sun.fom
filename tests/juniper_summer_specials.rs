use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-summer-specials-study and the retained Summer-reading Juniper bundle"]
fn juniper_summer_specials_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-summer-specials-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 3] = [
        ("laugh_start_south", &[66]),
        ("laugh_loop_south", &[70, 67]),
        ("laugh_end_south", &[70, 66]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171];
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
                (34, 45),
                (35, 45),
                (44, 46),
                (46, 46),
            ],
        ),
        (
            "laugh_loop_south",
            0,
            &[(38, 32), (41, 32), (37, 33), (42, 33)],
        ),
        (
            "laugh_loop_south",
            1,
            &[(38, 32), (41, 32), (37, 33), (42, 33)],
        ),
        (
            "laugh_end_south",
            0,
            &[(38, 32), (41, 32), (37, 33), (42, 33)],
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
                (35, 45),
                (44, 46),
                (46, 46),
            ],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("laugh_start_south", 0, 38, 34, 0xE3BF7F, false), // circlet corner
        ("laugh_start_south", 0, 37, 35, 0xBC8B43, false), // circlet side
        ("laugh_start_south", 0, 32, 43, 0xEFD89A, true),  // raised fingers
        ("laugh_start_south", 0, 34, 43, 0x763F21, true),  // hand shadow adjoining coupled cuff
        ("laugh_start_south", 0, 36, 43, 0xEFD89A, true),  // upper arm adjoining coupled cuff
        ("laugh_start_south", 0, 34, 45, 0xBC8B43, false), // separable raised cuff bottom
        ("laugh_start_south", 0, 35, 45, 0xBC8B43, false), // separable raised cuff inner bottom
        ("laugh_start_south", 0, 35, 44, 0xFFF45D, false), // gold cuff center
        ("laugh_start_south", 0, 39, 43, 0xE3BF7F, true),  // neckline
        ("laugh_start_south", 0, 38, 43, 0xD6CDF4, false), // blouse beside neckline
        ("laugh_start_south", 0, 44, 46, 0xBC8B43, false), // far cuff outer corner
        ("laugh_start_south", 0, 46, 46, 0xBC8B43, false), // far cuff inner corner
        ("laugh_start_south", 0, 46, 48, 0x763F21, true),  // far finger shadow
        ("laugh_start_south", 0, 39, 46, 0xEFD89A, true),  // midriff
        ("laugh_loop_south", 0, 38, 32, 0xE3BF7F, false),  // shifted circlet
        ("laugh_loop_south", 0, 37, 41, 0xEFD89A, true),   // lifted wrist
        ("laugh_loop_south", 0, 37, 42, 0x6C6287, false),  // Summer blouse where Spring had cuff
        ("laugh_loop_south", 0, 34, 42, 0xEFD89A, true),   // bent bare forearm
        ("laugh_loop_south", 0, 44, 42, 0x763F21, true),   // far bent upper arm
        ("laugh_loop_south", 0, 46, 43, 0xBC8B43, true),   // bare elbow, no Summer cuff
        ("laugh_loop_south", 0, 45, 44, 0xEFD89A, true),   // forearm down from elbow
        ("laugh_loop_south", 0, 45, 46, 0xBC8B43, true),   // hand touching waist
        ("laugh_loop_south", 1, 37, 40, 0xEFD89A, true),   // bouncing wrist
        ("laugh_loop_south", 1, 46, 42, 0xBC8B43, true),   // bouncing elbow
        ("laugh_loop_south", 1, 42, 48, 0xBC8B43, true),   // shin through Summer skirt
        ("laugh_end_south", 0, 46, 43, 0xBC8B43, true),    // elbow as laugh settles
        ("laugh_end_south", 1, 35, 45, 0xBC8B43, false),   // returning cuff bottom
        ("laugh_end_south", 1, 45, 47, 0xEFD89A, true),    // returning far hand
        // Known art exceptions: these edges beside the gold cuff share a
        // component with hand/upper-arm pixels immediately above them.
        ("laugh_start_south", 0, 34, 44, 0x763F21, true),
        ("laugh_start_south", 0, 36, 44, 0xE3BF7F, true),
        ("laugh_end_south", 1, 34, 44, 0x763F21, true),
        ("laugh_end_south", 1, 36, 44, 0xE3BF7F, true),
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
            let prefix = "specialanimation_summer";
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
        assert_eq!(changed, 339);

        for r in &profile["regions"].as_array().unwrap()[..198] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-summer-reading-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping a finger shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Summer/spr_npc_juniper_specialanimation_summer_laugh_start_south.png";
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
        .retain(|s| *s != json!([46, 48]));
    let omitted = apply_control(
        "missing-finger-shadow",
        missing,
        map.clone(),
        profile["color_groups"].clone(),
    );
    assert_eq!(omitted.get_pixel(46, 48).0, rgba(0x763F21));
    assert_ne!(omitted.get_pixel(46, 48), correct.get_pixel(46, 48));
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(46, 46).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(46, 46).0, rgba(0x6687AD));
}
