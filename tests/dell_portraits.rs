use serde_json::Value;
use std::{fs, path::Path, process::Command};
#[test]
#[ignore = "requires the 32 local portraits in extracted/dell-portraits-study"]
fn dell_skin_edges_recolor_without_changing_hair_bandage_or_mouth_interior() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/dell-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let recipe = std::env::var_os("FOM_DELL_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/dell-portraits.json"));
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
        [221, 144, 91, 255],
        [168, 87, 28, 255],
        [105, 27, 3, 255],
        [145, 57, 14, 255],
        [157, 68, 40, 255],
        [177, 86, 52, 255],
        [236, 170, 113, 255],
        [255, 222, 180, 255],
        [236, 152, 113, 255],
        [219, 145, 80, 255],
        [119, 12, 29, 255],
    ];
    let light = [157, 185, 212, 255];
    let medium = [127, 159, 189, 255];
    let shadow = [102, 135, 173, 255];
    let dark = [68, 95, 131, 255];
    let target = [
        light, medium, shadow, dark, dark, shadow, shadow, medium, light, medium, medium, dark,
    ];
    let mut changed = [0; 12];
    let mut skin_count = 0;
    let mut protected_count = 0;
    let mut fringe_count = 0;
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
                    assert_eq!(new.0, target[shade], "wrong color: {name} [{x},{y}]");
                    changed[shade] += 1;
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
        }
        // Literal coordinates were read from the original sprites, independently of component seeds.
        // These isolated warm pixels belong to the skin edge, beside the outer
        // eye and below the ear. Happy and wink close the viewer-right eye.
        for frame in 0..2 {
            for (x, y) in [(139, 105)].into_iter().chain(
                (!name.ends_with("_happy.png") && !name.ends_with("_wink.png"))
                    .then_some((163, 96)),
            ) {
                let x = x + frame * 296;
                assert_eq!(before.get_pixel(x, y).0, source[10]);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    target[10],
                    "missed eye/ear skin fringe: {name} [{x},{y}]"
                );
                fringe_count += 1;
            }
        }
        let skin: &[(u32, u32, usize)] = if name.ends_with("spring_neutral.png") {
            &[
                (150, 101, 0),
                (147, 83, 1),
                (154, 99, 2),
                (154, 102, 3),
                (148, 112, 4),
                (153, 98, 7),
                (147, 97, 8),
                (154, 101, 10),
                (137, 105, 11),
                (130, 168, 11),
                (133, 99, 2),
                (135, 100, 2),
                (136, 101, 2),
                (153, 125, 2),
                (154, 125, 2),
                (113, 146, 2),
                (114, 147, 11),
                (141, 109, 2),
                (144, 110, 2),
                (146, 111, 2),
                (143, 109, 10),
                (141, 110, 10),
            ]
        } else if name.ends_with("spring_happy.png") {
            &[(142, 97, 5)]
        } else if name.ends_with("spring_sad.png") {
            &[(154, 98, 9), (154, 99, 6)]
        } else if name.ends_with("summer_neutral.png") {
            &[
                (112, 145, 10),
                (113, 146, 2),
                (114, 147, 11),
                (125, 160, 10),
                (172, 168, 10),
                (141, 109, 2),
                (144, 110, 2),
                (146, 111, 2),
                (143, 109, 10),
                (141, 110, 10),
                (150, 115, 10),
                (151, 116, 10),
            ]
        } else if name.ends_with("autumn_neutral.png") {
            &[
                (145, 113, 10),
                (143, 109, 10),
                (141, 109, 2),
                (144, 110, 2),
                (146, 111, 2),
                (141, 113, 2),
                (143, 114, 2),
            ]
        } else if name.ends_with("winter_neutral.png") {
            &[
                (145, 113, 10),
                (143, 109, 10),
                (141, 109, 2),
                (144, 110, 2),
                (146, 111, 2),
                (145, 112, 2),
                (147, 113, 2),
                (141, 114, 2),
                (149, 114, 2),
                (143, 115, 2),
            ]
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
                (147, 81, source[2]),
                (133, 62, source[10]),
                (142, 88, source[3]),
                (133, 92, source[10]),
                (133, 95, source[2]),
                (135, 105, source[2]),
                (132, 103, source[10]),
                (132, 63, [255, 224, 165, 255]),
            ]
        } else if name.ends_with("summer_neutral.png") {
            &[
                (105, 147, source[8]),
                (104, 146, source[0]),
                (136, 152, source[2]),
            ]
        } else if name.ends_with("winter_neutral.png") {
            &[(140, 115, source[2]), (142, 116, source[2])]
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
                    "changed hair/accessory: {name} [{x},{y}]"
                );
                protected_count += 1;
            }
        }
        if name.ends_with("spring_neutral.png") {
            for (x, y, shade) in [(151, 105, 6), (446, 107, 1)] {
                assert_eq!(before.get_pixel(x, y).0, source[shade]);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    target[shade],
                    "missed outer mouth shading [{x},{y}]"
                );
                skin_count += 1;
            }
            for (x, y, color) in [
                (446, 104, [155, 43, 29, 255]),
                (446, 105, [233, 105, 132, 255]),
                (447, 105, [197, 56, 73, 255]),
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
    assert!(changed.iter().all(|&n| n > 0));
    assert_eq!(skin_count, 110);
    assert_eq!(protected_count, 29);
    assert_eq!(fringe_count, 112);
}
