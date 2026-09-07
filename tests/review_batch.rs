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

const PREFIX: &str = "assets/animations/NPCs/Test/Portraits/Spring/";
fn png(image: RgbaImage) -> Vec<u8> {
    let mut output = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(image)
        .write_to(&mut output, image::ImageFormat::Png)
        .unwrap();
    output.into_inner()
}
fn fixture(root: &Path) -> (PathBuf, PathBuf) {
    let mut frame = RgbaImage::new(5, 3);
    for (x, y) in [(1, 0), (1, 1), (3, 2)] {
        frame.put_pixel(x, y, Rgba([200, 100, 50, 255]));
    }
    let mut strip = RgbaImage::new(10, 3);
    image::imageops::replace(&mut strip, &frame, 0, 0);
    image::imageops::replace(&mut strip, &frame, 5, 0);
    strip.put_pixel(7, 0, Rgba([0, 0, 0, 255]));
    let source = png(strip);
    let mut changed = frame.clone();
    changed.put_pixel(3, 0, Rgba([200, 100, 50, 255]));
    let archive = root.join("assets.zip");
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    for (name, bytes, count) in [
        ("source", source.clone(), 2),
        ("duplicate", png(frame), 1),
        ("new", png(changed), 1),
    ] {
        zip.start_file(format!("{PREFIX}{name}.png"), SimpleFileOptions::default())
            .unwrap();
        zip.write_all(&bytes).unwrap();
        zip.start_file(
            format!("{PREFIX}{name}.meta.toml"),
            SimpleFileOptions::default(),
        )
        .unwrap();
        write!(
            zip,
            "[asset_properties]\nframe_size=[5,3]\nframe_len={count}\n"
        )
        .unwrap();
    }
    zip.finish().unwrap();
    fs::write(root.join("approved.json"),serde_json::to_vec(&json!({
        "source_colors":["#C86432"], "color_groups":[["#C86432"]], "regions":[{
            "asset":format!("{PREFIX}source.png"), "source_sha256":format!("{:x}",Sha256::digest(source)),
            "size":[10,3], "seeds":[[1,0],[6,0]]
        }]
    })).unwrap()).unwrap();
    let config = root.join("review.json");
    fs::write(
        &config,
        serde_json::to_vec(&json!({
            "label":"Test <character>","prefixes":[PREFIX],"source_colors":["#C86432"],
            "preview_colors":["#5599FF"],"reference_profile":"approved.json"
        }))
        .unwrap(),
    )
    .unwrap();
    (archive, config)
}
fn run(archive: &Path, config: &Path, output: &Path) -> Output {
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
fn success(result: Output) {
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
fn group<'a>(report: &'a Value, asset: &str) -> &'a Value {
    report["groups"]
        .as_array()
        .unwrap()
        .iter()
        .find(|g| {
            g["occurrences"]
                .as_array()
                .unwrap()
                .iter()
                .any(|o| o["asset"] == format!("{PREFIX}{asset}.png"))
        })
        .unwrap()
}

#[test]
fn review_inherits_color_groups_and_keeps_adjacent_details_separate() {
    let temp = tempfile::tempdir().unwrap();
    let (archive, config) = fixture(temp.path());
    let profile_path = temp.path().join("approved.json");
    let mut profile: Value = serde_json::from_slice(&fs::read(&profile_path).unwrap()).unwrap();
    profile["source_colors"] = json!(["#C86432", "#000000"]);
    profile["color_groups"] = json!([["#C86432"], ["#000000"]]);
    fs::write(profile_path, serde_json::to_vec(&profile).unwrap()).unwrap();
    let mut config_value: Value = serde_json::from_slice(&fs::read(&config).unwrap()).unwrap();
    config_value["source_colors"] = profile["source_colors"].clone();
    config_value["preview_colors"] = json!(["#5599FF", "#123456"]);
    fs::write(&config, serde_json::to_vec(&config_value).unwrap()).unwrap();
    let output = temp.path().join("review");
    success(run(&archive, &config, &output));
    let report: Value =
        serde_json::from_slice(&fs::read(output.join("batch.json")).unwrap()).unwrap();
    assert_eq!(report["color_groups"], profile["color_groups"]);
    let second = report["groups"]
        .as_array()
        .unwrap()
        .iter()
        .find(|g| {
            g["occurrences"]
                .as_array()
                .unwrap()
                .iter()
                .any(|o| o["asset"] == format!("{PREFIX}source.png") && o["frame"] == 1)
        })
        .unwrap();
    assert_eq!(second["status"], "reused");
    assert_eq!(second["selected"], json!([0]));
    assert_eq!(
        second["components"][0]["runs"],
        json!([[1, 1, 1], [6, 1, 1]])
    );
    assert_eq!(second["components"][1]["seed"], json!([2, 0]));
}

