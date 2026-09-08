use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 92 local portraits in extracted/valen-portraits-study"]
fn valen_masks_cover_fine_skin_shades_and_preserve_eye_and_mouth_details() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-portraits-study");
    let recipe = std::env::var_os("FOM_VALEN_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/valen-portraits.json"));
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
    assert_eq!(rows.len(), 92);
    let source = [
        [249, 205, 163, 255],
        [237, 162, 122, 255],
        [205, 130, 90, 255],
        [182, 89, 50, 255],
        [118, 46, 33, 255],
        [220, 134, 98, 255],
        [237, 163, 123, 255],
        [230, 154, 114, 255],
    ];
    let light = [157, 185, 212, 255];
    let medium = [127, 159, 189, 255];
    let shadow = [102, 135, 173, 255];
    let dark = [68, 95, 131, 255];
    let blue = [light, medium, shadow, dark, dark, shadow, medium, medium];
    let mut changed = [0; 8];
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
                if shade == 0 && (135..160).contains(&(x % 296)) && (50..87).contains(&y) {
                    assert_eq!(new.0, light, "missed face: {name} [{x},{y}]");
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
        }
        // Literal landmarks are chosen from the original art, independently of seeds.
        let skin: &[(u32, u32)] = if name.ends_with("spring_neutral.png") {
            &[(135, 62), (147, 91), (132, 151), (147, 76), (149, 80)]
        } else if name.ends_with("spring_panic.png") {
            &[(143, 69)]
        } else if name.ends_with("spring_caring_special.png") {
            &[(144, 64)]
        } else if name.ends_with("beach_bath_neutral.png") {
            &[(153, 87), (129, 145), (132, 151)]
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
        let eyes: &[(u32, u32)] = if name.ends_with("spring_neutral.png") {
            &[(154, 61), (138, 64), (134, 66)]
        } else {
            &[]
        };
        for &(x, y) in eyes {
            for frame in 0..2 {
                let x = x + frame * 296;
                assert_eq!(before.get_pixel(x, y).0, [118, 46, 33, 255]);
                assert_eq!(
                    after.get_pixel(x, y),
                    before.get_pixel(x, y),
                    "changed eye detail: {name} [{x},{y}]"
                );
                protected_landmarks += 1;
            }
        }
        if name.ends_with("spring_neutral.png") {
            assert_eq!(before.get_pixel(444, 77).0, [182, 89, 50, 255]);
            assert_eq!(
                after.get_pixel(444, 77),
                before.get_pixel(444, 77),
                "changed mouth interior"
            );
            assert_eq!(before.get_pixel(445, 81).0, [182, 89, 50, 255]);
            assert_eq!(after.get_pixel(445, 81).0, dark, "missed lower-lip shadow");
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(skin_landmarks, 20);
    assert_eq!(protected_landmarks, 6);
}
