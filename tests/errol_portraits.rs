use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 36 local portraits in extracted/errol-portraits-study"]
fn errol_skin_and_outer_lips_recolor_while_hair_and_mouth_interiors_stay_original() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/errol-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let recipe = std::env::var_os("FOM_ERROL_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/errol-portraits.json"));
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["apply", "--input"])
        .arg(&original)
        .arg("--palette")
        .arg(recipe)
        .arg("--output")
        .arg(&modified)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    let rows = report["files"].as_array().unwrap();
    assert_eq!(rows.len(), 36);
    let source = [
        [244, 196, 156, 255],
        [221, 171, 130, 255],
        [195, 140, 112, 255],
        [155, 102, 86, 255],
        [114, 74, 62, 255],
        [133, 88, 75, 255],
        [201, 146, 123, 255],
        [167, 126, 106, 255],
        [211, 178, 150, 255],
    ];
    let target = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [68, 95, 131, 255],
        [68, 95, 131, 255],
        [102, 135, 173, 255],
        [102, 135, 173, 255],
        [127, 159, 189, 255],
    ];
    let mut changed = [0; 9];
    let mut skin_count = 0;
    let mut protected_count = 0;
    for row in rows {
        let name = row["path"].as_str().unwrap();
        let before = image::open(original.join(name)).unwrap().to_rgba8();
        let after = image::open(modified.join(name)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (592, 180));
        assert_eq!(after.dimensions(), before.dimensions());
        let metadata = name.replace(".png", ".meta.toml");
        let original_metadata = fs::read(original.join(&metadata)).unwrap();
        assert_eq!(
            original_metadata,
            fs::read(modified.join(&metadata)).unwrap()
        );
        let parsed: toml::Value =
            toml::from_str(std::str::from_utf8(&original_metadata).unwrap()).unwrap();
        let season = name.split('/').nth(5).unwrap();
        assert_eq!(
            parsed["asset_properties"]["atlas"].as_str().unwrap(),
            format!("Portraits{season}")
        );
        assert_eq!(
            parsed["asset_properties"]["frame_len"].as_integer(),
            Some(2)
        );
        for (x, y, pixel) in before.enumerate_pixels() {
            let new = after.get_pixel(x, y);
            assert_eq!(pixel[3], new[3], "alpha: {name} [{x},{y}]");
            if let Some(shade) = source.iter().position(|c| c == &pixel.0) {
                if pixel != new {
                    assert_eq!(new.0, target[shade], "wrong color: {name} [{x},{y}]");
                    changed[shade] += 1;
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
        }
        // Source coordinates were inspected independently of the authored component seeds.
        let skin: &[(u32, u32, usize)] = if name.ends_with("spring_neutral.png") {
            &[
                (147, 58, 0),
                (152, 54, 1),
                (149, 57, 2),
                (157, 57, 3),
                (148, 49, 4),
                (109, 153, 4),
                (232, 120, 4),
                (131, 64, 4),
                (216, 120, 2),
                (208, 120, 0),
            ]
        } else if name.ends_with("autumn_neutral.png") {
            &[(134, 47, 5)]
        } else if name.ends_with("summer_neutral.png") {
            &[(134, 80, 6), (168, 88, 5)]
        } else {
            &[]
        };
        for &(x, y, shade) in skin {
            for frame in 0..2 {
                let x = x + frame * 296;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    source[shade],
                    "source skin: {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    target[shade],
                    "missed skin: {name} [{x},{y}]"
                );
                skin_count += 1;
            }
        }
        let protected: &[(u32, u32, [u8; 4])] = if name.ends_with("spring_neutral.png") {
            &[
                (146, 42, [239, 234, 227, 255]),
                (146, 41, [137, 124, 108, 255]),
                (134, 59, [209, 200, 188, 255]),
                (149, 53, [108, 99, 99, 255]),
                (106, 162, [137, 124, 108, 255]),
                (219, 144, [188, 178, 165, 255]),
                (218, 139, [188, 178, 165, 255]),
                (176, 86, [56, 31, 12, 255]),
            ]
        } else if name.ends_with("spring_happy.png") {
            &[(145, 55, [77, 45, 36, 255])]
        } else if name.ends_with("autumn_neutral.png") {
            &[(159, 37, [104, 71, 54, 255]), (135, 35, [58, 34, 30, 255])]
        } else if name.ends_with("summer_neutral.png") {
            &[
                (166, 80, [123, 142, 49, 255]),
                (156, 100, [188, 178, 165, 255]),
            ]
        } else if name.ends_with("winter_neutral.png") {
            &[(123, 84, [119, 76, 42, 255])]
        } else {
            &[]
        };
        for &(x, y, color) in protected {
            for frame in 0..2 {
                let x = x + frame * 296;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    color,
                    "source protected: {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    color,
                    "changed hair/eye/clothes: {name} [{x},{y}]"
                );
                protected_count += 1;
            }
        }
        if name.contains("/Spring/") || name.contains("/Summer/") {
            // Warm infill between gray wrist hairs follows the skin palette.
            // The gray strokes themselves remain covered by the unrelated-color check.
            for (x, y, shade) in [
                (106, 161, 7),
                (107, 161, 7),
                (109, 162, 7),
                (110, 162, 7),
                (111, 162, 7),
                (112, 162, 7),
                (110, 164, 7),
                (111, 164, 7),
                (219, 146, 8),
                (219, 147, 8),
            ] {
                for frame in 0..2 {
                    let x = x + frame * 296;
                    assert_eq!(before.get_pixel(x, y).0, source[shade]);
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        target[shade],
                        "missed skin around arm hair: {name} [{x},{y}]"
                    );
                    skin_count += 1;
                }
            }
        }
        if name.contains("/Summer/") {
            // The same warm fringe around chest hair follows the wrist rule.
            for (x, y) in [(157, 100), (158, 101), (164, 100), (165, 104), (165, 105)] {
                for frame in 0..2 {
                    let x = x + frame * 296;
                    assert_eq!(before.get_pixel(x, y).0, source[8]);
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        target[8],
                        "missed skin around chest hair: {name} [{x},{y}]"
                    );
                    skin_count += 1;
                }
            }
        }
        let offset = u32::from(name.ends_with("_ugh.png"));
        for (x, y) in [(152, 68 + offset), (447, 69 + offset)] {
            assert_eq!(
                before.get_pixel(x, y).0,
                source[4],
                "source outer lip: {name} [{x},{y}]"
            );
            assert_eq!(
                after.get_pixel(x, y).0,
                target[4],
                "missed outer lip: {name} [{x},{y}]"
            );
            skin_count += 1;
        }
        let upper = if ["_mad.png", "_sad.png", "_think.png", "_ugh.png"]
            .iter()
            .any(|suffix| name.ends_with(suffix))
        {
            [82, 47, 20, 255]
        } else {
            source[4]
        };
        assert_eq!(
            before.get_pixel(447, 66 + offset).0,
            upper,
            "source upper interior: {name}"
        );
        assert_eq!(
            after.get_pixel(447, 66 + offset).0,
            upper,
            "changed upper interior: {name}"
        );
        protected_count += 1;
        if name.ends_with("spring_neutral.png") {
            for (x, y, color) in [
                (445, 67, source[4]),
                (454, 67, source[4]),
                (446, 66, [82, 47, 20, 255]),
                (448, 67, [249, 111, 140, 255]),
                (451, 67, [161, 18, 29, 255]),
            ] {
                assert_eq!(before.get_pixel(x, y).0, color);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    color,
                    "changed tongue/interior [{x},{y}]"
                );
                protected_count += 1;
            }
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(skin_count, 548);
    assert_eq!(protected_count, 69);
}
