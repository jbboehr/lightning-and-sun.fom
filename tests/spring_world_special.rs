use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{fs, process::Command};

#[test]
fn spring_specials_package_every_strip_with_native_origins_timing_and_controls() {
    for (character, hotkey, expected_frames, cases) in [
        (
            "hayden",
            "F8",
            42,
            vec![
                ("hammer_east", 6, "[0.1,0.2,0.1,0.1,0.1,0.1]"),
                (
                    "harvest_east",
                    10,
                    "[0.4,0.15,0.2,0.2,0.2,0.2,0.2,0.2,0.15,1.4]",
                ),
                ("pet_east", 7, "0.15"),
                ("sigh_south", 4, "[0.6,0.1,1.2,0.1]"),
                ("till_east", 5, "[0.125,0.125,0.125,0.125,1.2]"),
                ("water_east", 4, "[0.15,1.0,0.15,2.5]"),
                ("wipebrow_south", 6, "[0.15,0.15,0.45,0.1,0.075,1.0]"),
            ],
        ),
        (
            "ryis",
            "F10",
            27,
            vec![
                ("hammer_east", 7, "0.1"),
                ("saw_east", 4, "[0.325,0.125,0.3,0.125]"),
                ("siteyesclosed_east", 1, ""),
                ("siteyesclosed_south", 1, ""),
                ("wipebrow_south", 6, "[0.15,0.15,0.45,0.1,0.075,1.0]"),
                ("write_start_south", 2, "0.15"),
                ("write_loop_south", 4, "0.15"),
                ("write_end_south", 2, "0.15"),
            ],
        ),
        (
            "celine",
            "INSERT",
            37,
            vec![
                ("book_stand_start_south", 3, "0.1"),
                ("book_stand_loop_south", 4, "[3.0,0.1,3.0,0.1]"),
                ("book_stand_end_south", 3, "0.1"),
                ("herb_south", 6, "[0.25,0.1,0.1,0.1,0.1,0.1]"),
                ("sweep_start_south", 1, ""),
                (
                    "sweep_loop_south",
                    15,
                    "[0.1,0.1,0.1,0.1,0.1,0.2,0.1,0.1,0.1,0.1,0.1,0.4,0.1,0.1,0.8]",
                ),
                ("sweep_end_south", 1, ""),
                ("water_east", 4, "[0.15,1.0,0.15,2.5]"),
            ],
        ),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("original");
        let modified = temp.path().join("modified");
        fs::create_dir(&original).unwrap();
        fs::create_dir(&modified).unwrap();
        for (name, frames, duration) in &cases {
            let stem = format!("spr_npc_{character}_specialanimation_spring_{name}");
            let timing = if duration.is_empty() {
                String::new()
            } else {
                format!("frame_len={frames}\nduration={duration}\n")
            };
            // These Ryis sources use a numeric origin; preserve its TOML type.
            let horizontal = if character == "ryis" {
                "40.0"
            } else {
                "'Middle'"
            };
            let meta = format!(
                "[meta_properties]\nid='synthetic'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[80,80]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal={horizontal}\nvertical=54.0\n"
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
        assert_eq!(table[0][4].as_array().unwrap().len(), cases.len());
        let mut packaged_frames = 0;
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
            packaged_frames += before["asset_properties"]
                .get("frame_len")
                .and_then(toml::Value::as_integer)
                .unwrap_or(1);
            assert_eq!(
                fs::read(modified.join(format!("{stem}.png"))).unwrap(),
                fs::read(output.join(format!("animations/LightningAndSun/{target}.png"))).unwrap()
            );
        }
        assert_eq!(packaged_frames, expected_frames);
    }
}
