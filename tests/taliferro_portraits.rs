use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 36 local portraits in extracted/taliferro-portraits-study"]
fn taliferro_eye_and_mouth_blends_recolor_without_changing_hair_or_jewelry() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/taliferro-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let recipe = std::env::var_os("FOM_TALIFERRO_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/taliferro-portraits.json"));
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
    let source = [
        0xFFDBB8FFu32,
        0xF2A174FF,
        0xC4613BFF,
        0x762E21FF,
        0xFCC49AFF,
        0xDE7E48FF,
        0xEFA67AFF,
        0xFCDEBEFF,
    ]
    .map(u32::to_be_bytes);
    let target = [
        0x9DB9D4FFu32,
        0x7F9FBDFF,
        0x6687ADFF,
        0x445F83FF,
        0x91AFCBFF,
        0x7393B5FF,
        0x7CA4C3FF,
        0x9ABCDAFF,
    ]
    .map(u32::to_be_bytes);
    let mut changed = [0; 8];
    let mut skin_count = 0;
    let mut protected_count = 0;
    for row in rows {
        let name = row["path"].as_str().unwrap();
        let before = image::open(original.join(name)).unwrap().to_rgba8();
        let after = image::open(modified.join(name)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (592, 180));
        assert_eq!(after.dimensions(), before.dimensions());
        let metadata = name.replace(".png", ".meta.toml");
        let bytes = fs::read(original.join(&metadata)).unwrap();
        assert_eq!(bytes, fs::read(modified.join(metadata)).unwrap());
        let parsed: toml::Value = toml::from_str(std::str::from_utf8(&bytes).unwrap()).unwrap();
        assert_eq!(
            parsed["asset_properties"]["atlas"].as_str(),
            Some("PortraitsMisc")
        );
        assert_eq!(
            parsed["asset_properties"]["frame_len"].as_integer(),
            Some(2)
        );
        // Literal source-art landmarks are independent of the profile's component seeds.
        if name.ends_with("_neutral.png") {
            for (x, y, i) in [
                (165, 52, 4),
                (150, 74, 0),
                (149, 73, 1),
                (150, 73, 2),
                (148, 72, 3),
                (149, 72, 5),
                (174, 96, 3),
                (130, 159, 3),
            ] {
                for f in 0..2 {
                    if f == 1 && (72..=74).contains(&y) && x < 170 {
                        continue;
                    }
                    let x = x + 296 * f;
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        source[i],
                        "source skin {name} [{x},{y}]"
                    );
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        target[i],
                        "missed skin blend {name} [{x},{y}]"
                    );
                    skin_count += 1;
                }
            }
            for (x, y, i) in [(148 + 296, 74, 6), (148 + 296, 71, 7)] {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    source[i],
                    "source lip {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    target[i],
                    "missed lip blend {name} [{x},{y}]"
                );
                skin_count += 1;
            }
            let mut protected = vec![
                (163, 33, 0x762E21FFu32),
                (137, 59, 0x762E21FF),
                (150, 56, 0x762E21FF),
                (147, 60, 0xF5F5F5FF),
            ];
            if name.contains("/Spring/") || name.contains("/Autumn/") {
                protected.extend([(150, 91, 0xF9C973FF), (149, 91, 0xB65932FF)]);
            }
            for (x, y, color) in protected {
                for f in 0..2 {
                    let x = x + 296 * f;
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        color.to_be_bytes(),
                        "source detail {name} [{x},{y}]"
                    );
                    assert_eq!(
                        after.get_pixel(x, y),
                        before.get_pixel(x, y),
                        "hair or jewelry spill {name} [{x},{y}]"
                    );
                    protected_count += 1;
                }
            }
            for (x, y, color) in [(151 + 296, 74, 0xDF4868FFu32), (150 + 296, 74, 0xF96F8CFF)] {
                assert_eq!(before.get_pixel(x, y).0, color.to_be_bytes());
                assert_eq!(
                    after.get_pixel(x, y),
                    before.get_pixel(x, y),
                    "tongue {name}"
                );
                protected_count += 1;
            }
        }
        for (x, y, p) in before.enumerate_pixels() {
            let q = after.get_pixel(x, y);
            assert_eq!(p[3], q[3], "alpha {name} [{x},{y}]");
            if let Some(i) = source.iter().position(|c| c == &p.0) {
                if p != q {
                    assert_eq!(q.0, target[i], "wrong mapped shade {name} [{x},{y}]");
                    changed[i] += 1;
                }
            } else {
                assert_eq!(p, q, "unrelated color {name} [{x},{y}]");
            }
        }
    }
    assert!(changed.iter().all(|&c| c > 0), "missing shade {changed:?}");
    assert_eq!((skin_count, protected_count), (52, 48));
}
