use image::{Rgba, RgbaImage};
use serde_json::{Value, json};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use tempfile::TempDir;
use zip::{ZipWriter, write::SimpleFileOptions};

struct Fixture {
    _temp: TempDir,
    root: PathBuf,
    original: PathBuf,
    output: PathBuf,
    palette: PathBuf,
}

fn cli(command: &str, args: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg(command)
        .args(args)
        .output()
        .unwrap()
}

fn ok(output: Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn rejected(output: Output) {
    assert!(
        !output.status.success(),
        "unexpected success: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("error:"));
}

fn files(root: &Path) -> Vec<String> {
    let mut names: Vec<_> = walkdir::WalkDir::new(root)
        .into_iter()
        .map(Result::unwrap)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| {
            entry
                .path()
                .strip_prefix(root)
                .unwrap()
                .to_str()
                .unwrap()
                .replace('\\', "/")
        })
        .collect();
    names.sort();
    names
}

impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().to_owned();
        let original = root.join("original");
        let output = root.join("modified");
        let palette = root.join("palette.json");
        fs::create_dir_all(original.join("nested")).unwrap();
        let image = RgbaImage::from_raw(
            4,
            1,
            vec![
                16, 32, 48, 255, 64, 80, 96, 255, 16, 32, 48, 128, 16, 32, 48, 0,
            ],
        )
        .unwrap();
        image.save(original.join("nested/spr_test.png")).unwrap();
        fs::write(
            original.join("nested/spr_test.meta.toml"),
            "[asset_properties]\nframe_size = [2, 1]\nframe_len = 2\n",
        )
        .unwrap();
        fs::write(&palette, json!({"rgba_map": {"#102030":"#405060", "#405060":"#708090", "#10203080":"#11223380"}}).to_string()).unwrap();
        Self {
            _temp: temp,
            root,
            original,
            output,
            palette,
        }
    }

    fn apply_to(&self, output: &Path) -> Output {
        cli(
            "apply",
            &[
                Path::new("--input"),
                &self.original,
                Path::new("--palette"),
                &self.palette,
                Path::new("--output"),
                output,
            ],
        )
    }

    fn apply(&self) -> Value {
        ok(self.apply_to(&self.output))
    }

    fn validate(&self) -> Output {
        cli(
            "validate",
            &[
                Path::new("--original"),
                &self.original,
                Path::new("--modified"),
                &self.output,
            ],
        )
    }

    fn package(&self, output: &Path) -> Output {
        cli(
            "package",
            &[
                Path::new("--original"),
                &self.original,
                Path::new("--modified"),
                &self.output,
                Path::new("--output"),
                output,
            ],
        )
    }

    fn add_sprite(&self, name: &str) {
        fs::copy(
            self.original.join("nested/spr_test.png"),
            self.original.join(format!("{name}.png")),
        )
        .unwrap();
        fs::copy(
            self.original.join("nested/spr_test.meta.toml"),
            self.original.join(format!("{name}.meta.toml")),
        )
        .unwrap();
    }
}

#[test]
fn exact_simultaneous_mapping_preserves_alpha_paths_metadata_and_source() {
    let f = Fixture::new();
    let before = fs::read(f.original.join("nested/spr_test.png")).unwrap();
    let report = f.apply();
    assert_eq!(report["changed_pixels"], 3);
    assert_eq!(report["files"][0]["changed_by_source"]["#102030FF"], 1);
    assert_eq!(report["files"][0]["path"], "nested/spr_test.png");
    let image = image::open(f.output.join("nested/spr_test.png"))
        .unwrap()
        .to_rgba8();
    assert_eq!(image.dimensions(), (4, 1));
    assert_eq!(
        image.into_raw(),
        vec![
            64, 80, 96, 255, 112, 128, 144, 255, 17, 34, 51, 128, 16, 32, 48, 0
        ]
    );
    assert_eq!(
        fs::read(f.original.join("nested/spr_test.png")).unwrap(),
        before
    );
    assert_eq!(
        fs::read(f.original.join("nested/spr_test.meta.toml")).unwrap(),
        fs::read(f.output.join("nested/spr_test.meta.toml")).unwrap()
    );
    let repeat = f.root.join("repeat");
    assert_eq!(ok(f.apply_to(&repeat)), report);
    for name in files(&f.output) {
        assert_eq!(
            fs::read(f.output.join(&name)).unwrap(),
            fs::read(repeat.join(name)).unwrap()
        );
    }
}

