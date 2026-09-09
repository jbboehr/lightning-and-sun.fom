use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 32 local portraits in extracted/hemlock-portraits-study"]
fn hemlock_covers_fine_skin_and_preserves_hair_and_neckwear() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hemlock-portraits-study");
    let recipe = std::env::var_os("FOM_HEMLOCK_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/hemlock-portraits.json"));
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
        [201, 144, 106, 255],
        [179, 104, 68, 255],
        [138, 63, 49, 255],
        [113, 41, 34, 255],
        [160, 87, 72, 255],
        [191, 123, 90, 255],
        [76, 36, 44, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [102, 135, 173, 255],
        [127, 159, 189, 255],
        [68, 95, 131, 255],
    ];
    let mut changed = [0; 7];
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
        // Literal source-art coordinates do not come from mask seeds.
        if name.ends_with("spring_neutral.png") {
            for (x, y, shade) in [
                (173, 61, 4),
                (156, 81, 5),
                (158, 88, 5),
                (160, 80, 6),
                (167, 86, 6),
                (174, 62, 6),
                (174, 67, 6),
                (173, 69, 6),
                (163, 77, 0),
                (159, 78, 1),
                (167, 75, 2),
                (168, 76, 3),
                (164, 85, 2),
                (464, 76, 3),
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
                (149, 85, [138, 63, 49, 255]),
                (150, 87, [138, 63, 49, 255]),
                (162, 100, [138, 63, 49, 255]),
                (170, 156, [138, 63, 49, 255]),
                (157, 70, [154, 125, 114, 255]),
                (152, 69, [203, 157, 109, 255]),
                (164, 60, [76, 36, 44, 255]),
                (177, 63, [76, 36, 44, 255]),
                (460, 70, [0, 0, 0, 255]),
                (461, 71, [161, 18, 29, 255]),
                (462, 73, [223, 72, 104, 255]),
                (463, 74, [249, 111, 140, 255]),
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
            for (x, y, shade) in [
                (95, 129, 5),
                (192, 131, 5),
                (173, 97, 5),
                (167, 109, 0),
                (126, 163, 1),
                (127, 163, 0),
            ] {
                assert_eq!(before.get_pixel(x, y).0, source[shade]);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[shade],
                    "missed Summer skin [{x},{y}]"
                );
                landmarks += 1;
            }
            assert_eq!(before.get_pixel(109, 141).0, [154, 125, 114, 255]);
            assert_eq!(
                after.get_pixel(109, 141),
                before.get_pixel(109, 141),
                "changed arm hair"
            );
            landmarks += 1;
        }
        if name.ends_with("autumn_neutral.png") {
            // The hanging fingers are separated from the wrist by coat/pants outlines.
            for (x, y) in [(179, 167), (180, 172)] {
                assert_eq!(before.get_pixel(x, y).0, source[3]);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[3],
                    "missed Autumn finger [{x},{y}]"
                );
                landmarks += 1;
            }
        }
        if name.ends_with("winter_neutral.png") {
            assert_eq!(before.get_pixel(180, 146).0, source[3]);
            assert_eq!(after.get_pixel(180, 146).0, blue[3], "missed Winter wrist");
            assert_eq!(before.get_pixel(175, 88).0, source[6]);
            assert_eq!(
                after.get_pixel(175, 88).0,
                source[6],
                "changed Winter collar"
            );
            landmarks += 2;
        }
    }
    assert_eq!(landmarks, 37);
    assert!(changed.iter().all(|count| *count > 0));
}
