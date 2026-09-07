use image::{DynamicImage, Rgba, RgbaImage};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Cursor, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
};
use zip::{ZipWriter, write::SimpleFileOptions};

const ASSET: &str = "assets/animations/NPCs/Test/Portraits/Spring/sprite.png";

fn apply(input: &Path, palette: &Path, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["apply", "--input"])
        .arg(input)
        .arg("--palette")
        .arg(palette)
        .arg("--output")
        .arg(output)
        .output()
        .unwrap()
}

fn source(root: &Path, image: &RgbaImage) -> (PathBuf, String) {
    let input = root.join("input");
    fs::create_dir(&input).unwrap();
    image.save(input.join("sprite.png")).unwrap();
    let bytes = fs::read(input.join("sprite.png")).unwrap();
    (input, format!("{:x}", Sha256::digest(bytes)))
}

fn assert_success(result: &Output) {
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn explicit_groups_connect_all_members_but_respect_original_rgba_boundaries() {
    let temp = tempfile::tempdir().unwrap();
    let image = RgbaImage::from_fn(5, 1, |x, _| {
        Rgba(match x {
            0 | 4 => [16, 32, 48, 255],
            1 | 3 => [64, 80, 96, 255],
            2 => [16, 32, 48, 128],
            _ => unreachable!(),
        })
    });
    let (input, hash) = source(temp.path(), &image);
    let palette = temp.path().join("palette.json");
    let mut recipe = json!({
        "rgba_map": {
            "#102030": "#405060",
            "#405060": "#102030",
            "#10203080": "#A0B0C080"
        },
        "color_groups": [["#102030FF", "#405060"], ["#10203080"]],
        "regions": [{
            "asset": "sprite.png",
            "source_sha256": hash,
            "size": [5, 1],
            "seeds": [[0, 0]]
        }]
    });
    fs::write(&palette, serde_json::to_vec(&recipe).unwrap()).unwrap();
    let output = temp.path().join("output");
    let result = apply(&input, &palette, &output);
    assert_success(&result);

    let after = image::open(output.join("sprite.png")).unwrap().to_rgba8();
    assert_eq!(after.get_pixel(0, 0).0, [64, 80, 96, 255]);
    assert_eq!(after.get_pixel(1, 0).0, [16, 32, 48, 255]);
    assert_eq!(after.get_pixel(2, 0).0, [16, 32, 48, 128]);
    assert_eq!(after.get_pixel(3, 0).0, [64, 80, 96, 255]);
    assert_eq!(after.get_pixel(4, 0).0, [16, 32, 48, 255]);
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["changed_pixels"], 2);
    assert_eq!(report["files"][0]["selected_pixels"], 2);
    assert_eq!(report["files"][0]["excluded_matching_pixels"], 3);

    recipe["color_groups"] = json!([]);
    fs::write(&palette, serde_json::to_vec(&recipe).unwrap()).unwrap();
    let empty = temp.path().join("empty-groups");
    assert_success(&apply(&input, &palette, &empty));
    recipe.as_object_mut().unwrap().remove("color_groups");
    fs::write(&palette, serde_json::to_vec(&recipe).unwrap()).unwrap();
    let omitted = temp.path().join("omitted-groups");
    assert_success(&apply(&input, &palette, &omitted));
    let empty_image = fs::read(empty.join("sprite.png")).unwrap();
    assert_eq!(empty_image, fs::read(omitted.join("sprite.png")).unwrap());
    let legacy = image::load_from_memory(&empty_image).unwrap().to_rgba8();
    assert_eq!(legacy.get_pixel(0, 0).0, [64, 80, 96, 255]);
    assert_eq!(legacy.get_pixel(1, 0).0, [16, 32, 48, 255]);
    assert_eq!(legacy.get_pixel(2, 0).0, [160, 176, 192, 128]);
    assert_eq!(legacy.get_pixel(3, 0).0, [16, 32, 48, 255]);
    assert_eq!(legacy.get_pixel(4, 0).0, [64, 80, 96, 255]);
}