#[test]
fn vanilla_keeps_original_png_bytes_and_package_has_only_manifest() {
    let f = Fixture::new();
    fs::write(&f.palette, r#"{"rgba_map":{}}"#).unwrap();
    assert_eq!(f.apply()["changed_pixels"], 0);
    assert_eq!(
        fs::read(f.original.join("nested/spr_test.png")).unwrap(),
        fs::read(f.output.join("nested/spr_test.png")).unwrap()
    );
    let package = f.root.join("package");
    assert_eq!(ok(f.package(&package))["replacements"], json!([]));
    assert_eq!(files(&package), ["manifest.toml"]);
}

#[test]
fn invalid_maps_do_not_create_output() {
    for text in [
        r##"{"rgba_map":{"#102030":"#40506080"}}"##,
        r##"{"rgba_map":{"#102030":"#405060","#102030ff":"#708090"}}"##,
        r##"{"rgba_map":{"#102030":"#405060","#102030":"#708090"}}"##,
        r##"{"rgba_map":{"#10203000":"#70809000"}}"##,
        r##"{"rgba_map":{"#ZZ2233":"#708090"}}"##,
        r#"{}"#,
    ] {
        let f = Fixture::new();
        fs::write(&f.palette, text).unwrap();
        rejected(f.apply_to(&f.output));
        assert!(!f.output.exists());
    }
}

#[test]
fn existing_and_overlapping_output_are_rejected_without_source_changes() {
    let f = Fixture::new();
    let before = fs::read(f.original.join("nested/spr_test.png")).unwrap();
    for output in [&f.original, &f.root, &f.original.join("generated")] {
        rejected(f.apply_to(output));
        assert_eq!(
            fs::read(f.original.join("nested/spr_test.png")).unwrap(),
            before
        );
    }
}

#[test]
fn invalid_later_png_does_not_leave_partial_output() {
    let f = Fixture::new();
    fs::write(f.original.join("zz-broken.png"), b"not an image").unwrap();
    rejected(f.apply_to(&f.output));
    assert!(!f.output.exists());
}

#[test]
fn reserved_report_path_collision_does_not_leave_partial_output() {
    let f = Fixture::new();
    fs::rename(
        f.original.join("nested"),
        f.original.join("palette-report.json"),
    )
    .unwrap();

    rejected(f.apply_to(&f.output));
    assert!(
        !f.output.exists(),
        "a rejected input must not leave a partial output tree"
    );
}

#[test]
fn validation_detects_image_and_metadata_mismatches() {
    let f = Fixture::new();
    f.apply();
    ok(f.validate());
    let png = f.output.join("nested/spr_test.png");
    let bytes = fs::read(&png).unwrap();
    fs::remove_file(&png).unwrap();
    rejected(f.validate());
    fs::write(&png, &bytes).unwrap();
    fs::write(f.output.join("extra.png"), &bytes).unwrap();
    rejected(f.validate());
    fs::remove_file(f.output.join("extra.png")).unwrap();
    RgbaImage::new(1, 1).save(&png).unwrap();
    rejected(f.validate());
    RgbaImage::from_pixel(4, 1, Rgba([0, 0, 0, 255]))
        .save(&png)
        .unwrap();
    rejected(f.validate());
    fs::write(&png, &bytes).unwrap();
    let meta = f.output.join("nested/spr_test.meta.toml");
    let bytes = fs::read(&meta).unwrap();
    fs::remove_file(&meta).unwrap();
    rejected(f.validate());
    fs::write(&meta, &bytes).unwrap();
    fs::write(f.output.join("extra.meta.toml"), "extra").unwrap();
    rejected(f.validate());
    fs::remove_file(f.output.join("extra.meta.toml")).unwrap();
    fs::write(&meta, "changed").unwrap();
    rejected(f.validate());
}

#[test]
fn contact_sheet_reports_pairs_and_uses_nearest_neighbor_at_both_zooms() {
    let f = Fixture::new();
    f.apply();
    for zoom in [4u32, 8] {
        let sheet = f.root.join(format!("sheet-{zoom}.png"));
        let report = ok(cli(
            "contact-sheet",
            &[
                Path::new("--original"),
                &f.original,
                Path::new("--modified"),
                &f.output,
                Path::new("--output"),
                &sheet,
                Path::new("--zoom"),
                Path::new(&zoom.to_string()),
            ],
        ));
        let written: Value =
            serde_json::from_slice(&fs::read(sheet.with_extension("json")).unwrap()).unwrap();
        assert_eq!(written, report);
        assert_eq!(report["files"][0]["path"], "nested/spr_test.png");
        let image = image::open(&sheet).unwrap().to_rgb8();
        for (key, color) in [
            ("original_box", [16, 32, 48]),
            ("modified_box", [64, 80, 96]),
        ] {
            let bounds = &report["files"][0][key];
            assert_eq!(bounds[2], 4 * zoom);
            assert_eq!(bounds[3], zoom);
            let x = bounds[0].as_u64().unwrap() as u32;
            let y = bounds[1].as_u64().unwrap() as u32;
            for dy in 0..zoom {
                for dx in 0..zoom {
                    assert_eq!(image.get_pixel(x + dx, y + dy).0, color);
                }
            }
            assert!(
                image
                    .view(x, 0, 60, y)
                    .pixels()
                    .any(|(_, _, p)| p.0[..3] == [0, 0, 0])
            );
        }
    }
}

#[test]
fn export_selects_only_requested_assets_and_leaves_archive_intact() {
    let f = Fixture::new();
    let archive = f.root.join("assets.zip");
    let members = [
        "assets/animations/NPCs/Example/spr_one.png",
        "assets/animations/NPCs/Example/spr_two.png",
        "assets/animations/NPCs/Example/spr_three.png",
    ];
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    for member in members {
        zip.start_file(member, SimpleFileOptions::default())
            .unwrap();
        zip.write_all(&fs::read(f.original.join("nested/spr_test.png")).unwrap())
            .unwrap();
        zip.start_file(
            member.replace(".png", ".meta.toml"),
            SimpleFileOptions::default(),
        )
        .unwrap();
        zip.write_all(&fs::read(f.original.join("nested/spr_test.meta.toml")).unwrap())
            .unwrap();
    }
    zip.finish().unwrap();
    let before = fs::read(&archive).unwrap();
    for count in 1..=3 {
        let output = f.root.join(format!("export-{count}"));
        let mut args = vec![
            Path::new("--archive"),
            &archive,
            Path::new("--output"),
            &output,
        ];
        for member in &members[..count] {
            args.extend([Path::new("--asset"), Path::new(member)]);
        }
        let result = cli("export", &args);
        let report = ok(result);
        assert_eq!(report["files"].as_array().unwrap().len(), count);
        for member in &members[..count] {
            assert_eq!(
                fs::read(output.join(member)).unwrap(),
                fs::read(f.original.join("nested/spr_test.png")).unwrap()
            );
        }
        if count < 3 {
            assert!(!output.join(members[2]).exists());
        }
        assert_eq!(fs::read(&archive).unwrap(), before);
    }
}

#[test]
fn export_file_directory_collision_does_not_leave_partial_output() {
    let f = Fixture::new();
    let archive = f.root.join("assets.zip");
    let members = ["assets/foo.png", "assets/foo.png/bar.png"];
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    for member in members {
        zip.start_file(member, SimpleFileOptions::default())
            .unwrap();
        zip.write_all(&fs::read(f.original.join("nested/spr_test.png")).unwrap())
            .unwrap();
        zip.start_file(
            Path::new(member)
                .with_extension("meta.toml")
                .to_str()
                .unwrap(),
            SimpleFileOptions::default(),
        )
        .unwrap();
        zip.write_all(&fs::read(f.original.join("nested/spr_test.meta.toml")).unwrap())
            .unwrap();
    }
    zip.finish().unwrap();
    let parent = f.root.join("new-parent");
    let output = parent.join("export");
    rejected(cli(
        "export",
        &[
            Path::new("--archive"),
            &archive,
            Path::new("--asset"),
            Path::new(members[0]),
            Path::new("--asset"),
            Path::new(members[1]),
            Path::new("--output"),
            &output,
        ],
    ));
    assert!(
        !parent.exists(),
        "a rejected input must not create output or parent directories"
    );
}

#[test]
fn package_contains_only_manifest_and_changed_pngs() {
    let f = Fixture::new();
    f.apply();
    let package = f.root.join("package");
    let report = ok(f.package(&package));
    assert_eq!(report["replacements"], json!(["spr_test.png"]));
    assert_eq!(
        files(&package),
        ["images/replace/spr_test.png", "manifest.toml"]
    );
    assert_eq!(
        fs::read(package.join("images/replace/spr_test.png")).unwrap(),
        fs::read(f.output.join("nested/spr_test.png")).unwrap()
    );
    assert_eq!(
        fs::read(package.join("manifest.toml")).unwrap(),
        include_bytes!("../mod/manifest.toml")
    );
}

#[test]
fn package_accepts_two_changes_and_rejects_three_before_writing() {
    for count in [2, 3] {
        let f = Fixture::new();
        for i in 1..count {
            f.add_sprite(&format!("spr_{i}"));
        }
        f.apply();
        let package = f.root.join("package");
        let result = f.package(&package);
        if count == 2 {
            assert_eq!(ok(result)["replacements"].as_array().unwrap().len(), 2);
        } else {
            rejected(result);
            assert!(!package.exists());
        }
    }
}

#[test]
fn package_checks_strip_frame_metadata() {
    let f = Fixture::new();
    fs::write(
        f.original.join("nested/spr_test.meta.toml"),
        "[asset_properties]\nframe_size=[2,1]\nframe_len=3\n",
    )
    .unwrap();
    f.apply();
    let package = f.root.join("package");
    rejected(f.package(&package));
    assert!(!package.exists());
}

use image::GenericImageView;

#[test]
fn duplicate_archive_member_is_rejected_before_export() {
    let f = Fixture::new();
    let archive = f.root.join("duplicate.zip");
    // The writer disallows duplicate names. Rename equal-length members in both
    // ZIP headers after writing; the payloads and their checksums stay intact.
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    for name in [
        "assets/spr_aaa.png",
        "assets/spr_bbb.png",
        "assets/spr_aaa.meta.toml",
    ] {
        zip.start_file(name, SimpleFileOptions::default()).unwrap();
        zip.write_all(&fs::read(f.original.join("nested/spr_test.png")).unwrap())
            .unwrap();
    }
    zip.finish().unwrap();
    let mut data = fs::read(&archive).unwrap();
    let old = b"assets/spr_bbb.png";
    for start in 0..=data.len() - old.len() {
        if &data[start..start + old.len()] == old {
            data[start..start + old.len()].copy_from_slice(b"assets/spr_aaa.png");
        }
    }
    fs::write(&archive, data).unwrap();
    let output = f.root.join("export");
    rejected(cli(
        "export",
        &[
            Path::new("--archive"),
            &archive,
            Path::new("--asset"),
            Path::new("assets/spr_aaa.png"),
            Path::new("--output"),
            &output,
        ],
    ));
    assert!(!output.exists());
}

#[test]
fn png_color_formats_preserve_decoded_rgba() {
    for (color, data, palette, transparency, expected) in [
        (
            png::ColorType::Indexed,
            vec![0, 1],
            Some(vec![16, 32, 48, 64, 80, 96]),
            Some(vec![255, 0]),
            vec![64, 80, 96, 255, 64, 80, 96, 0],
        ),
        (
            png::ColorType::Rgb,
            vec![16, 32, 48, 64, 80, 96],
            None,
            None,
            vec![64, 80, 96, 255, 112, 128, 144, 255],
        ),
        (
            png::ColorType::GrayscaleAlpha,
            vec![80, 128, 32, 0],
            None,
            None,
            vec![80, 80, 80, 128, 32, 32, 32, 0],
        ),
        (
            png::ColorType::Grayscale,
            vec![32, 80],
            None,
            None,
            vec![32, 32, 32, 255, 80, 80, 80, 255],
        ),
    ] {
        let f = Fixture::new();
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut bytes, 2, 1);
            encoder.set_color(color);
            encoder.set_depth(png::BitDepth::Eight);
            if let Some(palette) = palette {
                encoder.set_palette(palette);
            }
            if let Some(transparency) = transparency {
                encoder.set_trns(transparency);
            }
            encoder
                .write_header()
                .unwrap()
                .write_image_data(&data)
                .unwrap();
        }
        fs::write(f.original.join("nested/spr_test.png"), bytes).unwrap();
        f.apply();
        assert_eq!(
            image::open(f.output.join("nested/spr_test.png"))
                .unwrap()
                .to_rgba8()
                .into_raw(),
            expected
        );
    }
}

#[cfg(unix)]
#[test]
fn symlink_inputs_and_output_aliases_are_rejected() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new();
    let alias = f.root.join("alias");
    symlink(&f.original, &alias).unwrap();
    rejected(f.apply_to(&alias.join("output")));
    symlink(f.root.join("missing"), &f.output).unwrap();
    rejected(f.apply_to(&f.output));
    fs::remove_file(&f.output).unwrap();
    symlink(
        f.original.join("nested/spr_test.png"),
        f.original.join("link.png"),
    )
    .unwrap();
    rejected(f.apply_to(&f.output));
    assert!(!f.output.exists());
}

#[test]
fn animated_png_is_rejected_without_output() {
    let f = Fixture::new();
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, 1, 1);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_animated(2, 0).unwrap();
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&[16, 32, 48, 255]).unwrap();
        writer.write_image_data(&[64, 80, 96, 255]).unwrap();
    }
    fs::write(f.original.join("nested/spr_test.png"), bytes).unwrap();
    rejected(f.apply_to(&f.output));
    assert!(!f.output.exists());
}
