use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{fs, path::Path, process::Command};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn success(result: std::process::Output) -> Value {
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&result.stdout).unwrap()
}

#[test]
fn hayden_world_extends_the_reviewed_portraits_and_preserves_their_choices() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let old = read(root.join("palettes/profiles/hayden-portraits.json"));
    let world = read(root.join("palettes/profiles/hayden-world-trial.json"));
    assert_eq!(old["regions"].as_array().unwrap().len(), 133);
    assert_eq!(world["regions"].as_array().unwrap().len(), 155);
    assert_eq!(
        &world["regions"].as_array().unwrap()[..133],
        old["regions"].as_array().unwrap()
    );
    assert_eq!(
        &world["source_colors"].as_array().unwrap()[..11],
        old["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &world["color_groups"].as_array().unwrap()[..3],
        old["color_groups"].as_array().unwrap()
    );
    let mut expected = Vec::new();
    for cycle in ["idle", "walk"] {
        for direction in ["east", "north", "south"] {
            expected.push(format!("assets/animations/NPCs/Hayden/Sprites/Spring/spr_npc_hayden_spring_{cycle}_{direction}.png"));
        }
    }
    let actual: Vec<_> = world["regions"].as_array().unwrap()[133..139]
        .iter()
        .map(|r| r["asset"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(actual, expected);
    let old = read(root.join("palettes/sets/hayden-portraits-trial.json"));
    let world = read(root.join("palettes/sets/hayden-world-trial.json"));
    for (a, b) in old["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(world["presets"].as_array().unwrap())
    {
        assert_eq!(a["id"], b["id"]);
        assert_eq!(a["label"], b["label"]);
        assert_eq!(
            &b["colors"].as_array().unwrap()[..11],
            a["colors"].as_array().unwrap()
        );
        assert_eq!(
            &b["colors"].as_array().unwrap()[11..],
            &a["colors"].as_array().unwrap()[..4]
        );
    }
}

#[test]
fn hayden_world_package_keeps_his_control_and_native_geometry() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    for path in [&original, &modified] {
        fs::create_dir(path).unwrap();
    }
    for cycle in ["idle", "walk"] {
        let (frames, timing) = if cycle == "idle" {
            (1, "")
        } else {
            (4, "frame_len=4\nduration=0.15\n")
        };
        for direction in ["north", "south", "east"] {
            let name = format!("spr_npc_hayden_spring_{cycle}_{direction}");
            let meta = format!(
                "[meta_properties]\nid='original'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[2,3]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal='Middle'\nvertical=54.0\n"
            );
            for (path, color) in [
                (&original, [10, 20, 30, 255]),
                (&modified, [40, 50, 60, 255]),
            ] {
                RgbaImage::from_pixel(frames * 2, 3, Rgba(color))
                    .save(path.join(format!("{name}.png")))
                    .unwrap();
                fs::write(path.join(format!("{name}.meta.toml")), &meta).unwrap();
            }
        }
    }
    let output = temp.path().join("package");
    success(
        Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["package-toggle", "--original"])
            .arg(&original)
            .arg("--modified")
            .arg(&modified)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap(),
    );
    let script = fs::read_to_string(output.join("gml/palette_assets.gml")).unwrap();
    let table: Value = serde_json::from_str(
        script
            .split_once("return ")
            .unwrap()
            .1
            .split_once("; }")
            .unwrap()
            .0,
    )
    .unwrap();
    assert_eq!(table.as_array().unwrap().len(), 1);
    assert_eq!(table[0][0], "hayden");
    assert_eq!(table[0][2], "F8");
    assert_eq!(table[0][4].as_array().unwrap().len(), 6);
    for row in table[0][4].as_array().unwrap() {
        let name = row[0].as_str().unwrap();
        let target = row[1].as_str().unwrap();
        let before: toml::Value = toml::from_str(
            &fs::read_to_string(original.join(format!("{name}.meta.toml"))).unwrap(),
        )
        .unwrap();
        let after: toml::Value = toml::from_str(
            &fs::read_to_string(
                output.join(format!("animations/LightningAndSun/{target}.meta.toml")),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(before["asset_properties"], after["asset_properties"]);
        assert_eq!(
            fs::read(modified.join(format!("{name}.png"))).unwrap(),
            fs::read(output.join(format!("animations/LightningAndSun/{target}.png"))).unwrap()
        );
    }
}

#[test]
#[ignore = "requires the 155 local animations in extracted/hayden-standard-study"]
fn hayden_world_masks_cover_skin_without_crossing_into_shirt_shadows() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-standard-study");
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("bundle");
    let set = std::env::var_os("FOM_HAYDEN_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/hayden-world-trial.json"));
    success(
        Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["build-presets", "--original"])
            .arg(&original)
            .arg("--presets")
            .arg(&set)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap(),
    );
    let source = [0xE7B172, 0xAB7E3F, 0x815A2E, 0x523C26];
    let landmarks = [
        ("idle_south", 0, 39, 33, 0xE7B172, true),
        ("idle_south", 0, 32, 44, 0x523C26, true),
        ("idle_south", 0, 36, 42, 0x815A2E, false),
        ("idle_east", 0, 36, 33, 0xE7B172, true),
        ("idle_east", 0, 37, 42, 0x815A2E, false),
        ("idle_north", 0, 32, 42, 0xE7B172, true),
        ("walk_east", 1, 33, 43, 0xE7B172, true),
        ("walk_east", 1, 37, 43, 0x815A2E, false),
        ("walk_east", 1, 36, 42, 0x815A2E, false),
        ("walk_south", 1, 34, 44, 0x523C26, true),
        ("walk_south", 1, 37, 42, 0xAB7E3F, false),
        ("walk_north", 1, 36, 40, 0xAB7E3F, false),
        ("walk_north", 3, 42, 42, 0xAB7E3F, false),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut first_selection = Vec::new();
    for preset in read(&set)["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = preset["colors"].as_array().unwrap()[11..]
            .iter()
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let mut selection = Vec::new();
        let mut total = 0;
        for cycle in ["idle", "walk"] {
            for direction in ["north", "south", "east"] {
                let name = format!("{cycle}_{direction}");
                let asset = format!(
                    "assets/animations/NPCs/Hayden/Sprites/Spring/spr_npc_hayden_spring_{name}.png"
                );
                let before = image::open(original.join(&asset)).unwrap().to_rgba8();
                let modified = output.join("variants").join(id);
                let after = image::open(modified.join(&asset)).unwrap().to_rgba8();
                assert_eq!(before.dimensions(), after.dimensions());
                let meta = asset.replace(".png", ".meta.toml");
                assert_eq!(
                    fs::read(original.join(&meta)).unwrap(),
                    fs::read(modified.join(meta)).unwrap()
                );
                let mut counts = vec![0; before.width() as usize / 80];
                for (x, y, p) in before.enumerate_pixels() {
                    let q = after.get_pixel(x, y);
                    assert_eq!(p[3], q[3]);
                    selection.push(p != q);
                    if p != q {
                        let index = source
                            .iter()
                            .position(|c| rgba(*c) == p.0)
                            .expect("non-skin color changed");
                        assert_eq!(q.0, rgba(target[index]));
                        counts[x as usize / 80] += 1;
                        total += 1;
                    }
                }
                assert!(counts.iter().all(|n| *n > 0));
                for &(case, frame, x, y, color, skin) in &landmarks {
                    if case != name {
                        continue;
                    }
                    let x = x + frame * 80;
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        rgba(color),
                        "source landmark {case} [{x},{y}]"
                    );
                    let expected = if skin {
                        target[source.iter().position(|c| *c == color).unwrap()]
                    } else {
                        color
                    };
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        rgba(expected),
                        "material boundary {id} {case} [{x},{y}] skin={skin}"
                    );
                }
            }
        }
        if first_selection.is_empty() {
            first_selection = selection;
        } else {
            assert_eq!(first_selection, selection);
        }
        assert_eq!(total, 668);
    }
}
