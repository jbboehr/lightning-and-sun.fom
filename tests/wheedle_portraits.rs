use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 32 local portraits in extracted/wheedle-portraits-study"]
fn wheedle_skin_blends_recolor_without_changing_beard_or_winter_gloves() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/wheedle-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let recipe = std::env::var_os("FOM_WHEEDLE_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/wheedle-portraits.json"));
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
        0xF6C38DFFu32,
        0xD39876FF,
        0xAF7456FF,
        0xA56C54FF,
        0x81503CFF,
        0x553728FF,
        0xD89D7AFF,
        0xE5AC7EFF,
        0xB57657FF,
        0x7C5846FF,
    ]
    .map(u32::to_be_bytes);
    let target = [
        0x9DB9D4FFu32,
        0x7F9FBDFF,
        0x6687ADFF,
        0x5C7FABFF,
        0x445F83FF,
        0x2D3F57FF,
        0x84A4C1FF,
        0x8EACC9FF,
        0x6C89AEFF,
        0x3F678DFF,
    ]
    .map(u32::to_be_bytes);
    let mut changed = [0; 10];
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
        let season = ["Spring", "Summer", "Autumn", "Winter"]
            .into_iter()
            .find(|s| name.contains(&format!("/{s}/")))
            .unwrap();
        assert_eq!(
            parsed["asset_properties"]["atlas"].as_str().unwrap(),
            format!("Portraits{season}")
        );
        assert_eq!(
            parsed["asset_properties"]["frame_len"].as_integer(),
            Some(2)
        );

        // These source-art coordinates were chosen from enlarged portraits,
        // independently of the profile's component seeds.
        if name.ends_with("_neutral.png") {
            for (x, y, i) in [
                (147, 71, 0),
                (143, 69, 1),
                (146, 63, 2),
                (146, 72, 3),
                (129, 61, 4),
                (137, 76, 5),
                (163, 54, 6),
                (139, 60, 7),
                (149, 60, 8),
                (144, 61, 9),
            ] {
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
            for (x, y, color) in [
                (133, 40, 0x9A6D58FFu32),
                (135, 50, 0x553E33FF),
                (147, 79, 0x795645FF),
                (
                    147,
                    81,
                    if season == "Autumn" || season == "Summer" {
                        0x553E33FF
                    } else {
                        0x604639FF
                    },
                ),
                (145, 60, 0xFFFFFFFF),
                (143, 59, 0xD8D8D8FF),
            ] {
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
                        "hair or eye spill {name} [{x},{y}]"
                    );
                    protected_count += 1;
                }
            }
        }
        for f in 0..2 {
            for (x, y, i) in [(184, 116, 0), (168, 119, 0), (185, 116, 1)] {
                let x = x + 296 * f;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    source[i],
                    "source wrist {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    target[i],
                    "missed wrist {name} [{x},{y}]"
                );
                skin_count += 1;
            }
            for (x, y) in [(184, 86), (187, 86), (183, 87), (175, 97), (175, 116)] {
                let x = x + 296 * f;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    source[5],
                    "source finger detail {name} [{x},{y}]"
                );
                if season == "Winter" {
                    assert_eq!(
                        after.get_pixel(x, y),
                        before.get_pixel(x, y),
                        "winter glove spill {name} [{x},{y}]"
                    );
                    protected_count += 1;
                } else {
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        target[5],
                        "missed bare finger shading {name} [{x},{y}]"
                    );
                    skin_count += 1;
                }
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
    assert_eq!((skin_count, protected_count), (472, 128));
}
