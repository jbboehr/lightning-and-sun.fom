use serde_json::Value;
use std::{fs, path::Path, process::Command};

fn color(text: &str) -> [u8; 4] {
    let text = text.trim_start_matches('#');
    let mut result = [255; 4];
    for (index, channel) in result.iter_mut().enumerate().take(text.len() / 2) {
        *channel = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).unwrap();
    }
    result
}

fn build(source: &Path, set: &Path, output: &Path) {
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("build-presets")
        .arg("--original")
        .arg(source)
        .arg("--presets")
        .arg(set)
        .arg("--output")
        .arg(output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
#[ignore = "requires all 278 Reina strips in extracted/test-corpus/reina"]
fn reina_outer_mouth_skin_recolors_without_changing_lips_or_world_art() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let historical_set = root.join("palettes/sets/reina-world-trial.json");
    let candidate_set = std::env::var_os("FOM_REINA_CORRECTED_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/reina-mouth-corrected-trial.json"));
    let historical: Value = serde_json::from_slice(&fs::read(&historical_set).unwrap()).unwrap();
    let candidate: Value = serde_json::from_slice(&fs::read(&candidate_set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(
            historical_set
                .parent()
                .unwrap()
                .join(historical["profile"].as_str().unwrap()),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(profile["regions"].as_array().unwrap().len(), 278);
    assert_eq!(candidate["presets"].as_array().unwrap().len(), 4);
    let source = root.join("extracted/test-corpus/reina");
    let temp = tempfile::tempdir().unwrap();
    let before_bundle = temp.path().join("before");
    let after_bundle = temp.path().join("after");
    build(&source, &historical_set, &before_bundle);
    build(&source, &candidate_set, &after_bundle);
    let mut corrected = 0;
    let mut changed_portraits = 0;
    let mut unchanged_world = 0;
    let mut protected_mouth = 0;
    let mut metadata = 0;
    for preset in candidate["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let light = color(preset["colors"][0].as_str().unwrap());
        assert_ne!(light, [178, 104, 74, 255]);
        for region in profile["regions"].as_array().unwrap() {
            let asset = region["asset"].as_str().unwrap();
            let original = image::open(source.join(asset)).unwrap().to_rgba8();
            let before_path = before_bundle.join("variants").join(id).join(asset);
            let after_path = after_bundle.join("variants").join(id).join(asset);
            let before = image::open(&before_path).unwrap().to_rgba8();
            let after = image::open(&after_path).unwrap().to_rgba8();
            assert_eq!(original.dimensions(), after.dimensions());
            assert_eq!(before.dimensions(), after.dimensions());
            let is_portrait = asset.contains("/Portraits/");
            let mut changes = 0;
            for (x, y, pixel) in original.enumerate_pixels() {
                let previous = before.get_pixel(x, y);
                let actual = after.get_pixel(x, y);
                assert_eq!(pixel[3], actual[3], "alpha: {asset} [{x},{y}] {id}");
                // The source-inspected fringe lies outside the drawn lip/interior,
                // only in the speaking frame. The literal shade also guards against
                // a seed-generation bug silently changing the expected mask.
                if pixel.0 == [178, 104, 74, 255] {
                    assert!(
                        is_portrait && (438..=443).contains(&x) && (86..=90).contains(&y),
                        "unreviewed use of fringe shade: {asset} [{x},{y}]"
                    );
                    assert_eq!(
                        previous, pixel,
                        "historical fringe changed: {asset} [{x},{y}]"
                    );
                    assert_eq!(
                        actual.0, light,
                        "outer-mouth skin left warm: {asset} [{x},{y}] {id}"
                    );
                    corrected += 1;
                    changes += 1;
                } else {
                    assert_eq!(
                        actual, previous,
                        "unrelated pixel changed: {asset} [{x},{y}] {id}"
                    );
                }
                if is_portrait
                    && (138..163).contains(&(x % 296))
                    && (78..93).contains(&y)
                    && [
                        [88, 29, 31, 255],
                        [118, 46, 33, 255],
                        [148, 26, 51, 255],
                        [202, 77, 103, 255],
                        [172, 60, 118, 255],
                        [204, 82, 145, 255],
                    ]
                    .contains(&pixel.0)
                {
                    assert_eq!(
                        actual, pixel,
                        "mouth interior/lip changed: {asset} [{x},{y}] {id}"
                    );
                    protected_mouth += 1;
                }
            }
            if is_portrait {
                assert!(changes > 0, "portrait fringe absent: {asset}");
                changed_portraits += 1;
            } else {
                assert_eq!(changes, 0);
                assert_eq!(
                    fs::read(before_path).unwrap(),
                    fs::read(after_path).unwrap(),
                    "world PNG bytes changed: {asset} {id}"
                );
                unchanged_world += 1;
            }
            let meta = asset.replace(".png", ".meta.toml");
            let current_meta =
                fs::read(after_bundle.join("variants").join(id).join(&meta)).unwrap();
            assert_eq!(current_meta, fs::read(source.join(&meta)).unwrap());
            assert_eq!(
                current_meta,
                fs::read(before_bundle.join("variants").join(id).join(&meta)).unwrap()
            );
            metadata += 1;
        }
    }
    assert_eq!(corrected, 1700);
    assert_eq!(changed_portraits, 412);
    assert_eq!(unchanged_world, 700);
    assert_eq!(metadata, 1112);
    assert!(protected_mouth > 2000);
    println!(
        "425 fringe pixels / 103 speaking frames corrected in each of four palettes; 700 world PNGs and 1,112 metadata files byte-identical; {protected_mouth} protected lip/interior assertions"
    );
}
