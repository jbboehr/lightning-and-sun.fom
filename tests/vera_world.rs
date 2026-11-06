use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, process::Command};

const SEASONS: [&str; 4] = ["Autumn", "Spring", "Summer", "Winter"];
const ACTIONS: [(&str, u32, &str); 15] = [
    ("action_east", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("action_north", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("action_south", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("blink_east", 3, "[0.075,0.125,0.075]"),
    ("blink_south", 3, "[0.075,0.125,0.075]"),
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
    let before = read_json(root.join("palettes/profiles/vera-portraits.json"));
    let after = read_json(root.join("palettes/profiles/vera-world-trial.json"));
    assert_eq!(after["source_colors"], before["source_colors"]);
    assert_eq!(after["color_groups"], before["color_groups"]);
    let regions = after["regions"].as_array().unwrap();
    assert_eq!(&regions[..32], before["regions"].as_array().unwrap());
    assert_eq!(regions.len(), 92);
    let expected: BTreeSet<_> = SEASONS
        .iter()
        .flat_map(|season| {
            ACTIONS.iter().map(move |(action, _, _)| {
                format!(
                    "assets/animations/NPCs/Vera/Sprites/{season}/spr_npc_vera_{}_{action}.png",
                    season.to_lowercase()
                )
            })
        })
        .collect();
    let actual: BTreeSet<_> = regions[32..]
        .iter()
        .map(|r| r["asset"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(actual, expected);
    let old_set = read_json(root.join("palettes/sets/vera-portraits-trial.json"));
    let new_set = read_json(root.join("palettes/sets/vera-world-trial.json"));
    assert_eq!(new_set["presets"], old_set["presets"]);
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
    for season in SEASONS {
        for (action, frames, duration) in ACTIONS {
            let stem = format!("spr_npc_vera_{}_{action}", season.to_lowercase());
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
    assert_eq!(table[0][0], "vera");
    assert_eq!(table[0][4].as_array().unwrap().len(), 60);
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
    assert_eq!(frames, 200);
}

#[test]
#[ignore = "requires the complete Vera source corpus; checks all world pixels and retained portraits"]
fn every_world_pixel_matches_reviewed_materials_and_portraits_remain_identical() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/vera");
    let set = std::env::var_os("FOM_VERA_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/vera-world-trial.json"));
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
    let cases: [(&str, &[usize]); 60] = [
        ("autumn_action_east", &[55, 52, 51, 52, 51, 55, 59]),
        ("autumn_action_north", &[35, 33, 33, 33, 33, 37, 43]),
        ("autumn_action_south", &[65, 63, 64, 63, 64, 65, 69]),
        ("autumn_blink_east", &[62, 71, 62]),
        ("autumn_blink_south", &[72, 80, 72]),
        ("autumn_eat_south", &[66, 71, 67, 75, 65]),
        ("autumn_idle_east", &[59]),
        ("autumn_idle_north", &[43]),
        ("autumn_idle_south", &[69]),
        ("autumn_sit_east", &[54]),
        ("autumn_sit_north", &[35]),
        ("autumn_sit_south", &[65]),
        ("autumn_walk_east", &[59, 63, 59, 56]),
        ("autumn_walk_north", &[43, 37, 43, 37]),
        ("autumn_walk_south", &[69, 65, 69, 65]),
        ("spring_action_east", &[57, 57, 56, 57, 56, 57, 64]),
        ("spring_action_north", &[39, 37, 37, 37, 37, 41, 47]),
        ("spring_action_south", &[65, 67, 68, 67, 68, 65, 73]),
        ("spring_blink_east", &[67, 76, 67]),
        ("spring_blink_south", &[76, 84, 76]),
        ("spring_eat_south", &[66, 70, 67, 75, 65]),
        ("spring_idle_east", &[63]),
        ("spring_idle_north", &[47]),
        ("spring_idle_south", &[73]),
        ("spring_sit_east", &[55]),
        ("spring_sit_north", &[35]),
        ("spring_sit_south", &[65]),
        ("spring_walk_east", &[63, 66, 63, 64]),
        ("spring_walk_north", &[47, 43, 47, 43]),
        ("spring_walk_south", &[73, 67, 73, 67]),
        ("summer_action_east", &[55, 61, 57, 61, 57, 55, 62]),
        ("summer_action_north", &[40, 35, 35, 35, 35, 41, 46]),
        ("summer_action_south", &[64, 65, 66, 65, 66, 64, 72]),
        ("summer_blink_east", &[65, 74, 65]),
        ("summer_blink_south", &[75, 83, 75]),
        ("summer_eat_south", &[62, 65, 64, 69, 61]),
        ("summer_idle_east", &[62]),
        ("summer_idle_north", &[46]),
        ("summer_idle_south", &[72]),
        ("summer_sit_east", &[52]),
        ("summer_sit_north", &[31]),
        ("summer_sit_south", &[61]),
        ("summer_walk_east", &[62, 68, 62, 61]),
        ("summer_walk_north", &[46, 40, 46, 41]),
        ("summer_walk_south", &[72, 66, 72, 65]),
        ("winter_action_east", &[44, 45, 44, 45, 44, 44, 52]),
        ("winter_action_north", &[25, 24, 24, 24, 24, 25, 31]),
        ("winter_action_south", &[53, 56, 57, 56, 57, 53, 59]),
        ("winter_blink_east", &[55, 64, 55]),
        ("winter_blink_south", &[62, 70, 62]),
        ("winter_eat_south", &[53, 60, 59, 66, 53]),
        ("winter_idle_east", &[52]),
        ("winter_idle_north", &[31]),
        ("winter_idle_south", &[59]),
        ("winter_sit_east", &[45]),
        ("winter_sit_north", &[25]),
        ("winter_sit_south", &[53]),
        ("winter_walk_east", &[52, 55, 52, 49]),
        ("winter_walk_north", &[31, 28, 31, 28]),
        ("winter_walk_south", &[59, 54, 59, 54]),
    ];
    let rgb = |n: u32| [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255];
    let skin = [0xB68454, 0xA15F3F, 0x7F402B, 0x541C1E].map(rgb);
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
        let asset = format!("assets/animations/NPCs/Vera/Sprites/{season}/spr_npc_vera_{name}.png");
        let before = image::open(original.join(&asset)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (80 * counts.len() as u32, 80));
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
            if name.ends_with("idle_south") {
                let mut landmarks = vec![
                    (39, 35, 0),
                    (38, 33, 1),
                    (38, 32, 2),
                    (35, 35, 3),
                    (32, 46, 0),
                ];
                landmarks.push(if *season == "Winter" {
                    (38, 51, 0)
                } else {
                    (37, 53, 0)
                });
                if *season == "Summer" {
                    landmarks.push((36, 43, 0));
                }
                if *season == "Winter" {
                    landmarks.push((37, 42, 0));
                }
                for (x, y, shade) in landmarks {
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        skin[shade],
                        "source skin {name} {x},{y}"
                    );
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        target[shade],
                        "missed skin {name} {x},{y}"
                    );
                }
                let mut protected =
                    vec![(40, 29, 0xB23673), (34, 40, 0xE9AF68), (36, 36, 0x331224)];
                match *season {
                    "Autumn" => protected.extend([
                        (39, 43, 0x683F88),
                        (39, 44, 0xDC8D84),
                        (39, 47, 0xC26C68),
                    ]),
                    "Spring" => protected.extend([(39, 43, 0xE4F7FF), (39, 44, 0x28285A)]),
                    "Summer" => protected.extend([
                        (39, 43, 0xFFD6BF),
                        (37, 42, 0xEF9F88),
                        (39, 42, 0xD77C6D),
                        (39, 44, 0x28285A),
                    ]),
                    "Winter" => protected.extend([
                        (39, 43, 0x758793),
                        (39, 44, 0x5F3931),
                        (37, 53, 0x7F5443),
                    ]),
                    _ => unreachable!(),
                }
                for (x, y, color) in protected {
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        rgb(color),
                        "source protected {name} {x},{y}"
                    );
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        rgb(color),
                        "changed protected {name} {x},{y}"
                    );
                }
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
    assert_eq!(selected, 11105);
    let portraits = read_json(root.join("palettes/profiles/vera-portraits.json"));
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
        .arg(root.join("palettes/sets/vera-portraits-trial.json"))
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
