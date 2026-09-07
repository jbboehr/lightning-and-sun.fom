use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 119 local portraits in extracted/adeline-seasonal-beach-study"]
fn beach_masks_cover_both_frames_and_preserve_swimwear_and_towel() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/adeline-seasonal-beach-study");
    let recipe = std::env::var_os("FOM_BEACH_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/adeline-seasonal-beach.json"));
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
    assert_eq!(report["files"].as_array().unwrap().len(), 119);
    let source = [
        [227, 161, 123, 255],
        [212, 131, 99, 255],
        [196, 112, 84, 255],
        [159, 85, 68, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
    ];
    let mut strips = 0;
    let mut changed = 0;
    for row in report["files"].as_array().unwrap() {
        let name = row["path"].as_str().unwrap();
        if !name.contains("/Beach/") {
            continue;
        }
        let expression = Path::new(name)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_prefix("spr_portrait_adeline_beach_")
            .unwrap();
        // These poses move the swimsuit and bracelet up one source pixel.
        let dy = u32::from(
            [
                "blush",
                "embarrassed",
                "gloomy_special",
                "hope_special",
                "sigh",
                "sly",
                "wink",
            ]
            .contains(&expression),
        );
        strips += 1;
        let before = image::open(original.join(name)).unwrap().to_rgba8();
        let after = image::open(modified.join(name)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (592, 180));
        assert_eq!(before.dimensions(), after.dimensions());
        let meta = name.replace(".png", ".meta.toml");
        assert_eq!(
            fs::read(original.join(&meta)).unwrap(),
            fs::read(modified.join(&meta)).unwrap()
        );
        let mut frames = [0; 2];
        for (x, y, pixel) in before.enumerate_pixels() {
            let new = after.get_pixel(x, y);
            assert_eq!(pixel[3], new[3]);
            // In these 19 reviewed strips, all four source shades occur only
            // on skin. Swimwear, towel trim, jewelry, lips and blush use others.
            if let Some(shade) = source.iter().position(|c| c == &pixel.0) {
                assert_eq!(new.0, blue[shade], "missed beach skin: {name} [{x},{y}]");
                frames[(x / 296) as usize] += 1;
            } else {
                assert_eq!(
                    pixel, new,
                    "changed clothing or facial detail: {name} [{x},{y}]"
                );
            }
        }
        assert!(frames.iter().all(|&n| n > 0));
        changed += frames.iter().sum::<usize>();
        for frame in 0..2 {
            // Shoulder, chest, raised hand, forearm and both exposed thighs.
            for (x, y) in [
                (129, 102),
                (154, 103),
                (170, 96),
                (150, 137),
                (135, 179),
                (170, 179),
            ] {
                let x = x + frame * 296;
                assert_ne!(
                    before.get_pixel(x, y),
                    after.get_pixel(x, y),
                    "missed body region: {name} [{x},{y}]"
                );
            }
            // The towel has a different silhouette and must keep its gold trim.
            let clothing = if name.ends_with("_bath_neutral.png") {
                [(157, 112), (160, 115), (139, 167)]
            } else {
                [(149, 115), (178, 104), (159, 158)]
            };
            for (x, y) in clothing {
                let x = x + frame * 296;
                let y = y - dy;
                assert_eq!(
                    before.get_pixel(x, y),
                    after.get_pixel(x, y),
                    "changed clothing: {name} [{x},{y}]"
                );
            }
        }
    }
    assert_eq!(strips, 19);
    eprintln!("Beach: 19 strips / 38 frames, {changed} changed skin pixels");
}
