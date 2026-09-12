use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

const SOURCE: [u32; 16] = [
    0xC7875C, 0xA36049, 0x6D3328, 0x4A1B16, 0xB16A52, 0x8F5042, 0x472721, 0x6A3126, 0xB9755A,
    0x884636, 0x6C3327, 0xC87463, 0xBB6C5D, 0x5B241E, 0x9A5547, 0x9F5544,
];
const TARGETS: [[u32; 16]; 4] = [
    [
        0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83, 0x7F9FBD, 0x6687AD, 0x445F83, 0x445F83, 0x7F9FBD,
        0x6687AD, 0x445F83, 0x7F9FBD, 0x6687AD, 0x445F83, 0x6687AD, 0x6687AD,
    ],
    [
        0xE8B271, 0xCA9052, 0xB27146, 0x6E4922, 0xCA9052, 0xB27146, 0x6E4922, 0x6E4922, 0xCA9052,
        0xB27146, 0x6E4922, 0xCA9052, 0xB27146, 0x6E4922, 0xB27146, 0xB27146,
    ],
    [
        0xB06C57, 0x814A3A, 0x63342A, 0x491F1B, 0x814A3A, 0x63342A, 0x491F1B, 0x491F1B, 0x814A3A,
        0x63342A, 0x491F1B, 0x814A3A, 0x63342A, 0x491F1B, 0x63342A, 0x63342A,
    ],
    [
        0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48, 0xA69084, 0x8E746D, 0x624A48, 0x624A48, 0xA69084,
        0x8E746D, 0x624A48, 0xA69084, 0x8E746D, 0x624A48, 0x8E746D, 0x8E746D,
    ],
];
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}

#[test]
#[ignore = "requires the eight local strips in extracted/linnet-portraits-study"]
fn linnet_covers_fine_face_and_neck_shades_without_recoloring_scar_circlet_or_armor() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/linnet-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let mut blue_selection = Vec::new();
    for (target_index, target) in TARGETS.iter().enumerate() {
        let map: serde_json::Map<String, Value> = SOURCE
            .iter()
            .zip(target)
            .map(|(source, target)| (format!("#{source:06X}"), json!(format!("#{target:06X}"))))
            .collect();
        let recipe = temp.path().join(format!("recipe-{target_index}.json"));
        fs::write(&recipe, serde_json::to_vec(&json!({"profile":root.join("palettes/profiles/linnet-portraits.json"),"rgba_map":map})).unwrap()).unwrap();
        let recipe = if target_index == 0 {
            std::env::var_os("FOM_LINNET_RECIPE")
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
        let mut changed = [0; 16];
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
                    (151, 65, 0),
                    (153, 98, 1),
                    (164, 58, 2),
                    (150, 54, 3),
                    (147, 48, 4),
                    (159, 48, 5),
                    (150, 50, 6),
                    (150, 51, 7),
                    (154, 56, 8),
                    (152, 69, 9),
                    (133, 59, 10),
                    (151, 70, 11),
                    (152, 68, 12),
                    (144, 53, 13),
                    (133, 63, 14),
                    (164, 67, 15),
                    (151, 92, 0),
                    (449, 98, 1),
                    (447, 92, 0),
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
                    (148, 44, 0x472721),
                    (153, 44, 0x8F5042),
                    (155, 44, 0x6A3126),
                    (148, 46, 0xC9785A),
                    (147, 47, 0x472721),
                    (149, 46, 0xDC9648),
                    (133, 38, 0xE6E1FF),
                    (146, 58, 0xA476BA),
                    (150, 56, 0x6C6363),
                    (148, 56, 0xBABABC),
                    (155, 57, 0xEF977D),
                    (156, 58, 0xFFA793),
                    (160, 61, 0xD98868),
                    (121, 126, 0xC9785A),
                    (147, 164, 0x6D3328),
                    (168, 164, 0xA36049),
                    (444, 44, 0x472721),
                    (417, 126, 0xC9785A),
                    (447, 69, 0xF96F8C),
                    (449, 69, 0xCE4865),
                    (450, 70, 0xA1121D),
                    (451, 70, 0x762E21),
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
        assert_eq!(skin_landmarks, 19);
        assert_eq!(protected_landmarks, 22);
        if target_index == 0 {
            assert!(changed.iter().all(|n| *n > 0));
            blue_selection = selection;
        }
    }
}
