use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 32 local portraits in extracted/landen-portraits-study"]
fn landen_covers_palms_eye_fringes_and_lip_skin_while_preserving_facial_hair_and_mouths() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/landen-portraits-study");
    let recipe = std::env::var_os("FOM_LANDEN_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/landen-portraits.json"));
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
        [159, 98, 80, 255],
        [133, 77, 60, 255],
        [99, 52, 42, 255],
        [73, 31, 27, 255],
        [207, 152, 135, 255],
        [193, 129, 110, 255],
        [54, 23, 20, 255],
    ];
    let light = [157, 185, 212, 255];
    let medium = [127, 159, 189, 255];
    let shadow = [102, 135, 173, 255];
    let dark = [68, 95, 131, 255];
    let blue = [light, medium, shadow, dark, light, medium, dark];
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
        // Literal coordinates come from the source art, independent of profile seeds.
        if name.ends_with("spring_neutral.png") {
            // The palm has two lighter shades absent from the NPC catalog.
            for (x, y, shade) in [
                (224, 97, 4),
                (224, 107, 5),
                (157, 71, 0),
                (157, 72, 1),
                (159, 74, 2),
                (158, 43, 3),
                (149, 52, 3),
                (163, 55, 3),
                (154, 59, 3),
                (173, 58, 3),
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
            // Gray moustache/sideburns, black eye detail and actual mouth contours stay original.
            for (x, y, color) in [
                (158, 73, [133, 120, 116, 255]),
                (140, 60, [178, 160, 164, 255]),
                (156, 60, [0, 0, 0, 255]),
                (155, 61, [255, 255, 255, 255]),
                (161, 75, [73, 31, 27, 255]),
                (455, 73, [73, 31, 27, 255]),
                (456, 74, [73, 31, 27, 255]),
                (456, 75, [223, 72, 104, 255]),
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
                assert_eq!(before.get_pixel(202 + offset, 134).0, source[6]);
                assert_eq!(
                    after.get_pixel(202 + offset, 134).0,
                    dark,
                    "missed deep arm contour"
                );
                landmarks += 1;
                for (x, y) in [(169, 118), (170, 119), (169, 126), (170, 127)] {
                    assert_eq!(before.get_pixel(x + offset, y).0, source[4]);
                    assert_eq!(
                        after.get_pixel(x + offset, y).0,
                        source[4],
                        "recolored shirt button"
                    );
                    landmarks += 1;
                }
            }
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(landmarks, 28);
}