#[test]
fn references_outside_the_selected_prefix_respect_dimensions_and_transparent_rgb() {
    let temp = tempfile::tempdir().unwrap();
    let reference_name = "assets/animations/NPCs/Reference/approved.png";
    let mut approved = RgbaImage::new(2, 1);
    approved.put_pixel(0, 0, Rgba([200, 100, 50, 255]));
    approved.put_pixel(1, 0, Rgba([1, 2, 3, 0]));
    let approved = png(approved);
    let mut transparent_change = RgbaImage::new(2, 1);
    transparent_change.put_pixel(0, 0, Rgba([200, 100, 50, 255]));
    transparent_change.put_pixel(1, 0, Rgba([9, 8, 7, 0]));
    let mut tall = RgbaImage::new(1, 2);
    tall.put_pixel(0, 0, Rgba([200, 100, 50, 255]));
    tall.put_pixel(0, 1, Rgba([1, 2, 3, 0]));

    let archive = temp.path().join("assets.zip");
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    for (name, bytes, frame_size) in [
        (reference_name.to_owned(), approved.clone(), [2, 1]),
        (format!("{PREFIX}candidate.png"), approved.clone(), [2, 1]),
        (
            format!("{PREFIX}transparent-change.png"),
            png(transparent_change),
            [2, 1],
        ),
        (format!("{PREFIX}tall.png"), png(tall), [1, 2]),
    ] {
        zip.start_file(&name, SimpleFileOptions::default()).unwrap();
        zip.write_all(&bytes).unwrap();
        zip.start_file(
            format!("{}.meta.toml", name.strip_suffix(".png").unwrap()),
            SimpleFileOptions::default(),
        )
        .unwrap();
        write!(
            zip,
            "[asset_properties]\nframe_size=[{},{}]\nframe_len=1\n",
            frame_size[0], frame_size[1]
        )
        .unwrap();
    }
    zip.finish().unwrap();

    let reference = temp.path().join("approved.json");
    fs::write(
        &reference,
        serde_json::to_vec(&json!({
            "source_colors":["#C86432"],
            "regions":[{
                "asset":reference_name,
                "source_sha256":format!("{:x}",Sha256::digest(&approved)),
                "size":[2,1],
                "seeds":[[0,0]]
            }]
        }))
        .unwrap(),
    )
    .unwrap();
    let config = temp.path().join("review.json");
    fs::write(
        &config,
        serde_json::to_vec(&json!({
            "label":"Reference boundary test",
            "prefixes":[PREFIX],
            "source_colors":["#C86432"],
            "preview_colors":["#5599FF"],
            "reference_profile":"approved.json"
        }))
        .unwrap(),
    )
    .unwrap();

    let output = temp.path().join("review");
    success(run(&archive, &config, &output));
    let report: Value =
        serde_json::from_slice(&fs::read(output.join("batch.json")).unwrap()).unwrap();
    assert_eq!(report["groups"].as_array().unwrap().len(), 3);
    let candidate = group(&report, "candidate");
    assert_eq!(candidate["status"], "reused");
    assert_eq!(candidate["selected"], json!([0]));
    assert!(
        candidate["references"]
            .as_array()
            .unwrap()
            .iter()
            .any(|occurrence| occurrence["asset"] == reference_name)
    );
    let transparent_change = group(&report, "transparent-change");
    assert_ne!(candidate["id"], transparent_change["id"]);
    assert_eq!(transparent_change["status"], "suggested");
    assert_eq!(transparent_change["selected"], json!([0]));
    let tall = group(&report, "tall");
    assert_ne!(candidate["id"], tall["id"]);
    assert_eq!(tall["status"], "unreviewed");
}

