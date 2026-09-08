use image::{Rgba, RgbaImage};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Cursor, Write},
    path::Path,
    process::Command,
};
use zip::{ZipWriter, write::SimpleFileOptions};

fn fixture(root: &Path) -> std::path::PathBuf {
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    for (id, folder, colors) in [
        ("adeline", "Adeline", vec!["#28323C", "#64503C"]),
        ("hayden", "Hayden", vec!["#28323C"]),
        ("ryis", "Ryis", vec!["#28323C"]),
        ("reina", "Reina", vec!["#28323C"]),
        ("juniper", "Juniper", vec!["#28323C"]),
    ] {
        let name = format!(
            "assets/animations/NPCs/{folder}/Portraits/Spring/spr_portrait_{id}_spring_neutral.png"
        );
        let mut image = Cursor::new(Vec::new());
        RgbaImage::from_pixel(4, 1, Rgba([10, 20, 30, 255]))
            .write_to(&mut image, image::ImageFormat::Png)
            .unwrap();
        let bytes = image.into_inner();
        zip.start_file(&name, SimpleFileOptions::default()).unwrap();
        zip.write_all(&bytes).unwrap();
        zip.start_file(
            name.replace(".png", ".meta.toml"),
            SimpleFileOptions::default(),
        )
        .unwrap();
        zip.write_all(format!("[meta_properties]\nid='{id}'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[2,1]\nframe_len=2\natlas='PortraitsSpring'\n").as_bytes()).unwrap();
        fs::write(root.join(format!("{id}-profile.json")), serde_json::to_vec(&json!({
            "source_colors":["#0A141E"], "regions":[{"asset":name,"source_sha256":format!("{:x}",Sha256::digest(&bytes)),"size":[4,1],"seeds":[[0,0],[2,0]]}]
        })).unwrap()).unwrap();
        fs::write(root.join(format!("{id}.json")), serde_json::to_vec(&json!({
            "profile":format!("{id}-profile.json"), "presets":colors.iter().enumerate().map(|(i,c)| json!({"id":if i==0 {"blue"} else {"warm"},"label":if i==0 {"Debug Blue"} else {"Warm"},"colors":[c]})).collect::<Vec<_>>()
        })).unwrap()).unwrap();
    }
    fs::write(root.join("assets.zip"), zip.finish().unwrap().into_inner()).unwrap();
    let config = root.join("characters.json");
    fs::write(&config, br#"{"characters":[{"id":"adeline","presets":"adeline.json"},{"id":"hayden","presets":"hayden.json"}]}"#).unwrap();
    config
}
fn build(root: &Path, config: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("build-characters")
        .arg("--archive")
        .arg(root.join("assets.zip"))
        .arg("--characters")
        .arg(config)
        .arg("--output")
        .arg(root.join("bundle"))
        .output()
        .unwrap()
}
#[test]
fn characters_have_separate_presets_in_one_package() {
    let temp = tempfile::tempdir().unwrap();
    let config = fixture(temp.path());
    let result = build(temp.path(), &config);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["characters"].as_array().unwrap().len(), 2);
    let package = temp
        .path()
        .join("bundle/package/animations/LightningAndSun");
    for (name, color) in [
        ("adeline_spring_neutral_blue", [40, 50, 60, 255]),
        ("adeline_spring_neutral_warm", [100, 80, 60, 255]),
        ("hayden_spring_neutral_blue", [40, 50, 60, 255]),
    ] {
        let image = image::open(package.join(format!("spr_lns_{name}.png")))
            .unwrap()
            .to_rgba8();
        assert!(image.pixels().all(|p| p.0 == color));
        let meta: toml::Value = toml::from_str(
            &fs::read_to_string(package.join(format!("spr_lns_{name}.meta.toml"))).unwrap(),
        )
        .unwrap();
        assert!(meta["meta_properties"].get("id").is_none());
    }
    assert!(
        !package
            .join("spr_lns_hayden_spring_neutral_warm.png")
            .exists()
    );
    assert_eq!(fs::read_dir(&package).unwrap().count(), 6);
}

