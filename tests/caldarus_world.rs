use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, process::Command};

const SEASONS: [&str; 6] = ["Autumn", "Beach", "Spring", "Summer", "Wedding", "Winter"];
fn inventory() -> Vec<(String, u32, &'static str)> {
    vec![
        (
            "autumn_action_east".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "autumn_action_north".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "autumn_action_south".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        ("autumn_blink_east".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("autumn_blink_south".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("autumn_drink_east".to_owned(), 3, "1.0"),
        ("autumn_drink_north".to_owned(), 3, "1.0"),
        ("autumn_drink_south".to_owned(), 3, "1.0"),
        (
            "autumn_eat_east".to_owned(),
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        ("autumn_eat_north".to_owned(), 3, "1.0"),
        (
            "autumn_eat_south".to_owned(),
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        ("autumn_idle_east".to_owned(), 1, ""),
        ("autumn_idle_north".to_owned(), 1, ""),
        ("autumn_idle_south".to_owned(), 1, ""),
        ("autumn_kiss_east".to_owned(), 4, "[0.15,0.15,0.8,0.15]"),
        ("autumn_sit_east".to_owned(), 1, ""),
        ("autumn_sit_north".to_owned(), 1, ""),
        ("autumn_sit_south".to_owned(), 1, ""),
        ("autumn_sleep_east".to_owned(), 1, ""),
        ("autumn_walk_east".to_owned(), 4, "0.15"),
        ("autumn_walk_north".to_owned(), 4, "0.15"),
        ("autumn_walk_south".to_owned(), 4, "0.15"),
        ("specialanimation_autumn_eyes_closed_east".to_owned(), 1, ""),
        (
            "specialanimation_autumn_eyes_closed_south".to_owned(),
            1,
            "",
        ),
        (
            "specialanimation_autumn_harvest_east".to_owned(),
            10,
            "[0.4,0.1,0.2,0.2,0.2,0.2,0.2,0.2,0.1,1.4]",
        ),
        (
            "specialanimation_autumn_lute_play_end_south".to_owned(),
            2,
            "0.1",
        ),
        (
            "specialanimation_autumn_lute_play_loop_south".to_owned(),
            10,
            "0.1",
        ),
        (
            "specialanimation_autumn_lute_play_start_south".to_owned(),
            2,
            "0.1",
        ),
        (
            "specialanimation_autumn_read_end_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_autumn_read_loop_south".to_owned(),
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "specialanimation_autumn_read_sit_end_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_autumn_read_sit_loop_south".to_owned(),
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "specialanimation_autumn_read_sit_start_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_autumn_read_start_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_autumn_spell_cast_end_south".to_owned(),
            1,
            "",
        ),
        (
            "specialanimation_autumn_spell_cast_loop_south".to_owned(),
            4,
            "0.125",
        ),
        (
            "specialanimation_autumn_spell_cast_start_south".to_owned(),
            2,
            "0.125",
        ),
        (
            "beach_action_east".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "beach_action_north".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "beach_action_south".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        ("beach_bath_swim_east".to_owned(), 4, "0.15"),
        ("beach_bath_swim_south".to_owned(), 4, "0.15"),
        ("beach_blink_east".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("beach_blink_south".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("beach_idle_east".to_owned(), 1, ""),
        ("beach_idle_north".to_owned(), 1, ""),
        ("beach_idle_south".to_owned(), 1, ""),
        ("beach_kiss_east".to_owned(), 4, "[0.15,0.15,0.8,0.15]"),
        ("beach_walk_east".to_owned(), 4, "0.15"),
        ("beach_walk_north".to_owned(), 4, "0.15"),
        ("beach_walk_south".to_owned(), 4, "0.15"),
        ("specialanimation_beach_eyes_closed_east".to_owned(), 1, ""),
        ("specialanimation_beach_eyes_closed_south".to_owned(), 1, ""),
        ("specialanimation_spring_eyes_closed_east".to_owned(), 1, ""),
        (
            "specialanimation_spring_eyes_closed_south".to_owned(),
            1,
            "",
        ),
        (
            "specialanimation_spring_harvest_east".to_owned(),
            10,
            "[0.4,0.1,0.2,0.2,0.2,0.2,0.2,0.2,0.1,1.4]",
        ),
        (
            "specialanimation_spring_lute_play_end_south".to_owned(),
            2,
            "0.1",
        ),
        (
            "specialanimation_spring_lute_play_loop_south".to_owned(),
            10,
            "0.1",
        ),
        (
            "specialanimation_spring_lute_play_start_south".to_owned(),
            2,
            "0.1",
        ),
        (
            "specialanimation_spring_read_end_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_spring_read_loop_south".to_owned(),
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "specialanimation_spring_read_sit_end_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_spring_read_sit_loop_south".to_owned(),
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "specialanimation_spring_read_sit_start_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_spring_read_start_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_spring_spell_cast_end_south".to_owned(),
            1,
            "",
        ),
        (
            "specialanimation_spring_spell_cast_loop_south".to_owned(),
            4,
            "0.125",
        ),
        (
            "specialanimation_spring_spell_cast_start_south".to_owned(),
            2,
            "0.125",
        ),
        (
            "spring_action_east".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "spring_action_north".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "spring_action_south".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        ("spring_blink_east".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("spring_blink_south".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("spring_drink_east".to_owned(), 3, "1.0"),
        ("spring_drink_north".to_owned(), 3, "1.0"),
        ("spring_drink_south".to_owned(), 3, "1.0"),
        (
            "spring_eat_east".to_owned(),
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        ("spring_eat_north".to_owned(), 3, "1.0"),
        (
            "spring_eat_south".to_owned(),
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        ("spring_idle_east".to_owned(), 1, ""),
        ("spring_idle_north".to_owned(), 1, ""),
        ("spring_idle_south".to_owned(), 1, ""),
        ("spring_kiss_east".to_owned(), 4, "[0.15,0.15,0.8,0.15]"),
        ("spring_sit_east".to_owned(), 1, ""),
        ("spring_sit_north".to_owned(), 1, ""),
        ("spring_sit_south".to_owned(), 1, ""),
        ("spring_sleep_east".to_owned(), 1, ""),
        ("spring_walk_east".to_owned(), 4, "0.15"),
        ("spring_walk_north".to_owned(), 4, "0.15"),
        ("spring_walk_south".to_owned(), 4, "0.15"),
        ("specialanimation_summer_eyes_closed_east".to_owned(), 1, ""),
        (
            "specialanimation_summer_eyes_closed_south".to_owned(),
            1,
            "",
        ),
        (
            "specialanimation_summer_harvest_east".to_owned(),
            10,
            "[0.4,0.1,0.2,0.2,0.2,0.2,0.2,0.2,0.1,1.4]",
        ),
        (
            "specialanimation_summer_lute_play_end_south".to_owned(),
            2,
            "0.1",
        ),
        (
            "specialanimation_summer_lute_play_loop_south".to_owned(),
            10,
            "0.1",
        ),
        (
            "specialanimation_summer_lute_play_start_south".to_owned(),
            2,
            "0.1",
        ),
        (
            "specialanimation_summer_read_end_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_summer_read_loop_south".to_owned(),
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "specialanimation_summer_read_sit_end_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_summer_read_sit_loop_south".to_owned(),
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "specialanimation_summer_read_sit_start_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_summer_read_start_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_summer_spell_cast_end_south".to_owned(),
            1,
            "",
        ),
        (
            "specialanimation_summer_spell_cast_loop_south".to_owned(),
            4,
            "0.125",
        ),
        (
            "specialanimation_summer_spell_cast_start_south".to_owned(),
            2,
            "0.125",
        ),
        (
            "summer_action_east".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "summer_action_north".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "summer_action_south".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        ("summer_blink_east".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("summer_blink_south".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("summer_drink_east".to_owned(), 3, "1.0"),
        ("summer_drink_north".to_owned(), 3, "1.0"),
        ("summer_drink_south".to_owned(), 3, "1.0"),
        (
            "summer_eat_east".to_owned(),
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        ("summer_eat_north".to_owned(), 3, "1.0"),
        (
            "summer_eat_south".to_owned(),
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        ("summer_idle_east".to_owned(), 1, ""),
        ("summer_idle_north".to_owned(), 1, ""),
        ("summer_idle_south".to_owned(), 1, ""),
        ("summer_kiss_east".to_owned(), 4, "[0.15,0.15,0.8,0.15]"),
        ("summer_sit_east".to_owned(), 1, ""),
        ("summer_sit_north".to_owned(), 1, ""),
        ("summer_sit_south".to_owned(), 1, ""),
        ("summer_sleep_east".to_owned(), 1, ""),
        ("summer_walk_east".to_owned(), 4, "0.15"),
        ("summer_walk_north".to_owned(), 4, "0.15"),
        ("summer_walk_south".to_owned(), 4, "0.15"),
        (
            "wedding_action_east".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "wedding_action_north".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "wedding_action_south".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        ("wedding_blink_east".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("wedding_blink_south".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("wedding_idle_east".to_owned(), 1, ""),
        ("wedding_idle_north".to_owned(), 1, ""),
        ("wedding_idle_south".to_owned(), 1, ""),
        ("wedding_kiss_east".to_owned(), 4, "[0.15,0.15,0.8,0.15]"),
        ("wedding_sit_east".to_owned(), 1, ""),
        ("wedding_sit_north".to_owned(), 1, ""),
        ("wedding_sit_south".to_owned(), 1, ""),
        ("wedding_walk_east".to_owned(), 4, "0.15"),
        ("wedding_walk_north".to_owned(), 4, "0.15"),
        ("wedding_walk_south".to_owned(), 4, "0.15"),
        ("specialanimation_winter_eyes_closed_east".to_owned(), 1, ""),
        (
            "specialanimation_winter_eyes_closed_south".to_owned(),
            1,
            "",
        ),
        (
            "specialanimation_winter_harvest_east".to_owned(),
            10,
            "[0.4,0.1,0.2,0.2,0.2,0.2,0.2,0.2,0.1,1.4]",
        ),
        (
            "specialanimation_winter_lute_play_end_south".to_owned(),
            2,
            "0.1",
        ),
        (
            "specialanimation_winter_lute_play_loop_south".to_owned(),
            10,
            "0.1",
        ),
        (
            "specialanimation_winter_lute_play_start_south".to_owned(),
            2,
            "0.1",
        ),
        (
            "specialanimation_winter_read_end_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_winter_read_loop_south".to_owned(),
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "specialanimation_winter_read_sit_end_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_winter_read_sit_loop_south".to_owned(),
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "specialanimation_winter_read_sit_start_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_winter_read_start_south".to_owned(),
            3,
            "0.1",
        ),
        (
            "specialanimation_winter_spell_cast_end_south".to_owned(),
            1,
            "",
        ),
        (
            "specialanimation_winter_spell_cast_loop_south".to_owned(),
            4,
            "0.125",
        ),
        (
            "specialanimation_winter_spell_cast_start_south".to_owned(),
            2,
            "0.125",
        ),
        (
            "winter_action_east".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "winter_action_north".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "winter_action_south".to_owned(),
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        ("winter_blink_east".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("winter_blink_south".to_owned(), 3, "[0.075,0.125,0.075]"),
        ("winter_drink_east".to_owned(), 3, "1.0"),
        ("winter_drink_north".to_owned(), 3, "1.0"),
        ("winter_drink_south".to_owned(), 3, "1.0"),
        (
            "winter_eat_east".to_owned(),
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        ("winter_eat_north".to_owned(), 3, "1.0"),
        (
            "winter_eat_south".to_owned(),
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        ("winter_idle_east".to_owned(), 1, ""),
        ("winter_idle_north".to_owned(), 1, ""),
        ("winter_idle_south".to_owned(), 1, ""),
        ("winter_kiss_east".to_owned(), 4, "[0.15,0.15,0.8,0.15]"),
        ("winter_sit_east".to_owned(), 1, ""),
        ("winter_sit_north".to_owned(), 1, ""),
        ("winter_sit_south".to_owned(), 1, ""),
        ("winter_sleep_east".to_owned(), 1, ""),
        ("winter_walk_east".to_owned(), 4, "0.15"),
        ("winter_walk_north".to_owned(), 4, "0.15"),
        ("winter_walk_south".to_owned(), 4, "0.15"),
    ]
}
fn source_asset(name: &str) -> String {
    let season = SEASONS
        .iter()
        .find(|s| name.contains(&format!("{}_", s.to_lowercase())))
        .unwrap();
    format!("assets/animations/NPCs/Caldarus/Sprites/{season}/spr_npc_caldarus_{name}.png")
}
fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn world_profile_retains_every_portrait_region_and_palette_role() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let before = read_json(root.join("palettes/profiles/caldarus-portraits.json"));
    let after = read_json(root.join("palettes/profiles/caldarus-world-trial.json"));
    assert_eq!(
        after["source_colors"].as_array().unwrap()[..9],
        before["source_colors"].as_array().unwrap()[..]
    );
    assert_eq!(
        after["color_groups"].as_array().unwrap()[..9],
        before["color_groups"].as_array().unwrap()[..]
    );
    let regions = after["regions"].as_array().unwrap();
    assert_eq!(&regions[..98], before["regions"].as_array().unwrap());
    assert_eq!(regions.len(), 277);
    let expected: BTreeSet<_> = inventory()
        .iter()
        .map(|(name, _, _)| source_asset(name))
        .collect();
    assert_eq!(expected.len(), 179);
    let actual: BTreeSet<_> = regions[98..]
        .iter()
        .map(|r| r["asset"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(actual, expected);
    let old_set = read_json(root.join("palettes/sets/caldarus-portraits-trial.json"));
    let new_set = read_json(root.join("palettes/sets/caldarus-world-trial.json"));
    for (before, after) in old_set["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(new_set["presets"].as_array().unwrap())
    {
        assert_eq!(before["id"], after["id"]);
        assert_eq!(before["label"], after["label"]);
        assert_eq!(
            before["colors"].as_array().unwrap()[..],
            after["colors"].as_array().unwrap()[..9]
        );
    }
    assert_eq!(
        old_set["presets"].as_array().unwrap().len(),
        new_set["presets"].as_array().unwrap().len()
    );
    for region in &regions[98..] {
        assert!(!region["seeds"].as_array().unwrap().is_empty());
    }
}

#[test]
fn all_six_outfit_packages_preserve_native_frames_timing_offsets_and_atlas() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();
    for (name, frames, duration) in inventory() {
        let stem = format!("spr_npc_caldarus_{name}");
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
    assert_eq!(table[0][0], "caldarus");
    assert_eq!(table[0][4].as_array().unwrap().len(), 179);
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
    assert_eq!(frames, 605);
}

#[test]
#[ignore = "requires the complete Caldarus source corpus; checks all world pixels and retained portraits"]
fn every_world_pixel_matches_reviewed_materials_and_portraits_remain_identical() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/caldarus");
    let set = std::env::var_os("FOM_CALDARUS_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/caldarus-world-trial.json"));
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
    // Literal source-inspected horn/tail exclusions and per-frame totals are independent of recipe seeds.
    let cases: [(&str, &[usize], &[u32]); 179] = [
        (
            "autumn_action_east",
            &[49, 59, 56, 59, 56, 49, 53],
            &[
                15238, 15318, 15398, 15478, 15635, 15716, 15797, 15798, 15877, 15878, 15957, 15958,
                16037, 16038, 16116, 16194, 16195, 16275, 16276, 16357, 16358, 16360, 16368, 16437,
                16438, 16440, 16448, 16517, 16518, 16520, 16528, 16597, 16598, 16600, 16608, 16675,
                16676, 16754, 16755, 16757, 16765, 16835, 16836, 16838, 16846, 16918, 16919, 16920,
                16928, 16998, 16999, 17000, 17008, 17078, 17079, 17080, 17088, 17158, 17159, 17160,
                17168, 17235, 17236, 17238, 17246, 17315, 17316, 17317, 17325, 17396, 17397, 17398,
                17406, 17796, 17797, 17798, 17806, 19153, 19233, 19313, 19393, 19633, 20033, 20111,
                20271, 20272, 20351, 20352, 20431, 20432, 20511, 20512, 20751, 20752, 20830, 20910,
                20990, 21070, 21151, 21152, 21229, 21230, 21310, 21391, 21392, 21471, 21472, 21551,
                21552, 21631, 21632, 21710, 21788, 21871, 21950, 21951, 21952, 22030, 22031, 22032,
                22110, 22111, 22112, 22190, 22191, 22192, 22271, 22349, 22429, 22430, 22431, 22510,
                22590, 22670, 22750, 22829, 22830, 22831, 22907, 22908, 22909, 22989, 23071, 23072,
                23073, 23151, 23152, 23153, 23231, 23232, 23233, 23311, 23312, 23313, 23389, 23467,
                23550, 23551, 23631, 23632, 23711, 23712, 23791, 23792, 23871, 23872, 23950, 23951,
                24028, 24029, 24030, 24110, 24111, 24191, 24193, 24271, 24273, 24351, 24353, 24431,
                24433, 24510, 24511, 24588, 24589, 24670, 24753, 24754, 24833, 24834, 24913, 24914,
                24993, 24994, 25070, 25148, 25150, 25151, 25232, 25314, 25315, 25394, 25395, 25474,
                25475, 25554, 25555, 25632, 25710, 25711, 25712, 25713, 26271, 26272, 26273,
            ],
        ),
        (
            "autumn_action_north",
            &[24, 29, 29, 29, 29, 27, 28],
            &[
                15234, 15245, 15314, 15325, 15394, 15405, 15474, 15485, 15714, 15725, 15794, 15805,
                15874, 15885, 15954, 15965, 16034, 16045, 16114, 16125, 16194, 16205, 16274, 16285,
                16674, 16685, 16754, 16765, 19647, 19728, 19808, 19888, 19968, 20047, 20127, 20288,
                20289, 20368, 20369, 20448, 20449, 20528, 20529, 20768, 20769, 21168, 21169, 21248,
                21249, 21330, 21408, 21409, 21488, 21489, 21568, 21569, 21648, 21649, 21730, 21810,
                21889, 21968, 21969, 21970, 22048, 22049, 22050, 22128, 22129, 22130, 22208, 22209,
                22210, 22289, 22369, 22449, 22450, 22451, 22530, 22610, 22690, 22770, 22849, 22850,
                22851, 22929, 22930, 22931, 23011, 23088, 23089, 23168, 23169, 23248, 23249, 23328,
                23329, 23411, 23491, 23569, 23570, 23647, 23648, 23649, 23650, 23727, 23728, 23729,
                23730, 23807, 23808, 23809, 23810, 23887, 23888, 23889, 23890, 23969, 23970, 24049,
                24050, 24128, 24129, 24130, 24131, 24206, 24209, 24286, 24289, 24366, 24369, 24446,
                24449, 24528, 24529, 24530, 24531, 24608, 24609, 24610, 24611, 24686, 24687, 24690,
                24763, 24764, 24765, 24766, 24767, 24769, 24843, 24844, 24845, 24846, 24847, 24849,
                24923, 24924, 24925, 24926, 24927, 24929, 25003, 25004, 25005, 25006, 25007, 25009,
                25086, 25087, 25090, 25166, 25167, 25170, 25243, 25244, 25245, 25246, 25247, 25248,
                25320, 25321, 25322, 25323, 25324, 25325, 25326, 25327, 25328, 25400, 25401, 25402,
                25403, 25404, 25405, 25406, 25407, 25408, 25480, 25481, 25482, 25483, 25484, 25485,
                25486, 25487, 25488, 25560, 25561, 25562, 25563, 25564, 25565, 25566, 25567, 25568,
                25643, 25644, 25645, 25646, 25647, 25648, 25723, 25724, 25725, 25726, 25727, 25728,
                25800, 25801, 25802, 25803, 25804, 25805, 25806, 25807, 25881, 25882, 25883, 25884,
                25885, 25961, 25962, 25963, 25964, 25965, 26041, 26042, 26043, 26044, 26045, 26121,
                26122, 26123, 26124, 26125, 26200, 26201, 26202, 26203, 26204, 26205, 26206, 26207,
                26280, 26281, 26282, 26283, 26284, 26285, 26286, 26287, 26361, 26362, 26363, 26364,
                26365, 26761, 26762, 26763, 26764, 26765, 26841, 26842, 26843, 26844, 26845,
            ],
        ),
        (
            "autumn_action_south",
            &[54, 58, 58, 58, 58, 54, 61],
            &[
                15235, 15244, 15315, 15324, 15395, 15404, 15475, 15484, 15635, 15644, 15715, 15724,
                15794, 15795, 15804, 15874, 15875, 15884, 15954, 15955, 15964, 16034, 16035, 16044,
                16115, 16124, 16194, 16195, 16204, 16274, 16275, 16284, 16354, 16355, 16364, 16365,
                16434, 16435, 16444, 16445, 16514, 16515, 16524, 16525, 16594, 16595, 16604, 16605,
                16674, 16675, 16684, 16754, 16755, 16764, 16765, 16834, 16835, 16844, 16845, 16915,
                16916, 16924, 16995, 16996, 17004, 17075, 17076, 17084, 17155, 17156, 17164, 17234,
                17235, 17244, 17245, 17315, 17316, 17324, 17395, 17396, 17404, 17795, 17796, 17804,
                19711, 19791, 19871, 19951, 20112, 20192, 20592, 20829, 20830, 20909, 20910, 20989,
                20990, 21069, 21070, 21230, 21231, 21310, 21311, 21388, 21468, 21548, 21628, 21710,
                21711, 21789, 21869, 21949, 21950, 22029, 22030, 22109, 22110, 22189, 22190, 22269,
                22350, 22430, 22508, 22509, 22510, 22588, 22589, 22590, 22668, 22669, 22670, 22748,
                22749, 22750, 22830, 22908, 22909, 22910, 22988, 22989, 22990, 23068, 23148, 23228,
                23308, 23388, 23389, 23390, 23468, 23548, 23629, 23630, 23631, 23709, 23710, 23711,
                23789, 23790, 23791, 23869, 23870, 23871, 23948, 24029, 24030, 24031, 24109, 24110,
                24111, 24189, 24190, 24269, 24270, 24349, 24350, 24429, 24430, 24509, 24510, 24511,
                24589, 24590, 24669, 24670, 24749, 24751, 24752, 24829, 24831, 24832, 24909, 24911,
                24912, 24989, 24991, 24992, 25069, 25070, 25149, 25229, 25231, 25311, 25312, 25313,
                25391, 25392, 25393, 25471, 25472, 25473, 25551, 25552, 25553, 25629, 25631, 25711,
                25791, 25792, 25872, 25952, 26032, 26112, 26191, 26192, 26352, 26752,
            ],
        ),
        (
            "autumn_blink_east",
            &[57, 61, 57],
            &[
                6515, 6595, 6675, 6754, 6755, 6834, 6835, 6914, 6915, 6994, 6995, 6997, 7005, 7074,
                7075, 7077, 7085, 7154, 7155, 7157, 7165, 7235, 7236, 7237, 7245, 7315, 7316, 7317,
                7325, 7395, 7396, 7397, 7405, 8431, 8511, 8591, 8909, 8910, 8989, 8990, 9069, 9070,
                9148, 9228, 9308, 9389, 9469, 9549, 9627, 9628, 9629, 9707, 9708, 9709, 9787, 9788,
                9789, 9867, 9947, 10027, 10108, 10109, 10110, 10188, 10189, 10190, 10268, 10269,
                10270, 10348, 10349, 10428, 10429, 10508, 10509, 10588, 10590, 10591, 10668, 10670,
                10671, 10748, 10750, 10751, 10830, 10831, 10832, 10833, 10910, 10911, 10912, 10913,
                10990, 10991, 10992, 10993, 11071, 11072, 11073, 11151, 11152, 11153, 11231, 11232,
                11233,
            ],
        ),
        (
            "autumn_blink_south",
            &[65, 69, 65],
            &[
                6515, 6524, 6595, 6604, 6675, 6684, 6754, 6755, 6764, 6834, 6835, 6844, 6914, 6915,
                6924, 6994, 6995, 7004, 7005, 7074, 7075, 7084, 7085, 7154, 7155, 7164, 7165, 7235,
                7236, 7244, 7315, 7316, 7324, 7395, 7396, 7404, 8432, 8512, 8592, 8910, 8911, 8990,
                8991, 9070, 9071, 9149, 9229, 9309, 9390, 9470, 9550, 9628, 9629, 9630, 9708, 9709,
                9710, 9788, 9789, 9790, 9868, 9948, 10028, 10109, 10110, 10111, 10189, 10190,
                10191, 10269, 10270, 10271, 10349, 10350, 10429, 10430, 10509, 10510, 10589, 10669,
                10749, 10831, 10911, 10991,
            ],
        ),
        (
            "autumn_drink_east",
            &[48, 56, 48],
            &[
                7235, 7312, 7395, 7474, 7475, 7551, 7552, 7563, 7634, 7635, 7714, 7715, 7717, 7725,
                7792, 7794, 7795, 7803, 7874, 7875, 7877, 7885, 7955, 7956, 7957, 7965, 8033, 8034,
                8035, 8115, 8116, 8117, 8125, 9152, 9153, 9230, 9312, 9313, 9391, 9469, 9551, 9710,
                9870, 9871, 9949, 9950, 10030, 10031, 10110, 10189, 10270,
            ],
        ),
        (
            "autumn_drink_north",
            &[2, 2, 2],
            &[
                7474, 7485, 7634, 7645, 7714, 7725, 7794, 7805, 7874, 7885, 8034, 8045, 8686, 8846,
                9007, 9167, 9168, 9327, 9328, 9409, 9488, 9489, 9569, 9648, 9808, 9888, 9889, 9890,
                9968, 9969, 10048, 10049, 10050, 10130, 10208, 10209, 10210, 10290, 10368, 10369,
                10450, 10528, 10529, 10607, 10608, 10609, 10610, 10688, 10689, 10767, 10768, 10769,
                10770, 10845, 10846, 10849, 10927, 10928, 10929, 10930, 11005, 11006, 11009, 11082,
                11083, 11084, 11085, 11086, 11087, 11164, 11165, 11166, 11169, 11242, 11243, 11244,
                11245, 11246, 11247, 11320, 11321, 11322, 11323, 11324, 11325, 11326, 11400, 11401,
                11402, 11403, 11404, 11405, 11406, 11407, 11480, 11481, 11482, 11483, 11484, 11485,
                11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646, 11647,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "autumn_drink_south",
            &[56, 62, 56],
            &[
                7235, 7244, 7395, 7404, 7474, 7475, 7484, 7555, 7564, 7634, 7635, 7644, 7714, 7715,
                7724, 7725, 7794, 7795, 7804, 7874, 7875, 7884, 7885, 7955, 7956, 7964, 8034, 8035,
                8044, 8045, 8115, 8116, 8124, 8275, 8276, 8284, 8992, 9151, 9152, 9311, 9312, 9390,
                9470, 9471, 9472, 9550, 9631, 9709, 9791, 9869, 9870, 9871, 9950, 9951, 10029,
                10030, 10031, 10109, 10189, 10190, 10191, 10269, 10350, 10351, 10429, 10510, 10511,
                10590, 10591, 10670, 10671, 10750, 10751, 10830, 10910, 10911, 10990, 11072, 11150,
                11152, 11232, 11392, 11393,
            ],
        ),
        (
            "autumn_eat_east",
            &[47, 50, 43, 51, 47],
            &[
                11794, 12035, 12116, 12193, 12194, 12205, 12355, 12434, 12435, 12515, 12516, 12594,
                12596, 12597, 12605, 12675, 12754, 12755, 12834, 12835, 12837, 12845, 12915, 12916,
                12918, 12926, 12995, 12996, 12997, 13074, 13075, 13154, 13155, 13157, 13165, 13235,
                13236, 13237, 13245, 13316, 13317, 13318, 13326, 13474, 13475, 13477, 13485, 13555,
                13556, 13557, 13565, 13875, 13876, 13877, 13885, 14513, 14514, 14592, 14593, 15073,
                15232, 15233, 15312, 15313, 15391, 15392, 15552, 15553, 15631, 15711, 15790, 15871,
                15872, 15873, 15951, 16112, 16191, 16270, 16430, 16431, 16510, 16511, 16512, 16590,
                16591, 16671, 16750, 16751, 16830, 16910, 16990, 17070, 17071, 17150, 17311, 17470,
                17711, 18111,
            ],
        ),
        (
            "autumn_eat_north",
            &[2, 2, 2],
            &[
                7474, 7485, 7634, 7645, 7714, 7725, 7794, 7805, 7874, 7885, 8034, 8045, 8686, 8846,
                9007, 9167, 9168, 9327, 9328, 9409, 9488, 9489, 9569, 9648, 9808, 9888, 9889, 9890,
                9968, 9969, 10048, 10049, 10050, 10130, 10208, 10209, 10210, 10290, 10368, 10369,
                10450, 10528, 10529, 10607, 10608, 10609, 10610, 10688, 10689, 10767, 10768, 10769,
                10770, 10845, 10846, 10849, 10927, 10928, 10929, 10930, 11005, 11006, 11009, 11082,
                11083, 11084, 11085, 11086, 11087, 11164, 11165, 11166, 11169, 11242, 11243, 11244,
                11245, 11246, 11247, 11320, 11321, 11322, 11323, 11324, 11325, 11326, 11400, 11401,
                11402, 11403, 11404, 11405, 11406, 11407, 11480, 11481, 11482, 11483, 11484, 11485,
                11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646, 11647,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "autumn_eat_south",
            &[56, 54, 56, 67, 54],
            &[
                11795, 11804, 12035, 12044, 12194, 12195, 12204, 12355, 12364, 12434, 12435, 12444,
                12515, 12524, 12594, 12595, 12604, 12605, 12675, 12684, 12754, 12755, 12764, 12834,
                12835, 12844, 12845, 12914, 12915, 12924, 12995, 12996, 13004, 13074, 13075, 13084,
                13154, 13155, 13164, 13165, 13235, 13236, 13244, 13314, 13315, 13324, 13325, 13474,
                13475, 13484, 13485, 13555, 13556, 13564, 13715, 13716, 13724, 13875, 13876, 13884,
                14591, 14592, 14912, 15072, 15231, 15232, 15390, 15391, 15392, 15551, 15552, 15630,
                15710, 15711, 15712, 15789, 15870, 15871, 15872, 15950, 16031, 16190, 16191, 16351,
                16429, 16430, 16431, 16510, 16511, 16589, 16590, 16591, 16670, 16671, 16749, 16750,
                16751, 16829, 16909, 16910, 16911, 16989, 17069, 17070, 17071, 17149, 17230, 17231,
                17309, 17390, 17391, 17469, 17550, 17551, 17630, 17631, 17710, 17711, 17790, 17791,
                17870, 17871, 17950, 17951, 18030, 18110, 18111, 18190, 18270, 18271, 18350, 18432,
                18510, 18592, 18670, 18912, 19072,
            ],
        ),
        (
            "autumn_idle_east",
            &[53],
            &[
                2195, 2274, 2275, 2354, 2355, 2357, 2365, 2435, 2436, 2437, 2445, 2831, 2989, 2990,
                3068, 3149, 3227, 3228, 3229, 3307, 3388, 3389, 3390, 3468, 3469, 3548, 3550, 3551,
                3630, 3631, 3632, 3633, 3711, 3712, 3713,
            ],
        ),
        (
            "autumn_idle_north",
            &[28],
            &[
                2274, 2285, 2354, 2365, 2847, 3008, 3009, 3090, 3169, 3249, 3250, 3251, 3331, 3409,
                3410, 3488, 3489, 3490, 3491, 3566, 3567, 3570, 3643, 3644, 3645, 3646, 3647, 3648,
                3720, 3721, 3722, 3723, 3724, 3725, 3726, 3727, 3801, 3802, 3803, 3804, 3805,
            ],
        ),
        (
            "autumn_idle_south",
            &[61],
            &[
                2195, 2204, 2274, 2275, 2284, 2354, 2355, 2364, 2365, 2435, 2436, 2444, 2832, 2990,
                2991, 3069, 3150, 3228, 3229, 3230, 3308, 3389, 3390, 3391, 3469, 3470, 3549, 3631,
            ],
        ),
        (
            "autumn_kiss_east",
            &[49, 51, 64, 55],
            &[
                8838, 8994, 9076, 9157, 9158, 9236, 9313, 9314, 9395, 9396, 9477, 9478, 9480, 9488,
                9555, 9556, 9633, 9634, 9636, 9644, 9715, 9716, 9718, 9726, 9798, 9799, 9800, 9808,
                9875, 9876, 9878, 9886, 9954, 9955, 9956, 9964, 10036, 10037, 10038, 10046, 10196,
                10197, 10198, 10206, 11070, 11550, 11630, 11710, 11711, 11790, 12029, 12188, 12189,
                12269, 12270, 12350, 12351, 12429, 12430, 12507, 12588, 12669, 12670, 12671, 12748,
                12828, 12829, 12909, 12910, 12989, 13069, 13070, 13147, 13148, 13149, 13228, 13229,
                13230, 13310, 13311, 13388, 13389, 13390, 13467, 13548, 13630, 13631, 13708, 13788,
                13789, 13869, 13870, 13950, 13952, 14029, 14030, 14107, 14108, 14109, 14189, 14190,
                14272, 14273, 14349, 14350, 14428, 14430, 14509, 14511, 14592, 14593, 14669, 14671,
                14750, 14751, 14752, 14831, 14832, 14833, 14991, 14992, 14993, 15070, 15071, 15072,
                15151, 15152, 15153, 15311, 15312, 15313,
            ],
        ),
        (
            "autumn_sit_east",
            &[46],
            &[
                2435, 2514, 2515, 2594, 2595, 2597, 2605, 2675, 2676, 2677, 2685, 3072, 3073, 3151,
                3310, 3311, 3390,
            ],
        ),
        (
            "autumn_sit_north",
            &[2],
            &[
                2514, 2525, 2594, 2605, 2926, 3087, 3088, 3169, 3248, 3328, 3329, 3330, 3410, 3488,
                3489, 3567, 3568, 3569, 3570, 3645, 3646, 3649, 3722, 3723, 3724, 3725, 3726, 3727,
                3800, 3801, 3802, 3803, 3804, 3805, 3806, 3881, 3882, 3883, 3884,
            ],
        ),
        (
            "autumn_sit_south",
            &[54],
            &[
                2435, 2444, 2514, 2515, 2524, 2594, 2595, 2604, 2605, 2675, 2676, 2684, 3071, 3072,
                3150, 3231, 3309, 3310, 3311, 3389, 3470, 3471, 3550, 3551, 3630,
            ],
        ),
        (
            "autumn_sleep_east",
            &[52],
            &[
                2275, 2354, 2355, 2434, 2435, 2437, 2445, 2515, 2516, 2517, 2525,
            ],
        ),
        (
            "autumn_walk_east",
            &[53, 54, 53, 53],
            &[
                8675, 8835, 8994, 8995, 9075, 9154, 9155, 9235, 9314, 9315, 9317, 9325, 9394, 9395,
                9474, 9475, 9477, 9485, 9554, 9555, 9635, 9636, 9637, 9645, 9714, 9715, 9717, 9725,
                9795, 9796, 9797, 9805, 9874, 9875, 9877, 9885, 10035, 10036, 10037, 10045, 10195,
                10196, 10197, 10205, 11231, 11391, 11632, 11869, 11870, 12029, 12030, 12109, 12188,
                12270, 12271, 12348, 12428, 12429, 12509, 12589, 12669, 12827, 12828, 12829, 12910,
                12987, 12988, 12989, 13068, 13069, 13147, 13228, 13229, 13230, 13307, 13387, 13388,
                13389, 13468, 13469, 13470, 13548, 13628, 13629, 13630, 13707, 13788, 13789, 13869,
                13870, 13948, 13949, 14028, 14029, 14108, 14110, 14111, 14188, 14189, 14268, 14270,
                14271, 14347, 14348, 14349, 14430, 14431, 14432, 14433, 14510, 14511, 14590, 14591,
                14592, 14593, 14668, 14670, 14671, 14751, 14752, 14753, 14830, 14831, 14832, 14833,
                14834, 14911, 14912, 14913, 14988, 14990, 14991, 14992, 14993, 14994, 15151, 15152,
                15153, 15309, 15310, 15311, 15312, 15313, 15473, 15631, 15632,
            ],
        ),
        (
            "autumn_walk_north",
            &[28, 20, 28, 23],
            &[
                8994, 9005, 9154, 9165, 9314, 9325, 9394, 9405, 9474, 9485, 9554, 9565, 9714, 9725,
                9874, 9885, 11247, 11407, 11646, 11888, 11889, 12048, 12049, 12128, 12210, 12287,
                12288, 12370, 12448, 12449, 12529, 12609, 12689, 12849, 12850, 12851, 12928, 13009,
                13010, 13011, 13088, 13089, 13171, 13248, 13249, 13250, 13331, 13408, 13409, 13410,
                13489, 13490, 13570, 13649, 13650, 13730, 13808, 13809, 13810, 13811, 13888, 13889,
                13968, 13969, 13970, 13971, 14048, 14049, 14126, 14127, 14130, 14207, 14209, 14210,
                14286, 14287, 14290, 14367, 14368, 14369, 14370, 14443, 14444, 14445, 14446, 14447,
                14448, 14526, 14527, 14528, 14603, 14604, 14605, 14606, 14607, 14608, 14686, 14689,
                14760, 14761, 14762, 14763, 14764, 14765, 14766, 14767, 14843, 14844, 14845, 14846,
                14847, 14848, 14920, 14921, 14922, 14923, 14924, 14925, 14926, 14927, 15003, 15004,
                15005, 15006, 15007, 15009, 15081, 15082, 15083, 15084, 15085, 15160, 15161, 15162,
                15163, 15164, 15165, 15166, 15167, 15241, 15242, 15243, 15244, 15245, 15320, 15321,
                15322, 15323, 15324, 15325, 15326, 15327, 15328, 15481, 15482, 15483, 15484, 15485,
                15641, 15642, 15643, 15644, 15645,
            ],
        ),
        (
            "autumn_walk_south",
            &[61, 55, 61, 54],
            &[
                8675, 8684, 8835, 8844, 8994, 8995, 9004, 9075, 9084, 9154, 9155, 9164, 9235, 9244,
                9314, 9315, 9324, 9325, 9394, 9395, 9404, 9474, 9475, 9484, 9485, 9554, 9555, 9564,
                9635, 9636, 9644, 9714, 9715, 9724, 9725, 9795, 9796, 9804, 9874, 9875, 9884, 9885,
                10035, 10036, 10044, 10195, 10196, 10204, 11232, 11392, 11870, 11871, 12030, 12031,
                12111, 12189, 12271, 12272, 12349, 12430, 12431, 12510, 12590, 12670, 12828, 12829,
                12830, 12911, 12988, 12989, 12990, 13070, 13148, 13229, 13230, 13231, 13308, 13389,
                13390, 13469, 13470, 13471, 13549, 13629, 13630, 13631, 13709, 13789, 13790, 13870,
                13871, 13949, 13950, 14030, 14031, 14109, 14189, 14190, 14269, 14349, 14350, 14431,
                14511, 14591, 14671, 14672, 14831, 14832, 14991, 14992, 14993, 15152, 15153, 15311,
                15312,
            ],
        ),
        (
            "specialanimation_autumn_eyes_closed_east",
            &[61],
            &[
                2195, 2274, 2275, 2354, 2355, 2357, 2365, 2435, 2436, 2437, 2445, 2831, 2989, 2990,
                3068, 3149, 3227, 3228, 3229, 3307, 3388, 3389, 3390, 3468, 3469, 3548, 3550, 3551,
                3630, 3631, 3632, 3633, 3711, 3712, 3713,
            ],
        ),
        (
            "specialanimation_autumn_eyes_closed_south",
            &[69],
            &[
                2195, 2204, 2274, 2275, 2284, 2354, 2355, 2364, 2365, 2435, 2436, 2444, 2832, 2990,
                2991, 3069, 3150, 3228, 3229, 3230, 3308, 3389, 3390, 3391, 3469, 3470, 3549, 3631,
            ],
        ),
        (
            "specialanimation_autumn_harvest_east",
            &[56, 69, 74, 73, 74, 73, 74, 73, 69, 56],
            &[],
        ),
        (
            "specialanimation_autumn_lute_play_end_south",
            &[40, 56],
            &[
                4355, 4364, 4435, 4514, 4515, 4524, 4594, 4595, 4674, 4675, 4684, 4685, 4754, 4755,
                4835, 4844, 4915, 4916, 5632, 5712, 5950, 5951, 6030, 6031, 6109, 6189, 6270, 6350,
                6428, 6429, 6430, 6508, 6509, 6510, 6588, 6668, 6749, 6750, 6751, 6829, 6830, 6831,
                6909, 6910, 6989, 6990, 7069, 7071, 7072, 7149, 7151, 7152, 7231, 7232, 7233, 7311,
                7312, 7313, 7392, 7472,
            ],
        ),
        (
            "specialanimation_autumn_lute_play_loop_south",
            &[75, 75, 75, 70, 70, 70, 75, 75, 75, 75],
            &[
                21635, 21644, 21715, 21724, 21795, 21804, 22115, 22124, 22195, 22204, 22275, 22284,
                22355, 22364, 22434, 22435, 22444, 22514, 22515, 22524, 22594, 22595, 22604, 22675,
                22684, 22755, 22764, 22835, 22844, 22914, 22915, 22924, 22994, 22995, 23004, 23074,
                23075, 23084, 23154, 23155, 23164, 23234, 23235, 23244, 23245, 23314, 23315, 23324,
                23325, 23394, 23395, 23404, 23405, 23474, 23475, 23484, 23554, 23555, 23564, 23634,
                23635, 23644, 23714, 23715, 23724, 23725, 23794, 23795, 23804, 23805, 23874, 23875,
                23884, 23885, 23954, 23955, 23964, 23965, 24035, 24036, 24044, 24115, 24116, 24124,
                24195, 24196, 24204, 24274, 24275, 24284, 24285, 24354, 24355, 24364, 24365, 24434,
                24435, 24444, 24445, 24515, 24516, 24524, 24595, 24596, 24604, 24675, 24676, 24684,
                24755, 24756, 24764, 25075, 25076, 25084, 25155, 25156, 25164, 25235, 25236, 25244,
                29630, 29631, 29710, 29711, 29790, 29791, 30110, 30111, 30190, 30191, 30270, 30271,
                30350, 30351, 30429, 30509, 30589, 30670, 30750, 30830, 30909, 30989, 31069, 31149,
                31230, 31310, 31390, 31469, 31549, 31629, 31710, 31790, 31870, 31950, 32028, 32029,
                32030, 32108, 32109, 32110, 32188, 32189, 32190, 32270, 32350, 32430, 32508, 32509,
                32510, 32588, 32589, 32590, 32668, 32669, 32670, 32748, 32749, 32750, 32828, 32908,
                32988, 33068, 33069, 33070, 33148, 33149, 33150, 33228, 33229, 33230, 33308, 33388,
                33468, 33548, 33629, 33630, 33631, 33709, 33710, 33711, 33789, 33790, 33791, 33868,
                33948, 34028, 34109, 34110, 34111, 34189, 34190, 34191, 34269, 34270, 34271, 34349,
                34350, 34351, 34429, 34430, 34432, 34509, 34510, 34512, 34589, 34590, 34592, 34669,
                34670, 34671, 34749, 34750, 34751, 34829, 34830, 34831, 34909, 34910, 34912, 34989,
                34990, 34992, 35069, 35070, 35072, 35149, 35150, 35152, 35229, 35231, 35232, 35233,
                35309, 35311, 35312, 35313, 35389, 35391, 35392, 35393, 35469, 35470, 35472, 35549,
                35550, 35552, 35629, 35630, 35632, 35709, 35711, 35712, 35713, 35789, 35791, 35792,
                35793, 35869, 35871, 35872, 35873, 35949, 35951, 35952, 35953, 36031, 36032, 36111,
                36112, 36191, 36192, 36269, 36271, 36272, 36273, 36349, 36351, 36352, 36353, 36429,
                36431, 36432, 36433, 36511, 36512, 36591, 36592, 36671, 36672, 36751, 36752, 37071,
                37072, 37151, 37152, 37231, 37232,
            ],
        ),
        (
            "specialanimation_autumn_lute_play_start_south",
            &[56, 40],
            &[
                4355, 4435, 4444, 4514, 4515, 4594, 4595, 4604, 4674, 4675, 4754, 4755, 4764, 4765,
                4835, 4836, 4915, 4924, 5632, 5712, 5950, 5951, 6030, 6031, 6109, 6189, 6270, 6350,
                6428, 6429, 6430, 6508, 6509, 6510, 6588, 6668, 6749, 6750, 6751, 6829, 6830, 6831,
                6909, 6910, 6989, 6990, 7069, 7071, 7072, 7149, 7151, 7152, 7231, 7232, 7233, 7311,
                7312, 7313, 7392, 7472,
            ],
        ),
        (
            "specialanimation_autumn_read_end_south",
            &[48, 34, 55],
            &[
                6675, 6684, 6755, 6764, 6835, 6844, 6914, 6915, 6924, 6994, 6995, 7004, 7074, 7075,
                7084, 7154, 7155, 7164, 7165, 7234, 7235, 7244, 7245, 7314, 7315, 7324, 7325, 7395,
                7396, 7404, 7475, 7476, 7484, 7555, 7556, 7564, 8592, 8672, 8752, 9070, 9071, 9150,
                9151, 9230, 9231, 9309, 9389, 9469, 9550, 9630, 9710, 9788, 9789, 9790, 9868, 9869,
                9870, 9948, 9949, 9950, 10028, 10108, 10188, 10269, 10270, 10271, 10349, 10350,
                10351, 10429, 10430, 10431, 10509, 10510, 10589, 10590, 10669, 10670, 10672, 10749,
                10829, 10831, 10909, 10911, 10912, 10913, 10991, 11071, 11151, 11152, 11153, 11392,
            ],
        ),
        (
            "specialanimation_autumn_read_loop_south",
            &[44, 53, 44, 53],
            &[
                8675, 8844, 8994, 8995, 9075, 9084, 9164, 9165, 9235, 9244, 9314, 9315, 9317, 9325,
                9394, 9395, 9404, 9474, 9482, 9484, 9485, 9554, 9555, 9564, 9635, 9636, 9637, 9645,
                9714, 9715, 9724, 9725, 9794, 9802, 9803, 9804, 9874, 9875, 9884, 9885, 10035,
                10036, 10044, 10195, 10196, 10204, 11231, 11869, 11870, 12188, 12270, 12430, 12509,
                12589, 12749, 12827, 12828, 12829, 12910, 13070, 13147, 13228, 13229, 13230, 13388,
                13389, 13390, 13468, 13469, 13470, 13548, 13708, 13788, 13789, 13869, 13870, 14029,
                14030, 14108, 14110, 14111, 14189, 14190, 14349, 14350, 14430, 14431, 14432, 14509,
                14669, 14751, 14752,
            ],
        ),
        (
            "specialanimation_autumn_read_sit_end_south",
            &[38, 34, 50],
            &[
                7395, 7404, 7475, 7484, 7555, 7564, 7634, 7635, 7644, 7714, 7715, 7724, 7794, 7795,
                7804, 7874, 7875, 7884, 7885, 7954, 7955, 7964, 7965, 8034, 8035, 8044, 8045, 8115,
                8116, 8124, 8195, 8196, 8204, 8275, 8276, 8284, 8912, 8992, 9311, 9312, 9390, 9391,
                9392, 9470, 9471, 9472, 9550, 9791, 9870, 9871, 9950, 9951, 10029, 10030, 10031,
                10109, 10110, 10111, 10189, 10190, 10191, 10269, 10349, 10429, 10510, 10511, 10590,
                10591, 10670, 10671, 10750, 10751, 10830, 10831, 10910, 10911, 10990, 11070, 11150,
                11392,
            ],
        ),
        (
            "specialanimation_autumn_read_sit_loop_south",
            &[31, 40, 30, 40],
            &[
                9635, 9804, 9954, 9955, 10035, 10044, 10124, 10125, 10195, 10204, 10274, 10275,
                10277, 10285, 10354, 10355, 10364, 10434, 10442, 10444, 10445, 10514, 10515, 10524,
                10595, 10596, 10597, 10605, 10674, 10675, 10684, 10685, 10754, 10762, 10763, 10764,
                10834, 10835, 10844, 10845, 10995, 10996, 11004, 11155, 11156, 11164, 11552, 11952,
                12112, 12190, 12191, 12509, 12590, 12591, 12750, 12751, 12830, 13148, 13149, 13150,
                13230, 13390, 13468, 13549, 13550, 13709, 13710, 13789, 13790, 13869, 14029, 14109,
                14110, 14190, 14350, 14429, 14510, 14670, 14751, 14830, 14990,
            ],
        ),
        (
            "specialanimation_autumn_read_sit_start_south",
            &[50, 38, 34],
            &[
                7235, 7244, 7474, 7475, 7484, 7555, 7564, 7635, 7644, 7714, 7715, 7724, 7725, 7794,
                7795, 7804, 7874, 7875, 7884, 7955, 7956, 7964, 8034, 8035, 8044, 8045, 8114, 8115,
                8124, 8125, 8275, 8276, 8284, 8355, 8356, 8364, 8992, 9072, 9151, 9152, 9390, 9470,
                9471, 9472, 9550, 9551, 9552, 9631, 9869, 9870, 9871, 9950, 9951, 10030, 10031,
                10109, 10189, 10190, 10191, 10269, 10270, 10271, 10350, 10351, 10429, 10509, 10590,
                10591, 10670, 10671, 10750, 10751, 10830, 10910, 10911, 10990, 10991, 11150, 11230,
                11392,
            ],
        ),
        (
            "specialanimation_autumn_read_start_south",
            &[55, 38, 44],
            &[
                6515, 6524, 6754, 6755, 6764, 6835, 6844, 6915, 6924, 6994, 6995, 7004, 7005, 7074,
                7075, 7084, 7154, 7155, 7164, 7235, 7236, 7244, 7314, 7315, 7324, 7325, 7394, 7395,
                7404, 7405, 7555, 7556, 7564, 7635, 7636, 7644, 8432, 8752, 8832, 8910, 8911, 9149,
                9230, 9231, 9310, 9311, 9390, 9469, 9549, 9628, 9629, 9630, 9710, 9790, 9868, 9948,
                9949, 9950, 10028, 10029, 10030, 10109, 10110, 10111, 10188, 10268, 10349, 10350,
                10429, 10430, 10431, 10509, 10510, 10511, 10589, 10669, 10670, 10672, 10749, 10750,
                10831, 10909, 10911, 10912, 10913, 10989, 10991, 11151, 11152, 11153, 11231, 11392,
            ],
        ),
        (
            "specialanimation_autumn_spell_cast_end_south",
            &[65],
            &[
                2275, 2284, 2354, 2355, 2364, 2434, 2435, 2444, 2445, 2515, 2516, 2524, 2912, 3070,
                3071, 3149, 3230, 3308, 3309, 3310, 3388, 3469, 3470, 3471, 3549, 3550, 3629, 3711,
            ],
        ),
        (
            "specialanimation_autumn_spell_cast_loop_south",
            &[80, 79, 82, 83],
            &[
                8675, 8684, 8755, 8764, 8835, 8844, 8915, 8924, 8994, 8995, 9004, 9074, 9075, 9084,
                9154, 9155, 9164, 9234, 9235, 9244, 9314, 9315, 9324, 9325, 9394, 9395, 9404, 9405,
                9474, 9475, 9484, 9485, 9554, 9555, 9564, 9565, 9635, 9636, 9644, 9715, 9716, 9724,
                9795, 9796, 9804, 9875, 9876, 9884, 11311, 11551, 11949, 11950, 12268, 12508,
                12908, 12988, 13068, 13148, 13149, 13228, 13308, 13388, 13468, 13549, 13550, 13629,
                13630, 13709, 13710, 13789, 13790, 13869, 13870, 13872, 13949, 13950, 13952, 14029,
                14030, 14108, 14109, 14111, 14112, 14189, 14191, 14192, 14193, 14269, 14271, 14272,
                14273, 14349, 14351, 14429, 14431, 14432, 14433, 14511, 14512, 14513, 14591, 14592,
                14671, 14672, 14751, 14752, 14832, 14912, 14992,
            ],
        ),
        (
            "specialanimation_autumn_spell_cast_start_south",
            &[65, 72],
            &[
                4435, 4444, 4515, 4524, 4594, 4595, 4604, 4674, 4675, 4684, 4754, 4755, 4764, 4765,
                4834, 4835, 4844, 4845, 4915, 4916, 4924, 4995, 4996, 5004, 5712, 5792, 6030, 6031,
                6110, 6111, 6189, 6269, 6430, 6508, 6509, 6588, 6589, 6590, 6668, 6748, 6909, 6910,
                6911, 6989, 7069, 7070, 7149, 7151, 7152, 7153, 7229, 7311, 7312, 7313, 7391, 7472,
            ],
        ),
        (
            "beach_action_east",
            &[66, 76, 69, 76, 69, 66, 74],
            &[
                15238, 15318, 15398, 15478, 15635, 15716, 15797, 15798, 15877, 15878, 15957, 15958,
                16037, 16038, 16116, 16194, 16195, 16275, 16276, 16357, 16358, 16360, 16368, 16437,
                16438, 16440, 16448, 16517, 16518, 16520, 16528, 16597, 16598, 16600, 16608, 16675,
                16676, 16754, 16755, 16757, 16765, 16835, 16836, 16838, 16846, 16918, 16919, 16920,
                16928, 16998, 16999, 17000, 17008, 17078, 17079, 17080, 17088, 17158, 17159, 17160,
                17168, 17235, 17236, 17238, 17246, 17315, 17316, 17317, 17325, 17396, 17397, 17398,
                17406, 17796, 17797, 17798, 17806, 19153, 19233, 19313, 19393, 19633, 20033, 20111,
                20271, 20272, 20351, 20352, 20431, 20432, 20511, 20512, 20751, 20752, 20830, 20910,
                20990, 21070, 21151, 21152, 21229, 21230, 21310, 21391, 21392, 21471, 21472, 21551,
                21552, 21631, 21632, 21710, 21788, 21871, 21950, 21951, 21952, 22030, 22031, 22032,
                22110, 22111, 22112, 22190, 22191, 22192, 22271, 22349, 22429, 22430, 22431, 22510,
                22590, 22670, 22750, 22829, 22830, 22831, 22907, 22908, 22909, 22989, 23071, 23072,
                23073, 23151, 23152, 23153, 23231, 23232, 23233, 23311, 23312, 23313, 23389, 23467,
                23550, 23551, 23552, 23631, 23632, 23634, 23711, 23712, 23714, 23791, 23792, 23794,
                23871, 23872, 23874, 23950, 23951, 23952, 24028, 24029, 24030, 24110, 24111, 24113,
                24191, 24193, 24194, 24195, 24196, 24271, 24273, 24274, 24275, 24276, 24351, 24353,
                24354, 24355, 24356, 24431, 24433, 24434, 24435, 24436, 24510, 24511, 24513, 24588,
                24589, 24591, 24670, 24672, 24673, 24674, 24675, 24753, 24754, 24755, 24756, 24757,
                24833, 24834, 24835, 24836, 24837, 24913, 24914, 24915, 24916, 24917, 24993, 24994,
                24995, 24996, 24997, 25070, 25072, 25073, 25074, 25075, 25148, 25150, 25151, 25152,
                25232, 25233, 25234, 25235, 25236, 25237, 25314, 25315, 25316, 25394, 25395, 25396,
                25474, 25475, 25476, 25554, 25555, 25556, 25632, 25633, 25634, 25635, 25636, 25637,
                25710, 25711, 25712, 25713, 25793, 25794, 25795, 25796, 25876, 25956, 26036, 26116,
                26193, 26194, 26195, 26196, 26271, 26272, 26273, 26274, 26275, 26355, 26356, 26755,
                26756, 26833, 26834, 26835,
            ],
        ),
        (
            "beach_action_north",
            &[35, 33, 33, 33, 33, 34, 35],
            &[
                15234, 15245, 15314, 15325, 15394, 15405, 15474, 15485, 15714, 15725, 15794, 15805,
                15874, 15885, 15954, 15965, 16034, 16045, 16114, 16125, 16194, 16205, 16274, 16285,
                16674, 16685, 16754, 16765, 19647, 19728, 19808, 19888, 19968, 20047, 20127, 20288,
                20289, 20368, 20369, 20448, 20449, 20528, 20529, 20768, 20769, 21168, 21169, 21248,
                21249, 21330, 21408, 21409, 21488, 21489, 21568, 21569, 21648, 21649, 21730, 21810,
                21889, 21968, 21969, 21970, 22048, 22049, 22050, 22128, 22129, 22130, 22208, 22209,
                22210, 22289, 22369, 22449, 22450, 22451, 22530, 22610, 22690, 22770, 22849, 22850,
                22851, 22929, 22930, 22931, 23011, 23088, 23089, 23168, 23169, 23248, 23249, 23328,
                23329, 23411, 23491, 23569, 23570, 23647, 23648, 23649, 23650, 23727, 23728, 23729,
                23730, 23807, 23808, 23809, 23810, 23887, 23888, 23889, 23890, 23969, 23970, 24049,
                24050, 24128, 24129, 24130, 24131, 24206, 24209, 24286, 24289, 24366, 24369, 24446,
                24449, 24528, 24529, 24530, 24531, 24608, 24609, 24610, 24611, 24686, 24687, 24690,
                24763, 24764, 24765, 24766, 24767, 24769, 24843, 24844, 24845, 24846, 24847, 24849,
                24923, 24924, 24925, 24926, 24927, 24929, 25003, 25004, 25005, 25006, 25007, 25009,
                25086, 25087, 25090, 25166, 25167, 25170, 25243, 25244, 25245, 25246, 25247, 25248,
                25320, 25321, 25322, 25323, 25324, 25325, 25326, 25327, 25328, 25400, 25401, 25402,
                25403, 25404, 25405, 25406, 25407, 25408, 25480, 25481, 25482, 25483, 25484, 25485,
                25486, 25487, 25488, 25560, 25561, 25562, 25563, 25564, 25565, 25566, 25567, 25568,
                25643, 25644, 25645, 25646, 25647, 25648, 25723, 25724, 25725, 25726, 25727, 25728,
                25800, 25801, 25802, 25803, 25804, 25805, 25806, 25807, 25881, 25882, 25883, 25884,
                25885, 25961, 25962, 25963, 25964, 25965, 26041, 26042, 26043, 26044, 26045, 26121,
                26122, 26123, 26124, 26125, 26200, 26201, 26202, 26203, 26204, 26205, 26206, 26207,
                26280, 26281, 26282, 26283, 26284, 26285, 26286, 26287, 26361, 26362, 26363, 26364,
                26365, 26761, 26762, 26763, 26764, 26765, 26841, 26842, 26843, 26844, 26845,
            ],
        ),
        (
            "beach_action_south",
            &[74, 76, 72, 76, 72, 74, 85],
            &[
                15235, 15244, 15315, 15324, 15395, 15404, 15475, 15484, 15635, 15644, 15715, 15724,
                15794, 15795, 15804, 15874, 15875, 15884, 15954, 15955, 15964, 16034, 16035, 16044,
                16115, 16124, 16194, 16195, 16204, 16274, 16275, 16284, 16354, 16355, 16364, 16365,
                16434, 16435, 16444, 16445, 16514, 16515, 16524, 16525, 16594, 16595, 16604, 16605,
                16674, 16675, 16684, 16754, 16755, 16764, 16765, 16834, 16835, 16844, 16845, 16915,
                16916, 16924, 16995, 16996, 17004, 17075, 17076, 17084, 17155, 17156, 17164, 17234,
                17235, 17244, 17245, 17315, 17316, 17324, 17395, 17396, 17404, 17795, 17796, 17804,
                19711, 19791, 19871, 19951, 20112, 20192, 20592, 20829, 20830, 20909, 20910, 20989,
                20990, 21069, 21070, 21230, 21231, 21310, 21311, 21388, 21468, 21548, 21628, 21710,
                21711, 21789, 21869, 21949, 21950, 22029, 22030, 22109, 22110, 22189, 22190, 22269,
                22350, 22430, 22508, 22509, 22510, 22588, 22589, 22590, 22668, 22669, 22670, 22748,
                22749, 22750, 22830, 22908, 22909, 22910, 22988, 22989, 22990, 23068, 23148, 23228,
                23308, 23388, 23389, 23390, 23468, 23548, 23629, 23630, 23631, 23709, 23710, 23711,
                23789, 23790, 23791, 23869, 23870, 23871, 23948, 24029, 24030, 24031, 24109, 24110,
                24111, 24189, 24190, 24192, 24269, 24270, 24272, 24349, 24350, 24352, 24429, 24430,
                24432, 24509, 24510, 24511, 24589, 24590, 24669, 24670, 24749, 24751, 24752, 24829,
                24831, 24832, 24833, 24834, 24909, 24911, 24912, 24989, 24991, 24992, 24993, 24994,
                25069, 25070, 25149, 25229, 25231, 25311, 25312, 25313, 25391, 25392, 25393, 25394,
                25471, 25472, 25473, 25551, 25552, 25553, 25554, 25629, 25631, 25711, 25791, 25792,
                25872, 25873, 25874, 25952, 25953, 25954, 26032, 26033, 26034, 26112, 26113, 26114,
                26191, 26192, 26352, 26353, 26354, 26434, 26514, 26594, 26674, 26752, 26753, 26754,
                26914, 27314,
            ],
        ),
        ("beach_bath_swim_east", &[47, 47, 41, 41], &[]),
        ("beach_bath_swim_south", &[51, 51, 45, 45], &[]),
        (
            "beach_blink_east",
            &[78, 82, 78],
            &[
                6515, 6595, 6675, 6754, 6755, 6834, 6835, 6914, 6915, 6994, 6995, 6997, 7005, 7074,
                7075, 7077, 7085, 7154, 7155, 7157, 7165, 7235, 7236, 7237, 7245, 7315, 7316, 7317,
                7325, 7395, 7396, 7397, 7405, 8431, 8511, 8591, 8909, 8910, 8989, 8990, 9069, 9070,
                9148, 9228, 9308, 9389, 9469, 9549, 9627, 9628, 9629, 9707, 9708, 9709, 9787, 9788,
                9789, 9867, 9947, 10027, 10108, 10109, 10110, 10188, 10189, 10190, 10268, 10269,
                10270, 10348, 10349, 10351, 10428, 10429, 10431, 10508, 10509, 10511, 10588, 10590,
                10591, 10592, 10668, 10670, 10671, 10672, 10748, 10750, 10751, 10752, 10830, 10831,
                10832, 10833, 10910, 10911, 10912, 10913, 10990, 10991, 10992, 10993, 11071, 11072,
                11073, 11074, 11075, 11151, 11152, 11153, 11154, 11155, 11231, 11232, 11233, 11234,
                11235, 11313, 11314, 11315, 11393, 11394, 11395, 11473, 11474, 11475,
            ],
        ),
        (
            "beach_blink_south",
            &[89, 93, 89],
            &[
                6515, 6524, 6595, 6604, 6675, 6684, 6754, 6755, 6764, 6834, 6835, 6844, 6914, 6915,
                6924, 6994, 6995, 7004, 7005, 7074, 7075, 7084, 7085, 7154, 7155, 7164, 7165, 7235,
                7236, 7244, 7315, 7316, 7324, 7395, 7396, 7404, 8432, 8512, 8592, 8910, 8911, 8990,
                8991, 9070, 9071, 9149, 9229, 9309, 9390, 9470, 9550, 9628, 9629, 9630, 9708, 9709,
                9710, 9788, 9789, 9790, 9868, 9948, 10028, 10109, 10110, 10111, 10189, 10190,
                10191, 10269, 10270, 10271, 10349, 10350, 10429, 10430, 10509, 10510, 10589, 10669,
                10749, 10831, 10911, 10991,
            ],
        ),
        (
            "beach_idle_east",
            &[74],
            &[
                2195, 2274, 2275, 2354, 2355, 2357, 2365, 2435, 2436, 2437, 2445, 2831, 2989, 2990,
                3068, 3149, 3227, 3228, 3229, 3307, 3388, 3389, 3390, 3468, 3469, 3471, 3548, 3550,
                3551, 3552, 3630, 3631, 3632, 3633, 3711, 3712, 3713, 3714, 3715, 3793, 3794, 3795,
            ],
        ),
        (
            "beach_idle_north",
            &[35],
            &[
                2274, 2285, 2354, 2365, 2847, 3008, 3009, 3090, 3169, 3249, 3250, 3251, 3331, 3409,
                3410, 3488, 3489, 3490, 3491, 3566, 3567, 3570, 3643, 3644, 3645, 3646, 3647, 3648,
                3720, 3721, 3722, 3723, 3724, 3725, 3726, 3727, 3801, 3802, 3803, 3804, 3805,
            ],
        ),
        (
            "beach_idle_south",
            &[85],
            &[
                2195, 2204, 2274, 2275, 2284, 2354, 2355, 2364, 2365, 2435, 2436, 2444, 2832, 2990,
                2991, 3069, 3150, 3228, 3229, 3230, 3308, 3389, 3390, 3391, 3469, 3470, 3549, 3631,
            ],
        ),
        (
            "beach_kiss_east",
            &[69, 77, 90, 80],
            &[
                8838, 8994, 9076, 9157, 9158, 9236, 9313, 9314, 9395, 9396, 9477, 9478, 9480, 9488,
                9555, 9556, 9633, 9634, 9636, 9644, 9715, 9716, 9718, 9726, 9798, 9799, 9800, 9808,
                9875, 9876, 9878, 9886, 9954, 9955, 9956, 9964, 10036, 10037, 10038, 10046, 10196,
                10197, 10198, 10206, 11070, 11550, 11630, 11710, 11711, 11790, 12029, 12188, 12189,
                12269, 12270, 12350, 12351, 12429, 12430, 12507, 12588, 12669, 12670, 12671, 12748,
                12828, 12829, 12909, 12910, 12989, 13069, 13070, 13147, 13148, 13149, 13228, 13229,
                13230, 13310, 13311, 13312, 13388, 13389, 13390, 13467, 13548, 13630, 13631, 13633,
                13708, 13788, 13789, 13869, 13870, 13950, 13952, 13953, 14029, 14030, 14107, 14108,
                14109, 14189, 14190, 14272, 14273, 14274, 14349, 14350, 14428, 14430, 14431, 14509,
                14511, 14512, 14513, 14592, 14593, 14594, 14595, 14596, 14669, 14671, 14672, 14673,
                14750, 14751, 14752, 14753, 14831, 14832, 14833, 14834, 14913, 14914, 14915, 14991,
                14992, 14993, 14994, 15070, 15071, 15072, 15073, 15074, 15151, 15152, 15153, 15154,
                15155, 15235, 15311, 15312, 15313, 15314, 15315, 15392, 15393, 15394, 15474, 15475,
                15634, 15635,
            ],
        ),
        (
            "beach_walk_east",
            &[74, 74, 74, 74],
            &[
                8675, 8835, 8994, 8995, 9075, 9154, 9155, 9235, 9314, 9315, 9317, 9325, 9394, 9395,
                9474, 9475, 9477, 9485, 9554, 9555, 9635, 9636, 9637, 9645, 9714, 9715, 9717, 9725,
                9795, 9796, 9797, 9805, 9874, 9875, 9877, 9885, 10035, 10036, 10037, 10045, 10195,
                10196, 10197, 10205, 11231, 11391, 11632, 11869, 11870, 12029, 12030, 12109, 12188,
                12270, 12271, 12348, 12428, 12429, 12509, 12589, 12669, 12827, 12828, 12829, 12910,
                12987, 12988, 12989, 13068, 13069, 13147, 13228, 13229, 13230, 13307, 13387, 13388,
                13389, 13468, 13469, 13470, 13548, 13628, 13629, 13630, 13707, 13788, 13789, 13791,
                13869, 13870, 13948, 13949, 13951, 14028, 14029, 14108, 14110, 14111, 14112, 14188,
                14189, 14268, 14270, 14271, 14272, 14347, 14348, 14349, 14430, 14431, 14432, 14433,
                14510, 14511, 14590, 14591, 14592, 14593, 14668, 14670, 14671, 14751, 14752, 14753,
                14754, 14755, 14830, 14831, 14832, 14833, 14834, 14835, 14911, 14912, 14913, 14914,
                14915, 14988, 14990, 14991, 14992, 14993, 14994, 15073, 15074, 15075, 15151, 15152,
                15153, 15154, 15155, 15233, 15234, 15235, 15309, 15310, 15311, 15312, 15313, 15314,
                15473, 15474, 15631, 15632, 15633, 15634,
            ],
        ),
        (
            "beach_walk_north",
            &[35, 33, 35, 29],
            &[
                8994, 9005, 9154, 9165, 9314, 9325, 9394, 9405, 9474, 9485, 9554, 9565, 9714, 9725,
                9874, 9885, 11247, 11407, 11646, 11888, 11889, 12048, 12049, 12128, 12210, 12287,
                12288, 12370, 12448, 12449, 12529, 12609, 12689, 12849, 12850, 12851, 12928, 13009,
                13010, 13011, 13088, 13089, 13171, 13248, 13249, 13250, 13331, 13408, 13409, 13410,
                13489, 13490, 13570, 13649, 13650, 13730, 13808, 13809, 13810, 13811, 13888, 13889,
                13968, 13969, 13970, 13971, 14048, 14049, 14126, 14127, 14130, 14207, 14209, 14210,
                14286, 14287, 14290, 14367, 14368, 14369, 14370, 14443, 14444, 14445, 14446, 14447,
                14448, 14526, 14527, 14528, 14603, 14604, 14605, 14606, 14607, 14608, 14686, 14689,
                14760, 14761, 14762, 14763, 14764, 14765, 14766, 14767, 14843, 14844, 14845, 14846,
                14847, 14848, 14920, 14921, 14922, 14923, 14924, 14925, 14926, 14927, 15003, 15004,
                15005, 15006, 15007, 15009, 15081, 15082, 15083, 15084, 15085, 15160, 15161, 15162,
                15163, 15164, 15165, 15166, 15167, 15241, 15242, 15243, 15244, 15245, 15320, 15321,
                15322, 15323, 15324, 15325, 15326, 15327, 15328, 15481, 15482, 15483, 15484, 15485,
                15641, 15642, 15643, 15644, 15645,
            ],
        ),
        (
            "beach_walk_south",
            &[85, 79, 85, 78],
            &[
                8675, 8684, 8835, 8844, 8994, 8995, 9004, 9075, 9084, 9154, 9155, 9164, 9235, 9244,
                9314, 9315, 9324, 9325, 9394, 9395, 9404, 9474, 9475, 9484, 9485, 9554, 9555, 9564,
                9635, 9636, 9644, 9714, 9715, 9724, 9725, 9795, 9796, 9804, 9874, 9875, 9884, 9885,
                10035, 10036, 10044, 10195, 10196, 10204, 11232, 11392, 11870, 11871, 12030, 12031,
                12111, 12189, 12271, 12272, 12349, 12430, 12431, 12510, 12590, 12670, 12828, 12829,
                12830, 12911, 12988, 12989, 12990, 13070, 13071, 13148, 13229, 13230, 13231, 13308,
                13389, 13390, 13391, 13469, 13470, 13471, 13549, 13629, 13630, 13631, 13709, 13789,
                13790, 13870, 13871, 13949, 13950, 14030, 14031, 14109, 14189, 14190, 14269, 14349,
                14350, 14431, 14511, 14591, 14671, 14672, 14831, 14832, 14991, 14992, 14993, 14994,
                15152, 15153, 15154, 15311, 15312, 15313, 15314, 15474, 15633, 15634,
            ],
        ),
        (
            "specialanimation_beach_eyes_closed_east",
            &[82],
            &[
                2195, 2274, 2275, 2354, 2355, 2357, 2365, 2435, 2436, 2437, 2445, 2831, 2989, 2990,
                3068, 3149, 3227, 3228, 3229, 3307, 3388, 3389, 3390, 3468, 3469, 3471, 3548, 3550,
                3551, 3552, 3630, 3631, 3632, 3633, 3711, 3712, 3713, 3714, 3715, 3793, 3794, 3795,
            ],
        ),
        (
            "specialanimation_beach_eyes_closed_south",
            &[93],
            &[
                2195, 2204, 2274, 2275, 2284, 2354, 2355, 2364, 2365, 2435, 2436, 2444, 2832, 2990,
                2991, 3069, 3150, 3228, 3229, 3230, 3308, 3389, 3390, 3391, 3469, 3470, 3549, 3631,
            ],
        ),
        (
            "specialanimation_spring_eyes_closed_east",
            &[63],
            &[
                2195, 2274, 2275, 2354, 2355, 2357, 2365, 2435, 2436, 2437, 2445, 2831, 2989, 2990,
                3068, 3149, 3227, 3228, 3229, 3307, 3388, 3389, 3390, 3468, 3469, 3548, 3550, 3551,
                3630, 3631, 3632, 3633, 3711, 3712, 3713,
            ],
        ),
        (
            "specialanimation_spring_eyes_closed_south",
            &[73],
            &[
                2195, 2204, 2274, 2275, 2284, 2354, 2355, 2364, 2365, 2435, 2436, 2444, 2832, 2990,
                2991, 3069, 3150, 3228, 3229, 3230, 3308, 3389, 3390, 3469, 3470, 3549, 3631,
            ],
        ),
        (
            "specialanimation_spring_harvest_east",
            &[59, 70, 78, 77, 78, 77, 78, 77, 70, 59],
            &[],
        ),
        (
            "specialanimation_spring_lute_play_end_south",
            &[43, 59],
            &[
                4355, 4364, 4435, 4514, 4515, 4524, 4594, 4595, 4674, 4675, 4684, 4685, 4754, 4755,
                4835, 4844, 4915, 4916, 5632, 5712, 5950, 5951, 6030, 6031, 6109, 6189, 6270, 6350,
                6428, 6429, 6430, 6508, 6509, 6510, 6588, 6668, 6749, 6750, 6751, 6829, 6830, 6831,
                6909, 6910, 6989, 6990, 7069, 7071, 7072, 7149, 7151, 7152, 7231, 7232, 7311, 7312,
                7392, 7472,
            ],
        ),
        (
            "specialanimation_spring_lute_play_loop_south",
            &[78, 78, 78, 73, 73, 73, 78, 78, 78, 78],
            &[
                21635, 21644, 21715, 21724, 21795, 21804, 22115, 22124, 22195, 22204, 22275, 22284,
                22355, 22364, 22434, 22435, 22444, 22514, 22515, 22524, 22594, 22595, 22604, 22675,
                22684, 22755, 22764, 22835, 22844, 22914, 22915, 22924, 22994, 22995, 23004, 23074,
                23075, 23084, 23154, 23155, 23164, 23234, 23235, 23244, 23245, 23314, 23315, 23324,
                23325, 23394, 23395, 23404, 23405, 23474, 23475, 23484, 23554, 23555, 23564, 23634,
                23635, 23644, 23714, 23715, 23724, 23725, 23794, 23795, 23804, 23805, 23874, 23875,
                23884, 23885, 23954, 23955, 23964, 23965, 24035, 24036, 24044, 24115, 24116, 24124,
                24195, 24196, 24204, 24274, 24275, 24284, 24285, 24354, 24355, 24364, 24365, 24434,
                24435, 24444, 24445, 24515, 24516, 24524, 24595, 24596, 24604, 24675, 24676, 24684,
                24755, 24756, 24764, 25075, 25076, 25084, 25155, 25156, 25164, 25235, 25236, 25244,
                29630, 29631, 29710, 29711, 29790, 29791, 30110, 30111, 30190, 30191, 30270, 30271,
                30350, 30351, 30429, 30509, 30589, 30670, 30750, 30830, 30909, 30989, 31069, 31149,
                31230, 31310, 31390, 31469, 31549, 31629, 31710, 31790, 31870, 31950, 32028, 32029,
                32030, 32108, 32109, 32110, 32188, 32189, 32190, 32270, 32350, 32430, 32508, 32509,
                32510, 32588, 32589, 32590, 32668, 32669, 32670, 32748, 32749, 32750, 32828, 32908,
                32988, 33068, 33069, 33070, 33148, 33149, 33150, 33228, 33229, 33230, 33308, 33388,
                33468, 33548, 33629, 33630, 33631, 33709, 33710, 33711, 33789, 33790, 33791, 33868,
                33948, 34028, 34109, 34110, 34111, 34189, 34190, 34191, 34269, 34270, 34271, 34349,
                34350, 34351, 34429, 34430, 34432, 34509, 34510, 34512, 34589, 34590, 34592, 34669,
                34670, 34671, 34749, 34750, 34751, 34829, 34830, 34831, 34909, 34910, 34912, 34989,
                34990, 34992, 35069, 35070, 35072, 35149, 35150, 35152, 35229, 35231, 35232, 35233,
                35309, 35311, 35312, 35313, 35389, 35391, 35392, 35393, 35469, 35470, 35472, 35549,
                35550, 35552, 35629, 35630, 35632, 35709, 35711, 35712, 35713, 35789, 35791, 35792,
                35793, 35869, 35871, 35872, 35873, 35949, 35951, 35952, 35953, 36031, 36032, 36033,
                36111, 36112, 36113, 36191, 36192, 36193, 36269, 36271, 36272, 36273, 36349, 36351,
                36352, 36353, 36429, 36431, 36432, 36433, 36511, 36512, 36513, 36591, 36592, 36593,
                36671, 36672, 36673, 36751, 36752, 36753, 36832, 36912, 36992, 37071, 37072, 37073,
                37151, 37152, 37153, 37231, 37232, 37233, 37312, 37392, 37472, 37552, 37872, 37952,
                38032,
            ],
        ),
        (
            "specialanimation_spring_lute_play_start_south",
            &[59, 43],
            &[
                4355, 4435, 4444, 4514, 4515, 4594, 4595, 4604, 4674, 4675, 4754, 4755, 4764, 4765,
                4835, 4836, 4915, 4924, 5632, 5712, 5950, 5951, 6030, 6031, 6109, 6189, 6270, 6350,
                6428, 6429, 6430, 6508, 6509, 6510, 6588, 6668, 6749, 6750, 6751, 6829, 6830, 6831,
                6909, 6910, 6989, 6990, 7069, 7071, 7072, 7149, 7151, 7152, 7231, 7232, 7311, 7312,
                7392, 7472,
            ],
        ),
        (
            "specialanimation_spring_read_end_south",
            &[48, 34, 59],
            &[
                6675, 6684, 6755, 6764, 6835, 6844, 6914, 6915, 6924, 6994, 6995, 7004, 7074, 7075,
                7084, 7154, 7155, 7164, 7165, 7234, 7235, 7244, 7245, 7314, 7315, 7324, 7325, 7395,
                7396, 7404, 7475, 7476, 7484, 7555, 7556, 7564, 8592, 8672, 8752, 9070, 9071, 9150,
                9151, 9230, 9231, 9309, 9389, 9469, 9550, 9630, 9710, 9788, 9789, 9790, 9868, 9869,
                9870, 9948, 9949, 9950, 10028, 10108, 10188, 10269, 10270, 10349, 10350, 10351,
                10429, 10430, 10431, 10509, 10510, 10589, 10590, 10669, 10670, 10672, 10749, 10829,
                10831, 10909, 10911, 10912, 10913, 10991, 11071, 11151, 11152, 11392,
            ],
        ),
        (
            "specialanimation_spring_read_loop_south",
            &[44, 53, 44, 53],
            &[
                8675, 8844, 8994, 8995, 9075, 9084, 9164, 9165, 9235, 9244, 9314, 9315, 9317, 9325,
                9394, 9395, 9404, 9474, 9482, 9484, 9485, 9554, 9555, 9564, 9635, 9636, 9637, 9645,
                9714, 9715, 9724, 9725, 9794, 9802, 9803, 9804, 9874, 9875, 9884, 9885, 10035,
                10036, 10044, 10195, 10196, 10204, 11231, 11869, 11870, 12188, 12270, 12430, 12509,
                12589, 12749, 12827, 12828, 12829, 12910, 13070, 13147, 13228, 13229, 13230, 13388,
                13389, 13390, 13468, 13469, 13470, 13548, 13708, 13788, 13789, 13869, 13870, 14029,
                14030, 14108, 14110, 14111, 14189, 14190, 14349, 14350, 14430, 14431, 14432, 14509,
                14669, 14751, 14752,
            ],
        ),
        (
            "specialanimation_spring_read_sit_end_south",
            &[38, 34, 50],
            &[
                7395, 7404, 7475, 7484, 7555, 7564, 7634, 7635, 7644, 7714, 7715, 7724, 7794, 7795,
                7804, 7874, 7875, 7884, 7885, 7954, 7955, 7964, 7965, 8034, 8035, 8044, 8045, 8115,
                8116, 8124, 8195, 8196, 8204, 8275, 8276, 8284, 8912, 8992, 9311, 9312, 9390, 9391,
                9392, 9470, 9471, 9472, 9550, 9791, 9870, 9871, 9950, 9951, 10029, 10030, 10031,
                10109, 10110, 10111, 10189, 10190, 10191, 10269, 10349, 10429, 10510, 10511, 10590,
                10591, 10670, 10671, 10750, 10751, 10830, 10831, 10910, 10911, 10990, 11070, 11150,
                11392,
            ],
        ),
        (
            "specialanimation_spring_read_sit_loop_south",
            &[31, 40, 30, 40],
            &[
                9635, 9804, 9954, 9955, 10035, 10044, 10124, 10125, 10195, 10204, 10274, 10275,
                10277, 10285, 10354, 10355, 10364, 10434, 10442, 10444, 10445, 10514, 10515, 10524,
                10595, 10596, 10597, 10605, 10674, 10675, 10684, 10685, 10754, 10762, 10763, 10764,
                10834, 10835, 10844, 10845, 10995, 10996, 11004, 11155, 11156, 11164, 11552, 11952,
                12112, 12190, 12191, 12509, 12590, 12591, 12750, 12751, 12830, 13148, 13149, 13150,
                13230, 13390, 13468, 13549, 13550, 13709, 13710, 13789, 13790, 13869, 14029, 14109,
                14110, 14190, 14350, 14429, 14510, 14670, 14751, 14830, 14990,
            ],
        ),
        (
            "specialanimation_spring_read_sit_start_south",
            &[50, 38, 34],
            &[
                7235, 7244, 7474, 7475, 7484, 7555, 7564, 7635, 7644, 7714, 7715, 7724, 7725, 7794,
                7795, 7804, 7874, 7875, 7884, 7955, 7956, 7964, 8034, 8035, 8044, 8045, 8114, 8115,
                8124, 8125, 8275, 8276, 8284, 8355, 8356, 8364, 8992, 9072, 9151, 9152, 9390, 9470,
                9471, 9472, 9550, 9551, 9552, 9631, 9869, 9870, 9871, 9950, 9951, 10030, 10031,
                10109, 10189, 10190, 10191, 10269, 10270, 10271, 10350, 10351, 10429, 10509, 10590,
                10591, 10670, 10671, 10750, 10751, 10830, 10910, 10911, 10990, 10991, 11150, 11230,
                11392,
            ],
        ),
        (
            "specialanimation_spring_read_start_south",
            &[59, 38, 44],
            &[
                6515, 6524, 6754, 6755, 6764, 6835, 6844, 6915, 6924, 6994, 6995, 7004, 7005, 7074,
                7075, 7084, 7154, 7155, 7164, 7235, 7236, 7244, 7314, 7315, 7324, 7325, 7394, 7395,
                7404, 7405, 7555, 7556, 7564, 7635, 7636, 7644, 8432, 8752, 8832, 8910, 8911, 9149,
                9230, 9231, 9310, 9311, 9390, 9469, 9549, 9628, 9629, 9630, 9710, 9790, 9868, 9948,
                9949, 9950, 10028, 10029, 10030, 10109, 10110, 10188, 10268, 10349, 10350, 10429,
                10430, 10431, 10509, 10510, 10511, 10589, 10669, 10670, 10672, 10749, 10750, 10831,
                10909, 10911, 10912, 10913, 10989, 10991, 11151, 11152, 11231, 11392,
            ],
        ),
        (
            "specialanimation_spring_spell_cast_end_south",
            &[69],
            &[
                2275, 2284, 2354, 2355, 2364, 2434, 2435, 2444, 2445, 2515, 2516, 2524, 2912, 3070,
                3071, 3149, 3230, 3308, 3309, 3310, 3388, 3469, 3470, 3549, 3550, 3629, 3711,
            ],
        ),
        (
            "specialanimation_spring_spell_cast_loop_south",
            &[83, 83, 86, 87],
            &[
                8675, 8684, 8755, 8764, 8835, 8844, 8915, 8924, 8994, 8995, 9004, 9074, 9075, 9084,
                9154, 9155, 9164, 9234, 9235, 9244, 9314, 9315, 9324, 9325, 9394, 9395, 9404, 9405,
                9474, 9475, 9484, 9485, 9554, 9555, 9564, 9565, 9635, 9636, 9644, 9715, 9716, 9724,
                9795, 9796, 9804, 9875, 9876, 9884, 11311, 11551, 11949, 11950, 12268, 12508,
                12908, 12988, 13068, 13148, 13149, 13228, 13308, 13388, 13468, 13549, 13550, 13629,
                13630, 13709, 13710, 13789, 13790, 13869, 13870, 13872, 13949, 13950, 13952, 14029,
                14030, 14108, 14109, 14111, 14112, 14189, 14191, 14192, 14193, 14269, 14271, 14272,
                14273, 14349, 14351, 14429, 14431, 14432, 14433, 14511, 14512, 14513, 14591, 14592,
                14671, 14672, 14673, 14751, 14752, 14832, 14912, 14992,
            ],
        ),
        (
            "specialanimation_spring_spell_cast_start_south",
            &[69, 76],
            &[
                4435, 4444, 4515, 4524, 4594, 4595, 4604, 4674, 4675, 4684, 4754, 4755, 4764, 4765,
                4834, 4835, 4844, 4845, 4915, 4916, 4924, 4995, 4996, 5004, 5712, 5792, 6030, 6031,
                6110, 6111, 6189, 6269, 6430, 6508, 6509, 6588, 6589, 6590, 6668, 6748, 6909, 6910,
                6989, 7069, 7070, 7149, 7151, 7152, 7153, 7229, 7311, 7312, 7391, 7472,
            ],
        ),
        (
            "spring_action_east",
            &[52, 61, 59, 61, 59, 52, 55],
            &[
                15238, 15318, 15398, 15478, 15635, 15716, 15797, 15798, 15877, 15878, 15957, 15958,
                16037, 16038, 16116, 16194, 16195, 16275, 16276, 16357, 16358, 16360, 16368, 16437,
                16438, 16440, 16448, 16517, 16518, 16520, 16528, 16597, 16598, 16600, 16608, 16675,
                16676, 16754, 16755, 16757, 16765, 16835, 16836, 16838, 16846, 16918, 16919, 16920,
                16928, 16998, 16999, 17000, 17008, 17078, 17079, 17080, 17088, 17158, 17159, 17160,
                17168, 17235, 17236, 17238, 17246, 17315, 17316, 17317, 17325, 17396, 17397, 17398,
                17406, 17796, 17797, 17798, 17806, 19153, 19233, 19313, 19393, 19633, 20033, 20111,
                20271, 20272, 20351, 20352, 20431, 20432, 20511, 20512, 20751, 20752, 20830, 20910,
                20990, 21070, 21151, 21152, 21229, 21230, 21310, 21391, 21392, 21471, 21472, 21551,
                21552, 21631, 21632, 21710, 21788, 21871, 21950, 21951, 21952, 22030, 22031, 22032,
                22110, 22111, 22112, 22190, 22191, 22192, 22271, 22349, 22429, 22430, 22431, 22510,
                22590, 22670, 22750, 22829, 22830, 22831, 22907, 22908, 22909, 22989, 23071, 23072,
                23073, 23151, 23152, 23153, 23231, 23232, 23233, 23311, 23312, 23313, 23389, 23467,
                23550, 23551, 23631, 23632, 23711, 23712, 23791, 23792, 23871, 23872, 23950, 23951,
                24028, 24029, 24030, 24110, 24111, 24191, 24193, 24271, 24273, 24351, 24353, 24431,
                24433, 24510, 24511, 24588, 24589, 24670, 24753, 24754, 24833, 24834, 24913, 24914,
                24993, 24994, 25070, 25148, 25150, 25151, 25232, 25314, 25315, 25394, 25395, 25474,
                25475, 25554, 25555, 25632, 25710, 25711, 25712, 25713, 26271, 26272, 26273, 26833,
            ],
        ),
        (
            "spring_action_north",
            &[24, 29, 29, 29, 29, 27, 28],
            &[
                15234, 15245, 15314, 15325, 15394, 15405, 15474, 15485, 15714, 15725, 15794, 15805,
                15874, 15885, 15954, 15965, 16034, 16045, 16114, 16125, 16194, 16205, 16274, 16285,
                16674, 16685, 16754, 16765, 19647, 19728, 19808, 19888, 19968, 20047, 20127, 20288,
                20289, 20368, 20369, 20448, 20449, 20528, 20529, 20768, 20769, 21168, 21169, 21248,
                21249, 21330, 21408, 21409, 21488, 21489, 21568, 21569, 21648, 21649, 21730, 21810,
                21889, 21968, 21969, 21970, 22048, 22049, 22050, 22128, 22129, 22130, 22208, 22209,
                22210, 22289, 22369, 22449, 22450, 22451, 22530, 22610, 22690, 22770, 22849, 22850,
                22851, 22929, 22930, 22931, 23011, 23088, 23089, 23168, 23169, 23248, 23249, 23328,
                23329, 23411, 23491, 23569, 23570, 23647, 23648, 23649, 23650, 23727, 23728, 23729,
                23730, 23807, 23808, 23809, 23810, 23887, 23888, 23889, 23890, 23969, 23970, 24049,
                24050, 24128, 24129, 24130, 24131, 24206, 24209, 24286, 24289, 24366, 24369, 24446,
                24449, 24528, 24529, 24530, 24531, 24608, 24609, 24610, 24611, 24686, 24687, 24690,
                24763, 24764, 24765, 24766, 24767, 24769, 24843, 24844, 24845, 24846, 24847, 24849,
                24923, 24924, 24925, 24926, 24927, 24929, 25003, 25004, 25005, 25006, 25007, 25009,
                25086, 25087, 25090, 25166, 25167, 25170, 25243, 25244, 25245, 25246, 25247, 25248,
                25320, 25321, 25322, 25323, 25324, 25325, 25326, 25327, 25328, 25400, 25401, 25402,
                25403, 25404, 25405, 25406, 25407, 25408, 25480, 25481, 25482, 25483, 25484, 25485,
                25486, 25487, 25488, 25560, 25561, 25562, 25563, 25564, 25565, 25566, 25567, 25568,
                25643, 25644, 25645, 25646, 25647, 25648, 25723, 25724, 25725, 25726, 25727, 25728,
                25800, 25801, 25802, 25803, 25804, 25805, 25806, 25807, 25881, 25882, 25883, 25884,
                25885, 25961, 25962, 25963, 25964, 25965, 26041, 26042, 26043, 26044, 26045, 26121,
                26122, 26123, 26124, 26125, 26200, 26201, 26202, 26203, 26204, 26205, 26206, 26207,
                26280, 26281, 26282, 26283, 26284, 26285, 26286, 26287, 26361, 26362, 26363, 26364,
                26365, 26761, 26762, 26763, 26764, 26765, 26841, 26842, 26843, 26844, 26845,
            ],
        ),
        (
            "spring_action_south",
            &[57, 59, 59, 59, 59, 57, 65],
            &[
                15235, 15244, 15315, 15324, 15395, 15404, 15475, 15484, 15635, 15644, 15715, 15724,
                15794, 15795, 15804, 15874, 15875, 15884, 15954, 15955, 15964, 16034, 16035, 16044,
                16115, 16124, 16194, 16195, 16204, 16274, 16275, 16284, 16354, 16355, 16364, 16365,
                16434, 16435, 16444, 16445, 16514, 16515, 16524, 16525, 16594, 16595, 16604, 16605,
                16674, 16675, 16684, 16754, 16755, 16764, 16765, 16834, 16835, 16844, 16845, 16915,
                16916, 16924, 16995, 16996, 17004, 17075, 17076, 17084, 17155, 17156, 17164, 17234,
                17235, 17244, 17245, 17315, 17316, 17324, 17395, 17396, 17404, 17795, 17796, 17804,
                19711, 19791, 19871, 19951, 20112, 20192, 20592, 20829, 20830, 20909, 20910, 20989,
                20990, 21069, 21070, 21230, 21231, 21310, 21311, 21388, 21468, 21548, 21628, 21710,
                21711, 21789, 21869, 21949, 21950, 22029, 22030, 22109, 22110, 22189, 22190, 22269,
                22350, 22430, 22508, 22509, 22510, 22588, 22589, 22590, 22668, 22669, 22670, 22748,
                22749, 22750, 22830, 22908, 22909, 22910, 22988, 22989, 22990, 23068, 23148, 23228,
                23308, 23388, 23389, 23390, 23468, 23548, 23629, 23630, 23631, 23709, 23710, 23711,
                23789, 23790, 23791, 23869, 23870, 23871, 23948, 24029, 24030, 24109, 24110, 24111,
                24189, 24190, 24269, 24270, 24349, 24350, 24429, 24430, 24509, 24510, 24511, 24589,
                24590, 24669, 24670, 24749, 24751, 24752, 24829, 24831, 24832, 24909, 24911, 24912,
                24989, 24991, 24992, 25069, 25070, 25149, 25229, 25231, 25311, 25312, 25313, 25391,
                25392, 25393, 25471, 25472, 25473, 25551, 25552, 25553, 25629, 25631, 25711, 25791,
                25792, 25872, 25952, 26032, 26112, 26191, 26192, 26352, 26752,
            ],
        ),
        (
            "spring_blink_east",
            &[59, 63, 59],
            &[
                6515, 6595, 6675, 6754, 6755, 6834, 6835, 6914, 6915, 6994, 6995, 6997, 7005, 7074,
                7075, 7077, 7085, 7154, 7155, 7157, 7165, 7235, 7236, 7237, 7245, 7315, 7316, 7317,
                7325, 7395, 7396, 7397, 7405, 8431, 8511, 8591, 8909, 8910, 8989, 8990, 9069, 9070,
                9148, 9228, 9308, 9389, 9469, 9549, 9627, 9628, 9629, 9707, 9708, 9709, 9787, 9788,
                9789, 9867, 9947, 10027, 10108, 10109, 10110, 10188, 10189, 10190, 10268, 10269,
                10270, 10348, 10349, 10428, 10429, 10508, 10509, 10588, 10590, 10591, 10668, 10670,
                10671, 10748, 10750, 10751, 10830, 10831, 10832, 10833, 10910, 10911, 10912, 10913,
                10990, 10991, 10992, 10993, 11071, 11072, 11073, 11151, 11152, 11153, 11231, 11232,
                11233, 11313, 11393, 11473,
            ],
        ),
        (
            "spring_blink_south",
            &[69, 73, 69],
            &[
                6515, 6524, 6595, 6604, 6675, 6684, 6754, 6755, 6764, 6834, 6835, 6844, 6914, 6915,
                6924, 6994, 6995, 7004, 7005, 7074, 7075, 7084, 7085, 7154, 7155, 7164, 7165, 7235,
                7236, 7244, 7315, 7316, 7324, 7395, 7396, 7404, 8432, 8512, 8592, 8910, 8911, 8990,
                8991, 9070, 9071, 9149, 9229, 9309, 9390, 9470, 9550, 9628, 9629, 9630, 9708, 9709,
                9710, 9788, 9789, 9790, 9868, 9948, 10028, 10109, 10110, 10189, 10190, 10269,
                10270, 10349, 10350, 10429, 10430, 10509, 10510, 10589, 10669, 10749, 10831, 10911,
                10991,
            ],
        ),
        (
            "spring_drink_east",
            &[44, 53, 44],
            &[
                7235, 7312, 7395, 7474, 7475, 7551, 7552, 7563, 7634, 7635, 7714, 7715, 7717, 7725,
                7792, 7794, 7795, 7803, 7874, 7875, 7877, 7885, 7955, 7956, 7957, 7965, 8033, 8034,
                8035, 8115, 8116, 8117, 8125, 9152, 9153, 9230, 9312, 9313, 9391, 9469, 9551, 9710,
                9870, 9871, 9949, 9950, 10030, 10031, 10110, 10189, 10270,
            ],
        ),
        (
            "spring_drink_north",
            &[2, 2, 2],
            &[
                7474, 7485, 7634, 7645, 7714, 7725, 7794, 7805, 7874, 7885, 8034, 8045, 8686, 8846,
                9007, 9167, 9168, 9327, 9328, 9409, 9488, 9489, 9569, 9648, 9808, 9888, 9889, 9890,
                9968, 9969, 10048, 10049, 10050, 10130, 10208, 10209, 10210, 10290, 10368, 10369,
                10450, 10528, 10529, 10607, 10608, 10609, 10610, 10688, 10689, 10767, 10768, 10769,
                10770, 10845, 10846, 10849, 10927, 10928, 10929, 10930, 11005, 11006, 11009, 11082,
                11083, 11084, 11085, 11086, 11087, 11164, 11165, 11166, 11169, 11242, 11243, 11244,
                11245, 11246, 11247, 11320, 11321, 11322, 11323, 11324, 11325, 11326, 11400, 11401,
                11402, 11403, 11404, 11405, 11406, 11407, 11480, 11481, 11482, 11483, 11484, 11485,
                11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646, 11647,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "spring_drink_south",
            &[56, 60, 56],
            &[
                7235, 7244, 7395, 7404, 7474, 7475, 7484, 7555, 7564, 7634, 7635, 7644, 7714, 7715,
                7724, 7725, 7794, 7795, 7804, 7874, 7875, 7884, 7885, 7955, 7956, 7964, 8034, 8035,
                8044, 8045, 8115, 8116, 8124, 8275, 8276, 8284, 8992, 9151, 9152, 9311, 9312, 9390,
                9470, 9471, 9472, 9550, 9631, 9709, 9791, 9869, 9870, 9871, 9950, 9951, 10029,
                10030, 10031, 10109, 10189, 10190, 10191, 10269, 10350, 10351, 10429, 10510, 10511,
                10590, 10591, 10670, 10671, 10750, 10751, 10830, 10910, 10911, 10990, 11072, 11150,
                11232, 11392,
            ],
        ),
        (
            "spring_eat_east",
            &[42, 47, 41, 46, 42],
            &[
                11794, 12035, 12116, 12193, 12194, 12205, 12355, 12434, 12435, 12515, 12516, 12594,
                12596, 12597, 12605, 12675, 12754, 12755, 12834, 12835, 12837, 12845, 12915, 12916,
                12918, 12926, 12995, 12996, 12997, 13074, 13075, 13154, 13155, 13157, 13165, 13235,
                13236, 13237, 13245, 13316, 13317, 13318, 13326, 13474, 13475, 13477, 13485, 13555,
                13556, 13557, 13565, 13875, 13876, 13877, 13885, 14513, 14514, 14592, 14593, 15073,
                15232, 15233, 15312, 15313, 15391, 15392, 15552, 15553, 15631, 15711, 15790, 15871,
                15872, 15873, 15951, 16112, 16191, 16270, 16430, 16431, 16510, 16511, 16512, 16590,
                16591, 16671, 16750, 16751, 16830, 16910, 16990, 17070, 17071, 17150, 17311, 17470,
                17711, 18111,
            ],
        ),
        (
            "spring_eat_north",
            &[2, 2, 2],
            &[
                7474, 7485, 7634, 7645, 7714, 7725, 7794, 7805, 7874, 7885, 8034, 8045, 8686, 8846,
                9007, 9167, 9168, 9327, 9328, 9409, 9488, 9489, 9569, 9648, 9808, 9888, 9889, 9890,
                9968, 9969, 10048, 10049, 10050, 10130, 10208, 10209, 10210, 10290, 10368, 10369,
                10450, 10528, 10529, 10607, 10608, 10609, 10610, 10688, 10689, 10767, 10768, 10769,
                10770, 10845, 10846, 10849, 10927, 10928, 10929, 10930, 11005, 11006, 11009, 11082,
                11083, 11084, 11085, 11086, 11087, 11164, 11165, 11166, 11169, 11242, 11243, 11244,
                11245, 11246, 11247, 11320, 11321, 11322, 11323, 11324, 11325, 11326, 11400, 11401,
                11402, 11403, 11404, 11405, 11406, 11407, 11480, 11481, 11482, 11483, 11484, 11485,
                11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646, 11647,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "spring_eat_south",
            &[56, 54, 56, 67, 54],
            &[
                11795, 11804, 12035, 12044, 12194, 12195, 12204, 12355, 12364, 12434, 12435, 12444,
                12515, 12524, 12594, 12595, 12604, 12605, 12675, 12684, 12754, 12755, 12764, 12834,
                12835, 12844, 12845, 12914, 12915, 12924, 12995, 12996, 13004, 13074, 13075, 13084,
                13154, 13155, 13164, 13165, 13235, 13236, 13244, 13314, 13315, 13324, 13325, 13474,
                13475, 13484, 13485, 13555, 13556, 13564, 13715, 13716, 13724, 13875, 13876, 13884,
                14591, 14592, 14912, 15072, 15231, 15232, 15390, 15391, 15392, 15551, 15552, 15630,
                15710, 15711, 15712, 15789, 15870, 15871, 15872, 15950, 16031, 16190, 16191, 16351,
                16429, 16430, 16431, 16510, 16511, 16589, 16590, 16591, 16670, 16671, 16749, 16750,
                16751, 16829, 16909, 16910, 16911, 16989, 17069, 17070, 17071, 17149, 17230, 17231,
                17309, 17390, 17391, 17469, 17550, 17551, 17630, 17631, 17710, 17711, 17790, 17791,
                17870, 17871, 17950, 17951, 18030, 18110, 18111, 18190, 18270, 18271, 18350, 18432,
                18510, 18592, 18670, 18912, 19072,
            ],
        ),
        (
            "spring_idle_east",
            &[55],
            &[
                2195, 2274, 2275, 2354, 2355, 2357, 2365, 2435, 2436, 2437, 2445, 2831, 2989, 2990,
                3068, 3149, 3227, 3228, 3229, 3307, 3388, 3389, 3390, 3468, 3469, 3548, 3550, 3551,
                3630, 3631, 3632, 3633, 3711, 3712, 3713, 3793,
            ],
        ),
        (
            "spring_idle_north",
            &[28],
            &[
                2274, 2285, 2354, 2365, 2847, 3008, 3009, 3090, 3169, 3249, 3250, 3251, 3331, 3409,
                3410, 3488, 3489, 3490, 3491, 3566, 3567, 3570, 3643, 3644, 3645, 3646, 3647, 3648,
                3720, 3721, 3722, 3723, 3724, 3725, 3726, 3727, 3801, 3802, 3803, 3804, 3805,
            ],
        ),
        (
            "spring_idle_south",
            &[65],
            &[
                2195, 2204, 2274, 2275, 2284, 2354, 2355, 2364, 2365, 2435, 2436, 2444, 2832, 2990,
                2991, 3069, 3150, 3228, 3229, 3230, 3308, 3389, 3390, 3469, 3470, 3549, 3631,
            ],
        ),
        (
            "spring_kiss_east",
            &[51, 54, 67, 58],
            &[
                8838, 8994, 9076, 9157, 9158, 9236, 9313, 9314, 9395, 9396, 9477, 9478, 9480, 9488,
                9555, 9556, 9633, 9634, 9636, 9644, 9715, 9716, 9718, 9726, 9798, 9799, 9800, 9808,
                9875, 9876, 9878, 9886, 9954, 9955, 9956, 9964, 10036, 10037, 10038, 10046, 10196,
                10197, 10198, 10206, 11070, 11550, 11630, 11710, 11711, 11790, 12029, 12188, 12189,
                12269, 12270, 12350, 12351, 12429, 12430, 12507, 12588, 12669, 12670, 12671, 12748,
                12828, 12829, 12909, 12910, 12989, 13069, 13070, 13147, 13148, 13149, 13228, 13229,
                13230, 13310, 13311, 13388, 13389, 13390, 13467, 13548, 13630, 13631, 13708, 13788,
                13789, 13869, 13870, 13950, 13952, 14029, 14030, 14107, 14108, 14109, 14189, 14190,
                14272, 14273, 14274, 14349, 14350, 14428, 14430, 14509, 14511, 14592, 14593, 14594,
                14669, 14671, 14750, 14751, 14752, 14753, 14831, 14832, 14833, 14834, 14913, 14991,
                14992, 14993, 14994, 15070, 15071, 15072, 15073, 15151, 15152, 15153, 15154, 15311,
                15312, 15313, 15314, 15392,
            ],
        ),
        (
            "spring_sit_east",
            &[43],
            &[
                2435, 2514, 2515, 2594, 2595, 2597, 2605, 2675, 2676, 2677, 2685, 3072, 3073, 3151,
                3310, 3311, 3390,
            ],
        ),
        (
            "spring_sit_north",
            &[2],
            &[
                2514, 2525, 2594, 2605, 2926, 3087, 3088, 3169, 3248, 3328, 3329, 3330, 3410, 3488,
                3489, 3567, 3568, 3569, 3570, 3645, 3646, 3649, 3722, 3723, 3724, 3725, 3726, 3727,
                3800, 3801, 3802, 3803, 3804, 3805, 3806, 3881, 3882, 3883, 3884,
            ],
        ),
        (
            "spring_sit_south",
            &[54],
            &[
                2435, 2444, 2514, 2515, 2524, 2594, 2595, 2604, 2605, 2675, 2676, 2684, 3071, 3072,
                3150, 3231, 3309, 3310, 3311, 3389, 3470, 3471, 3550, 3551, 3630,
            ],
        ),
        (
            "spring_sleep_east",
            &[55],
            &[
                2275, 2354, 2355, 2434, 2435, 2437, 2445, 2515, 2516, 2517, 2525,
            ],
        ),
        (
            "spring_walk_east",
            &[55, 55, 55, 55],
            &[
                8675, 8835, 8994, 8995, 9075, 9154, 9155, 9235, 9314, 9315, 9317, 9325, 9394, 9395,
                9474, 9475, 9477, 9485, 9554, 9555, 9635, 9636, 9637, 9645, 9714, 9715, 9717, 9725,
                9795, 9796, 9797, 9805, 9874, 9875, 9877, 9885, 10035, 10036, 10037, 10045, 10195,
                10196, 10197, 10205, 11231, 11391, 11632, 11869, 11870, 12029, 12030, 12109, 12188,
                12270, 12271, 12348, 12428, 12429, 12509, 12589, 12669, 12827, 12828, 12829, 12910,
                12987, 12988, 12989, 13068, 13069, 13147, 13228, 13229, 13230, 13307, 13387, 13388,
                13389, 13468, 13469, 13470, 13548, 13628, 13629, 13630, 13707, 13788, 13789, 13869,
                13870, 13948, 13949, 14028, 14029, 14108, 14110, 14111, 14188, 14189, 14268, 14270,
                14271, 14347, 14348, 14349, 14430, 14431, 14432, 14433, 14510, 14511, 14590, 14591,
                14592, 14593, 14668, 14670, 14671, 14751, 14752, 14753, 14830, 14831, 14832, 14833,
                14834, 14911, 14912, 14913, 14988, 14990, 14991, 14992, 14993, 15073, 15151, 15152,
                15153, 15233, 15309, 15310, 15311, 15312, 15473, 15631, 15632,
            ],
        ),
        (
            "spring_walk_north",
            &[28, 20, 28, 19],
            &[
                8994, 9005, 9154, 9165, 9314, 9325, 9394, 9405, 9474, 9485, 9554, 9565, 9714, 9725,
                9874, 9885, 11247, 11407, 11646, 11888, 11889, 12048, 12049, 12128, 12210, 12287,
                12288, 12370, 12448, 12449, 12529, 12609, 12689, 12849, 12850, 12851, 12928, 13009,
                13010, 13011, 13088, 13089, 13171, 13248, 13249, 13250, 13331, 13408, 13409, 13410,
                13489, 13490, 13570, 13649, 13650, 13730, 13808, 13809, 13810, 13811, 13888, 13889,
                13968, 13969, 13970, 13971, 14048, 14049, 14126, 14127, 14130, 14207, 14209, 14210,
                14286, 14287, 14290, 14367, 14368, 14369, 14370, 14443, 14444, 14445, 14446, 14447,
                14448, 14526, 14527, 14528, 14603, 14604, 14605, 14606, 14607, 14608, 14686, 14689,
                14760, 14761, 14762, 14763, 14764, 14765, 14766, 14767, 14843, 14844, 14845, 14846,
                14847, 14848, 14920, 14921, 14922, 14923, 14924, 14925, 14926, 14927, 15003, 15004,
                15005, 15006, 15007, 15009, 15081, 15082, 15083, 15084, 15085, 15160, 15161, 15162,
                15163, 15164, 15165, 15166, 15167, 15241, 15242, 15243, 15244, 15245, 15320, 15321,
                15322, 15323, 15324, 15325, 15326, 15327, 15328, 15481, 15482, 15483, 15484, 15485,
                15641, 15642, 15643, 15644, 15645,
            ],
        ),
        (
            "spring_walk_south",
            &[65, 58, 65, 57],
            &[
                8675, 8684, 8835, 8844, 8994, 8995, 9004, 9075, 9084, 9154, 9155, 9164, 9235, 9244,
                9314, 9315, 9324, 9325, 9394, 9395, 9404, 9474, 9475, 9484, 9485, 9554, 9555, 9564,
                9635, 9636, 9644, 9714, 9715, 9724, 9725, 9795, 9796, 9804, 9874, 9875, 9884, 9885,
                10035, 10036, 10044, 10195, 10196, 10204, 11232, 11392, 11870, 11871, 12030, 12031,
                12111, 12189, 12271, 12272, 12349, 12430, 12431, 12510, 12590, 12670, 12828, 12829,
                12830, 12911, 12988, 12989, 12990, 13070, 13148, 13229, 13230, 13231, 13308, 13389,
                13390, 13469, 13470, 13549, 13629, 13630, 13709, 13789, 13790, 13870, 13871, 13949,
                13950, 14030, 14031, 14109, 14189, 14190, 14269, 14349, 14350, 14431, 14511, 14591,
                14671, 14672, 14831, 14832, 14991, 14992, 15152, 15311,
            ],
        ),
        (
            "specialanimation_summer_eyes_closed_east",
            &[66],
            &[
                2195, 2274, 2275, 2354, 2355, 2357, 2365, 2435, 2436, 2437, 2445, 2831, 2989, 2990,
                3068, 3149, 3227, 3228, 3229, 3307, 3388, 3389, 3390, 3468, 3469, 3548, 3550, 3551,
                3630, 3631, 3632, 3633, 3711, 3712,
            ],
        ),
        (
            "specialanimation_summer_eyes_closed_south",
            &[74],
            &[
                2195, 2204, 2274, 2275, 2284, 2354, 2355, 2364, 2365, 2435, 2436, 2444, 2832, 2990,
                2991, 3069, 3150, 3228, 3229, 3230, 3308, 3389, 3390, 3391, 3469, 3470, 3549, 3631,
            ],
        ),
        (
            "specialanimation_summer_harvest_east",
            &[61, 68, 79, 78, 79, 78, 79, 78, 68, 61],
            &[],
        ),
        (
            "specialanimation_summer_lute_play_end_south",
            &[40, 58],
            &[
                4355, 4364, 4435, 4514, 4515, 4524, 4594, 4595, 4674, 4675, 4684, 4685, 4754, 4755,
                4835, 4844, 4915, 4916, 5632, 5712, 5950, 5951, 6030, 6031, 6109, 6189, 6270, 6350,
                6428, 6429, 6430, 6508, 6509, 6510, 6588, 6668, 6749, 6750, 6751, 6829, 6830, 6831,
                6909, 6910, 6989, 6990, 7069, 7071, 7072, 7149, 7151, 7152, 7231, 7311,
            ],
        ),
        (
            "specialanimation_summer_lute_play_loop_south",
            &[77, 77, 77, 72, 72, 72, 77, 77, 77, 77],
            &[
                21635, 21644, 21715, 21724, 21795, 21804, 22115, 22124, 22195, 22204, 22275, 22284,
                22355, 22364, 22434, 22435, 22444, 22514, 22515, 22524, 22594, 22595, 22604, 22675,
                22684, 22755, 22764, 22835, 22844, 22914, 22915, 22924, 22994, 22995, 23004, 23074,
                23075, 23084, 23154, 23155, 23164, 23234, 23235, 23244, 23245, 23314, 23315, 23324,
                23325, 23394, 23395, 23404, 23405, 23474, 23475, 23484, 23554, 23555, 23564, 23634,
                23635, 23644, 23714, 23715, 23724, 23725, 23794, 23795, 23804, 23805, 23874, 23875,
                23884, 23885, 23954, 23955, 23964, 23965, 24035, 24036, 24044, 24115, 24116, 24124,
                24195, 24196, 24204, 24274, 24275, 24284, 24285, 24354, 24355, 24364, 24365, 24434,
                24435, 24444, 24445, 24515, 24516, 24524, 24595, 24596, 24604, 24675, 24676, 24684,
                24755, 24756, 24764, 25075, 25076, 25084, 25155, 25156, 25164, 25235, 25236, 25244,
                29630, 29631, 29710, 29711, 29790, 29791, 30110, 30111, 30190, 30191, 30270, 30271,
                30350, 30351, 30429, 30509, 30589, 30670, 30750, 30830, 30909, 30989, 31069, 31149,
                31230, 31310, 31390, 31469, 31549, 31629, 31710, 31790, 31870, 31950, 32028, 32029,
                32030, 32108, 32109, 32110, 32188, 32189, 32190, 32270, 32350, 32430, 32508, 32509,
                32510, 32588, 32589, 32590, 32668, 32669, 32670, 32748, 32749, 32750, 32828, 32908,
                32988, 33068, 33069, 33070, 33148, 33149, 33150, 33228, 33229, 33230, 33308, 33388,
                33468, 33548, 33629, 33630, 33631, 33709, 33710, 33711, 33789, 33790, 33791, 33868,
                33948, 34028, 34109, 34110, 34111, 34189, 34190, 34191, 34269, 34270, 34271, 34349,
                34350, 34351, 34429, 34430, 34432, 34509, 34510, 34512, 34589, 34590, 34592, 34669,
                34670, 34671, 34749, 34750, 34751, 34829, 34830, 34831, 34909, 34910, 34912, 34989,
                34990, 34992, 35069, 35070, 35072, 35149, 35150, 35152, 35229, 35231, 35232, 35309,
                35311, 35312, 35389, 35391, 35392, 35469, 35470, 35472, 35549, 35550, 35552, 35629,
                35630, 35632, 35709, 35711, 35712, 35789, 35791, 35792, 35869, 35871, 35872, 35949,
                35951, 35952, 36031, 36111, 36191, 36269, 36271, 36272, 36349, 36351, 36352, 36429,
                36431, 36432, 36511, 36591, 36671, 36751, 37071, 37151, 37231,
            ],
        ),
        (
            "specialanimation_summer_lute_play_start_south",
            &[58, 40],
            &[
                4355, 4435, 4444, 4514, 4515, 4594, 4595, 4604, 4674, 4675, 4754, 4755, 4764, 4765,
                4835, 4836, 4915, 4924, 5632, 5712, 5950, 5951, 6030, 6031, 6109, 6189, 6270, 6350,
                6428, 6429, 6430, 6508, 6509, 6510, 6588, 6668, 6749, 6750, 6751, 6829, 6830, 6831,
                6909, 6910, 6989, 6990, 7069, 7071, 7072, 7149, 7151, 7152, 7231, 7311,
            ],
        ),
        (
            "specialanimation_summer_read_end_south",
            &[50, 36, 59],
            &[
                6675, 6684, 6755, 6764, 6835, 6844, 6914, 6915, 6924, 6994, 6995, 7004, 7074, 7075,
                7084, 7154, 7155, 7164, 7165, 7234, 7235, 7244, 7245, 7314, 7315, 7324, 7325, 7395,
                7396, 7404, 7475, 7476, 7484, 7555, 7556, 7564, 8592, 8672, 8752, 9070, 9071, 9150,
                9151, 9230, 9231, 9309, 9389, 9469, 9550, 9630, 9710, 9788, 9789, 9790, 9868, 9869,
                9870, 9948, 9949, 9950, 10028, 10108, 10188, 10269, 10270, 10271, 10349, 10350,
                10351, 10429, 10430, 10431, 10509, 10510, 10589, 10590, 10669, 10670, 10672, 10749,
                10829, 10831, 10909, 10911, 10912, 10991, 11071, 11151,
            ],
        ),
        (
            "specialanimation_summer_read_loop_south",
            &[46, 55, 46, 55],
            &[
                8675, 8844, 8994, 8995, 9075, 9084, 9164, 9165, 9235, 9244, 9314, 9315, 9317, 9325,
                9394, 9395, 9404, 9474, 9482, 9484, 9485, 9554, 9555, 9564, 9635, 9636, 9637, 9645,
                9714, 9715, 9724, 9725, 9794, 9802, 9803, 9804, 9874, 9875, 9884, 9885, 10035,
                10036, 10044, 10195, 10196, 10204, 11231, 11869, 11870, 12188, 12270, 12430, 12509,
                12589, 12749, 12827, 12828, 12829, 12910, 13070, 13147, 13228, 13229, 13230, 13388,
                13389, 13390, 13468, 13469, 13470, 13548, 13708, 13788, 13789, 13869, 13870, 14029,
                14030, 14108, 14110, 14111, 14189, 14190, 14349, 14350, 14430, 14431, 14509, 14669,
                14751,
            ],
        ),
        (
            "specialanimation_summer_read_sit_end_south",
            &[40, 36, 53],
            &[
                7395, 7404, 7475, 7484, 7555, 7564, 7634, 7635, 7644, 7714, 7715, 7724, 7794, 7795,
                7804, 7874, 7875, 7884, 7885, 7954, 7955, 7964, 7965, 8034, 8035, 8044, 8045, 8115,
                8116, 8124, 8195, 8196, 8204, 8275, 8276, 8284, 8912, 8992, 9311, 9312, 9390, 9391,
                9392, 9470, 9471, 9472, 9550, 9791, 9870, 9871, 9950, 9951, 10029, 10030, 10031,
                10109, 10110, 10111, 10189, 10190, 10191, 10269, 10349, 10429, 10510, 10511, 10590,
                10591, 10670, 10671, 10750, 10751, 10830, 10831, 10910, 10911, 10990, 11070, 11150,
                11392,
            ],
        ),
        (
            "specialanimation_summer_read_sit_loop_south",
            &[31, 40, 30, 40],
            &[
                9635, 9804, 9954, 9955, 10035, 10044, 10124, 10125, 10195, 10204, 10274, 10275,
                10277, 10285, 10354, 10355, 10364, 10434, 10442, 10444, 10445, 10514, 10515, 10524,
                10595, 10596, 10597, 10605, 10674, 10675, 10684, 10685, 10754, 10762, 10763, 10764,
                10834, 10835, 10844, 10845, 10995, 10996, 11004, 11155, 11156, 11164, 11552, 11952,
                12112, 12190, 12191, 12509, 12590, 12591, 12750, 12751, 12830, 13148, 13149, 13150,
                13230, 13390, 13468, 13549, 13550, 13709, 13710, 13789, 13790, 13869, 14029, 14109,
                14110, 14190, 14350, 14429, 14510, 14670, 14751, 14830, 14990,
            ],
        ),
        (
            "specialanimation_summer_read_sit_start_south",
            &[53, 40, 36],
            &[
                7235, 7244, 7474, 7475, 7484, 7555, 7564, 7635, 7644, 7714, 7715, 7724, 7725, 7794,
                7795, 7804, 7874, 7875, 7884, 7955, 7956, 7964, 8034, 8035, 8044, 8045, 8114, 8115,
                8124, 8125, 8275, 8276, 8284, 8355, 8356, 8364, 8992, 9072, 9151, 9152, 9390, 9470,
                9471, 9472, 9550, 9551, 9552, 9631, 9869, 9870, 9871, 9950, 9951, 10030, 10031,
                10109, 10189, 10190, 10191, 10269, 10270, 10271, 10350, 10351, 10429, 10509, 10590,
                10591, 10670, 10671, 10750, 10751, 10830, 10910, 10911, 10990, 10991, 11150, 11230,
                11392,
            ],
        ),
        (
            "specialanimation_summer_read_start_south",
            &[59, 40, 46],
            &[
                6515, 6524, 6754, 6755, 6764, 6835, 6844, 6915, 6924, 6994, 6995, 7004, 7005, 7074,
                7075, 7084, 7154, 7155, 7164, 7235, 7236, 7244, 7314, 7315, 7324, 7325, 7394, 7395,
                7404, 7405, 7555, 7556, 7564, 7635, 7636, 7644, 8432, 8752, 8832, 8910, 8911, 9149,
                9230, 9231, 9310, 9311, 9390, 9469, 9549, 9628, 9629, 9630, 9710, 9790, 9868, 9948,
                9949, 9950, 10028, 10029, 10030, 10109, 10110, 10111, 10188, 10268, 10349, 10350,
                10429, 10430, 10431, 10509, 10510, 10511, 10589, 10669, 10670, 10672, 10749, 10750,
                10831, 10909, 10911, 10912, 10989, 10991, 11151, 11231,
            ],
        ),
        (
            "specialanimation_summer_spell_cast_end_south",
            &[70],
            &[
                2275, 2284, 2354, 2355, 2364, 2434, 2435, 2444, 2445, 2515, 2516, 2524, 2912, 3070,
                3071, 3149, 3230, 3308, 3309, 3310, 3388, 3469, 3470, 3471, 3549, 3550, 3629, 3711,
            ],
        ),
        (
            "specialanimation_summer_spell_cast_loop_south",
            &[88, 87, 90, 91],
            &[
                8675, 8684, 8755, 8764, 8835, 8844, 8915, 8924, 8994, 8995, 9004, 9074, 9075, 9084,
                9154, 9155, 9164, 9234, 9235, 9244, 9314, 9315, 9324, 9325, 9394, 9395, 9404, 9405,
                9474, 9475, 9484, 9485, 9554, 9555, 9564, 9565, 9635, 9636, 9644, 9715, 9716, 9724,
                9795, 9796, 9804, 9875, 9876, 9884, 11311, 11551, 11949, 11950, 12268, 12508,
                12908, 12988, 13068, 13148, 13149, 13228, 13308, 13388, 13468, 13549, 13550, 13629,
                13630, 13709, 13710, 13789, 13790, 13869, 13870, 13872, 13949, 13950, 13952, 14029,
                14030, 14108, 14109, 14111, 14189, 14191, 14269, 14271, 14272, 14349, 14351, 14429,
                14431, 14591, 14592, 14671, 14672, 14751,
            ],
        ),
        (
            "specialanimation_summer_spell_cast_start_south",
            &[70, 78],
            &[
                4435, 4444, 4515, 4524, 4594, 4595, 4604, 4674, 4675, 4684, 4754, 4755, 4764, 4765,
                4834, 4835, 4844, 4845, 4915, 4916, 4924, 4995, 4996, 5004, 5712, 5792, 6030, 6031,
                6110, 6111, 6189, 6269, 6430, 6508, 6509, 6588, 6589, 6590, 6668, 6748, 6909, 6910,
                6911, 6989, 7069, 7070, 7149, 7151, 7152, 7229, 7311, 7312, 7391,
            ],
        ),
        (
            "summer_action_east",
            &[53, 64, 63, 64, 63, 53, 58],
            &[
                15238, 15318, 15398, 15478, 15635, 15716, 15797, 15798, 15877, 15878, 15957, 15958,
                16037, 16038, 16116, 16194, 16195, 16275, 16276, 16357, 16358, 16360, 16368, 16437,
                16438, 16440, 16448, 16517, 16518, 16520, 16528, 16597, 16598, 16600, 16608, 16675,
                16676, 16754, 16755, 16757, 16765, 16835, 16836, 16838, 16846, 16918, 16919, 16920,
                16928, 16998, 16999, 17000, 17008, 17078, 17079, 17080, 17088, 17158, 17159, 17160,
                17168, 17235, 17236, 17238, 17246, 17315, 17316, 17317, 17325, 17396, 17397, 17398,
                17406, 17796, 17797, 17798, 17806, 19153, 19233, 19313, 19393, 19633, 20033, 20111,
                20271, 20272, 20351, 20352, 20431, 20432, 20511, 20512, 20751, 20752, 20830, 20910,
                20990, 21070, 21151, 21152, 21229, 21230, 21310, 21391, 21392, 21471, 21472, 21551,
                21552, 21631, 21632, 21710, 21788, 21871, 21950, 21951, 21952, 22030, 22031, 22032,
                22110, 22111, 22112, 22190, 22191, 22192, 22271, 22349, 22429, 22430, 22431, 22510,
                22590, 22670, 22750, 22829, 22830, 22831, 22907, 22908, 22909, 22989, 23071, 23072,
                23073, 23151, 23152, 23153, 23231, 23232, 23233, 23311, 23312, 23313, 23389, 23467,
                23550, 23551, 23631, 23632, 23711, 23712, 23791, 23792, 23871, 23872, 23950, 23951,
                24028, 24029, 24030, 24110, 24111, 24191, 24193, 24271, 24273, 24351, 24353, 24431,
                24433, 24510, 24511, 24588, 24589, 24670, 24753, 24754, 24833, 24834, 24913, 24914,
                24993, 24994, 25070, 25148, 25150, 25151, 25232, 25314, 25315, 25394, 25395, 25474,
                25475, 25554, 25555, 25632, 25710, 25711, 25712, 25713, 26271, 26272,
            ],
        ),
        (
            "summer_action_north",
            &[24, 29, 29, 29, 29, 27, 28],
            &[
                15234, 15245, 15314, 15325, 15394, 15405, 15474, 15485, 15714, 15725, 15794, 15805,
                15874, 15885, 15954, 15965, 16034, 16045, 16114, 16125, 16194, 16205, 16274, 16285,
                16674, 16685, 16754, 16765, 19647, 19728, 19808, 19888, 19968, 20047, 20127, 20288,
                20289, 20368, 20369, 20448, 20449, 20528, 20529, 20768, 20769, 21168, 21169, 21248,
                21249, 21330, 21408, 21409, 21488, 21489, 21568, 21569, 21648, 21649, 21730, 21810,
                21889, 21968, 21969, 21970, 22048, 22049, 22050, 22128, 22129, 22130, 22208, 22209,
                22210, 22289, 22369, 22449, 22450, 22451, 22530, 22610, 22690, 22770, 22849, 22850,
                22851, 22929, 22930, 22931, 23011, 23088, 23089, 23168, 23169, 23248, 23249, 23328,
                23329, 23411, 23491, 23569, 23570, 23647, 23648, 23649, 23650, 23727, 23728, 23729,
                23730, 23807, 23808, 23809, 23810, 23887, 23888, 23889, 23890, 23969, 23970, 24049,
                24050, 24128, 24129, 24130, 24131, 24206, 24209, 24286, 24289, 24366, 24369, 24446,
                24449, 24528, 24529, 24530, 24531, 24608, 24609, 24610, 24611, 24686, 24687, 24690,
                24763, 24764, 24765, 24766, 24767, 24769, 24843, 24844, 24845, 24846, 24847, 24849,
                24923, 24924, 24925, 24926, 24927, 24929, 25003, 25004, 25005, 25006, 25007, 25009,
                25086, 25087, 25090, 25166, 25167, 25170, 25243, 25244, 25245, 25246, 25247, 25248,
                25320, 25321, 25322, 25323, 25324, 25325, 25326, 25327, 25328, 25400, 25401, 25402,
                25403, 25404, 25405, 25406, 25407, 25408, 25480, 25481, 25482, 25483, 25484, 25485,
                25486, 25487, 25488, 25560, 25561, 25562, 25563, 25564, 25565, 25566, 25567, 25568,
                25643, 25644, 25645, 25646, 25647, 25648, 25723, 25724, 25725, 25726, 25727, 25728,
                25800, 25801, 25802, 25803, 25804, 25805, 25806, 25807, 25881, 25882, 25883, 25884,
                25885, 25961, 25962, 25963, 25964, 25965, 26041, 26042, 26043, 26044, 26045, 26121,
                26122, 26123, 26124, 26125, 26200, 26201, 26202, 26203, 26204, 26205, 26206, 26207,
                26280, 26281, 26282, 26283, 26284, 26285, 26286, 26287, 26361, 26362, 26363, 26364,
                26365, 26761, 26762, 26763, 26764, 26765, 26841, 26842, 26843, 26844, 26845,
            ],
        ),
        (
            "summer_action_south",
            &[58, 62, 61, 62, 61, 58, 66],
            &[
                15235, 15244, 15315, 15324, 15395, 15404, 15475, 15484, 15635, 15644, 15715, 15724,
                15794, 15795, 15804, 15874, 15875, 15884, 15954, 15955, 15964, 16034, 16035, 16044,
                16115, 16124, 16194, 16195, 16204, 16274, 16275, 16284, 16354, 16355, 16364, 16365,
                16434, 16435, 16444, 16445, 16514, 16515, 16524, 16525, 16594, 16595, 16604, 16605,
                16674, 16675, 16684, 16754, 16755, 16764, 16765, 16834, 16835, 16844, 16845, 16915,
                16916, 16924, 16995, 16996, 17004, 17075, 17076, 17084, 17155, 17156, 17164, 17234,
                17235, 17244, 17245, 17315, 17316, 17324, 17395, 17396, 17404, 17795, 17796, 17804,
                19711, 19791, 19871, 19951, 20112, 20192, 20592, 20829, 20830, 20909, 20910, 20989,
                20990, 21069, 21070, 21230, 21231, 21310, 21311, 21388, 21468, 21548, 21628, 21710,
                21711, 21789, 21869, 21949, 21950, 22029, 22030, 22109, 22110, 22189, 22190, 22269,
                22350, 22430, 22508, 22509, 22510, 22588, 22589, 22590, 22668, 22669, 22670, 22748,
                22749, 22750, 22830, 22908, 22909, 22910, 22988, 22989, 22990, 23068, 23148, 23228,
                23308, 23388, 23389, 23390, 23468, 23548, 23629, 23630, 23631, 23709, 23710, 23711,
                23789, 23790, 23791, 23869, 23870, 23871, 23948, 24029, 24030, 24031, 24109, 24110,
                24111, 24189, 24190, 24269, 24270, 24349, 24350, 24429, 24430, 24509, 24510, 24511,
                24589, 24590, 24669, 24670, 24749, 24751, 24752, 24829, 24831, 24832, 24909, 24911,
                24912, 24989, 24991, 24992, 25069, 25070, 25149, 25229, 25231, 25311, 25312, 25391,
                25392, 25471, 25472, 25551, 25552, 25629, 25631, 25711, 25791, 25792, 25872, 25952,
                26032, 26112, 26191, 26192,
            ],
        ),
        (
            "summer_blink_east",
            &[62, 66, 62],
            &[
                6515, 6595, 6675, 6754, 6755, 6834, 6835, 6914, 6915, 6994, 6995, 6997, 7005, 7074,
                7075, 7077, 7085, 7154, 7155, 7157, 7165, 7235, 7236, 7237, 7245, 7315, 7316, 7317,
                7325, 7395, 7396, 7397, 7405, 8431, 8511, 8591, 8909, 8910, 8989, 8990, 9069, 9070,
                9148, 9228, 9308, 9389, 9469, 9549, 9627, 9628, 9629, 9707, 9708, 9709, 9787, 9788,
                9789, 9867, 9947, 10027, 10108, 10109, 10110, 10188, 10189, 10190, 10268, 10269,
                10270, 10348, 10349, 10428, 10429, 10508, 10509, 10588, 10590, 10591, 10668, 10670,
                10671, 10748, 10750, 10751, 10830, 10831, 10832, 10833, 10910, 10911, 10912, 10913,
                10990, 10991, 10992, 10993, 11071, 11072, 11151, 11152, 11231, 11232,
            ],
        ),
        (
            "summer_blink_south",
            &[70, 74, 70],
            &[
                6515, 6524, 6595, 6604, 6675, 6684, 6754, 6755, 6764, 6834, 6835, 6844, 6914, 6915,
                6924, 6994, 6995, 7004, 7005, 7074, 7075, 7084, 7085, 7154, 7155, 7164, 7165, 7235,
                7236, 7244, 7315, 7316, 7324, 7395, 7396, 7404, 8432, 8512, 8592, 8910, 8911, 8990,
                8991, 9070, 9071, 9149, 9229, 9309, 9390, 9470, 9550, 9628, 9629, 9630, 9708, 9709,
                9710, 9788, 9789, 9790, 9868, 9948, 10028, 10109, 10110, 10111, 10189, 10190,
                10191, 10269, 10270, 10271, 10349, 10350, 10429, 10430, 10509, 10510, 10589, 10669,
                10749, 10831, 10911, 10991,
            ],
        ),
        (
            "summer_drink_east",
            &[49, 56, 49],
            &[
                7235, 7312, 7395, 7474, 7475, 7551, 7552, 7563, 7634, 7635, 7714, 7715, 7717, 7725,
                7792, 7794, 7795, 7803, 7874, 7875, 7877, 7885, 7955, 7956, 7957, 7965, 8033, 8034,
                8035, 8115, 8116, 8117, 8125, 9152, 9153, 9230, 9312, 9313, 9391, 9469, 9551, 9710,
                9870, 9871, 9949, 9950, 10030, 10031, 10110, 10189, 10270,
            ],
        ),
        (
            "summer_drink_north",
            &[2, 2, 2],
            &[
                7474, 7485, 7634, 7645, 7714, 7725, 7794, 7805, 7874, 7885, 8034, 8045, 8686, 8846,
                9007, 9167, 9168, 9327, 9328, 9409, 9488, 9489, 9569, 9648, 9808, 9888, 9889, 9890,
                9968, 9969, 10048, 10049, 10050, 10130, 10208, 10209, 10210, 10290, 10368, 10369,
                10450, 10528, 10529, 10607, 10608, 10609, 10610, 10688, 10689, 10767, 10768, 10769,
                10770, 10845, 10846, 10849, 10927, 10928, 10929, 10930, 11005, 11006, 11009, 11082,
                11083, 11084, 11085, 11086, 11087, 11164, 11165, 11166, 11169, 11242, 11243, 11244,
                11245, 11246, 11247, 11320, 11321, 11322, 11323, 11324, 11325, 11326, 11400, 11401,
                11402, 11403, 11404, 11405, 11406, 11407, 11480, 11481, 11482, 11483, 11484, 11485,
                11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646, 11647,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "summer_drink_south",
            &[58, 62, 58],
            &[
                7235, 7244, 7395, 7404, 7474, 7475, 7484, 7555, 7564, 7634, 7635, 7644, 7714, 7715,
                7724, 7725, 7794, 7795, 7804, 7874, 7875, 7884, 7885, 7955, 7956, 7964, 8034, 8035,
                8044, 8045, 8115, 8116, 8124, 8275, 8276, 8284, 8992, 9151, 9152, 9311, 9312, 9390,
                9470, 9471, 9472, 9550, 9631, 9709, 9791, 9869, 9870, 9871, 9950, 9951, 10029,
                10030, 10031, 10109, 10189, 10190, 10191, 10269, 10350, 10351, 10429, 10510, 10511,
                10590, 10591, 10670, 10671, 10750, 10751, 10830, 10910, 10911, 10990, 11072, 11150,
                11232, 11392,
            ],
        ),
        (
            "summer_eat_east",
            &[47, 51, 45, 51, 49],
            &[
                11794, 12035, 12116, 12193, 12194, 12205, 12355, 12434, 12435, 12515, 12516, 12594,
                12596, 12597, 12605, 12675, 12754, 12755, 12834, 12835, 12837, 12845, 12915, 12916,
                12918, 12926, 12995, 12996, 12997, 13074, 13075, 13154, 13155, 13157, 13165, 13235,
                13236, 13237, 13245, 13316, 13317, 13318, 13326, 13474, 13475, 13477, 13485, 13555,
                13556, 13557, 13565, 13875, 13876, 13877, 13885, 14513, 14514, 14592, 14593, 15073,
                15232, 15233, 15312, 15313, 15391, 15392, 15552, 15553, 15631, 15711, 15790, 15871,
                15872, 15873, 15951, 16112, 16191, 16270, 16430, 16431, 16510, 16511, 16512, 16590,
                16591, 16671, 16750, 16751, 16830, 16910, 16990, 17070, 17071, 17150, 17311, 17470,
                17711, 18111,
            ],
        ),
        (
            "summer_eat_north",
            &[2, 2, 2],
            &[
                7474, 7485, 7634, 7645, 7714, 7725, 7794, 7805, 7874, 7885, 8034, 8045, 8686, 8846,
                9007, 9167, 9168, 9327, 9328, 9409, 9488, 9489, 9569, 9648, 9808, 9888, 9889, 9890,
                9968, 9969, 10048, 10049, 10050, 10130, 10208, 10209, 10210, 10290, 10368, 10369,
                10450, 10528, 10529, 10607, 10608, 10609, 10610, 10688, 10689, 10767, 10768, 10769,
                10770, 10845, 10846, 10849, 10927, 10928, 10929, 10930, 11005, 11006, 11009, 11082,
                11083, 11084, 11085, 11086, 11087, 11164, 11165, 11166, 11169, 11242, 11243, 11244,
                11245, 11246, 11247, 11320, 11321, 11322, 11323, 11324, 11325, 11326, 11400, 11401,
                11402, 11403, 11404, 11405, 11406, 11407, 11480, 11481, 11482, 11483, 11484, 11485,
                11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646, 11647,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "summer_eat_south",
            &[58, 56, 57, 67, 56],
            &[
                11795, 11804, 12035, 12044, 12194, 12195, 12204, 12355, 12364, 12434, 12435, 12444,
                12515, 12524, 12594, 12595, 12604, 12605, 12675, 12684, 12754, 12755, 12764, 12834,
                12835, 12844, 12845, 12914, 12915, 12924, 12995, 12996, 13004, 13074, 13075, 13084,
                13154, 13155, 13164, 13165, 13235, 13236, 13244, 13314, 13315, 13324, 13325, 13474,
                13475, 13484, 13485, 13555, 13556, 13564, 13715, 13716, 13724, 13875, 13876, 13884,
                14591, 14592, 14912, 15072, 15231, 15232, 15390, 15391, 15392, 15551, 15552, 15630,
                15710, 15711, 15712, 15789, 15870, 15871, 15872, 15950, 16031, 16190, 16191, 16351,
                16429, 16430, 16431, 16510, 16511, 16589, 16590, 16591, 16670, 16671, 16749, 16750,
                16751, 16829, 16909, 16910, 16911, 16989, 17069, 17070, 17071, 17149, 17230, 17231,
                17309, 17390, 17391, 17469, 17550, 17551, 17630, 17631, 17710, 17711, 17790, 17791,
                17870, 17871, 17950, 17951, 18030, 18110, 18111, 18190, 18270, 18271, 18350, 18432,
                18510, 18592, 18670, 18912, 19072,
            ],
        ),
        (
            "summer_idle_east",
            &[58],
            &[
                2195, 2274, 2275, 2354, 2355, 2357, 2365, 2435, 2436, 2437, 2445, 2831, 2989, 2990,
                3068, 3149, 3227, 3228, 3229, 3307, 3388, 3389, 3390, 3468, 3469, 3548, 3550, 3551,
                3630, 3631, 3632, 3633, 3711, 3712,
            ],
        ),
        (
            "summer_idle_north",
            &[28],
            &[
                2274, 2285, 2354, 2365, 2847, 3008, 3009, 3090, 3169, 3249, 3250, 3251, 3331, 3409,
                3410, 3488, 3489, 3490, 3491, 3566, 3567, 3570, 3643, 3644, 3645, 3646, 3647, 3648,
                3720, 3721, 3722, 3723, 3724, 3725, 3726, 3727, 3801, 3802, 3803, 3804, 3805,
            ],
        ),
        (
            "summer_idle_south",
            &[66],
            &[
                2195, 2204, 2274, 2275, 2284, 2354, 2355, 2364, 2365, 2435, 2436, 2444, 2832, 2990,
                2991, 3069, 3150, 3228, 3229, 3230, 3308, 3389, 3390, 3391, 3469, 3470, 3549, 3631,
            ],
        ),
        (
            "summer_kiss_east",
            &[50, 55, 67, 59],
            &[
                8838, 8994, 9076, 9157, 9158, 9236, 9313, 9314, 9395, 9396, 9477, 9478, 9480, 9488,
                9555, 9556, 9633, 9634, 9636, 9644, 9715, 9716, 9718, 9726, 9798, 9799, 9800, 9808,
                9875, 9876, 9878, 9886, 9954, 9955, 9956, 9964, 10036, 10037, 10038, 10046, 10196,
                10197, 10198, 10206, 11070, 11550, 11630, 11710, 11711, 11790, 12029, 12188, 12189,
                12269, 12270, 12350, 12351, 12429, 12430, 12507, 12588, 12669, 12670, 12671, 12748,
                12828, 12829, 12909, 12910, 12989, 13069, 13070, 13147, 13148, 13149, 13228, 13229,
                13230, 13310, 13311, 13388, 13389, 13390, 13467, 13548, 13630, 13631, 13708, 13788,
                13789, 13869, 13870, 13950, 13952, 14029, 14030, 14107, 14108, 14109, 14189, 14190,
                14272, 14273, 14274, 14349, 14350, 14428, 14430, 14509, 14511, 14592, 14593, 14669,
                14671, 14750, 14751, 14752, 14831, 14832, 14833, 14991, 14992, 14993, 15070, 15071,
                15151, 15152, 15311, 15312,
            ],
        ),
        (
            "summer_sit_east",
            &[49],
            &[
                2435, 2514, 2515, 2594, 2595, 2597, 2605, 2675, 2676, 2677, 2685, 3072, 3073, 3151,
                3310, 3311, 3390,
            ],
        ),
        (
            "summer_sit_north",
            &[2],
            &[
                2514, 2525, 2594, 2605, 2926, 3087, 3088, 3169, 3248, 3328, 3329, 3330, 3410, 3488,
                3489, 3567, 3568, 3569, 3570, 3645, 3646, 3649, 3722, 3723, 3724, 3725, 3726, 3727,
                3800, 3801, 3802, 3803, 3804, 3805, 3806, 3881, 3882, 3883, 3884,
            ],
        ),
        (
            "summer_sit_south",
            &[56],
            &[
                2435, 2444, 2514, 2515, 2524, 2594, 2595, 2604, 2605, 2675, 2676, 2684, 3071, 3072,
                3150, 3231, 3309, 3310, 3311, 3389, 3470, 3471, 3550, 3551, 3630,
            ],
        ),
        (
            "summer_sleep_east",
            &[55],
            &[
                2275, 2354, 2355, 2434, 2435, 2437, 2445, 2515, 2516, 2517, 2525,
            ],
        ),
        (
            "summer_walk_east",
            &[58, 59, 58, 56],
            &[
                8675, 8835, 8994, 8995, 9075, 9154, 9155, 9235, 9314, 9315, 9317, 9325, 9394, 9395,
                9474, 9475, 9477, 9485, 9554, 9555, 9635, 9636, 9637, 9645, 9714, 9715, 9717, 9725,
                9795, 9796, 9797, 9805, 9874, 9875, 9877, 9885, 10035, 10036, 10037, 10045, 10195,
                10196, 10197, 10205, 11231, 11391, 11632, 11869, 11870, 12029, 12030, 12109, 12188,
                12270, 12271, 12348, 12428, 12429, 12509, 12589, 12669, 12827, 12828, 12829, 12910,
                12987, 12988, 12989, 13068, 13069, 13147, 13228, 13229, 13230, 13307, 13387, 13388,
                13389, 13468, 13469, 13470, 13548, 13628, 13629, 13630, 13707, 13788, 13789, 13869,
                13870, 13948, 13949, 14028, 14029, 14108, 14110, 14111, 14188, 14189, 14268, 14270,
                14271, 14347, 14348, 14349, 14430, 14431, 14432, 14433, 14510, 14511, 14590, 14591,
                14592, 14593, 14668, 14670, 14671, 14751, 14752, 14830, 14831, 14832, 14833, 14911,
                14912, 14988, 14990, 14991, 14992, 15151, 15152, 15153, 15309, 15310, 15311, 15312,
                15473, 15631,
            ],
        ),
        (
            "summer_walk_north",
            &[28, 20, 28, 23],
            &[
                8994, 9005, 9154, 9165, 9314, 9325, 9394, 9405, 9474, 9485, 9554, 9565, 9714, 9725,
                9874, 9885, 11247, 11407, 11646, 11888, 11889, 12048, 12049, 12128, 12210, 12287,
                12288, 12370, 12448, 12449, 12529, 12609, 12689, 12849, 12850, 12851, 12928, 13009,
                13010, 13011, 13088, 13089, 13171, 13248, 13249, 13250, 13331, 13408, 13409, 13410,
                13489, 13490, 13570, 13649, 13650, 13730, 13808, 13809, 13810, 13811, 13888, 13889,
                13968, 13969, 13970, 13971, 14048, 14049, 14126, 14127, 14130, 14207, 14209, 14210,
                14286, 14287, 14290, 14367, 14368, 14369, 14370, 14443, 14444, 14445, 14446, 14447,
                14448, 14526, 14527, 14528, 14603, 14604, 14605, 14606, 14607, 14608, 14686, 14689,
                14760, 14761, 14762, 14763, 14764, 14765, 14766, 14767, 14843, 14844, 14845, 14846,
                14847, 14848, 14920, 14921, 14922, 14923, 14924, 14925, 14926, 14927, 15003, 15004,
                15005, 15006, 15007, 15009, 15081, 15082, 15083, 15084, 15085, 15160, 15161, 15162,
                15163, 15164, 15165, 15166, 15167, 15241, 15242, 15243, 15244, 15245, 15320, 15321,
                15322, 15323, 15324, 15325, 15326, 15327, 15328, 15481, 15482, 15483, 15484, 15485,
                15641, 15642, 15643, 15644, 15645,
            ],
        ),
        (
            "summer_walk_south",
            &[66, 60, 66, 59],
            &[
                8675, 8684, 8835, 8844, 8994, 8995, 9004, 9075, 9084, 9154, 9155, 9164, 9235, 9244,
                9314, 9315, 9324, 9325, 9394, 9395, 9404, 9474, 9475, 9484, 9485, 9554, 9555, 9564,
                9635, 9636, 9644, 9714, 9715, 9724, 9725, 9795, 9796, 9804, 9874, 9875, 9884, 9885,
                10035, 10036, 10044, 10195, 10196, 10204, 11232, 11392, 11870, 11871, 12030, 12031,
                12111, 12189, 12271, 12272, 12349, 12430, 12431, 12510, 12590, 12670, 12828, 12829,
                12830, 12911, 12988, 12989, 12990, 13070, 13148, 13229, 13230, 13231, 13308, 13389,
                13390, 13469, 13470, 13471, 13549, 13629, 13630, 13631, 13709, 13789, 13790, 13870,
                13871, 13949, 13950, 14030, 14031, 14109, 14189, 14190, 14269, 14349, 14350, 14431,
                14511, 14591, 14671, 14672, 14831, 14832, 14991, 14992, 15152, 15311, 15312,
            ],
        ),
        (
            "wedding_action_east",
            &[46, 56, 53, 56, 53, 46, 49],
            &[
                15238, 15318, 15398, 15478, 15635, 15716, 15797, 15798, 15877, 15878, 15957, 15958,
                16037, 16038, 16116, 16194, 16195, 16275, 16276, 16357, 16358, 16360, 16368, 16437,
                16438, 16440, 16448, 16517, 16518, 16520, 16528, 16597, 16598, 16600, 16608, 16675,
                16676, 16754, 16755, 16757, 16765, 16835, 16836, 16838, 16846, 16918, 16919, 16920,
                16928, 16998, 16999, 17000, 17008, 17078, 17079, 17080, 17088, 17158, 17159, 17160,
                17168, 17235, 17236, 17238, 17246, 17315, 17316, 17317, 17325, 17396, 17397, 17398,
                17406, 17796, 17797, 17798, 17806, 20271, 20272, 20351, 20352, 20431, 20432, 20511,
                20512, 20751, 20830, 20910, 20990, 21070, 21151, 21229, 21310, 21391, 21392, 21471,
                21472, 21551, 21552, 21631, 21632, 21710, 21788, 21871, 21950, 21951, 21952, 22030,
                22031, 22032, 22110, 22111, 22112, 22190, 22191, 22192, 22271, 22349, 22429, 22430,
                22431, 22510, 22590, 22670, 22750, 22829, 22830, 22831, 22907, 22908, 22909, 22989,
                23071, 23072, 23073, 23151, 23152, 23153, 23231, 23232, 23233, 23311, 23312, 23313,
                23389, 23467, 23550, 23551, 23552, 23631, 23632, 23634, 23711, 23712, 23714, 23791,
                23792, 23794, 23871, 23872, 23874, 23950, 23951, 23952, 24028, 24029, 24030, 24110,
                24111, 24113, 24191, 24193, 24194, 24195, 24196, 24271, 24273, 24274, 24275, 24276,
                24351, 24353, 24354, 24355, 24356, 24431, 24433, 24434, 24435, 24436, 24510, 24511,
                24513, 24588, 24589, 24591, 24670, 24672, 24673, 24674, 24675, 24753, 24754, 24755,
                24833, 24834, 24835, 24913, 24914, 24915, 24993, 24994, 24995, 25070, 25072, 25073,
                25074, 25075, 25148, 25150, 25151, 25152, 25232, 25233, 25234, 25235, 25314, 25315,
                25394, 25395, 25474, 25475, 25554, 25555, 25632, 25633, 25634, 25635, 25710, 25711,
                25712, 25713, 25793, 25794, 25795, 26193, 26194, 26195, 26271, 26272, 26273, 26833,
            ],
        ),
        (
            "wedding_action_north",
            &[20, 21, 21, 21, 21, 16, 19],
            &[
                15234, 15245, 15314, 15325, 15394, 15405, 15474, 15485, 15714, 15725, 15794, 15805,
                15874, 15885, 15954, 15965, 16034, 16045, 16114, 16125, 16194, 16205, 16274, 16285,
                16674, 16685, 16754, 16765, 20769, 21169, 21249, 21330, 21408, 21409, 21488, 21489,
                21568, 21569, 21648, 21649, 21730, 21810, 21889, 21968, 21969, 21970, 22048, 22049,
                22050, 22128, 22129, 22130, 22208, 22209, 22210, 22289, 22369, 22449, 22450, 22451,
                22530, 22610, 22690, 22770, 22849, 22850, 22851, 22929, 22930, 22931, 23011, 23088,
                23089, 23168, 23169, 23248, 23249, 23328, 23329, 23411, 23491, 23569, 23570, 23647,
                23648, 23649, 23650, 23727, 23728, 23729, 23730, 23807, 23808, 23809, 23810, 23887,
                23888, 23889, 23890, 23969, 23970, 24049, 24050, 24128, 24129, 24130, 24131, 24206,
                24209, 24286, 24289, 24366, 24369, 24446, 24449, 24528, 24529, 24530, 24531, 24608,
                24609, 24610, 24611, 24686, 24687, 24690, 24763, 24764, 24765, 24766, 24767, 24769,
                24843, 24844, 24845, 24846, 24847, 24849, 24923, 24924, 24925, 24926, 24927, 24929,
                25003, 25004, 25005, 25006, 25007, 25009, 25086, 25087, 25090, 25166, 25167, 25170,
                25243, 25244, 25245, 25246, 25247, 25248, 25320, 25321, 25322, 25323, 25324, 25325,
                25326, 25327, 25328, 25400, 25401, 25402, 25403, 25404, 25405, 25406, 25407, 25408,
                25480, 25481, 25482, 25483, 25484, 25485, 25486, 25487, 25488, 25560, 25561, 25562,
                25563, 25564, 25565, 25566, 25567, 25568, 25643, 25644, 25645, 25646, 25647, 25648,
                25723, 25724, 25725, 25726, 25727, 25728, 25800, 25801, 25802, 25803, 25804, 25805,
                25806, 25807, 25881, 25882, 25883, 25884, 25885, 25961, 25962, 25963, 25964, 25965,
                26041, 26042, 26043, 26044, 26045, 26121, 26122, 26123, 26124, 26125, 26200, 26201,
                26202, 26203, 26204, 26205, 26206, 26207, 26280, 26281, 26282, 26283, 26284, 26285,
                26286, 26287, 26361, 26362, 26363, 26364, 26365, 26761, 26762, 26763, 26764, 26765,
                26841, 26842, 26843, 26844, 26845,
            ],
        ),
        (
            "wedding_action_south",
            &[49, 53, 53, 53, 53, 49, 56],
            &[
                15235, 15244, 15315, 15324, 15395, 15404, 15475, 15484, 15635, 15644, 15715, 15724,
                15794, 15795, 15804, 15874, 15875, 15884, 15954, 15955, 15964, 16034, 16035, 16044,
                16115, 16124, 16194, 16195, 16204, 16274, 16275, 16284, 16354, 16355, 16364, 16365,
                16434, 16435, 16444, 16445, 16514, 16515, 16524, 16525, 16594, 16595, 16604, 16605,
                16674, 16675, 16684, 16754, 16755, 16764, 16765, 16834, 16835, 16844, 16845, 16915,
                16916, 16924, 16995, 16996, 17004, 17075, 17076, 17084, 17155, 17156, 17164, 17234,
                17235, 17244, 17245, 17315, 17316, 17324, 17395, 17396, 17404, 17795, 17796, 17804,
                20829, 20909, 20989, 21069, 21230, 21310, 21388, 21468, 21548, 21628, 21710, 21789,
                21869, 21949, 21950, 22029, 22030, 22109, 22110, 22189, 22190, 22269, 22350, 22430,
                22508, 22509, 22510, 22588, 22589, 22590, 22668, 22669, 22670, 22748, 22749, 22750,
                22830, 22908, 22909, 22910, 22988, 22989, 22990, 23068, 23148, 23228, 23308, 23388,
                23389, 23390, 23468, 23548, 23629, 23630, 23631, 23709, 23710, 23711, 23789, 23790,
                23791, 23869, 23870, 23871, 23948, 24029, 24030, 24031, 24109, 24110, 24111, 24189,
                24190, 24192, 24269, 24270, 24272, 24349, 24350, 24352, 24429, 24430, 24432, 24509,
                24510, 24511, 24589, 24590, 24669, 24670, 24749, 24751, 24752, 24829, 24831, 24832,
                24833, 24909, 24911, 24912, 24989, 24991, 24992, 24993, 25069, 25070, 25149, 25229,
                25231, 25311, 25312, 25391, 25392, 25393, 25471, 25472, 25551, 25552, 25553, 25629,
                25631, 25711, 25791, 25792, 25872, 25952, 26032, 26112, 26191, 26192, 26352, 26752,
            ],
        ),
        (
            "wedding_blink_east",
            &[53, 57, 53],
            &[
                6515, 6595, 6675, 6754, 6755, 6834, 6835, 6914, 6915, 6994, 6995, 6997, 7005, 7074,
                7075, 7077, 7085, 7154, 7155, 7157, 7165, 7235, 7236, 7237, 7245, 7315, 7316, 7317,
                7325, 7395, 7396, 7397, 7405, 8909, 8989, 9069, 9148, 9228, 9308, 9389, 9469, 9549,
                9627, 9628, 9629, 9707, 9708, 9709, 9787, 9788, 9789, 9867, 9947, 10027, 10108,
                10109, 10110, 10188, 10189, 10190, 10268, 10269, 10270, 10348, 10349, 10351, 10428,
                10429, 10431, 10508, 10509, 10511, 10588, 10590, 10591, 10592, 10668, 10670, 10671,
                10672, 10748, 10750, 10751, 10752, 10830, 10831, 10832, 10833, 10910, 10911, 10912,
                10913, 10990, 10991, 10992, 10993, 11071, 11072, 11073, 11151, 11152, 11153, 11231,
                11232, 11233, 11313, 11393, 11473,
            ],
        ),
        (
            "wedding_blink_south",
            &[60, 64, 60],
            &[
                6515, 6524, 6595, 6604, 6675, 6684, 6754, 6755, 6764, 6834, 6835, 6844, 6914, 6915,
                6924, 6994, 6995, 7004, 7005, 7074, 7075, 7084, 7085, 7154, 7155, 7164, 7165, 7235,
                7236, 7244, 7315, 7316, 7324, 7395, 7396, 7404, 8910, 8990, 9070, 9149, 9229, 9309,
                9390, 9470, 9550, 9628, 9629, 9630, 9708, 9709, 9710, 9788, 9789, 9790, 9868, 9948,
                10028, 10109, 10110, 10111, 10189, 10190, 10191, 10269, 10270, 10271, 10349, 10350,
                10429, 10430, 10509, 10510, 10589, 10669, 10749, 10831, 10911, 10991,
            ],
        ),
        (
            "wedding_idle_east",
            &[49],
            &[
                2195, 2274, 2275, 2354, 2355, 2357, 2365, 2435, 2436, 2437, 2445, 2989, 3068, 3149,
                3227, 3228, 3229, 3307, 3388, 3389, 3390, 3468, 3469, 3471, 3548, 3550, 3551, 3552,
                3630, 3631, 3632, 3633, 3711, 3712, 3713, 3793,
            ],
        ),
        (
            "wedding_idle_north",
            &[19],
            &[
                2274, 2285, 2354, 2365, 3009, 3090, 3169, 3249, 3250, 3251, 3331, 3409, 3410, 3488,
                3489, 3490, 3491, 3566, 3567, 3570, 3643, 3644, 3645, 3646, 3647, 3648, 3720, 3721,
                3722, 3723, 3724, 3725, 3726, 3727, 3801, 3802, 3803, 3804, 3805,
            ],
        ),
        (
            "wedding_idle_south",
            &[56],
            &[
                2195, 2204, 2274, 2275, 2284, 2354, 2355, 2364, 2365, 2435, 2436, 2444, 2990, 3069,
                3150, 3228, 3229, 3230, 3308, 3389, 3390, 3391, 3469, 3470, 3549, 3631,
            ],
        ),
        (
            "wedding_kiss_east",
            &[45, 49, 61, 52],
            &[
                8838, 8994, 9076, 9157, 9158, 9236, 9313, 9314, 9395, 9396, 9477, 9478, 9480, 9488,
                9555, 9556, 9633, 9634, 9636, 9644, 9715, 9716, 9718, 9726, 9798, 9799, 9800, 9808,
                9875, 9876, 9878, 9886, 9954, 9955, 9956, 9964, 10036, 10037, 10038, 10046, 10196,
                10197, 10198, 10206, 11709, 11710, 12029, 12188, 12268, 12350, 12351, 12428, 12507,
                12588, 12669, 12670, 12671, 12748, 12828, 12829, 12909, 12910, 12989, 13069, 13070,
                13147, 13148, 13149, 13228, 13229, 13230, 13310, 13311, 13312, 13388, 13389, 13390,
                13467, 13548, 13630, 13631, 13633, 13708, 13788, 13789, 13869, 13870, 13950, 13952,
                13953, 14029, 14030, 14107, 14108, 14109, 14189, 14190, 14272, 14273, 14349, 14350,
                14428, 14430, 14431, 14509, 14511, 14512, 14513, 14592, 14593, 14669, 14671, 14672,
                14673, 14750, 14751, 14752, 14753, 14831, 14832, 14833, 14913, 14991, 14992, 14993,
                15070, 15071, 15072, 15073, 15151, 15152, 15153, 15311, 15312, 15313, 15392, 15393,
            ],
        ),
        (
            "wedding_sit_east",
            &[41],
            &[
                2435, 2514, 2515, 2594, 2595, 2597, 2605, 2675, 2676, 2677, 2685, 3072, 3151, 3232,
                3310, 3311, 3312, 3390, 3471, 3472, 3473, 3551, 3552, 3554, 3631, 3633,
            ],
        ),
        (
            "wedding_sit_north",
            &[3],
            &[
                2514, 2525, 2594, 2605, 3088, 3169, 3248, 3328, 3329, 3330, 3410, 3488, 3489, 3567,
                3568, 3569, 3570, 3645, 3646, 3649, 3722, 3723, 3724, 3725, 3726, 3727, 3800, 3801,
                3802, 3803, 3804, 3805, 3806, 3881, 3882, 3883, 3884,
            ],
        ),
        (
            "wedding_sit_south",
            &[49],
            &[
                2435, 2444, 2514, 2515, 2524, 2594, 2595, 2604, 2605, 2675, 2676, 2684, 3071, 3150,
                3231, 3309, 3310, 3311, 3389, 3470, 3471, 3472, 3550, 3551, 3553, 3630, 3632,
            ],
        ),
        (
            "wedding_walk_east",
            &[49, 45, 49, 50],
            &[
                8675, 8835, 8994, 8995, 9075, 9154, 9155, 9235, 9314, 9315, 9317, 9325, 9394, 9395,
                9474, 9475, 9477, 9485, 9554, 9555, 9635, 9636, 9637, 9645, 9714, 9715, 9717, 9725,
                9795, 9796, 9797, 9805, 9874, 9875, 9877, 9885, 10035, 10036, 10037, 10045, 10195,
                10196, 10197, 10205, 11869, 12029, 12188, 12270, 12348, 12509, 12589, 12669, 12747,
                12827, 12828, 12829, 12910, 12987, 12988, 12989, 13068, 13069, 13147, 13228, 13229,
                13230, 13307, 13387, 13388, 13389, 13468, 13469, 13470, 13548, 13628, 13629, 13630,
                13707, 13788, 13789, 13791, 13869, 13870, 13948, 13949, 13951, 14028, 14029, 14108,
                14110, 14111, 14112, 14188, 14189, 14268, 14270, 14271, 14272, 14347, 14348, 14349,
                14430, 14431, 14432, 14433, 14510, 14511, 14590, 14591, 14592, 14593, 14668, 14670,
                14671, 14751, 14752, 14753, 14830, 14831, 14832, 14833, 14834, 14911, 14912, 14913,
                14988, 14990, 14991, 14992, 14993, 14994, 15073, 15151, 15152, 15153, 15233, 15309,
                15310, 15311, 15312, 15313, 15473, 15631, 15632, 15633,
            ],
        ),
        (
            "wedding_walk_north",
            &[19, 19, 19, 13],
            &[
                8994, 9005, 9154, 9165, 9314, 9325, 9394, 9405, 9474, 9485, 9554, 9565, 9714, 9725,
                9874, 9885, 11889, 12049, 12210, 12288, 12370, 12529, 12609, 12689, 12849, 12850,
                12851, 12928, 13009, 13010, 13011, 13088, 13089, 13171, 13248, 13249, 13250, 13331,
                13408, 13409, 13410, 13489, 13490, 13570, 13649, 13650, 13730, 13808, 13809, 13810,
                13811, 13888, 13889, 13968, 13969, 13970, 13971, 14048, 14049, 14126, 14127, 14130,
                14207, 14209, 14210, 14286, 14287, 14290, 14367, 14368, 14369, 14370, 14443, 14444,
                14445, 14446, 14447, 14448, 14526, 14527, 14528, 14603, 14604, 14605, 14606, 14607,
                14608, 14686, 14689, 14760, 14761, 14762, 14763, 14764, 14765, 14766, 14767, 14843,
                14844, 14845, 14846, 14847, 14848, 14920, 14921, 14922, 14923, 14924, 14925, 14926,
                14927, 15003, 15004, 15005, 15006, 15007, 15009, 15081, 15082, 15083, 15084, 15085,
                15160, 15161, 15162, 15163, 15164, 15165, 15166, 15167, 15241, 15242, 15243, 15244,
                15245, 15320, 15321, 15322, 15323, 15324, 15325, 15326, 15327, 15328, 15481, 15482,
                15483, 15484, 15485, 15641, 15642, 15643, 15644, 15645,
            ],
        ),
        (
            "wedding_walk_south",
            &[56, 49, 56, 53],
            &[
                8675, 8684, 8835, 8844, 8994, 8995, 9004, 9075, 9084, 9154, 9155, 9164, 9235, 9244,
                9314, 9315, 9324, 9325, 9394, 9395, 9404, 9474, 9475, 9484, 9485, 9554, 9555, 9564,
                9635, 9636, 9644, 9714, 9715, 9724, 9725, 9795, 9796, 9804, 9874, 9875, 9884, 9885,
                10035, 10036, 10044, 10195, 10196, 10204, 11870, 12030, 12189, 12271, 12349, 12510,
                12590, 12670, 12749, 12828, 12829, 12830, 12911, 12988, 12989, 12990, 13070, 13071,
                13148, 13229, 13230, 13231, 13308, 13389, 13390, 13391, 13469, 13470, 13471, 13549,
                13629, 13630, 13631, 13709, 13789, 13790, 13870, 13871, 13949, 13950, 14030, 14031,
                14109, 14189, 14190, 14269, 14349, 14350, 14431, 14511, 14591, 14671, 14672, 14831,
                14832, 14991, 14992, 15152, 15311, 15312,
            ],
        ),
        (
            "specialanimation_winter_eyes_closed_east",
            &[56],
            &[
                2195, 2274, 2275, 2354, 2355, 2357, 2365, 2435, 2436, 2437, 2445, 2831, 2989, 2990,
                3068, 3149, 3227, 3228, 3229, 3307, 3388, 3389, 3390, 3468, 3469, 3548, 3550, 3551,
                3630, 3631, 3632, 3633, 3711, 3712, 3713, 3793,
            ],
        ),
        (
            "specialanimation_winter_eyes_closed_south",
            &[64],
            &[
                2195, 2204, 2274, 2275, 2284, 2354, 2355, 2364, 2365, 2435, 2436, 2444, 2832, 2990,
                2991, 3069, 3150, 3228, 3229, 3230, 3308, 3389, 3390, 3391, 3469, 3470, 3549, 3631,
            ],
        ),
        (
            "specialanimation_winter_harvest_east",
            &[55, 69, 75, 74, 75, 74, 75, 74, 69, 55],
            &[],
        ),
        (
            "specialanimation_winter_lute_play_end_south",
            &[36, 51],
            &[
                4355, 4364, 4435, 4514, 4515, 4524, 4594, 4595, 4674, 4675, 4684, 4685, 4754, 4755,
                4835, 4844, 4915, 4916, 5632, 5712, 5950, 5951, 6030, 6031, 6109, 6189, 6270, 6350,
                6428, 6429, 6430, 6508, 6509, 6510, 6588, 6668, 6749, 6750, 6751, 6829, 6830, 6831,
                6909, 6910, 6989, 6990, 7069, 7071, 7072, 7149, 7151, 7152, 7231, 7232, 7233, 7311,
                7312, 7313, 7392, 7472,
            ],
        ),
        (
            "specialanimation_winter_lute_play_loop_south",
            &[69, 69, 69, 65, 65, 65, 69, 69, 69, 69],
            &[
                21635, 21644, 21715, 21724, 21795, 21804, 22115, 22124, 22195, 22204, 22275, 22284,
                22355, 22364, 22434, 22435, 22444, 22514, 22515, 22524, 22594, 22595, 22604, 22675,
                22684, 22755, 22764, 22835, 22844, 22914, 22915, 22924, 22994, 22995, 23004, 23074,
                23075, 23084, 23154, 23155, 23164, 23234, 23235, 23244, 23245, 23314, 23315, 23324,
                23325, 23394, 23395, 23404, 23405, 23474, 23475, 23484, 23554, 23555, 23564, 23634,
                23635, 23644, 23714, 23715, 23724, 23725, 23794, 23795, 23804, 23805, 23874, 23875,
                23884, 23885, 23954, 23955, 23964, 23965, 24035, 24036, 24044, 24115, 24116, 24124,
                24195, 24196, 24204, 24274, 24275, 24284, 24285, 24354, 24355, 24364, 24365, 24434,
                24435, 24444, 24445, 24515, 24516, 24524, 24595, 24596, 24604, 24675, 24676, 24684,
                24755, 24756, 24764, 25075, 25076, 25084, 25155, 25156, 25164, 25235, 25236, 25244,
                29630, 29631, 29710, 29711, 29790, 29791, 30110, 30111, 30190, 30191, 30270, 30271,
                30350, 30351, 30429, 30509, 30589, 30670, 30750, 30830, 30909, 30989, 31069, 31149,
                31230, 31310, 31390, 31469, 31549, 31629, 31710, 31790, 31870, 31950, 32028, 32029,
                32030, 32108, 32109, 32110, 32188, 32189, 32190, 32270, 32350, 32430, 32508, 32509,
                32510, 32588, 32589, 32590, 32668, 32669, 32670, 32748, 32749, 32750, 32828, 32908,
                32988, 33068, 33069, 33070, 33148, 33149, 33150, 33228, 33229, 33230, 33308, 33388,
                33468, 33548, 33629, 33630, 33631, 33709, 33710, 33711, 33789, 33790, 33791, 33868,
                33948, 34028, 34109, 34110, 34111, 34189, 34190, 34191, 34269, 34270, 34271, 34349,
                34350, 34351, 34429, 34430, 34432, 34509, 34510, 34512, 34589, 34590, 34592, 34669,
                34670, 34671, 34749, 34750, 34751, 34829, 34830, 34831, 34909, 34910, 34912, 34989,
                34990, 34992, 35069, 35070, 35072, 35149, 35150, 35152, 35229, 35231, 35232, 35233,
                35309, 35311, 35312, 35313, 35389, 35391, 35392, 35393, 35469, 35470, 35472, 35549,
                35550, 35552, 35629, 35630, 35632, 35709, 35711, 35712, 35713, 35789, 35791, 35792,
                35793, 35869, 35871, 35872, 35873, 35949, 35951, 35952, 35953, 36031, 36032, 36033,
                36111, 36112, 36113, 36191, 36192, 36193, 36269, 36271, 36272, 36273, 36349, 36351,
                36352, 36353, 36429, 36431, 36432, 36433, 36511, 36512, 36513, 36591, 36592, 36593,
                36671, 36672, 36673, 36751, 36752, 36753, 36832, 36912, 36992, 37071, 37072, 37073,
                37151, 37152, 37153, 37231, 37232, 37233, 37312, 37392, 37472, 37552, 37872, 37952,
                38032,
            ],
        ),
        (
            "specialanimation_winter_lute_play_start_south",
            &[51, 36],
            &[
                4355, 4435, 4444, 4514, 4515, 4594, 4595, 4604, 4674, 4675, 4754, 4755, 4764, 4765,
                4835, 4836, 4915, 4924, 5632, 5712, 5950, 5951, 6030, 6031, 6109, 6189, 6270, 6350,
                6428, 6429, 6430, 6508, 6509, 6510, 6588, 6668, 6749, 6750, 6751, 6829, 6830, 6831,
                6909, 6910, 6989, 6990, 7069, 7071, 7072, 7149, 7151, 7152, 7231, 7232, 7233, 7311,
                7312, 7313, 7392, 7472,
            ],
        ),
        (
            "specialanimation_winter_read_end_south",
            &[47, 33, 50],
            &[
                6675, 6684, 6755, 6764, 6835, 6844, 6914, 6915, 6924, 6994, 6995, 7004, 7074, 7075,
                7084, 7154, 7155, 7164, 7165, 7234, 7235, 7244, 7245, 7314, 7315, 7324, 7325, 7395,
                7396, 7404, 7475, 7476, 7484, 7555, 7556, 7564, 8592, 8672, 8752, 9070, 9071, 9150,
                9151, 9230, 9231, 9309, 9389, 9469, 9550, 9630, 9710, 9788, 9789, 9790, 9868, 9869,
                9870, 9948, 9949, 9950, 10028, 10108, 10188, 10269, 10270, 10271, 10349, 10350,
                10351, 10429, 10430, 10431, 10509, 10510, 10589, 10590, 10669, 10670, 10672, 10749,
                10829, 10831, 10909, 10911, 10912, 10913, 10991, 11071, 11151, 11152, 11153, 11392,
            ],
        ),
        (
            "specialanimation_winter_read_loop_south",
            &[41, 48, 41, 48],
            &[
                8675, 8844, 8994, 8995, 9075, 9084, 9164, 9165, 9235, 9244, 9314, 9315, 9317, 9325,
                9394, 9395, 9404, 9474, 9482, 9484, 9485, 9554, 9555, 9564, 9635, 9636, 9637, 9645,
                9714, 9715, 9724, 9725, 9794, 9802, 9803, 9804, 9874, 9875, 9884, 9885, 10035,
                10036, 10044, 10195, 10196, 10204, 11231, 11869, 11870, 12188, 12270, 12430, 12509,
                12589, 12749, 12827, 12828, 12829, 12910, 13070, 13147, 13228, 13229, 13230, 13388,
                13389, 13390, 13468, 13469, 13470, 13548, 13708, 13788, 13789, 13869, 13870, 14029,
                14030, 14108, 14110, 14111, 14189, 14190, 14349, 14350, 14430, 14431, 14432, 14509,
                14669, 14751, 14752,
            ],
        ),
        (
            "specialanimation_winter_read_sit_end_south",
            &[37, 33, 49],
            &[
                7395, 7404, 7475, 7484, 7555, 7564, 7634, 7635, 7644, 7714, 7715, 7724, 7794, 7795,
                7804, 7874, 7875, 7884, 7885, 7954, 7955, 7964, 7965, 8034, 8035, 8044, 8045, 8115,
                8116, 8124, 8195, 8196, 8204, 8275, 8276, 8284, 8912, 8992, 9311, 9312, 9390, 9391,
                9392, 9470, 9471, 9472, 9550, 9791, 9870, 9871, 9950, 9951, 10029, 10030, 10031,
                10109, 10110, 10111, 10189, 10190, 10191, 10269, 10349, 10429, 10510, 10511, 10590,
                10591, 10670, 10671, 10750, 10751, 10830, 10831, 10910, 10911, 10990, 11070, 11150,
                11392,
            ],
        ),
        (
            "specialanimation_winter_read_sit_loop_south",
            &[30, 39, 30, 39],
            &[
                9635, 9804, 9954, 9955, 10035, 10044, 10124, 10125, 10195, 10204, 10274, 10275,
                10277, 10285, 10354, 10355, 10364, 10434, 10442, 10444, 10445, 10514, 10515, 10524,
                10595, 10596, 10597, 10605, 10674, 10675, 10684, 10685, 10754, 10762, 10763, 10764,
                10834, 10835, 10844, 10845, 10995, 10996, 11004, 11155, 11156, 11164, 11552, 11952,
                12112, 12190, 12191, 12509, 12590, 12591, 12750, 12751, 12830, 13148, 13149, 13150,
                13230, 13390, 13468, 13549, 13550, 13709, 13710, 13789, 13790, 13869, 14029, 14109,
                14110, 14190, 14350, 14429, 14510, 14670, 14751, 14830, 14990,
            ],
        ),
        (
            "specialanimation_winter_read_sit_start_south",
            &[49, 37, 33],
            &[
                7235, 7244, 7474, 7475, 7484, 7555, 7564, 7635, 7644, 7714, 7715, 7724, 7725, 7794,
                7795, 7804, 7874, 7875, 7884, 7955, 7956, 7964, 8034, 8035, 8044, 8045, 8114, 8115,
                8124, 8125, 8275, 8276, 8284, 8355, 8356, 8364, 8992, 9072, 9151, 9152, 9390, 9470,
                9471, 9472, 9550, 9551, 9552, 9631, 9869, 9870, 9871, 9950, 9951, 10030, 10031,
                10109, 10189, 10190, 10191, 10269, 10270, 10271, 10350, 10351, 10429, 10509, 10590,
                10591, 10670, 10671, 10750, 10751, 10830, 10910, 10911, 10990, 10991, 11150, 11230,
                11392,
            ],
        ),
        (
            "specialanimation_winter_read_start_south",
            &[50, 37, 43],
            &[
                6515, 6524, 6754, 6755, 6764, 6835, 6844, 6915, 6924, 6994, 6995, 7004, 7005, 7074,
                7075, 7084, 7154, 7155, 7164, 7235, 7236, 7244, 7314, 7315, 7324, 7325, 7394, 7395,
                7404, 7405, 7555, 7556, 7564, 7635, 7636, 7644, 8432, 8752, 8832, 8910, 8911, 9149,
                9230, 9231, 9310, 9311, 9390, 9469, 9549, 9628, 9629, 9630, 9710, 9790, 9868, 9948,
                9949, 9950, 10028, 10029, 10030, 10109, 10110, 10111, 10188, 10268, 10349, 10350,
                10429, 10430, 10431, 10509, 10510, 10511, 10589, 10669, 10670, 10672, 10749, 10750,
                10831, 10909, 10911, 10912, 10913, 10989, 10991, 11151, 11152, 11153, 11231, 11392,
            ],
        ),
        (
            "specialanimation_winter_spell_cast_end_south",
            &[60],
            &[
                2275, 2284, 2354, 2355, 2364, 2434, 2435, 2444, 2445, 2515, 2516, 2524, 2912, 3070,
                3071, 3149, 3230, 3308, 3309, 3310, 3388, 3469, 3470, 3471, 3549, 3550, 3629, 3711,
            ],
        ),
        (
            "specialanimation_winter_spell_cast_loop_south",
            &[75, 74, 77, 78],
            &[
                8675, 8684, 8755, 8764, 8835, 8844, 8915, 8924, 8994, 8995, 9004, 9074, 9075, 9084,
                9154, 9155, 9164, 9234, 9235, 9244, 9314, 9315, 9324, 9325, 9394, 9395, 9404, 9405,
                9474, 9475, 9484, 9485, 9554, 9555, 9564, 9565, 9635, 9636, 9644, 9715, 9716, 9724,
                9795, 9796, 9804, 9875, 9876, 9884, 11311, 11551, 11949, 11950, 12268, 12508,
                12908, 12988, 13068, 13148, 13149, 13228, 13308, 13388, 13468, 13549, 13550, 13629,
                13630, 13709, 13710, 13789, 13790, 13869, 13870, 13872, 13949, 13950, 13952, 14029,
                14030, 14108, 14109, 14111, 14112, 14189, 14191, 14192, 14193, 14269, 14271, 14272,
                14273, 14349, 14351, 14429, 14431, 14432, 14433, 14511, 14512, 14513, 14591, 14592,
                14671, 14672, 14751, 14752, 14832, 14912, 14992,
            ],
        ),
        (
            "specialanimation_winter_spell_cast_start_south",
            &[60, 67],
            &[
                4435, 4444, 4515, 4524, 4594, 4595, 4604, 4674, 4675, 4684, 4754, 4755, 4764, 4765,
                4834, 4835, 4844, 4845, 4915, 4916, 4924, 4995, 4996, 5004, 5712, 5792, 6030, 6031,
                6110, 6111, 6189, 6269, 6430, 6508, 6509, 6588, 6589, 6590, 6668, 6748, 6909, 6910,
                6911, 6989, 7069, 7070, 7149, 7151, 7152, 7153, 7229, 7311, 7312, 7313, 7391, 7472,
            ],
        ),
        (
            "winter_action_east",
            &[45, 54, 52, 54, 52, 45, 48],
            &[
                15238, 15318, 15398, 15478, 15635, 15716, 15797, 15798, 15877, 15878, 15957, 15958,
                16037, 16038, 16116, 16194, 16195, 16275, 16276, 16357, 16358, 16360, 16368, 16437,
                16438, 16440, 16448, 16517, 16518, 16520, 16528, 16597, 16598, 16600, 16608, 16675,
                16676, 16754, 16755, 16757, 16765, 16835, 16836, 16838, 16846, 16918, 16919, 16920,
                16928, 16998, 16999, 17000, 17008, 17078, 17079, 17080, 17088, 17158, 17159, 17160,
                17168, 17235, 17236, 17238, 17246, 17315, 17316, 17317, 17325, 17396, 17397, 17398,
                17406, 17796, 17797, 17798, 17806, 19153, 19233, 19313, 19393, 19633, 20033, 20111,
                20271, 20272, 20351, 20352, 20431, 20432, 20511, 20512, 20751, 20752, 20830, 20910,
                20990, 21070, 21151, 21152, 21229, 21230, 21310, 21391, 21392, 21471, 21472, 21551,
                21552, 21631, 21632, 21710, 21788, 21871, 21950, 21951, 21952, 22030, 22031, 22032,
                22110, 22111, 22112, 22190, 22191, 22192, 22271, 22349, 22429, 22430, 22431, 22510,
                22590, 22670, 22750, 22829, 22830, 22831, 22907, 22908, 22909, 22989, 23071, 23072,
                23073, 23151, 23152, 23153, 23231, 23232, 23233, 23311, 23312, 23313, 23389, 23467,
                23550, 23551, 23631, 23632, 23711, 23712, 23791, 23792, 23871, 23872, 23950, 23951,
                24028, 24029, 24030, 24110, 24111, 24191, 24193, 24271, 24273, 24351, 24353, 24431,
                24433, 24510, 24511, 24588, 24589, 24670, 24753, 24754, 24833, 24834, 24913, 24914,
                24993, 24994, 25070, 25148, 25150, 25151, 25232, 25314, 25315, 25394, 25395, 25474,
                25475, 25554, 25555, 25632, 25710, 25711, 25712, 25713, 26271, 26272, 26273, 26833,
            ],
        ),
        (
            "winter_action_north",
            &[19, 24, 24, 24, 24, 22, 23],
            &[
                15234, 15245, 15314, 15325, 15394, 15405, 15474, 15485, 15714, 15725, 15794, 15805,
                15874, 15885, 15954, 15965, 16034, 16045, 16114, 16125, 16194, 16205, 16274, 16285,
                16674, 16685, 16754, 16765, 19647, 19728, 19808, 19888, 19968, 20047, 20127, 20288,
                20289, 20368, 20369, 20448, 20449, 20528, 20529, 20768, 20769, 21168, 21169, 21248,
                21249, 21330, 21408, 21409, 21488, 21489, 21568, 21569, 21648, 21649, 21730, 21810,
                21889, 21968, 21969, 21970, 22048, 22049, 22050, 22128, 22129, 22130, 22208, 22209,
                22210, 22289, 22369, 22449, 22450, 22451, 22530, 22610, 22690, 22770, 22849, 22850,
                22851, 22929, 22930, 22931, 23011, 23088, 23089, 23168, 23169, 23248, 23249, 23328,
                23329, 23411, 23491, 23569, 23570, 23647, 23648, 23649, 23650, 23727, 23728, 23729,
                23730, 23807, 23808, 23809, 23810, 23887, 23888, 23889, 23890, 23969, 23970, 24049,
                24050, 24128, 24129, 24130, 24131, 24206, 24209, 24286, 24289, 24366, 24369, 24446,
                24449, 24528, 24529, 24530, 24531, 24608, 24609, 24610, 24611, 24686, 24687, 24690,
                24763, 24764, 24765, 24766, 24767, 24769, 24843, 24844, 24845, 24846, 24847, 24849,
                24923, 24924, 24925, 24926, 24927, 24929, 25003, 25004, 25005, 25006, 25007, 25009,
                25086, 25087, 25090, 25166, 25167, 25170, 25243, 25244, 25245, 25246, 25247, 25248,
                25320, 25321, 25322, 25323, 25324, 25325, 25326, 25327, 25328, 25400, 25401, 25402,
                25403, 25404, 25405, 25406, 25407, 25408, 25480, 25481, 25482, 25483, 25484, 25485,
                25486, 25487, 25488, 25560, 25561, 25562, 25563, 25564, 25565, 25566, 25567, 25568,
                25643, 25644, 25645, 25646, 25647, 25648, 25723, 25724, 25725, 25726, 25727, 25728,
                25800, 25801, 25802, 25803, 25804, 25805, 25806, 25807, 25881, 25882, 25883, 25884,
                25885, 25961, 25962, 25963, 25964, 25965, 26041, 26042, 26043, 26044, 26045, 26121,
                26122, 26123, 26124, 26125, 26200, 26201, 26202, 26203, 26204, 26205, 26206, 26207,
                26280, 26281, 26282, 26283, 26284, 26285, 26286, 26287, 26361, 26362, 26363, 26364,
                26365, 26761, 26762, 26763, 26764, 26765, 26841, 26842, 26843, 26844, 26845,
            ],
        ),
        (
            "winter_action_south",
            &[49, 53, 53, 53, 53, 49, 56],
            &[
                15235, 15244, 15315, 15324, 15395, 15404, 15475, 15484, 15635, 15644, 15715, 15724,
                15794, 15795, 15804, 15874, 15875, 15884, 15954, 15955, 15964, 16034, 16035, 16044,
                16115, 16124, 16194, 16195, 16204, 16274, 16275, 16284, 16354, 16355, 16364, 16365,
                16434, 16435, 16444, 16445, 16514, 16515, 16524, 16525, 16594, 16595, 16604, 16605,
                16674, 16675, 16684, 16754, 16755, 16764, 16765, 16834, 16835, 16844, 16845, 16915,
                16916, 16924, 16995, 16996, 17004, 17075, 17076, 17084, 17155, 17156, 17164, 17234,
                17235, 17244, 17245, 17315, 17316, 17324, 17395, 17396, 17404, 17795, 17796, 17804,
                19711, 19791, 19871, 19951, 20112, 20192, 20592, 20829, 20830, 20909, 20910, 20989,
                20990, 21069, 21070, 21230, 21231, 21310, 21311, 21388, 21468, 21548, 21628, 21710,
                21711, 21789, 21869, 21949, 21950, 22029, 22030, 22109, 22110, 22189, 22190, 22269,
                22350, 22430, 22508, 22509, 22510, 22588, 22589, 22590, 22668, 22669, 22670, 22748,
                22749, 22750, 22830, 22908, 22909, 22910, 22988, 22989, 22990, 23068, 23148, 23228,
                23308, 23388, 23389, 23390, 23468, 23548, 23629, 23630, 23631, 23709, 23710, 23711,
                23789, 23790, 23791, 23869, 23870, 23871, 23948, 24029, 24030, 24031, 24109, 24110,
                24111, 24189, 24190, 24269, 24270, 24349, 24350, 24429, 24430, 24509, 24510, 24511,
                24589, 24590, 24669, 24670, 24749, 24751, 24752, 24829, 24831, 24832, 24909, 24911,
                24912, 24989, 24991, 24992, 25069, 25070, 25149, 25229, 25231, 25311, 25312, 25313,
                25391, 25392, 25393, 25471, 25472, 25473, 25551, 25552, 25553, 25629, 25631, 25711,
                25791, 25792, 25872, 25873, 25952, 25953, 26032, 26033, 26112, 26113, 26191, 26192,
                26352, 26752,
            ],
        ),
        (
            "winter_blink_east",
            &[52, 56, 52],
            &[
                6515, 6595, 6675, 6754, 6755, 6834, 6835, 6914, 6915, 6994, 6995, 6997, 7005, 7074,
                7075, 7077, 7085, 7154, 7155, 7157, 7165, 7235, 7236, 7237, 7245, 7315, 7316, 7317,
                7325, 7395, 7396, 7397, 7405, 8431, 8511, 8591, 8909, 8910, 8989, 8990, 9069, 9070,
                9148, 9228, 9308, 9389, 9469, 9549, 9627, 9628, 9629, 9707, 9708, 9709, 9787, 9788,
                9789, 9867, 9947, 10027, 10108, 10109, 10110, 10188, 10189, 10190, 10268, 10269,
                10270, 10348, 10349, 10428, 10429, 10508, 10509, 10588, 10590, 10591, 10668, 10670,
                10671, 10748, 10750, 10751, 10830, 10831, 10832, 10833, 10910, 10911, 10912, 10913,
                10990, 10991, 10992, 10993, 11071, 11072, 11073, 11151, 11152, 11153, 11231, 11232,
                11233, 11313, 11393, 11473,
            ],
        ),
        (
            "winter_blink_south",
            &[60, 64, 60],
            &[
                6515, 6524, 6595, 6604, 6675, 6684, 6754, 6755, 6764, 6834, 6835, 6844, 6914, 6915,
                6924, 6994, 6995, 7004, 7005, 7074, 7075, 7084, 7085, 7154, 7155, 7164, 7165, 7235,
                7236, 7244, 7315, 7316, 7324, 7395, 7396, 7404, 8432, 8512, 8592, 8910, 8911, 8990,
                8991, 9070, 9071, 9149, 9229, 9309, 9390, 9470, 9550, 9628, 9629, 9630, 9708, 9709,
                9710, 9788, 9789, 9790, 9868, 9948, 10028, 10109, 10110, 10111, 10189, 10190,
                10191, 10269, 10270, 10271, 10349, 10350, 10429, 10430, 10509, 10510, 10589, 10669,
                10749, 10831, 10911, 10991,
            ],
        ),
        (
            "winter_drink_east",
            &[43, 52, 43],
            &[
                7235, 7312, 7395, 7474, 7475, 7551, 7552, 7563, 7634, 7635, 7714, 7715, 7717, 7725,
                7792, 7794, 7795, 7803, 7874, 7875, 7877, 7885, 7955, 7956, 7957, 7965, 8033, 8034,
                8035, 8115, 8116, 8117, 8125, 9152, 9153, 9230, 9312, 9313, 9391, 9469, 9551, 9710,
                9870, 9871, 9949, 9950, 10030, 10031, 10110, 10189, 10270,
            ],
        ),
        (
            "winter_drink_north",
            &[2, 2, 2],
            &[
                7474, 7485, 7634, 7645, 7714, 7725, 7794, 7805, 7874, 7885, 8034, 8045, 8686, 8846,
                9007, 9167, 9168, 9327, 9328, 9409, 9488, 9489, 9569, 9648, 9808, 9888, 9889, 9890,
                9968, 9969, 10048, 10049, 10050, 10130, 10208, 10209, 10210, 10290, 10368, 10369,
                10450, 10528, 10529, 10607, 10608, 10609, 10610, 10688, 10689, 10767, 10768, 10769,
                10770, 10845, 10846, 10849, 10927, 10928, 10929, 10930, 11005, 11006, 11009, 11082,
                11083, 11084, 11085, 11086, 11087, 11164, 11165, 11166, 11169, 11242, 11243, 11244,
                11245, 11246, 11247, 11320, 11321, 11322, 11323, 11324, 11325, 11326, 11400, 11401,
                11402, 11403, 11404, 11405, 11406, 11407, 11480, 11481, 11482, 11483, 11484, 11485,
                11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646, 11647,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "winter_drink_south",
            &[55, 61, 55],
            &[
                7235, 7244, 7395, 7404, 7474, 7475, 7484, 7555, 7564, 7634, 7635, 7644, 7714, 7715,
                7724, 7725, 7794, 7795, 7804, 7874, 7875, 7884, 7885, 7955, 7956, 7964, 8034, 8035,
                8044, 8045, 8115, 8116, 8124, 8275, 8276, 8284, 8992, 9151, 9152, 9311, 9312, 9390,
                9470, 9471, 9472, 9550, 9631, 9709, 9791, 9869, 9870, 9871, 9950, 9951, 10029,
                10030, 10031, 10109, 10189, 10190, 10191, 10269, 10350, 10351, 10429, 10510, 10511,
                10590, 10591, 10670, 10671, 10750, 10751, 10830, 10910, 10911, 10990, 11072, 11150,
                11152, 11232, 11392, 11393,
            ],
        ),
        (
            "winter_eat_east",
            &[41, 43, 39, 45, 41],
            &[
                11794, 12035, 12116, 12193, 12194, 12205, 12355, 12434, 12435, 12515, 12516, 12594,
                12596, 12597, 12605, 12675, 12754, 12755, 12834, 12835, 12837, 12845, 12915, 12916,
                12918, 12926, 12995, 12996, 12997, 13074, 13075, 13154, 13155, 13157, 13165, 13235,
                13236, 13237, 13245, 13316, 13317, 13318, 13326, 13474, 13475, 13477, 13485, 13555,
                13556, 13557, 13565, 13875, 13876, 13877, 13885, 14513, 14514, 14592, 14593, 15073,
                15232, 15233, 15312, 15313, 15391, 15392, 15552, 15553, 15631, 15711, 15790, 15871,
                15872, 15873, 15951, 16112, 16191, 16270, 16430, 16431, 16510, 16511, 16512, 16590,
                16591, 16671, 16750, 16751, 16830, 16910, 16990, 17070, 17071, 17150, 17311, 17470,
                17711, 18111,
            ],
        ),
        (
            "winter_eat_north",
            &[2, 2, 2],
            &[
                7474, 7485, 7634, 7645, 7714, 7725, 7794, 7805, 7874, 7885, 8034, 8045, 8686, 8846,
                9007, 9167, 9168, 9327, 9328, 9409, 9488, 9489, 9569, 9648, 9808, 9888, 9889, 9890,
                9968, 9969, 10048, 10049, 10050, 10130, 10208, 10209, 10210, 10290, 10368, 10369,
                10450, 10528, 10529, 10607, 10608, 10609, 10610, 10688, 10689, 10767, 10768, 10769,
                10770, 10845, 10846, 10849, 10927, 10928, 10929, 10930, 11005, 11006, 11009, 11082,
                11083, 11084, 11085, 11086, 11087, 11164, 11165, 11166, 11169, 11242, 11243, 11244,
                11245, 11246, 11247, 11320, 11321, 11322, 11323, 11324, 11325, 11326, 11400, 11401,
                11402, 11403, 11404, 11405, 11406, 11407, 11480, 11481, 11482, 11483, 11484, 11485,
                11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646, 11647,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "winter_eat_south",
            &[55, 53, 54, 65, 53],
            &[
                11795, 11804, 12035, 12044, 12194, 12195, 12204, 12355, 12364, 12434, 12435, 12444,
                12515, 12524, 12594, 12595, 12604, 12605, 12675, 12684, 12754, 12755, 12764, 12834,
                12835, 12844, 12845, 12914, 12915, 12924, 12995, 12996, 13004, 13074, 13075, 13084,
                13154, 13155, 13164, 13165, 13235, 13236, 13244, 13314, 13315, 13324, 13325, 13474,
                13475, 13484, 13485, 13555, 13556, 13564, 13715, 13716, 13724, 13875, 13876, 13884,
                14591, 14592, 14912, 15072, 15231, 15232, 15390, 15391, 15392, 15551, 15552, 15630,
                15710, 15711, 15712, 15789, 15870, 15871, 15872, 15950, 16031, 16190, 16191, 16351,
                16429, 16430, 16431, 16510, 16511, 16589, 16590, 16591, 16670, 16671, 16749, 16750,
                16751, 16829, 16909, 16910, 16911, 16989, 17069, 17070, 17071, 17149, 17230, 17231,
                17309, 17390, 17391, 17469, 17550, 17551, 17630, 17631, 17710, 17711, 17790, 17791,
                17870, 17871, 17950, 17951, 18030, 18110, 18111, 18190, 18270, 18271, 18350, 18432,
                18510, 18592, 18670, 18912, 19072,
            ],
        ),
        (
            "winter_idle_east",
            &[48],
            &[
                2195, 2274, 2275, 2354, 2355, 2357, 2365, 2435, 2436, 2437, 2445, 2831, 2989, 2990,
                3068, 3149, 3227, 3228, 3229, 3307, 3388, 3389, 3390, 3468, 3469, 3548, 3550, 3551,
                3630, 3631, 3632, 3633, 3711, 3712, 3713, 3793,
            ],
        ),
        (
            "winter_idle_north",
            &[23],
            &[
                2274, 2285, 2354, 2365, 2847, 3008, 3009, 3090, 3169, 3249, 3250, 3251, 3331, 3409,
                3410, 3488, 3489, 3490, 3491, 3566, 3567, 3570, 3643, 3644, 3645, 3646, 3647, 3648,
                3720, 3721, 3722, 3723, 3724, 3725, 3726, 3727, 3801, 3802, 3803, 3804, 3805,
            ],
        ),
        (
            "winter_idle_south",
            &[56],
            &[
                2195, 2204, 2274, 2275, 2284, 2354, 2355, 2364, 2365, 2435, 2436, 2444, 2832, 2990,
                2991, 3069, 3150, 3228, 3229, 3230, 3308, 3389, 3390, 3391, 3469, 3470, 3549, 3631,
            ],
        ),
        (
            "winter_kiss_east",
            &[44, 48, 58, 52],
            &[
                8838, 8994, 9076, 9157, 9158, 9236, 9313, 9314, 9395, 9396, 9477, 9478, 9480, 9488,
                9555, 9556, 9633, 9634, 9636, 9644, 9715, 9716, 9718, 9726, 9798, 9799, 9800, 9808,
                9875, 9876, 9878, 9886, 9954, 9955, 9956, 9964, 10036, 10037, 10038, 10046, 10196,
                10197, 10198, 10206, 11070, 11550, 11630, 11710, 11711, 11790, 12029, 12188, 12189,
                12269, 12270, 12350, 12351, 12429, 12430, 12507, 12588, 12669, 12670, 12671, 12748,
                12828, 12829, 12909, 12910, 12989, 13069, 13070, 13147, 13148, 13149, 13228, 13229,
                13230, 13310, 13311, 13388, 13389, 13390, 13467, 13548, 13630, 13631, 13708, 13788,
                13789, 13869, 13870, 13950, 13952, 14029, 14030, 14107, 14108, 14109, 14189, 14190,
                14272, 14273, 14274, 14349, 14350, 14428, 14430, 14509, 14511, 14592, 14593, 14669,
                14671, 14750, 14751, 14752, 14753, 14831, 14832, 14833, 14834, 14991, 14992, 14993,
                14994, 15070, 15071, 15072, 15073, 15151, 15152, 15153, 15311, 15312, 15313, 15392,
            ],
        ),
        (
            "winter_sit_east",
            &[42],
            &[
                2435, 2514, 2515, 2594, 2595, 2597, 2605, 2675, 2676, 2677, 2685, 3072, 3073, 3151,
                3310, 3311, 3390,
            ],
        ),
        (
            "winter_sit_north",
            &[2],
            &[
                2514, 2525, 2594, 2605, 2926, 3087, 3088, 3169, 3248, 3328, 3329, 3330, 3410, 3488,
                3489, 3567, 3568, 3569, 3570, 3645, 3646, 3649, 3722, 3723, 3724, 3725, 3726, 3727,
                3800, 3801, 3802, 3803, 3804, 3805, 3806, 3881, 3882, 3883, 3884,
            ],
        ),
        (
            "winter_sit_south",
            &[53],
            &[
                2435, 2444, 2514, 2515, 2524, 2594, 2595, 2604, 2605, 2675, 2676, 2684, 3071, 3072,
                3150, 3231, 3309, 3310, 3311, 3389, 3470, 3471, 3550, 3551, 3630,
            ],
        ),
        (
            "winter_sleep_east",
            &[48],
            &[
                2275, 2354, 2355, 2434, 2435, 2437, 2445, 2515, 2516, 2517, 2525,
            ],
        ),
        (
            "winter_walk_east",
            &[48, 49, 48, 49],
            &[
                8675, 8835, 8994, 8995, 9075, 9154, 9155, 9235, 9314, 9315, 9317, 9325, 9394, 9395,
                9474, 9475, 9477, 9485, 9554, 9555, 9635, 9636, 9637, 9645, 9714, 9715, 9717, 9725,
                9795, 9796, 9797, 9805, 9874, 9875, 9877, 9885, 10035, 10036, 10037, 10045, 10195,
                10196, 10197, 10205, 11231, 11391, 11632, 11869, 11870, 12029, 12030, 12109, 12188,
                12270, 12271, 12348, 12428, 12429, 12509, 12589, 12669, 12827, 12828, 12829, 12910,
                12987, 12988, 12989, 13068, 13069, 13147, 13228, 13229, 13230, 13307, 13387, 13388,
                13389, 13468, 13469, 13470, 13548, 13628, 13629, 13630, 13707, 13788, 13789, 13869,
                13870, 13948, 13949, 14028, 14029, 14108, 14110, 14111, 14188, 14189, 14268, 14270,
                14271, 14347, 14348, 14349, 14430, 14431, 14432, 14433, 14510, 14511, 14590, 14591,
                14592, 14593, 14668, 14670, 14671, 14751, 14752, 14753, 14830, 14831, 14832, 14833,
                14834, 14911, 14912, 14913, 14988, 14990, 14991, 14992, 14993, 15073, 15151, 15152,
                15153, 15154, 15233, 15309, 15310, 15311, 15312, 15313, 15473, 15631, 15632,
            ],
        ),
        (
            "winter_walk_north",
            &[23, 18, 23, 17],
            &[
                8994, 9005, 9154, 9165, 9314, 9325, 9394, 9405, 9474, 9485, 9554, 9565, 9714, 9725,
                9874, 9885, 11247, 11407, 11646, 11888, 11889, 12048, 12049, 12128, 12210, 12287,
                12288, 12370, 12448, 12449, 12529, 12609, 12689, 12849, 12850, 12851, 12928, 13009,
                13010, 13011, 13088, 13089, 13171, 13248, 13249, 13250, 13331, 13408, 13409, 13410,
                13489, 13490, 13570, 13649, 13650, 13730, 13808, 13809, 13810, 13811, 13888, 13889,
                13968, 13969, 13970, 13971, 14048, 14049, 14126, 14127, 14130, 14207, 14209, 14210,
                14286, 14287, 14290, 14367, 14368, 14369, 14370, 14443, 14444, 14445, 14446, 14447,
                14448, 14526, 14527, 14528, 14603, 14604, 14605, 14606, 14607, 14608, 14686, 14689,
                14760, 14761, 14762, 14763, 14764, 14765, 14766, 14767, 14843, 14844, 14845, 14846,
                14847, 14848, 14920, 14921, 14922, 14923, 14924, 14925, 14926, 14927, 15003, 15004,
                15005, 15006, 15007, 15009, 15081, 15082, 15083, 15084, 15085, 15160, 15161, 15162,
                15163, 15164, 15165, 15166, 15167, 15241, 15242, 15243, 15244, 15245, 15320, 15321,
                15322, 15323, 15324, 15325, 15326, 15327, 15328, 15481, 15482, 15483, 15484, 15485,
                15641, 15642, 15643, 15644, 15645,
            ],
        ),
        (
            "winter_walk_south",
            &[56, 52, 56, 49],
            &[
                8675, 8684, 8835, 8844, 8994, 8995, 9004, 9075, 9084, 9154, 9155, 9164, 9235, 9244,
                9314, 9315, 9324, 9325, 9394, 9395, 9404, 9474, 9475, 9484, 9485, 9554, 9555, 9564,
                9635, 9636, 9644, 9714, 9715, 9724, 9725, 9795, 9796, 9804, 9874, 9875, 9884, 9885,
                10035, 10036, 10044, 10195, 10196, 10204, 11232, 11392, 11870, 11871, 12030, 12031,
                12111, 12189, 12271, 12272, 12349, 12430, 12431, 12510, 12590, 12670, 12828, 12829,
                12830, 12911, 12988, 12989, 12990, 13070, 13148, 13229, 13230, 13231, 13308, 13389,
                13390, 13469, 13470, 13471, 13549, 13629, 13630, 13631, 13709, 13789, 13790, 13870,
                13871, 13949, 13950, 14030, 14031, 14109, 14189, 14190, 14269, 14349, 14350, 14431,
                14511, 14591, 14671, 14672, 14831, 14832, 14991, 14992, 14993, 15152, 15153, 15311,
                15312,
            ],
        ),
    ];
    fn rgb(n: u32) -> [u8; 4] {
        [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255]
    }
    let skin = [0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48, 0xAF8D81].map(rgb);
    let choices = [
        ("blue", [0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83, 0x7F9FBD]),
        (
            "npc_adeline",
            [0xE3A17B, 0xD48363, 0xC47054, 0x9F5544, 0xD48363],
        ),
        (
            "npc_hayden",
            [0xE8B271, 0xCA9052, 0xB27146, 0x6E4922, 0xCA9052],
        ),
        (
            "npc_ryis",
            [0xB06C57, 0x814A3A, 0x63342A, 0x491F1B, 0x814A3A],
        ),
    ];
    let mut selected = 0;
    for (name, counts, protected) in cases {
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
        assert_eq!(properties["offset"]["horizontal"].as_str(), Some("Middle"));
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
            if skin.contains(&p.0) && !protected.contains(&(y * before.width() + x)) {
                assert!(
                    (24..=58).contains(&(x % 80)) && (19..=62).contains(&y),
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
            let mut landmarks: Vec<(u32, u32, u32, u32, Option<usize>)> = Vec::new();
            if name.ends_with("idle_south") {
                landmarks.extend([
                    (0, 39, 33, 0xC1AFA5, Some(0)),
                    (0, 40, 33, 0x8E746D, Some(2)),
                    (0, 40, 32, 0x624A48, Some(3)),
                    (0, 38, 35, 0xA69084, Some(1)),
                    (0, 34, 33, 0xC1AFA5, Some(0)),
                    (0, 44, 33, 0xC1AFA5, Some(0)),
                ]);
                landmarks.extend([
                    (0, 35, 25, 0xF5F5F5, None),
                    (0, 35, 28, 0x8E746D, None),
                    (0, 36, 30, 0xC1AFA5, None),
                    (0, 28, 42, 0xF5F5F5, None),
                    (0, 30, 43, 0xC1AFA5, None),
                    (0, 37, 33, 0xC2B9BE, None),
                    (0, 37, 34, 0xECF0E9, None),
                    (0, 35, 35, 0xEABF67, None),
                    (0, 40, 28, 0x42A2AD, None),
                ]);
                if *season == "Autumn" {
                    landmarks.extend([
                        (0, 45, 43, 0xC1AFA5, Some(0)),
                        (0, 46, 44, 0x624A48, Some(3)),
                        (0, 38, 53, 0xF5F5F5, None),
                    ]);
                }
                if *season == "Beach" {
                    landmarks.extend([
                        (0, 39, 40, 0xC1AFA5, Some(0)),
                        (0, 38, 39, 0xF5F5F5, None),
                        (0, 35, 41, 0xE7D5D5, None),
                    ]);
                }
            }
            if name == "beach_walk_east" {
                landmarks.push((1, 40, 51, 0xAF8D81, Some(4)));
            }
            if name == "beach_action_south" {
                landmarks.push((0, 35, 42, 0xD7BBBB, None));
            }
            if name == "autumn_eat_east" {
                landmarks.extend([(2, 42, 36, 0x9E2626, None)]);
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
                    shade.map_or(rgb(color), |i| target[i]),
                    "material landmark {name} {choice} {x},{y}"
                );
            }
            for (x, y, p) in before.enumerate_pixels() {
                let expected = skin
                    .iter()
                    .position(|c| c == &p.0)
                    .filter(|_| !protected.contains(&(y * before.width() + x)))
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
    assert_eq!(selected, 31355);
    let portraits = read_json(root.join("palettes/profiles/caldarus-portraits.json"));
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
        .arg(root.join("palettes/sets/caldarus-portraits-trial.json"))
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
