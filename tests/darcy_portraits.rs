use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 32 local portraits in extracted/darcy-portraits-study"]
fn darcy_covers_arms_eye_fringes_and_lip_edges_without_changing_ornaments_or_mouths() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/darcy-portraits-study");
    let recipe = std::env::var_os("FOM_DARCY_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/darcy-portraits.json"));
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
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
    assert_eq!(rows.len(), 32);
    let source = [
        [252, 216, 169, 255],
        [234, 185, 121, 255],
        [217, 157, 96, 255],
        [183, 103, 47, 255],
        [255, 216, 172, 255],
        [221, 162, 102, 255],
        [144, 69, 34, 255],
        [108, 52, 7, 255],
        [85, 29, 6, 255],
        [123, 40, 6, 255],
    ];
    let light = [157, 185, 212, 255];
    let medium = [127, 159, 189, 255];
    let shadow = [102, 135, 173, 255];
    let dark = [68, 95, 131, 255];
    let blue = [
        light, medium, shadow, dark, light, shadow, dark, dark, dark, dark,
    ];
    let mut changed = [0; 10];
    let mut landmarks = 0;
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
        // These literal source landmarks were inspected independently of profile seeds.
        if name.ends_with("spring_neutral.png") {
            // Fine nose/ear/jaw shades, the collar opening, and skin below the lips.
            for (x, y, shade) in [
                (150, 80, 0),
                (149, 82, 1),
                (159, 82, 2),
                (150, 83, 3),
                (153, 77, 5),
                (153, 79, 6),
                (157, 86, 7),
                (144, 90, 8),
                (153, 90, 8),
                (152, 100, 0),
                (153, 99, 7),
                (132, 77, 6),
                (161, 67, 5),
                (153, 66, 3),
                (444, 85, 6),
                (445, 85, 6),
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
            // Gold accessory, true eye contours, cosmetics, and speaking interior stay original.
            for (x, y, color) in [
                (130, 62, [183, 103, 47, 255]),
                (144, 69, [85, 29, 6, 255]),
                (161, 74, [85, 29, 6, 255]),
                (139, 70, [0, 0, 0, 255]),
                (153, 69, [79, 64, 64, 255]),
                (145, 68, [132, 97, 89, 255]),
                (143, 75, [132, 97, 89, 255]),
                (147, 83, [252, 180, 176, 255]),
                (148, 82, [221, 113, 107, 255]),
                (443, 82, [85, 29, 6, 255]),
                (447, 83, [85, 29, 6, 255]),
                (445, 83, [249, 111, 140, 255]),
                (446, 84, [158, 10, 44, 255]),
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
            for frame in 0..2 {
                let offset = frame * 296;
                for (x, y, shade) in [(120, 118, 4), (118, 121, 9), (135, 130, 9)] {
                    assert_eq!(before.get_pixel(x + offset, y).0, source[shade]);
                    assert_eq!(
                        after.get_pixel(x + offset, y).0,
                        blue[shade],
                        "missed arm [{x},{y}]"
                    );
                    landmarks += 1;
                }
                // The Summer ornament borrows even the lightest skin colors.
                for (x, y, shade) in [(131, 60, 1), (132, 61, 0), (134, 61, 1)] {
                    assert_eq!(before.get_pixel(x + offset, y).0, source[shade]);
                    assert_eq!(
                        after.get_pixel(x + offset, y).0,
                        source[shade],
                        "changed gold [{x},{y}]"
                    );
                    landmarks += 1;
                }
            }
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(landmarks, 41);
}
