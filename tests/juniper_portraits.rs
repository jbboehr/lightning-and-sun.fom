use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 132 local portraits in extracted/juniper-portraits-study"]
fn juniper_masks_cover_skin_and_preserve_jewelry_clothing_and_mouths() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-portraits-study");
    let recipe = std::env::var_os("FOM_JUNIPER_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/juniper-portraits.json"));
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
    assert_eq!(rows.len(), 132);
    let source = [
        [239, 216, 154, 255],
        [227, 191, 127, 255],
        [210, 171, 102, 255],
        [184, 125, 76, 255],
        [118, 63, 33, 255],
        [89, 35, 35, 255],
        [161, 108, 61, 255],
        [233, 204, 139, 255],
        [210, 156, 102, 255],
        [150, 85, 50, 255],
        [223, 171, 104, 255],
        [157, 90, 51, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
    ];
    let roles = [0, 1, 2, 3, 3, 3, 3, 0, 2, 3, 2, 3];
    let mut covered = [0; 12];
    let mut protected = 0;
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
                    assert_eq!(new.0, blue[roles[shade]], "wrong color: {name} [{x},{y}]");
                    covered[shade] += 1;
                }
                // The light face colors are skin in every expression. The
                // isolated wide-laugh lip detail is deliberately preserved.
                if shade < 4
                    && (135..170).contains(&(x % 296))
                    && (50..90).contains(&y)
                    && !(name.contains("wild_laugh")
                        && (151..=156).contains(&(x % 296))
                        && (74..=82).contains(&y))
                {
                    assert_eq!(
                        new.0, blue[roles[shade]],
                        "missed face detail: {name} [{x},{y}]"
                    );
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
        }
        // Literal landmarks from enlarged source artwork, independent of seeds.
        let skin: &[(u32, u32, usize)] = if name.ends_with("beach_neutral.png") {
            &[(165, 118, 6), (153, 144, 7), (141, 120, 8)]
        } else if name.ends_with("beach_bath_neutral.png") {
            &[(153, 54, 10), (177, 110, 11), (147, 85, 5)]
        } else if name.ends_with("autumn_neutral.png") {
            &[(165, 122, 9), (157, 76, 4)]
        } else if name.ends_with("winter_neutral.png") {
            &[(140, 111, 5), (136, 111, 0)]
        } else {
            &[]
        };
        let clothing: &[(u32, u32, usize)] = if name.ends_with("wedding_neutral.png") {
            &[(155, 144, 5), (165, 102, 5), (182, 113, 5), (126, 128, 5)]
        } else if name.ends_with("spring_neutral.png") {
            &[(126, 120, 4)]
        } else if name.ends_with("summer_neutral.png") {
            &[(156, 103, 5), (194, 154, 5)]
        } else if name.ends_with("autumn_neutral.png") {
            &[(152, 55, 5), (196, 150, 5), (152, 78, 4)]
        } else if name.ends_with("winter_neutral.png") {
            &[(138, 104, 5), (193, 163, 3)]
        } else if name.ends_with("beach_bath_neutral.png") {
            &[(139, 173, 1), (133, 174, 10), (184, 177, 0)]
        } else {
            &[]
        };
        for frame in 0..2 {
            for &(x, y, shade) in skin {
                let x = x + frame * 296;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    source[shade],
                    "source landmark: {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[roles[shade]],
                    "missed fine shading: {name} [{x},{y}]"
                );
            }
            for &(x, y, shade) in clothing {
                let shade =
                    if name.ends_with("autumn_neutral.png") && x == 152 && y == 78 && frame == 1 {
                        5
                    } else {
                        shade
                    };
                let x = x + frame * 296;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    source[shade],
                    "protected source: {name} [{x},{y}]"
                );
                assert_eq!(
                    before.get_pixel(x, y),
                    after.get_pixel(x, y),
                    "changed protected detail: {name} [{x},{y}]"
                );
                protected += 1;
            }
        }
        if name.ends_with("spring_laugh.png") {
            assert_eq!(before.get_pixel(126, 113).0, source[4]);
            assert_eq!(
                before.get_pixel(126, 113),
                after.get_pixel(126, 113),
                "changed shifted armband"
            );
        }
        if name.ends_with("winter_laugh.png") {
            assert_eq!(before.get_pixel(140, 110).0, source[5]);
            assert_eq!(
                after.get_pixel(140, 110).0,
                blue[3],
                "missed shifted arm contour"
            );
        }
        if name.ends_with("autumn_wild_laugh.png") {
            for (x, y, shade) in [(151, 74, 4), (152, 80, 3)] {
                assert_eq!(before.get_pixel(x, y).0, source[shade]);
                assert_eq!(
                    before.get_pixel(x, y),
                    after.get_pixel(x, y),
                    "changed wide-laugh mouth"
                );
            }
        }
    }
    assert!(covered.iter().all(|&count| count > 0));
    assert_eq!(protected, 30);
}
