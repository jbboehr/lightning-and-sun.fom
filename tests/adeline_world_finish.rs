use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, process::Command};

const CASES: [(&str, &str, &[u32]); 33] = [
    ("Beach", "action_east", &[63, 77, 71, 77, 71, 63, 76]),
    ("Beach", "action_north", &[22, 23, 23, 23, 23, 24, 32]),
    ("Beach", "action_south", &[81, 80, 77, 80, 77, 81, 89]),
    ("Beach", "bath_swim_east", &[25, 25, 20, 20]),
    ("Beach", "bath_swim_north", &[0, 0, 0, 0]),
    ("Beach", "bath_swim_south", &[26, 26, 20, 20]),
    ("Beach", "blink_east", &[78, 86, 78]),
    ("Beach", "blink_south", &[91, 99, 91]),
    ("Beach", "idle_east", &[76]),
    ("Beach", "idle_north", &[32]),
    ("Beach", "idle_south", &[89]),
    ("Beach", "kiss_east", &[69, 73, 82, 78]),
    ("Beach", "sit_east", &[56]),
    ("Beach", "sit_north", &[13]),
    ("Beach", "sit_south", &[70]),
    ("Beach", "walk_east", &[76, 79, 76, 72]),
    ("Beach", "walk_north", &[32, 25, 32, 25]),
    ("Beach", "walk_south", &[89, 81, 89, 80]),
    ("Wedding", "action_east", &[28, 26, 28, 26, 28, 28, 28]),
    ("Wedding", "action_north", &[0, 0, 0, 0, 0, 0, 0]),
    ("Wedding", "action_south", &[28, 28, 28, 28, 28, 28, 28]),
    ("Wedding", "blink_east", &[30, 38, 30]),
    ("Wedding", "blink_south", &[30, 38, 30]),
    ("Wedding", "idle_east", &[28]),
    ("Wedding", "idle_north", &[0]),
    ("Wedding", "idle_south", &[28]),
    ("Wedding", "kiss_east", &[28, 33, 38, 38]),
    ("Wedding", "sit_east", &[28]),
    ("Wedding", "sit_north", &[0]),
    ("Wedding", "sit_south", &[28]),
    ("Wedding", "walk_east", &[28, 28, 28, 29]),
    ("Wedding", "walk_north", &[0, 0, 0, 0]),
    ("Wedding", "walk_south", &[28, 28, 28, 28]),
];

fn asset(outfit: &str, pose: &str) -> String {
    format!(
        "assets/animations/NPCs/Adeline/Sprites/{outfit}/spr_npc_adeline_{}_{pose}.png",
        outfit.to_lowercase()
    )
}
fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn adeline_finish_preserves_existing_regions_and_maps_every_remaining_outfit() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let before = read_json(root.join("palettes/profiles/adeline-world-actions.json"));
    let after = read_json(root.join("palettes/profiles/adeline-world-finish.json"));
    assert_eq!(before["regions"].as_array().unwrap().len(), 282);
    assert_eq!(
        &after["regions"].as_array().unwrap()[..282],
        before["regions"].as_array().unwrap()
    );
    assert_eq!(after["source_colors"], before["source_colors"]);
    assert_eq!(after["color_groups"], before["color_groups"]);
    let old_set = read_json(root.join("palettes/sets/adeline-world-actions-trial.json"));
    let new_set = read_json(root.join("palettes/sets/adeline-world-finish-trial.json"));
    assert_eq!(new_set["presets"], old_set["presets"]);
    assert_eq!(new_set["profile"], "../profiles/adeline-world-finish.json");
    let expected: BTreeSet<_> = CASES.iter().map(|(o, p, _)| asset(o, p)).collect();
    let rows = &after["regions"].as_array().unwrap()[282..];
    assert_eq!(rows.len(), 33);
    assert_eq!(
        rows.iter()
            .map(|r| r["asset"].as_str().unwrap().to_string())
            .collect::<BTreeSet<_>>(),
        expected
    );
    let mut empty = 0;
    for (outfit, pose, counts) in CASES {
        let row = rows
            .iter()
            .find(|r| r["asset"] == asset(outfit, pose))
            .unwrap();
        assert_eq!(row["size"], serde_json::json!([80 * counts.len(), 80]));
        assert_eq!(
            row["seeds"].as_array().unwrap().is_empty(),
            counts.iter().all(|n| *n == 0)
        );
        empty += usize::from(counts.iter().all(|n| *n == 0));
    }
    assert_eq!(empty, 5);
}

