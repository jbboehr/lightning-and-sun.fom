use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-wedding-finish-study and the retained Beach-swimming Juniper bundle"]
fn juniper_beach_finish_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-wedding-finish-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 4] = [
        ("swim_idle_east", &[29, 29, 24, 24]),
        ("swim_idle_north", &[2, 2, 2, 2]),
        ("swim_idle_south", &[32, 32, 26, 26]),
        ("swim_spell_cast_south", &[79, 78, 73, 68]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // The pink water covers the body; casting exposes both hands. Matching shades are skin;
    // hair, eyes, hair tie, water and foam use distinct colors.
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("swim_idle_east", 0, 39, 48, 0xBC8B43, true), // forehead below hair
        ("swim_idle_east", 0, 40, 49, 0xEFD89A, true), // face highlight
        ("swim_idle_east", 0, 36, 51, 0xEFD89A, true), // ear beside water
        ("swim_idle_east", 0, 38, 54, 0x763F21, true), // jaw at waterline
        ("swim_idle_east", 0, 40, 54, 0xE3BF7F, true), // lower face above foam
        ("swim_idle_east", 0, 40, 55, 0xFFD5FB, false), // touching foam
        ("swim_idle_east", 0, 34, 54, 0xEB96CB, false), // dark surrounding water
        ("swim_idle_east", 0, 49, 49, 0xFFD5FB, false), // detached splash
        ("swim_idle_east", 0, 35, 42, 0xDF8D4B, false), // warm hair tie
        ("swim_idle_east", 0, 36, 42, 0xFFD565, false), // gold hair tie highlight
        ("swim_idle_east", 0, 37, 51, 0x5A3668, false), // side hair
        ("swim_idle_east", 0, 38, 51, 0xC2B9BE, false), // shaded eye white
        ("swim_idle_east", 0, 38, 52, 0xECF0E9, false), // eye white
        ("swim_idle_east", 0, 39, 51, 0x000000, false), // pupil
        ("swim_idle_east", 1, 43, 53, 0xE3BF7F, true), // opposite cheek
        ("swim_idle_east", 1, 40, 55, 0xFFD5FB, false), // animated foam
        ("swim_idle_east", 2, 39, 49, 0xBC8B43, true), // lowered forehead
        ("swim_idle_east", 2, 36, 52, 0xEFD89A, true), // lowered ear
        ("swim_idle_east", 2, 43, 54, 0xE3BF7F, true), // lowest cheek at waterline
        ("swim_idle_east", 2, 48, 53, 0xFFD5FB, false), // raised splash beyond head
        ("swim_idle_east", 2, 43, 59, 0xFFD5FB, false), // detached lower foam
        ("swim_idle_east", 3, 40, 54, 0xEFD89A, true), // bobbing face remains covered
        ("swim_idle_south", 0, 38, 48, 0xBC8B43, true), // isolated forehead shade
        ("swim_idle_south", 0, 35, 51, 0xEFD89A, true), // left ear
        ("swim_idle_south", 0, 44, 51, 0xEFD89A, true), // right ear
        ("swim_idle_south", 0, 37, 54, 0x763F21, true), // left jaw at waterline
        ("swim_idle_south", 0, 42, 54, 0x763F21, true), // right jaw at waterline
        ("swim_idle_south", 0, 39, 54, 0xE3BF7F, true), // chin above foam
        ("swim_idle_south", 0, 39, 55, 0xFFD5FB, false), // adjacent foam
        ("swim_idle_south", 0, 46, 51, 0xEB96CB, false), // water beside head
        ("swim_idle_south", 0, 30, 59, 0xFFD5FB, false), // detached left splash
        ("swim_idle_south", 1, 42, 53, 0xBC8B43, true), // shaded cheek in second pose
        ("swim_idle_south", 2, 38, 49, 0xBC8B43, true), // lowered forehead
        ("swim_idle_south", 2, 44, 52, 0xEFD89A, true), // lowered opposite ear
        ("swim_idle_south", 2, 39, 54, 0xEFD89A, true), // cheek against higher waterline
        ("swim_idle_south", 2, 39, 55, 0xFFD5FB, false), // foam touching cheek
        ("swim_idle_south", 2, 33, 59, 0xFFD5FB, false), // curved detached wake
        ("swim_idle_south", 3, 42, 54, 0xBC8B43, true), // final shaded cheek
        ("swim_idle_north", 0, 35, 51, 0xEFD89A, true), // only exposed ear, upper pixel
        ("swim_idle_north", 0, 35, 52, 0xEFD89A, true), // same ear lower pixel
        ("swim_idle_north", 1, 35, 51, 0xEFD89A, true),
        ("swim_idle_north", 1, 35, 52, 0xEFD89A, true),
        ("swim_idle_north", 2, 35, 52, 0xEFD89A, true), // lowered ear
        ("swim_idle_north", 2, 35, 53, 0xEFD89A, true),
        ("swim_idle_north", 3, 35, 52, 0xEFD89A, true),
        ("swim_idle_north", 3, 35, 53, 0xEFD89A, true),
        ("swim_spell_cast_south", 0, 38, 45, 0x763F21, true), // exposed forehead above hairline
        ("swim_spell_cast_south", 0, 39, 45, 0x763F21, true), // adjacent forehead patch
        ("swim_spell_cast_south", 0, 37, 46, 0x763F21, true), // separate forehead corner
        ("swim_spell_cast_south", 0, 32, 52, 0xEFD89A, true), // raised left finger
        ("swim_spell_cast_south", 0, 47, 52, 0xEFD89A, true), // raised right finger
        ("swim_spell_cast_south", 0, 31, 55, 0xBC8B43, true), // left hand edge against pink foam
        ("swim_spell_cast_south", 0, 34, 55, 0x763F21, true), // deeper hand shadow
        ("swim_spell_cast_south", 0, 38, 54, 0xBC8B43, true), // chin at waterline
        ("swim_spell_cast_south", 0, 37, 49, 0xB789D5, false), // eyelid cosmetics
        ("swim_spell_cast_south", 0, 35, 55, 0xFFD5FB, false), // foam beside hand
        ("swim_spell_cast_south", 0, 43, 44, 0xB789D5, false), // flying hair highlight
        ("swim_spell_cast_south", 1, 36, 47, 0x763F21, true), // newly exposed forehead corner
        ("swim_spell_cast_south", 1, 36, 48, 0xBC8B43, true), // lower forehead edge
        ("swim_spell_cast_south", 1, 42, 52, 0x715E8E, false), // moving hair across cheek
        ("swim_spell_cast_south", 1, 44, 52, 0xEFD89A, true), // opposite ear
        ("swim_spell_cast_south", 1, 32, 54, 0xEFD89A, true), // left palm
        ("swim_spell_cast_south", 2, 39, 46, 0xE3BF7F, true), // forehead under returning hair
        ("swim_spell_cast_south", 2, 37, 54, 0x763F21, true), // revealed jaw shadow
        ("swim_spell_cast_south", 2, 42, 53, 0x715E8E, false), // hair covers opposite cheek
        ("swim_spell_cast_south", 2, 35, 42, 0xDF8D4B, false), // revealed hair tie
        ("swim_spell_cast_south", 2, 33, 59, 0xFFD5FB, false), // detached pink wake
        ("swim_spell_cast_south", 3, 38, 51, 0xEFD89A, true), // cheek during final pose
        ("swim_spell_cast_south", 3, 36, 52, 0x9E77B3, false), // pale strand beside ear
        ("swim_spell_cast_south", 3, 34, 44, 0xFFD565, false), // hair tie highlight
        ("swim_spell_cast_south", 3, 42, 54, 0x5A3668, false), // hair covers jaw corner
        ("swim_spell_cast_south", 3, 47, 54, 0xEFD89A, true), // right palm
        ("swim_spell_cast_south", 3, 49, 55, 0x763F21, true), // right hand outline above foam
        ("swim_spell_cast_south", 3, 38, 57, 0xEB96CB, false), // deeper pink water
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
            let prefix = "specialanimation_beach";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Beach/spr_npc_juniper_{prefix}_{name}.png"
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
                    Some(i) => {
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
        assert_eq!(changed, 528);

        for r in &profile["regions"].as_array().unwrap()[..286] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-beach-swim-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: missing forehead/hand/jaw components and an accidental foam palette entry.
    let asset = "assets/animations/NPCs/Juniper/Sprites/Beach/spr_npc_juniper_specialanimation_beach_swim_spell_cast_south.png";
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
        ("missing-forehead", [38, 45], 0x763F21),
        ("missing-hand", [32, 52], 0xEFD89A),
        ("missing-jaw", [42, 54], 0x763F21),
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
    let mut too_broad = map;
    too_broad.insert("#FFD5FB".into(), json!("#FFFFFF"));
    let mut broad_groups = profile["color_groups"].clone();
    broad_groups
        .as_array_mut()
        .unwrap()
        .push(json!(["#FFD5FB"]));
    let spilled = apply_control("water-spill", Value::Null, too_broad, broad_groups);
    for point in [[39, 55], [30, 59]] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(0xFFD5FB));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(0xFFFFFF));
    }
}
