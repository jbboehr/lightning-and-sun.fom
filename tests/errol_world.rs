use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, process::Command};

const SEASONS: [&str; 4] = ["Autumn", "Spring", "Summer", "Winter"];
// Frozen archive inventory: 26 seasonal strips each, plus three Spring shocked poses.
const ACTIONS: [(&str, u32, &str); 107] = [
    ("autumn_action_east", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    (
        "autumn_action_north",
        7,
        "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
    ),
    (
        "autumn_action_south",
        7,
        "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
    ),
    ("autumn_blink_east", 3, "[0.075,0.125,0.075]"),
    ("autumn_blink_south", 3, "[0.075,0.125,0.075]"),
    ("autumn_drink_east", 3, "1"),
    ("autumn_drink_north", 3, "1"),
    ("autumn_drink_south", 3, "1"),
    ("autumn_eat_east", 5, "[0.125,0.15,0.175,0.125,0.6]"),
    ("autumn_eat_north", 3, "1"),
    ("autumn_eat_south", 5, "[0.125,0.15,0.175,0.125,0.6]"),
    ("autumn_idle_east", 1, ""),
    ("autumn_idle_north", 1, ""),
    ("autumn_idle_south", 1, ""),
    ("autumn_sit_east", 1, ""),
    ("autumn_sit_north", 1, ""),
    ("autumn_sit_south", 1, ""),
    ("autumn_walk_east", 4, "0.15"),
    ("autumn_walk_north", 4, "0.15"),
    ("autumn_walk_south", 4, "0.15"),
    (
        "specialanimation_autumn_brush_east",
        7,
        "[0.1,0.25,0.25,0.25,0.1,0.125,0.8]",
    ),
    ("specialanimation_autumn_magnify_end_south", 1, ""),
    ("specialanimation_autumn_magnify_loop_south", 1, ""),
    (
        "specialanimation_autumn_magnify_start_south",
        2,
        "[0.1,0.125]",
    ),
    (
        "specialanimation_autumn_pickaxe_east",
        6,
        "[0.35,0.1,0.1,0.1,0.1,0.35]",
    ),
    (
        "specialanimation_autumn_trowel_east",
        10,
        "[0.4,0.15,0.2,0.2,0.2,0.2,0.2,0.2,0.15,1.4]",
    ),
    (
        "specialanimation_spring_brush_east",
        7,
        "[0.1,0.25,0.25,0.25,0.1,0.125,0.8]",
    ),
    ("specialanimation_spring_magnify_end_south", 1, ""),
    ("specialanimation_spring_magnify_loop_south", 1, ""),
    (
        "specialanimation_spring_magnify_start_south",
        2,
        "[0.1,0.125]",
    ),
    (
        "specialanimation_spring_pickaxe_east",
        6,
        "[0.35,0.1,0.1,0.1,0.1,0.35]",
    ),
    (
        "specialanimation_spring_trowel_east",
        10,
        "[0.4,0.15,0.2,0.2,0.2,0.2,0.2,0.2,0.15,1.4]",
    ),
    ("spring_action_east", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    (
        "spring_action_north",
        7,
        "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
    ),
    (
        "spring_action_south",
        7,
        "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
    ),
    ("spring_blink_east", 3, "[0.075,0.125,0.075]"),
    ("spring_blink_south", 3, "[0.075,0.125,0.075]"),
    ("spring_drink_east", 3, "1"),
    ("spring_drink_north", 3, "1"),
    ("spring_drink_south", 3, "1"),
    ("spring_eat_east", 5, "[0.125,0.15,0.175,0.125,0.6]"),
    ("spring_eat_north", 3, "1"),
    ("spring_eat_south", 5, "[0.125,0.15,0.175,0.125,0.6]"),
    ("spring_idle_east", 1, ""),
    ("spring_idle_north", 1, ""),
    ("spring_idle_south", 1, ""),
    ("spring_shocked_end_south", 1, ""),
    ("spring_shocked_loop_south", 1, ""),
    ("spring_shocked_start_south", 1, ""),
    ("spring_sit_east", 1, ""),
    ("spring_sit_north", 1, ""),
    ("spring_sit_south", 1, ""),
    ("spring_walk_east", 4, "0.15"),
    ("spring_walk_north", 4, "0.15"),
    ("spring_walk_south", 4, "0.15"),
    (
        "specialanimation_summer_brush_east",
        7,
        "[0.1,0.25,0.25,0.25,0.1,0.125,0.8]",
    ),
    ("specialanimation_summer_magnify_end_south", 1, ""),
    ("specialanimation_summer_magnify_loop_south", 1, ""),
    (
        "specialanimation_summer_magnify_start_south",
        2,
        "[0.1,0.125]",
    ),
    (
        "specialanimation_summer_pickaxe_east",
        6,
        "[0.35,0.1,0.1,0.1,0.1,0.35]",
    ),
    (
        "specialanimation_summer_trowel_east",
        10,
        "[0.4,0.15,0.2,0.2,0.2,0.2,0.2,0.2,0.15,1.4]",
    ),
    ("summer_action_east", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    (
        "summer_action_north",
        7,
        "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
    ),
    (
        "summer_action_south",
        7,
        "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
    ),
    ("summer_blink_east", 3, "[0.075,0.125,0.075]"),
    ("summer_blink_south", 3, "[0.075,0.125,0.075]"),
    ("summer_drink_east", 3, "1"),
    ("summer_drink_north", 3, "1"),
    ("summer_drink_south", 3, "1"),
    ("summer_eat_east", 5, "[0.125,0.15,0.175,0.125,0.6]"),
    ("summer_eat_north", 3, "1"),
    ("summer_eat_south", 5, "[0.125,0.15,0.175,0.125,0.6]"),
    ("summer_idle_east", 1, ""),
    ("summer_idle_north", 1, ""),
    ("summer_idle_south", 1, ""),
    ("summer_sit_east", 1, ""),
    ("summer_sit_north", 1, ""),
    ("summer_sit_south", 1, ""),
    ("summer_walk_east", 4, "0.15"),
    ("summer_walk_north", 4, "0.15"),
    ("summer_walk_south", 4, "0.15"),
    (
        "specialanimation_winter_brush_east",
        7,
        "[0.1,0.25,0.25,0.25,0.1,0.125,0.8]",
    ),
    ("specialanimation_winter_magnify_end_south", 1, ""),
    ("specialanimation_winter_magnify_loop_south", 1, ""),
    (
        "specialanimation_winter_magnify_start_south",
        2,
        "[0.1,0.125]",
    ),
    (
        "specialanimation_winter_pickaxe_east",
        6,
        "[0.35,0.1,0.1,0.1,0.1,0.35]",
    ),
    (
        "specialanimation_winter_trowel_east",
        10,
        "[0.4,0.15,0.2,0.2,0.2,0.2,0.2,0.2,0.15,1.4]",
    ),
    ("winter_action_east", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    (
        "winter_action_north",
        7,
        "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
    ),
    (
        "winter_action_south",
        7,
        "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
    ),
    ("winter_blink_east", 3, "[0.075,0.125,0.075]"),
    ("winter_blink_south", 3, "[0.075,0.125,0.075]"),
    ("winter_drink_east", 3, "1"),
    ("winter_drink_north", 3, "1"),
    ("winter_drink_south", 3, "1"),
    ("winter_eat_east", 5, "[0.125,0.15,0.175,0.125,0.6]"),
    ("winter_eat_north", 3, "1"),
    ("winter_eat_south", 5, "[0.125,0.15,0.175,0.125,0.6]"),
    ("winter_idle_east", 1, ""),
    ("winter_idle_north", 1, ""),
    ("winter_idle_south", 1, ""),
    ("winter_sit_east", 1, ""),
    ("winter_sit_north", 1, ""),
    ("winter_sit_south", 1, ""),
    ("winter_walk_east", 4, "0.15"),
    ("winter_walk_north", 4, "0.15"),
    ("winter_walk_south", 4, "0.15"),
];

fn asset(name: &str) -> String {
    let season = SEASONS
        .iter()
        .find(|s| {
            name.starts_with(&format!("{}_", s.to_lowercase()))
                || name.contains(&format!("_{}_", s.to_lowercase()))
        })
        .unwrap();
    format!("assets/animations/NPCs/Errol/Sprites/{season}/spr_npc_errol_{name}.png")
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn world_profile_retains_every_portrait_region_and_palette_role() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let before = read_json(root.join("palettes/profiles/errol-portraits.json"));
    let after = read_json(root.join("palettes/profiles/errol-world-trial.json"));
    assert_eq!(after["source_colors"], before["source_colors"]);
    assert_eq!(after["color_groups"], before["color_groups"]);
    let regions = after["regions"].as_array().unwrap();
    assert_eq!(&regions[..36], before["regions"].as_array().unwrap());
    assert_eq!(regions.len(), 143);
    let expected: BTreeSet<_> = ACTIONS.iter().map(|(name, _, _)| asset(name)).collect();
    let actual: BTreeSet<_> = regions[36..]
        .iter()
        .map(|r| r["asset"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(actual, expected);
    let old_set = read_json(root.join("palettes/sets/errol-portraits-trial.json"));
    let new_set = read_json(root.join("palettes/sets/errol-world-trial.json"));
    assert_eq!(new_set["presets"].as_array().unwrap().len(), 4);
    assert_eq!(old_set["presets"], new_set["presets"]);
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
    for (action, frames, duration) in ACTIONS {
        let stem = format!("spr_npc_errol_{action}");
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
    assert_eq!(table[0][0], "errol");
    assert_eq!(table[0][4].as_array().unwrap().len(), 107);
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
    assert_eq!(frames, 379);
}

#[test]
#[ignore = "requires the complete Errol source corpus; checks all world pixels and retained portraits"]
fn every_world_pixel_matches_reviewed_materials_and_portraits_remain_identical() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/errol");
    let set = std::env::var_os("FOM_ERROL_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/errol-world-trial.json"));
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
    // Counts come from an independent full-source material census, never from mask seeds.
    let cases: [(&str, &[usize]); 107] = [
        ("autumn_action_east", &[25, 26, 26, 26, 26, 25, 34]),
        ("autumn_action_north", &[58, 53, 55, 53, 55, 57, 64]),
        ("autumn_action_south", &[36, 37, 37, 37, 37, 36, 42]),
        ("autumn_blink_east", &[34, 42, 34]),
        ("autumn_blink_south", &[42, 52, 42]),
        ("autumn_drink_east", &[26, 39, 26]),
        ("autumn_drink_north", &[56, 57, 56]),
        ("autumn_drink_south", &[42, 52, 42]),
        ("autumn_eat_east", &[26, 25, 22, 29, 25]),
        ("autumn_eat_north", &[56, 57, 56]),
        ("autumn_eat_south", &[42, 46, 38, 51, 42]),
        ("autumn_idle_east", &[34]),
        ("autumn_idle_north", &[64]),
        ("autumn_idle_south", &[42]),
        ("autumn_sit_east", &[26]),
        ("autumn_sit_north", &[66]),
        ("autumn_sit_south", &[42]),
        ("autumn_walk_east", &[34, 32, 34, 27]),
        ("autumn_walk_north", &[64, 57, 64, 57]),
        ("autumn_walk_south", &[42, 35, 42, 35]),
        (
            "specialanimation_autumn_brush_east",
            &[25, 26, 26, 26, 26, 25, 34],
        ),
        ("specialanimation_autumn_magnify_end_south", &[51]),
        ("specialanimation_autumn_magnify_loop_south", &[39]),
        ("specialanimation_autumn_magnify_start_south", &[38, 41]),
        (
            "specialanimation_autumn_pickaxe_east",
            &[35, 27, 27, 27, 27, 35],
        ),
        (
            "specialanimation_autumn_trowel_east",
            &[31, 24, 25, 24, 25, 24, 25, 24, 24, 31],
        ),
        (
            "specialanimation_spring_brush_east",
            &[57, 54, 53, 54, 53, 57, 61],
        ),
        ("specialanimation_spring_magnify_end_south", &[82]),
        ("specialanimation_spring_magnify_loop_south", &[71]),
        ("specialanimation_spring_magnify_start_south", &[63, 72]),
        (
            "specialanimation_spring_pickaxe_east",
            &[58, 55, 55, 55, 55, 58],
        ),
        (
            "specialanimation_spring_trowel_east",
            &[61, 56, 50, 48, 50, 48, 50, 48, 56, 61],
        ),
        ("spring_action_east", &[57, 54, 53, 54, 53, 57, 61]),
        ("spring_action_north", &[101, 88, 94, 88, 94, 100, 102]),
        ("spring_action_south", &[61, 65, 70, 65, 70, 61, 68]),
        ("spring_blink_east", &[65, 74, 65]),
        ("spring_blink_south", &[72, 82, 72]),
        ("spring_drink_east", &[54, 69, 54]),
        ("spring_drink_north", &[100, 91, 100]),
        ("spring_drink_south", &[68, 79, 68]),
        ("spring_eat_east", &[56, 59, 52, 62, 55]),
        ("spring_eat_north", &[100, 91, 100]),
        ("spring_eat_south", &[70, 74, 66, 81, 68]),
        ("spring_idle_east", &[61]),
        ("spring_idle_north", &[102]),
        ("spring_idle_south", &[68]),
        ("spring_shocked_end_south", &[82]),
        ("spring_shocked_loop_south", &[66]),
        ("spring_shocked_start_south", &[82]),
        ("spring_sit_east", &[51]),
        ("spring_sit_north", &[104]),
        ("spring_sit_south", &[68]),
        ("spring_walk_east", &[61, 55, 61, 55]),
        ("spring_walk_north", &[102, 95, 102, 95]),
        ("spring_walk_south", &[68, 61, 68, 61]),
        (
            "specialanimation_summer_brush_east",
            &[49, 47, 48, 47, 48, 49, 53],
        ),
        ("specialanimation_summer_magnify_end_south", &[70]),
        ("specialanimation_summer_magnify_loop_south", &[56]),
        ("specialanimation_summer_magnify_start_south", &[55, 60]),
        (
            "specialanimation_summer_pickaxe_east",
            &[45, 52, 52, 52, 51, 45],
        ),
        (
            "specialanimation_summer_trowel_east",
            &[56, 50, 50, 49, 50, 49, 50, 49, 50, 56],
        ),
        ("summer_action_east", &[49, 47, 48, 47, 48, 49, 53]),
        ("summer_action_north", &[87, 84, 85, 84, 85, 86, 92]),
        ("summer_action_south", &[53, 54, 55, 54, 55, 53, 58]),
        ("summer_blink_east", &[57, 66, 57]),
        ("summer_blink_south", &[62, 72, 62]),
        ("summer_drink_east", &[42, 56, 42]),
        ("summer_drink_north", &[87, 86, 87]),
        ("summer_drink_south", &[55, 68, 55]),
        ("summer_eat_east", &[45, 50, 41, 53, 48]),
        ("summer_eat_north", &[87, 86, 87]),
        ("summer_eat_south", &[58, 66, 56, 75, 58]),
        ("summer_idle_east", &[53]),
        ("summer_idle_north", &[92]),
        ("summer_idle_south", &[58]),
        ("summer_sit_east", &[47]),
        ("summer_sit_north", &[94]),
        ("summer_sit_south", &[58]),
        ("summer_walk_east", &[53, 56, 53, 50]),
        ("summer_walk_north", &[92, 87, 92, 87]),
        ("summer_walk_south", &[58, 53, 58, 53]),
        (
            "specialanimation_winter_brush_east",
            &[23, 23, 23, 23, 23, 23, 24],
        ),
        ("specialanimation_winter_magnify_end_south", &[43]),
        ("specialanimation_winter_magnify_loop_south", &[28]),
        ("specialanimation_winter_magnify_start_south", &[28, 33]),
        (
            "specialanimation_winter_pickaxe_east",
            &[27, 22, 22, 22, 24, 27],
        ),
        (
            "specialanimation_winter_trowel_east",
            &[28, 24, 24, 22, 24, 22, 24, 22, 24, 28],
        ),
        ("winter_action_east", &[23, 23, 23, 23, 23, 23, 24]),
        ("winter_action_north", &[40, 41, 41, 41, 41, 43, 46]),
        ("winter_action_south", &[28, 28, 30, 28, 30, 26, 30]),
        ("winter_blink_east", &[24, 32, 24]),
        ("winter_blink_south", &[30, 40, 30]),
        ("winter_drink_east", &[23, 38, 23]),
        ("winter_drink_north", &[43, 41, 43]),
        ("winter_drink_south", &[35, 45, 35]),
        ("winter_eat_east", &[20, 22, 20, 26, 20]),
        ("winter_eat_north", &[43, 41, 43]),
        ("winter_eat_south", &[35, 39, 36, 46, 36]),
        ("winter_idle_east", &[24]),
        ("winter_idle_north", &[46]),
        ("winter_idle_south", &[30]),
        ("winter_sit_east", &[23]),
        ("winter_sit_north", &[48]),
        ("winter_sit_south", &[36]),
        ("winter_walk_east", &[24, 26, 24, 22]),
        ("winter_walk_north", &[46, 42, 46, 42]),
        ("winter_walk_south", &[30, 26, 30, 26]),
    ];
    let rgb = |n: u32| [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255];
    let skin = [0xF4C49C, 0xC9927B, 0x85584B].map(rgb);
    let choices = [
        ("blue", [0x9DB9D4, 0x6687AD, 0x445F83]),
        ("npc_hayden", [0xE8B271, 0xB27146, 0x6E4922]),
        ("npc_ryis", [0xB06C57, 0x63342A, 0x491F1B]),
        ("npc_seridia", [0xC1AFA5, 0x8E746D, 0x624A48]),
    ];
    let mut selected = 0;
    let mut unique = BTreeSet::new();
    let mut native_frames = 0;
    for (name, counts) in cases {
        let asset = asset(name);
        let before = image::open(original.join(&asset)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (80 * counts.len() as u32, 80));
        for f in 0..counts.len() as u32 {
            unique.insert(
                image::imageops::crop_imm(&before, f * 80, 0, 80, 80)
                    .to_image()
                    .into_raw(),
            );
            native_frames += 1;
        }
        let mut observed = vec![0; counts.len()];
        for (x, y, p) in before.enumerate_pixels() {
            if skin.contains(&p.0) {
                assert!(
                    (30..=51).contains(&(x % 80)) && (26..=53).contains(&y),
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
            // Literal anatomical and protected-material points inspected in the source.
            let landmarks: &[(u32, u32, u32, u32, bool)] = if name.ends_with("idle_south") {
                &[
                    (0, 35, 33, 0xF4C49C, true),
                    (0, 36, 33, 0x85584B, true),
                    (0, 39, 37, 0xC9927B, true), // skin between moustache and beard
                    (0, 39, 35, 0xEFEAE3, false),
                    (0, 36, 35, 0x897C6C, false),
                    (0, 38, 35, 0xBCB2A5, false),
                    (0, 37, 32, 0xC2B9BE, false),
                    (0, 37, 33, 0xECF0E9, false),
                    (0, 38, 33, 0x000000, false),
                ]
            } else if name.ends_with("eat_east") {
                &[
                    (2, 40, 32, 0x410808, false),
                    (2, 40, 33, 0x9E2626, false),
                    (2, 40, 36, 0xC9927B, true),
                    (2, 44, 37, 0xF4C49C, true),
                ]
            } else if name.ends_with("shocked_loop_south") {
                &[
                    (0, 39, 33, 0x410808, false),
                    (0, 39, 34, 0x9E2626, false),
                    (0, 39, 26, 0xF4C49C, true),
                    (0, 30, 30, 0xC9927B, true),
                ]
            } else if name.ends_with("magnify_start_south") {
                &[(1, 44, 39, 0xE7F7FF, false), (1, 35, 34, 0xF4C49C, true)]
            } else if name.ends_with("trowel_east") {
                &[(3, 53, 54, 0xC5CECF, false), (3, 49, 52, 0xF4C49C, true)]
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
    assert_eq!(selected, 18794);
    assert_eq!(native_frames, 379);
    assert_eq!(unique.len(), 227);
    let portraits = read_json(root.join("palettes/profiles/errol-portraits.json"));
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
        .arg(root.join("palettes/sets/errol-portraits-trial.json"))
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
