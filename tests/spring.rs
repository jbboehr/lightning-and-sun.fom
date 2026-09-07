use image::{Rgba, RgbaImage};
use serde_json::{Value, json};
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Output},
};
use zip::{ZipWriter, write::SimpleFileOptions};

const EXPRESSIONS: [&str; 25] = [
    "angry_blush",
    "blush",
    "cartoon_embarrassed",
    "concerned",
    "embarrassed",
    "embarrassed_tired",
    "evasive_tired",
    "gloomy_special",
    "happy",
    "happy_blush",
    "hope_special",
    "mad",
    "neutral",
    "neutral_tired",
    "sad",
    "shocked",
    "sick_eyes_closed",
    "sick_eyes_open",
    "sick_smile",
    "sick_think",
    "sigh",
    "sly",
    "think",
    "ugh",
    "wink",
];

fn command(name: &str, args: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg(name)
        .args(args)
        .output()
        .unwrap()
}
fn success(result: Output) -> Value {
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&result.stdout).unwrap()
}
fn fixture(root: &Path, name: &str, frames: u32, color: [u8; 4]) {
    fs::create_dir_all(root).unwrap();
    RgbaImage::from_pixel(2 * frames, 1, Rgba(color))
        .save(root.join(format!("{name}.png")))
        .unwrap();
    fs::write(root.join(format!("{name}.meta.toml")),format!("[meta_properties]\nid = '{name}'\nasset_kind = 'Animation'\n[asset_properties]\nframe_size = [2,1]\nframe_len = {frames}\nduration = 0.2\natlas = 'PortraitsSpring'\n")).unwrap();
}

#[test]
fn export_supports_a_complete_50_portrait_selection() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    fixture(&source, "sample", 2, [10, 20, 30, 255]);
    let archive = temp.path().join("assets.zip");
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    for i in 0..50 {
        for ext in ["png", "meta.toml"] {
            zip.start_file(
                format!("assets/spr_test_{i}.{ext}"),
                SimpleFileOptions::default(),
            )
            .unwrap();
            zip.write_all(&fs::read(source.join(format!("sample.{ext}"))).unwrap())
                .unwrap();
        }
    }
    zip.finish().unwrap();
    let before = fs::read(&archive).unwrap();
    let output = temp.path().join("export");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_mistria-palette"));
    cmd.arg("export")
        .arg("--archive")
        .arg(&archive)
        .arg("--output")
        .arg(&output);
    for i in 0..50 {
        cmd.arg("--asset").arg(format!("assets/spr_test_{i}.png"));
    }
    let report = success(cmd.output().unwrap());
    assert_eq!(report["files"].as_array().unwrap().len(), 50);
    for i in 0..50 {
        assert_eq!(
            fs::read(output.join(format!("assets/spr_test_{i}.png"))).unwrap(),
            fs::read(source.join("sample.png")).unwrap()
        );
    }
    assert_eq!(fs::read(archive).unwrap(), before);
}

#[test]
fn export_rejects_empty_duplicate_and_51_asset_selections_before_writing() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    fixture(&source, "sample", 1, [10, 20, 30, 255]);
    let archive = temp.path().join("assets.zip");
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    for i in 0..51 {
        for ext in ["png", "meta.toml"] {
            zip.start_file(
                format!("assets/spr_test_{i}.{ext}"),
                SimpleFileOptions::default(),
            )
            .unwrap();
            zip.write_all(&fs::read(source.join(format!("sample.{ext}"))).unwrap())
                .unwrap();
        }
    }
    zip.finish().unwrap();
    let before = fs::read(&archive).unwrap();

    for (case, assets) in [
        ("empty", Vec::new()),
        ("duplicate", vec!["assets/spr_test_0.png".to_owned(); 2]),
        (
            "over-limit",
            (0..51)
                .map(|i| format!("assets/spr_test_{i}.png"))
                .collect(),
        ),
    ] {
        let output = temp.path().join(format!("export-{case}"));
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_mistria-palette"));
        cmd.arg("export")
            .arg("--archive")
            .arg(&archive)
            .arg("--output")
            .arg(&output);
        for asset in assets {
            cmd.arg("--asset").arg(asset);
        }
        let result = cmd.output().unwrap();
        assert!(!result.status.success(), "accepted {case} selection");
        assert!(!output.exists(), "rejected {case} selection wrote output");
        assert_eq!(fs::read(&archive).unwrap(), before);
    }
}

