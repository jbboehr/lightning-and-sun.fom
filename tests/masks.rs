use image::{Rgba, RgbaImage};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};

struct Lab {
    _temp: tempfile::TempDir,
    original: PathBuf,
    modified: PathBuf,
    palette: PathBuf,
    before: RgbaImage,
    recipe: Value,
}
impl Lab {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("original");
        let modified = temp.path().join("modified");
        let palette = temp.path().join("palette.json");
        fs::create_dir(&original).unwrap();
        let mut before = RgbaImage::from_pixel(5, 3, Rgba([16, 32, 48, 0]));
        for (x, y) in [(0, 0), (1, 0), (3, 0), (2, 2)] {
            before.put_pixel(x, y, Rgba([16, 32, 48, 255]));
        }
        before.put_pixel(1, 1, Rgba([64, 80, 96, 128]));
        before.put_pixel(4, 0, Rgba([64, 80, 96, 128]));
        before.put_pixel(4, 2, Rgba([1, 2, 3, 255]));
        before.save(original.join("sprite.png")).unwrap();
        fs::write(original.join("sprite.meta.toml"), "metadata stays exact\n").unwrap();
        let hash = format!(
            "{:x}",
            Sha256::digest(fs::read(original.join("sprite.png")).unwrap())
        );
        let recipe = json!({"rgba_map":{"#102030":"#405060", "#40506080":"#70809080", "#10203000":"#10203000"},
            "regions":[{"asset":"sprite.png","source_sha256":hash,"size":[5,3],"seeds":[[0,0],[1,0]]}]});
        Self {
            _temp: temp,
            original,
            modified,
            palette,
            before,
            recipe,
        }
    }
    fn apply(&self) -> Output {
        fs::write(&self.palette, serde_json::to_vec(&self.recipe).unwrap()).unwrap();
        Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .arg("apply")
            .arg("--input")
            .arg(&self.original)
            .arg("--palette")
            .arg(&self.palette)
            .arg("--output")
            .arg(&self.modified)
            .output()
            .unwrap()
    }
}

#[test]
fn color_groups_keep_detail_seeds_from_joining_skin_and_clothing() {
    for shared_profile in [false, true] {
        let mut lab = Lab::new();
        let original = RgbaImage::from_fn(5, 1, |x, _| {
            Rgba(match x {
                0 | 2 => [16, 32, 48, 255],
                1 | 4 => [64, 80, 96, 255],
                _ => [0, 0, 0, 255],
            })
        });
        original.save(lab.original.join("sprite.png")).unwrap();
        let hash = format!(
            "{:x}",
            Sha256::digest(fs::read(lab.original.join("sprite.png")).unwrap())
        );
        lab.recipe = json!({
            "rgba_map":{"#102030":"#405060", "#405060":"#708090"},
            "color_groups":[["#102030"],["#405060"]],
            "regions":[{"asset":"sprite.png","source_sha256":hash,"size":[5,1],"seeds":[[0,0],[4,0]]}]
        });
        if shared_profile {
            fs::write(
                lab.palette.with_file_name("profile.json"),
                serde_json::to_vec(&json!({
                    "source_colors":["#102030", "#405060"],
                    "color_groups":lab.recipe["color_groups"], "regions":lab.recipe["regions"]
                }))
                .unwrap(),
            )
            .unwrap();
            lab.recipe.as_object_mut().unwrap().remove("regions");
            lab.recipe.as_object_mut().unwrap().remove("color_groups");
            lab.recipe["profile"] = json!("profile.json");
        }
        let result = lab.apply();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let mut expected = original;
        expected.put_pixel(0, 0, Rgba([64, 80, 96, 255]));
        expected.put_pixel(4, 0, Rgba([112, 128, 144, 255]));
        assert_eq!(
            image::open(lab.modified.join("sprite.png"))
                .unwrap()
                .to_rgba8(),
            expected
        );
    }
}

#[test]
fn color_groups_reject_ambiguous_or_incomplete_color_partitions() {
    for groups in [
        json!([["#102030"], []]),
        json!([["#102030"]]),
        json!([["#102030", "#40506080", "#10203000"], ["#102030FF"]]),
        json!([["#102030", "#40506080", "#10203000", "#FFFFFF"]]),
        Value::Null,
    ] {
        let mut lab = Lab::new();
        lab.recipe["color_groups"] = groups;
        assert!(!lab.apply().status.success());
        assert!(!lab.modified.exists());
    }
}

#[test]
fn seeded_regions_limit_recoloring_and_count_overlapping_seeds_once() {
    let lab = Lab::new();
    let result = lab.apply();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let mut expected = lab.before.clone();
    expected.put_pixel(0, 0, Rgba([64, 80, 96, 255]));
    expected.put_pixel(1, 0, Rgba([64, 80, 96, 255]));
    expected.put_pixel(1, 1, Rgba([112, 128, 144, 128]));
    assert_eq!(
        image::open(lab.modified.join("sprite.png"))
            .unwrap()
            .to_rgba8(),
        expected
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["changed_pixels"], 3);
    assert_eq!(report["files"][0]["selected_pixels"], 3);
    assert_eq!(report["files"][0]["excluded_matching_pixels"], 3);
    assert_eq!(
        fs::read(lab.original.join("sprite.meta.toml")).unwrap(),
        fs::read(lab.modified.join("sprite.meta.toml")).unwrap()
    );
    assert_eq!(
        image::open(lab.original.join("sprite.png"))
            .unwrap()
            .to_rgba8(),
        lab.before
    );
}

