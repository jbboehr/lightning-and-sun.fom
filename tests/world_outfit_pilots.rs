use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{fs, process::Command};

#[test]
fn outfit_pilots_package_all_directions_with_native_timing_and_character_controls() {
    for (character, hotkey, prefix, idle, walk, duration) in [
        (
            "hayden",
            "F8",
            "specialanimation_spring",
            "ride_idle_1",
            "ride_walk",
            "0.125",
        ),
        ("ryis", "F10", "summer", "idle", "walk", "0.15"),
        ("celine", "INSERT", "spring_garden", "idle", "walk", "0.15"),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("original");
        let modified = temp.path().join("modified");
        fs::create_dir(&original).unwrap();
        fs::create_dir(&modified).unwrap();
        for (cycle, frames) in [(idle, 1), (walk, 4)] {
            for direction in ["north", "south", "east"] {
                let stem = format!("spr_npc_{character}_{prefix}_{cycle}_{direction}");
                let timing = if frames == 1 {
                    String::new()
                } else {
                    format!("frame_len=4\nduration={duration}\n")
                };
                let meta = format!(
                    "[meta_properties]\nid='synthetic'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[80,80]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal='Middle'\nvertical=54.0\n"
                );
                for (directory, color) in [
                    (&original, [10, 20, 30, 255]),
                    (&modified, [40, 50, 60, 255]),
                ] {
                    RgbaImage::from_pixel(frames * 80, 80, Rgba(color))
                        .save(directory.join(format!("{stem}.png")))
                        .unwrap();
                    fs::write(directory.join(format!("{stem}.meta.toml")), &meta).unwrap();
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
            "{character}: {}",
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
        assert_eq!(table[0][0], character);
        assert_eq!(table[0][2], hotkey);
        assert_eq!(table[0][4].as_array().unwrap().len(), 6);
        let mut frames = 0;
        for row in table[0][4].as_array().unwrap() {
            let stem = row[0].as_str().unwrap();
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
        assert_eq!(frames, 15);
    }
}
