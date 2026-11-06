use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, process::Command};

const SEASONS: [&str; 4] = ["Autumn", "Spring", "Summer", "Winter"];
const ACTIONS: [(&str, u32, &str); 14] = [
    ("action_east", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("action_north", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("action_south", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("blink_east", 3, "[0.075,0.125,0.075]"),
    ("blink_south", 3, "[0.075,0.125,0.075]"),
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
    let before = read_json(root.join("palettes/profiles/wheedle-portraits.json"));
    let after = read_json(root.join("palettes/profiles/wheedle-world-trial.json"));
    assert_eq!(
        &after["source_colors"].as_array().unwrap()[..10],
        before["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &after["color_groups"].as_array().unwrap()[..10],
        before["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        &after["source_colors"].as_array().unwrap()[10..],
        &["#F6C897", "#DCA77C", "#C18561", "#7F523E"]
    );
    let regions = after["regions"].as_array().unwrap();
    assert_eq!(&regions[..32], before["regions"].as_array().unwrap());
    assert_eq!(regions.len(), 88);
    let expected: BTreeSet<_> = SEASONS
        .iter()
        .flat_map(|season| {
            ACTIONS.iter().map(move |(action, _, _)| {
                format!(
                    "assets/animations/NPCs/Wheedle/Sprites/{season}/spr_npc_wheedle_{}_{action}.png",
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
    let old_set = read_json(root.join("palettes/sets/wheedle-portraits-trial.json"));
    let new_set = read_json(root.join("palettes/sets/wheedle-world-trial.json"));
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
            &new["colors"].as_array().unwrap()[..10]
        );
        for (added, role) in [0, 1, 2, 4].into_iter().enumerate() {
            assert_eq!(new["colors"][10 + added], old["colors"][role]);
        }
    }
    assert!(
        regions[32..]
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
            let stem = format!("spr_npc_wheedle_{}_{action}", season.to_lowercase());
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
    assert_eq!(table[0][0], "wheedle");
    assert_eq!(table[0][4].as_array().unwrap().len(), 56);
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
    assert_eq!(frames, 180);
}

fn hand_shadow(action: &str, frame: u32, x: u32, y: u32) -> bool {
    match action {
        "action_east" => match frame {
            0 | 5 => (x, y) == (40, 47),
            1 | 3 => (x, y) == (48, 44),
            2 | 4 => (x, y) == (45, 45),
            6 => (x, y) == (35, 46),
            _ => false,
        },
        "action_north" => match frame {
            0 => [(45, 40), (33, 44)].contains(&(x, y)),
            1..=4 => (x, y) == (35, 45),
            5 => [(45, 40), (34, 46)].contains(&(x, y)),
            6 => [(33, 46), (46, 46)].contains(&(x, y)),
            _ => false,
        },
        "action_south" => match frame {
            0 | 5 => [(46, 46), (34, 47)].contains(&(x, y)),
            1 | 3 => [(45, 45), (35, 46)].contains(&(x, y)),
            2 | 4 => [(45, 45), (38, 46)].contains(&(x, y)),
            6 => [(33, 46), (46, 46)].contains(&(x, y)),
            _ => false,
        },
        "blink_east" | "idle_east" => (x, y) == (35, 46),
        "blink_south" | "idle_north" | "idle_south" => [(33, 46), (46, 46)].contains(&(x, y)),
        "sit_east" => (x, y) == (35, 47),
        "sit_north" | "sit_south" => [(34, 47), (45, 47)].contains(&(x, y)),
        "walk_east" => match frame {
            0 | 2 => (x, y) == (35, 46),
            1 => [(33, 46), (46, 46)].contains(&(x, y)),
            3 => [(44, 46), (36, 47)].contains(&(x, y)),
            _ => false,
        },
        "walk_north" | "walk_south" => match frame {
            0 | 2 => [(33, 46), (46, 46)].contains(&(x, y)),
            1 => [(46, 46), (34, 47)].contains(&(x, y)),
            3 => [(33, 46), (45, 47)].contains(&(x, y)),
            _ => false,
        },
        _ => false,
    }
}

#[test]
#[ignore = "requires the complete Wheedle source corpus; checks all world pixels and retained portraits"]
fn every_world_pixel_matches_reviewed_materials_and_portraits_remain_identical() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/wheedle");
    let set = std::env::var_os("FOM_WHEEDLE_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/wheedle-world-trial.json"));
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
    let cases: [(&str, &[usize]); 56] = [
        ("autumn_action_east", &[25, 27, 25, 27, 25, 25, 28]),
        ("autumn_action_north", &[12, 10, 10, 10, 10, 12, 16]),
        ("autumn_action_south", &[31, 30, 30, 30, 30, 31, 33]),
        ("autumn_blink_east", &[32, 36, 32]),
        ("autumn_blink_south", &[37, 41, 37]),
        ("autumn_idle_east", &[28]),
        ("autumn_idle_north", &[16]),
        ("autumn_idle_south", &[33]),
        ("autumn_sit_east", &[24]),
        ("autumn_sit_north", &[14]),
        ("autumn_sit_south", &[31]),
        ("autumn_walk_east", &[28, 31, 28, 27]),
        ("autumn_walk_north", &[16, 14, 16, 14]),
        ("autumn_walk_south", &[33, 31, 33, 31]),
        ("spring_action_east", &[25, 27, 25, 27, 25, 25, 25]),
        ("spring_action_north", &[11, 9, 9, 9, 9, 11, 14]),
        ("spring_action_south", &[29, 28, 28, 28, 28, 29, 31]),
        ("spring_blink_east", &[30, 34, 30]),
        ("spring_blink_south", &[35, 39, 35]),
        ("spring_idle_east", &[25]),
        ("spring_idle_north", &[14]),
        ("spring_idle_south", &[31]),
        ("spring_sit_east", &[24]),
        ("spring_sit_north", &[14]),
        ("spring_sit_south", &[31]),
        ("spring_walk_east", &[25, 29, 25, 25]),
        ("spring_walk_north", &[14, 12, 14, 11]),
        ("spring_walk_south", &[31, 29, 31, 30]),
        ("summer_action_east", &[29, 30, 32, 30, 32, 29, 34]),
        ("summer_action_north", &[16, 14, 14, 14, 14, 14, 20]),
        ("summer_action_south", &[35, 38, 38, 38, 38, 35, 41]),
        ("summer_blink_east", &[38, 42, 38]),
        ("summer_blink_south", &[45, 49, 45]),
        ("summer_idle_east", &[34]),
        ("summer_idle_north", &[20]),
        ("summer_idle_south", &[41]),
        ("summer_sit_east", &[28]),
        ("summer_sit_north", &[14]),
        ("summer_sit_south", &[35]),
        ("summer_walk_east", &[34, 37, 34, 33]),
        ("summer_walk_north", &[20, 16, 20, 16]),
        ("summer_walk_south", &[41, 37, 41, 37]),
        ("winter_action_east", &[19, 19, 19, 19, 19, 19, 19]),
        ("winter_action_north", &[4, 4, 4, 4, 4, 4, 4]),
        ("winter_action_south", &[21, 21, 21, 21, 21, 21, 21]),
        ("winter_blink_east", &[23, 27, 23]),
        ("winter_blink_south", &[25, 29, 25]),
        ("winter_idle_east", &[19]),
        ("winter_idle_north", &[4]),
        ("winter_idle_south", &[21]),
        ("winter_sit_east", &[19]),
        ("winter_sit_north", &[4]),
        ("winter_sit_south", &[21]),
        ("winter_walk_east", &[19, 19, 19, 19]),
        ("winter_walk_north", &[4, 4, 4, 4]),
        ("winter_walk_south", &[21, 21, 21, 21]),
    ];
    let rgb = |n: u32| [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255];
    let skin = [0xF6C897, 0xDCA77C, 0xC18561, 0x7F523E].map(rgb);
    let choices = [
        ("blue", [0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83]),
        ("npc_hayden", [0xE8B271, 0xCA9052, 0xB27146, 0x6E4922]),
        ("npc_ryis", [0xB06C57, 0x814A3A, 0x63342A, 0x491F1B]),
        ("npc_seridia", [0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48]),
    ];
    let mut selected = 0;
    let mut protected = 0;
    for (name, counts) in cases {
        let season = name.split('_').next().unwrap();
        let season = SEASONS.iter().find(|s| s.to_lowercase() == season).unwrap();
        let asset =
            format!("assets/animations/NPCs/Wheedle/Sprites/{season}/spr_npc_wheedle_{name}.png");
        let before = image::open(original.join(&asset)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (80 * counts.len() as u32, 80));
        let action = name.split_once('_').unwrap().1;
        let selected_pixel = |frame: u32, x: u32, y: u32, p: [u8; 4]| {
            skin[..3].contains(&p)
                || (p == skin[3]
                    && ((*season != "Winter" && hand_shadow(action, frame, x, y))
                        || (*season == "Summer"
                            && action == "walk_east"
                            && matches!((frame, x, y), (1, 42, 51) | (3, 44, 51)))))
        };
        let mut observed = vec![0; counts.len()];
        for (x, y, p) in before.enumerate_pixels() {
            if selected_pixel(x / 80, x % 80, y, p.0) {
                observed[x as usize / 80] += 1;
            } else if skin.contains(&p.0) {
                protected += 1;
            }
        }
        assert_eq!(observed, counts, "source material counts {name}");
        selected += observed.iter().sum::<usize>();
        for (choice, target) in choices {
            let target = target.map(rgb);
            let after = image::open(output.join("variants").join(choice).join(&asset))
                .unwrap()
                .to_rgba8();
            assert_eq!(before.dimensions(), after.dimensions());
            for (x, y, p) in before.enumerate_pixels() {
                let expected = if selected_pixel(x / 80, x % 80, y, p.0) {
                    target[skin.iter().position(|c| c == &p.0).unwrap()]
                } else {
                    p.0
                };
                assert_eq!(after.get_pixel(x, y).0, expected, "{name} {choice} {x},{y}");
            }
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(output.join("variants").join(choice).join(&meta)).unwrap()
            );
        }
    }
    assert_eq!(selected, 4314);
    assert_eq!(protected, 2731);
    let portraits = read_json(root.join("palettes/profiles/wheedle-portraits.json"));
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
        .arg(root.join("palettes/sets/wheedle-portraits-trial.json"))
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
