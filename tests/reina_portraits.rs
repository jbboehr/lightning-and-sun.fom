use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 103 local portraits in extracted/reina-portraits-study"]
fn reina_masks_cover_skin_details_and_preserve_shared_color_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/reina-portraits-study");
    let recipe = std::env::var_os("FOM_REINA_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/reina-portraits.json"));
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
    assert_eq!(rows.len(), 103);
    let source = [
        [179, 102, 68, 255],
        [158, 81, 47, 255],
        [127, 56, 43, 255],
        [84, 28, 30, 255],
        [173, 95, 60, 255],
        [169, 88, 53, 255],
        [203, 129, 92, 255],
        [179, 104, 68, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [127, 159, 189, 255],
        [127, 159, 189, 255],
        [157, 185, 212, 255],
        [157, 185, 212, 255],
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
                // Source-inspected forehead and nose include the fine nose
                // shadow and warm base beneath blush, absent from the catalog.
                if (131..155).contains(&(x % 296)) && (63..80).contains(&y) {
                    assert_eq!(new.0, blue[shade], "missed face detail: {name} [{x},{y}]");
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
        }
        // Literal landmarks chosen from original artwork, independently of seeds.
        let skin: &[(u32, u32)] = if name.ends_with("spring_neutral.png") {
            &[(158, 85), (169, 139), (124, 178), (147, 178)]
        } else if name.ends_with("summer_neutral.png") {
            &[(140, 138)]
        } else if name.ends_with("beach_neutral.png") {
            &[(170, 132)]
        } else if name.ends_with("wedding_neutral.png") {
            &[(126, 74)]
        } else {
            &[]
        };
        for &(x, y) in skin {
            for frame in 0..2 {
                let x = x + frame * 296;
                let shade = source
                    .iter()
                    .position(|c| c == &before.get_pixel(x, y).0)
                    .unwrap_or_else(|| panic!("skin landmark: {name} [{x},{y}]"));
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[shade],
                    "missed skin: {name} [{x},{y}]"
                );
            }
        }
        let clothing: &[(u32, u32)] = if name.ends_with("spring_neutral.png") {
            &[(138, 164), (177, 120), (162, 146)]
        } else if name.ends_with("autumn_neutral.png") {
            &[(166, 131), (156, 141)]
        } else if name.ends_with("summer_neutral.png") {
            &[(172, 125), (171, 132), (169, 139)]
        } else if name.ends_with("winter_neutral.png") {
            &[(162, 89), (146, 169)]
        } else if name.ends_with("wedding_neutral.png") {
            &[
                (128, 86),
                (128, 89),
                (127, 91),
                (129, 91),
                (126, 92),
                (123, 96),
                (170, 138),
            ]
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
    assert_eq!(protected, 34);
}
