use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 100 local portraits in extracted/adeline-all-seasons-study"]
fn autumn_winter_masks_cover_skin_and_preserve_cuffs_and_trim() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/adeline-all-seasons-study");
    let recipe = std::env::var_os("FOM_ALL_SEASONS_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/adeline-all-seasons.json"));
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
    assert_eq!(report["files"].as_array().unwrap().len(), 100);
    let source = [
        [227, 161, 123, 255],
        [212, 131, 99, 255],
        [196, 112, 84, 255],
        [159, 85, 68, 255],
    ];
    for season in ["Autumn", "Winter"] {
        let mut strips = 0;
        let mut changed = 0;
        let mut protected = 0;
        for row in report["files"].as_array().unwrap() {
            let name = row["path"].as_str().unwrap();
            if !name.contains(&format!("/{season}/")) {
                continue;
            }
            strips += 1;
            // These seven poses raise the torso by one pixel in both outfits.
            let expression = Path::new(name)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .strip_prefix(&format!("spr_portrait_adeline_{}_", season.to_lowercase()))
                .unwrap();
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
                if y >= 145 || !source.contains(&pixel.0) {
                    assert_eq!(
                        pixel, new,
                        "changed clothing or unrelated color: {name} [{x},{y}]"
                    );
                }
                // This shade is confined to skin in these reviewed source strips.
                if pixel.0 == [212, 131, 99, 255] {
                    assert_eq!(
                        new.0,
                        [127, 159, 189, 255],
                        "missed skin shade: {name} [{x},{y}]"
                    );
                }
                if pixel != new {
                    frames[(x / 296) as usize] += 1;
                } else if source.contains(&pixel.0) {
                    protected += 1;
                }
            }
            assert!(frames.iter().all(|&n| n > 0));
            changed += frames.iter().sum::<usize>();
            for frame in 0..2 {
                let mut skin = vec![(170, 95), (180, 135), (188, 142), (187, 143)];
                if season == "Autumn" {
                    skin.extend([(158, 100), (160, 107)]);
                }
                for (x, y) in skin {
                    let x = x + frame * 296;
                    let y = y - dy;
                    assert_ne!(
                        before.get_pixel(x, y),
                        after.get_pixel(x, y),
                        "missed hand or chest: {name} [{x},{y}]"
                    );
                }
                // The gold collar and autumn cuff/cape reuse the source ramp.
                let mut clothing = vec![(148, 89), (150, 90)];
                if season == "Autumn" {
                    clothing.extend([(181, 104), (138, 115), (172, 139)]);
                }
                for (x, y) in clothing {
                    let x = x + frame * 296;
                    let y = y - dy;
                    assert_eq!(
                        before.get_pixel(x, y),
                        after.get_pixel(x, y),
                        "changed cuff or trim: {name} [{x},{y}]"
                    );
                }
            }
        }
        assert_eq!(strips, 25);
        assert!(protected > 0);
        eprintln!(
            "{season}: 25 strips / 50 frames, {changed} changed pixels, {protected} matching clothing pixels preserved"
        );
    }
}
