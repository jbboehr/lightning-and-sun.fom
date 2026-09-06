use image::RgbaImage;
use serde_json::Value;
use std::{fs, process::Command};

const SOURCE: &str = "spr_portrait_adeline_spring_neutral";
const VARIANT: &str = "spr_lns_adeline_spring_neutral_blue";

#[test]
fn toggle_package_adds_a_variant_without_replacing_vanilla() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    let output = temp.path().join("package");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();
    let before = RgbaImage::from_raw(
        4,
        1,
        vec![
            10, 20, 30, 255, 40, 50, 60, 128, 70, 80, 90, 0, 10, 20, 30, 255,
        ],
    )
    .unwrap();
    let after = RgbaImage::from_raw(
        4,
        1,
        vec![
            90, 80, 70, 255, 40, 50, 60, 128, 70, 80, 90, 0, 90, 80, 70, 255,
        ],
    )
    .unwrap();
    before.save(original.join(format!("{SOURCE}.png"))).unwrap();
    after.save(modified.join(format!("{SOURCE}.png"))).unwrap();
    let metadata = "[meta_properties]\nid = '0123456789abcdef'\nasset_kind = 'Animation'\n[asset_properties]\nframe_size = [2,1]\nframe_len = 2\nduration = 0.2\natlas = 'PortraitsSpring'\noffset.horizontal = 'Middle'\noffset.vertical = 1\n";
    for root in [&original, &modified] {
        fs::write(root.join(format!("{SOURCE}.meta.toml")), metadata).unwrap();
    }
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("package-toggle")
        .arg("--original")
        .arg(&original)
        .arg("--modified")
        .arg(&modified)
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
    assert_eq!(report["changed_pixels"], 2);
    let variant = format!("animations/LightningAndSun/{VARIANT}");
    assert_eq!(
        image::open(output.join(format!("{variant}.png")))
            .unwrap()
            .to_rgba8(),
        after
    );
    let meta: toml::Value =
        toml::from_str(&fs::read_to_string(output.join(format!("{variant}.meta.toml"))).unwrap())
            .unwrap();
    let source_meta: toml::Value = toml::from_str(metadata).unwrap();
    assert_eq!(meta["asset_properties"], source_meta["asset_properties"]);
    assert_eq!(
        meta["meta_properties"]["asset_kind"].as_str(),
        Some("Animation")
    );
    assert!(meta["meta_properties"].get("id").is_none());
    assert!(meta["meta_properties"].get("replace_id").is_none());
    assert_eq!(
        image::open(original.join(format!("{SOURCE}.png")))
            .unwrap()
            .to_rgba8(),
        before
    );
    let mut names: Vec<_> = walkdir::WalkDir::new(&output)
        .into_iter()
        .map(Result::unwrap)
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            e.path()
                .strip_prefix(&output)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect();
    names.sort();
    assert_eq!(
        names,
        vec![
            format!("{variant}.meta.toml"),
            format!("{variant}.png"),
            "gml/palette_assets.gml".into(),
            "gml/palette_toggle.gml".into(),
            "manifest.toml".into()
        ]
    );
    let manifest: toml::Value =
        toml::from_str(&fs::read_to_string(output.join("manifest.toml")).unwrap()).unwrap();
    assert_eq!(
        manifest["requires_hooks"].as_array().unwrap(),
        &[toml::Value::String("ui.menu_opened".into())]
    );
}

#[test]
fn toggle_package_rejects_a_second_unchanged_png() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    let output = temp.path().join("package");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();

    let before = RgbaImage::from_pixel(2, 1, image::Rgba([10, 20, 30, 255]));
    let after = RgbaImage::from_pixel(2, 1, image::Rgba([30, 20, 10, 255]));
    before.save(original.join(format!("{SOURCE}.png"))).unwrap();
    after.save(modified.join(format!("{SOURCE}.png"))).unwrap();
    let metadata = "[meta_properties]\nid = '0123456789abcdef'\nasset_kind = 'Animation'\n[asset_properties]\nframe_size = [1,1]\nframe_len = 2\natlas = 'PortraitsSpring'\n";
    for root in [&original, &modified] {
        fs::write(root.join(format!("{SOURCE}.meta.toml")), metadata).unwrap();
        before.save(root.join("spr_unrelated.png")).unwrap();
        fs::write(root.join("spr_unrelated.meta.toml"), metadata).unwrap();
    }

    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("package-toggle")
        .arg("--original")
        .arg(&original)
        .arg("--modified")
        .arg(&modified)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();

    assert!(
        !result.status.success(),
        "accepted a tree containing a second unchanged PNG: {}",
        String::from_utf8_lossy(&result.stdout)
    );
    assert!(!output.exists(), "rejected input left an output tree");
}