#[test]
fn adeline_finish_package_preserves_native_metadata_and_no_change_animations() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();
    for (outfit, pose, counts) in CASES {
        let path = asset(outfit, pose);
        let stem = Path::new(&path).file_stem().unwrap().to_str().unwrap();
        let timing = if pose.starts_with("action_") {
            "frame_len=7\nduration=[0.1,0.25,0.25,0.25,0.25,0.1,0.4]\n"
        } else if pose.starts_with("blink_") {
            "frame_len=3\nduration=[0.075,0.125,0.075]\n"
        } else if pose.starts_with("kiss_") {
            "frame_len=4\nduration=[0.15,0.15,0.8,0.15]\n"
        } else if pose.starts_with("walk_") || pose.starts_with("bath_swim_") {
            "frame_len=4\nduration=0.15\n"
        } else {
            ""
        };
        let meta = format!(
            "[meta_properties]\nid='synthetic'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[80,80]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal='Middle'\nvertical=54.0\n"
        );
        let unchanged = counts.iter().all(|n| *n == 0);
        for (directory, color) in [
            (&original, [10, 20, 30, 255]),
            (
                &modified,
                if unchanged {
                    [10, 20, 30, 255]
                } else {
                    [40, 50, 60, 255]
                },
            ),
        ] {
            RgbaImage::from_pixel(counts.len() as u32 * 80, 80, Rgba(color))
                .save(directory.join(format!("{stem}.png")))
                .unwrap();
            fs::write(directory.join(format!("{stem}.meta.toml")), &meta).unwrap();
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
    assert_eq!(table[0][0], "adeline");
    assert_eq!(table[0][2], "F6");
    let rows = table[0][4].as_array().unwrap();
    assert_eq!(rows.len(), 33);
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
    assert_eq!(total_frames, 110);
}

#[test]
#[ignore = "requires Adeline's complete original export in extracted/adeline-finish-study"]
fn adeline_finish_source_materials_match_all_four_palettes_and_preserve_costumes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/adeline-finish-study");
    let set = std::env::var_os("FOM_ADELINE_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/adeline-world-finish-trial.json"));
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("bundle");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("build-presets")
        .arg("--original")
        .arg(&original)
        .arg("--presets")
        .arg(set)
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
    let skin = [0xE9A980, 0xDE8F5D, 0xBA6A4C, 0x7D3B14].map(rgba);
    let choices = [
        ("blue", [0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83]),
        ("npc_hayden", [0xE8B271, 0xCA9052, 0xB27146, 0x6E4922]),
        ("npc_ryis", [0xB06C57, 0x814A3A, 0x63342A, 0x491F1B]),
        ("npc_seridia", [0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48]),
    ];
    // Literal landmarks distinguish exposed forearm/finger/toe skin from white sandals,
    // crown shades shared with the portrait ramp, gold trim and white gloves.
    let landmarks = [
        ("Beach", "action_east", 0, 41, 35, 0xDE8F5D, true),
        ("Beach", "action_east", 0, 38, 43, 0xBA6A4C, true),
        ("Beach", "action_east", 0, 40, 48, 0x7D3B14, true),
        ("Beach", "action_east", 0, 40, 50, 0xE9A980, true),
        ("Beach", "action_east", 0, 39, 53, 0xE9A980, true),
        ("Beach", "action_east", 0, 38, 52, 0xCFDBF6, false),
        ("Beach", "bath_swim_east", 0, 40, 48, 0xBA6A4C, true),
        ("Beach", "bath_swim_east", 0, 40, 50, 0xE9A980, true),
        ("Beach", "bath_swim_east", 0, 38, 54, 0x7D3B14, true),
        ("Wedding", "action_east", 0, 39, 29, 0x9F5544, false),
        ("Wedding", "action_east", 0, 40, 28, 0xF1BE73, false),
        ("Wedding", "action_east", 0, 40, 30, 0xCB7A5D, false),
        ("Wedding", "action_east", 0, 41, 35, 0xDE8F5D, true),
        ("Wedding", "action_east", 0, 41, 42, 0xDE8F5D, true),
        ("Wedding", "action_east", 0, 40, 43, 0x9F5544, false),
        ("Wedding", "action_east", 0, 41, 44, 0xF1BE73, false),
        ("Wedding", "action_east", 0, 43, 46, 0x9F5544, false),
        ("Wedding", "action_north", 0, 38, 28, 0x9F5544, false),
        ("Wedding", "kiss_east", 3, 41, 35, 0xDE8F5D, true),
        ("Wedding", "kiss_east", 3, 39, 46, 0x9F5544, false),
    ];
    for (choice, shades) in choices {
        let targets = shades.map(rgba);
        let mut total = 0;
        for (outfit, pose, counts) in CASES {
            let path = asset(outfit, pose);
            let before = image::open(original.join(&path)).unwrap().to_rgba8();
            let after = image::open(output.join("variants").join(choice).join(&path))
                .unwrap()
                .to_rgba8();
            assert_eq!(before.dimensions(), (counts.len() as u32 * 80, 80));
            assert_eq!(after.dimensions(), before.dimensions());
            let meta = path.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(output.join("variants").join(choice).join(&meta)).unwrap()
            );
            for &(o, p, frame, x, y, color, selected) in &landmarks {
                if o != outfit || p != pose {
                    continue;
                }
                let x = frame * 80 + x;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    rgba(color),
                    "source landmark {path} {x},{y}"
                );
                let want = if selected {
                    targets[skin.iter().position(|c| *c == rgba(color)).unwrap()]
                } else {
                    rgba(color)
                };
                assert_eq!(
                    after.get_pixel(x, y).0,
                    want,
                    "material landmark {choice} {path} {x},{y}"
                );
            }
            let mut source_counts = vec![0; counts.len()];
            for (x, y, pixel) in before.enumerate_pixels() {
                let expected = match skin.iter().position(|c| *c == pixel.0) {
                    Some(shade) => {
                        source_counts[x as usize / 80] += 1;
                        targets[shade]
                    }
                    None => pixel.0,
                };
                assert_eq!(
                    after.get_pixel(x, y).0,
                    expected,
                    "source material {choice} {path} {x},{y}"
                );
            }
            assert_eq!(
                source_counts, counts,
                "independent source color inventory {path}"
            );
            total += source_counts.iter().sum::<u32>();
        }
        assert_eq!(total, 4390);
    }
}
