use image::{Rgba, RgbaImage};
use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

const EXPRESSIONS: &str = "angry_blush blush cartoon_embarrassed concerned embarrassed embarrassed_tired evasive_tired gloomy_special happy happy_blush hope_special mad neutral neutral_tired sad shocked sick_eyes_closed sick_eyes_open sick_smile sick_think sigh sly think ugh wink";
const BEACH_EXPRESSIONS: &str = "angry_blush bath_neutral blush cartoon_embarrassed concerned embarrassed gloomy_special happy happy_blush hope_special mad neutral sad shocked sigh sly think ugh wink";
const WEDDING_EXPRESSIONS: &str = "embarrassed happy_blush hope_special neutral sad sly think";

fn fixture(root: &Path, season: &str, expression: &str, atlas: &str, color: [u8; 4]) {
    fs::create_dir_all(root).unwrap();
    let name = format!("spr_portrait_adeline_{season}_{expression}");
    RgbaImage::from_pixel(4, 1, Rgba(color))
        .save(root.join(format!("{name}.png")))
        .unwrap();
    fs::write(root.join(format!("{name}.meta.toml")), format!("[meta_properties]\nid='{name}'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[2,1]\nframe_len=2\natlas='{atlas}'\nduration=0.2\n")).unwrap();
}

#[test]
fn complete_package_keeps_all_126_portraits_in_their_source_atlas() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    let output = temp.path().join("package");
    for (season, atlas) in [
        ("spring", "PortraitsSpring"),
        ("summer", "PortraitsSummer"),
        ("autumn", "PortraitsAutumn"),
        ("winter", "PortraitsWinter"),
        ("beach", "PortraitsSummer"),
        ("wedding", "PortraitsMisc"),
    ] {
        let expressions = match season {
            "beach" => BEACH_EXPRESSIONS,
            "wedding" => WEDDING_EXPRESSIONS,
            _ => EXPRESSIONS,
        };
        for expression in expressions.split_whitespace() {
            fixture(&original, season, expression, atlas, [10, 20, 30, 255]);
            fixture(&modified, season, expression, atlas, [40, 50, 60, 255]);
        }
    }
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
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["variants"].as_array().unwrap().len(), 126);
    for (season, atlas) in [
        ("spring", "PortraitsSpring"),
        ("summer", "PortraitsSummer"),
        ("autumn", "PortraitsAutumn"),
        ("winter", "PortraitsWinter"),
        ("beach", "PortraitsSummer"),
        ("wedding", "PortraitsMisc"),
    ] {
        let expressions = match season {
            "beach" => BEACH_EXPRESSIONS,
            "wedding" => WEDDING_EXPRESSIONS,
            _ => EXPRESSIONS,
        };
        for expression in expressions.split_whitespace() {
            let path = output.join(format!(
                "animations/LightningAndSun/spr_lns_adeline_{season}_{expression}_blue"
            ));
            let meta: toml::Value =
                toml::from_str(&fs::read_to_string(path.with_extension("meta.toml")).unwrap())
                    .unwrap();
            assert_eq!(meta["asset_properties"]["atlas"].as_str(), Some(atlas));
            assert!(meta["meta_properties"].get("id").is_none());
            assert_eq!(
                image::open(path.with_extension("png")).unwrap().to_rgba8(),
                RgbaImage::from_pixel(4, 1, Rgba([40, 50, 60, 255]))
            );
        }
    }
    let script = fs::read_to_string(output.join("gml/palette_assets.gml")).unwrap();
    let table = script
        .split_once("return ")
        .unwrap()
        .1
        .split_once("; }")
        .unwrap()
        .0;
    let groups: Value = serde_json::from_str(table).unwrap();
    assert_eq!(groups.as_array().unwrap().len(), 126);
    assert!(groups.as_array().unwrap().contains(&json!([
        "spr_portrait_adeline_beach_bath_neutral",
        "spr_lns_adeline_beach_bath_neutral_blue"
    ])));
    assert!(groups.as_array().unwrap().contains(&json!([
        "spr_portrait_adeline_wedding_happy_blush",
        "spr_lns_adeline_wedding_happy_blush_blue"
    ])));
    assert!(groups.as_array().unwrap().contains(&json!([
        "spr_portrait_adeline_spring_neutral",
        "spr_lns_adeline_spring_neutral_blue"
    ])));
}

#[test]
fn packaging_rejects_a_season_with_the_other_seasons_atlas() {
    for (season, atlas) in [
        ("summer", "PortraitsSpring"),
        ("spring", "PortraitsSummer"),
        ("autumn", "PortraitsWinter"),
        ("winter", "PortraitsAutumn"),
        ("beach", "PortraitsSpring"),
        ("wedding", "PortraitsSummer"),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("original");
        let modified = temp.path().join("modified");
        let output = temp.path().join("package");
        fixture(&original, season, "neutral", atlas, [10, 20, 30, 255]);
        fixture(&modified, season, "neutral", atlas, [40, 50, 60, 255]);
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
        assert!(!result.status.success());
        assert!(!output.exists());
    }
}