#[test]
fn toggle_package_preserves_each_expressions_frames_and_properties() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    let output = temp.path().join("package");
    for (index, expression) in EXPRESSIONS.iter().enumerate() {
        let frames = (index % 3 + 1) as u32;
        let name = format!("spr_portrait_adeline_spring_{expression}");
        fixture(&original, &name, frames, [10, 20, 30, 255]);
        fixture(&modified, &name, frames, [40, 50, 60, 255]);
    }
    let report = success(command(
        "package-toggle",
        &[
            Path::new("--original"),
            &original,
            Path::new("--modified"),
            &modified,
            Path::new("--output"),
            &output,
        ],
    ));
    assert_eq!(report["variants"].as_array().unwrap().len(), 25);
    for (index, expression) in EXPRESSIONS.iter().enumerate() {
        let frames = (index % 3 + 1) as u32;
        let source = format!("spr_portrait_adeline_spring_{expression}");
        let variant =
            format!("animations/LightningAndSun/spr_lns_adeline_spring_{expression}_blue");
        assert_eq!(
            image::open(output.join(format!("{variant}.png")))
                .unwrap()
                .to_rgba8(),
            RgbaImage::from_pixel(2 * frames, 1, Rgba([40, 50, 60, 255]))
        );
        let before: toml::Value = toml::from_str(
            &fs::read_to_string(original.join(format!("{source}.meta.toml"))).unwrap(),
        )
        .unwrap();
        let after: toml::Value = toml::from_str(
            &fs::read_to_string(output.join(format!("{variant}.meta.toml"))).unwrap(),
        )
        .unwrap();
        assert_eq!(before["asset_properties"], after["asset_properties"]);
        assert!(after["meta_properties"].get("id").is_none());
    }
    let script = fs::read_to_string(output.join("gml/palette_assets.gml")).unwrap();
    let table = script
        .strip_prefix("// Generated from the portraits included in this local package.\nfunction lns_palette_assets() { return ")
        .and_then(|value| value.split_once("; }\n"))
        .map(|(table, _)| table)
        .expect("unexpected palette asset script wrapper");
    let actual: Value = serde_json::from_str(table).unwrap();
    let expected: Vec<_> = EXPRESSIONS
        .iter()
        .map(|expression| {
            json!([
                format!("spr_portrait_adeline_spring_{expression}"),
                format!("spr_lns_adeline_spring_{expression}_blue")
            ])
        })
        .collect();
    assert_eq!(actual, json!(expected));
    assert_eq!(report["changed_pixels"], json!(98));
}

#[test]
fn toggle_package_rejects_duplicate_expression_paths_before_writing() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    let output = temp.path().join("package");
    for sub in ["first", "second"] {
        fixture(
            &original.join(sub),
            "spr_portrait_adeline_spring_neutral",
            2,
            [10, 20, 30, 255],
        );
        fixture(
            &modified.join(sub),
            "spr_portrait_adeline_spring_neutral",
            2,
            [40, 50, 60, 255],
        );
    }
    let result = command(
        "package-toggle",
        &[
            Path::new("--original"),
            &original,
            Path::new("--modified"),
            &modified,
            Path::new("--output"),
            &output,
        ],
    );
    assert!(!result.status.success());
    assert!(!output.exists());
}

#[test]
fn toggle_package_rejects_a_near_miss_expression_before_writing() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    let output = temp.path().join("package");
    let name = "spr_portrait_adeline_spring_neutral_extra";
    fixture(&original, name, 2, [10, 20, 30, 255]);
    fixture(&modified, name, 2, [40, 50, 60, 255]);

    let result = command(
        "package-toggle",
        &[
            Path::new("--original"),
            &original,
            Path::new("--modified"),
            &modified,
            Path::new("--output"),
            &output,
        ],
    );
    assert!(!result.status.success(), "accepted a near-miss expression");
    assert!(!output.exists());
}

#[test]
#[ignore = "requires the 25 local Adeline spring portraits in extracted/adeline-spring-study"]
fn spring_masks_cover_skin_and_preserve_reviewed_clothing_landmarks() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/adeline-spring-study");
    let palette = std::env::var_os("FOM_SPRING_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/adeline-spring.json"));
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let report = success(command(
        "apply",
        &[
            Path::new("--input"),
            &original,
            Path::new("--palette"),
            &palette,
            Path::new("--output"),
            &modified,
        ],
    ));
    let files = report["files"].as_array().unwrap();
    assert_eq!(files.len(), 25);
    let mut total = 0usize;
    for row in files {
        let name = row["path"].as_str().unwrap();
        let before = image::open(original.join(name)).unwrap().to_rgba8();
        let after = image::open(modified.join(name)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (592, 180));
        assert_eq!(before.dimensions(), after.dimensions());
        let mut changed = [0usize; 2];
        for (x, y, left) in before.enumerate_pixels() {
            let right = after.get_pixel(x, y);
            assert_eq!(left[3], right[3]);
            if y >= 148 {
                assert_eq!(left, right, "robe changed: {name} [{x},{y}]");
            }
            if left.0 == [212, 131, 99, 255] {
                assert_eq!(
                    right.0,
                    [127, 159, 189, 255],
                    "missed fourth skin shade: {name} [{x},{y}]"
                );
            }
            if left != right {
                changed[(x / 296) as usize] += 1;
            }
        }
        // Interior landmarks shared by every pose; neutral's boundary seeds are
        // not valid landmarks when an expression shifts its arm by a pixel.
        for frame in 0..2 {
            for (x, y) in [(179, 100), (139, 121), (187, 132), (152, 138)] {
                let x = x + 296 * frame;
                assert_ne!(
                    before.get_pixel(x, y),
                    after.get_pixel(x, y),
                    "unchanged skin: {name} [{x},{y}]"
                );
            }
            for (x, y) in [(149, 101), (134, 118), (172, 144)] {
                let x = x + 296 * frame;
                assert_eq!(
                    before.get_pixel(x, y),
                    after.get_pixel(x, y),
                    "changed jewelry: {name} [{x},{y}]"
                );
            }
        }
        assert!(changed.iter().all(|&n| n > 0));
        total += changed.iter().sum::<usize>();
        assert_eq!(
            fs::read(original.join(name.replace(".png", ".meta.toml"))).unwrap(),
            fs::read(modified.join(name.replace(".png", ".meta.toml"))).unwrap()
        );
    }
    eprintln!("Verified 25 strips / 50 frames; changed {total} pixels");
}
