use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, process::Command};

const SEASONS: [&str; 4] = ["Autumn", "Spring", "Summer", "Winter"];
const CASES: [(&str, u32); 11] = [
    ("action_east", 7),
    ("action_north", 7),
    ("action_south", 7),
    ("blink_east", 3),
    ("blink_south", 3),
    ("idle_east", 1),
    ("idle_north", 1),
    ("idle_south", 1),
    ("walk_east", 4),
    ("walk_north", 4),
    ("walk_south", 4),
];

fn asset(season: &str, name: &str) -> String {
    format!(
        "assets/animations/NPCs/Louis/Sprites/{season}/spr_npc_louis_{}_{name}.png",
        season.to_lowercase()
    )
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn louis_world_preserves_portrait_recipes_and_covers_every_seasonal_cycle() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let old = read_json(root.join("palettes/profiles/louis-portraits.json"));
    let new = read_json(root.join("palettes/profiles/louis-world-trial.json"));
    assert_eq!(old["regions"].as_array().unwrap().len(), 32);
    assert_eq!(
        &new["regions"].as_array().unwrap()[..32],
        old["regions"].as_array().unwrap()
    );
    assert_eq!(
        &new["source_colors"].as_array().unwrap()[..16],
        old["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &new["color_groups"].as_array().unwrap()[..16],
        old["color_groups"].as_array().unwrap()
    );
    assert_eq!(new["source_colors"].as_array().unwrap().len(), 18);
    assert_eq!(new["color_groups"][16], serde_json::json!(["#FFDBB8"]));
    assert_eq!(new["color_groups"][17], serde_json::json!(["#B46D5F"]));
    let previous = read_json(root.join("palettes/sets/louis-portraits-trial.json"));
    let current = read_json(root.join("palettes/sets/louis-world-trial.json"));
    assert_eq!(current["presets"].as_array().unwrap().len(), 4);
    for (before, after) in previous["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(current["presets"].as_array().unwrap())
    {
        assert_eq!(before["id"], after["id"]);
        assert_eq!(before["label"], after["label"]);
        assert_eq!(
            &after["colors"].as_array().unwrap()[..16],
            before["colors"].as_array().unwrap()
        );
        assert_eq!(after["colors"][16], before["colors"][0]);
        assert_eq!(after["colors"][17], before["colors"][2]);
    }
    let mut expected = BTreeSet::new();
    for season in SEASONS {
        for (name, frames) in CASES {
            let path = asset(season, name);
            expected.insert(path.clone());
            let row = new["regions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["asset"] == path)
                .unwrap();
            assert_eq!(row["size"], serde_json::json!([80 * frames, 80]));
            assert!(!row["seeds"].as_array().unwrap().is_empty());
        }
    }
    let world: Vec<_> = new["regions"]
        .as_array()
        .unwrap()
        .iter()
        .skip(32)
        .map(|r| r["asset"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(world.len(), 44);
    assert_eq!(world.into_iter().collect::<BTreeSet<_>>(), expected);
}

#[test]
fn louis_world_package_retains_native_frame_timing_offsets_and_pixels() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();
    for season in SEASONS {
        for (name, frames) in CASES {
            let path = asset(season, name);
            let stem = Path::new(&path).file_stem().unwrap().to_str().unwrap();
            let timing = match name.split('_').next().unwrap() {
                "action" => "frame_len=7\nduration=[0.1,0.25,0.25,0.25,0.25,0.1,0.4]\n",
                "blink" => "frame_len=3\nduration=[0.075,0.125,0.075]\n",
                "walk" => "frame_len=4\nduration=0.15\n",
                "idle" => "",
                _ => unreachable!(),
            };
            let meta = format!(
                "[meta_properties]\nid='synthetic'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[80,80]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal=40.0\nvertical=54.0\n"
            );
            for (directory, color) in [
                (&original, [10, 20, 30, 255]),
                (&modified, [40, 50, 60, 255]),
            ] {
                RgbaImage::from_pixel(frames * 80, 80, Rgba(color))
                    .save(directory.join(format!("{stem}.png")))
                    .unwrap();
                fs::write(directory.join(format!("{stem}.meta.toml")), &meta).unwrap();
            }
        }
    }
    let output = temp.path().join("package");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["package-toggle", "--original"])
        .arg(&original)
        .arg("--modified")
        .arg(&modified)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
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
    assert_eq!(table[0][0], "louis");
    assert_eq!(table[0][2], "V");
    let rows = table[0][4].as_array().unwrap();
    assert_eq!(rows.len(), 44);
    let mut total_frames = 0;
    for row in rows {
        let stem = row[0].as_str().unwrap();
        let target = row[1].as_str().unwrap();
        let before: toml::Value = toml::from_str(
            &fs::read_to_string(original.join(format!("{stem}.meta.toml"))).unwrap(),
        )
        .unwrap();
        let after: toml::Value = toml::from_str(
            &fs::read_to_string(
                output.join(format!("animations/LightningAndSun/{target}.meta.toml")),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            before["asset_properties"], after["asset_properties"],
            "{stem}"
        );
        assert_eq!(
            fs::read(modified.join(format!("{stem}.png"))).unwrap(),
            fs::read(output.join(format!("animations/LightningAndSun/{target}.png"))).unwrap()
        );
        total_frames += before["asset_properties"]
            .get("frame_len")
            .and_then(toml::Value::as_integer)
            .unwrap_or(1);
    }
    assert_eq!(total_frames, 168);
}

#[test]
#[ignore = "requires Louis's complete source export in extracted/louis-pilot-study"]
fn louis_world_recolors_source_skin_and_preserves_hair_cuffs_glasses_mouths_and_boots() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/louis-pilot-study");
    let set = std::env::var_os("FOM_LOUIS_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/louis-world-trial.json"));
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("bundle");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("build-presets")
        .arg("--original")
        .arg(&original)
        .arg("--presets")
        .arg(&set)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let rgba = |n: u32| [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255];
    let source = [
        0xF6D2AF, 0xCB9879, 0xAD6F5C, 0x784B42, 0xAB6F5C, 0xAC6F5C, 0xC99679, 0xCA9779, 0xCA9879,
        0xF4D0AE, 0xE3B590, 0xE3BD97, 0xF1CEAD, 0x634040, 0x40252A, 0x491F1B, 0xFFDBB8, 0xB46D5F,
    ]
    .map(rgba);
    let roles = [0, 1, 2, 3, 2, 2, 1, 1, 1, 0, 1, 1, 0, 3, 3, 3, 0, 2];
    let counts: [&[u32]; 44] = [
        &[32, 34, 33, 34, 33, 32, 35],
        &[14, 12, 12, 12, 12, 14, 18],
        &[39, 38, 38, 38, 38, 39, 41],
        &[38, 47, 38],
        &[44, 52, 44],
        &[35],
        &[18],
        &[41],
        &[35, 38, 35, 34],
        &[18, 16, 18, 16],
        &[41, 39, 41, 39],
        &[32, 34, 32, 34, 32, 32, 35],
        &[14, 12, 12, 12, 12, 14, 18],
        &[39, 39, 39, 39, 39, 39, 41],
        &[39, 47, 39],
        &[44, 52, 44],
        &[35],
        &[18],
        &[41],
        &[35, 38, 35, 34],
        &[18, 16, 18, 16],
        &[41, 39, 41, 39],
        &[35, 36, 35, 36, 35, 35, 37],
        &[18, 15, 15, 15, 15, 18, 24],
        &[43, 41, 42, 41, 42, 43, 46],
        &[41, 49, 41],
        &[49, 57, 49],
        &[37],
        &[24],
        &[46],
        &[37, 42, 37, 36],
        &[24, 20, 24, 20],
        &[46, 43, 46, 43],
        &[32, 34, 33, 34, 33, 32, 33],
        &[14, 12, 12, 12, 12, 14, 18],
        &[39, 38, 38, 38, 38, 39, 41],
        &[36, 45, 36],
        &[44, 52, 44],
        &[33],
        &[18],
        &[41],
        &[33, 38, 33, 33],
        &[18, 16, 18, 16],
        &[41, 39, 41, 39],
    ];
    let choices = [
        ("blue", [0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83]),
        ("npc_hayden", [0xE8B271, 0xCA9052, 0xB27146, 0x6E4922]),
        ("npc_ryis", [0xB06C57, 0x814A3A, 0x63342A, 0x491F1B]),
        ("npc_seridia", [0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48]),
    ];
    for (choice, shades) in choices {
        let target = shades.map(rgba);
        let mut total = 0;
        for (si, season) in SEASONS.iter().enumerate() {
            for (ci, (name, frames)) in CASES.iter().enumerate() {
                let path = asset(season, name);
                let before = image::open(original.join(&path)).unwrap().to_rgba8();
                let after = image::open(output.join("variants").join(choice).join(&path))
                    .unwrap()
                    .to_rgba8();
                assert_eq!(before.dimensions(), (frames * 80, 80));
                assert_eq!(after.dimensions(), before.dimensions());
                let meta = path.replace(".png", ".meta.toml");
                assert_eq!(
                    fs::read(original.join(&meta)).unwrap(),
                    fs::read(output.join("variants").join(choice).join(&meta)).unwrap()
                );
                let landmarks: &[(&str, &str, u32, u32, u32, u32, bool)] = &[
                    ("Autumn", "action_east", 0, 39, 46, 0xFFDBB8, true),
                    ("Autumn", "action_east", 0, 40, 47, 0x784B42, true),
                    ("Autumn", "action_north", 0, 45, 40, 0x784B42, true),
                    ("Autumn", "action_south", 0, 45, 45, 0xB46D5F, true),
                    ("Spring", "action_east", 0, 41, 41, 0xCB9879, false),
                    ("Spring", "action_east", 0, 43, 42, 0xB46D5F, false),
                    ("Spring", "action_east", 0, 42, 43, 0xCB9879, false),
                    ("Spring", "action_east", 0, 41, 46, 0xCB9879, true),
                    ("Spring", "action_south", 1, 44, 44, 0xB46D5F, true),
                    ("Summer", "action_east", 0, 38, 44, 0xB46D5F, false),
                    ("Summer", "action_east", 0, 38, 45, 0xCB9879, false),
                    ("Summer", "action_east", 0, 39, 45, 0x784B42, true),
                    ("Summer", "action_east", 0, 40, 45, 0x784B42, true),
                    ("Summer", "action_east", 1, 44, 41, 0xCB9879, false),
                    ("Summer", "action_east", 1, 43, 42, 0xCB9879, false),
                    ("Summer", "action_east", 1, 44, 43, 0xB46D5F, false),
                    ("Summer", "action_east", 1, 45, 43, 0xCB9879, false),
                    ("Summer", "action_east", 1, 46, 42, 0xB46D5F, false),
                    ("Summer", "action_east", 1, 47, 42, 0x784B42, true),
                    ("Summer", "action_east", 1, 46, 43, 0x784B42, true),
                    ("Summer", "action_east", 2, 42, 43, 0xB46D5F, false),
                    ("Summer", "action_east", 2, 43, 42, 0xCB9879, false),
                    ("Summer", "action_east", 2, 44, 43, 0x784B42, true),
                    ("Summer", "action_east", 2, 43, 44, 0x784B42, true),
                    ("Summer", "action_north", 0, 35, 41, 0xCB9879, false),
                    ("Summer", "action_north", 0, 36, 42, 0xCB9879, false),
                    ("Summer", "action_north", 0, 43, 42, 0xB46D5F, false),
                    ("Summer", "action_north", 0, 44, 43, 0xB46D5F, false),
                    ("Summer", "action_north", 0, 33, 42, 0x784B42, true),
                    ("Summer", "action_north", 0, 34, 42, 0x784B42, true),
                    ("Summer", "action_north", 0, 35, 43, 0x784B42, true),
                    ("Summer", "action_north", 0, 46, 41, 0x784B42, true),
                    ("Winter", "action_east", 6, 44, 46, 0x621944, false),
                    ("Winter", "idle_east", 0, 44, 46, 0x621944, false),
                ];
                for &(outfit, pose, frame, x, y, color, selected) in landmarks {
                    if outfit != *season || pose != *name {
                        continue;
                    }
                    let x = frame * 80 + x;
                    let source_pixel = rgba(color);
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        source_pixel,
                        "source landmark {path} {x},{y}"
                    );
                    let expected = if selected {
                        target[roles[source.iter().position(|p| *p == source_pixel).unwrap()]]
                    } else {
                        source_pixel
                    };
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        expected,
                        "material landmark {choice} {path} {x},{y}"
                    );
                }
                let mut changed = vec![0; *frames as usize];
                for (x, y, p) in before.enumerate_pixels() {
                    let new = after.get_pixel(x, y);
                    let anatomical_face =
                        y < 38 || (y < 40 && (!name.ends_with("north") || p.0 != rgba(0xCB9879)));
                    if anatomical_face
                        && [0xCB9879, 0xF6D2AF, 0xFFDBB8, 0xB46D5F, 0x784B42]
                            .map(rgba)
                            .contains(&p.0)
                    {
                        let shade = source.iter().position(|v| *v == p.0).unwrap();
                        assert_eq!(
                            new.0, target[roles[shade]],
                            "source face {choice} {path} {x},{y}"
                        );
                    }
                    assert_eq!(p[3], new[3], "alpha {path} {x},{y}");
                    if p != new {
                        let shade = source
                            .iter()
                            .position(|v| v == &p.0)
                            .unwrap_or_else(|| panic!("unrelated material {path} {x},{y}"));
                        assert_eq!(
                            new.0, target[roles[shade]],
                            "wrong mapping {choice} {path} {x},{y}"
                        );
                        assert!(y < 49, "boots/trousers changed {path} {x},{y}");
                        assert!(
                            ![0x634040, 0x40252A, 0x491F1B].map(rgba).contains(&p.0),
                            "hair/mouth changed {path} {x},{y}"
                        );
                        changed[x as usize / 80] += 1;
                    }
                }
                // These literal source coordinates identify cream sleeve corners below the hair,
                // where a face-height cutoff incorrectly includes the cuff during the action.
                if *season == "Summer" && *name == "action_north" {
                    for (frame, x) in [(1, 44), (2, 45), (3, 44), (4, 45)] {
                        let x = frame * 80 + x;
                        assert_eq!(before.get_pixel(x, 39).0, rgba(0xCB9879));
                        assert_eq!(
                            after.get_pixel(x, 39),
                            before.get_pixel(x, 39),
                            "cream cuff {path} {x},39"
                        );
                    }
                }
                assert_eq!(
                    changed,
                    counts[si * 11 + ci],
                    "source material counts {choice} {path}"
                );
                total += changed.iter().sum::<u32>();
            }
        }
        assert_eq!(total, 5442);
    }
}
