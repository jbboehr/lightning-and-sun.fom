use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 32 local portraits in extracted/josephine-portraits-study"]
fn josephine_masks_cover_fine_skin_without_changing_hair_cosmetics_or_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/josephine-portraits-study");
    let recipe = std::env::var_os("FOM_JOSEPHINE_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/josephine-portraits.json"));
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("apply")
        .arg("--input")
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
    assert_eq!(rows.len(), 32);
    let source = [
        [155, 100, 58, 255],
        [129, 64, 48, 255],
        [117, 47, 35, 255],
        [91, 36, 30, 255],
        [72, 21, 24, 255],
        [141, 80, 54, 255],
        [142, 82, 47, 255],
        [70, 35, 43, 255],
        [58, 29, 36, 255],
    ];
    let light = [157, 185, 212, 255];
    let medium = [127, 159, 189, 255];
    let shadow = [102, 135, 173, 255];
    let dark = [68, 95, 131, 255];
    let blue = [
        light, medium, shadow, dark, dark, medium, medium, dark, dark,
    ];
    let mut changed = [0; 9];
    let mut skin_landmarks = 0;
    let mut protected_landmarks = 0;
    for row in rows {
        let name = row["path"].as_str().unwrap();
        let before = image::open(original.join(name)).unwrap().to_rgba8();
        let after = image::open(modified.join(name)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (592, 180));
        assert_eq!(after.dimensions(), before.dimensions());
        let metadata = name.replace(".png", ".meta.toml");
        assert_eq!(
            fs::read(original.join(&metadata)).unwrap(),
            fs::read(modified.join(&metadata)).unwrap()
        );
        for (x, y, pixel) in before.enumerate_pixels() {
            let new = after.get_pixel(x, y);
            assert_eq!(pixel[3], new[3], "alpha: {name} [{x},{y}]");
            if let Some(shade) = source.iter().position(|c| c == &pixel.0) {
                if pixel != new {
                    assert_eq!(new.0, blue[shade], "wrong color: {name} [{x},{y}]");
                    changed[shade] += 1;
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
        }
        // Coordinates were read directly from the original artwork, independently of mask seeds.
        let skin: &[(u32, u32, usize)] = if name.ends_with("spring_neutral.png") {
            &[
                (155, 78, 0),
                (156, 55, 1),
                (160, 76, 2),
                (150, 89, 4),
                (160, 70, 5),
                (156, 102, 6),
                (179, 99, 7),
                (175, 100, 7),
                (177, 102, 7),
                (161, 74, 8),
                (160, 77, 8),
                (123, 178, 7),
            ]
        } else if name.ends_with("summer_neutral.png") {
            &[(97, 133, 3), (129, 105, 3), (172, 105, 3), (198, 141, 3)]
        } else if name.ends_with("autumn_neutral.png") {
            &[(166, 111, 1), (171, 119, 3)]
        } else if name.ends_with("winter_neutral.png") {
            &[(166, 108, 0), (156, 102, 6), (179, 99, 7)]
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
                    blue[shade],
                    "missed skin: {name} [{x},{y}]"
                );
                skin_landmarks += 1;
            }
        }
        let protected: &[(u32, u32, [u8; 4])] = if name.ends_with("spring_neutral.png") {
            &[
                (135, 50, [70, 35, 43, 255]),
                (161, 63, [58, 29, 36, 255]),
                (174, 114, [58, 29, 36, 255]),
                (154, 111, [164, 79, 53, 255]),
                (98, 150, [91, 36, 30, 255]),
                (166, 100, [245, 241, 235, 255]),
                (139, 80, [254, 213, 173, 255]),
            ]
        } else if name.ends_with("autumn_neutral.png") {
            &[(175, 110, [70, 35, 43, 255])]
        } else if name.ends_with("winter_neutral.png") {
            &[(180, 115, [155, 100, 58, 255])]
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
                    "changed hair/clothing/accessory: {name} [{x},{y}]"
                );
                protected_landmarks += 1;
            }
        }
        if name.ends_with("spring_neutral.png") {
            // Closed and speaking lower-lip skin shadows move with the skin palette.
            for (x, y) in [(158, 84), (454, 86)] {
                assert_eq!(before.get_pixel(x, y).0, source[3]);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    dark,
                    "missed lower-lip skin [{x},{y}]"
                );
                skin_landmarks += 1;
            }
            // Josephine's saturated lipstick already suits the target ramps; retain it and the tongue/interior.
            for (x, y, color) in [
                (157, 82, [193, 88, 113, 255]),
                (158, 80, [111, 38, 53, 255]),
                (451, 81, [91, 36, 30, 255]),
                (454, 80, [105, 12, 19, 255]),
                (453, 81, [249, 111, 140, 255]),
                (452, 81, [223, 72, 104, 255]),
                (455, 79, [70, 35, 43, 255]),
            ] {
                assert_eq!(before.get_pixel(x, y).0, color, "source mouth [{x},{y}]");
                assert_eq!(
                    after.get_pixel(x, y).0,
                    color,
                    "changed cosmetics/tongue/interior [{x},{y}]"
                );
                protected_landmarks += 1;
            }
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(skin_landmarks, 44);
    assert_eq!(protected_landmarks, 25);
}
