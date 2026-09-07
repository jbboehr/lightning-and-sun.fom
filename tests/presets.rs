use image::RgbaImage;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
const NAME: &str = "spr_portrait_adeline_spring_neutral";
fn run(args: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(args)
        .output()
        .unwrap()
}
fn fixture(root: &Path) -> (std::path::PathBuf, std::path::PathBuf) {
    let original = root.join("original");
    fs::create_dir(&original).unwrap();
    let image = RgbaImage::from_fn(8, 1, |x, _| {
        image::Rgba(if x % 2 == 0 {
            [10, 20, 30, 255]
        } else {
            [0, 0, 0, 255]
        })
    });
    image.save(original.join(format!("{NAME}.png"))).unwrap();
    fs::write(original.join(format!("{NAME}.meta.toml")),"[meta_properties]\nid='source'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[4,1]\nframe_len=2\natlas='PortraitsSpring'\nduration=0.2\n").unwrap();
    let hash = format!(
        "{:x}",
        Sha256::digest(fs::read(original.join(format!("{NAME}.png"))).unwrap())
    );
    fs::write(root.join("profile.json"),serde_json::to_vec(&json!({"source_colors":["#0A141E"],"regions":[{"asset":format!("{NAME}.png"),"source_sha256":hash,"size":[8,1],"seeds":[[0,0],[4,0]]}]})).unwrap()).unwrap();
    let set = root.join("presets.json");
    fs::write(&set,serde_json::to_vec(&json!({"profile":"profile.json","presets":[{"id":"blue","label":"Debug Blue","colors":["#28323C"]},{"id":"warm","label":"Warm trial","colors":["#64503C"]}]})).unwrap()).unwrap();
    (original, set)
}
#[test]
fn presets_preserve_profile_color_groups() {
    let temp = tempfile::tempdir().unwrap();
    let (original, set) = fixture(temp.path());
    let profile_path = temp.path().join("profile.json");
    let mut profile: Value = serde_json::from_slice(&fs::read(&profile_path).unwrap()).unwrap();
    profile["source_colors"] = json!(["#0A141E", "#000000"]);
    profile["color_groups"] = json!([["#0A141E"], ["#000000"]]);
    fs::write(profile_path, serde_json::to_vec(&profile).unwrap()).unwrap();
    let mut definition: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    for preset in definition["presets"].as_array_mut().unwrap() {
        preset["colors"]
            .as_array_mut()
            .unwrap()
            .push(json!("#010203"));
    }
    fs::write(&set, serde_json::to_vec(&definition).unwrap()).unwrap();
    let output = temp.path().join("bundle");
    let result = run(&[
        Path::new("build-presets"),
        Path::new("--original"),
        &original,
        Path::new("--presets"),
        &set,
        Path::new("--output"),
        &output,
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for id in ["blue", "warm"] {
        let after = image::open(output.join(format!("variants/{id}/{NAME}.png")))
            .unwrap()
            .to_rgba8();
        assert_eq!(after.get_pixel(1, 0).0, [0, 0, 0, 255]);
        assert_eq!(after.get_pixel(2, 0).0, [10, 20, 30, 255]);
    }
}

#[test]
fn shared_masks_generate_distinct_presets_and_one_runtime_package() {
    let temp = tempfile::tempdir().unwrap();
    let (original, set) = fixture(temp.path());
    let output = temp.path().join("bundle");
    let result = run(&[
        Path::new("build-presets"),
        Path::new("--original"),
        &original,
        Path::new("--presets"),
        &set,
        Path::new("--output"),
        &output,
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["presets"].as_array().unwrap().len(), 2);
    assert_eq!(report["package"]["changed_pixels"], 4);
    for (id, color) in [("blue", [40, 50, 60, 255]), ("warm", [100, 80, 60, 255])] {
        let image = image::open(output.join(format!("variants/{id}/{NAME}.png")))
            .unwrap()
            .to_rgba8();
        assert_eq!(image.get_pixel(0, 0).0, color);
        assert_eq!(image.get_pixel(4, 0).0, color);
        assert_eq!(image.get_pixel(2, 0).0, [10, 20, 30, 255]);
        assert_eq!(image.get_pixel(6, 0).0, [10, 20, 30, 255]);
        let installed = image::open(output.join(format!(
            "package/animations/LightningAndSun/spr_lns_adeline_spring_neutral_{id}.png"
        )))
        .unwrap()
        .to_rgba8();
        assert_eq!(image, installed);
        let meta: toml::Value = toml::from_str(
            &fs::read_to_string(output.join(format!(
                "package/animations/LightningAndSun/spr_lns_adeline_spring_neutral_{id}.meta.toml"
            )))
            .unwrap(),
        )
        .unwrap();
        assert_eq!(meta["asset_properties"]["frame_len"].as_integer(), Some(2));
        assert!(meta["meta_properties"].get("id").is_none());
    }
    let script = fs::read_to_string(output.join("package/gml/palette_assets.gml")).unwrap();
    assert!(script.contains("spr_lns_adeline_spring_neutral_warm"));
    assert!(script.contains("Warm trial"));
}
#[test]
fn invalid_or_incomplete_preset_sets_leave_no_output() {
    for case in [
        "duplicate",
        "path_id",
        "wrong_length",
        "alpha",
        "missing_profile",
        "unknown_region",
    ] {
        let temp = tempfile::tempdir().unwrap();
        let (original, set) = fixture(temp.path());
        let output = temp.path().join("bundle");
        let mut value: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
        match case {
            "duplicate" => value["presets"][1]["id"] = json!("blue"),
            "path_id" => value["presets"][1]["id"] = json!("../escaped"),
            "wrong_length" => value["presets"][1]["colors"] = json!([]),
            "alpha" => value["presets"][1]["colors"] = json!(["#64503C80"]),
            "missing_profile" => value["profile"] = json!("missing.json"),
            "unknown_region" => {
                let profile = temp.path().join("profile.json");
                let mut p: Value = serde_json::from_slice(&fs::read(&profile).unwrap()).unwrap();
                p["regions"][0]["source_sha256"] = json!("0".repeat(64));
                fs::write(profile, serde_json::to_vec(&p).unwrap()).unwrap();
            }
            _ => unreachable!(),
        }
        fs::write(&set, serde_json::to_vec(&value).unwrap()).unwrap();
        let result = run(&[
            Path::new("build-presets"),
            Path::new("--original"),
            &original,
            Path::new("--presets"),
            &set,
            Path::new("--output"),
            &output,
        ]);
        assert!(!result.status.success(), "accepted {case}");
        assert!(!output.exists(), "published partial {case}");
    }
}

#[test]
fn preset_count_boundaries_accept_one_and_eight_but_reject_zero_and_nine() {
    for count in [0usize, 1, 8, 9] {
        let temp = tempfile::tempdir().unwrap();
        let (original, set) = fixture(temp.path());
        let output = temp.path().join("bundle");
        let mut value: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
        value["presets"] = json!(
            (0..count)
                .map(|index| json!({
                    "id": format!("variant_{index}"),
                    "label": format!("Variant {index}"),
                    "colors": [format!("#{:02X}5060", 40 + index)],
                }))
                .collect::<Vec<_>>()
        );
        fs::write(&set, serde_json::to_vec(&value).unwrap()).unwrap();

        let result = run(&[
            Path::new("build-presets"),
            Path::new("--original"),
            &original,
            Path::new("--presets"),
            &set,
            Path::new("--output"),
            &output,
        ]);
        if (1..=8).contains(&count) {
            assert!(
                result.status.success(),
                "rejected {count} presets: {}",
                String::from_utf8_lossy(&result.stderr)
            );
            let report: Value = serde_json::from_slice(&result.stdout).unwrap();
            assert_eq!(report["presets"].as_array().unwrap().len(), count);
            assert_eq!(
                report["package"]["variants"].as_array().unwrap().len(),
                count
            );
            for index in 0..count {
                assert!(
                    output
                        .join(format!(
                            "package/animations/LightningAndSun/\
                             spr_lns_adeline_spring_neutral_variant_{index}.png"
                        ))
                        .is_file(),
                    "preset {index} was omitted from the shared package"
                );
            }
        } else {
            assert!(!result.status.success(), "accepted {count} presets");
            assert!(!output.exists(), "published rejected {count}-preset set");
        }
    }
}

#[test]
fn null_profile_is_rejected_without_publishing_unrestricted_output() {
    let temp = tempfile::tempdir().unwrap();
    let (original, _) = fixture(temp.path());
    let recipe = temp.path().join("blue.json");
    let output = temp.path().join("single");
    fs::write(
        &recipe,
        serde_json::to_vec(&json!({"profile":null,"rgba_map":{"#0A141E":"#28323C"}})).unwrap(),
    )
    .unwrap();
    let result = run(&[
        Path::new("apply"),
        Path::new("--input"),
        &original,
        Path::new("--palette"),
        &recipe,
        Path::new("--output"),
        &output,
    ]);
    assert!(
        !result.status.success(),
        "accepted a null profile and recolored outside reviewed masks: {}",
        String::from_utf8_lossy(&result.stdout)
    );
    assert!(!output.exists(), "published output from a null profile");
}

#[test]
fn single_palette_recipes_can_reuse_a_profile_without_copying_seeds() {
    let temp = tempfile::tempdir().unwrap();
    let (original, _) = fixture(temp.path());
    let recipe = temp.path().join("blue.json");
    let output = temp.path().join("single");
    fs::write(
        &recipe,
        serde_json::to_vec(&json!({"profile":"profile.json","rgba_map":{"#0A141E":"#28323C"}}))
            .unwrap(),
    )
    .unwrap();
    let result = run(&[
        Path::new("apply"),
        Path::new("--input"),
        &original,
        Path::new("--palette"),
        &recipe,
        Path::new("--output"),
        &output,
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["changed_pixels"], 2);
    assert_eq!(
        image::open(output.join(format!("{NAME}.png")))
            .unwrap()
            .to_rgba8()
            .get_pixel(2, 0)
            .0,
        [10, 20, 30, 255]
    );
}
