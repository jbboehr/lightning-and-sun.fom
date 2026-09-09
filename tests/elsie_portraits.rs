use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 36 local portraits in extracted/elsie-portraits-study"]
fn elsie_covers_wrinkles_and_fine_skin_without_recoloring_cosmetics_or_accessories() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/elsie-portraits-study");
    let recipe = std::env::var_os("FOM_ELSIE_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/elsie-portraits.json"));
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
    assert_eq!(rows.len(), 36);
    let source = [
        [246, 210, 175, 255],
        [227, 181, 152, 255],
        [203, 148, 114, 255],
        [180, 109, 95, 255],
        [121, 64, 53, 255],
        [76, 15, 30, 255],
        [203, 152, 121, 255],
        [120, 75, 66, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [68, 95, 131, 255],
        [68, 95, 131, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
    ];
    let mut changed = [0; 8];
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
        // Literal source-art landmarks are independent of the component seeds.
        if name.ends_with("spring_neutral.png") {
            for (x, y, shade) in [
                (155, 73, 0),
                (153, 73, 1),
                (153, 72, 2),
                (161, 70, 3),
                (154, 59, 4),
                (137, 68, 4),
                (161, 67, 4),
                (153, 78, 4),
                (153, 84, 4),
                (155, 85, 4),
                (160, 86, 4),
                (152, 83, 5),
                (154, 84, 5),
                (158, 85, 5),
                (152, 99, 4),
                (174, 117, 4),
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
            // True brow strokes, earring, ring, nail and mouth colors remain original.
            for (x, y, color) in [
                (149, 56, [76, 15, 30, 255]),
                (152, 56, [143, 68, 87, 255]),
                (140, 77, [101, 39, 58, 255]),
                (141, 74, [193, 134, 78, 255]),
                (169, 92, [76, 15, 30, 255]),
                (171, 93, [76, 15, 30, 255]),
                (158, 95, [101, 39, 58, 255]),
                (157, 74, [214, 83, 105, 255]),
                (451, 74, [101, 39, 58, 255]),
                (453, 75, [252, 112, 141, 255]),
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
                // Deep arm/chest creases and the continuous finger underside are skin.
                for (x, y) in [(134, 118), (175, 120), (93, 173), (161, 95)] {
                    assert_eq!(before.get_pixel(x + offset, y).0, source[4]);
                    assert_eq!(
                        after.get_pixel(x + offset, y).0,
                        blue[4],
                        "missed Summer skin [{x},{y}]"
                    );
                    landmarks += 1;
                }
                for (x, y) in [
                    (130, 93),
                    (139, 92),
                    (133, 100),
                    (151, 92),
                    (155, 93),
                    (153, 94),
                ] {
                    assert_eq!(before.get_pixel(x + offset, y).0, source[4]);
                    assert_eq!(
                        after.get_pixel(x + offset, y).0,
                        source[4],
                        "changed bow [{x},{y}]"
                    );
                    landmarks += 1;
                }
            }
        }
        if name.ends_with("winter_neutral.png") {
            for (x, y) in [(129, 42), (143, 62), (152, 89)] {
                assert_eq!(before.get_pixel(x, y).0, source[4]);
                assert_eq!(after.get_pixel(x, y).0, source[4], "changed hood [{x},{y}]");
                landmarks += 1;
            }
            assert_eq!(before.get_pixel(152, 83).0, source[5]);
            assert_eq!(
                after.get_pixel(152, 83).0,
                blue[5],
                "missed jaw beside hood"
            );
            landmarks += 1;
        }
        // Expression-specific skin fringes are absent from the neutral catalog sample.
        for (suffix, points) in [
            ("autumn_closed_eyes.png", &[(153, 61, 6), (144, 64, 7)][..]),
            ("autumn_happy.png", &[(148, 64, 6), (147, 64, 7)][..]),
            ("winter_closed_eyes.png", &[(144, 61, 4)][..]),
        ] {
            if name.ends_with(suffix) {
                for &(x, y, shade) in points {
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        source[shade],
                        "source fringe [{x},{y}]"
                    );
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        blue[shade],
                        "missed fringe [{x},{y}]"
                    );
                    landmarks += 1;
                }
            }
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(landmarks, 55);
}
