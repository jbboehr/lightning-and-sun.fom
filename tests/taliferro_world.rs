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
    let before = read_json(root.join("palettes/profiles/taliferro-portraits.json"));
    let after = read_json(root.join("palettes/profiles/taliferro-world-trial.json"));
    assert_eq!(after["source_colors"], before["source_colors"]);
    assert_eq!(after["color_groups"], before["color_groups"]);
    let regions = after["regions"].as_array().unwrap();
    assert_eq!(&regions[..36], before["regions"].as_array().unwrap());
    assert_eq!(regions.len(), 96);
    let expected: BTreeSet<_> = SEASONS
        .iter()
        .flat_map(|season| {
            ACTIONS.iter().map(move |(action, _, _)| {
                format!(
                    "assets/animations/NPCs/Taliferro/Sprites/{season}/spr_npc_taliferro_{}_{action}.png",
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
    let old_set = read_json(root.join("palettes/sets/taliferro-portraits-trial.json"));
    let new_set = read_json(root.join("palettes/sets/taliferro-world-trial.json"));
    assert_eq!(new_set["presets"], old_set["presets"]);
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
            let stem = format!("spr_npc_taliferro_{}_{action}", season.to_lowercase());
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
    assert_eq!(table[0][0], "taliferro");
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
#[ignore = "requires the complete Taliferro source corpus; checks all world pixels and retained portraits"]
fn every_world_pixel_matches_reviewed_materials_and_portraits_remain_identical() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/taliferro");
    let set = std::env::var_os("FOM_TALIFERRO_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/taliferro-world-trial.json"));
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
        ("autumn_action_east", &[45, 47, 46, 47, 46, 45, 48]),
        ("autumn_action_north", &[18, 16, 16, 16, 16, 18, 22]),
        ("autumn_action_south", &[53, 54, 54, 54, 54, 53, 55]),
        ("autumn_blink_east", &[52, 58, 52]),
        ("autumn_blink_south", &[59, 65, 59]),
        ("autumn_eat_south", &[54, 56, 59, 67, 53]),
        ("autumn_idle_east", &[48]),
        ("autumn_idle_north", &[22]),
        ("autumn_idle_south", &[55]),
        ("autumn_sit_east", &[45]),
        ("autumn_sit_north", &[20]),
        ("autumn_sit_south", &[53]),
        ("autumn_walk_east", &[48, 51, 48, 47]),
        ("autumn_walk_north", &[22, 20, 22, 20]),
        ("autumn_walk_south", &[55, 53, 55, 53]),
        ("spring_action_east", &[45, 47, 46, 47, 46, 45, 50]),
        ("spring_action_north", &[18, 16, 16, 16, 16, 19, 24]),
        ("spring_action_south", &[56, 53, 53, 53, 53, 56, 59]),
        ("spring_blink_east", &[54, 60, 54]),
        ("spring_blink_south", &[63, 69, 63]),
        ("spring_eat_south", &[56, 61, 60, 68, 55]),
        ("spring_idle_east", &[50]),
        ("spring_idle_north", &[26]),
        ("spring_idle_south", &[59]),
        ("spring_sit_east", &[46]),
        ("spring_sit_north", &[22]),
        ("spring_sit_south", &[55]),
        ("spring_walk_east", &[50, 54, 50, 48]),
        ("spring_walk_north", &[26, 23, 26, 23]),
        ("spring_walk_south", &[59, 57, 59, 57]),
        ("summer_action_east", &[45, 47, 46, 47, 46, 45, 50]),
        ("summer_action_north", &[18, 16, 16, 16, 16, 20, 26]),
        ("summer_action_south", &[56, 53, 53, 53, 53, 56, 59]),
        ("summer_blink_east", &[54, 60, 54]),
        ("summer_blink_south", &[63, 69, 63]),
        ("summer_eat_south", &[56, 61, 60, 68, 55]),
        ("summer_idle_east", &[50]),
        ("summer_idle_north", &[26]),
        ("summer_idle_south", &[59]),
        ("summer_sit_east", &[46]),
        ("summer_sit_north", &[22]),
        ("summer_sit_south", &[55]),
        ("summer_walk_east", &[50, 54, 50, 48]),
        ("summer_walk_north", &[26, 23, 26, 23]),
        ("summer_walk_south", &[59, 57, 59, 57]),
        ("winter_action_east", &[45, 47, 45, 47, 45, 45, 46]),
        ("winter_action_north", &[14, 12, 12, 12, 12, 14, 18]),
        ("winter_action_south", &[53, 52, 52, 52, 52, 53, 55]),
        ("winter_blink_east", &[50, 56, 50]),
        ("winter_blink_south", &[59, 65, 59]),
        ("winter_eat_south", &[54, 56, 59, 67, 53]),
        ("winter_idle_east", &[46]),
        ("winter_idle_north", &[18]),
        ("winter_idle_south", &[55]),
        ("winter_sit_east", &[45]),
        ("winter_sit_north", &[12]),
        ("winter_sit_south", &[53]),
        ("winter_walk_east", &[46, 51, 46, 45]),
        ("winter_walk_north", &[18, 16, 18, 16]),
        ("winter_walk_south", &[55, 53, 55, 53]),
    ];
    let rgb = |n: u32| [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255];
    let skin = [0xFFDBB8, 0xF2A174, 0xC4613B, 0x762E21].map(rgb);
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
            "assets/animations/NPCs/Taliferro/Sprites/{season}/spr_npc_taliferro_{name}.png"
        );
        let before = image::open(original.join(&asset)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (80 * counts.len() as u32, 80));
        let mut observed = vec![0; counts.len()];
        for (x, y, p) in before.enumerate_pixels() {
            if skin.contains(&p.0) {
                assert!(
                    (31..=49).contains(&(x % 80)) && (30..=49).contains(&y),
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
                    (0, 37, 28, 0x76462A, false),
                    (0, 36, 30, 0xC97F26, false),
                    (0, 39, 40, 0xF5F5F5, false),
                    (0, 36, 35, 0x762E21, true),
                    (0, 36, 37, 0x762E21, true),
                    (0, 33, 46, 0x762E21, true),
                    (0, 39, 37, 0xFFDBB8, true),
                ]
            } else if name.ends_with("eat_south") {
                &[
                    (2, 39, 37, 0x9E2626, false),
                    (2, 39, 35, 0x410808, false),
                    (2, 36, 40, 0x762E21, true),
                    (2, 37, 41, 0xFFDBB8, true),
                ]
            } else {
                &[]
            };
            if name.ends_with("eat_south") {
                let color = match *season {
                    "Autumn" | "Summer" => 0x94592E,
                    "Spring" => 0x4F4040,
                    "Winter" => 0x5E4E43,
                    _ => unreachable!(),
                };
                let x = 2 * 80 + 38;
                assert_eq!(
                    before.get_pixel(x, 44).0,
                    rgb(color),
                    "source garment {name}"
                );
                assert_eq!(
                    after.get_pixel(x, 44),
                    before.get_pixel(x, 44),
                    "garment changed {name} {choice}"
                );
            }
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
    assert_eq!(selected, 8883);
    let portraits = read_json(root.join("palettes/profiles/taliferro-portraits.json"));
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
        .arg(root.join("palettes/sets/taliferro-portraits-trial.json"))
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