#[test]
fn referenced_palette_accepts_empty_groups_but_rejects_nonempty_recipe_groups() {
    let temp = tempfile::tempdir().unwrap();
    let image = RgbaImage::from_fn(2, 1, |x, _| {
        Rgba(if x == 0 {
            [16, 32, 48, 255]
        } else {
            [64, 80, 96, 255]
        })
    });
    let (input, hash) = source(temp.path(), &image);
    let profile = temp.path().join("profile.json");
    fs::write(
        &profile,
        serde_json::to_vec(&json!({
            "source_colors": ["#102030", "#405060"],
            "color_groups": [["#102030"], ["#405060"]],
            "regions": [{
                "asset": "sprite.png",
                "source_sha256": hash,
                "size": [2, 1],
                "seeds": [[0, 0]]
            }]
        }))
        .unwrap(),
    )
    .unwrap();
    let palette = temp.path().join("palette.json");
    let mut recipe = json!({
        "rgba_map": {"#102030": "#A0B0C0", "#405060": "#708090"},
        "profile": "profile.json",
        "color_groups": []
    });
    fs::write(&palette, serde_json::to_vec(&recipe).unwrap()).unwrap();
    let accepted = temp.path().join("accepted");
    assert_success(&apply(&input, &palette, &accepted));
    let after = image::open(accepted.join("sprite.png")).unwrap().to_rgba8();
    assert_eq!(after.get_pixel(0, 0).0, [160, 176, 192, 255]);
    assert_eq!(after.get_pixel(1, 0).0, [64, 80, 96, 255]);

    recipe["color_groups"] = json!([["#102030"], ["#405060"]]);
    fs::write(&palette, serde_json::to_vec(&recipe).unwrap()).unwrap();
    let rejected = temp.path().join("rejected");
    let result = apply(&input, &palette, &rejected);
    assert!(!result.status.success());
    assert!(
        String::from_utf8_lossy(&result.stderr)
            .contains("Color groups must come from the referenced profile")
    );
    assert!(!rejected.exists());
}

fn png(image: RgbaImage) -> Vec<u8> {
    let mut bytes = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(image)
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    bytes.into_inner()
}

fn review_fixture(root: &Path) -> (PathBuf, PathBuf, PathBuf) {
    let mut image = RgbaImage::from_pixel(4, 1, Rgba([1, 2, 3, 255]));
    image.put_pixel(1, 0, Rgba([16, 32, 48, 255]));
    image.put_pixel(2, 0, Rgba([64, 80, 96, 255]));
    let bytes = png(image);
    let archive = root.join("assets.zip");
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    zip.start_file(ASSET, SimpleFileOptions::default()).unwrap();
    zip.write_all(&bytes).unwrap();
    zip.start_file(
        ASSET.replace(".png", ".meta.toml"),
        SimpleFileOptions::default(),
    )
    .unwrap();
    zip.write_all(b"[asset_properties]\nframe_size=[2,1]\nframe_len=2\n")
        .unwrap();
    zip.finish().unwrap();

    let profile = root.join("profile.json");
    fs::write(
        &profile,
        serde_json::to_vec(&json!({
            "source_colors": ["#102030", "#405060"],
            "color_groups": [["#102030"], ["#405060"]],
            "regions": [{
                "asset": ASSET,
                "source_sha256": format!("{:x}", Sha256::digest(&bytes)),
                "size": [4, 1],
                "seeds": [[1, 0], [2, 0]]
            }]
        }))
        .unwrap(),
    )
    .unwrap();
    let config = root.join("review.json");
    fs::write(
        &config,
        serde_json::to_vec(&json!({
            "label": "Grouped boundary",
            "prefixes": ["assets/animations/NPCs/Test/Portraits/Spring/"],
            "source_colors": ["#102030", "#405060"],
            "preview_colors": ["#A0B0C0", "#708090"],
            "reference_profile": "profile.json"
        }))
        .unwrap(),
    )
    .unwrap();
    (archive, config, profile)
}

fn review(archive: &Path, config: &Path, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["review-batch", "--archive"])
        .arg(archive)
        .arg("--config")
        .arg(config)
        .arg("--output")
        .arg(output)
        .output()
        .unwrap()
}

#[test]
fn reference_groups_control_frame_boundaries_and_cannot_be_overridden() {
    let temp = tempfile::tempdir().unwrap();
    let (archive, config, profile) = review_fixture(temp.path());
    let split = temp.path().join("split");
    assert_success(&review(&archive, &config, &split));
    let report: Value =
        serde_json::from_slice(&fs::read(split.join("batch.json")).unwrap()).unwrap();
    assert_eq!(report["color_groups"], json!([["#102030"], ["#405060"]]));

    let mut definition: Value = serde_json::from_slice(&fs::read(&config).unwrap()).unwrap();
    definition["color_groups"] = json!([["#102030"], ["#405060"]]);
    fs::write(&config, serde_json::to_vec(&definition).unwrap()).unwrap();
    let overridden = temp.path().join("overridden");
    let result = review(&archive, &config, &overridden);
    assert!(!result.status.success());
    assert!(
        String::from_utf8_lossy(&result.stderr)
            .contains("Color groups must come from the referenced profile")
    );
    assert!(!overridden.exists());

    definition.as_object_mut().unwrap().remove("color_groups");
    fs::write(&config, serde_json::to_vec(&definition).unwrap()).unwrap();
    let mut reference: Value = serde_json::from_slice(&fs::read(&profile).unwrap()).unwrap();
    reference["color_groups"] = json!([["#102030", "#405060"]]);
    fs::write(&profile, serde_json::to_vec(&reference).unwrap()).unwrap();
    let joined = temp.path().join("joined");
    let result = review(&archive, &config, &joined);
    assert!(!result.status.success());
    assert!(
        String::from_utf8_lossy(&result.stderr)
            .contains("Source-color regions cross a frame boundary")
    );
    assert!(!joined.exists());
}