#[test]
fn review_groups_exact_frames_and_marks_component_transfers_as_unreviewed() {
    let temp = tempfile::tempdir().unwrap();
    let (archive, config) = fixture(temp.path());
    let before = fs::read(&archive).unwrap();
    let output = temp.path().join("review");
    success(run(&archive, &config, &output));
    let report: Value =
        serde_json::from_slice(&fs::read(output.join("batch.json")).unwrap()).unwrap();
    assert_eq!(report["assets"].as_array().unwrap().len(), 3);
    assert_eq!(report["groups"].as_array().unwrap().len(), 3);
    assert_eq!(report["summary"]["frames"], 4);
    assert_eq!(report["summary"]["reused_frames"], 3);
    let duplicate = group(&report, "duplicate");
    assert_eq!(duplicate["status"], "reused");
    assert_eq!(duplicate["selected"], json!([0]));
    assert_eq!(duplicate["occurrences"].as_array().unwrap().len(), 2);
    let new = group(&report, "new");
    assert_eq!(new["status"], "suggested");
    assert_eq!(new["selected"], json!([0]));
    // The extra disconnected hand needs review, and same-color trim stays out.
    assert_eq!(new["components"].as_array().unwrap().len(), 3);
    assert_eq!(new["components"][1]["seed"], json!([3, 0]));
    assert_eq!(new["components"][2]["seed"], json!([3, 2]));
    assert!(output.join("index.html").is_file());
    assert!(output.join(duplicate["image"].as_str().unwrap()).is_file());
    let second = temp.path().join("second");
    success(run(&archive, &config, &second));
    assert_eq!(
        fs::read(output.join("batch.json")).unwrap(),
        fs::read(second.join("batch.json")).unwrap()
    );
    assert_eq!(fs::read(&archive).unwrap(), before);
}

#[test]
fn review_rejects_stale_reference_masks_before_writing() {
    let temp = tempfile::tempdir().unwrap();
    let (archive, config) = fixture(temp.path());
    let path = temp.path().join("approved.json");
    let mut profile: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    profile["regions"][0]["source_sha256"] = json!("0".repeat(64));
    fs::write(path, serde_json::to_vec(&profile).unwrap()).unwrap();
    let output = temp.path().join("review");
    let result = run(&archive, &config, &output);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("Region source checksum mismatch"));
    assert!(!output.exists());
}

