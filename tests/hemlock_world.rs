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
            (
                "chop_north",
                20,
                "[0.15,0.3,0.075,0.05,0.075,0.075,0.075,0.05,0.075,0.075,0.075,0.05,0.075,0.075,0.075,0.05,0.075,0.075,0.1,0.8]",
            ),
            ("polish_end_east", 2, "[0.125,0.1]"),
            ("polish_loop_east", 4, "[0.1,0.125,0.1,0.225]"),
            ("polish_start_east", 3, "0.125"),
            ("write_end_south", 2, "[0.125,0.1]"),
            ("write_loop_south", 4, "[0.1,0.125,0.1,0.3]"),
            ("write_start_south", 2, "0.125"),
        ] {
            entries.push((format!("specialanimation_{lower}_{action}"), n, d));
        }
        if season != "Winter" {
            for (part, n) in [("end", 2), ("loop", 10), ("start", 2)] {
                entries.push((
                    format!("specialanimation_{lower}_lute_play_{part}_south"),
                    n,
                    "0.1",
                ));
            }
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
    format!("assets/animations/NPCs/Hemlock/Sprites/{season}/spr_npc_hemlock_{name}.png")
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn world_profile_retains_every_portrait_region_and_palette_role() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let before = read_json(root.join("palettes/profiles/hemlock-portraits.json"));
    let after = read_json(root.join("palettes/profiles/hemlock-world-trial.json"));
    assert_eq!(
        &after["source_colors"].as_array().unwrap()[..7],
        before["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &after["source_colors"].as_array().unwrap()[7..],
        &[serde_json::json!("#C48E68"), serde_json::json!("#8E4538")]
    );
    assert_eq!(
        &after["color_groups"].as_array().unwrap()[..7],
        before["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        &after["color_groups"].as_array().unwrap()[7..],
        &[
            serde_json::json!(["#C48E68"]),
            serde_json::json!(["#8E4538"])
        ]
    );
    let regions = after["regions"].as_array().unwrap();
    assert_eq!(&regions[..32], before["regions"].as_array().unwrap());
    assert_eq!(regions.len(), 152);
    let expected: BTreeSet<_> = inventory()
        .iter()
        .map(|(name, _, _)| source_asset(name))
        .collect();
    assert_eq!(expected.len(), 120);
    let actual: BTreeSet<_> = regions[32..]
        .iter()
        .map(|r| r["asset"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(actual, expected);
    let old_set = read_json(root.join("palettes/sets/hemlock-portraits-trial.json"));
    let new_set = read_json(root.join("palettes/sets/hemlock-world-trial.json"));
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
            &after["colors"].as_array().unwrap()[..7]
        );
        assert_eq!(after["colors"][7], before["colors"][0]);
        assert_eq!(after["colors"][8], before["colors"][2]);
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
        let stem = format!("spr_npc_hemlock_{name}");
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
    assert_eq!(table[0][0], "hemlock");
    assert_eq!(table[0][4].as_array().unwrap().len(), 120);
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
    assert_eq!(frames, 461);
}

#[test]
#[ignore = "requires the complete Hemlock source corpus; checks all world pixels and retained portraits"]
fn every_world_pixel_matches_reviewed_materials_and_portraits_remain_identical() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/hemlock");
    let set = std::env::var_os("FOM_HEMLOCK_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/hemlock-world-trial.json"));
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
    let cases: [(&str, &[usize]); 120] = [
        ("autumn_action_east", &[49, 46, 46, 46, 46, 49, 50]),
        ("autumn_action_north", &[21, 20, 20, 20, 20, 22, 28]),
        ("autumn_action_south", &[58, 56, 57, 56, 57, 58, 61]),
        ("autumn_blink_east", &[54, 60, 54]),
        ("autumn_blink_south", &[65, 71, 65]),
        ("autumn_drink_east", &[45, 52, 45]),
        ("autumn_drink_north", &[14, 12, 14]),
        ("autumn_drink_south", &[55, 63, 55]),
        ("autumn_eat_east", &[42, 46, 43, 46, 43]),
        ("autumn_eat_north", &[14, 12, 14]),
        ("autumn_eat_south", &[54, 54, 58, 68, 53]),
        ("autumn_idle_east", &[50]),
        ("autumn_idle_north", &[28]),
        ("autumn_idle_south", &[61]),
        ("autumn_sit_east", &[45]),
        ("autumn_sit_north", &[16]),
        ("autumn_sit_south", &[53]),
        ("autumn_walk_east", &[50, 55, 50, 50]),
        ("autumn_walk_north", &[28, 23, 28, 23]),
        ("autumn_walk_south", &[61, 58, 61, 58]),
        (
            "specialanimation_autumn_chop_north",
            &[
                20, 22, 12, 8, 8, 12, 12, 8, 8, 12, 12, 8, 8, 12, 12, 8, 8, 12, 20, 28,
            ],
        ),
        ("specialanimation_autumn_lute_play_end_south", &[37, 50]),
        (
            "specialanimation_autumn_lute_play_loop_south",
            &[70, 70, 70, 69, 69, 69, 70, 70, 70, 70],
        ),
        ("specialanimation_autumn_lute_play_start_south", &[50, 37]),
        ("specialanimation_autumn_polish_end_east", &[50, 60]),
        (
            "specialanimation_autumn_polish_loop_east",
            &[45, 45, 45, 45],
        ),
        ("specialanimation_autumn_polish_start_east", &[53, 45, 45]),
        ("specialanimation_autumn_write_end_south", &[47, 55]),
        (
            "specialanimation_autumn_write_loop_south",
            &[52, 52, 53, 53],
        ),
        ("specialanimation_autumn_write_start_south", &[55, 47]),
        (
            "specialanimation_spring_chop_north",
            &[
                27, 20, 12, 16, 18, 17, 12, 16, 18, 17, 12, 16, 18, 17, 12, 16, 18, 17, 27, 28,
            ],
        ),
        ("specialanimation_spring_lute_play_end_south", &[36, 49]),
        (
            "specialanimation_spring_lute_play_loop_south",
            &[70, 70, 70, 69, 69, 69, 70, 70, 70, 70],
        ),
        ("specialanimation_spring_lute_play_start_south", &[49, 36]),
        ("specialanimation_spring_polish_end_east", &[52, 61]),
        (
            "specialanimation_spring_polish_loop_east",
            &[44, 45, 43, 43],
        ),
        ("specialanimation_spring_polish_start_east", &[53, 43, 43]),
        ("specialanimation_spring_write_end_south", &[47, 55]),
        (
            "specialanimation_spring_write_loop_south",
            &[53, 53, 54, 54],
        ),
        ("specialanimation_spring_write_start_south", &[55, 47]),
        ("spring_action_east", &[50, 51, 50, 51, 50, 50, 51]),
        ("spring_action_north", &[24, 21, 21, 21, 21, 26, 28]),
        ("spring_action_south", &[60, 58, 58, 58, 58, 60, 61]),
        ("spring_blink_east", &[55, 61, 55]),
        ("spring_blink_south", &[65, 71, 65]),
        ("spring_drink_east", &[46, 53, 46]),
        ("spring_drink_north", &[20, 20, 20]),
        ("spring_drink_south", &[57, 65, 57]),
        ("spring_eat_east", &[44, 51, 47, 48, 47]),
        ("spring_eat_north", &[20, 20, 20]),
        ("spring_eat_south", &[59, 62, 60, 70, 57]),
        ("spring_idle_east", &[51]),
        ("spring_idle_north", &[28]),
        ("spring_idle_south", &[61]),
        ("spring_shocked_end_south", &[71]),
        ("spring_shocked_loop_south", &[77]),
        ("spring_shocked_start_south", &[71]),
        ("spring_sit_east", &[46]),
        ("spring_sit_north", &[18]),
        ("spring_sit_south", &[57]),
        ("spring_walk_east", &[51, 56, 51, 50]),
        ("spring_walk_north", &[28, 22, 28, 22]),
        ("spring_walk_south", &[61, 58, 61, 58]),
        (
            "specialanimation_summer_chop_north",
            &[
                36, 30, 21, 22, 22, 22, 21, 22, 22, 22, 21, 22, 22, 22, 21, 22, 22, 22, 36, 38,
            ],
        ),
        ("specialanimation_summer_lute_play_end_south", &[41, 54]),
        (
            "specialanimation_summer_lute_play_loop_south",
            &[74, 74, 74, 73, 73, 73, 74, 74, 74, 74],
        ),
        ("specialanimation_summer_lute_play_start_south", &[54, 41]),
        ("specialanimation_summer_polish_end_east", &[62, 71]),
        (
            "specialanimation_summer_polish_loop_east",
            &[55, 56, 54, 54],
        ),
        ("specialanimation_summer_polish_start_east", &[61, 51, 51]),
        ("specialanimation_summer_write_end_south", &[51, 60]),
        (
            "specialanimation_summer_write_loop_south",
            &[58, 58, 58, 58],
        ),
        ("specialanimation_summer_write_start_south", &[60, 51]),
        ("summer_action_east", &[58, 58, 56, 58, 56, 58, 61]),
        ("summer_action_north", &[35, 28, 29, 28, 29, 36, 38]),
        ("summer_action_south", &[67, 64, 66, 64, 66, 67, 71]),
        ("summer_blink_east", &[65, 71, 65]),
        ("summer_blink_south", &[75, 81, 75]),
        ("summer_drink_east", &[53, 61, 53]),
        ("summer_drink_north", &[24, 19, 24]),
        ("summer_drink_south", &[58, 66, 58]),
        ("summer_eat_east", &[50, 57, 53, 54, 52]),
        ("summer_eat_north", &[24, 19, 24]),
        ("summer_eat_south", &[61, 64, 61, 70, 59]),
        ("summer_idle_east", &[61]),
        ("summer_idle_north", &[38]),
        ("summer_idle_south", &[71]),
        ("summer_sit_east", &[52]),
        ("summer_sit_north", &[22]),
        ("summer_sit_south", &[59]),
        ("summer_walk_east", &[61, 64, 61, 53]),
        ("summer_walk_north", &[38, 32, 38, 32]),
        ("summer_walk_south", &[71, 65, 71, 65]),
        (
            "specialanimation_winter_chop_north",
            &[
                19, 19, 12, 11, 11, 15, 12, 11, 11, 15, 12, 11, 11, 15, 12, 11, 11, 15, 19, 24,
            ],
        ),
        ("specialanimation_winter_polish_end_east", &[49, 58]),
        (
            "specialanimation_winter_polish_loop_east",
            &[43, 43, 43, 43],
        ),
        ("specialanimation_winter_polish_start_east", &[49, 43, 43]),
        ("specialanimation_winter_write_end_south", &[45, 53]),
        (
            "specialanimation_winter_write_loop_south",
            &[49, 49, 50, 50],
        ),
        ("specialanimation_winter_write_start_south", &[53, 45]),
        ("winter_action_east", &[45, 48, 45, 48, 45, 45, 48]),
        ("winter_action_north", &[20, 19, 19, 19, 19, 20, 24]),
        ("winter_action_south", &[54, 53, 53, 53, 53, 54, 55]),
        ("winter_blink_east", &[52, 58, 52]),
        ("winter_blink_south", &[59, 65, 59]),
        ("winter_drink_east", &[45, 52, 45]),
        ("winter_drink_north", &[15, 13, 15]),
        ("winter_drink_south", &[53, 61, 53]),
        ("winter_eat_east", &[42, 46, 43, 46, 42]),
        ("winter_eat_north", &[15, 13, 15]),
        ("winter_eat_south", &[52, 52, 57, 66, 51]),
        ("winter_idle_east", &[48]),
        ("winter_idle_north", &[24]),
        ("winter_idle_south", &[55]),
        ("winter_sit_east", &[43]),
        ("winter_sit_north", &[18]),
        ("winter_sit_south", &[51]),
        ("winter_walk_east", &[48, 51, 48, 47]),
        ("winter_walk_north", &[24, 20, 24, 20]),
        ("winter_walk_south", &[55, 53, 55, 53]),
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
                    (30..=49).contains(&(x % 80)) && (30..=54).contains(&y),
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
            // These literal landmarks describe anatomy and adjacent materials independently of seeds.
            let mut landmarks: Vec<(u32, u32, u32, u32, Option<usize>)> = Vec::new();
            if name.ends_with("idle_south") {
                for (x, y, shade) in [
                    (39, 35, 0),
                    (39, 32, 1),
                    (38, 31, 2),
                    (35, 33, 3),
                    (32, 44, 0),
                    (38, 51, 0),
                    (39, 37, 1),
                    (39, 38, 2),
                ] {
                    let color = [0xC48E68, 0xB36844, 0x8E4538, 0x712922][shade];
                    landmarks.push((0, x, y, color, Some(shade)));
                }
                for (x, y, color) in [
                    (36, 32, 0x7C7672),
                    (41, 28, 0x554A44),
                    (37, 34, 0xC2B9BE),
                    (37, 35, 0xECF0E9),
                ] {
                    landmarks.push((0, x, y, color, None));
                }
                match *season {
                    "Autumn" => landmarks.extend([
                        (0, 39, 42, 0xCF9389, None),
                        (0, 37, 42, 0x5C4A4D, None),
                        (0, 37, 53, 0x957D7F, None),
                    ]),
                    "Spring" => landmarks.extend([
                        (0, 39, 40, 0xEA9866, None),
                        (0, 39, 42, 0x67323D, None),
                        (0, 37, 53, 0x92525F, None),
                    ]),
                    "Summer" => landmarks.extend([
                        (0, 35, 42, 0xB36844, Some(1)),
                        (0, 37, 53, 0xC48E68, Some(0)),
                        (0, 39, 42, 0xA87A68, None),
                        (0, 36, 42, 0xEBF5EC, None),
                    ]),
                    "Winter" => landmarks.extend([
                        (0, 36, 42, 0x4C242C, None),
                        (0, 33, 43, 0xFAC0B5, None),
                        (0, 39, 39, 0x2F0A1A, None),
                        (0, 37, 53, 0xCD5B68, None),
                    ]),
                    _ => unreachable!(),
                }
            }
            if name.ends_with("eat_east") {
                landmarks.extend([
                    (2, 38, 35, 0xC48E68, Some(0)),
                    (2, 38, 37, 0xB36844, Some(1)),
                    (2, 38, 38, 0x8E4538, Some(2)),
                    (2, 39, 39, 0x712922, Some(3)),
                    (2, 43, 35, 0xC48E68, Some(0)),
                    (2, 40, 35, 0x410808, None),
                    (2, 40, 36, 0x9E2626, None),
                ]);
            }
            if name == "specialanimation_autumn_chop_north" {
                landmarks.extend([
                    (0, 34, 40, 0xB36844, Some(1)),
                    (0, 46, 42, 0x712922, Some(3)),
                    (0, 47, 39, 0xA1BBC0, None),
                ]);
            }
            if name.ends_with("lute_play_loop_south") {
                landmarks.extend([
                    (1, 34, 39, 0xC48E68, Some(0)),
                    (1, 44, 42, 0xC48E68, Some(0)),
                    (1, 31, 34, 0xF4E3C0, None),
                    (1, 39, 35, 0x410808, None),
                    (1, 39, 36, 0x9E2626, None),
                ]);
            }
            if name.ends_with("polish_loop_east") {
                landmarks.extend([(1, 42, 42, 0x85CAEA, None), (1, 40, 44, 0xF5F5F5, None)]);
            }
            if name.ends_with("write_loop_south") {
                landmarks.extend([
                    (1, 36, 44, 0xC48E68, Some(0)),
                    (1, 42, 45, 0xC48E68, Some(0)),
                    (1, 37, 41, 0xD36A0E, None),
                    (1, 43, 42, 0xB28159, None),
                    (1, 38, 43, 0xFFD8D1, None),
                ]);
            }
            if name == "spring_shocked_loop_south" {
                landmarks.extend([
                    (0, 31, 34, 0xC48E68, Some(0)),
                    (0, 46, 34, 0xC48E68, Some(0)),
                    (0, 39, 35, 0x410808, None),
                    (0, 39, 36, 0x9E2626, None),
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
    assert_eq!(selected, 20129);
    let portraits = read_json(root.join("palettes/profiles/hemlock-portraits.json"));
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
        .arg(root.join("palettes/sets/hemlock-portraits-trial.json"))
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
