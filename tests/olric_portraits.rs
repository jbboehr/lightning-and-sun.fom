use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 36 local portraits in extracted/olric-portraits-study"]
fn olric_masks_cover_eye_and_lip_shading_without_changing_black_outlines_or_mouth_interior() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/olric-portraits-study");
    let recipe = std::env::var_os("FOM_OLRIC_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/olric-portraits.json"));
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
    assert_eq!(rows.len(), 36);
    let source = [
        [242, 219, 167, 255],
        [221, 169, 102, 255],
        [195, 123, 67, 255],
        [183, 103, 47, 255],
        [129, 62, 22, 255],
        [229, 197, 134, 255],
        [182, 89, 50, 255],
        [203, 136, 78, 255],
    ];
    let light = [157, 185, 212, 255];
    let medium = [127, 159, 189, 255];
    let shadow = [102, 135, 173, 255];
    let dark = [68, 95, 131, 255];
    let blue = [light, medium, shadow, dark, dark, light, dark, shadow];
    let mut changed = [0; 8];
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
                if shade == 0 && (136..161).contains(&(x % 296)) && (42..95).contains(&y) {
                    assert_eq!(new.0, light, "missed face/neck: {name} [{x},{y}]");
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
        }
        // Original-art landmarks are independent of the generated component seeds.
        let skin: &[(u32, u32, usize)] = if name.ends_with("spring_neutral.png") {
            &[
                (147, 45, 0),
                (140, 41, 1),
                (156, 50, 2),
                (147, 41, 3),
                (152, 60, 4),
                (152, 52, 5),
                (158, 85, 6),
                (218, 105, 7),
            ]
        } else if name.ends_with("summer_neutral.png") {
            &[(188, 96, 5), (177, 114, 6), (216, 98, 7)]
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
        if name.ends_with("spring_neutral.png") {
            // Brown brow fringes and eye creases follow the surrounding skin.
            for (x, y, shade) in [
                (138, 49, 4),
                (145, 50, 4),
                (155, 48, 4),
                (160, 48, 4),
                (139, 48, 3),
                (146, 48, 3),
                (139, 52, 4),
                (144, 52, 4),
                (154, 51, 4),
                (153, 52, 4),
                (155, 52, 4),
                (159, 52, 4),
                (154, 53, 4),
                (153, 54, 4),
            ] {
                for frame in 0..2 {
                    let x = x + frame * 296;
                    assert_eq!(before.get_pixel(x, y).0, source[shade]);
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        blue[shade],
                        "missed eye/brow shading: [{x},{y}]"
                    );
                    skin_landmarks += 1;
                }
            }
            // Black brow and lash pixels and the gray iris detail remain original.
            for (x, y, color) in [
                (140, 48, [0, 0, 0, 255]),
                (156, 48, [0, 0, 0, 255]),
                (140, 53, [0, 0, 0, 255]),
                (156, 53, [0, 0, 0, 255]),
                (141, 56, [79, 64, 64, 255]),
                (156, 56, [79, 64, 64, 255]),
            ] {
                for frame in 0..2 {
                    let x = x + frame * 296;
                    assert_eq!(before.get_pixel(x, y).0, color);
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        color,
                        "changed eye detail: [{x},{y}]"
                    );
                    protected_landmarks += 1;
                }
            }
            // Light lip edges and the isolated lower-lip shadow follow the target.
            for (x, y, shade) in [(149, 70, 2), (442, 68, 1), (443, 69, 1), (445, 71, 2)] {
                assert_eq!(before.get_pixel(x, y).0, source[shade]);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[shade],
                    "missed lip skin: [{x},{y}]"
                );
                skin_landmarks += 1;
            }
            // Deep open-mouth interior contour and the pink tongue stay original.
            for (x, y, color) in [
                (443, 66, [129, 62, 22, 255]),
                (442, 67, [129, 62, 22, 255]),
                (444, 69, [129, 62, 22, 255]),
                (443, 67, [223, 72, 104, 255]),
                (444, 67, [249, 111, 140, 255]),
            ] {
                assert_eq!(before.get_pixel(x, y).0, color);
                assert_eq!(
                    after.get_pixel(x, y),
                    before.get_pixel(x, y),
                    "changed mouth interior: [{x},{y}]"
                );
                protected_landmarks += 1;
            }
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(skin_landmarks, 54);
    assert_eq!(protected_landmarks, 17);
}