#[test]
fn packaging_rejects_expressions_absent_from_the_selected_outfit() {
    for (outfit, expression) in [
        ("beach", "sick_smile"),
        ("beach", "neutral_tired"),
        ("summer", "bath_neutral"),
        ("wedding", "happy"),
        ("wedding", "bath_neutral"),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("original");
        let modified = temp.path().join("modified");
        let output = temp.path().join("package");
        fixture(
            &original,
            outfit,
            expression,
            if outfit == "wedding" {
                "PortraitsMisc"
            } else {
                "PortraitsSummer"
            },
            [10, 20, 30, 255],
        );
        fixture(
            &modified,
            outfit,
            expression,
            if outfit == "wedding" {
                "PortraitsMisc"
            } else {
                "PortraitsSummer"
            },
            [40, 50, 60, 255],
        );
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
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("Unsupported Adeline"));
        assert!(!output.exists());
    }
}

#[test]
#[ignore = "requires the 50 local portraits in extracted/adeline-seasonal-study"]
fn seasonal_masks_cover_exposed_skin_and_preserve_summer_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/adeline-seasonal-study");
    let recipe = std::env::var_os("FOM_SEASONAL_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/adeline-spring-summer.json"));
    let temp = tempfile::tempdir().unwrap();
    let modified = temp.path().join("modified");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("apply")
        .arg("--input")
        .arg(&original)
        .arg("--palette")
        .arg(recipe)
        .arg("--output")
        .arg(&modified)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    let files = report["files"].as_array().unwrap();
    assert_eq!(files.len(), 50);
    let mut summer_count = 0;
    let mut summer_changed = 0;
    let mut summer_preserved = 0;
    let source = [
        [227, 161, 123, 255],
        [212, 131, 99, 255],
        [196, 112, 84, 255],
        [159, 85, 68, 255],
    ];
    for row in files {
        let name = row["path"].as_str().unwrap();
        let before = image::open(original.join(name)).unwrap().to_rgba8();
        let after = image::open(modified.join(name)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (592, 180));
        assert_eq!(before.dimensions(), after.dimensions());
        assert_eq!(
            fs::read(original.join(name.replace(".png", ".meta.toml"))).unwrap(),
            fs::read(modified.join(name.replace(".png", ".meta.toml"))).unwrap()
        );
        let summer = name.contains("/Summer/");
        let mut changes = [0; 2];
        for (x, y, pixel) in before.enumerate_pixels() {
            let new = after.get_pixel(x, y);
            assert_eq!(pixel[3], new[3]);
            if y >= 148 || !source.contains(&pixel.0) {
                assert_eq!(
                    pixel, new,
                    "changed clothing or unrelated color: {name} [{x},{y}]"
                );
            }
            // This shade occurs only on skin in the reviewed corpus, including
            // disconnected pixels beside the eyebrow in the sly expression.
            if pixel.0 == [212, 131, 99, 255] {
                assert_eq!(
                    new.0,
                    [127, 159, 189, 255],
                    "missed skin shade: {name} [{x},{y}]"
                );
            }
            if pixel != new {
                changes[(x / 296) as usize] += 1;
            } else if summer && source.contains(&pixel.0) {
                summer_preserved += 1;
            }
        }
        assert!(changes.iter().all(|&n| n > 0));
        if summer {
            summer_count += 1;
            summer_changed += changes.iter().sum::<usize>();
            for frame in 0..2 {
                // Chest, crossed forearm, raised forearm, and arm shadow.
                for (x, y) in [(160, 106), (153, 140), (183, 117), (173, 126)] {
                    let x = x + 296 * frame;
                    assert_ne!(
                        before.get_pixel(x, y),
                        after.get_pixel(x, y),
                        "missed summer skin: {name} [{x},{y}]"
                    );
                }
                // Cape, wrist jewelry, dress border, and bodice.
                for (x, y) in [(120, 132), (181, 108), (145, 107), (146, 116)] {
                    let x = x + 296 * frame;
                    assert_eq!(
                        before.get_pixel(x, y),
                        after.get_pixel(x, y),
                        "changed summer clothing: {name} [{x},{y}]"
                    );
                }
            }
        }
    }
    assert_eq!(summer_count, 25);
    assert!(summer_preserved > 0);
    eprintln!(
        "Verified 50 strips / 100 frames; summer changed {summer_changed} pixels and preserved {summer_preserved} matching clothing pixels"
    );
}
