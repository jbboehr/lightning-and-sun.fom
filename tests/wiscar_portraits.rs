use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the eight portraits in extracted/wiscar-portraits-study"]
fn wiscar_skin_and_eye_blends_recolor_without_changing_gold_or_beard() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/wiscar-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let recipe = std::env::var_os("FOM_WISCAR_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/wiscar-portraits.json"));
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
    let source = [
        0xE3A17BFFu32,
        0xD98364FF,
        0xC47054FF,
        0x9F5544FF,
        0x6D3328FF,
        0x834132FF,
        0x441B14FF,
        0xF4C2A7FF,
        0xDDAB82FF,
    ]
    .map(u32::to_be_bytes);
    let target = [
        0x9DB9D4FFu32,
        0x7F9FBDFF,
        0x6687ADFF,
        0x445F83FF,
        0x2D3F57FF,
        0x364C69FF,
        0x1B2634FF,
        0xB6CBDFFF,
        0x97C3DBFF,
    ]
    .map(u32::to_be_bytes);
    let mut changed = [0; 9];
    let mut landmarks = 0;
    for row in rows {
        let name = row["path"].as_str().unwrap();
        let before = image::open(original.join(name)).unwrap().to_rgba8();
        let after = image::open(modified.join(name)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (592, 180));
        assert_eq!(after.dimensions(), before.dimensions());
        let meta = name.replace(".png", ".meta.toml");
        let bytes = fs::read(original.join(&meta)).unwrap();
        assert_eq!(bytes, fs::read(modified.join(meta)).unwrap());
        let meta: toml::Value = toml::from_str(std::str::from_utf8(&bytes).unwrap()).unwrap();
        assert_eq!(
            meta["asset_properties"]["atlas"].as_str(),
            Some("PortraitsMisc")
        );
        assert_eq!(meta["asset_properties"]["frame_len"].as_integer(), Some(2));
        // Coordinates and expected source colors were picked from enlarged source art,
        // independently of component seeds and the target recipe.
        let mut check = |x, y, from: [u8; 4], to: [u8; 4], label| {
            assert_eq!(
                before.get_pixel(x, y).0,
                from,
                "source {label}: {name} [{x},{y}]"
            );
            assert_eq!(after.get_pixel(x, y).0, to, "{label}: {name} [{x},{y}]");
            landmarks += 1;
        };
        if name.ends_with("_neutral.png") {
            for (x, y, i) in [
                (152, 74, 0),
                (149, 75, 1),
                (150, 80, 2),
                (150, 79, 3),
                (151, 80, 4),
                (149, 63, 5),
                (164, 75, 6),
                (149, 68, 7),
            ] {
                check(x, y, source[i], target[i], "missed skin blend");
            }
        }
        if name.ends_with("_happy.png") || name.ends_with("_wink.png") {
            for f in 0..2 {
                check(
                    142 + 296 * f,
                    67,
                    source[8],
                    target[8],
                    "missed closed-eye blend",
                );
            }
        }
        for f in 0..2 {
            for (x, y, i) in [
                (140, 90, 2),
                (146, 96, 4),
                (182, 175, 0),
                (181, 173, 2),
                (180, 176, 3),
            ] {
                check(x + 296 * f, y, source[i], target[i], "missed neck or hand");
            }
            for (x, y, c) in [
                (154, 52, 0x6D3328FFu32),
                (155, 54, 0x6D3328FF),
                (154, 53, 0xF0BC70FF),
                (112, 97, 0x9F5544FF),
                (152, 86, 0xC57286FF),
                (134, 83, 0x884636FF),
            ] {
                check(
                    x + 296 * f,
                    y,
                    c.to_be_bytes(),
                    c.to_be_bytes(),
                    "gold or beard spill",
                );
            }
        }
        for (x, y, p) in before.enumerate_pixels() {
            let q = after.get_pixel(x, y);
            assert_eq!(p[3], q[3], "alpha {name} [{x},{y}]");
            if let Some(i) = source.iter().position(|c| c == &p.0) {
                if p != q {
                    assert_eq!(q.0, target[i], "mapped shade {name} [{x},{y}]");
                    changed[i] += 1;
                }
            } else {
                assert_eq!(p, q, "unrelated detail {name} [{x},{y}]");
            }
        }
    }
    assert!(changed.iter().all(|&n| n > 0), "missing shade {changed:?}");
    assert_eq!(landmarks, 188);
}
