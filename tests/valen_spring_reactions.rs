use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/valen-summer-actions-study and the retained Spring-standard Valen bundle"]
fn valen_spring_reactions_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-summer-actions-study");
    let profile_path = root.join("palettes/profiles/valen-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/valen-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("shocked_start_south", &[67]),
        ("shocked_loop_south", &[78]),
        ("shocked_end_south", &[67]),
        ("read_sit_start_south", &[48, 43, 42]),
        ("read_sit_loop_south", &[36, 49, 36, 49]),
        ("read_sit_end_south", &[51, 34, 48]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xFBD3A7, 0xEFA67A, 0xC37555, 0x762E21];
    // Frame-local coordinates are zero-based. Book paper and pink covers have their own
    // shades; small fingers and jaw corners beside them still use the existing skin roles.
    let landmarks = [
        ("shocked_loop_south", 0, 39, 30, 0xEFA67A, true), // exposed forehead
        ("shocked_loop_south", 0, 41, 30, 0xC37555, true), // forehead corner
        ("shocked_loop_south", 0, 31, 33, 0xFBD3A7, true), // raised near fingers
        ("shocked_loop_south", 0, 32, 33, 0xC37555, true), // isolated finger shade
        ("shocked_loop_south", 0, 30, 34, 0xEFA67A, true), // outside fingertip
        ("shocked_loop_south", 0, 46, 33, 0xFBD3A7, true), // opposite raised hand
        ("shocked_loop_south", 0, 47, 33, 0xC37555, true), // opposite finger shade
        ("shocked_loop_south", 0, 49, 34, 0xEFA67A, true), // opposite outer fingertip
        ("shocked_loop_south", 0, 37, 33, 0xC2B9BE, false), // eye white shade
        ("shocked_loop_south", 0, 37, 34, 0xECF0E9, false), // eye white
        ("shocked_loop_south", 0, 38, 34, 0x000000, false), // pupil
        ("shocked_loop_south", 0, 39, 36, 0x410808, false), // open mouth outline
        ("shocked_loop_south", 0, 39, 37, 0x9E2626, false), // mouth interior
        ("shocked_loop_south", 0, 38, 37, 0xC37555, true), // cheek at mouth edge
        ("shocked_loop_south", 0, 37, 37, 0x762E21, true), // deep jaw outline
        ("shocked_loop_south", 0, 33, 38, 0xFBD3A7, true), // wrist above cuff
        ("shocked_loop_south", 0, 34, 38, 0x9F86A6, false), // cuff beside wrist
        ("shocked_loop_south", 0, 34, 39, 0xDCDBED, false), // light cuff
        ("shocked_loop_south", 0, 33, 49, 0xC37555, true), // spread ankle shade
        ("shocked_loop_south", 0, 34, 49, 0xFBD3A7, true), // ankle highlight
        ("shocked_loop_south", 0, 34, 50, 0xEFA67A, true), // ankle at shoe edge
        ("shocked_loop_south", 0, 32, 50, 0xFCF5F1, false), // shoe
        ("shocked_loop_south", 0, 45, 49, 0xFBD3A7, true), // opposite ankle
        ("shocked_loop_south", 0, 46, 49, 0xC37555, true), // opposite ankle shade
        ("read_sit_start_south", 1, 38, 39, 0xEFA67A, true), // cheek just above book
        ("read_sit_start_south", 1, 37, 40, 0xDFB1BD, false), // cover tip beside cheek
        ("read_sit_start_south", 1, 37, 41, 0xC98597, false), // pink book cover
        ("read_sit_start_south", 1, 38, 41, 0xC9AF9C, false), // warm shaded page edge
        ("read_sit_start_south", 1, 39, 41, 0xF6E4D7, false), // cream page
        ("read_sit_start_south", 1, 34, 46, 0xFBD3A7, true), // hand outside closed book
        ("read_sit_start_south", 1, 35, 47, 0x762E21, true), // hand outline
        ("read_sit_start_south", 1, 44, 46, 0xFBD3A7, true), // other hand
        ("read_sit_start_south", 1, 44, 47, 0x762E21, true), // other hand outline
        ("read_sit_start_south", 1, 38, 47, 0xA16775, false), // deep cover shade
        ("read_sit_start_south", 1, 38, 49, 0xDFB1BD, false), // cover bottom highlight
        ("read_sit_start_south", 2, 37, 40, 0x762E21, true), // jaw exposed beside opening book
        ("read_sit_start_south", 2, 39, 41, 0xC37555, true), // chin over pages
        ("read_sit_start_south", 2, 35, 40, 0xF6E4D7, false), // page beside jaw
        ("read_sit_start_south", 2, 35, 42, 0x76404E, false), // darkest book hinge
        ("read_sit_start_south", 2, 39, 43, 0xF6E4D7, false), // open paper
        ("read_sit_start_south", 2, 34, 46, 0xFBD3A7, true), // near hand around open book
        ("read_sit_start_south", 2, 45, 46, 0xFBD3A7, true), // far hand around open book
        ("read_sit_start_south", 2, 37, 46, 0x76404E, false), // binding next to hand
        ("read_sit_start_south", 2, 39, 46, 0xC98597, false), // cover spine
        ("read_sit_loop_south", 0, 40, 40, 0xC37555, true), // chin above book
        ("read_sit_loop_south", 0, 39, 41, 0xEFA67A, true), // neck
        ("read_sit_loop_south", 0, 35, 41, 0xFCF5F1, false), // coat above pages
        ("read_sit_loop_south", 0, 32, 42, 0xF6E4D7, false), // lifted page
        ("read_sit_loop_south", 0, 34, 42, 0xC9AF9C, false), // shaded lifted page
        ("read_sit_loop_south", 0, 33, 43, 0x76404E, false), // page hinge
        ("read_sit_loop_south", 0, 30, 44, 0xDFB1BD, false), // left cover edge
        ("read_sit_loop_south", 0, 33, 45, 0xC98597, false), // pink cover midtone
        ("read_sit_loop_south", 0, 34, 44, 0xA16775, false), // cover shade
        ("read_sit_loop_south", 0, 45, 45, 0x76404E, false), // right binding
        ("read_sit_loop_south", 0, 38, 43, 0xDCDBED, false), // cuff above spine
        ("read_sit_end_south", 1, 39, 39, 0xFBD3A7, true), // face while book closes
        ("read_sit_end_south", 1, 37, 40, 0xDFB1BD, false), // closing cover tip
        ("read_sit_end_south", 1, 39, 41, 0xF6E4D7, false), // closing pages
        ("read_sit_end_south", 1, 38, 41, 0xC9AF9C, false), // warm page edge
        ("read_sit_end_south", 1, 34, 46, 0xFBD3A7, true), // returning hand
        ("read_sit_end_south", 1, 35, 47, 0x762E21, true), // returning hand outline
        ("read_sit_end_south", 1, 35, 45, 0x9F86A6, false), // cuff above hand
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
        let targets = [8, 9, 10, 4].map(|i| {
            rgba(u32::from_str_radix(&preset["colors"][i].as_str().unwrap()[1..7], 16).unwrap())
        });
        let mut changed = 0;

        for (name, counts) in cases {
            let prefix = if name.starts_with("read_sit") {
                "specialanimation_spring"
            } else {
                "spring"
            };
            let asset = format!(
                "assets/animations/NPCs/Valen/Sprites/Spring/spr_npc_valen_{prefix}_{name}.png"
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
        assert_eq!(changed, 648);

        for r in &profile["regions"].as_array().unwrap()[..114] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-balor-valen-eiland-spring-standard-trial/characters/valen/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: omit small face/hand components and deliberately pollute the book-page palette.
    let asset = "assets/animations/NPCs/Valen/Sprites/Spring/spr_npc_valen_specialanimation_spring_read_sit_start_south.png";
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
        ("missing-forehead", [39, 33], 0xEFA67A),
        ("missing-forehead-corner", [41, 33], 0xC37555),
        ("missing-hand", [114, 46], 0xFBD3A7),
        ("missing-hand-outline", [115, 47], 0x762E21),
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
    too_broad.insert("#F6E4D7".into(), json!("#FF0000"));
    let mut broad_groups = profile["color_groups"].clone();
    broad_groups
        .as_array_mut()
        .unwrap()
        .push(json!(["#F6E4D7"]));
    let spilled = apply_control("book-spill", Value::Null, too_broad, broad_groups);
    for point in [[119, 41], [120, 44]] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(0xF6E4D7));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(0xFF0000));
    }
}
