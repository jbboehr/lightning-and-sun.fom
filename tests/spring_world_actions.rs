use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{fs, process::Command};

#[test]
fn spring_actions_keep_each_characters_controls_and_directional_timing() {
    for (character, hotkey) in [("hayden", "F8"), ("ryis", "F10"), ("celine", "INSERT")] {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("original");
        let modified = temp.path().join("modified");
        fs::create_dir(&original).unwrap();
        fs::create_dir(&modified).unwrap();
        for action in ["blink", "sit", "eat", "drink", "action", "sleep", "kiss"] {
            for direction in ["north", "south", "east"] {
                if (action == "blink" && direction == "north")
                    || (matches!(action, "sleep" | "kiss") && direction != "east")
                {
                    continue;
                }
                let (frames, timing) = match (action, direction) {
                    ("blink", _) => (3, "frame_len=3\nduration=[0.075,0.125,0.075]\n"),
                    ("sit" | "sleep", _) => (1, ""),
                    ("action", _) => (
                        7,
                        "frame_len=7\nduration=[0.1,0.25,0.25,0.25,0.25,0.1,0.4]\n",
                    ),
                    ("kiss", _) => (4, "frame_len=4\nduration=[0.15,0.15,0.8,0.15]\n"),
                    ("eat", "east" | "south") => {
                        (5, "frame_len=5\nduration=[0.125,0.15,0.175,0.125,0.6]\n")
                    }
                    _ => (3, "frame_len=3\nduration=1.0\n"),
                };
                let name = format!("spr_npc_{character}_spring_{action}_{direction}");
                let meta = format!(
                    "[meta_properties]\nid='synthetic'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[2,3]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal='Middle'\nvertical=54.0\n"
                );
                for (path, color) in [
                    (&original, [10, 20, 30, 255]),
                    (&modified, [40, 50, 60, 255]),
                ] {
                    RgbaImage::from_pixel(frames * 2, 3, Rgba(color))
                        .save(path.join(format!("{name}.png")))
                        .unwrap();
                    fs::write(path.join(format!("{name}.meta.toml")), &meta).unwrap();
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
        assert_eq!(table[0][4].as_array().unwrap().len(), 16);
        let mut frames = 0;
        for row in table[0][4].as_array().unwrap() {
            let name = row[0].as_str().unwrap();
            let target = row[1].as_str().unwrap();
            let before: toml::Value = toml::from_str(
                &fs::read_to_string(original.join(format!("{name}.meta.toml"))).unwrap(),
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
                "{name}"
            );
            frames += before["asset_properties"]
                .get("frame_len")
                .and_then(toml::Value::as_integer)
                .unwrap_or(1);
            assert_eq!(
                fs::read(modified.join(format!("{name}.png"))).unwrap(),
                fs::read(output.join(format!("animations/LightningAndSun/{target}.png"))).unwrap()
            );
        }
        assert_eq!(frames, 57);
    }
}
