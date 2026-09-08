use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 32 local portraits in extracted/nora-portraits-study"]
fn nora_masks_cover_eye_lip_and_collar_skin_without_changing_hair_jewelry_or_mouth_interior() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/nora-portraits-study");
    let recipe = std::env::var_os("FOM_NORA_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/nora-portraits.json"));
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
        [252, 222, 190, 255],
        [239, 166, 122, 255],
        [182, 89, 50, 255],
        [118, 46, 33, 255],
        [211, 114, 73, 255],
        [244, 191, 148, 255],
        [245, 187, 144, 255],
        [179, 90, 53, 255],
        [252, 176, 182, 255],
        [230, 104, 122, 255],
    ];
    let light = [157, 185, 212, 255];
    let medium = [127, 159, 189, 255];
    let shadow = [102, 135, 173, 255];
    let dark = [68, 95, 131, 255];
    let blue = [
        light,
        medium,
        shadow,
        dark,
        shadow,
        medium,
        medium,
        shadow,
        [157, 147, 203, 255],
        [143, 87, 136, 255],
    ];
    let mut changed = [0; 10];
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
                if shade == 0 && (141..166).contains(&(x % 296)) && (58..102).contains(&y) {
                    assert_eq!(new.0, light, "missed face/neck: {name} [{x},{y}]");
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
        }
        // Literal original-art landmarks are independent of component seeds.
        let skin: &[(u32, u32, usize)] = if name.ends_with("spring_neutral.png") {
            &[
                (151, 58, 0),
                (151, 57, 1),
                (150, 94, 2),
                (158, 77, 3),
                (193, 94, 3),
                (149, 94, 4),
                (151, 92, 5),
                (177, 138, 5),
                (159, 87, 6),
                (150, 65, 7),
                (153, 65, 7),
                (149, 67, 7),
                (151, 68, 7),
                (152, 69, 7),
                (165, 68, 7),
                (165, 69, 7),
                (160, 70, 7),
                (163, 70, 7),
                (165, 72, 7),
            ]
        } else if name.ends_with("winter_neutral.png") {
            &[(156, 104, 0), (157, 106, 1), (156, 100, 2), (157, 100, 3)]
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
                    blue[shade],
                    "missed skin: {name} [{x},{y}]"
                );
                skin_landmarks += 1;
            }
        }
        if name.ends_with("spring_neutral.png") {
            for (x, y, color) in [
                (139, 46, [118, 46, 33, 255]),
                (138, 46, [179, 90, 53, 255]),
                (154, 55, [179, 90, 53, 255]),
                (134, 70, [229, 174, 110, 255]),
                (145, 67, [80, 25, 16, 255]),
                (146, 70, [97, 146, 120, 255]),
                (143, 68, [0, 0, 0, 255]),
            ] {
                for frame in 0..2 {
                    let x = x + frame * 296;
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        color,
                        "source protected: [{x},{y}]"
                    );
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        color,
                        "changed hair/eye detail: [{x},{y}]"
                    );
                    protected_landmarks += 1;
                }
            }
            // Lip edges and the shifted lower-lip shadow follow the skin palette.
            for (x, y, shade) in [(150, 81, 3), (154, 83, 3), (152, 85, 2), (448, 86, 2)] {
                assert_eq!(before.get_pixel(x, y).0, source[shade]);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[shade],
                    "missed lip skin: [{x},{y}]"
                );
                skin_landmarks += 1;
            }
            for (x, y, color) in [
                (446, 81, [118, 46, 33, 255]),
                (451, 82, [118, 46, 33, 255]),
                (450, 83, [118, 46, 33, 255]),
                (447, 81, [161, 18, 29, 255]),
                (447, 82, [249, 111, 140, 255]),
            ] {
                assert_eq!(before.get_pixel(x, y).0, color);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    color,
                    "changed mouth interior: [{x},{y}]"
                );
                protected_landmarks += 1;
            }
        }
        if name.ends_with("autumn_neutral.png") {
            for frame in 0..2 {
                let x = 140 + frame * 296;
                assert_eq!(before.get_pixel(x, 103).0, [118, 46, 33, 255]);
                assert_eq!(
                    after.get_pixel(x, 103),
                    before.get_pixel(x, 103),
                    "changed necklace"
                );
                protected_landmarks += 1;
            }
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(skin_landmarks, 50);
    assert_eq!(protected_landmarks, 21);
}

#[test]
#[ignore = "requires the 32 local portraits in extracted/nora-portraits-study"]
fn nora_lip_colors_follow_each_preset_without_recoloring_tongue_or_mouth_interior() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/nora-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("presets");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("build-presets")
        .arg("--original")
        .arg(&original)
        .arg("--presets")
        .arg(root.join("palettes/sets/nora-portraits-trial.json"))
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for (preset, light, medium) in [
        ("blue", [157, 147, 203, 255], [143, 87, 136, 255]),
        ("npc_hayden", [232, 141, 108, 255], [212, 83, 73, 255]),
        ("npc_ryis", [176, 86, 83, 255], [160, 51, 56, 255]),
        ("npc_seridia", [193, 139, 158, 255], [176, 82, 106, 255]),
    ] {
        let mut lip_counts = [0, 0];
        let mut preserved_interior = 0;
        for season in ["Spring", "Summer", "Autumn", "Winter"] {
            for expression in [
                "embarrassed",
                "happy",
                "mad",
                "neutral",
                "sad",
                "think",
                "ugh",
                "wink",
            ] {
                let asset = format!(
                    "assets/animations/NPCs/Nora/Portraits/{season}/spr_portrait_nora_{}_{expression}.png",
                    season.to_lowercase()
                );
                let before = image::open(original.join(&asset)).unwrap().to_rgba8();
                let after = image::open(output.join("variants").join(preset).join(&asset))
                    .unwrap()
                    .to_rgba8();
                for (x, y, pixel) in before.enumerate_pixels() {
                    let shade = match pixel.0 {
                        [252, 176, 182, 255] => Some(0),
                        [230, 104, 122, 255] => Some(1),
                        _ => None,
                    };
                    if let Some(shade) = shade {
                        assert!(
                            (150..=154).contains(&(x % 296)) && (82..=86).contains(&y),
                            "unexpected lip-color use: {asset} [{x},{y}]"
                        );
                        assert_eq!(
                            after.get_pixel(x, y).0,
                            [light, medium][shade],
                            "unadapted lip: {preset} {asset} [{x},{y}]"
                        );
                        lip_counts[shade] += 1;
                    }
                    if [
                        [161, 18, 29, 255],
                        [206, 72, 101, 255],
                        [249, 111, 140, 255],
                        [180, 87, 50, 255],
                    ]
                    .contains(&pixel.0)
                    {
                        assert_eq!(
                            after.get_pixel(x, y),
                            pixel,
                            "changed tongue/interior: {preset} {asset} [{x},{y}]"
                        );
                        preserved_interior += 1;
                    }
                }
            }
        }
        assert_eq!(lip_counts, [240, 64]);
        assert!(preserved_interior > 0);
    }
}
