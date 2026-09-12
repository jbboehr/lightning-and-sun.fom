use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 32 local portraits in extracted/terithia-portraits-study"]
fn terithia_skin_and_scars_recolor_without_changing_cord_bag_or_tongue() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/terithia-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let recipe = std::env::var_os("FOM_TERITHIA_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/terithia-portraits.json"));
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
        0xD29A7DFFu32,
        0xBA7E66FF,
        0xAD7465FF,
        0x7E4C3BFF,
        0x724A3EFF,
        0x583131FF,
        0x482323FF,
        0xDBA191FF,
        0xE0A797FF,
        0xE8B1A2FF,
        0x7A4D45FF,
    ]
    .map(u32::to_be_bytes);
    let target = [
        0x9DB9D4FFu32,
        0x7F9FBDFF,
        0x6687ADFF,
        0x445F83FF,
        0x445F83FF,
        0x445F83FF,
        0x445F83FF,
        0xA6C0E8FF,
        0xABC6EEFF,
        0xB3D0F9FF,
        0x445F83FF,
    ]
    .map(u32::to_be_bytes);
    let mut changed = [0; 11];
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
            parsed["asset_properties"]["atlas"].as_str(),
            Some(format!("Portraits{season}").as_str())
        );
        assert_eq!(
            parsed["asset_properties"]["frame_len"].as_integer(),
            Some(2)
        );
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
        // These coordinates come from source-art inspection, independent of the profile seeds.
        if name.ends_with("_neutral.png") {
            let mut skin = vec![
                (153, 68, 4),
                (137, 78, 5),
                (146, 65, 6),
                (149, 55, 2),
                (149, 56, 0),
                (156, 56, 1),
                (163, 68, 3),
                (144, 78, 7),
                (145, 79, 9),
                (115, 150, 8),
                (140, 172, 6),
                (185, 158, 6),
                (150, 92, 5),
                (151, 93, 3),
                (154, 93, 5),
                (154, 94, 5),
            ];
            if season != "Winter" {
                skin.push((148, 91, 3));
            }
            if season == "Spring" {
                skin.extend([(141, 104, 3), (144, 105, 3), (149, 106, 3)]);
            }
            if season == "Autumn" {
                skin.push((140, 96, 10));
            }
            if season == "Summer" {
                skin.extend([(166, 128, 2), (167, 130, 0), (168, 129, 2)]);
            }
            for (x, y, i) in skin {
                for f in 0..2 {
                    let x = x + 296 * f;
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        source[i],
                        "source skin {name} [{x},{y}]"
                    );
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        target[i],
                        "missed skin/scar {name} [{x},{y}]"
                    );
                    skin_count += 1;
                }
            }
            let mut protected = vec![
                (150, 99, 0x7E4C3BFFu32),
                (153, 99, 0x583131FF),
                (125, 175, 0x452424FF),
                (146, 66, 0x000000FF),
                (150, 71, 0xC3B5B5FF),
                (112, 168, 0x583131FF),
            ];
            if season == "Autumn" {
                protected.push((124, 154, 0x7A4D45FF));
            }
            if season == "Summer" {
                protected.extend([(164, 126, 0x583131FF), (164, 127, 0x583131FF)]);
            }
            if season == "Spring" {
                protected.push((128, 154, 0x583131FF));
            }
            for (x, y, color) in protected {
                for f in 0..2 {
                    let x = x + 296 * f;
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        color.to_be_bytes(),
                        "source accessory {name} [{x},{y}]"
                    );
                    assert_eq!(
                        after.get_pixel(x, y),
                        before.get_pixel(x, y),
                        "accessory spill {name} [{x},{y}]"
                    );
                    protected_count += 1;
                }
            }
            for (x, y, color) in [(155 + 296, 84, 0xDF4868FFu32), (156 + 296, 84, 0xF96F8CFF)] {
                assert_eq!(before.get_pixel(x, y).0, color.to_be_bytes());
                assert_eq!(
                    after.get_pixel(x, y),
                    before.get_pixel(x, y),
                    "tongue {name}"
                );
                protected_count += 1;
            }
        }
    }
    assert!(changed.iter().all(|&c| c > 0), "missing shade {changed:?}");
    assert_eq!((skin_count, protected_count), (148, 64));
}
