use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 133 local portraits in extracted/hayden-portraits-study"]
fn hayden_masks_cover_skin_shading_and_preserve_hair_and_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-portraits-study");
    let recipe = std::env::var_os("FOM_HAYDEN_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/hayden-portraits.json"));
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("apply")
        .arg("--input")
        .arg(&original)
        .arg("--palette")
        .arg(&recipe)
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
    assert_eq!(rows.len(), 133);
    let source = [
        [232, 178, 113, 255],
        [202, 144, 82, 255],
        [178, 113, 70, 255],
        [110, 73, 34, 255],
        [250, 216, 168, 255],
        [249, 196, 161, 255],
        [203, 173, 147, 255],
        [202, 137, 77, 255],
        [197, 138, 74, 255],
        [206, 145, 59, 255],
        [159, 125, 111, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [157, 185, 212, 255],
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [127, 159, 189, 255],
        [127, 159, 189, 255],
    ];
    let mut mouth_pixels = 0;
    let mut body_hair_pixels = 0;
    let mut light_counts = [0; 3];
    let mut edge_counts = [0; 2];
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
            match source.iter().position(|c| c == &pixel.0) {
                // In this corpus these three extra colors occur on lighter skin
                // patches, including disconnected pixels at wrists and neckline.
                Some(shade @ 4..=6) => {
                    assert_eq!(new.0, blue[shade], "missed lighter skin: {name} [{x},{y}]");
                    light_counts[shade - 4] += 1;
                }
                // The original four-tone ramp also missed these skin shadows
                // along the hairline, eyebrows, nose, and necklace.
                Some(shade @ 7..=8) => {
                    assert_eq!(new.0, blue[shade], "missed skin edge: {name} [{x},{y}]");
                    edge_counts[shade - 7] += 1;
                }
                Some(9) if (143..160).contains(&(x % 296)) && (50..67).contains(&y) => {
                    assert_eq!(
                        new.0,
                        [127, 159, 189, 255],
                        "missed mouth edge: {name} [{x},{y}]"
                    );
                    mouth_pixels += 1;
                }
                Some(10) if y >= 80 || x % 296 >= 175 => {
                    assert_eq!(
                        new.0,
                        [102, 135, 173, 255],
                        "missed body hair: {name} [{x},{y}]"
                    );
                    body_hair_pixels += 1;
                }
                Some(9 | 10) => {
                    assert_eq!(pixel, new, "changed clothing/head hair: {name} [{x},{y}]")
                }
                Some(shade) if pixel != new => assert_eq!(new.0, blue[shade]),
                None => assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]"),
                _ => (),
            }
        }
        if name.ends_with("spring_neutral.png") {
            for frame in 0..2 {
                // Hair highlight beside the temple, a brown sideburn strand,
                // black eyebrow, and the pendant highlight are not skin.
                for (x, y, rgba) in [
                    (136, 40, [195, 181, 181, 255]),
                    (137, 41, [159, 125, 111, 255]),
                    (145, 35, [0, 0, 0, 255]),
                    (157, 79, [245, 245, 245, 255]),
                ] {
                    let x = x + frame * 296;
                    assert_eq!(before.get_pixel(x, y).0, rgba, "landmark: {name}");
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        rgba,
                        "changed hair/jewelry: {name}"
                    );
                }
            }
        }
        for frame in 0..2 {
            let x = 150 + frame * 296;
            assert_eq!(
                before.get_pixel(x, 30).0,
                source[0],
                "forehead landmark: {name}"
            );
            assert_eq!(after.get_pixel(x, 30).0, blue[0], "missed face: {name}");
        }
        // These literal landmarks use source skin colors on clothing/accessories.
        // An unrestricted color swap would alter them even though its colors match.
        let checks: &[(u32, u32)] = if name.ends_with("summer_neutral.png") {
            &[(116, 117), (106, 127), (145, 93)]
        } else if name.ends_with("spring_neutral.png") {
            &[(187, 103), (111, 120)]
        } else if name.ends_with("autumn_neutral.png") {
            &[(131, 157)]
        } else if name.ends_with("beach_neutral.png") {
            &[(161, 164), (129, 157)]
        } else if name.ends_with("beach_bath_neutral.png") {
            &[(125, 161)]
        } else if name.ends_with("wedding_neutral.png") {
            &[(181, 78)]
        } else {
            &[]
        };
        for &(x, y) in checks {
            for frame in 0..2 {
                let x = x + frame * 296;
                let pixel = before.get_pixel(x, y);
                assert!(
                    source.contains(&pixel.0),
                    "invalid clothing landmark: {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y),
                    pixel,
                    "changed clothing: {name} [{x},{y}]"
                );
                protected += 1;
            }
        }
    }
    assert!(light_counts.iter().all(|&n| n > 0));
    assert!(edge_counts.iter().all(|&n| n > 0));
    assert!(mouth_pixels > 0);
    assert!(body_hair_pixels > 0);
    assert_eq!(protected, 20);
}
