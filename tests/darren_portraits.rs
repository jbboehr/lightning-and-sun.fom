use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

const SOURCE: [u32; 7] = [
    0xA3614E, 0x83453A, 0x63342A, 0x491F1B, 0x955343, 0x87473B, 0x392222,
];
const TARGETS: [[u32; 7]; 4] = [
    [
        0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83, 0x7F9FBD, 0x7F9FBD, 0x445F83,
    ],
    [
        0xE8B271, 0xCA9052, 0xB27146, 0x6E4922, 0xCA9052, 0xCA9052, 0x6E4922,
    ],
    [
        0xB06C57, 0x814A3A, 0x63342A, 0x491F1B, 0x814A3A, 0x814A3A, 0x491F1B,
    ],
    [
        0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48, 0xA69084, 0xA69084, 0x624A48,
    ],
];
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}

#[test]
#[ignore = "requires the eight local strips in extracted/darren-portraits-study"]
fn darren_covers_fine_skin_and_hands_without_recoloring_beard_cuffs_or_buttons() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/darren-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let mut blue_selection = Vec::new();
    for (target_index, target) in TARGETS.iter().enumerate() {
        let map: serde_json::Map<String, Value> = SOURCE
            .iter()
            .zip(target)
            .map(|(source, target)| (format!("#{source:06X}"), json!(format!("#{target:06X}"))))
            .collect();
        let recipe = temp.path().join(format!("recipe-{target_index}.json"));
        fs::write(&recipe, serde_json::to_vec(&json!({"profile":root.join("palettes/profiles/darren-portraits.json"),"rgba_map":map})).unwrap()).unwrap();
        let recipe = if target_index == 0 {
            std::env::var_os("FOM_DARREN_RECIPE")
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
        assert_eq!(rows.len(), 8);
        let mut selection = Vec::new();
        let mut changed = [0; 7];
        let mut skin_landmarks = 0;
        let mut protected_landmarks = 0;
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
            // Literal locations were chosen from original art, independently of profile seeds.
            if name.ends_with("_neutral.png") {
                for (x, y, shade) in [
                    (158, 65, 0),
                    (158, 62, 1),
                    (158, 60, 2),
                    (166, 68, 3),
                    (154, 68, 4),
                    (147, 78, 5),
                    (126, 171, 6),
                    (120, 177, 1),
                    (200, 160, 0),
                    (422, 171, 6),
                    (416, 177, 1),
                    (496, 160, 0),
                    (197, 152, 6),
                    (198, 152, 6),
                    (199, 152, 6),
                    (493, 152, 6),
                    (494, 152, 6),
                    (495, 152, 6),
                ] {
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
                for (x, y, color) in [
                    (139, 160, 0xA3614E),
                    (138, 161, 0xA3614E),
                    (435, 160, 0xA3614E),
                    (146, 68, 0x6A4A42),
                    (159, 68, 0x4C2D2D),
                    (149, 69, 0x5F3F36),
                    (148, 51, 0x392222),
                    (143, 97, 0x392222),
                    (126, 170, 0xBE7C66),
                    (137, 81, 0xFCC5AA),
                    (144, 38, 0x161819),
                    (156, 57, 0xFFFFFF),
                    (158, 57, 0xBABABC),
                    (157, 59, 0xBF743F),
                    (154, 57, 0x6C6363),
                    (457, 73, 0xDF4868),
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
            for (x, y, pixel) in before.enumerate_pixels() {
                let new = after.get_pixel(x, y);
                assert_eq!(pixel[3], new[3], "alpha {name} [{x},{y}]");
                if let Some(shade) = SOURCE.iter().position(|c| rgba(*c) == pixel.0) {
                    if pixel != new {
                        assert_eq!(new.0, rgba(target[shade]), "wrong color {name} [{x},{y}]");
                        changed[shade] += 1;
                    }
                    // Ryis can legitimately equal a source shade. Blue records selection,
                    // so a retained identity shade cannot hide a different target's spill.
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
        assert_eq!(skin_landmarks, 18);
        assert_eq!(protected_landmarks, 16);
        if target_index == 0 {
            assert!(changed.iter().all(|n| *n > 0));
            blue_selection = selection;
        }
    }
}
