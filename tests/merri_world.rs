use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, process::Command};

const SEASONS: [&str; 4] = ["Autumn", "Spring", "Summer", "Winter"];
const ACTIONS: [(&str, u32, &str); 11] = [
    ("action_east", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("action_north", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("action_south", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("blink_east", 3, "[0.075,0.125,0.075]"),
    ("blink_south", 3, "[0.075,0.125,0.075]"),
    ("idle_east", 1, ""),
    ("idle_north", 1, ""),
    ("idle_south", 1, ""),
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
    let before = read_json(root.join("palettes/profiles/merri-portraits.json"));
    let after = read_json(root.join("palettes/profiles/merri-world-trial.json"));
    assert_eq!(after["source_colors"], before["source_colors"]);
    assert_eq!(after["color_groups"], before["color_groups"]);
    let regions = after["regions"].as_array().unwrap();
    assert_eq!(&regions[..32], before["regions"].as_array().unwrap());
    assert_eq!(regions.len(), 76);
    let expected: BTreeSet<_> = SEASONS
        .iter()
        .flat_map(|season| {
            ACTIONS.iter().map(move |(action, _, _)| {
                format!(
                    "assets/animations/NPCs/Merri/Sprites/{season}/spr_npc_merri_{}_{action}.png",
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
    let old_set = read_json(root.join("palettes/sets/merri-portraits-trial.json"));
    let new_set = read_json(root.join("palettes/sets/merri-world-trial.json"));
    assert_eq!(new_set["presets"], old_set["presets"]);
    for region in &regions[32..] {
        let asset = region["asset"].as_str().unwrap();
        assert_eq!(
            region["seeds"].as_array().unwrap().is_empty(),
            asset.contains("/Winter/") && asset.ends_with("_north.png")
        );
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
            let stem = format!("spr_npc_merri_{}_{action}", season.to_lowercase());
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
    assert_eq!(table[0][0], "merri");
    assert_eq!(table[0][4].as_array().unwrap().len(), 44);
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
    assert_eq!(frames, 168);
}

fn clothing_shadow(name: &str, frame: u32, x: u32, y: u32) -> bool {
    matches!(
        (name, frame, x, y),
        ("spring_action_north", 2 | 4, 44 | 45, 40)
            | ("spring_action_north", 2 | 4, 43, 42)
            | ("spring_action_south", 0 | 5, 37 | 45, 45)
            | ("spring_action_south", 0 | 5, 44, 46)
            | ("spring_walk_east", 1, 36 | 43 | 45, 45)
            | ("spring_walk_east", 1, 44, 46)
            | ("spring_walk_east", 1, 40 | 42, 51)
            | ("spring_walk_east", 3, 43, 44 | 45)
            | ("spring_walk_north" | "spring_walk_south", 1, 43, 45)
            | ("spring_walk_north" | "spring_walk_south", 1, 44, 46)
            | ("spring_walk_north" | "spring_walk_south", 3, 36, 45)
            | ("spring_walk_north" | "spring_walk_south", 3, 35, 46)
    )
}

#[test]
#[ignore = "requires the complete Merri source corpus; checks all world pixels and retained portraits"]
fn every_world_pixel_matches_reviewed_materials_and_portraits_remain_identical() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/merri");
    let set = std::env::var_os("FOM_MERRI_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/merri-world-trial.json"));
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
    let cases: [(&str, &[usize]); 44] = [
        ("autumn_action_east", &[39, 41, 40, 41, 40, 39, 42]),
        ("autumn_action_north", &[9, 7, 7, 7, 7, 9, 14]),
        ("autumn_action_south", &[49, 48, 47, 48, 47, 49, 52]),
        ("autumn_blink_east", &[46, 55, 46]),
        ("autumn_blink_south", &[56, 64, 56]),
        ("autumn_idle_east", &[42]),
        ("autumn_idle_north", &[14]),
        ("autumn_idle_south", &[52]),
        ("autumn_walk_east", &[42, 48, 42, 42]),
        ("autumn_walk_north", &[14, 11, 14, 11]),
        ("autumn_walk_south", &[52, 49, 52, 49]),
        ("spring_action_east", &[43, 46, 43, 46, 43, 43, 43]),
        ("spring_action_north", &[10, 8, 8, 8, 8, 10, 13]),
        ("spring_action_south", &[49, 51, 50, 51, 50, 49, 52]),
        ("spring_blink_east", &[47, 56, 47]),
        ("spring_blink_south", &[56, 64, 56]),
        ("spring_idle_east", &[43]),
        ("spring_idle_north", &[13]),
        ("spring_idle_south", &[52]),
        ("spring_walk_east", &[43, 48, 43, 41]),
        ("spring_walk_north", &[13, 12, 13, 12]),
        ("spring_walk_south", &[52, 50, 52, 50]),
        ("summer_action_east", &[43, 46, 45, 46, 45, 43, 44]),
        ("summer_action_north", &[12, 7, 7, 7, 7, 12, 16]),
        ("summer_action_south", &[52, 50, 50, 50, 50, 52, 56]),
        ("summer_blink_east", &[48, 57, 48]),
        ("summer_blink_south", &[60, 68, 60]),
        ("summer_idle_east", &[44]),
        ("summer_idle_north", &[16]),
        ("summer_idle_south", &[56]),
        ("summer_walk_east", &[44, 50, 44, 42]),
        ("summer_walk_north", &[16, 12, 16, 12]),
        ("summer_walk_south", &[56, 52, 56, 50]),
        ("winter_action_east", &[35, 35, 35, 35, 35, 35, 35]),
        ("winter_action_north", &[0, 0, 0, 0, 0, 0, 0]),
        ("winter_action_south", &[40, 40, 40, 40, 40, 40, 40]),
        ("winter_blink_east", &[39, 48, 39]),
        ("winter_blink_south", &[44, 52, 44]),
        ("winter_idle_east", &[35]),
        ("winter_idle_north", &[0]),
        ("winter_idle_south", &[40]),
        ("winter_walk_east", &[35, 37, 35, 35]),
        ("winter_walk_north", &[0, 0, 0, 0]),
        ("winter_walk_south", &[40, 40, 40, 40]),
    ];
    let rgb = |n: u32| [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255];
    let skin = [0xDD9266, 0xB36644, 0x9E512F, 0x7F382B].map(rgb);
    let choices = [
        ("blue", [0x9DB9D4, 0x6687AD, 0x445F83, 0x445F83]),
        ("npc_hayden", [0xE8B271, 0xB27146, 0x6E4922, 0x6E4922]),
        ("npc_ryis", [0xB06C57, 0x63342A, 0x491F1B, 0x491F1B]),
        ("npc_seridia", [0xC1AFA5, 0x8E746D, 0x624A48, 0x624A48]),
    ];
    let mut selected = 0;
    let mut protected = 0;
    for (name, counts) in cases {
        let season = name.split('_').next().unwrap();
        let season = SEASONS.iter().find(|s| s.to_lowercase() == season).unwrap();
        let asset =
            format!("assets/animations/NPCs/Merri/Sprites/{season}/spr_npc_merri_{name}.png");
        let before = image::open(original.join(&asset)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (80 * counts.len() as u32, 80));
        let mut observed = vec![0; counts.len()];
        for (x, y, p) in before.enumerate_pixels() {
            if skin.contains(&p.0) {
                if clothing_shadow(name, x / 80, x % 80, y) {
                    assert_eq!(p.0, rgb(0x7F382B));
                    protected += 1;
                } else {
                    assert!(
                        (32..=49).contains(&(x % 80)) && (30..=48).contains(&y),
                        "unexpected skin material {name} {x},{y}"
                    );
                    observed[x as usize / 80] += 1;
                }
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
                let expected = if clothing_shadow(name, x / 80, x % 80, y) {
                    p.0
                } else {
                    skin.iter()
                        .position(|c| c == &p.0)
                        .map_or(p.0, |i| target[i])
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
    assert_eq!(selected, 5941);
    assert_eq!(protected, 28);
    let portraits = read_json(root.join("palettes/profiles/merri-portraits.json"));
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
        .arg(root.join("palettes/sets/merri-portraits-trial.json"))
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
