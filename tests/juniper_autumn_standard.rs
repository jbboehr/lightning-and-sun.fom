use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/juniper-beach-actions-study and the retained Autumn-actions Juniper bundle"]
fn juniper_autumn_standard_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-beach-actions-study");
    let profile_path = root.join("palettes/profiles/juniper-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/juniper-world-trial.json"));
    let cases: [(&str, &[usize]); 5] = [
        ("action_north", &[5, 5, 5, 5, 5, 6, 12]),
        ("action_south", &[40, 39, 38, 39, 38, 40, 44]),
        ("action_east", &[31, 32, 29, 32, 29, 31, 37]),
        ("sleep_east", &[32]),
        ("kiss_east", &[34, 34, 37, 38]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21, 0xE8B171, 0xF1E791];
    // Literal source-material exclusions: circlet and bracers.
    type MaterialCase<'a> = (&'a str, u32, &'a [(u32, u32)]);
    let clothing: &[MaterialCase<'_>] = &[
        ("action_north", 0, &[]),
        ("action_north", 1, &[(34, 44)]),
        ("action_north", 2, &[(34, 44)]),
        ("action_north", 3, &[(34, 44)]),
        ("action_north", 4, &[(34, 44)]),
        ("action_north", 5, &[(33, 45)]),
        ("action_north", 6, &[(33, 45), (46, 45)]),
        (
            "action_south",
            0,
            &[
                (38, 34),
                (41, 34),
                (37, 35),
                (42, 35),
                (33, 46),
                (45, 46),
                (46, 46),
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
                (33, 46),
                (45, 46),
                (46, 46),
            ],
        ),
        (
            "action_south",
            6,
            &[(38, 33), (41, 33), (37, 34), (42, 34), (33, 45), (46, 45)],
        ),
        ("action_east", 0, &[(40, 34), (43, 34), (39, 35), (44, 35)]),
        (
            "action_east",
            1,
            &[(42, 33), (45, 33), (41, 34), (46, 34), (46, 44)],
        ),
        (
            "action_east",
            2,
            &[(42, 33), (45, 33), (41, 34), (46, 34), (45, 44)],
        ),
        (
            "action_east",
            3,
            &[(42, 33), (45, 33), (41, 34), (46, 34), (46, 44)],
        ),
        (
            "action_east",
            4,
            &[(42, 33), (45, 33), (41, 34), (46, 34), (45, 44)],
        ),
        ("action_east", 5, &[(40, 34), (43, 34), (39, 35), (44, 35)]),
        (
            "action_east",
            6,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (34, 45), (44, 45)],
        ),
        (
            "sleep_east",
            0,
            &[(39, 33), (42, 33), (38, 34), (43, 34), (44, 41)],
        ),
        (
            "kiss_east",
            0,
            &[(38, 34), (41, 34), (37, 35), (42, 35), (34, 46)],
        ),
        (
            "kiss_east",
            1,
            &[(40, 34), (43, 34), (39, 35), (44, 35), (35, 46)],
        ),
        (
            "kiss_east",
            2,
            &[(42, 33), (45, 33), (41, 34), (46, 34), (35, 44)],
        ),
        (
            "kiss_east",
            3,
            &[(40, 34), (43, 34), (39, 35), (44, 35), (35, 46)],
        ),
    ];
    // Zero-based frame numbers and frame-local coordinates.
    let landmarks = [
        ("action_north", 0, 33, 43, 0xF8F960, false), // exposed cuff gold above hand
        ("action_north", 0, 32, 44, 0xEFD89A, true),  // fingers below cuff
        ("action_north", 0, 33, 45, 0x763F21, true),  // finger shadow
        ("action_north", 1, 34, 44, 0xBC8B43, false), // shaded top of moving cuff
        ("action_north", 1, 34, 45, 0xEFD89A, true),  // hand below shaded cuff
        ("action_north", 2, 35, 46, 0x763F21, true),  // finger shadow through action loop
        ("action_north", 3, 34, 44, 0xBC8B43, false), // returning shaded cuff
        ("action_north", 4, 34, 44, 0xBC8B43, false), // repeated cuff boundary
        ("action_north", 5, 33, 45, 0xBC8B43, false), // border alongside cuff gold
        ("action_north", 5, 33, 47, 0xBC8B43, true),  // same shade on fingers two rows lower
        ("action_north", 6, 46, 45, 0xBC8B43, false), // opposite shaded cuff
        ("action_north", 6, 40, 42, 0x9E77B3, false), // hair over back
        ("action_south", 0, 38, 34, 0xE3BF7F, false), // circlet corner
        ("action_south", 0, 40, 34, 0xCA3561, false), // circlet gemstone
        ("action_south", 0, 39, 43, 0xBC8B43, true),  // exposed blouse keyhole
        ("action_south", 0, 39, 46, 0xF1E791, true),  // established midriff highlight
        ("action_south", 0, 33, 46, 0xBC8B43, false), // near cuff border
        ("action_south", 0, 45, 46, 0xE3BF7F, false), // shaded far cuff reuses skin midtone
        ("action_south", 0, 46, 46, 0xBC8B43, false), // far cuff edge
        ("action_south", 0, 45, 47, 0xBC8B43, true),  // fingers below far cuff
        ("action_south", 0, 34, 48, 0x763F21, true),  // near fingertip shadow
        ("action_south", 1, 34, 45, 0xBC8B43, false), // moving near cuff
        ("action_south", 1, 44, 45, 0xE3BF7F, false), // moving far cuff
        ("action_south", 1, 39, 45, 0xF1E791, true),  // midriff during upward bob
        ("action_south", 2, 37, 45, 0xFFF45D, false), // bent cuff gold
        ("action_south", 2, 36, 46, 0xBC8B43, true),  // bent hand edge below cuff
        ("action_south", 2, 38, 47, 0x763F21, true),  // bent fingertip
        ("action_south", 3, 36, 44, 0xCA3561, false), // cuff gemstone
        ("action_south", 4, 36, 45, 0x4F4873, false), // sleeve beside gold cuff
        ("action_south", 5, 39, 47, 0xCA3561, false), // skirt below exposed midriff
        ("action_south", 6, 46, 45, 0xBC8B43, false), // restored opposite cuff edge
        ("action_east", 0, 40, 34, 0xE3BF7F, false),  // shifted circlet
        ("action_east", 0, 40, 46, 0xFBCC5A, false),  // cuff shading
        ("action_east", 0, 41, 47, 0xEFD89A, true),   // fingers below cuff
        ("action_east", 1, 46, 44, 0xBC8B43, false),  // extended cuff corner beneath gemstone
        ("action_east", 1, 48, 43, 0xEFD89A, true),   // extended hand beside cuff
        ("action_east", 1, 48, 45, 0x763F21, true),   // extended fingertip shadow
        ("action_east", 2, 45, 44, 0xBC8B43, false),  // lowered cuff corner
        ("action_east", 2, 44, 46, 0xBC8B43, true),   // same shade at lower hand edge
        ("action_east", 3, 49, 45, 0xBC8B43, true),   // returning extended fingertip edge
        ("action_east", 4, 44, 44, 0xFFF45D, false),  // lowered cuff gold
        ("action_east", 5, 40, 48, 0x763F21, true),   // lowered fingertip
        ("action_east", 6, 44, 45, 0xBC8B43, false),  // shaded far cuff
        ("action_east", 6, 44, 46, 0xE3BF7F, true),   // far hand below cuff
        ("sleep_east", 0, 38, 36, 0x8D80C7, false),   // closed eyelid cosmetics
        ("sleep_east", 0, 43, 39, 0xE3BF7F, true),    // raised hand by face
        ("sleep_east", 0, 44, 39, 0x763F21, true),    // sleeping finger crease
        ("sleep_east", 0, 43, 41, 0xFFF45D, false),   // cuff below sleeping hand
        ("sleep_east", 0, 44, 41, 0xBC8B43, false),   // separable sleeping cuff corner
        ("sleep_east", 0, 40, 45, 0xEFD89A, true),    // exposed midriff
        ("kiss_east", 0, 34, 46, 0xBC8B43, false),    // lowered cuff edge
        ("kiss_east", 0, 43, 47, 0xE3BF7F, true),     // small opposite hand
        ("kiss_east", 1, 39, 36, 0x8D80C7, false),    // cosmetics over closing eye
        ("kiss_east", 1, 43, 40, 0xE3BF7F, true),     // lower face
        ("kiss_east", 2, 46, 37, 0xEFD89A, true),     // extended closed-mouth silhouette
        ("kiss_east", 2, 35, 44, 0xBC8B43, false),    // moving cuff edge
        ("kiss_east", 3, 44, 38, 0xEFD89A, true),     // final closed-mouth silhouette
        ("kiss_east", 3, 36, 48, 0x763F21, true),     // final finger shadow
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
        let targets = [0, 1, 12, 4, 13, 14].map(|i| {
            rgba(u32::from_str_radix(&preset["colors"][i].as_str().unwrap()[1..7], 16).unwrap())
        });
        let mut changed = 0;

        for (name, counts) in cases {
            let prefix = "autumn";
            let asset = format!(
                "assets/animations/NPCs/Juniper/Sprites/Autumn/spr_npc_juniper_{prefix}_{name}.png"
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
        assert_eq!(changed, 717);

        for r in &profile["regions"].as_array().unwrap()[..223] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-reina-juniper-march-autumn-actions-trial/characters/juniper/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: dropping a finger shadow component, or merging groups
    // and allowing skin selection to cross into a same-shade gold bracer.
    let asset =
        "assets/animations/NPCs/Juniper/Sprites/Autumn/spr_npc_juniper_autumn_action_south.png";
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
    let mut missing_alias = region.clone();
    missing_alias["seeds"]
        .as_array_mut()
        .unwrap()
        .retain(|s| *s != json!([39, 46]));
    let omitted_alias = apply_control(
        "missing-midriff-highlight",
        missing_alias,
        map.clone(),
        profile["color_groups"].clone(),
    );
    assert_eq!(omitted_alias.get_pixel(39, 46).0, rgba(0xF1E791));
    assert_ne!(omitted_alias.get_pixel(39, 46), correct.get_pixel(39, 46));
    let spilled = apply_control(
        "merged-groups",
        region,
        map,
        json!([profile["source_colors"]]),
    );
    assert_eq!(correct.get_pixel(33, 46).0, rgba(0xBC8B43));
    assert_eq!(spilled.get_pixel(33, 46).0, rgba(0x6687AD));
}
