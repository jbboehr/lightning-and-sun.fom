use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

const SOURCE: [[u8; 4]; 6] = [
    [232, 201, 158, 255],
    [203, 162, 130, 255],
    [189, 135, 108, 255],
    [143, 86, 76, 255],
    [90, 47, 47, 255],
    [170, 107, 96, 255],
];
const TARGETS: [[[u8; 4]; 6]; 4] = [
    [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
        [68, 95, 131, 255],
        [102, 135, 173, 255],
    ],
    [
        [232, 178, 113, 255],
        [202, 144, 82, 255],
        [178, 113, 70, 255],
        [110, 73, 34, 255],
        [110, 73, 34, 255],
        [178, 113, 70, 255],
    ],
    [
        [176, 108, 87, 255],
        [129, 74, 58, 255],
        [99, 52, 42, 255],
        [73, 31, 27, 255],
        [73, 31, 27, 255],
        [99, 52, 42, 255],
    ],
    [
        [193, 175, 165, 255],
        [166, 144, 132, 255],
        [142, 116, 109, 255],
        [98, 74, 72, 255],
        [98, 74, 72, 255],
        [142, 116, 109, 255],
    ],
];
fn hex(c: [u8; 4]) -> String {
    format!("#{:02X}{:02X}{:02X}{:02X}", c[0], c[1], c[2], c[3])
}

#[test]
#[ignore = "requires the 36 local strips in extracted/stillwell-portraits-study"]
fn stillwell_covers_fine_face_and_exposed_hands_without_changing_eyes_or_clothes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/stillwell-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let mut blue_selection = Vec::new();
    for (target_index, target) in TARGETS.iter().enumerate() {
        let map: serde_json::Map<String, Value> = SOURCE
            .iter()
            .zip(target)
            .map(|(source, target)| (hex(*source), json!(hex(*target))))
            .collect();
        let recipe = temp.path().join(format!("recipe-{target_index}.json"));
        fs::write(&recipe, serde_json::to_vec(&json!({"profile":root.join("palettes/profiles/stillwell-portraits.json"),"rgba_map":map})).unwrap()).unwrap();
        let recipe = if target_index == 0 {
            std::env::var_os("FOM_STILLWELL_RECIPE")
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
        assert_eq!(rows.len(), 36);
        let mut selection = Vec::new();
        let mut changed = [0; 6];
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
            // Literal coordinates come from original art, independently of stored seeds.
            let skin: &[(u32, u32, usize)] = if name.ends_with("spring_neutral.png") {
                &[
                    (172, 66, 4),
                    (169, 86, 5),
                    (170, 87, 0),
                    (170, 90, 1),
                    (173, 88, 2),
                    (172, 91, 3),
                    (170, 86, 4),
                    (154, 66, 0),
                    (150, 99, 4),
                    (150, 102, 4),
                    (183, 122, 4),
                    (185, 126, 4),
                    (172, 68, 3),
                    (466, 88, 5),
                ]
            } else if name.ends_with("summer_neutral.png") {
                &[
                    (176, 145, 0),
                    (122, 109, 1),
                    (184, 126, 3),
                    (153, 119, 1),
                    (148, 144, 0),
                    (133, 135, 0),
                    (182, 151, 0),
                ]
            } else if name.ends_with("winter_neutral.png") {
                &[(172, 108, 0), (173, 108, 0), (183, 122, 4), (185, 126, 4)]
            } else if name.ends_with("autumn_neutral.png") {
                &[(150, 99, 3), (183, 122, 4), (185, 126, 4)]
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
                    (165, 70, [94, 97, 103, 255]),
                    (171, 72, [108, 99, 99, 255]),
                    (165, 72, [127, 131, 137, 255]),
                    (166, 74, [186, 186, 188, 255]),
                    (147, 98, [55, 66, 92, 255]),
                    (174, 55, [15, 26, 38, 255]),
                    (151, 68, [255, 255, 255, 255]),
                    (466, 86, [249, 111, 140, 255]),
                    (467, 86, [208, 65, 96, 255]),
                ]
            } else if name.ends_with("summer_neutral.png") {
                &[
                    (121, 143, [112, 171, 196, 255]),
                    (116, 147, [98, 145, 165, 255]),
                ]
            } else if name.ends_with("winter_neutral.png") {
                &[
                    (176, 102, [34, 42, 62, 255]),
                    (179, 122, [124, 68, 115, 255]),
                    (146, 121, [96, 50, 89, 255]),
                ]
            } else if name.ends_with("spring_embarrassed.png") {
                &[(165, 77, [219, 137, 118, 255])]
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
        }
        assert_eq!(landmarks, 43);
        assert!(changed.iter().all(|n| *n > 0));
        assert!(SOURCE.iter().zip(target).all(|(a, b)| a != b));
        if target_index == 0 {
            blue_selection = selection;
        } else {
            assert_eq!(selection, blue_selection, "preset mask differs");
        }
    }
}
