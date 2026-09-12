use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

const SOURCE: [u32; 9] = [
    0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48, 0x755A54, 0xC3AA9B, 0xAA8E7E, 0xB8A396, 0xBEABA0,
];
const TARGETS: [[u32; 9]; 4] = [
    [
        0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83, 0x445F83, 0x9DB9D4, 0x7F9FBD, 0x94ADC5, 0x9DB9D4,
    ],
    [
        0xE3A17B, 0xD48363, 0xC47054, 0x9F5544, 0x9F5544, 0xE3A17B, 0xD48363, 0xDA956C, 0xE3A17B,
    ],
    [
        0xE8B271, 0xCA9052, 0xB27146, 0x6E4922, 0x6E4922, 0xE8B271, 0xCA9052, 0xDFA662, 0xE8B271,
    ],
    [
        0xB06C57, 0x814A3A, 0x63342A, 0x491F1B, 0x491F1B, 0xB06C57, 0x814A3A, 0xA76048, 0xB06C57,
    ],
];
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}

#[test]
#[ignore = "requires the 98 local strips in extracted/caldarus-portraits-study"]
fn caldarus_covers_skin_and_chest_detail_without_recoloring_horns_markings_or_dragon_forms() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/caldarus-portraits-study");
    let profile_path = root.join("palettes/profiles/caldarus-portraits.json");
    let profile: Value = serde_json::from_slice(&fs::read(&profile_path).unwrap()).unwrap();
    let dragon_regions: Vec<_> = profile["regions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["asset"].as_str().unwrap().contains("/Dragon Statue/"))
        .collect();
    assert_eq!(dragon_regions.len(), 3);
    assert!(
        dragon_regions
            .iter()
            .all(|r| r["seeds"].as_array().unwrap().is_empty())
    );
    let temp = tempfile::tempdir().unwrap();
    let mut blue_selection = Vec::new();
    for (target_index, target) in TARGETS.iter().enumerate() {
        let map: serde_json::Map<String, Value> = SOURCE
            .iter()
            .zip(target)
            .map(|(source, target)| (format!("#{source:06X}"), json!(format!("#{target:06X}"))))
            .collect();
        let recipe = temp.path().join(format!("recipe-{target_index}.json"));
        fs::write(
            &recipe,
            serde_json::to_vec(&json!({"profile":profile_path,"rgba_map":map})).unwrap(),
        )
        .unwrap();
        let recipe = if target_index == 0 {
            std::env::var_os("FOM_CALDARUS_RECIPE")
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
        assert_eq!(rows.len(), 98);
        let mut selection = Vec::new();
        let mut changed = [0; 9];
        let mut skin_landmarks = 0;
        let mut protected_landmarks = 0;
        let mut unchanged_dragons = 0;
        let mut pixel_index = 0;
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
            if name.contains("/Dragon Statue/") {
                assert_eq!(
                    fs::read(original.join(name)).unwrap(),
                    fs::read(modified.join(name)).unwrap()
                );
                assert_eq!(row["changed_pixels"], 0);
                unchanged_dragons += 1;
            }
            // Literal locations were chosen from original art, independently of profile seeds.
            let skin: &[(u32, u32, usize)] = if name.ends_with("_spring_neutral.png") {
                &[
                    (128, 70, 0),
                    (131, 65, 1),
                    (128, 69, 2),
                    (129, 69, 3),
                    (137, 50, 4),
                    (106, 52, 5),
                    (125, 48, 8),
                    (60, 178, 0),
                    (356, 178, 0),
                    (402, 52, 5),
                ]
            } else if name.ends_with("_beach_neutral.png") {
                &[
                    (106, 94, 6),
                    (127, 103, 7),
                    (196, 109, 0),
                    (205, 107, 0),
                    (212, 101, 2),
                    (423, 103, 7),
                ]
            } else if name.ends_with("_beach_bath_neutral.png") {
                &[
                    (106, 94, 6),
                    (127, 103, 7),
                    (196, 109, 0),
                    (212, 101, 2),
                    (165, 100, 0),
                    (423, 103, 7),
                ]
            } else if name.ends_with("_wedding_neutral.png") {
                &[(60, 179, 0), (59, 178, 2), (356, 179, 0)]
            } else {
                &[]
            };
            for &(x, y, shade) in skin {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    rgba(SOURCE[shade]),
                    "source skin {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    rgba(target[shade]),
                    "missed skin {name} [{x},{y}]"
                );
                skin_landmarks += 1;
            }
            let protected: &[(u32, u32, u32)] = if name.ends_with("_spring_neutral.png") {
                &[
                    (113, 17, 0xC1AFA5),
                    (149, 48, 0xA69084),
                    (151, 49, 0x755A54),
                    (149, 50, 0x755A54),
                    (445, 48, 0xA69084),
                    (447, 49, 0x755A54),
                    (66, 123, 0xC1AFA5),
                    (113, 40, 0xA69084),
                    (116, 40, 0x35616F),
                    (120, 56, 0x7E4E37),
                    (111, 66, 0xB36844),
                    (122, 55, 0xC2C2C8),
                    (117, 59, 0xFFFFFF),
                    (118, 81, 0x2D3030),
                    (423, 69, 0xDF4868),
                    (425, 69, 0xA1121D),
                ]
            } else if name.ends_with("_beach_neutral.png") {
                &[
                    (118, 91, 0xFFFFFF),
                    (117, 91, 0xE7D5D5),
                    (150, 96, 0x0DA7C7),
                    (165, 100, 0x68C5C0),
                ]
            } else if name.ends_with("_beach_bath_neutral.png") {
                &[
                    (118, 91, 0xFFFFFF),
                    (117, 91, 0xE7D5D5),
                    (88, 142, 0xE7D5D5),
                    (220, 99, 0xFFFFFF),
                ]
            } else if name.ends_with("_wedding_neutral.png") {
                &[(49, 174, 0x8E746D)]
            } else {
                &[]
            };
            for &(x, y, color) in protected {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    rgba(color),
                    "source protected {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    rgba(color),
                    "changed protected {name} [{x},{y}]"
                );
                protected_landmarks += 1;
            }
            for (x, y, pixel) in before.enumerate_pixels() {
                let new = after.get_pixel(x, y);
                assert_eq!(pixel[3], new[3], "alpha {name} [{x},{y}]");
                if let Some(shade) = SOURCE.iter().position(|c| rgba(*c) == pixel.0) {
                    if pixel != new {
                        assert_eq!(new.0, rgba(target[shade]), "wrong color {name} [{x},{y}]");
                        changed[shade] += 1;
                    }
                    if target_index != 0 {
                        let expected = if blue_selection[pixel_index] {
                            rgba(target[shade])
                        } else {
                            pixel.0
                        };
                        assert_eq!(new.0, expected, "preset selection differs {name} [{x},{y}]");
                    }
                } else {
                    assert_eq!(pixel, new, "unrelated color {name} [{x},{y}]");
                }
                selection.push(pixel != new);
                pixel_index += 1;
            }
        }
        assert_eq!(skin_landmarks, 25);
        assert_eq!(protected_landmarks, 25);
        assert_eq!(unchanged_dragons, 3);
        assert!(changed.iter().all(|n| *n > 0));
        if target_index == 0 {
            blue_selection = selection;
        }
    }
}
