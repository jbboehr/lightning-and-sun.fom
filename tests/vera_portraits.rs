use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

const SOURCE: [u32; 6] = [0xB68454, 0xA15F3F, 0x7F402B, 0x541C1E, 0xB07342, 0x762E21];
const TARGETS: [[u32; 6]; 4] = [
    [0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83, 0x7F9FBD, 0x445F83],
    [0xE8B271, 0xCA9052, 0xB27146, 0x6E4922, 0xCA9052, 0x6E4922],
    [0xB06C57, 0x814A3A, 0x63342A, 0x491F1B, 0x814A3A, 0x491F1B],
    [0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48, 0xA69084, 0x624A48],
];
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}

#[test]
#[ignore = "requires the 32 local strips in extracted/vera-portraits-study"]
fn vera_covers_fine_skin_blends_and_hands_without_changing_makeup_or_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/vera-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let mut blue_selection = Vec::new();
    for (target_index, target) in TARGETS.iter().enumerate() {
        let map: serde_json::Map<String, Value> = SOURCE
            .iter()
            .zip(target)
            .map(|(source, target)| (format!("#{source:06X}"), json!(format!("#{target:06X}"))))
            .collect();
        let recipe = temp.path().join(format!("recipe-{target_index}.json"));
        fs::write(&recipe, serde_json::to_vec(&json!({"profile":root.join("palettes/profiles/vera-portraits.json"),"rgba_map":map})).unwrap()).unwrap();
        let recipe = if target_index == 0 {
            std::env::var_os("FOM_VERA_RECIPE")
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
        let mut changed = [0; 6];
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
            // Literal coordinates were chosen from the original art, not profile seeds.
            if name.ends_with("_neutral.png") {
                for frame in 0..2 {
                    for &(x, y, shade) in &[
                        (144, 59, 0),
                        (144, 58, 1),
                        (149, 58, 2),
                        (140, 67, 3),
                        (143, 68, 4),
                    ] {
                        let x = x + frame * 296;
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
                }
                // One brown transition pixel sits outside the speaking lipstick.
                let (x, y, shade) = (443, 86, 5);
                assert_eq!(
                    before.get_pixel(x, y).0,
                    rgba(SOURCE[shade]),
                    "source mouth {name}"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    rgba(target[shade]),
                    "missed mouth transition {name} [{x},{y}]"
                );
                skin_landmarks += 1;
                for &(x, y, color) in &[
                    (153, 41, 0x581B46),
                    (133, 83, 0xE9AF68),
                    (141, 75, 0x5237A9),
                    (142, 73, 0xF5F5F5),
                    (148, 84, 0xAB4561),
                    (149, 85, 0x731A45),
                    (444, 84, 0x581D1F),
                    (444, 85, 0x941A33),
                    (445, 86, 0xCA4D67),
                ] {
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
            }
            let body: &[(u32, u32, usize)] = if name.ends_with("summer_neutral.png") {
                &[
                    (120, 112, 0),
                    (121, 131, 0),
                    (130, 143, 0),
                    (179, 130, 0),
                    (180, 144, 0),
                    (171, 156, 0),
                    (172, 155, 1),
                    (145, 107, 0),
                ]
            } else if name.ends_with("winter_neutral.png") {
                &[(126, 108, 0)]
            } else {
                &[]
            };
            for &(x, y, shade) in body {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    rgba(SOURCE[shade]),
                    "source body {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    rgba(target[shade]),
                    "missed body {name} [{x},{y}]"
                );
                skin_landmarks += 1;
            }
            let protected: &[(u32, u32, u32)] = if name.ends_with("summer_neutral.png") {
                &[(166, 158, 0x581B46), (142, 90, 0x8F295A)]
            } else if name.ends_with("winter_neutral.png") {
                &[(129, 118, 0xF3FCFF), (158, 129, 0x7F5443)]
            } else if name.ends_with("spring_ugh.png") {
                &[(140, 79, 0x657B9F)]
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
                } else {
                    assert_eq!(pixel, new, "unrelated color {name} [{x},{y}]");
                }
                selection.push(pixel != new);
            }
        }
        assert_eq!(skin_landmarks, 53);
        assert_eq!(protected_landmarks, 41);
        assert!(changed.iter().all(|n| *n > 0));
        assert!(SOURCE.iter().zip(target).all(|(a, b)| a != b));
        if target_index == 0 {
            blue_selection = selection;
        } else {
            assert_eq!(selection, blue_selection, "preset mask differs");
        }
    }
}
