use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

fn rgba(text: &str) -> [u8; 4] {
    let text = text.trim_start_matches('#');
    let mut result = [255; 4];
    for (index, channel) in result.iter_mut().enumerate().take(text.len() / 2) {
        *channel = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).unwrap();
    }
    result
}

#[test]
#[ignore = "requires the retained Reina and Juniper portrait corpora"]
fn before_facial_correction_portrait_mouths_preserve_lips_and_recolor_adjacent_skin() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut protected_pixels = 0;
    let mut landmarks = 0;
    for (character, expected_strips) in [("reina", 103), ("juniper", 132)] {
        let selected_set =
            std::env::var_os(format!("FOM_{}_MOUTH_SET", character.to_ascii_uppercase()))
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| {
                    root.join(format!("palettes/sets/{character}-native-face-before.json"))
                });
        let mut set: Value = serde_json::from_slice(&fs::read(&selected_set).unwrap()).unwrap();
        let mut profile: Value = serde_json::from_slice(
            &fs::read(
                selected_set
                    .parent()
                    .unwrap()
                    .join(set["profile"].as_str().unwrap()),
            )
            .unwrap(),
        )
        .unwrap();
        profile["regions"]
            .as_array_mut()
            .unwrap()
            .retain(|row| row["asset"].as_str().unwrap().contains("/Portraits/"));
        assert_eq!(
            profile["regions"].as_array().unwrap().len(),
            expected_strips
        );
        let temp = tempfile::tempdir().unwrap();
        let profile_path = temp.path().join("profile.json");
        fs::write(&profile_path, serde_json::to_vec(&profile).unwrap()).unwrap();
        set["profile"] = json!(profile_path);
        let set_path = temp.path().join("set.json");
        fs::write(&set_path, serde_json::to_vec(&set).unwrap()).unwrap();
        let source = root.join(format!("extracted/{character}-portraits-study"));
        let output = temp.path().join("candidate");
        let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .arg("build-presets")
            .arg("--original")
            .arg(&source)
            .arg("--presets")
            .arg(&set_path)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let colors: Vec<_> = profile["source_colors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| rgba(v.as_str().unwrap()))
            .collect();
        let preset_ids: Vec<_> = set["presets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|preset| preset["id"].as_str().unwrap())
            .collect();
        assert_eq!(
            preset_ids,
            [
                "blue",
                "npc_hayden",
                "npc_ryis",
                "npc_seridia",
                "player_04",
                "npc_march",
                "npc_wiscar",
                "player_16",
                "player_22",
                "player_35"
            ]
        );
        for preset in set["presets"].as_array().unwrap() {
            let id = preset["id"].as_str().unwrap();
            let targets: Vec<_> = preset["colors"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| rgba(v.as_str().unwrap()))
                .collect();
            assert_eq!(targets.len(), colors.len());
            for region in profile["regions"].as_array().unwrap() {
                let asset = region["asset"].as_str().unwrap();
                let before = image::open(source.join(asset)).unwrap().to_rgba8();
                let after = image::open(output.join("variants").join(id).join(asset))
                    .unwrap()
                    .to_rgba8();
                assert_eq!(before.dimensions(), (592, 180));
                assert_eq!(before.dimensions(), after.dimensions());
                let meta = asset.replace(".png", ".meta.toml");
                assert_eq!(
                    fs::read(source.join(&meta)).unwrap(),
                    fs::read(output.join("variants").join(id).join(&meta)).unwrap()
                );
                for (x, y, pixel) in before.enumerate_pixels() {
                    let new = after.get_pixel(x, y);
                    assert_eq!(pixel[3], new[3], "alpha: {asset} [{x},{y}]");
                    if pixel != new {
                        let shade = colors
                            .iter()
                            .position(|color| color == &pixel.0)
                            .unwrap_or_else(|| panic!("unrelated color: {asset} [{x},{y}]"));
                        assert_eq!(new.0, targets[shade], "target: {asset} [{x},{y}]");
                    }
                    // Source-inspected lip/interior colors stay original. The formerly
                    // preserved #B2684A fringe is skin; reina_mouth_correction
                    // independently checks its complete replacement.
                    if character == "reina"
                        && (138..163).contains(&(x % 296))
                        && (78..93).contains(&y)
                        && [
                            [88, 29, 31, 255],
                            [118, 46, 33, 255],
                            [148, 26, 51, 255],
                            [202, 77, 103, 255],
                            [172, 60, 118, 255],
                            [204, 82, 145, 255],
                        ]
                        .contains(&pixel.0)
                    {
                        assert_eq!(pixel, new, "mouth material: {asset} [{x},{y}] {id}");
                        protected_pixels += 1;
                    }
                }
                if character == "juniper" {
                    for frame in 0..2 {
                        let mut protected = vec![];
                        if asset.ends_with("_neutral.png") && !asset.contains("bath_") {
                            protected.push((
                                152,
                                78,
                                if frame == 0 {
                                    rgba("#763F21")
                                } else {
                                    rgba("#592323")
                                },
                            ));
                        }
                        if asset.ends_with("_wild_laugh.png") {
                            protected.extend([
                                (151, if frame == 0 { 74 } else { 76 }, rgba("#763F21")),
                                (152, if frame == 0 { 80 } else { 79 }, rgba("#B87D4C")),
                            ]);
                        }
                        for (x, y, color) in protected {
                            let x = x + frame * 296;
                            assert_eq!(
                                before.get_pixel(x, y).0,
                                color,
                                "source landmark: {asset} [{x},{y}]"
                            );
                            assert_eq!(
                                after.get_pixel(x, y).0,
                                color,
                                "mouth material: {asset} [{x},{y}] {id}"
                            );
                            landmarks += 1;
                        }
                        if asset.ends_with("autumn_neutral.png") {
                            let x = 157 + frame * 296;
                            assert_eq!(before.get_pixel(x, 76).0, rgba("#763F21"));
                            let shade = colors
                                .iter()
                                .position(|color| color == &rgba("#763F21"))
                                .unwrap();
                            assert_eq!(
                                after.get_pixel(x, 76).0,
                                targets[shade],
                                "adjacent skin contour"
                            );
                            landmarks += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(protected_pixels > 1000);
    assert!(landmarks > 100);
    println!(
        "235 strips / 470 frames / ten target palettes: {protected_pixels} protected Reina mouth pixels and {landmarks} independent Juniper material landmarks"
    );
}
