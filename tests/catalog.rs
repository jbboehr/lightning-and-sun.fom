use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use serde_json::Value;
use std::{
    fs,
    io::{Cursor, Write},
    process::Command,
};
use zip::{ZipWriter, write::SimpleFileOptions};
fn png(image: RgbaImage) -> Vec<u8> {
    let mut out = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(image)
        .write_to(&mut out, ImageFormat::Png)
        .unwrap();
    out.into_inner()
}
#[test]
fn catalog_preserves_creator_entries_and_keeps_portrait_colors_unclassified() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("assets.zip");
    let output = temp.path().join("catalog");
    let mut lut = RgbaImage::from_pixel(4, 256, Rgba([0, 0, 0, 255]));
    for (row, rgb) in [
        (225, [250, 230, 220]),
        (186, [220, 170, 150]),
        (125, [150, 100, 80]),
        (82, [80, 60, 50]),
    ] {
        for x in [1, 2] {
            lut.put_pixel(x, row, Rgba([rgb[0], rgb[1], rgb[2], 255]));
        }
        lut.put_pixel(3, row, Rgba([70, 50, 30, 255]));
    }
    let mut zip = ZipWriter::new(fs::File::create(&archive).unwrap());
    for (name, data) in [
        (
            "assets/animations/Player/Base/spr_player_base_lut.png",
            png(lut),
        ),
        (
            "assets/animations/NPCs/Test/Portraits/Spring/spr_portrait_test_spring_neutral.png",
            png(RgbaImage::from_raw(
                3,
                1,
                vec![250, 230, 220, 255, 250, 230, 220, 255, 250, 230, 220, 0],
            )
            .unwrap()),
        ),
    ] {
        zip.start_file(name, SimpleFileOptions::default()).unwrap();
        zip.write_all(&data).unwrap();
    }
    zip.finish().unwrap();
    let before = fs::read(&archive).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("catalog")
        .arg("--archive")
        .arg(&archive)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value =
        serde_json::from_slice(&fs::read(output.join("catalog.json")).unwrap()).unwrap();
    assert_eq!(report["player_presets"].as_array().unwrap().len(), 3);
    assert_eq!(
        report["player_presets"][0]["colors"],
        serde_json::json!(["#FAE6DCFF", "#DCAA96FF", "#966450FF", "#503C32FF"])
    );
    assert_eq!(report["distinct_player_ramps"], 2);
    assert_eq!(report["player_presets"][1]["same_ramp_as"], "player_01");
    assert_eq!(report["portraits"].as_array().unwrap().len(), 1);
    assert_eq!(report["portraits"][0]["status"], "unreviewed_colors");
    assert_eq!(report["portraits"][0]["opaque_colors"]["#FAE6DCFF"], 2);
    assert_eq!(
        report["portraits"][0]["opaque_colors"]
            .as_object()
            .unwrap()
            .len(),
        1
    );
    assert!(output.join("player-palettes.png").is_file());
    assert_eq!(fs::read(&archive).unwrap(), before);
}
