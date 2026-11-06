use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, process::Command};

const SEASONS: [&str; 4] = ["Autumn", "Spring", "Summer", "Winter"];
const ACTIONS: [(&str, u32, &str); 20] = [
    ("action_east", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("action_north", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("action_south", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("blink_east", 3, "[0.075,0.125,0.075]"),
    ("blink_south", 3, "[0.075,0.125,0.075]"),
    ("drink_east", 3, "1.0"),
    ("drink_north", 3, "1.0"),
    ("drink_south", 3, "1.0"),
    ("eat_east", 5, "[0.125,0.15,0.175,0.125,0.6]"),
    ("eat_north", 3, "1.0"),
    ("eat_south", 5, "[0.125,0.15,0.175,0.125,0.6]"),
    ("idle_east", 1, ""),
    ("idle_north", 1, ""),
    ("idle_south", 1, ""),
    ("sit_east", 1, ""),
    ("sit_north", 1, ""),
    ("sit_south", 1, ""),
    ("walk_east", 4, "0.15"),
    ("walk_north", 4, "0.15"),
    ("walk_south", 4, "0.15"),
];

fn inventory() -> Vec<(String, u32, &'static str)> {
    let mut entries = Vec::new();
    for season in SEASONS {
        let lower = season.to_lowercase();
        entries.extend(ACTIONS.map(|(a, n, d)| (format!("{lower}_{a}"), n, d)));
        for (action, n, d) in [
            ("net_east", 7, "[0.1,0.1,0.1,0.1,0.1,0.2,0.1]"),
            ("scratch_head_end_south", 1, ""),
            ("scratch_head_loop_south", 2, "0.2"),
            ("scratch_head_start_south", 1, ""),
        ] {
            entries.push((format!("specialanimation_{lower}_{action}"), n, d));
        }
        if season == "Spring" {
            for part in ["end", "loop", "start"] {
                entries.push((format!("spring_shocked_{part}_south"), 1, ""));
            }
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

fn source_asset(name: &str) -> String {
    let season = SEASONS
        .iter()
        .find(|season| name.contains(&format!("{}_", season.to_lowercase())))
        .unwrap();
    format!("assets/animations/NPCs/Luc/Sprites/{season}/spr_npc_luc_{name}.png")
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn world_profile_retains_every_portrait_region_and_palette_role() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let before = read_json(root.join("palettes/profiles/luc-portraits.json"));
    let after = read_json(root.join("palettes/profiles/luc-world-trial.json"));
    assert_eq!(
        &after["source_colors"].as_array().unwrap()[..9],
        before["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &after["source_colors"].as_array().unwrap()[9..],
        &[serde_json::json!("#8E4538")]
    );
    assert_eq!(
        &after["color_groups"].as_array().unwrap()[..9],
        before["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        &after["color_groups"].as_array().unwrap()[9..],
        &[serde_json::json!(["#8E4538"])]
    );
    let regions = after["regions"].as_array().unwrap();
    assert_eq!(&regions[..32], before["regions"].as_array().unwrap());
    assert_eq!(regions.len(), 131);
    let expected: BTreeSet<_> = inventory()
        .iter()
        .map(|(name, _, _)| source_asset(name))
        .collect();
    assert_eq!(expected.len(), 99);
    let actual: BTreeSet<_> = regions[32..]
        .iter()
        .map(|r| r["asset"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(actual, expected);
    let old_set = read_json(root.join("palettes/sets/luc-portraits-trial.json"));
    let new_set = read_json(root.join("palettes/sets/luc-world-trial.json"));
    for (before, after) in old_set["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(new_set["presets"].as_array().unwrap())
    {
        assert_eq!(before["id"], after["id"]);
        assert_eq!(before["label"], after["label"]);
        assert_eq!(
            before["colors"].as_array().unwrap(),
            &after["colors"].as_array().unwrap()[..9]
        );
        assert_eq!(after["colors"][9], before["colors"][3]);
    }
    assert_eq!(
        old_set["presets"].as_array().unwrap().len(),
        new_set["presets"].as_array().unwrap().len()
    );
    for region in &regions[32..] {
        assert!(!region["seeds"].as_array().unwrap().is_empty());
    }
}

#[test]
fn all_four_season_packages_preserve_native_frames_timing_offsets_and_atlas() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();
    for (name, frames, duration) in inventory() {
        let stem = format!("spr_npc_luc_{name}");
        let timing = if frames == 1 {
            String::new()
        } else {
            format!("frame_len={frames}\nduration={duration}\n")
        };
        let metadata = format!(
            "[meta_properties]\nid='synthetic'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[80,80]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal=40.0\nvertical=54.0\n"
        );
        for (directory, color) in [
            (&original, [10, 20, 30, 255]),
            (&modified, [40, 50, 60, 255]),
        ] {
            RgbaImage::from_pixel(frames * 80, 80, Rgba(color))
                .save(directory.join(format!("{stem}.png")))
                .unwrap();
            fs::write(directory.join(format!("{stem}.meta.toml")), &metadata).unwrap();
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
    assert_eq!(table[0][0], "luc");
    assert_eq!(table[0][4].as_array().unwrap().len(), 99);
    let mut frames = 0;
    for row in table[0][4].as_array().unwrap() {
        let stem = row[0].as_str().unwrap();
        assert!(!stem.contains("west"));
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
        frames += before["asset_properties"]
            .get("frame_len")
            .and_then(toml::Value::as_integer)
            .unwrap_or(1);
        assert_eq!(
            fs::read(modified.join(format!("{stem}.png"))).unwrap(),
            fs::read(output.join(format!("animations/LightningAndSun/{target}.png"))).unwrap()
        );
    }
    assert_eq!(frames, 315);
}

#[test]
#[ignore = "requires the complete Luc source corpus; checks all world pixels and retained portraits"]
fn every_world_pixel_matches_reviewed_materials_and_portraits_remain_identical() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/luc");
    let set = std::env::var_os("FOM_LUC_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/luc-world-trial.json"));
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("bundle");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["build-presets", "--original"])
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
    let cases: [(&str, &[usize]); 99] = [
        ("autumn_action_east", &[40, 39, 39, 39, 39, 40, 40]),
        ("autumn_action_north", &[16, 12, 12, 12, 12, 16, 16]),
        ("autumn_action_south", &[46, 46, 47, 46, 47, 46, 48]),
        ("autumn_blink_east", &[40, 40, 40]),
        ("autumn_blink_south", &[48, 48, 48]),
        ("autumn_drink_east", &[42, 35, 42]),
        ("autumn_drink_north", &[21, 11, 21]),
        ("autumn_drink_south", &[51, 49, 51]),
        ("autumn_eat_east", &[39, 39, 27, 39, 39]),
        ("autumn_eat_north", &[21, 11, 21]),
        ("autumn_eat_south", &[51, 51, 47, 54, 50]),
        ("autumn_idle_east", &[40]),
        ("autumn_idle_north", &[16]),
        ("autumn_idle_south", &[48]),
        ("autumn_sit_east", &[40]),
        ("autumn_sit_north", &[18]),
        ("autumn_sit_south", &[50]),
        ("autumn_walk_east", &[40, 42, 40, 39]),
        ("autumn_walk_north", &[16, 12, 16, 12]),
        ("autumn_walk_south", &[48, 45, 48, 45]),
        (
            "specialanimation_autumn_net_east",
            &[45, 38, 39, 43, 43, 43, 44],
        ),
        ("specialanimation_autumn_scratch_head_end_south", &[50]),
        ("specialanimation_autumn_scratch_head_loop_south", &[47, 46]),
        ("specialanimation_autumn_scratch_head_start_south", &[50]),
        (
            "specialanimation_spring_net_east",
            &[53, 50, 51, 51, 51, 51, 52],
        ),
        ("specialanimation_spring_scratch_head_end_south", &[66]),
        ("specialanimation_spring_scratch_head_loop_south", &[61, 62]),
        ("specialanimation_spring_scratch_head_start_south", &[66]),
        ("spring_action_east", &[46, 52, 51, 52, 51, 47, 53]),
        ("spring_action_north", &[25, 21, 23, 21, 23, 25, 26]),
        ("spring_action_south", &[55, 53, 55, 53, 55, 55, 58]),
        ("spring_blink_east", &[53, 53, 53]),
        ("spring_blink_south", &[58, 58, 58]),
        ("spring_drink_east", &[52, 48, 52]),
        ("spring_drink_north", &[23, 19, 23]),
        ("spring_drink_south", &[61, 57, 61]),
        ("spring_eat_east", &[49, 52, 39, 46, 47]),
        ("spring_eat_north", &[23, 19, 23]),
        ("spring_eat_south", &[62, 57, 52, 63, 62]),
        ("spring_idle_east", &[53]),
        ("spring_idle_north", &[26]),
        ("spring_idle_south", &[58]),
        ("spring_shocked_end_south", &[54]),
        ("spring_shocked_loop_south", &[60]),
        ("spring_shocked_start_south", &[54]),
        ("spring_sit_east", &[50]),
        ("spring_sit_north", &[20]),
        ("spring_sit_south", &[62]),
        ("spring_walk_east", &[53, 48, 53, 46]),
        ("spring_walk_north", &[26, 14, 26, 14]),
        ("spring_walk_south", &[58, 49, 58, 49]),
        (
            "specialanimation_summer_net_east",
            &[54, 50, 51, 53, 53, 53, 54],
        ),
        ("specialanimation_summer_scratch_head_end_south", &[65]),
        ("specialanimation_summer_scratch_head_loop_south", &[63, 64]),
        ("specialanimation_summer_scratch_head_start_south", &[65]),
        ("summer_action_east", &[43, 50, 51, 50, 51, 43, 53]),
        ("summer_action_north", &[29, 25, 27, 25, 27, 29, 30]),
        ("summer_action_south", &[54, 53, 55, 53, 55, 54, 58]),
        ("summer_blink_east", &[53, 53, 53]),
        ("summer_blink_south", &[58, 58, 58]),
        ("summer_drink_east", &[56, 54, 56]),
        ("summer_drink_north", &[27, 23, 27]),
        ("summer_drink_south", &[61, 62, 61]),
        ("summer_eat_east", &[51, 56, 43, 48, 48]),
        ("summer_eat_north", &[27, 23, 27]),
        ("summer_eat_south", &[63, 61, 57, 62, 62]),
        ("summer_idle_east", &[53]),
        ("summer_idle_north", &[30]),
        ("summer_idle_south", &[58]),
        ("summer_sit_east", &[50]),
        ("summer_sit_north", &[24]),
        ("summer_sit_south", &[62]),
        ("summer_walk_east", &[53, 52, 53, 50]),
        ("summer_walk_north", &[30, 20, 30, 20]),
        ("summer_walk_south", &[58, 51, 58, 51]),
        (
            "specialanimation_winter_net_east",
            &[37, 26, 27, 33, 33, 33, 33],
        ),
        ("specialanimation_winter_scratch_head_end_south", &[40]),
        ("specialanimation_winter_scratch_head_loop_south", &[40, 40]),
        ("specialanimation_winter_scratch_head_start_south", &[40]),
        ("winter_action_east", &[35, 35, 35, 35, 35, 35, 35]),
        ("winter_action_north", &[12, 12, 12, 12, 12, 12, 12]),
        ("winter_action_south", &[40, 40, 40, 40, 40, 40, 40]),
        ("winter_blink_east", &[35, 35, 35]),
        ("winter_blink_south", &[40, 40, 40]),
        ("winter_drink_east", &[35, 30, 35]),
        ("winter_drink_north", &[16, 8, 16]),
        ("winter_drink_south", &[40, 38, 40]),
        ("winter_eat_east", &[35, 31, 23, 35, 35]),
        ("winter_eat_north", &[16, 8, 16]),
        ("winter_eat_south", &[40, 38, 33, 40, 40]),
        ("winter_idle_east", &[35]),
        ("winter_idle_north", &[12]),
        ("winter_idle_south", &[40]),
        ("winter_sit_east", &[35]),
        ("winter_sit_north", &[12]),
        ("winter_sit_south", &[40]),
        ("winter_walk_east", &[35, 35, 35, 35]),
        ("winter_walk_north", &[12, 12, 12, 12]),
        ("winter_walk_south", &[40, 40, 40, 40]),
    ];
    fn rgb(n: u32) -> [u8; 4] {
        [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255]
    }
    let skin = [0xC48E68, 0xB36844, 0x8E4538, 0x712922].map(rgb);
    let choices = [
        ("blue", [0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83]),
        ("npc_hayden", [0xE8B271, 0xCA9052, 0xB27146, 0x6E4922]),
        ("npc_ryis", [0xB06C57, 0x814A3A, 0x63342A, 0x491F1B]),
        ("npc_seridia", [0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48]),
    ];
    let mut selected = 0;
    for (name, counts) in cases {
        let season = SEASONS
            .iter()
            .find(|s| name.contains(&format!("{}_", s.to_lowercase())))
            .unwrap();
        let asset = source_asset(name);
        let before = image::open(original.join(&asset)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (80 * counts.len() as u32, 80));
        let metadata: toml::Value = toml::from_str(
            &fs::read_to_string(original.join(asset.replace(".png", ".meta.toml"))).unwrap(),
        )
        .unwrap();
        let properties = &metadata["asset_properties"];
        let (_, frames, duration) = inventory()
            .into_iter()
            .find(|(stem, _, _)| stem == name)
            .unwrap();
        assert_eq!(frames as usize, counts.len());
        assert_eq!(
            properties["frame_size"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_integer().unwrap())
                .collect::<Vec<_>>(),
            [80, 80]
        );
        assert_eq!(
            properties
                .get("frame_len")
                .and_then(toml::Value::as_integer)
                .unwrap_or(1),
            i64::from(frames)
        );
        assert_eq!(properties["atlas"].as_str(), Some("Default"));
        assert_eq!(properties["offset"]["horizontal"].as_float(), Some(40.0));
        assert_eq!(properties["offset"]["vertical"].as_float(), Some(54.0));
        if frames > 1 {
            let expected: toml::Value = toml::from_str(&format!("duration={duration}")).unwrap();
            assert_eq!(
                properties["duration"], expected["duration"],
                "native timing {name}"
            );
        } else {
            assert!(properties.get("duration").is_none());
        }
        let mut observed = vec![0; counts.len()];
        for (x, y, p) in before.enumerate_pixels() {
            if skin.contains(&p.0) {
                assert!(
                    (30..=51).contains(&(x % 80)) && (30..=54).contains(&y),
                    "unexpected skin location {name} {x},{y}"
                );
                observed[x as usize / 80] += 1;
            }
        }
        assert_eq!(observed, counts, "{name}");
        selected += counts.iter().sum::<usize>();
        for (choice, target) in choices {
            let target = target.map(rgb);
            let after = image::open(output.join("variants").join(choice).join(&asset))
                .unwrap()
                .to_rgba8();
            assert_eq!(before.dimensions(), after.dimensions());
            // Literal anatomy and neighboring materials are independent of component seeds.
            let mut landmarks: Vec<(u32, u32, u32, u32, Option<usize>)> = Vec::new();
            if name.ends_with("idle_south") {
                landmarks.extend([
                    (0, 39, 36, 0xC48E68, Some(0)),
                    (0, 39, 40, 0xB36844, Some(1)),
                    (0, 38, 42, 0x8E4538, Some(2)),
                    (0, 37, 42, 0x712922, Some(3)),
                    (0, 37, 34, 0x513C2E, None),
                    (0, 37, 37, 0x513C2E, None),
                    (0, 36, 39, 0xECF0E9, None),
                    (0, 36, 38, 0xEABF67, None),
                    (0, 38, 39, 0x000000, None),
                ]);
                match *season {
                    "Autumn" => landmarks.extend([
                        (0, 34, 47, 0xC48E68, Some(0)),
                        (0, 34, 46, 0xB6612E, None),
                        (0, 39, 44, 0xAB3558, None),
                        (0, 39, 49, 0x000000, None),
                    ]),
                    "Spring" => landmarks
                        .extend([(0, 34, 47, 0xB36844, Some(1)), (0, 39, 44, 0xA7F27D, None)]),
                    "Summer" => landmarks.extend([
                        (0, 34, 47, 0xB36844, Some(1)),
                        (0, 35, 47, 0xC48E68, Some(0)),
                        (0, 39, 44, 0xECF0E9, None),
                        (0, 39, 47, 0xF5F2EB, None),
                    ]),
                    "Winter" => {
                        landmarks.extend([(0, 34, 47, 0x862E26, None), (0, 39, 44, 0x3B6762, None)])
                    }
                    _ => unreachable!(),
                }
            }
            if name.ends_with("eat_east") {
                landmarks.extend([
                    (2, 36, 35, 0xC48E68, Some(0)),
                    (2, 37, 36, 0x8E4538, Some(2)),
                    (2, 39, 38, 0xB36844, Some(1)),
                    (2, 39, 39, 0x712922, Some(3)),
                    (2, 40, 35, 0x410808, None),
                    (2, 40, 36, 0x9E2626, None),
                    (2, 37, 35, 0xECF0E9, None),
                ]);
            }
            if name.ends_with("net_east") {
                landmarks.extend([
                    (1, 42, 35, 0xC48E68, Some(0)),
                    (1, 44, 35, 0xB36844, Some(1)),
                    (1, 45, 35, 0x8E4538, Some(2)),
                    (1, 46, 40, 0x712922, Some(3)),
                    (1, 33, 34, 0x469AB5, None),
                    (1, 34, 34, 0x77C9DB, None),
                    (1, 36, 36, 0xA54235, None),
                    (1, 39, 38, 0xECF0E9, None),
                ]);
            }
            if name.ends_with("scratch_head_loop_south") {
                landmarks.extend([
                    (1, 39, 35, 0xC48E68, Some(0)),
                    (1, 36, 38, 0xECF0E9, None),
                    (1, 36, 37, 0xEABF67, None),
                ]);
            }
            if name.ends_with("scratch_head_loop_south") {
                landmarks.push(if *season == "Winter" {
                    (1, 34, 41, 0xB13A31, None)
                } else {
                    (1, 34, 41, 0x8E4538, Some(2))
                });
            }
            if name == "spring_shocked_loop_south" {
                landmarks.extend([
                    (0, 33, 38, 0xC48E68, Some(0)),
                    (0, 45, 38, 0x8E4538, Some(2)),
                    (0, 39, 38, 0x410808, None),
                    (0, 39, 39, 0x9E2626, None),
                    (0, 36, 36, 0xECF0E9, None),
                ]);
            }
            for (frame, x, y, color, shade) in landmarks {
                let x = frame * 80 + x;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    rgb(color),
                    "source landmark {name} {x},{y}"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    shade.map_or(rgb(color), |s| target[s]),
                    "material landmark {name} {choice} {x},{y}"
                );
            }

            for (x, y, p) in before.enumerate_pixels() {
                let expected = skin
                    .iter()
                    .position(|c| c == &p.0)
                    .map_or(p.0, |i| target[i]);
                assert_eq!(after.get_pixel(x, y).0, expected, "{name} {choice} {x},{y}");
            }
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(output.join("variants").join(choice).join(&meta)).unwrap()
            );
        }
    }
    assert_eq!(selected, 12762);
    let portraits = read_json(root.join("palettes/profiles/luc-portraits.json"));
    let prior = temp.path().join("prior");
    fs::create_dir(&prior).unwrap();
    for region in portraits["regions"].as_array().unwrap() {
        let asset = region["asset"].as_str().unwrap();
        for asset in [asset.to_string(), asset.replace(".png", ".meta.toml")] {
            let to = prior.join(&asset);
            fs::create_dir_all(to.parent().unwrap()).unwrap();
            fs::copy(original.join(&asset), to).unwrap();
        }
    }
    let prior_output = temp.path().join("prior-bundle");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["build-presets", "--original"])
        .arg(prior)
        .arg("--presets")
        .arg(root.join("palettes/sets/luc-portraits-trial.json"))
        .arg("--output")
        .arg(&prior_output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for (choice, _) in choices {
        for region in portraits["regions"].as_array().unwrap() {
            let asset = region["asset"].as_str().unwrap();
            for asset in [asset.to_string(), asset.replace(".png", ".meta.toml")] {
                let path = Path::new("variants").join(choice).join(asset);
                assert_eq!(
                    fs::read(output.join(&path)).unwrap(),
                    fs::read(prior_output.join(path)).unwrap()
                );
            }
        }
    }
}
