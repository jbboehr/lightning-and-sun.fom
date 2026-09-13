use image::{Rgba, RgbaImage};
use std::{collections::BTreeMap, fs, process::Command};

#[test]
fn world_trial_preserves_portraits_and_adds_only_the_six_supported_strips() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let portraits: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/adeline-portraits.json")).unwrap(),
    )
    .unwrap();
    let world: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/adeline-world-trial.json")).unwrap(),
    )
    .unwrap();
    let portrait_regions: BTreeMap<_, _> = portraits["regions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|region| (region["asset"].as_str().unwrap(), region))
        .collect();
    let world_regions = world["regions"].as_array().unwrap();
    assert_eq!(world_regions.len(), 132);
    let preserved_portraits: BTreeMap<_, _> = world_regions
        .iter()
        .filter(|region| region["asset"].as_str().unwrap().contains("/Portraits/"))
        .map(|region| (region["asset"].as_str().unwrap(), region))
        .collect();
    assert_eq!(portrait_regions.len(), 126);
    assert_eq!(
        preserved_portraits, portrait_regions,
        "the world profile changed an existing portrait region"
    );

    let actual: BTreeMap<_, _> = world_regions
        .iter()
        .filter(|region| region["asset"].as_str().unwrap().contains("/Sprites/"))
        .map(|region| (region["asset"].as_str().unwrap().to_owned(), ()))
        .collect();
    let expected: BTreeMap<_, _> = ["idle", "walk"]
        .into_iter()
        .flat_map(|cycle| {
            ["north", "south", "east"].map(move |direction| {
                (
                    format!(
                        "assets/animations/NPCs/Adeline/Sprites/Spring/spr_npc_adeline_spring_{cycle}_{direction}.png"
                    ),
                    (),
                )
            })
        })
        .collect();
    assert_eq!(actual, expected);

    let presets: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("palettes/sets/adeline-world-trial.json")).unwrap(),
    )
    .unwrap();
    let choices: Vec<_> = std::iter::once("Vanilla")
        .chain(
            presets["presets"]
                .as_array()
                .unwrap()
                .iter()
                .map(|preset| preset["label"].as_str().unwrap()),
        )
        .collect();
    assert_eq!(
        choices,
        [
            "Vanilla",
            "Debug Blue",
            "Hayden palette",
            "Ryis palette",
            "Seridia palette"
        ]
    );
}

#[test]
fn world_package_preserves_idle_defaults_walk_timing_and_offsets() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();
    for season in ["spring", "summer", "autumn", "winter"] {
        for cycle in ["idle", "walk"] {
            for direction in ["north", "south", "east"] {
                let name = format!("spr_npc_adeline_{season}_{cycle}_{direction}");
                let timing = if cycle == "idle" {
                    ""
                } else {
                    "frame_len=4\nduration=0.15\n"
                };
                let count = if cycle == "idle" { 1 } else { 4 };
                let meta = format!(
                    "[meta_properties]\nid='{name}'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[2,3]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal='Middle'\nvertical=54.0\n"
                );
                for (root, color) in [
                    (&original, [10, 20, 30, 255]),
                    (&modified, [40, 50, 60, 255]),
                ] {
                    RgbaImage::from_pixel(2 * count, 3, Rgba(color))
                        .save(root.join(format!("{name}.png")))
                        .unwrap();
                    fs::write(root.join(format!("{name}.meta.toml")), &meta).unwrap();
                }
            }
        }
    }
    let output = temp.path().join("package");
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
    let mut expected = Vec::new();
    for season in ["spring", "summer", "autumn", "winter"] {
        for cycle in ["idle", "walk"] {
            for direction in ["north", "south", "east"] {
                let name = format!("spr_npc_adeline_{season}_{cycle}_{direction}");
                let target = format!("spr_lns_npc_adeline_{season}_{cycle}_{direction}_blue");
                let variant = output.join(format!("animations/LightningAndSun/{target}"));
                let before: toml::Value = toml::from_str(
                    &fs::read_to_string(original.join(format!("{name}.meta.toml"))).unwrap(),
                )
                .unwrap();
                let after: toml::Value = toml::from_str(
                    &fs::read_to_string(variant.with_extension("meta.toml")).unwrap(),
                )
                .unwrap();
                assert_eq!(before["asset_properties"], after["asset_properties"]);
                assert!(after["meta_properties"].get("id").is_none());
                assert_eq!(
                    fs::read(variant.with_extension("png")).unwrap(),
                    fs::read(modified.join(format!("{name}.png"))).unwrap()
                );
                expected.push(vec![name, target]);
            }
        }
    }
    let script = fs::read_to_string(output.join("gml/palette_assets.gml")).unwrap();
    let definitions: serde_json::Value = serde_json::from_str(
        script
            .split_once("return ")
            .unwrap()
            .1
            .split_once("; }")
            .unwrap()
            .0,
    )
    .unwrap();
    let table = &definitions[0][4];
    expected.sort();
    assert_eq!(table, &serde_json::json!(expected));
}