#[test]
fn empty_seeds_select_nothing_while_omitted_regions_remain_unrestricted() {
    let mut empty = Lab::new();
    empty.recipe["regions"][0]["seeds"] = json!([]);
    let result = empty.apply();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["changed_pixels"], 0);
    assert_eq!(report["files"][0]["selected_pixels"], 0);
    assert_eq!(report["files"][0]["excluded_matching_pixels"], 6);
    assert_eq!(
        fs::read(empty.original.join("sprite.png")).unwrap(),
        fs::read(empty.modified.join("sprite.png")).unwrap()
    );

    let mut unrestricted = Lab::new();
    unrestricted
        .recipe
        .as_object_mut()
        .unwrap()
        .remove("regions");
    let result = unrestricted.apply();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["changed_pixels"], 6);
    assert!(report["files"][0].get("selected_pixels").is_none());
    let mut expected = unrestricted.before.clone();
    for (x, y) in [(0, 0), (1, 0), (3, 0), (2, 2)] {
        expected.put_pixel(x, y, Rgba([64, 80, 96, 255]));
    }
    for (x, y) in [(1, 1), (4, 0)] {
        expected.put_pixel(x, y, Rgba([112, 128, 144, 128]));
    }
    assert_eq!(
        image::open(unrestricted.modified.join("sprite.png"))
            .unwrap()
            .to_rgba8(),
        expected
    );
}

#[test]
fn mask_selection_and_mapping_both_use_original_pixels() {
    let mut lab = Lab::new();
    let before = RgbaImage::from_raw(
        5,
        1,
        vec![
            16, 32, 48, 255, 64, 80, 96, 255, 16, 32, 48, 255, 1, 2, 3, 255, 64, 80, 96, 255,
        ],
    )
    .unwrap();
    before.save(lab.original.join("sprite.png")).unwrap();
    let hash = format!(
        "{:x}",
        Sha256::digest(fs::read(lab.original.join("sprite.png")).unwrap())
    );
    lab.recipe = json!({
        "rgba_map": {"#102030":"#405060", "#405060":"#708090"},
        "regions": [{"asset":"sprite.png", "source_sha256":hash, "size":[5,1], "seeds":[[0,0]]}]
    });

    let result = lab.apply();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["changed_pixels"], 3);
    assert_eq!(report["files"][0]["selected_pixels"], 3);
    assert_eq!(report["files"][0]["excluded_matching_pixels"], 1);
    assert_eq!(
        image::open(lab.modified.join("sprite.png"))
            .unwrap()
            .to_rgba8()
            .into_raw(),
        vec![
            64, 80, 96, 255, 112, 128, 144, 255, 64, 80, 96, 255, 1, 2, 3, 255, 64, 80, 96, 255,
        ]
    );
}

#[test]
fn invalid_region_recipes_fail_without_output() {
    for mutation in [
        "hash",
        "short_hash",
        "non_hex_hash",
        "size",
        "seed_bounds",
        "seed_color",
        "seed_unmapped",
        "missing_asset",
        "extra_asset",
        "duplicate_asset",
        "unknown_field",
        "null_regions",
    ] {
        let mut lab = Lab::new();
        match mutation {
            "hash" => lab.recipe["regions"][0]["source_sha256"] = json!("0".repeat(64)),
            "short_hash" => lab.recipe["regions"][0]["source_sha256"] = json!("0".repeat(63)),
            "non_hex_hash" => lab.recipe["regions"][0]["source_sha256"] = json!("g".repeat(64)),
            "size" => lab.recipe["regions"][0]["size"] = json!([3, 5]),
            "seed_bounds" => lab.recipe["regions"][0]["seeds"] = json!([[5, 0]]),
            "seed_color" => lab.recipe["regions"][0]["seeds"] = json!([[2, 0]]),
            "seed_unmapped" => lab.recipe["regions"][0]["seeds"] = json!([[4, 2]]),
            "missing_asset" => lab.recipe["regions"] = json!([]),
            "extra_asset" => {
                let mut extra = lab.recipe["regions"][0].clone();
                extra["asset"] = json!("missing.png");
                lab.recipe["regions"].as_array_mut().unwrap().push(extra);
            }
            "duplicate_asset" => {
                let copy = lab.recipe["regions"][0].clone();
                lab.recipe["regions"].as_array_mut().unwrap().push(copy);
            }
            "unknown_field" => lab.recipe["regions"][0]["seed"] = json!([[0, 0]]),
            "null_regions" => lab.recipe["regions"] = Value::Null,
            _ => unreachable!(),
        }
        let result = lab.apply();
        assert!(!result.status.success(), "accepted {mutation}");
        assert!(!lab.modified.exists(), "left output after {mutation}");
    }
}

