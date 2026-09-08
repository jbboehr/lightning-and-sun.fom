use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 32 local portraits in extracted/holt-portraits-study"]
fn holt_covers_palms_eye_fringes_and_lip_skin_while_preserving_facial_hair_and_mouths() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/holt-portraits-study");
    let recipe = std::env::var_os("FOM_HOLT_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/holt-portraits.json"));
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
        [245, 200, 142, 255],
        [205, 144, 103, 255],
        [180, 87, 50, 255],
        [118, 46, 33, 255],
        [229, 175, 126, 255],
        [195, 131, 88, 255],
        [163, 89, 60, 255],
        [154, 73, 41, 255],
        [160, 95, 76, 255],
    ];
    let light = [157, 185, 212, 255];
    let medium = [127, 159, 189, 255];
    let shadow = [102, 135, 173, 255];
    let dark = [68, 95, 131, 255];
    let blue = [
        light, medium, shadow, dark, medium, shadow, shadow, dark, shadow,
    ];
    let mut changed = [0; 9];
    let mut landmarks = 0;
    let mut mouth_edge_landmarks = 0;
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
                assert_eq!(pixel, new, "unrelated color: {name} [{x},{y}]");
            }
        }
        // These isolated skin-shading pixels sit below the moustache outline.
        // Speaking frames and the downturned mouths shift the same boundary.
        let mouth_edges: &[(u32, u32, usize)] = if ["embarrassed", "happy", "neutral", "wink"]
            .iter()
            .any(|expression| name.ends_with(&format!("_{expression}.png")))
        {
            &[
                (148, 67, 1),
                (149, 67, 2),
                (156, 67, 2),
                (157, 67, 1),
                (440, 65, 1),
                (452, 66, 2),
                (453, 66, 1),
            ]
        } else if name.ends_with("_sad.png") {
            &[(141, 68, 1), (437, 67, 1)]
        } else if name.ends_with("_ugh.png") {
            &[(141, 69, 1), (437, 68, 1)]
        } else {
            &[]
        };
        for &(x, y, shade) in mouth_edges {
            assert_eq!(
                before.get_pixel(x, y).0,
                source[shade],
                "source mouth edge: {name} [{x},{y}]"
            );
            assert_eq!(
                after.get_pixel(x, y).0,
                blue[shade],
                "missed mouth edge: {name} [{x},{y}]"
            );
            mouth_edge_landmarks += 1;
        }
        // Literal source-art points are independent of the component mask.
        if name.ends_with("spring_neutral.png") {
            for (x, y, shade) in [
                (147, 39, 4),
                (141, 49, 5),
                (126, 67, 6),
                (151, 81, 7),
                (159, 51, 8),
                (148, 73, 0),
                (149, 70, 1),
                (150, 70, 2),
                (151, 80, 3),
                (140, 51, 3),
                (129, 68, 3),
                (158, 88, 1),
            ] {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    source[shade],
                    "source skin [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[shade],
                    "missed skin [{x},{y}]"
                );
                landmarks += 1;
            }
            for (x, y, color) in [
                (131, 30, [205, 144, 103, 255]),
                (143, 62, [205, 144, 103, 255]),
                (137, 47, [154, 73, 41, 255]),
                (140, 46, [118, 46, 33, 255]),
                (129, 46, [118, 46, 33, 255]),
                (131, 55, [154, 73, 41, 255]),
                (159, 50, [75, 33, 26, 255]),
                (154, 52, [108, 99, 99, 255]),
                (153, 53, [255, 255, 255, 255]),
                (442, 66, [223, 72, 104, 255]),
                (442, 65, [161, 18, 29, 255]),
            ] {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    color,
                    "source protected [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    color,
                    "changed protected [{x},{y}]"
                );
                landmarks += 1;
            }
        }
        if name.ends_with("summer_neutral.png") {
            for (x, y, shade) in [(227, 99, 5), (158, 86, 8)] {
                for frame in 0..2 {
                    let x = x + frame * 296;
                    assert_eq!(before.get_pixel(x, y).0, source[shade]);
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        blue[shade],
                        "missed fine hand/neck skin"
                    );
                    landmarks += 1;
                }
            }
        }
        if name.ends_with("spring_ugh.png") {
            // This expression lowers the moustache by a pixel.
            for (x, y, color) in [
                (144, 66, [205, 144, 103, 255]),
                (141, 68, [180, 87, 50, 255]),
            ] {
                assert_eq!(before.get_pixel(x, y).0, color, "source shifted moustache");
                assert_eq!(after.get_pixel(x, y).0, color, "changed shifted moustache");
                landmarks += 1;
            }
        }
        if name.ends_with("winter_neutral.png") {
            for frame in 0..2 {
                let offset = frame * 296;
                for (x, y, color) in [
                    (141, 96, [180, 87, 50, 255]),
                    (198, 111, [180, 87, 50, 255]),
                    (131, 100, [118, 46, 33, 255]),
                ] {
                    assert_eq!(before.get_pixel(x + offset, y).0, color, "source cuff");
                    assert_eq!(
                        after.get_pixel(x + offset, y).0,
                        color,
                        "changed knitted cuff"
                    );
                    landmarks += 1;
                }
                assert_eq!(before.get_pixel(134 + offset, 92).0, source[2]);
                assert_eq!(
                    after.get_pixel(134 + offset, 92).0,
                    shadow,
                    "missed hand crease above cuff"
                );
                landmarks += 1;
            }
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(landmarks, 37);
    assert_eq!(mouth_edge_landmarks, 128);
}
