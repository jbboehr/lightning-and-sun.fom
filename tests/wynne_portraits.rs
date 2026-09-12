use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the eight portraits in extracted/wynne-portraits-study"]
fn wynne_palms_and_eye_blends_recolor_without_changing_cosmetics_or_jewelry() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/wynne-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let recipe = std::env::var_os("FOM_WYNNE_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/wynne-portraits.json"));
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
        0xA3614EFFu32,
        0x83453AFF,
        0x63342AFF,
        0x491F1BFF,
        0x955343FF,
        0xAD6C59FF,
        0xB67565FF,
        0xC88A76FF,
        0x612924FF,
        0x804136FF,
    ]
    .map(u32::to_be_bytes);
    let target = [
        0x9DB9D4FFu32,
        0x7F9FBDFF,
        0x6687ADFF,
        0x445F83FF,
        0x91ADC6FF,
        0xA7C4DFFF,
        0xB0CDEBFF,
        0xC2E2FCFF,
        0x647CA7FF,
        0x7C9BB9FF,
    ]
    .map(u32::to_be_bytes);
    let mut changed = [0; 10];
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
        // Literal source-art landmarks are independent of component seeds.
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
                (150, 76, 0),
                (146, 80, 1),
                (153, 90, 2),
                (147, 82, 3),
                (155, 52, 4),
            ] {
                check(x, y, source[i], target[i], "missed skin shade");
            }
            for f in 0..2 {
                for (x, y, c) in [
                    (158, 79, if f == 0 { 0x925051FFu32 } else { 0x613954FF }),
                    (160, if f == 0 { 79 } else { 78 }, 0x613954FF),
                    (148, 67, 0x69598CFF),
                ] {
                    check(
                        x + 296 * f,
                        y,
                        c.to_be_bytes(),
                        c.to_be_bytes(),
                        "cosmetic spill",
                    );
                }
            }
        }
        if name.ends_with("_think.png") {
            for f in 0..2 {
                for (x, y, i) in [(159, 67, 8), (159, 69, 9)] {
                    check(
                        x + 296 * f,
                        y,
                        source[i],
                        target[i],
                        "missed think-eye blend",
                    );
                }
            }
        }
        for f in 0..2 {
            for (x, y, i) in [(201, 106, 5), (211, 101, 6), (215, 97, 7)] {
                check(x + 296 * f, y, source[i], target[i], "missed lighter palm");
            }
            for (x, y, c) in [
                (132, 48, 0x312623FFu32),
                (135, 77, 0xFBCB89FF),
                (168, 86, 0xE3A452FF),
            ] {
                check(
                    x + 296 * f,
                    y,
                    c.to_be_bytes(),
                    c.to_be_bytes(),
                    "hair or gold spill",
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
    assert_eq!(landmarks, 111);
}
