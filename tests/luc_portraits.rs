use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 32 local portraits in extracted/luc-portraits-study"]
fn luc_covers_fine_skin_without_recoloring_glasses_eyes_or_clothes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/luc-portraits-study");
    let recipe = std::env::var_os("FOM_LUC_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/luc-portraits.json"));
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
        [196, 142, 104, 255],
        [179, 104, 68, 255],
        [113, 41, 34, 255],
        [155, 83, 60, 255],
        [131, 70, 46, 255],
        [150, 83, 50, 255],
        [121, 61, 37, 255],
        [232, 184, 150, 255],
        [240, 192, 157, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [68, 95, 131, 255],
        [102, 135, 173, 255],
        [102, 135, 173, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        // Complexion blended into the white glasses rim.
        [192, 209, 225, 255],
        [201, 215, 229, 255],
    ];
    let mut counts = [0; 9];
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
        for (x, y, p) in before.enumerate_pixels() {
            let new = after.get_pixel(x, y);
            assert_eq!(p[3], new[3], "alpha {name} [{x},{y}]");
            if let Some(i) = source.iter().position(|c| c == &p.0) {
                if i >= 7 {
                    assert_eq!(new.0, blue[i], "missed glasses blend {name} [{x},{y}]");
                }
                if p != new {
                    assert_eq!(new.0, blue[i], "target {name} [{x},{y}]");
                    counts[i] += 1;
                }
            } else {
                assert_eq!(p, new, "unrelated color {name} [{x},{y}]");
            }
        }
        // Source-art landmarks are chosen independently of the component seeds.
        if name.ends_with("_neutral.png") {
            for (x, y, i) in [
                (158, 109, 3),
                (155, 104, 0),
                (155, 106, 1),
                (156, 106, 2),
                (160, 98, 2),
                (163, 101, 2),
                (166, 98, 2),
                (167, 99, 4),
                (162, 102, 4),
                (142, 99, 7),
                (147, 105, 8),
            ] {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    source[i],
                    "source skin {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[i],
                    "missed skin {name} [{x},{y}]"
                );
                landmarks += 1;
            }
            for (x, y, c) in [
                (136, 101, source[2]),
                (137, 100, source[2]),
                (143, 96, source[4]),
                (145, 96, source[0]),
                (146, 101, source[1]),
                (171, 94, [245, 245, 245, 255]),
                (164, 90, [210, 150, 47, 255]),
            ] {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    c,
                    "source protected {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    c,
                    "changed protected {name} [{x},{y}]"
                );
                landmarks += 1;
            }
        }
        if name.ends_with("summer_neutral.png") {
            for (x, y, i) in [(180, 131, 5), (121, 139, 6), (153, 117, 4)] {
                assert_eq!(before.get_pixel(x, y).0, source[i]);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[i],
                    "missed Summer skin [{x},{y}]"
                );
                landmarks += 1;
            }
            for (x, y, c) in [
                (119, 138, source[2]),
                (118, 140, source[2]),
                (126, 131, [171, 105, 70, 255]),
            ] {
                assert_eq!(before.get_pixel(x, y).0, c);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    c,
                    "changed Summer sleeve [{x},{y}]"
                );
                landmarks += 1;
            }
            for offset in [0, 296] {
                assert_eq!(before.get_pixel(124 + offset, 151).0, source[0]);
                assert_eq!(
                    after.get_pixel(124 + offset, 151).0,
                    blue[0],
                    "missed Summer forearm"
                );
                landmarks += 1;
            }
        }
        if name.ends_with("spring_neutral.png") {
            for offset in [0, 296] {
                for (x, y, i) in [(135, 151, 0), (176, 133, 5)] {
                    assert_eq!(before.get_pixel(x + offset, y).0, source[i]);
                    assert_eq!(
                        after.get_pixel(x + offset, y).0,
                        blue[i],
                        "missed Spring arm [{x},{y}]"
                    );
                    landmarks += 1;
                }
            }
            for (x, y, c) in [
                (455, 106, [161, 18, 29, 255]),
                (453, 106, [223, 72, 104, 255]),
            ] {
                assert_eq!(before.get_pixel(x, y).0, c);
                assert_eq!(after.get_pixel(x, y).0, c, "changed mouth interior");
                landmarks += 1;
            }
            assert_eq!(before.get_pixel(454, 110).0, source[3]);
            assert_eq!(
                after.get_pixel(454, 110).0,
                blue[3],
                "missed speaking lower lip"
            );
            landmarks += 1;
        }
        if name.ends_with("autumn_neutral.png") {
            for offset in [0, 296] {
                for (x, y, i) in [(160, 120, 1), (126, 146, 4), (184, 152, 1), (137, 119, 1)] {
                    assert_eq!(before.get_pixel(x + offset, y).0, source[i]);
                    assert_eq!(
                        after.get_pixel(x + offset, y).0,
                        source[i],
                        "changed Autumn collar/cuff [{x},{y}]"
                    );
                    landmarks += 1;
                }
            }
        }
        if name.ends_with("_happy.png") || name.ends_with("_wink.png") {
            // The same coordinate is iris in Neutral, skin beside the shut lid here.
            for offset in [0, 296] {
                assert_eq!(before.get_pixel(143 + offset, 96).0, source[4]);
                assert_eq!(
                    after.get_pixel(143 + offset, 96).0,
                    blue[4],
                    "missed closed-eye skin {name}"
                );
                landmarks += 1;
            }
        }
    }
    assert_eq!(landmarks, 111);
    assert!(counts.iter().all(|n| *n > 0));
    assert_eq!(&counts[7..], &[704, 64]);
}