#[test]
fn conflicting_identical_reference_frames_require_a_new_decision() {
    let temp = tempfile::tempdir().unwrap();
    let (archive, config) = fixture(temp.path());
    let output = temp.path().join("first");
    success(run(&archive, &config, &output));
    let report: Value =
        serde_json::from_slice(&fs::read(output.join("batch.json")).unwrap()).unwrap();
    let duplicate = report["assets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["asset"] == format!("{PREFIX}duplicate.png"))
        .unwrap();
    let path = temp.path().join("approved.json");
    let mut profile: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    profile["regions"].as_array_mut().unwrap().push(json!({
        "asset":duplicate["asset"],"source_sha256":duplicate["source_sha256"],"size":[5,3],"seeds":[[3,2]]
    }));
    fs::write(path, serde_json::to_vec(&profile).unwrap()).unwrap();
    let output = temp.path().join("conflict");
    success(run(&archive, &config, &output));
    let report: Value =
        serde_json::from_slice(&fs::read(output.join("batch.json")).unwrap()).unwrap();
    assert_eq!(group(&report, "duplicate")["status"], "conflict");
    assert_eq!(group(&report, "duplicate")["selected"], json!([]));
    assert_eq!(group(&report, "new")["selected"], json!([]));
    assert_eq!(report["summary"]["conflicting_groups"], 1);
}

#[test]
fn joined_frames_are_rejected_before_exporting_unfaithful_seed_masks() {
    let temp = tempfile::tempdir().unwrap();
    let (archive, config) = fixture(temp.path());
    let mut definition: Value = serde_json::from_slice(&fs::read(&config).unwrap()).unwrap();
    definition
        .as_object_mut()
        .unwrap()
        .remove("reference_profile");
    fs::write(&config, serde_json::to_vec(&definition).unwrap()).unwrap();
    let mut image = RgbaImage::new(10, 3);
    image.put_pixel(4, 1, Rgba([200, 100, 50, 255]));
    image.put_pixel(5, 1, Rgba([200, 100, 50, 255]));
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    zip.start_file(format!("{PREFIX}joined.png"), SimpleFileOptions::default())
        .unwrap();
    zip.write_all(&png(image)).unwrap();
    zip.start_file(
        format!("{PREFIX}joined.meta.toml"),
        SimpleFileOptions::default(),
    )
    .unwrap();
    zip.write_all(b"[asset_properties]\nframe_size=[5,3]\nframe_len=2\n")
        .unwrap();
    zip.finish().unwrap();
    let output = temp.path().join("review");
    let result = run(&archive, &config, &output);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("cross a frame boundary"));
    assert!(!output.exists());
}

#[test]
#[ignore = "requires FOM_REVIEW_BROWSER pointing to a Chromium executable"]
fn browser_preserves_outside_reference_negative_evidence_during_live_suggestions() {
    reference_evidence_browser(false);
}

#[test]
#[ignore = "requires FOM_REVIEW_BROWSER pointing to a Chromium executable"]
fn browser_preserves_conflicting_reference_evidence_until_manually_resolved() {
    reference_evidence_browser(true);
}

