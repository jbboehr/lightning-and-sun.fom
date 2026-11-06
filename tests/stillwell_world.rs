use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, process::Command};

const SEASONS: [&str; 4] = ["Autumn", "Spring", "Summer", "Winter"];
const ACTIONS: [(&str, u32, &str); 17] = [
    ("action_east", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("action_north", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("action_south", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("blink_east", 3, "[0.075,0.125,0.075]"),
    ("blink_south", 3, "[0.075,0.125,0.075]"),
    ("eat_east", 5, "[0.125,0.15,0.175,0.125,0.6]"),
    ("eat_north", 3, "1"),
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

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn world_profile_retains_every_portrait_region_and_palette_role() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let before = read_json(root.join("palettes/profiles/stillwell-portraits.json"));
    let after = read_json(root.join("palettes/profiles/stillwell-world-trial.json"));
    assert_eq!(
        &after["source_colors"].as_array().unwrap()[..6],
        before["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &after["color_groups"].as_array().unwrap()[..6],
        before["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        &after["source_colors"].as_array().unwrap()[6..],
        &["#E7CE9F", "#CAA17E", "#A06955", "#5A2F2C"]
    );
    for (group, color) in after["color_groups"].as_array().unwrap()[6..]
        .iter()
        .zip(["#E7CE9F", "#CAA17E", "#A06955", "#5A2F2C"])
    {
        assert_eq!(group.as_array().unwrap(), &[color]);
    }
    let regions = after["regions"].as_array().unwrap();
    assert_eq!(&regions[..36], before["regions"].as_array().unwrap());
    assert_eq!(regions.len(), 104);
    let expected: BTreeSet<_> = SEASONS
        .iter()
        .flat_map(|season| {
            ACTIONS.iter().map(move |(action, _, _)| {
                format!(
                    "assets/animations/NPCs/Stillwell/Sprites/{season}/spr_npc_stillwell_{}_{action}.png",
                    season.to_lowercase()
                )
            })
        })
        .collect();
    let actual: BTreeSet<_> = regions[36..]
        .iter()
        .map(|r| r["asset"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(actual, expected);
    let old_set = read_json(root.join("palettes/sets/stillwell-portraits-trial.json"));
    let new_set = read_json(root.join("palettes/sets/stillwell-world-trial.json"));
    assert_eq!(new_set["presets"].as_array().unwrap().len(), 4);
    for (old, new) in old_set["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(new_set["presets"].as_array().unwrap())
    {
        assert_eq!(old["id"], new["id"]);
        assert_eq!(old["label"], new["label"]);
        assert_eq!(
            old["colors"].as_array().unwrap(),
            &new["colors"].as_array().unwrap()[..6]
        );
        for role in 0..4 {
            assert_eq!(new["colors"][6 + role], old["colors"][role]);
        }
    }
    assert!(
        regions[36..]
            .iter()
            .all(|r| !r["seeds"].as_array().unwrap().is_empty())
    );
}

#[test]
fn all_four_season_packages_preserve_native_frames_timing_offsets_and_atlas() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();
    for season in SEASONS {
        for (action, frames, duration) in ACTIONS {
            let stem = format!("spr_npc_stillwell_{}_{action}", season.to_lowercase());
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
    assert_eq!(table[0][0], "stillwell");
    assert_eq!(table[0][4].as_array().unwrap().len(), 68);
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
    assert_eq!(frames, 232);
}

#[test]
#[ignore = "requires the complete Stillwell source corpus; checks all world pixels and retained portraits"]
fn every_world_pixel_matches_reviewed_materials_and_portraits_remain_identical() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/stillwell");
    let set = std::env::var_os("FOM_STILLWELL_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/stillwell-world-trial.json"));
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
    let cases: [(&str, &[usize]); 68] = [
        ("autumn_action_east", &[34, 32, 33, 32, 33, 34, 35]),
        ("autumn_action_north", &[5, 6, 6, 6, 6, 6, 12]),
        ("autumn_action_south", &[39, 37, 37, 37, 37, 39, 41]),
        ("autumn_blink_east", &[36, 39, 36]),
        ("autumn_blink_south", &[42, 45, 42]),
        ("autumn_eat_east", &[28, 32, 28, 29, 29]),
        ("autumn_eat_north", &[6, 5, 6]),
        ("autumn_eat_south", &[40, 39, 36, 47, 39]),
        ("autumn_idle_east", &[35]),
        ("autumn_idle_north", &[12]),
        ("autumn_idle_south", &[41]),
        ("autumn_sit_east", &[33]),
        ("autumn_sit_north", &[10]),
        ("autumn_sit_south", &[39]),
        ("autumn_walk_east", &[35, 40, 35, 34]),
        ("autumn_walk_north", &[12, 10, 12, 10]),
        ("autumn_walk_south", &[41, 38, 41, 38]),
        ("spring_action_east", &[37, 36, 37, 36, 37, 37, 39]),
        ("spring_action_north", &[5, 6, 6, 6, 6, 6, 12]),
        ("spring_action_south", &[42, 41, 41, 41, 41, 42, 45]),
        ("spring_blink_east", &[40, 43, 40]),
        ("spring_blink_south", &[46, 49, 46]),
        ("spring_eat_east", &[32, 36, 32, 32, 31]),
        ("spring_eat_north", &[6, 5, 6]),
        ("spring_eat_south", &[42, 40, 38, 49, 41]),
        ("spring_idle_east", &[39]),
        ("spring_idle_north", &[12]),
        ("spring_idle_south", &[45]),
        ("spring_sit_east", &[37]),
        ("spring_sit_north", &[10]),
        ("spring_sit_south", &[41]),
        ("spring_walk_east", &[39, 42, 39, 38]),
        ("spring_walk_north", &[12, 10, 12, 10]),
        ("spring_walk_south", &[45, 41, 45, 41]),
        ("summer_action_east", &[35, 34, 32, 34, 32, 35, 39]),
        ("summer_action_north", &[7, 8, 8, 8, 8, 9, 16]),
        ("summer_action_south", &[42, 41, 39, 41, 39, 42, 47]),
        ("summer_blink_east", &[40, 43, 40]),
        ("summer_blink_south", &[48, 51, 48]),
        ("summer_eat_east", &[30, 33, 33, 32, 34]),
        ("summer_eat_north", &[9, 6, 9]),
        ("summer_eat_south", &[43, 45, 38, 48, 43]),
        ("summer_idle_east", &[39]),
        ("summer_idle_north", &[16]),
        ("summer_idle_south", &[47]),
        ("summer_sit_east", &[35]),
        ("summer_sit_north", &[12]),
        ("summer_sit_south", &[43]),
        ("summer_walk_east", &[39, 45, 39, 37]),
        ("summer_walk_north", &[16, 13, 16, 13]),
        ("summer_walk_south", &[47, 43, 47, 43]),
        ("winter_action_east", &[34, 32, 33, 32, 33, 34, 35]),
        ("winter_action_north", &[5, 6, 6, 6, 6, 6, 12]),
        ("winter_action_south", &[39, 37, 37, 37, 37, 39, 41]),
        ("winter_blink_east", &[36, 39, 36]),
        ("winter_blink_south", &[42, 45, 42]),
        ("winter_eat_east", &[28, 32, 28, 29, 29]),
        ("winter_eat_north", &[6, 5, 6]),
        ("winter_eat_south", &[40, 39, 36, 47, 39]),
        ("winter_idle_east", &[35]),
        ("winter_idle_north", &[12]),
        ("winter_idle_south", &[41]),
        ("winter_sit_east", &[33]),
        ("winter_sit_north", &[10]),
        ("winter_sit_south", &[39]),
        ("winter_walk_east", &[35, 40, 35, 34]),
        ("winter_walk_north", &[12, 10, 12, 10]),
        ("winter_walk_south", &[41, 38, 41, 38]),
    ];
    let rgb = |n: u32| [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255];
    let skin = [0xE7CE9F, 0xCAA17E, 0xA06955, 0x5A2F2C].map(rgb);
    let choices = [
        ("blue", [0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83]),
        ("npc_hayden", [0xE8B271, 0xCA9052, 0xB27146, 0x6E4922]),
        ("npc_ryis", [0xB06C57, 0x814A3A, 0x63342A, 0x491F1B]),
        ("npc_seridia", [0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48]),
    ];
    let mut selected = 0;
    for (name, counts) in cases {
        let season = name.split('_').next().unwrap();
        let season = SEASONS.iter().find(|s| s.to_lowercase() == season).unwrap();
        let asset = format!(
            "assets/animations/NPCs/Stillwell/Sprites/{season}/spr_npc_stillwell_{name}.png"
        );
        let before = image::open(original.join(&asset)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (80 * counts.len() as u32, 80));
        let mut observed = vec![0; counts.len()];
        for (x, y, p) in before.enumerate_pixels() {
            if skin.contains(&p.0) {
                assert!(
                    (31..=49).contains(&(x % 80)) && (30..=51).contains(&y),
                    "unexpected source skin location {name} {x},{y}"
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
            for (x, y, p) in before.enumerate_pixels() {
                let expected = skin
                    .iter()
                    .position(|c| c == &p.0)
                    .map_or(p.0, |i| target[i]);
                assert_eq!(after.get_pixel(x, y).0, expected, "{name} {choice} {x},{y}");
            }
            let landmarks: &[(u32, u32, u32, u32, bool)] = if name.ends_with("idle_south") {
                &[
                    (0, 36, 33, 0x0F1A26, false),
                    (0, 43, 29, 0x182B3E, false),
                    (0, 37, 35, 0xC2B9BE, false),
                    (0, 39, 43, 0xCFD9E9, false),
                    (0, 39, 37, 0xE7CE9F, true),
                    (0, 39, 36, 0xCAA17E, true),
                    (0, 39, 39, 0xA06955, true),
                    (0, 33, 46, 0x5A2F2C, true),
                ]
            } else if name.ends_with("eat_south") {
                &[
                    (2, 39, 37, 0x9E2626, false),
                    (2, 39, 35, 0x410808, false),
                    (2, 39, 39, 0xCAA17E, true),
                    (2, 37, 40, 0xE7CE9F, true),
                ]
            } else if name.ends_with("eat_east") {
                &[
                    (2, 39, 37, 0x9E2626, false),
                    (2, 39, 36, 0x410808, false),
                    (2, 39, 39, 0x5A2F2C, true),
                ]
            } else {
                &[]
            };
            for &(frame, x, y, color, selected) in landmarks {
                let x = frame * 80 + x;
                let p = rgb(color);
                assert_eq!(
                    before.get_pixel(x, y).0,
                    p,
                    "source material landmark {name} {x},{y}"
                );
                let expected = if selected {
                    target[skin.iter().position(|c| c == &p).unwrap()]
                } else {
                    p
                };
                assert_eq!(
                    after.get_pixel(x, y).0,
                    expected,
                    "material landmark {name} {choice} {x},{y}"
                );
            }
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(output.join("variants").join(choice).join(&meta)).unwrap()
            );
        }
    }
    assert_eq!(selected, 7004);
    let portraits = read_json(root.join("palettes/profiles/stillwell-portraits.json"));
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
        .arg(root.join("palettes/sets/stillwell-portraits-trial.json"))
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