#[test]
fn world_package_rejects_explicit_invalid_frame_counts() {
    for (case, frame_len) in [
        ("zero", "0"),
        ("negative", "-1"),
        ("string", "'1'"),
        ("float", "1.0"),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("original");
        let modified = temp.path().join("modified");
        fs::create_dir(&original).unwrap();
        fs::create_dir(&modified).unwrap();
        let name = "spr_npc_adeline_spring_idle_south";
        let meta = format!(
            "[meta_properties]\nid='{name}'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[2,3]\nframe_len={frame_len}\natlas='Default'\n"
        );
        for (root, color) in [
            (&original, [10, 20, 30, 255]),
            (&modified, [40, 50, 60, 255]),
        ] {
            RgbaImage::from_pixel(2, 3, Rgba(color))
                .save(root.join(format!("{name}.png")))
                .unwrap();
            fs::write(root.join(format!("{name}.meta.toml")), &meta).unwrap();
        }
        let output = temp.path().join("package");
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
            "accepted {case} frame_len: {}",
            String::from_utf8_lossy(&result.stdout)
        );
        assert!(!output.exists(), "rejected {case} frame_len wrote output");
    }
}

#[test]
#[ignore = "requires the 132 local animations in extracted/adeline-world-trial"]
fn world_masks_cover_faces_and_arms_without_recoloring_clothing() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/adeline-world-trial");
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("variants");
    let recipe = std::env::var_os("FOM_WORLD_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/adeline-world-trial.json"));
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("apply")
        .arg("--input")
        .arg(&original)
        .arg("--palette")
        .arg(recipe)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["files"].as_array().unwrap().len(), 132);
    let source = [
        [233, 169, 128, 255],
        [222, 143, 93, 255],
        [186, 106, 76, 255],
        [125, 59, 20, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
    ];
    let mut frames = 0;
    let mut changed = 0;
    for row in report["files"].as_array().unwrap() {
        let name = row["path"].as_str().unwrap();
        if !name.contains("/Sprites/") {
            continue;
        }
        let before = image::open(original.join(name)).unwrap().to_rgba8();
        let after = image::open(output.join(name)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), after.dimensions());
        let meta = name.replace(".png", ".meta.toml");
        assert_eq!(
            fs::read(original.join(&meta)).unwrap(),
            fs::read(output.join(&meta)).unwrap()
        );
        frames += before.width() / 80;
        for (x, y, p) in before.enumerate_pixels() {
            let q = after.get_pixel(x, y);
            assert_eq!(p[3], q[3]);
            let shade = source.iter().position(|c| c == &p.0);
            if shade.is_none() || y >= 49 || ((38..=43).contains(&(x % 80)) && y >= 44) {
                assert_eq!(p, q, "changed clothing: {name} [{x},{y}]");
            }
            if y < 44
                && let Some(shade) = shade
            {
                assert_eq!(q.0, blue[shade], "missed face: {name} [{x},{y}]");
            }
            if p != q {
                changed += 1;
            }
        }
        for frame in 0..before.width() / 80 {
            // Reviewed hand landmarks account for the walking pose changes.
            let (x, y) = if name.ends_with("north.png") {
                (33, 45 + frame % 2)
            } else if name.ends_with("south.png") {
                (34, 44 + frame % 2)
            } else {
                (
                    match frame {
                        1 => 33,
                        3 => 36,
                        _ => 35,
                    },
                    44 + frame % 2,
                )
            };
            let x = x + 80 * frame;
            let shade = source
                .iter()
                .position(|c| c == &before.get_pixel(x, y).0)
                .unwrap();
            assert_eq!(
                after.get_pixel(x, y).0,
                blue[shade],
                "missed hand: {name} [{x},{y}]"
            );
        }
    }
    assert_eq!(frames, 15);
    eprintln!("Verified six world strips / {frames} frames: {changed} skin pixels changed");
}
