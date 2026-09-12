use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

const SOURCE: [[u8; 4]; 8] = [
    [221, 146, 102, 255],
    [203, 122, 86, 255],
    [179, 102, 68, 255],
    [127, 56, 43, 255],
    [158, 81, 47, 255],
    [126, 76, 59, 255],
    [240, 168, 126, 255],
    [72, 35, 35, 255],
];
const TARGETS: [[[u8; 4]; 8]; 4] = [
    [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [68, 95, 131, 255],
        [68, 95, 131, 255],
        [176, 207, 236, 255],
        [68, 95, 131, 255],
    ],
    [
        [232, 178, 113, 255],
        [202, 144, 82, 255],
        [178, 113, 70, 255],
        [110, 73, 34, 255],
        [110, 73, 34, 255],
        [110, 73, 34, 255],
        [251, 200, 137, 255],
        [110, 73, 34, 255],
    ],
    [
        [176, 108, 87, 255],
        [129, 74, 58, 255],
        [99, 52, 42, 255],
        [73, 31, 27, 255],
        [73, 31, 27, 255],
        [73, 31, 27, 255],
        [195, 130, 111, 255],
        [73, 31, 27, 255],
    ],
    [
        [193, 175, 165, 255],
        [166, 144, 132, 255],
        [142, 116, 109, 255],
        [98, 74, 72, 255],
        [98, 74, 72, 255],
        [98, 74, 72, 255],
        [212, 197, 189, 255],
        [98, 74, 72, 255],
    ],
];
fn hex(c: [u8; 4]) -> String {
    format!("#{:02X}{:02X}{:02X}{:02X}", c[0], c[1], c[2], c[3])
}

#[test]
#[ignore = "requires the 32 local strips in extracted/merri-portraits-study"]
fn merri_covers_skin_fringes_and_preserves_iris_and_clothing_in_all_presets() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/merri-portraits-study");
    let set: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/sets/merri-portraits-trial.json")).unwrap(),
    )
    .unwrap();
    let temp = tempfile::tempdir().unwrap();
    let mut blue_selection = Vec::new();
    for (target_index, target) in TARGETS.iter().enumerate() {
        let preset = &set["presets"][target_index];
        let mut map = serde_json::Map::new();
        for (shade, source) in SOURCE.iter().enumerate() {
            assert_eq!(
                preset["colors"][shade]
                    .as_str()
                    .unwrap()
                    .to_uppercase()
                    .trim_end_matches("FF"),
                hex(target[shade]).trim_end_matches("FF")
            );
            map.insert(hex(*source), json!(hex(target[shade])));
        }
        let recipe = temp.path().join(format!("recipe-{target_index}.json"));
        fs::write(&recipe, serde_json::to_vec(&json!({"profile":root.join("palettes/profiles/merri-portraits.json"),"rgba_map":map})).unwrap()).unwrap();
        let recipe = if target_index == 0 {
            std::env::var_os("FOM_MERRI_RECIPE")
                .map(std::path::PathBuf::from)
                .unwrap_or(recipe)
        } else {
            recipe
        };
        let modified = temp.path().join(format!("modified-{target_index}"));
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
        assert_eq!(rows.len(), 32);
        let mut selection = Vec::new();
        let mut changed = [0; 8];
        let mut landmarks = 0;
        for row in rows {
            let name = row["path"].as_str().unwrap();
            let before = image::open(original.join(name)).unwrap().to_rgba8();
            let after = image::open(modified.join(name)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), (592, 180));
            assert_eq!(after.dimensions(), before.dimensions());
            let meta = name.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(modified.join(&meta)).unwrap()
            );
            for (x, y, pixel) in before.enumerate_pixels() {
                let new = after.get_pixel(x, y);
                assert_eq!(pixel[3], new[3], "alpha {name} [{x},{y}]");
                if let Some(shade) = SOURCE.iter().position(|c| c == &pixel.0) {
                    if pixel != new {
                        assert_eq!(new.0, target[shade], "wrong color {name} [{x},{y}]");
                        changed[shade] += 1;
                    }
                } else {
                    assert_eq!(pixel, new, "unrelated color {name} [{x},{y}]");
                }
                selection.push(pixel != new);
            }
            // These coordinates were chosen from original art, independently of mask seeds.
            let skin: &[(u32, u32, usize)] = if name.ends_with("spring_neutral.png") {
                &[
                    (139, 64, 4),
                    (150, 80, 5),
                    (145, 71, 6),
                    (141, 87, 7),
                    (136, 83, 7),
                    (159, 66, 7),
                    (160, 67, 7),
                    (150, 79, 0),
                    (150, 72, 1),
                    (149, 79, 2),
                    (153, 76, 3),
                    (143, 88, 7),
                    (145, 89, 7),
                    (148, 90, 7),
                    (115, 172, 0),
                    (441, 71, 6),
                    (446, 84, 0),
                ]
            } else if name.ends_with("summer_neutral.png") {
                &[(201, 98, 4), (115, 163, 0), (202, 88, 0), (98, 139, 0)]
            } else {
                &[]
            };
            for &(x, y, shade) in skin {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    SOURCE[shade],
                    "source skin {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    target[shade],
                    "missed skin {name} [{x},{y}]"
                );
                landmarks += 1;
            }
            let protected: &[(u32, u32, [u8; 4])] = if name.ends_with("spring_neutral.png") {
                &[
                    (141, 70, [72, 35, 35, 255]),
                    (142, 70, [72, 35, 35, 255]),
                    (156, 70, [72, 35, 35, 255]),
                    (141, 62, [0, 0, 0, 255]),
                    (445, 79, [223, 72, 104, 255]),
                    (446, 81, [249, 111, 140, 255]),
                ]
            } else if name.ends_with("summer_neutral.png") {
                &[(102, 127, [166, 133, 107, 255])]
            } else if name.ends_with("winter_neutral.png") {
                &[(151, 95, [211, 199, 186, 255])]
            } else {
                &[]
            };
            for &(x, y, color) in protected {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    color,
                    "source protected {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    color,
                    "changed protected {name} [{x},{y}]"
                );
                landmarks += 1;
            }
        }
        assert_eq!(landmarks, 29);
        assert!(changed.iter().all(|n| *n > 0));
        // None of these literal source/target pairs is an identity mapping.
        assert!(SOURCE.iter().zip(target).all(|(a, b)| a != b));
        if target_index == 0 {
            blue_selection = selection;
        } else {
            assert_eq!(selection, blue_selection, "preset mask differs");
        }
    }
}
