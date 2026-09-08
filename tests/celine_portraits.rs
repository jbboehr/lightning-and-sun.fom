use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 183 local portraits in extracted/celine-portraits-study"]
fn celine_masks_cover_fine_skin_and_preserve_hair_and_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/celine-portraits-study");
    let recipe = std::env::var_os("FOM_CELINE_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/celine-portraits.json"));
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
    assert_eq!(rows.len(), 183);
    let source = [
        [252, 222, 190, 255],
        [245, 187, 144, 255],
        [234, 152, 102, 255],
        [179, 90, 53, 255],
        [154, 72, 37, 255],
        [226, 138, 85, 255],
        [149, 77, 36, 255],
        [251, 223, 192, 255],
        [237, 167, 125, 255],
        [103, 33, 21, 255],
        [217, 124, 86, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [68, 95, 131, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [68, 95, 131, 255],
        [102, 135, 173, 255],
    ];
    let mut covered = [0; 11];
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
        // These source-inspected landmarks are independent of the profile's seeds.
        let skin: &[(u32, u32)] = if name.ends_with("spring_neutral.png") {
            &[(155, 80), (138, 78), (156, 71)]
        } else if name.ends_with("spring_blush.png") {
            &[(159, 97), (160, 96)]
        } else if name.ends_with("beach_bath_neutral.png") {
            &[(139, 110), (130, 110)]
        } else if name.ends_with("wedding_neutral.png") {
            &[(145, 101), (150, 102), (143, 91), (148, 95)]
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
            &[(148, 43), (164, 109), (167, 114), (169, 120), (145, 99)]
        } else if name.ends_with("wedding_neutral.png") {
            &[(135, 95), (155, 93)]
        } else {
            &[]
        };
        for &(x, y) in clothing {
            for frame in 0..2 {
                let x = x + frame * 296;
                let pixel = before.get_pixel(x, y);
                assert!(
                    source.contains(&pixel.0),
                    "protected landmark: {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y),
                    pixel,
                    "changed hair/clothing: {name} [{x},{y}]"
                );
                protected += 1;
            }
        }
        if name.ends_with("spring_neutral.png") {
            assert_eq!(before.get_pixel(152, 86).0, [179, 90, 53, 255]);
            assert_eq!(
                after.get_pixel(152, 86),
                before.get_pixel(152, 86),
                "changed lower lip: {name}"
            );
        }
        for (x, y, pixel) in before.enumerate_pixels() {
            let new = after.get_pixel(x, y);
            assert_eq!(pixel[3], new[3], "alpha: {name} [{x},{y}]");
            if let Some(shade) = source.iter().position(|c| c == &pixel.0) {
                if pixel != new {
                    assert_eq!(new.0, blue[shade], "wrong color: {name} [{x},{y}]");
                    covered[shade] += 1;
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
            // The upper head is hair and accessories throughout this corpus.
            if y < 65 {
                assert_eq!(pixel, new, "changed upper hair: {name} [{x},{y}]");
            }
        }
    }
    assert!(
        covered.iter().all(|&count| count > 0),
        "uncovered skin shade: {covered:?}"
    );
    assert_eq!(protected, 14);
}
