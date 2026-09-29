use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{fs, process::Command};
#[test]
fn reina_juniper_beach_finish_preserve_native_metadata() {
    for (character, hotkey, expected_frames, cases) in [
        (
            "reina",
            "HOME",
            3,
            &[
                ("beach_sit_east", 1, "", "\"Middle\""),
                ("beach_sit_north", 1, "", "\"Middle\""),
                ("beach_sit_south", 1, "", "\"Middle\""),
            ][..],
        ),
        (
            "juniper",
            "PAGE_DOWN",
            16,
            &[
                ("specialanimation_beach_swim_idle_east", 4, "0.15", "40.0"),
                ("specialanimation_beach_swim_idle_north", 4, "0.15", "40.0"),
                ("specialanimation_beach_swim_idle_south", 4, "0.15", "40.0"),
                (
                    "specialanimation_beach_swim_spell_cast_south",
                    4,
                    "0.15",
                    "40.0",
                ),
            ][..],
        ),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("original");
        let modified = temp.path().join("modified");
        fs::create_dir(&original).unwrap();
        fs::create_dir(&modified).unwrap();
        for (name, frames, duration, horizontal) in cases {
            let stem = format!("spr_npc_{character}_{name}");
            let timing = if *frames == 1 {
                String::new()
            } else {
                format!("frame_len={frames}\nduration={duration}\n")
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
        assert_eq!(frames, expected_frames);
    }
}