#[test]
fn recipe_validation_rejects_wrong_or_outside_region_changes() {
    let lab = Lab::new();
    assert!(lab.apply().status.success());
    let validate = || {
        Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .arg("validate")
            .arg("--original")
            .arg(&lab.original)
            .arg("--modified")
            .arg(&lab.modified)
            .arg("--palette")
            .arg(&lab.palette)
            .output()
            .unwrap()
    };
    let result = validate();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&result.stdout).unwrap()["palette_verified"],
        true
    );
    let correct = image::open(lab.modified.join("sprite.png"))
        .unwrap()
        .to_rgba8();
    for (x, y, color) in [
        (3, 0, [64, 80, 96, 255]),
        (0, 0, [16, 32, 48, 255]),
        (1, 1, [16, 32, 48, 128]),
        (2, 0, [0, 0, 0, 0]),
    ] {
        let mut wrong = correct.clone();
        wrong.put_pixel(x, y, Rgba(color));
        wrong.save(lab.modified.join("sprite.png")).unwrap();
        let result = validate();
        assert!(!result.status.success(), "accepted wrong pixel [{x},{y}]");
        assert!(String::from_utf8_lossy(&result.stderr).contains("Palette mismatch"));
    }
}

#[test]
fn comparison_preview_highlights_only_changed_pixels() {
    let lab = Lab::new();
    assert!(lab.apply().status.success());
    let output = lab._temp.path().join("preview.png");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("contact-sheet")
        .arg("--original")
        .arg(&lab.original)
        .arg("--modified")
        .arg(&lab.modified)
        .arg("--output")
        .arg(&output)
        .arg("--changes")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    let area = report["files"][0]["changes_box"].as_array().unwrap();
    let x = area[0].as_u64().unwrap() as u32;
    let y = area[1].as_u64().unwrap() as u32;
    assert_eq!(&area[2..], &[json!(20), json!(12)]);
    let preview = image::open(output).unwrap().to_rgb8();
    assert_eq!(preview.get_pixel(x, y).0, [255, 0, 255]);
    assert_eq!(preview.get_pixel(x + 12, y).0, [255, 255, 255]);
}

#[test]
#[ignore = "requires the locally exported Adeline portrait in extracted/adeline-rust"]
fn adeline_regions_preserve_clothing_and_cover_both_frames() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/adeline-rust");
    let asset =
        "assets/animations/NPCs/Adeline/Portraits/Spring/spr_portrait_adeline_spring_neutral.png";
    let bytes = fs::read(original.join(asset)).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "52e2871f2c96db217a568d15f3daaef666c2af9b90263ef0b39f0e38722c51bc"
    );
    let palette = std::env::var_os("FOM_PALETTE_TEST_RECIPE")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/adeline.json"));
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("modified");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("apply")
        .arg("--input")
        .arg(&original)
        .arg("--palette")
        .arg(palette)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let before = image::load_from_memory(&bytes).unwrap().to_rgba8();
    let after = image::open(output.join(asset)).unwrap().to_rgba8();
    assert_eq!(before.dimensions(), after.dimensions());
    let mut changed = [0usize; 2];
    let mut fourth_shade = 0;
    for (x, y, left) in before.enumerate_pixels() {
        let right = after.get_pixel(x, y);
        assert_eq!(left[3], right[3]);
        if y >= 148 {
            assert_eq!(left, right, "robe changed at [{x},{y}]");
        }
        if left != right {
            changed[(x / 296) as usize] += 1;
        }
        if left.0 == [212, 131, 99, 255] {
            assert_eq!(
                right.0,
                [127, 159, 189, 255],
                "missed skin shade at [{x},{y}]"
            );
            fourth_shade += 1;
        }
    }
    // Reviewed landmarks on face, ear, neck, arm and hand, followed by jewelry
    // highlights that share the old palette colors and must stay original.
    for frame in 0..2 {
        for (x, y) in [(152, 55), (133, 70), (140, 82), (137, 116), (183, 128)] {
            assert_ne!(
                before.get_pixel(x + 296 * frame, y),
                after.get_pixel(x + 296 * frame, y)
            );
        }
        for (x, y) in [
            (136, 82),
            (138, 82),
            (139, 88),
            (150, 95),
            (185, 107),
            (178, 110),
            (185, 113),
            (160, 135),
            (165, 135),
            (166, 143),
            (172, 143),
        ] {
            assert_eq!(
                before.get_pixel(x + 296 * frame, y),
                after.get_pixel(x + 296 * frame, y),
                "jewelry changed at [{x},{y}], frame {frame}"
            );
        }
    }
    assert!(fourth_shade > 0);
    assert_eq!(
        fs::read(original.join(asset.replace(".png", ".meta.toml"))).unwrap(),
        fs::read(output.join(asset.replace(".png", ".meta.toml"))).unwrap()
    );
    eprintln!("Changed pixels by frame: {changed:?}; fourth skin shade: {fourth_shade}");
}
