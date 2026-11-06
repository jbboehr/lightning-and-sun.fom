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
        for (part, n, d) in [("end", 1, ""), ("loop", 2, "0.2"), ("start", 1, "")] {
            entries.push((
                format!("specialanimation_{lower}_scratch_head_{part}_south"),
                n,
                d,
            ));
        }
        if ["Spring", "Summer"].contains(&season) {
            for (part, n, d) in [
                ("end", 1, ""),
                ("loop", 4, "[0.1,0.4,0.1,0.4]"),
                ("start", 1, ""),
            ] {
                entries.push((
                    format!("specialanimation_{lower}_play_doll_{part}_south"),
                    n,
                    d,
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
    format!("assets/animations/NPCs/Maple/Sprites/{season}/spr_npc_maple_{name}.png")
}

fn doll_face(name: &str, frame: u32, x: u32, y: u32) -> bool {
    if !name.contains("play_doll_") {
        return false;
    }
    let (left, top, right, bottom) = if name.contains("_loop_") {
        [
            (37, 44, 42, 47),
            (39, 43, 44, 46),
            (37, 44, 42, 47),
            (35, 43, 40, 46),
        ][frame as usize]
    } else {
        (35, 46, 38, 48)
    };
    (left..=right).contains(&x) && (top..=bottom).contains(&y)
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn world_profile_retains_every_portrait_region_and_palette_role() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let before = read_json(root.join("palettes/profiles/maple-portraits.json"));
    let after = read_json(root.join("palettes/profiles/maple-world-trial.json"));
    assert_eq!(
        &after["source_colors"].as_array().unwrap()[..10],
        before["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &after["source_colors"].as_array().unwrap()[10..],
        &[serde_json::json!("#96623B")]
    );
    assert_eq!(
        &after["color_groups"].as_array().unwrap()[..9],
        before["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        &after["color_groups"].as_array().unwrap()[9..],
        &[serde_json::json!(["#96623B"])]
    );
    let regions = after["regions"].as_array().unwrap();
    assert_eq!(&regions[..32], before["regions"].as_array().unwrap());
    assert_eq!(regions.len(), 133);
    let expected: BTreeSet<_> = inventory()
        .iter()
        .map(|(name, _, _)| source_asset(name))
        .collect();
    assert_eq!(expected.len(), 101);
    let actual: BTreeSet<_> = regions[32..]
        .iter()
        .map(|r| r["asset"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(actual, expected);
    let old_set = read_json(root.join("palettes/sets/maple-portraits-trial.json"));
    let new_set = read_json(root.join("palettes/sets/maple-world-trial.json"));
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
            &after["colors"].as_array().unwrap()[..10]
        );
        assert_eq!(after["colors"][10], before["colors"][0]);
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
        let stem = format!("spr_npc_maple_{name}");
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
    assert_eq!(table[0][0], "maple");
    assert_eq!(table[0][4].as_array().unwrap().len(), 101);
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
    assert_eq!(frames, 299);
}

#[test]
#[ignore = "requires the complete Maple source corpus; checks all world pixels and retained portraits"]
fn every_world_pixel_matches_reviewed_materials_and_portraits_remain_identical() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/maple");
    let set = std::env::var_os("FOM_MAPLE_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/maple-world-trial.json"));
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
    let cases: [(&str, &[usize]); 101] = [
        ("autumn_action_east", &[35, 35, 35, 35, 35, 35, 38]),
        ("autumn_action_north", &[6, 4, 4, 4, 4, 6, 10]),
        ("autumn_action_south", &[38, 37, 38, 37, 38, 38, 42]),
        ("autumn_blink_east", &[42, 50, 42]),
        ("autumn_blink_south", &[46, 54, 46]),
        ("autumn_drink_east", &[40, 48, 40]),
        ("autumn_drink_north", &[14, 6, 14]),
        ("autumn_drink_south", &[44, 58, 44]),
        ("autumn_eat_east", &[39, 48, 36, 48, 37]),
        ("autumn_eat_north", &[14, 6, 14]),
        ("autumn_eat_south", &[46, 53, 45, 59, 44]),
        ("autumn_idle_east", &[38]),
        ("autumn_idle_north", &[10]),
        ("autumn_idle_south", &[42]),
        ("autumn_sit_east", &[38]),
        ("autumn_sit_north", &[14]),
        ("autumn_sit_south", &[44]),
        ("autumn_walk_east", &[38, 39, 38, 36]),
        ("autumn_walk_north", &[10, 6, 10, 6]),
        ("autumn_walk_south", &[42, 37, 42, 37]),
        ("specialanimation_autumn_scratch_head_end_south", &[56]),
        ("specialanimation_autumn_scratch_head_loop_south", &[46, 44]),
        ("specialanimation_autumn_scratch_head_start_south", &[56]),
        ("specialanimation_spring_play_doll_end_south", &[49]),
        (
            "specialanimation_spring_play_doll_loop_south",
            &[36, 23, 36, 23],
        ),
        ("specialanimation_spring_play_doll_start_south", &[49]),
        ("specialanimation_spring_scratch_head_end_south", &[66]),
        ("specialanimation_spring_scratch_head_loop_south", &[54, 55]),
        ("specialanimation_spring_scratch_head_start_south", &[66]),
        ("spring_action_east", &[39, 42, 43, 42, 43, 39, 43]),
        ("spring_action_north", &[11, 12, 12, 12, 12, 11, 12]),
        ("spring_action_south", &[44, 47, 48, 47, 48, 44, 50]),
        ("spring_blink_east", &[47, 56, 47]),
        ("spring_blink_south", &[54, 62, 54]),
        ("spring_drink_east", &[48, 57, 48]),
        ("spring_drink_north", &[14, 9, 14]),
        ("spring_drink_south", &[55, 68, 55]),
        ("spring_eat_east", &[46, 55, 52, 52, 44]),
        ("spring_eat_north", &[14, 9, 14]),
        ("spring_eat_south", &[56, 64, 59, 69, 56]),
        ("spring_idle_east", &[43]),
        ("spring_idle_north", &[12]),
        ("spring_idle_south", &[50]),
        ("spring_shocked_end_south", &[58]),
        ("spring_shocked_loop_south", &[64]),
        ("spring_shocked_start_south", &[58]),
        ("spring_sit_east", &[45]),
        ("spring_sit_north", &[16]),
        ("spring_sit_south", &[56]),
        ("spring_walk_east", &[43, 42, 43, 37]),
        ("spring_walk_north", &[12, 9, 12, 9]),
        ("spring_walk_south", &[50, 44, 50, 44]),
        ("specialanimation_summer_play_doll_end_south", &[55]),
        (
            "specialanimation_summer_play_doll_loop_south",
            &[29, 29, 29, 31],
        ),
        ("specialanimation_summer_play_doll_start_south", &[55]),
        ("specialanimation_summer_scratch_head_end_south", &[73]),
        ("specialanimation_summer_scratch_head_loop_south", &[62, 63]),
        ("specialanimation_summer_scratch_head_start_south", &[73]),
        ("summer_action_east", &[41, 50, 50, 50, 50, 41, 52]),
        ("summer_action_north", &[21, 19, 20, 19, 20, 21, 23]),
        ("summer_action_south", &[49, 50, 52, 50, 52, 49, 58]),
        ("summer_blink_east", &[58, 68, 58]),
        ("summer_blink_south", &[62, 70, 62]),
        ("summer_drink_east", &[45, 54, 45]),
        ("summer_drink_north", &[19, 12, 19]),
        ("summer_drink_south", &[57, 70, 57]),
        ("summer_eat_east", &[45, 52, 46, 56, 45]),
        ("summer_eat_north", &[19, 12, 19]),
        ("summer_eat_south", &[59, 64, 53, 71, 58]),
        ("summer_idle_east", &[54]),
        ("summer_idle_north", &[23]),
        ("summer_idle_south", &[58]),
        ("summer_sit_east", &[40]),
        ("summer_sit_north", &[18]),
        ("summer_sit_south", &[58]),
        ("summer_walk_east", &[54, 46, 54, 40]),
        ("summer_walk_north", &[23, 17, 23, 15]),
        ("summer_walk_south", &[58, 48, 58, 49]),
        ("specialanimation_winter_scratch_head_end_south", &[51]),
        ("specialanimation_winter_scratch_head_loop_south", &[43, 43]),
        ("specialanimation_winter_scratch_head_start_south", &[51]),
        ("winter_action_east", &[32, 33, 33, 33, 33, 32, 35]),
        ("winter_action_north", &[2, 2, 2, 2, 2, 2, 4]),
        ("winter_action_south", &[38, 38, 39, 38, 39, 38, 40]),
        ("winter_blink_east", &[39, 48, 39]),
        ("winter_blink_south", &[44, 52, 44]),
        ("winter_drink_east", &[38, 44, 38]),
        ("winter_drink_north", &[6, 4, 6]),
        ("winter_drink_south", &[45, 54, 45]),
        ("winter_eat_east", &[34, 42, 38, 43, 34]),
        ("winter_eat_north", &[6, 4, 6]),
        ("winter_eat_south", &[45, 49, 49, 58, 44]),
        ("winter_idle_east", &[35]),
        ("winter_idle_north", &[4]),
        ("winter_idle_south", &[40]),
        ("winter_sit_east", &[36]),
        ("winter_sit_north", &[12]),
        ("winter_sit_south", &[44]),
        ("winter_walk_east", &[35, 37, 35, 33]),
        ("winter_walk_north", &[4, 4, 4, 4]),
        ("winter_walk_south", &[40, 37, 40, 37]),
    ];
    fn rgb(n: u32) -> [u8; 4] {
        [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255]
    }
    let skin = [0x96623B, 0x83462E, 0x713527, 0x57271D].map(rgb);
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
            if skin.contains(&p.0) && !doll_face(name, x / 80, x % 80, y) {
                assert!(
                    (32..=47).contains(&(x % 80)) && (30..=53).contains(&y),
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
            // Anatomical and adjacent-material landmarks remain independent of component seeds.
            let mut landmarks: Vec<(u32, u32, u32, u32, Option<usize>)> = Vec::new();
            if name.ends_with("idle_south") {
                for (x, y, shade) in [
                    (40, 36, 0),
                    (40, 35, 1),
                    (37, 41, 2),
                    (37, 42, 3),
                    (35, 47, 0),
                    (34, 46, 3),
                    (39, 43, 2),
                ] {
                    let color = [0x96623B, 0x83462E, 0x713527, 0x57271D][shade];
                    // Spring/Summer have slightly different hand cuffs.
                    if y < 46 || *season == "Autumn" {
                        landmarks.push((0, x, y, color, Some(shade)));
                    }
                }
                landmarks.extend([
                    (0, 38, 38, 0xC2B9BE, None),
                    (0, 37, 39, 0xECF0E9, None),
                    (0, 38, 40, 0x000000, None),
                ]);
                let (hair, cloth) = match *season {
                    "Autumn" => (0x5F3737, 0xE5C4C7),
                    "Spring" => (0xFDF5E0, 0xFDF5E0),
                    "Summer" => (0x5F3737, 0xFFF3DB),
                    "Winter" => (0xF78F95, 0xFFC19F),
                    _ => unreachable!(),
                };
                landmarks.extend([(0, 37, 33, hair, None), (0, 39, 44, cloth, None)]);
            }
            if name == "autumn_eat_east" {
                landmarks.extend([
                    (2, 41, 32, 0x96623B, Some(0)),
                    (2, 41, 31, 0x83462E, Some(1)),
                    (2, 44, 35, 0x713527, Some(2)),
                    (2, 44, 34, 0x57271D, Some(3)),
                    (2, 40, 35, 0x410808, None),
                    (2, 40, 36, 0xC83E37, None),
                ]);
            }
            if name.ends_with("scratch_head_loop_south") && *season == "Autumn" {
                landmarks.extend([
                    (0, 35, 43, 0x96623B, Some(0)),
                    (0, 34, 43, 0x83462E, Some(1)),
                    (0, 44, 46, 0x96623B, Some(0)),
                    (0, 45, 45, 0x57271D, Some(3)),
                ]);
            }
            if name.ends_with("play_doll_loop_south") {
                landmarks.extend([
                    (0, 39, 45, 0x96623B, None),
                    (0, 38, 44, 0x83462E, None),
                    (0, 36, 48, 0x96623B, Some(0)),
                    (0, 37, 49, 0x57271D, Some(3)),
                    (0, 39, 42, 0xF8E175, None),
                    (0, 38, 43, 0xE4B754, None),
                    (0, 39, 50, 0xEF728D, None),
                ]);
            }
            if name == "spring_shocked_loop_south" {
                landmarks.extend([
                    (0, 33, 38, 0x96623B, Some(0)),
                    (0, 34, 38, 0x713527, Some(2)),
                    (0, 46, 38, 0x96623B, Some(0)),
                    (0, 39, 39, 0xC83E37, None),
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
                    .filter(|_| !doll_face(name, x / 80, x % 80, y))
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
    assert_eq!(selected, 11204);
    let portraits = read_json(root.join("palettes/profiles/maple-portraits.json"));
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
        .arg(root.join("palettes/sets/maple-portraits-trial.json"))
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
