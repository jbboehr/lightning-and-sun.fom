use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 110 local portraits in extracted/balor-portraits-study"]
fn balor_masks_cover_skin_gaps_and_preserve_mouth_scar_and_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-portraits-study");
    let recipe = std::env::var_os("FOM_BALOR_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/balor-portraits.json"));
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
    assert_eq!(rows.len(), 110);
    let source = [
        [252, 222, 190, 255],
        [239, 166, 122, 255],
        [193, 102, 64, 255],
        [145, 74, 44, 255],
        [244, 191, 152, 255],
        [167, 89, 57, 255],
        [97, 32, 38, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [127, 159, 189, 255],
        [68, 95, 131, 255],
        [68, 95, 131, 255],
    ];
    let mut covered = [0; 7];
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

        // Literal source landmarks were picked from enlarged source art, not seeds.
        // The body landmarks are shared by both animation frames; mouths are not.
        let skin: &[(u32, u32, usize)] = if name.ends_with("spring_neutral.png") {
            &[
                (156, 61, 5),
                (157, 67, 6),
                (153, 112, 2),
                (187, 141, 0),
                (187, 140, 1),
            ]
        } else if name.ends_with("summer_neutral.png") {
            &[(162, 117, 6)]
        } else if name.ends_with("beach_bath_neutral.png") {
            &[(178, 77, 4), (144, 77, 6)]
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
                landmarks += 1;
            }
        }
        let protected: &[(u32, u32, [u8; 4])] = if name.ends_with("spring_neutral.png") {
            &[
                (148, 53, [97, 32, 38, 255]),     // eyebrow
                (131, 152, [97, 32, 38, 255]),    // red clothing
                (149, 71, [97, 32, 38, 255]),     // closed smile corner
                (154, 72, [97, 32, 38, 255]),     // opposite smile corner
                (184, 140, [179, 175, 189, 255]), // fabric beside the sleeve tear
            ]
        } else if name.ends_with("beach_bath_neutral.png") {
            &[
                (169, 97, [248, 181, 162, 255]),  // scar shadow
                (167, 101, [253, 197, 182, 255]), // scar highlight
            ]
        } else {
            &[]
        };
        for &(x, y, color) in protected {
            assert_eq!(
                before.get_pixel(x, y).0,
                color,
                "source feature: {name} [{x},{y}]"
            );
            assert_eq!(
                after.get_pixel(x, y).0,
                color,
                "changed feature: {name} [{x},{y}]"
            );
            landmarks += 1;
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
        }
    }
    assert!(
        covered.iter().all(|&count| count > 0),
        "uncovered skin shade: {covered:?}"
    );
    assert_eq!(landmarks, 23);
}

#[test]
#[ignore = "requires the 110 local portraits in extracted/balor-portraits-study"]
fn balor_ryis_lip_shading_follows_skin_in_closed_and_speaking_frames() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-portraits-study");
    let profile = root.join("palettes/profiles/balor-portraits.json");
    let masks: Value = serde_json::from_slice(&fs::read(&profile).unwrap()).unwrap();
    let set: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/sets/balor-portraits-trial.json")).unwrap(),
    )
    .unwrap();
    let ryis = set["presets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == "npc_ryis")
        .unwrap();
    let mapping: serde_json::Map<String, Value> = masks["source_colors"]
        .as_array()
        .unwrap()
        .iter()
        .zip(ryis["colors"].as_array().unwrap())
        .map(|(source, target)| (source.as_str().unwrap().to_owned(), target.clone()))
        .collect();
    let temp = tempfile::tempdir().unwrap();
    let recipe = temp.path().join("ryis.json");
    fs::write(
        &recipe,
        serde_json::to_vec(&serde_json::json!({"profile": profile, "rgba_map": mapping})).unwrap(),
    )
    .unwrap();
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
    let mut checked = 0;
    for row in report["files"].as_array().unwrap() {
        let name = row["path"].as_str().unwrap();
        if !name.ends_with("_neutral.png") {
            continue;
        }
        let before = image::open(original.join(name)).unwrap().to_rgba8();
        let after = image::open(modified.join(name)).unwrap().to_rgba8();
        // Independently inspected lip edges and lower-lip shadows, across all
        // outfits. The closed smile and speaking frame use different pixels.
        for (frame, x, y, source, target) in [
            (0, 148, 70, [239, 166, 122, 255], [129, 74, 58, 255]),
            (0, 150, 71, [193, 102, 64, 255], [99, 52, 42, 255]),
            (0, 151, 72, [239, 166, 122, 255], [129, 74, 58, 255]),
            (0, 151, 74, [193, 102, 64, 255], [99, 52, 42, 255]),
            (1, 151, 70, [239, 166, 122, 255], [129, 74, 58, 255]),
            (1, 150, 70, [193, 102, 64, 255], [99, 52, 42, 255]),
            (1, 148, 72, [239, 166, 122, 255], [129, 74, 58, 255]),
            (1, 154, 73, [193, 102, 64, 255], [99, 52, 42, 255]),
            (1, 151, 76, [193, 102, 64, 255], [99, 52, 42, 255]),
        ] {
            let x = x + frame * 296;
            assert_eq!(
                before.get_pixel(x, y).0,
                source,
                "source lip: {name} [{x},{y}]"
            );
            assert_eq!(
                after.get_pixel(x, y).0,
                target,
                "missed lip shading: {name} [{x},{y}]"
            );
        }
        for (frame, x, y) in [(0, 149, 71), (0, 154, 72), (1, 151, 71)] {
            let x = x + frame * 296;
            assert_eq!(before.get_pixel(x, y).0, [97, 32, 38, 255]);
            assert_eq!(
                after.get_pixel(x, y),
                before.get_pixel(x, y),
                "changed mouth interior: {name} [{x},{y}]"
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 7);
}
