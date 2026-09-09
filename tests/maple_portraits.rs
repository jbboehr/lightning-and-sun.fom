use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 32 local portraits in extracted/maple-portraits-study"]
fn maple_fine_skin_recolors_without_changing_irises_hair_or_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/maple-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let recipe = std::env::var_os("FOM_MAPLE_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/maple-portraits.json"));
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
        0x97623EFFu32,
        0x89492AFF,
        0x713527FF,
        0x57271DFF,
        0x83462EFF,
        0x5D3228FF,
        0xCE794EFF,
        0x89502AFF,
        0x392222FF,
        0xBF5C70FF,
    ]
    .map(u32::to_be_bytes);
    let light = [157, 185, 212, 255];
    let middle = [127, 159, 189, 255];
    let shadow = [102, 135, 173, 255];
    let dark = [68, 95, 131, 255];
    let target = [
        light,
        middle,
        shadow,
        dark,
        middle,
        dark,
        light,
        middle,
        dark,
        [148, 127, 167, 255],
    ];
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
        assert_eq!(bytes, fs::read(modified.join(&metadata)).unwrap());
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
        for (x, y, pixel) in before.enumerate_pixels() {
            let new = after.get_pixel(x, y);
            assert_eq!(pixel[3], new[3], "alpha {name} [{x},{y}]");
            if let Some(shade) = source.iter().position(|c| c == &pixel.0) {
                if pixel != new {
                    assert_eq!(new.0, target[shade], "wrong color {name} [{x},{y}]");
                    changed[shade] += 1;
                }
            } else {
                assert_eq!(pixel, new, "unrelated color {name} [{x},{y}]");
            }
        }
        // Literal source-art coordinates are independent of the stored component seeds.
        if name.ends_with("_neutral.png") {
            for (x, y, shade) in [
                (151, 100, 0),
                (152, 97, 1),
                (149, 88, 2),
                (163, 81, 3),
                (158, 75, 4),
                (144, 86, 5),
                (203, 169, 6),
                (146, 109, 8),
                (156, 100, 8),
                (140, 103, 8),
                (143, 107, 8),
            ] {
                for frame in 0..2 {
                    let x = x + frame * 296;
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        source[shade],
                        "source skin {name} [{x},{y}]"
                    );
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        target[shade],
                        "missed skin {name} [{x},{y}]"
                    );
                    skin_count += 1;
                }
            }
            for (x, y, color) in [
                (145, 86, 0x392222FFu32),
                (143, 95, 0xCE794EFF),
                (143, 94, 0x83462EFF),
                (148, 91, 0x6C6363FF),
                (144, 92, 0xFAF9F9FF),
            ] {
                for frame in 0..2 {
                    let x = x + frame * 296;
                    let color = color.to_be_bytes();
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        color,
                        "source protected {name} [{x},{y}]"
                    );
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        color,
                        "changed eye/hair {name} [{x},{y}]"
                    );
                    protected_count += 1;
                }
            }
            for (x, y, shade) in [(448, 104, 3), (449, 106, 3), (451, 103, 4), (448, 105, 9)] {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    source[shade],
                    "source outer lip {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    target[shade],
                    "outer lip {name} [{x},{y}]"
                );
                skin_count += 1;
            }
            for (x, y, color) in [
                (448, 101, 0x57271DFFu32),
                (447, 102, 0x57271DFF),
                (450, 102, 0x57271DFF),
                (448, 102, 0xDF4868FF),
                (449, 102, 0xB52745FF),
                (448, 103, 0xC33A57FF),
            ] {
                assert_eq!(
                    before.get_pixel(x, y).0,
                    color.to_be_bytes(),
                    "source mouth {name} [{x},{y}]"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    color.to_be_bytes(),
                    "changed mouth interior {name} [{x},{y}]"
                );
                protected_count += 1;
            }
        }
        if name.ends_with("summer_neutral.png") {
            for (x, y, color) in [
                (141, 114, 0x97623EFFu32),
                (125, 117, 0x57271DFF),
                (153, 120, 0x97623EFF),
            ] {
                for frame in 0..2 {
                    let x = x + frame * 296;
                    let color = color.to_be_bytes();
                    assert_eq!(before.get_pixel(x, y).0, color, "source clothes [{x},{y}]");
                    assert_eq!(after.get_pixel(x, y).0, color, "changed clothing [{x},{y}]");
                    protected_count += 1;
                }
            }
        }
        if name.ends_with("_happy.png") {
            for (x, y) in [(453, 100), (450, 101)] {
                assert_eq!(before.get_pixel(x, y).0, source[7]);
                assert_eq!(
                    after.get_pixel(x, y).0,
                    target[7],
                    "nose shade {name} [{x},{y}]"
                );
                skin_count += 1;
            }
        }
    }
    assert!(changed.iter().all(|&count| count > 0));
    assert_eq!(skin_count, 112);
    assert_eq!(protected_count, 70);
}

#[test]
#[ignore = "requires the 32 local portraits in extracted/maple-portraits-study"]
fn maple_lip_shade_tracks_each_preset_without_recoloring_the_tongue() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/maple-portraits-study");
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("presets");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["build-presets", "--original"])
        .arg(&original)
        .arg("--presets")
        .arg(root.join("palettes/sets/maple-portraits-trial.json"))
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["presets"].as_array().unwrap().len(), 4);
    for (id, lip) in [
        ("blue", [148, 127, 167, 255]),
        ("npc_hayden", [188, 120, 96, 255]),
        ("npc_ryis", [151, 83, 91, 255]),
        ("npc_seridia", [161, 123, 136, 255]),
    ] {
        let preset = report["presets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["id"] == id)
            .unwrap();
        let rows = preset["report"]["files"].as_array().unwrap();
        assert_eq!(rows.len(), 32);
        let mut lips = 0;
        let mut interior = 0;
        for row in rows {
            let name = row["path"].as_str().unwrap();
            let before = image::open(original.join(name)).unwrap().to_rgba8();
            let after = image::open(output.join("variants").join(id).join(name))
                .unwrap()
                .to_rgba8();
            assert_eq!(before.dimensions(), after.dimensions());
            let metadata = name.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&metadata)).unwrap(),
                fs::read(output.join("variants").join(id).join(&metadata)).unwrap()
            );
            for (x, y, pixel) in before.enumerate_pixels() {
                assert_eq!(pixel[3], after.get_pixel(x, y)[3]);
                if pixel.0 == [191, 92, 112, 255] {
                    assert!(
                        (152..=154).contains(&(x % 296)) && (104..=107).contains(&y),
                        "unexpected lip location {name} [{x},{y}]"
                    );
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        lip,
                        "fixed pink lip {id} {name} [{x},{y}]"
                    );
                    lips += 1;
                }
                if [[223, 72, 104, 255], [181, 39, 69, 255], [195, 58, 87, 255]].contains(&pixel.0)
                {
                    assert_eq!(
                        pixel,
                        after.get_pixel(x, y),
                        "changed tongue/interior {id} {name} [{x},{y}]"
                    );
                    interior += 1;
                }
            }
            if name.ends_with("_neutral.png") {
                for (x, y) in [(448, 101), (447, 102), (450, 102)] {
                    assert_eq!(before.get_pixel(x, y).0, [87, 39, 29, 255]);
                    assert_eq!(
                        before.get_pixel(x, y),
                        after.get_pixel(x, y),
                        "changed upper interior {id} {name} [{x},{y}]"
                    );
                }
            }
        }
        assert_eq!(lips, 160);
        assert_eq!(interior, 112);
    }
}
