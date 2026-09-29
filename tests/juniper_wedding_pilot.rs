use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-wedding-pilot-study and the retained Beach-completion Juniper bundle"]
fn juniper_wedding_pilot_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-wedding-pilot-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 6] = [
        ("idle_north", &[2]),
        ("idle_south", &[59]),
        ("idle_east", &[57]),
        ("walk_north", &[2, 3, 2, 3]),
        ("walk_south", &[59, 62, 59, 63]),
        ("walk_east", &[57, 66, 57, 56]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [
        0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791, 0xB58E45, 0xE0B572,
    ];
    // Literal protected gold hairpin, arm-band and anklet borders.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        (
            "idle_south",
            0,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (34, 44),
                (36, 44),
                (43, 44),
                (45, 44),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "idle_east",
            0,
            &[(37, 30), (34, 32), (35, 32), (37, 44), (41, 52), (42, 52)],
        ),
        (
            "walk_south",
            0,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (34, 44),
                (36, 44),
                (43, 44),
                (45, 44),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "walk_south",
            1,
            &[
                (35, 31),
                (44, 31),
                (33, 33),
                (46, 33),
                (34, 45),
                (41, 53),
                (42, 53),
            ],
        ),
        (
            "walk_south",
            2,
            &[
                (35, 30),
                (44, 30),
                (33, 32),
                (46, 32),
                (34, 44),
                (36, 44),
                (43, 44),
                (45, 44),
                (37, 52),
                (38, 52),
                (41, 52),
                (42, 52),
            ],
        ),
        (
            "walk_south",
            3,
            &[
                (35, 31),
                (44, 31),
                (33, 33),
                (46, 33),
                (45, 45),
                (37, 53),
                (38, 53),
            ],
        ),
        (
            "walk_east",
            0,
            &[(37, 30), (34, 32), (35, 32), (37, 44), (41, 52), (42, 52)],
        ),
        ("walk_east", 1, &[(37, 31), (34, 33), (35, 33), (33, 45)]),
        (
            "walk_east",
            2,
            &[(37, 30), (34, 32), (35, 32), (37, 44), (41, 52), (42, 52)],
        ),
        ("walk_east", 3, &[(37, 31), (34, 33), (35, 33), (35, 46)]),
    ];
    // Zero-based frames and frame-local coordinates, independently reviewed against source art.
    let landmarks = [
        ("idle_north", 0, 32, 47, 0xEFD89A, true), // tiny exposed left hand
        ("idle_north", 0, 47, 47, 0xEFD89A, true), // opposite hand
        ("idle_north", 0, 36, 33, 0xF7C155, false), // back hair band
        ("idle_north", 0, 39, 34, 0xFFFFFF, false), // gold band's light reflection
        ("idle_north", 0, 34, 28, 0xFF6C89, false), // hair ornament
        ("idle_south", 0, 35, 30, 0xBC8B43, false), // hairpin border shares skin shade
        ("idle_south", 0, 34, 31, 0xFFF45D, false), // gold hairpin center
        ("idle_south", 0, 38, 33, 0xBC8B43, true), // forehead below hair
        ("idle_south", 0, 39, 34, 0xEFD89A, true), // face highlight
        ("idle_south", 0, 35, 36, 0xEFD89A, true), // ear
        ("idle_south", 0, 37, 36, 0xC2B9BE, false), // eye white
        ("idle_south", 0, 37, 39, 0x763F21, true), // jaw shadow
        ("idle_south", 0, 39, 40, 0xBC8B43, true), // chin
        ("idle_south", 0, 39, 41, 0xFFF45D, false), // neckline clasp
        ("idle_south", 0, 39, 42, 0xBC8B43, true), // exposed keyhole edge
        ("idle_south", 0, 40, 42, 0xE3BF7F, true), // keyhole skin highlight
        ("idle_south", 0, 39, 44, 0x763F21, true), // deep keyhole shadow
        ("idle_south", 0, 35, 43, 0xBC8B43, true), // upper arm above band
        ("idle_south", 0, 34, 44, 0x763F21, false), // dark gold-band edge
        ("idle_south", 0, 35, 44, 0xFFF45D, false), // band center
        ("idle_south", 0, 33, 45, 0xBC8B43, true), // wrist below band
        ("idle_south", 0, 32, 46, 0xEFD89A, true), // hand
        ("idle_south", 0, 35, 46, 0xDF4675, false), // hanging sleeve
        ("idle_south", 0, 37, 49, 0xE3BF7F, true), // exposed leg through dress slit
        ("idle_south", 0, 37, 51, 0xFFF45D, false), // anklet highlight
        ("idle_south", 0, 37, 52, 0xBC8B43, false), // separable anklet lower border
        ("idle_south", 0, 37, 53, 0xEFD89A, true), // toes below band
        ("idle_east", 0, 37, 30, 0xBC8B43, false), // side hairpin border
        ("idle_east", 0, 39, 33, 0xBC8B43, true),  // side forehead
        ("idle_east", 0, 36, 36, 0xEFD89A, true),  // side ear
        ("idle_east", 0, 38, 39, 0x763F21, true),  // side jaw
        ("idle_east", 0, 41, 42, 0xE3BF7F, true),  // keyhole viewed from side
        ("idle_east", 0, 38, 41, 0xBC8B43, true),  // upper shoulder
        ("idle_east", 0, 37, 44, 0x763F21, false), // arm-band edge
        ("idle_east", 0, 35, 44, 0xB65932, false), // distinct gold shadow
        ("idle_east", 0, 36, 45, 0xDF8D4B, false), // distinct gold lower edge
        ("idle_east", 0, 35, 46, 0xEFD89A, true),  // palm
        ("idle_east", 0, 38, 53, 0xBC8B43, true),  // foot shadow sharing anklet component
        ("idle_east", 0, 41, 52, 0xBC8B43, false), // opposite anklet border stays original
        ("walk_north", 1, 33, 48, 0xBC8B43, true), // moving hand emerges from hair
        ("walk_north", 1, 41, 54, 0xEFD89A, true), // bare foot emerging below hem
        ("walk_north", 1, 42, 54, 0xE3BF7F, true), // foot shade
        ("walk_north", 3, 46, 48, 0xBC8B43, true), // opposite moving hand
        ("walk_north", 3, 37, 54, 0xE3BF7F, true), // opposite bare foot
        ("walk_south", 1, 34, 45, 0x763F21, false), // forward arm-band edge
        ("walk_south", 1, 44, 45, 0xE3BF7F, true), // bare opposite arm
        ("walk_south", 1, 45, 46, 0xBC8B43, true), // shaded opposite hand
        ("walk_south", 1, 41, 53, 0xBC8B43, false), // shifted anklet border
        ("walk_south", 1, 41, 54, 0xEFD89A, true), // shifted toes
        ("walk_south", 3, 45, 45, 0x763F21, false), // opposite moving arm-band edge
        ("walk_south", 3, 35, 45, 0xE3BF7F, true), // upper arm in shade
        ("walk_south", 3, 37, 53, 0xBC8B43, false), // opposite shifted anklet
        ("walk_south", 3, 37, 54, 0xEFD89A, true), // opposite shifted toes
        ("walk_east", 1, 33, 45, 0xBC8B43, false), // bracelet corner beside distinct gold
        ("walk_east", 1, 32, 46, 0xEFD89A, true),  // leading fingertips
        ("walk_east", 1, 47, 46, 0xEFD89A, true),  // trailing fingertips
        ("walk_east", 1, 40, 52, 0xFFF45D, false), // angled anklet highlight
        ("walk_east", 1, 43, 53, 0xEFD89A, true),  // angled toes
        ("walk_east", 3, 35, 46, 0xBC8B43, false), // moving bracelet lower corner
        ("walk_east", 3, 35, 47, 0xEFD89A, true),  // hand directly below it
        ("walk_east", 3, 36, 51, 0xFFF45D, false), // leading ankle jewelry
        ("walk_east", 3, 43, 52, 0xFFF45D, false), // trailing ankle jewelry
        ("walk_east", 3, 45, 52, 0xEFD89A, true),  // far trailing toe
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
            let prefix = "wedding";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Wedding/spr_npc_juniper_{prefix}_{name}.png"
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
                        if !clothing.iter().any(|(case, frame, points)| {
                            *case == name && *frame == x / 80 && points.contains(&(x % 80, y))
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
            // Six accepted anklet-edge occurrences share a component with bare foot shadow.
            if name == "idle_east" || name == "walk_east" {
                for frame in if name == "idle_east" {
                    vec![0]
                } else {
                    vec![0, 2]
                } {
                    for x in [38, 39] {
                        assert_eq!(before.get_pixel(frame * 80 + x, 52).0, rgba(0xBC8B43));
                        assert_eq!(after.get_pixel(frame * 80 + x, 52).0, targets[2]);
                    }
                    assert_eq!(after.get_pixel(frame * 80 + 38, 53).0, targets[2]);
                }
            }
            changed += actual.iter().sum::<usize>();
            for &(case, frame, x, y, color, changes) in &landmarks {
                if case == name {
                    let x = frame * 80 + x;
                    assert_eq!(before.get_pixel(x, y).0, rgba(color));
                    assert_eq!(before.get_pixel(x, y) != after.get_pixel(x, y), changes);
                }
            }
        }
        assert_eq!(changed, 607);

        for r in &profile["regions"].as_array().unwrap()[..290] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-beach-finish-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: omitted forehead, keyhole, hand and toes; unrestricted material spill.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Wedding/spr_npc_juniper_wedding_idle_south.png";
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
        ("missing-forehead", [38, 33], 0xBC8B43),
        ("missing-keyhole", [39, 42], 0xBC8B43),
        ("missing-hand", [32, 46], 0xEFD89A),
        ("missing-toes", [37, 53], 0xEFD89A),
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
    let spilled = apply_control(
        "unrestricted-spill",
        Value::Null,
        map,
        profile["color_groups"].clone(),
    );
    for (point, color) in [
        ([35, 30], 0xBC8B43),
        ([34, 44], 0x763F21),
        ([37, 52], 0xBC8B43),
    ] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(color));
        assert_ne!(
            spilled.get_pixel(point[0], point[1]),
            correct.get_pixel(point[0], point[1])
        );
    }
}
