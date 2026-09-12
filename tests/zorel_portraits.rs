use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

const SOURCE: [u32; 10] = [
    0xB0755D, 0x985750, 0x874048, 0x642E3D, 0x432234, 0xA56553, 0x7A354C, 0xDA977E, 0x975250,
    0x7A3118,
];
const TARGETS: [[u32; 10]; 4] = [
    [
        0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83, 0x445F83, 0x7F9FBD, 0x445F83, 0xC7DBF5, 0x7F9FBD,
        0x445F83,
    ],
    [
        0xE8B271, 0xCA9052, 0xB27146, 0x6E4922, 0x6E4922, 0xCA9052, 0x6E4922, 0xFFD492, 0xCA9052,
        0x6E4922,
    ],
    [
        0xB06C57, 0x814A3A, 0x63342A, 0x491F1B, 0x491F1B, 0x814A3A, 0x491F1B, 0xDA8E78, 0x814A3A,
        0x491F1B,
    ],
    [
        0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48, 0x624A48, 0xA69084, 0x624A48, 0xEBD1C6, 0xA69084,
        0x624A48,
    ],
];
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}

#[test]
#[ignore = "requires the 32 local strips in extracted/zorel-portraits-study"]
fn zorel_covers_fine_skin_blends_and_hands_without_changing_hair_gloves_or_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/zorel-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let mut blue_selection = Vec::new();
    for (target_index, target) in TARGETS.iter().enumerate() {
        let map: serde_json::Map<String, Value> = SOURCE
            .iter()
            .zip(target)
            .map(|(source, target)| (format!("#{source:06X}"), json!(format!("#{target:06X}"))))
            .collect();
        let recipe = temp.path().join(format!("recipe-{target_index}.json"));
        fs::write(&recipe, serde_json::to_vec(&json!({"profile":root.join("palettes/profiles/zorel-portraits.json"),"rgba_map":map})).unwrap()).unwrap();
        let recipe = if target_index == 0 {
            std::env::var_os("FOM_ZOREL_RECIPE")
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
        let mut changed = [0; 10];
        let mut skin_landmarks = 0;
        let mut protected_landmarks = 0;
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
            // Coordinates were chosen from original art, independently of profile seeds.
            let mut skin = Vec::new();
            let mut protected = Vec::new();
            if name.ends_with("_neutral.png") {
                for frame in 0..2 {
                    for (x, y, shade) in [
                        (135, 72, 0),
                        (140, 64, 1),
                        (140, 63, 2),
                        (156, 60, 3),
                        (156, 59, 4),
                        (145, 73, 5),
                        (136, 68, 6),
                    ] {
                        skin.push((x + frame * 296, y, shade));
                    }
                }
                protected.extend([
                    (148, 34, 0x3F2A46),
                    (150, 39, 0x241730),
                    (139, 72, 0x1C4E44),
                    (136, 71, 0xBABBBC),
                    (137, 70, 0x777782),
                    (157, 71, 0x605559),
                    (442, 82, 0xE95E7A),
                    (444, 82, 0x971831),
                ]);
            }
            if name.ends_with("spring_neutral.png") {
                skin.extend([(83, 104, 7), (212, 96, 7), (118, 147, 8)]);
                protected.extend([
                    (137, 68, 0x6B1E6B),
                    (223, 108, 0xF4E3C0),
                    (222, 108, 0xC8A082),
                    (207, 121, 0xDBB796),
                    (202, 124, 0x85605B),
                    (188, 61, 0xC09893),
                ]);
            }
            if name.ends_with("summer_neutral.png") {
                skin.push((214, 102, 7));
            }
            if name.ends_with("winter_neutral.png") {
                skin.extend([(147, 99, 0), (153, 101, 1), (151, 105, 3), (214, 102, 7)]);
                for frame in 0..2 {
                    for (x, y) in [
                        (71, 112),
                        (69, 113),
                        (75, 113),
                        (75, 114),
                        (73, 115),
                        (75, 115),
                        (71, 116),
                        (72, 116),
                    ] {
                        skin.push((x + frame * 296, y, 9));
                    }
                    for (x, y) in [(210, 119), (208, 120), (211, 120), (209, 121)] {
                        protected.push((x + frame * 296, y, 0x7A3118));
                    }
                }
                protected.push((216, 109, 0x241730));
            }
            for (x, y, shade) in skin {
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
            for (x, y, color) in protected {
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
                } else {
                    assert_eq!(pixel, new, "unrelated color {name} [{x},{y}]");
                }
                selection.push(pixel != new);
            }
        }
        assert_eq!(skin_landmarks, 80);
        assert_eq!(protected_landmarks, 47);
        assert!(changed.iter().all(|n| *n > 0));
        assert!(SOURCE.iter().zip(target).all(|(a, b)| a != b));
        if target_index == 0 {
            blue_selection = selection;
        } else {
            assert_eq!(selection, blue_selection, "preset mask differs");
        }
    }
}
