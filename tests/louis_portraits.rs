use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 32 local portraits in extracted/louis-portraits-study"]
fn louis_fine_skin_shades_recolor_without_changing_hair_glasses_or_clothes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/louis-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let recipe = std::env::var_os("FOM_LOUIS_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/louis-portraits.json"));
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
    let source = [
        0xF6D2AFFFu32,
        0xCB9879FF,
        0xAD6F5CFF,
        0x784B42FF,
        0xAB6F5CFF,
        0xAC6F5CFF,
        0xC99679FF,
        0xCA9779FF,
        0xCA9879FF,
        0xF4D0AEFF,
        0xE3B590FF,
        0xE3BD97FF,
        0xF1CEADFF,
        0x634040FF,
        0x40252AFF,
        0x491F1BFF,
    ]
    .map(u32::to_be_bytes);
    let light = [157, 185, 212, 255];
    let medium = [127, 159, 189, 255];
    let shadow = [102, 135, 173, 255];
    let dark = [68, 95, 131, 255];
    let target = [
        light, medium, shadow, dark, shadow, shadow, medium, medium, medium, light, medium, medium,
        light, dark, dark, dark,
    ];
    let mut changed = [0; 16];
    let mut skin_count = 0;
    let mut protected_count = 0;
    for row in rows {
        let name = row["path"].as_str().unwrap();
        let before = image::open(original.join(name)).unwrap().to_rgba8();
        let after = image::open(modified.join(name)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (592, 180));
        assert_eq!(after.dimensions(), before.dimensions());
        let metadata = name.replace(".png", ".meta.toml");
        let source_metadata = fs::read(original.join(&metadata)).unwrap();
        assert_eq!(source_metadata, fs::read(modified.join(&metadata)).unwrap());
        let parsed: toml::Value =
            toml::from_str(std::str::from_utf8(&source_metadata).unwrap()).unwrap();
        assert_eq!(
            parsed["asset_properties"]["atlas"].as_str(),
            Some("PortraitsMisc")
        );
        assert_eq!(
            parsed["asset_properties"]["frame_len"].as_integer(),
            Some(2)
        );
        for (x, y, pixel) in before.enumerate_pixels() {
            let new = after.get_pixel(x, y);
            assert_eq!(pixel[3], new[3], "alpha: {name} [{x},{y}]");
            if let Some(shade) = source.iter().position(|c| c == &pixel.0) {
                if pixel != new {
                    assert_eq!(new.0, target[shade], "wrong color: {name} [{x},{y}]");
                    changed[shade] += 1;
                }
            } else {
                assert_eq!(pixel, new, "changed unrelated color: {name} [{x},{y}]");
            }
        }
        // Literal coordinates come from the source sprites, independently of mask seeds.
        let skin: &[(u32, u32, usize)] = if name.ends_with("spring_neutral.png") {
            &[
                (155, 44, 0),
                (158, 43, 1),
                (159, 43, 2),
                (154, 45, 3),
                (147, 51, 4),
                (150, 50, 5),
                (152, 50, 6),
                (148, 51, 7),
                (151, 50, 8),
                (151, 51, 9),
                (154, 60, 10),
                (158, 83, 11),
                (139, 86, 12),
                (152, 74, 13),
                (157, 73, 14),
                (145, 59, 15),
                (148, 89, 2),
                (149, 90, 2),
                (150, 91, 2),
                (149, 89, 3),
                (153, 90, 3),
                (157, 84, 11),
                (145, 84, 11),
                (132, 156, 0),
                (120, 171, 1),
            ]
        } else if name.ends_with("autumn_neutral.png") {
            &[(145, 87, 1), (147, 90, 1)]
        } else if name.ends_with("summer_neutral.png") {
            &[
                (143, 86, 1),
                (148, 89, 1),
                (149, 89, 2),
                (150, 89, 3),
                (153, 91, 3),
                (145, 89, 3),
            ]
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
                    target[shade],
                    "missed skin: {name} [{x},{y}]"
                );
                skin_count += 1;
            }
        }
        let protected: &[(u32, u32, u32)] = if name.ends_with("spring_neutral.png") {
            &[
                (157, 38, 0x634040FF),
                (156, 38, 0x40252AFF),
                (165, 42, 0x523232FF),
                (146, 59, 0x000000FF),
                (148, 65, 0x6E380CFF),
                (148, 67, 0x4C7DC5FF),
                (147, 90, 0xCB9879FF),
                (135, 93, 0xCB9879FF),
                (154, 92, 0xCB9879FF),
                (150, 94, 0x784B42FF),
                (152, 99, 0x784B42FF),
                (155, 92, 0x784B42FF),
            ]
        } else if name.ends_with("summer_neutral.png") {
            &[
                (125, 163, 0x40252AFF),
                (121, 166, 0x40252AFF),
                (110, 132, 0xCB9879FF),
                (122, 137, 0xAD6F5CFF),
                (155, 90, 0xAD6F5CFF),
                (156, 88, 0xCB9879FF),
                (144, 90, 0xE3BD97FF),
                (178, 62, 0xCB9879FF),
                (176, 65, 0xCB9879FF),
                (185, 75, 0xCB9879FF),
            ]
        } else {
            &[]
        };
        for &(x, y, color) in protected {
            for frame in 0..2 {
                let x = x + frame * 296;
                let color = color.to_be_bytes();
                assert_eq!(
                    before.get_pixel(x, y).0,
                    color,
                    "source protected: {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    color,
                    "changed hair/glasses/clothes: {name} [{x},{y}]"
                );
                protected_count += 1;
            }
        }
        if name.ends_with("spring_neutral.png") {
            for (x, y, shade) in [
                (151, 79, 0),
                (153, 80, 1),
                (150, 78, 2),
                (151, 78, 3),
                (448, 80, 13),
                (447, 79, 13),
            ] {
                assert_eq!(before.get_pixel(x, y).0, source[shade]);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    target[shade],
                    "missed mouth skin [{x},{y}]"
                );
                skin_count += 1;
            }
            for (x, y, color) in [
                (447, 77, 0x491F1BFFu32),
                (446, 78, 0x634040FF),
                (450, 78, 0xA1121DFF),
                (447, 78, 0xDF4868FF),
                (448, 79, 0xF96F8CFF),
            ] {
                assert_eq!(before.get_pixel(x, y).0, color.to_be_bytes());
                assert_eq!(
                    after.get_pixel(x, y).0,
                    color.to_be_bytes(),
                    "changed mouth interior [{x},{y}]"
                );
                protected_count += 1;
            }
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(skin_count, 72);
    assert_eq!(protected_count, 49);
}
