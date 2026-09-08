use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 78 local portraits in extracted/eiland-portraits-study"]
fn eiland_masks_recolor_fine_skin_and_keep_gold_and_mouth_interiors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/eiland-portraits-study");
    let recipe = std::env::var_os("FOM_EILAND_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/eiland-portraits.json"));
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
    assert_eq!(rows.len(), 78);
    let source = [
        [233, 169, 128, 255],
        [217, 142, 114, 255],
        [201, 120, 90, 255],
        [156, 82, 65, 255],
        [225, 143, 113, 255],
        [106, 49, 38, 255],
        [193, 122, 91, 255],
        [149, 83, 66, 255],
        [101, 50, 39, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [127, 159, 189, 255],
        [68, 95, 131, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [68, 95, 131, 255],
    ];
    let mut covered = [0; 9];
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
        // Literal landmarks come from enlarged source art, independently of seeds.
        // Coordinates include the animation-frame offset where needed.
        let skin: &[(u32, u32, usize)] = if name.ends_with("spring_neutral.png") {
            &[
                (133, 61, 4),
                (143, 73, 2),
                (144, 73, 3),
                (144, 75, 3),
                (442, 72, 2),
                // User-reported eye, cheek, and collar-gap omissions, both frames.
                (142, 56, 5),
                (438, 56, 5),
                (154, 65, 2),
                (450, 65, 2),
                (153, 75, 2),
                (449, 75, 2),
                (143, 85, 3),
                (439, 85, 3),
                (144, 93, 0),
                (440, 93, 0),
                (144, 100, 5),
                (440, 100, 5),
            ]
        } else if name.ends_with("summer_neutral.png") {
            &[(223, 106, 6), (222, 108, 7), (207, 112, 8)]
        } else if name.ends_with("beach_bath_neutral.png") {
            &[(134, 76, 5), (131, 70, 5)]
        } else if name.ends_with("autumn_neutral.png") {
            &[(146, 94, 2)]
        } else if name.ends_with("winter_neutral.png") || name.ends_with("wedding_neutral.png") {
            &[(143, 85, 3)]
        } else {
            &[]
        };
        for &(x, y, shade) in skin {
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
        let protected: &[(u32, u32, [u8; 4])] = if name.ends_with("spring_neutral.png") {
            &[
                // Black brow hair stays; its brown fringe follows nearby shading.
                (135, 56, [0, 0, 0, 255]),
                (128, 87, [201, 120, 90, 255]),
                (131, 84, [106, 49, 38, 255]),
                (439, 72, [106, 49, 38, 255]),
                (439, 74, [106, 49, 38, 255]),
                (440, 75, [106, 49, 38, 255]),
                (440, 72, [155, 43, 29, 255]),
            ]
        } else if name.ends_with("summer_neutral.png") {
            &[
                (221, 119, [232, 192, 114, 255]),
                (221, 120, [245, 245, 245, 255]),
            ]
        } else if name.ends_with("beach_neutral.png") {
            &[
                (186, 129, [106, 49, 38, 255]),
                (187, 132, [201, 120, 90, 255]),
                (187, 135, [156, 82, 65, 255]),
                (190, 134, [201, 120, 90, 255]),
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
        covered.iter().all(|&n| n > 0),
        "uncovered shade: {covered:?}"
    );
    assert_eq!(landmarks, 38);
}
