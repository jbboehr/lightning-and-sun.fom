use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 181 local portraits in extracted/march-portraits-study"]
fn march_masks_cover_fine_skin_shades_and_preserve_eyebrows_mouths_and_necklace() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/march-portraits-study");
    let recipe = std::env::var_os("FOM_MARCH_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/march-portraits.json"));
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
    assert_eq!(rows.len(), 181);
    let source = [
        [238, 221, 165, 255],
        [230, 189, 124, 255],
        [220, 169, 102, 255],
        [204, 129, 85, 255],
        [174, 84, 46, 255],
        [179, 100, 54, 255],
        [180, 97, 62, 255],
        [223, 171, 104, 255],
        [233, 196, 135, 255],
        [237, 202, 146, 255],
        [125, 59, 20, 255],
        [182, 89, 50, 255],
        [208, 161, 121, 255],
    ];
    let light = [157, 185, 212, 255];
    let medium = [127, 159, 189, 255];
    let shadow = [102, 135, 173, 255];
    let dark = [68, 95, 131, 255];
    let blue = [
        light, medium, shadow, dark, dark, dark, dark, shadow, medium, medium, dark, dark, shadow,
    ];
    let mut changed = [0; 13];
    let mut skin_landmarks = 0;
    let mut protected_landmarks = 0;
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
                // The main light tone is exposed skin in every source expression.
                if shade == 0 && (145..173).contains(&(x % 296)) && (50..84).contains(&y) {
                    assert_eq!(new.0, light, "missed face: {name} [{x},{y}]");
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
        }
        // Independent, literal source-art landmarks; these do not inspect seeds.
        let skin: &[(u32, u32)] = if name.ends_with("beach_bath_neutral.png") {
            &[
                (153, 78),
                (160, 80),
                (119, 107),
                (181, 119),
                (121, 131),
                (165, 78),
            ]
        } else if name.ends_with("spring_neutral.png") {
            &[(141, 69), (159, 72), (157, 79)]
        } else if name.ends_with("autumn_hurt_blush.png") {
            &[(150, 71)]
        } else if name.ends_with("wedding_neutral.png") {
            &[(140, 69), (149, 46)]
        } else if name.ends_with("summer_neutral.png") {
            &[(146, 94), (174, 104)]
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
                skin_landmarks += 1;
            }
        }
        if name.ends_with("spring_neutral.png") {
            assert_eq!(before.get_pixel(455, 70).0, [204, 129, 85, 255]);
            assert_eq!(
                after.get_pixel(455, 70),
                before.get_pixel(455, 70),
                "changed open mouth interior"
            );
            assert_eq!(before.get_pixel(459, 72).0, [180, 97, 62, 255]);
            assert_eq!(
                after.get_pixel(459, 72).0,
                dark,
                "missed lower lip skin shadow"
            );
        }
        let protected: &[(u32, u32)] = if name.ends_with("summer_neutral.png") {
            &[(145, 89), (148, 93), (154, 101), (156, 104), (172, 104)]
        } else if name.ends_with("spring_neutral.png") {
            &[(145, 89), (148, 93)]
        } else if name.ends_with("wedding_drunk.png") {
            &[(164, 52), (165, 53)]
        } else {
            &[]
        };
        for &(x, y) in protected {
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
                    "changed accessory or eyebrow: {name} [{x},{y}]"
                );
                protected_landmarks += 1;
            }
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(skin_landmarks, 28);
    assert_eq!(protected_landmarks, 18);
}
