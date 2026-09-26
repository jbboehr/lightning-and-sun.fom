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
fn ryis_world_extends_the_reviewed_portraits_and_preserves_their_choices() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let old = read(root.join("palettes/profiles/ryis-portraits.json"));
    let world = read(root.join("palettes/profiles/ryis-world-trial.json"));
    assert_eq!(old["regions"].as_array().unwrap().len(), 109);
    assert_eq!(world["regions"].as_array().unwrap().len(), 258);
    assert_eq!(
        &world["regions"].as_array().unwrap()[..109],
        old["regions"].as_array().unwrap()
    );
    assert_eq!(
        &world["source_colors"].as_array().unwrap()[..8],
        old["source_colors"].as_array().unwrap()
    );
    let mut expected = Vec::new();
    for cycle in ["idle", "walk"] {
        for direction in ["east", "north", "south"] {
            expected.push(format!("assets/animations/NPCs/Ryis/Sprites/Spring/spr_npc_ryis_spring_{cycle}_{direction}.png"));
        }
    }
    let actual: Vec<_> = world["regions"].as_array().unwrap()[109..115]
        .iter()
        .map(|r| r["asset"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(actual, expected);
    let old = read(root.join("palettes/sets/ryis-portraits-trial.json"));
    let world = read(root.join("palettes/sets/ryis-world-trial.json"));
    assert_eq!(world["presets"].as_array().unwrap().len(), 4);
    for (a, b) in old["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(world["presets"].as_array().unwrap())
    {
        assert_eq!(a["id"], b["id"]);
        assert_eq!(a["label"], b["label"]);
        assert_eq!(
            &b["colors"].as_array().unwrap()[..8],
            a["colors"].as_array().unwrap()
        );
        assert_eq!(
            &b["colors"].as_array().unwrap()[8..],
            &a["colors"].as_array().unwrap()[1..2]
        );
    }
}

#[test]
fn ryis_world_package_keeps_his_control_and_native_geometry() {
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
            let name = format!("spr_npc_ryis_spring_{cycle}_{direction}");
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
    assert_eq!(table[0][0], "ryis");
    assert_eq!(table[0][2], "F10");
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
#[ignore = "requires the 258 local animations in extracted/ryis-beach-swim-study"]
fn ryis_world_covers_skin_and_fingers_but_preserves_hair_and_gloves() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-beach-swim-study");
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("bundle");
    let set = std::env::var_os("FOM_RYIS_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/ryis-world-trial.json"));
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
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // These four colors occur only on reviewed skin in the six pinned strips.
    // In particular, 854D3C differs from the portrait's 814A3A shadow.
    let source = [0xB06C57, 0x854D3C, 0x63342A, 0x491F1B];
    let landmarks = [
        ("idle_east", 39, 33, 0x854D3C, true),
        ("idle_east", 35, 44, 0xB06C57, true),
        ("idle_east", 35, 45, 0xECC45E, false),
        ("idle_east", 35, 46, 0x63342A, true),
        ("idle_south", 39, 33, 0x854D3C, true),
        ("idle_south", 39, 39, 0x63342A, true),
        ("idle_south", 40, 30, 0x1B1717, false),
        ("idle_north", 39, 35, 0x5E423B, false),
        ("idle_north", 44, 35, 0xB06C57, true),
        ("idle_north", 39, 39, 0x63342A, true),
    ];
    for preset in read(&set)["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let targets: Vec<_> = [0, 8, 2, 3]
            .map(|i| u32::from_str_radix(&preset["colors"][i].as_str().unwrap()[1..7], 16).unwrap())
            .into();
        let mut total = 0;
        for (name, expected_count) in [
            ("idle_east", 51),
            ("idle_north", 32),
            ("idle_south", 58),
            ("walk_east", 205),
            ("walk_north", 124),
            ("walk_south", 230),
        ] {
            let asset = format!(
                "assets/animations/NPCs/Ryis/Sprites/Spring/spr_npc_ryis_spring_{name}.png"
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
                let expected = match source.iter().position(|c| rgba(*c) == p.0) {
                    Some(i) => {
                        counts[x as usize / 80] += 1;
                        rgba(targets[i])
                    }
                    None => p.0,
                };
                assert_eq!(
                    after.get_pixel(x, y).0,
                    expected,
                    "skin/material coverage {id} {name} [{x},{y}]"
                );
            }
            assert!(counts.iter().all(|n| *n > 0));
            assert_eq!(counts.iter().sum::<usize>(), expected_count);
            total += expected_count;
            for &(case, x, y, color, skin) in &landmarks {
                if case != name {
                    continue;
                }
                assert_eq!(before.get_pixel(x, y).0, rgba(color));
                assert_eq!(before.get_pixel(x, y) != after.get_pixel(x, y), skin);
            }
        }
        assert_eq!(total, 700);
    }
}