#[test]
fn hayden_can_be_built_alone_with_his_own_runtime_control() {
    let temp = tempfile::tempdir().unwrap();
    let config = fixture(temp.path());
    fs::write(
        &config,
        br#"{"characters":[{"id":"hayden","presets":"hayden.json"}]}"#,
    )
    .unwrap();

    let result = build(temp.path(), &config);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let script =
        fs::read_to_string(temp.path().join("bundle/package/gml/palette_assets.gml")).unwrap();
    let table = script
        .lines()
        .find_map(|line| {
            line.strip_prefix("function lns_palette_definitions() { return ")
                .and_then(|value| value.strip_suffix("; }"))
        })
        .expect("generated package defines the runtime character table");
    assert_eq!(
        serde_json::from_str::<Value>(table).unwrap(),
        json!([[
            "hayden",
            "Hayden",
            "F8",
            ["Vanilla", "Debug Blue"],
            [[
                "spr_portrait_hayden_spring_neutral",
                "spr_lns_hayden_spring_neutral_blue"
            ]]
        ]])
    );
}

#[test]
fn characters_can_be_built_alone_or_with_existing_characters() {
    for (ids, label, hotkey) in [
        (vec!["ryis"], "Ryis", "F10"),
        (vec!["adeline", "hayden", "ryis"], "Ryis", "F10"),
        (vec!["reina"], "Reina", "HOME"),
        (vec!["juniper"], "Juniper", "PAGE_DOWN"),
        (
            vec!["adeline", "hayden", "ryis", "reina", "juniper"],
            "Juniper",
            "PAGE_DOWN",
        ),
    ] {
        let id = ids.last().unwrap();
        let temp = tempfile::tempdir().unwrap();
        let config = fixture(temp.path());
        fs::write(
            &config,
            serde_json::to_vec(&json!({"characters": ids.iter().map(|id| {
                json!({"id":id,"presets":format!("{id}.json")})
            }).collect::<Vec<_>>()}))
            .unwrap(),
        )
        .unwrap();
        let result = build(temp.path(), &config);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let package = temp.path().join("bundle/package");
        let script = fs::read_to_string(package.join("gml/palette_assets.gml")).unwrap();
        let table: Value = serde_json::from_str(
            script
                .split_once("return ")
                .unwrap()
                .1
                .split_once("; }")
                .unwrap()
                .0,
        )
        .unwrap();
        let characters = table.as_array().unwrap();
        assert_eq!(characters.len(), ids.len());
        assert_eq!(
            characters.last().unwrap(),
            &json!([
                id,
                label,
                hotkey,
                ["Vanilla", "Debug Blue"],
                [[
                    format!("spr_portrait_{id}_spring_neutral"),
                    format!("spr_lns_{id}_spring_neutral_blue")
                ]]
            ])
        );
        let image = image::open(package.join(format!(
            "animations/LightningAndSun/spr_lns_{id}_spring_neutral_blue.png"
        )))
        .unwrap()
        .to_rgba8();
        assert_eq!(image.dimensions(), (4, 1));
        assert!(image.pixels().all(|p| p.0 == [40, 50, 60, 255]));
        if ids.len() > 1 {
            assert_eq!(characters[0][3], json!(["Vanilla", "Debug Blue", "Warm"]));
            assert_eq!(characters[1][3], json!(["Vanilla", "Debug Blue"]));
        }
    }
}

#[test]
fn characters_reject_wrong_owner_or_duplicate_selection_without_output() {
    for contents in [
        r#"{"characters":[{"id":"hayden","presets":"adeline.json"}]}"#,
        r#"{"characters":[{"id":"hayden","presets":"hayden.json"},{"id":"hayden","presets":"hayden.json"}]}"#,
    ] {
        let temp = tempfile::tempdir().unwrap();
        let config = fixture(temp.path());
        fs::write(&config, contents).unwrap();
        let result = build(temp.path(), &config);
        assert!(!result.status.success());
        assert!(!temp.path().join("bundle").exists());
    }
}