fn reference_evidence_browser(conflicting_selected_reference: bool) {
    let temp = tempfile::tempdir().unwrap();
    let reference_name = if conflicting_selected_reference {
        format!("{PREFIX}negative.png")
    } else {
        "assets/animations/NPCs/Reference/negative.png".to_owned()
    };
    let mut reference = RgbaImage::new(3, 1);
    reference.put_pixel(0, 0, Rgba([200, 100, 50, 255]));
    reference.put_pixel(1, 0, Rgba([10, 0, 0, 255]));
    let reference = png(reference);
    let mut first = RgbaImage::new(3, 1);
    first.put_pixel(0, 0, Rgba([200, 100, 50, 255]));
    first.put_pixel(1, 0, Rgba([20, 0, 0, 255]));
    let mut second = RgbaImage::new(3, 1);
    second.put_pixel(0, 0, Rgba([200, 100, 50, 255]));
    second.put_pixel(1, 0, Rgba([30, 0, 0, 255]));

    let archive = temp.path().join("assets.zip");
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    let mut images = vec![
        (reference_name.clone(), reference.clone()),
        (format!("{PREFIX}first.png"), png(first)),
        (format!("{PREFIX}second.png"), png(second)),
    ];
    if conflicting_selected_reference {
        images.push((format!("{PREFIX}positive.png"), reference.clone()));
    }
    for (name, bytes) in images {
        zip.start_file(&name, SimpleFileOptions::default()).unwrap();
        zip.write_all(&bytes).unwrap();
        zip.start_file(
            format!("{}.meta.toml", name.strip_suffix(".png").unwrap()),
            SimpleFileOptions::default(),
        )
        .unwrap();
        zip.write_all(b"[asset_properties]\nframe_size=[3,1]\nframe_len=1\n")
            .unwrap();
    }
    zip.finish().unwrap();

    let mut references = vec![json!({
        "asset":reference_name,
        "source_sha256":format!("{:x}",Sha256::digest(&reference)),
        "size":[3,1],
        "seeds":[]
    })];
    if conflicting_selected_reference {
        references.push(json!({
            "asset":format!("{PREFIX}positive.png"),
            "source_sha256":format!("{:x}",Sha256::digest(&reference)),
            "size":[3,1],
            "seeds":[[0,0]]
        }));
    }
    fs::write(
        temp.path().join("approved.json"),
        serde_json::to_vec(&json!({
            "source_colors":["#C86432"],
            "regions":references
        }))
        .unwrap(),
    )
    .unwrap();
    let config = temp.path().join("review.json");
    fs::write(
        &config,
        serde_json::to_vec(&json!({
            "label":"Outside negative evidence",
            "prefixes":[PREFIX],
            "source_colors":["#C86432"],
            "preview_colors":["#5599FF"],
            "reference_profile":"approved.json"
        }))
        .unwrap(),
    )
    .unwrap();
    let output = temp.path().join("review");
    success(run(&archive, &config, &output));
    let report: Value =
        serde_json::from_slice(&fs::read(output.join("batch.json")).unwrap()).unwrap();
    assert_eq!(
        report["groups"].as_array().unwrap().len(),
        if conflicting_selected_reference { 3 } else { 2 }
    );
    for name in ["first", "second"] {
        let candidate = group(&report, name);
        assert_eq!(candidate["status"], "unreviewed");
        assert_eq!(candidate["selected"], json!([]));
        assert_eq!(candidate["references"], json!([]));
    }
    if conflicting_selected_reference {
        assert_eq!(group(&report, "negative")["status"], "conflict");
    }

    let script = include_str!("review_outside_negative_browser.js");
    let html = output.join("index.html");
    let source = fs::read_to_string(&html).unwrap();
    fs::write(
        &html,
        source.replace("</html>", &format!("<script>{script}</script></html>")),
    )
    .unwrap();
    let browser = std::env::var_os("FOM_REVIEW_BROWSER").expect("Set FOM_REVIEW_BROWSER");
    let result = Command::new(browser)
        .args([
            "--headless",
            "--no-sandbox",
            "--disable-gpu",
            "--disable-dev-shm-usage",
            "--no-first-run",
            "--dump-dom",
            "--virtual-time-budget=5000",
        ])
        .arg(format!(
            "--user-data-dir={}",
            temp.path().join("browser").display()
        ))
        .arg(format!("file://{}", html.display()))
        .output()
        .unwrap();
    let dom = String::from_utf8(result.stdout).unwrap();
    assert!(
        result.status.success() && dom.contains("data-test-result=\"passed\""),
        "{}\n{}",
        dom,
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
#[ignore = "requires FOM_REVIEW_BROWSER pointing to a Chromium executable"]
fn browser_edits_masks_exports_only_reviewed_strips_and_roundtrips_progress() {
    let temp = tempfile::tempdir().unwrap();
    let (archive, config) = fixture(temp.path());
    let output = temp.path().join("review");
    success(run(&archive, &config, &output));
    let script = include_str!("review_browser.js");
    let html = output.join("index.html");
    let source = fs::read_to_string(&html).unwrap();
    fs::write(
        &html,
        source.replace("</html>", &format!("<script>{script}</script></html>")),
    )
    .unwrap();
    let browser = std::env::var_os("FOM_REVIEW_BROWSER").expect("Set FOM_REVIEW_BROWSER");
    let result = Command::new(browser)
        .args([
            "--headless",
            "--no-sandbox",
            "--disable-gpu",
            "--disable-dev-shm-usage",
            "--no-first-run",
            "--dump-dom",
            "--virtual-time-budget=5000",
        ])
        .arg(format!(
            "--user-data-dir={}",
            temp.path().join("browser").display()
        ))
        .arg(format!("file://{}", html.display()))
        .output()
        .unwrap();
    let dom = String::from_utf8(result.stdout).unwrap();
    assert!(
        result.status.success() && dom.contains("data-test-result=\"passed\""),
        "{}\n{}",
        dom,
        String::from_utf8_lossy(&result.stderr)
    );
    // Consume the actual browser-exported profile through the existing Rust pipeline.
    let serialized = dom
        .split("<pre id=\"exported-profile\">")
        .nth(1)
        .unwrap()
        .split("</pre>")
        .next()
        .unwrap();
    let profile: Value = serde_json::from_str(serialized).unwrap();
    let original = temp.path().join("original");
    let mut export = Command::new(env!("CARGO_BIN_EXE_mistria-palette"));
    export
        .args(["export", "--archive"])
        .arg(&archive)
        .arg("--output")
        .arg(&original);
    for name in ["source", "duplicate", "new"] {
        export.arg("--asset").arg(format!("{PREFIX}{name}.png"));
    }
    success(export.output().unwrap());
    let recipe = temp.path().join("recipe.json");
    fs::write(
        &recipe,
        serde_json::to_vec(&json!({"rgba_map":{"#C86432":"#5599FF"},"regions":profile["regions"]}))
            .unwrap(),
    )
    .unwrap();
    let modified = temp.path().join("modified");
    success(
        Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["apply", "--input"])
            .arg(&original)
            .arg("--palette")
            .arg(&recipe)
            .arg("--output")
            .arg(&modified)
            .output()
            .unwrap(),
    );
    let image = image::open(modified.join(format!("{PREFIX}new.png")))
        .unwrap()
        .to_rgba8();
    assert_eq!(image.get_pixel(1, 0).0, [85, 153, 255, 255]);
    assert_eq!(image.get_pixel(3, 0).0, [85, 153, 255, 255]);
    assert_eq!(image.get_pixel(3, 2).0, [200, 100, 50, 255]);
}

#[test]
#[ignore = "requires the mounted game archive and the 143 local Adeline animations"]
fn batch_reuse_reproduces_every_approved_adeline_mask() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("review");
    success(run(
        &root.join("tmp/fields-of-mistria/assets.zip"),
        &root.join("palettes/review/adeline.json"),
        &output,
    ));
    let report: Value =
        serde_json::from_slice(&fs::read(output.join("batch.json")).unwrap()).unwrap();
    let groups: std::collections::BTreeMap<_, _> = report["groups"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| (g["id"].as_str().unwrap(), g))
        .collect();
    let mut regions = Vec::new();
    for asset in report["assets"].as_array().unwrap() {
        let frames = asset["frames"].as_array().unwrap();
        if !frames
            .iter()
            .all(|id| groups[id.as_str().unwrap()]["status"] == "reused")
        {
            continue;
        }
        let mut seeds = Vec::new();
        for (frame, id) in frames.iter().enumerate() {
            let group = groups[id.as_str().unwrap()];
            for index in group["selected"].as_array().unwrap() {
                let seed = &group["components"][index.as_u64().unwrap() as usize]["seed"];
                seeds.push(json!([
                    seed[0].as_u64().unwrap()
                        + frame as u64 * asset["frame_size"][0].as_u64().unwrap(),
                    seed[1]
                ]));
            }
        }
        regions.push(json!({"asset":asset["asset"],"source_sha256":asset["source_sha256"],"size":asset["size"],"seeds":seeds}));
    }
    assert_eq!(regions.len(), 143);
    let mut recipe: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/stylized/adeline-world-actions.json")).unwrap(),
    )
    .unwrap();
    recipe.as_object_mut().unwrap().remove("profile");
    recipe["regions"] = json!(regions);
    let path = temp.path().join("reused.json");
    fs::write(&path, serde_json::to_vec(&recipe).unwrap()).unwrap();
    let original = root.join("extracted/adeline-world-actions-study");
    let modified = temp.path().join("recolored");
    success(
        Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["apply", "--input"])
            .arg(&original)
            .arg("--palette")
            .arg(path)
            .arg("--output")
            .arg(&modified)
            .output()
            .unwrap(),
    );
    success(
        Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["validate", "--original"])
            .arg(original)
            .arg("--modified")
            .arg(modified)
            .arg("--palette")
            .arg(root.join("palettes/stylized/adeline-world-actions.json"))
            .output()
            .unwrap(),
    );
}
