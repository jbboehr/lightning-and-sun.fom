use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 109 local portraits in extracted/ryis-portraits-study"]
fn ryis_masks_cover_skin_details_and_preserve_shared_color_accessories() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/ryis-portraits-study");
    let recipe = std::env::var_os("FOM_RYIS_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/ryis-portraits.json"));
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
    assert_eq!(rows.len(), 109);
    let source = [
        [176, 108, 87, 255],
        [129, 74, 58, 255],
        [99, 52, 42, 255],
        [73, 31, 27, 255],
        [153, 91, 72, 255],
        [165, 98, 78, 255],
        [214, 144, 106, 255],
        [194, 123, 85, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [127, 159, 189, 255],
        [127, 159, 189, 255],
        [157, 185, 212, 255],
        [127, 159, 189, 255],
    ];
    let mut covered = [0; 8];
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
                    assert_eq!(new.0, blue[shade], "wrong color: {name} [{x},{y}]");
                    covered[shade] += 1;
                }
                // Hand-inspected face region across expressions, including fine
                // shading omitted by the original four catalog samples.
                if (125..168).contains(&(x % 296)) && (48..91).contains(&y) {
                    assert_eq!(new.0, blue[shade], "missed face detail: {name} [{x},{y}]");
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
        }
        for frame in 0..2 {
            let x = 150 + frame * 296;
            assert_eq!(
                before.get_pixel(x, 52).0,
                source[0],
                "forehead landmark: {name}"
            );
            assert_eq!(after.get_pixel(x, 52).0, blue[0], "missed forehead: {name}");
        }
        // Literal landmarks taken from the source artwork, not from the masks.
        let skin: &[(u32, u32, usize)] = if name.ends_with("beach_bath_neutral.png") {
            &[(124, 96, 5)]
        } else if [
            "spring_neutral.png",
            "summer_neutral.png",
            "autumn_neutral.png",
        ]
        .iter()
        .any(|suffix| name.ends_with(suffix))
        {
            &[(171, 177, 6), (170, 178, 7)]
        } else {
            &[]
        };
        for &(x, y, shade) in skin {
            for frame in 0..2 {
                let x = x + frame * 296;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    source[shade],
                    "skin landmark: {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[shade],
                    "missed body/fingertip: {name} [{x},{y}]"
                );
            }
        }
        let clothing: &[(u32, u32)] = if name.ends_with("spring_neutral.png") {
            &[(123, 94), (177, 161), (184, 172)]
        } else if name.ends_with("autumn_neutral.png") {
            &[(165, 103), (177, 161)]
        } else if name.ends_with("winter_neutral.png") {
            &[(150, 108), (151, 109)]
        } else if name.ends_with("wedding_neutral.png") {
            &[(124, 93), (146, 124), (167, 162)]
        } else if name.ends_with("beach_bath_neutral.png") {
            &[(129, 161), (163, 168)]
        } else {
            &[]
        };
        for &(x, y) in clothing {
            for frame in 0..2 {
                let x = x + frame * 296;
                let pixel = before.get_pixel(x, y);
                assert!(
                    source.contains(&pixel.0),
                    "clothing landmark: {name} [{x},{y}]"
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
    assert!(covered.iter().all(|&count| count > 0));
    assert_eq!(protected, 24);
}
