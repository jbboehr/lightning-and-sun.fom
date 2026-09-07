use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 126 local portraits in extracted/adeline-portraits-study"]
fn wedding_masks_cover_skin_and_preserve_earrings_embroidery_and_gloves() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/adeline-portraits-study");
    let recipe = std::env::var_os("FOM_WEDDING_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/adeline-portraits.json"));
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
    assert_eq!(report["files"].as_array().unwrap().len(), 126);
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
    let mut preserved = 0;
    for row in report["files"].as_array().unwrap() {
        let name = row["path"].as_str().unwrap();
        if !name.contains("/Wedding/") {
            continue;
        }
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
            let shade = source.iter().position(|c| c == &pixel.0);
            // Below the exposed upper arm, matching shades belong to the
            // glove cuff, dress trim and embroidery. Other colors stay intact.
            if shade.is_none() || y >= 126 {
                assert_eq!(
                    pixel, new,
                    "changed clothing or facial detail: {name} [{x},{y}]"
                );
            }
            // The head's only non-skin source-color component is the earring.
            // This also checks isolated skin beside the far eyebrow in all poses.
            if y < 94
                && !(x % 296 == 137 && (76..=78).contains(&y))
                && let Some(shade) = shade
            {
                assert_eq!(new.0, blue[shade], "missed face or neck: {name} [{x},{y}]");
            }
            if pixel != new {
                frames[(x / 296) as usize] += 1;
            } else if shade.is_some() {
                preserved += 1;
            }
        }
        assert!(frames.iter().all(|&n| n > 0));
        changed += frames.iter().sum::<usize>();
        for frame in 0..2 {
            // Chest highlight, cleavage shadow, and exposed upper arm.
            for (x, y) in [(150, 104), (164, 114), (136, 116)] {
                let x = x + 296 * frame;
                let shade = source
                    .iter()
                    .position(|c| c == &before.get_pixel(x, y).0)
                    .unwrap();
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[shade],
                    "missed wedding skin: {name} [{x},{y}]"
                );
            }
            // Earring, shoulder embroidery, necklace, glove cuff, bodice, skirt.
            for (x, y) in [
                (137, 77),
                (139, 95),
                (134, 99),
                (157, 102),
                (136, 126),
                (142, 121),
                (171, 146),
            ] {
                let x = x + 296 * frame;
                assert!(source.contains(&before.get_pixel(x, y).0));
                assert_eq!(
                    before.get_pixel(x, y),
                    after.get_pixel(x, y),
                    "changed gold detail: {name} [{x},{y}]"
                );
            }
        }
    }
    assert_eq!(strips, 7);
    assert!(preserved > 0);
    eprintln!(
        "Verified 7 wedding strips / 14 frames: {changed} skin pixels changed, {preserved} matching detail pixels preserved"
    );
}
