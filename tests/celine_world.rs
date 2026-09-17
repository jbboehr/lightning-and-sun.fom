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
fn celine_world_extends_the_reviewed_portraits_and_preserves_their_choices() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let old = read(root.join("palettes/profiles/celine-portraits.json"));
    let world = read(root.join("palettes/profiles/celine-world-trial.json"));
    assert_eq!(old["regions"].as_array().unwrap().len(), 183);
    assert_eq!(world["regions"].as_array().unwrap().len(), 242);
    assert_eq!(
        &world["regions"].as_array().unwrap()[..183],
        old["regions"].as_array().unwrap()
    );
    assert_eq!(
        &world["source_colors"].as_array().unwrap()[..11],
        old["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &world["color_groups"].as_array().unwrap()[..6],
        old["color_groups"].as_array().unwrap()
    );
    let mut expected = Vec::new();
    for cycle in ["idle", "walk"] {
        for direction in ["east", "north", "south"] {
            expected.push(format!("assets/animations/NPCs/Celine/Sprites/Spring/spr_npc_celine_spring_{cycle}_{direction}.png"));
        }
    }
    let actual: Vec<_> = world["regions"].as_array().unwrap()[183..189]
        .iter()
        .map(|r| r["asset"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(actual, expected);
    let old = read(root.join("palettes/sets/celine-portraits-trial.json"));
    let world = read(root.join("palettes/sets/celine-world-trial.json"));
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
            &b["colors"].as_array().unwrap()[..11],
            a["colors"].as_array().unwrap()
        );
        assert_eq!(
            &b["colors"].as_array().unwrap()[11..],
            &a["colors"].as_array().unwrap()[..3]
        );
    }
}

#[test]
fn celine_world_package_keeps_her_control_and_native_geometry() {
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
            let name = format!("spr_npc_celine_spring_{cycle}_{direction}");
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
    assert_eq!(table[0][0], "celine");
    assert_eq!(table[0][2], "INSERT");
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
#[ignore = "requires the 242 local animations in extracted/celine-summer-expansion-study"]
fn celine_world_covers_skin_without_crossing_into_hair_or_boots() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/celine-summer-expansion-study");
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("bundle");
    let set = std::env::var_os("FOM_CELINE_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/celine-world-trial.json"));
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
    let source = [0xFCD9B3, 0xF0B988, 0xD37A57, 0x672115];
    // The outline shares a color with hair and boots. Pin both sides of those
    // boundaries; the other three shades occur only on reviewed world skin.
    let landmarks = [
        ("idle_south", 0, 36, 36, 0x672115, true),
        ("idle_south", 0, 35, 36, 0xB65932, false),
        ("idle_south", 0, 33, 47, 0x672115, true),
        ("idle_south", 0, 37, 52, 0x672115, false),
        ("idle_east", 0, 44, 36, 0x672115, true),
        ("idle_east", 0, 35, 31, 0x672115, false),
        ("idle_east", 0, 38, 45, 0x672115, false),
        ("idle_east", 0, 35, 47, 0x672115, true),
        ("idle_north", 0, 38, 36, 0x672115, false),
        ("idle_north", 0, 35, 45, 0x672115, false),
        ("idle_north", 0, 33, 47, 0x672115, true),
        ("walk_east", 1, 39, 50, 0xFCD9B3, true),
    ];
    let mut first_selection = Vec::new();
    for preset in read(&set)["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let targets = [11, 12, 13, 9].map(|i| {
            u32::from_str_radix(&preset["colors"][i].as_str().unwrap()[1..7], 16).unwrap()
        });
        let mut selection = Vec::new();
        let mut total = 0;
        for (name, expected_count) in [
            ("idle_east", 45),
            ("idle_north", 12),
            ("idle_south", 52),
            ("walk_east", 184),
            ("walk_north", 38),
            ("walk_south", 198),
        ] {
            let asset = format!(
                "assets/animations/NPCs/Celine/Sprites/Spring/spr_npc_celine_spring_{name}.png"
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
                let shade = source.iter().position(|c| rgba(*c) == p.0);
                if p != q {
                    let i = shade.expect("non-skin material color changed");
                    assert_eq!(q.0, rgba(targets[i]));
                    counts[x as usize / 80] += 1;
                } else if let Some(i) = shade {
                    assert_eq!(i, 3, "missing skin shade {id} {name} [{x},{y}]");
                }
            }
            assert!(counts.iter().all(|n| *n > 0));
            assert_eq!(counts.iter().sum::<usize>(), expected_count, "{id} {name}");
            total += expected_count;
            for &(case, frame, x, y, color, skin) in &landmarks {
                if case != name {
                    continue;
                }
                let x = x + frame * 80;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    rgba(color),
                    "source {case} [{x},{y}]"
                );
                assert_eq!(
                    before.get_pixel(x, y) != after.get_pixel(x, y),
                    skin,
                    "material boundary {id} {case} [{x},{y}]"
                );
            }
        }
        if first_selection.is_empty() {
            first_selection = selection;
        } else {
            assert_eq!(first_selection, selection);
        }
        assert_eq!(total, 529);
    }
}
