use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/elsie and the local accepted Elsie world baseline"]
fn elsie_summer_actions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/elsie");
    let baseline = root.join("local/fixtures/slice-182-build/characters/elsie");
    let set = std::env::var_os("FOM_ELSIE_SUMMER_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/elsie-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/elsie-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..77];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..77], prior);
    assert_eq!(candidate["source_colors"], profile["source_colors"]);
    assert_eq!(candidate["color_groups"], profile["color_groups"]);
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("bundle");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["build-presets", "--original"])
        .arg(&original)
        .arg("--presets")
        .arg(&set)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let source = [0xFBDCB9, 0xF3C193, 0xE08C6A, 0xA45838];
    // The four world shades occur only in skin in these five pinned sources.
    // Forehead, face, closed eyelids, neckline, arms, hands and exposed ankles follow the palette.
    // Check every pixel independently of component seeds; hair, eye details,
    // earrings, scarf, shawl, dress and shoes keep their original colors.
    let landmarks = [
        ("blink_east", 0, 41, 33, 0xFBDCB9, true),
        ("blink_east", 0, 41, 32, 0xF3C193, true),
        ("blink_east", 0, 41, 31, 0xE08C6A, true),
        ("blink_east", 0, 36, 35, 0xA45838, true),
        ("blink_east", 0, 40, 40, 0xA45838, true),
        ("blink_east", 0, 35, 44, 0xE08C6A, true),
        ("blink_east", 0, 45, 46, 0xF3C193, true),
        ("blink_east", 0, 38, 51, 0xE08C6A, true),
        ("blink_east", 0, 36, 34, 0x000000, false),
        ("blink_east", 0, 38, 36, 0xC2B9BE, false),
        ("blink_east", 0, 38, 37, 0xECF0E9, false),
        ("blink_east", 0, 37, 27, 0x5B566E, false),
        ("blink_east", 0, 38, 27, 0x91879B, false),
        ("blink_east", 0, 39, 27, 0xD7C8CE, false),
        ("blink_east", 0, 40, 45, 0x251F52, false),
        ("blink_east", 0, 39, 44, 0x3C347B, false),
        ("blink_east", 0, 39, 43, 0x6062B6, false),
        ("blink_east", 0, 37, 45, 0x344C7B, false),
        ("blink_east", 0, 38, 45, 0x71BDDD, false),
        ("blink_east", 0, 39, 41, 0xAD5107, false),
        ("blink_east", 0, 41, 41, 0xEB9F12, false),
        ("blink_east", 0, 37, 41, 0xFFD139, false),
        ("blink_east", 0, 36, 40, 0xBBEAEA, false),
        ("blink_east", 1, 41, 33, 0xFBDCB9, true),
        ("blink_east", 1, 41, 32, 0xF3C193, true),
        ("blink_east", 1, 41, 31, 0xE08C6A, true),
        ("blink_east", 1, 36, 35, 0xA45838, true),
        ("blink_east", 1, 40, 40, 0xA45838, true),
        ("blink_east", 1, 35, 44, 0xE08C6A, true),
        ("blink_east", 1, 45, 46, 0xF3C193, true),
        ("blink_east", 1, 38, 51, 0xE08C6A, true),
        ("blink_east", 1, 37, 35, 0xE08C6A, true),
        ("blink_east", 1, 36, 34, 0x000000, false),
        ("blink_east", 1, 37, 27, 0x5B566E, false),
        ("blink_east", 1, 38, 27, 0x91879B, false),
        ("blink_east", 1, 39, 27, 0xD7C8CE, false),
        ("blink_east", 1, 40, 45, 0x251F52, false),
        ("blink_east", 1, 39, 44, 0x3C347B, false),
        ("blink_east", 1, 39, 43, 0x6062B6, false),
        ("blink_east", 1, 37, 45, 0x344C7B, false),
        ("blink_east", 1, 38, 45, 0x71BDDD, false),
        ("blink_east", 1, 39, 41, 0xAD5107, false),
        ("blink_east", 1, 41, 41, 0xEB9F12, false),
        ("blink_east", 1, 37, 41, 0xFFD139, false),
        ("blink_east", 1, 36, 40, 0xBBEAEA, false),
        ("blink_east", 2, 41, 33, 0xFBDCB9, true),
        ("blink_east", 2, 41, 32, 0xF3C193, true),
        ("blink_east", 2, 41, 31, 0xE08C6A, true),
        ("blink_east", 2, 36, 35, 0xA45838, true),
        ("blink_east", 2, 40, 40, 0xA45838, true),
        ("blink_east", 2, 35, 44, 0xE08C6A, true),
        ("blink_east", 2, 45, 46, 0xF3C193, true),
        ("blink_east", 2, 38, 51, 0xE08C6A, true),
        ("blink_east", 2, 36, 34, 0x000000, false),
        ("blink_east", 2, 38, 36, 0xC2B9BE, false),
        ("blink_east", 2, 38, 37, 0xECF0E9, false),
        ("blink_east", 2, 37, 27, 0x5B566E, false),
        ("blink_east", 2, 38, 27, 0x91879B, false),
        ("blink_east", 2, 39, 27, 0xD7C8CE, false),
        ("blink_east", 2, 40, 45, 0x251F52, false),
        ("blink_east", 2, 39, 44, 0x3C347B, false),
        ("blink_east", 2, 39, 43, 0x6062B6, false),
        ("blink_east", 2, 37, 45, 0x344C7B, false),
        ("blink_east", 2, 38, 45, 0x71BDDD, false),
        ("blink_east", 2, 39, 41, 0xAD5107, false),
        ("blink_east", 2, 41, 41, 0xEB9F12, false),
        ("blink_east", 2, 37, 41, 0xFFD139, false),
        ("blink_east", 2, 36, 40, 0xBBEAEA, false),
        ("blink_south", 0, 40, 33, 0xFBDCB9, true),
        ("blink_south", 0, 40, 32, 0xF3C193, true),
        ("blink_south", 0, 40, 31, 0xE08C6A, true),
        ("blink_south", 0, 35, 35, 0xA45838, true),
        ("blink_south", 0, 39, 40, 0xA45838, true),
        ("blink_south", 0, 33, 45, 0xE08C6A, true),
        ("blink_south", 0, 46, 45, 0xE08C6A, true),
        ("blink_south", 0, 37, 51, 0xF3C193, true),
        ("blink_south", 0, 35, 34, 0x000000, false),
        ("blink_south", 0, 37, 36, 0xC2B9BE, false),
        ("blink_south", 0, 37, 37, 0xECF0E9, false),
        ("blink_south", 0, 36, 28, 0x5B566E, false),
        ("blink_south", 0, 37, 27, 0x91879B, false),
        ("blink_south", 0, 38, 27, 0xD7C8CE, false),
        ("blink_south", 0, 38, 45, 0x251F52, false),
        ("blink_south", 0, 37, 44, 0x3C347B, false),
        ("blink_south", 0, 38, 43, 0x6062B6, false),
        ("blink_south", 0, 45, 44, 0x344C7B, false),
        ("blink_south", 0, 43, 42, 0x44758F, false),
        ("blink_south", 0, 44, 44, 0x71BDDD, false),
        ("blink_south", 0, 37, 41, 0xAD5107, false),
        ("blink_south", 0, 38, 52, 0xE59712, false),
        ("blink_south", 0, 39, 41, 0xEB9F12, false),
        ("blink_south", 0, 35, 41, 0xFFD139, false),
        ("blink_south", 0, 44, 40, 0xBBEAEA, false),
        ("blink_south", 1, 40, 33, 0xFBDCB9, true),
        ("blink_south", 1, 40, 32, 0xF3C193, true),
        ("blink_south", 1, 40, 31, 0xE08C6A, true),
        ("blink_south", 1, 35, 35, 0xA45838, true),
        ("blink_south", 1, 39, 40, 0xA45838, true),
        ("blink_south", 1, 33, 45, 0xE08C6A, true),
        ("blink_south", 1, 46, 45, 0xE08C6A, true),
        ("blink_south", 1, 37, 51, 0xF3C193, true),
        ("blink_south", 1, 37, 35, 0xF3C193, true),
        ("blink_south", 1, 35, 34, 0x000000, false),
        ("blink_south", 1, 36, 28, 0x5B566E, false),
        ("blink_south", 1, 37, 27, 0x91879B, false),
        ("blink_south", 1, 38, 27, 0xD7C8CE, false),
        ("blink_south", 1, 38, 45, 0x251F52, false),
        ("blink_south", 1, 37, 44, 0x3C347B, false),
        ("blink_south", 1, 38, 43, 0x6062B6, false),
        ("blink_south", 1, 45, 44, 0x344C7B, false),
        ("blink_south", 1, 43, 42, 0x44758F, false),
        ("blink_south", 1, 44, 44, 0x71BDDD, false),
        ("blink_south", 1, 37, 41, 0xAD5107, false),
        ("blink_south", 1, 38, 52, 0xE59712, false),
        ("blink_south", 1, 39, 41, 0xEB9F12, false),
        ("blink_south", 1, 35, 41, 0xFFD139, false),
        ("blink_south", 1, 44, 40, 0xBBEAEA, false),
        ("blink_south", 2, 40, 33, 0xFBDCB9, true),
        ("blink_south", 2, 40, 32, 0xF3C193, true),
        ("blink_south", 2, 40, 31, 0xE08C6A, true),
        ("blink_south", 2, 35, 35, 0xA45838, true),
        ("blink_south", 2, 39, 40, 0xA45838, true),
        ("blink_south", 2, 33, 45, 0xE08C6A, true),
        ("blink_south", 2, 46, 45, 0xE08C6A, true),
        ("blink_south", 2, 37, 51, 0xF3C193, true),
        ("blink_south", 2, 35, 34, 0x000000, false),
        ("blink_south", 2, 37, 36, 0xC2B9BE, false),
        ("blink_south", 2, 37, 37, 0xECF0E9, false),
        ("blink_south", 2, 36, 28, 0x5B566E, false),
        ("blink_south", 2, 37, 27, 0x91879B, false),
        ("blink_south", 2, 38, 27, 0xD7C8CE, false),
        ("blink_south", 2, 38, 45, 0x251F52, false),
        ("blink_south", 2, 37, 44, 0x3C347B, false),
        ("blink_south", 2, 38, 43, 0x6062B6, false),
        ("blink_south", 2, 45, 44, 0x344C7B, false),
        ("blink_south", 2, 43, 42, 0x44758F, false),
        ("blink_south", 2, 44, 44, 0x71BDDD, false),
        ("blink_south", 2, 37, 41, 0xAD5107, false),
        ("blink_south", 2, 38, 52, 0xE59712, false),
        ("blink_south", 2, 39, 41, 0xEB9F12, false),
        ("blink_south", 2, 35, 41, 0xFFD139, false),
        ("blink_south", 2, 44, 40, 0xBBEAEA, false),
        ("sit_east", 0, 41, 33, 0xFBDCB9, true),
        ("sit_east", 0, 41, 32, 0xF3C193, true),
        ("sit_east", 0, 41, 31, 0xE08C6A, true),
        ("sit_east", 0, 36, 35, 0xA45838, true),
        ("sit_east", 0, 40, 40, 0xA45838, true),
        ("sit_east", 0, 35, 45, 0xFBDCB9, true),
        ("sit_east", 0, 44, 46, 0xFBDCB9, true),
        ("sit_east", 0, 35, 47, 0xA45838, true),
        ("sit_east", 0, 36, 34, 0x000000, false),
        ("sit_east", 0, 38, 35, 0xC2B9BE, false),
        ("sit_east", 0, 38, 36, 0xECF0E9, false),
        ("sit_east", 0, 37, 27, 0x5B566E, false),
        ("sit_east", 0, 38, 27, 0x91879B, false),
        ("sit_east", 0, 39, 27, 0xD7C8CE, false),
        ("sit_east", 0, 40, 44, 0x251F52, false),
        ("sit_east", 0, 39, 43, 0x3C347B, false),
        ("sit_east", 0, 43, 42, 0x6062B6, false),
        ("sit_east", 0, 37, 45, 0x344C7B, false),
        ("sit_east", 0, 38, 45, 0x71BDDD, false),
        ("sit_east", 0, 39, 41, 0xAD5107, false),
        ("sit_east", 0, 41, 41, 0xEB9F12, false),
        ("sit_east", 0, 37, 41, 0xFFD139, false),
        ("sit_east", 0, 36, 40, 0xBBEAEA, false),
        ("sit_north", 0, 39, 40, 0xE08C6A, true),
        ("sit_north", 0, 35, 44, 0xE08C6A, true),
        ("sit_north", 0, 44, 44, 0xE08C6A, true),
        ("sit_north", 0, 34, 47, 0xA45838, true),
        ("sit_north", 0, 36, 28, 0x5B566E, false),
        ("sit_north", 0, 38, 27, 0x91879B, false),
        ("sit_north", 0, 39, 27, 0xD7C8CE, false),
        ("sit_north", 0, 37, 47, 0x3C347B, false),
        ("sit_north", 0, 38, 47, 0x6062B6, false),
        ("sit_north", 0, 37, 41, 0x344C7B, false),
        ("sit_north", 0, 36, 42, 0x44758F, false),
        ("sit_north", 0, 36, 43, 0x71BDDD, false),
        ("sit_north", 0, 38, 41, 0xAD5107, false),
        ("sit_north", 0, 39, 41, 0xE59712, false),
        ("sit_south", 0, 40, 33, 0xFBDCB9, true),
        ("sit_south", 0, 40, 32, 0xF3C193, true),
        ("sit_south", 0, 40, 31, 0xE08C6A, true),
        ("sit_south", 0, 35, 35, 0xA45838, true),
        ("sit_south", 0, 39, 40, 0xA45838, true),
        ("sit_south", 0, 34, 45, 0xF3C193, true),
        ("sit_south", 0, 45, 45, 0xF3C193, true),
        ("sit_south", 0, 34, 47, 0xA45838, true),
        ("sit_south", 0, 35, 34, 0x000000, false),
        ("sit_south", 0, 37, 35, 0xC2B9BE, false),
        ("sit_south", 0, 37, 36, 0xECF0E9, false),
        ("sit_south", 0, 36, 28, 0x5B566E, false),
        ("sit_south", 0, 37, 27, 0x91879B, false),
        ("sit_south", 0, 38, 27, 0xD7C8CE, false),
        ("sit_south", 0, 38, 44, 0x251F52, false),
        ("sit_south", 0, 42, 42, 0x3C347B, false),
        ("sit_south", 0, 41, 42, 0x6062B6, false),
        ("sit_south", 0, 44, 44, 0x344C7B, false),
        ("sit_south", 0, 43, 42, 0x44758F, false),
        ("sit_south", 0, 43, 43, 0x71BDDD, false),
        ("sit_south", 0, 37, 41, 0xAD5107, false),
        ("sit_south", 0, 38, 49, 0xE59712, false),
        ("sit_south", 0, 39, 41, 0xEB9F12, false),
        ("sit_south", 0, 35, 41, 0xFFD139, false),
        ("sit_south", 0, 44, 40, 0xBBEAEA, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[58, 68, 58]),
        ("blink_south", &[62, 72, 62]),
        ("sit_east", &[49]),
        ("sit_north", &[17]),
        ("sit_south", &[55]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [8, 9, 10, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_elsie_summer";
            let asset = format!("assets/animations/NPCs/Elsie/Sprites/Summer/{prefix}_{case}.png");
            let before = image::open(original.join(&asset)).unwrap().to_rgba8();
            let after = image::open(variant.join(&asset)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), (80 * frames, 80));
            assert_eq!(before.dimensions(), after.dimensions());
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(variant.join(meta)).unwrap()
            );
            let mut per_frame = vec![0; frames as usize];
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                // All four reviewed source shades are skin in this pinned batch.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Elsie Summer material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin independently of target colors.
                if source.iter().any(|c| rgba(*c) == p.0) {
                    let index = source
                        .iter()
                        .position(|c| rgba(*c) == p.0)
                        .expect("non-skin material changed");
                    assert_eq!(q.0, rgba(target[index]));
                    per_frame[x as usize / 80] += 1;
                }
            }
            assert!(per_frame.iter().all(|n| *n > 0), "empty frame in {case}");
            assert_eq!(per_frame, expected_per_frame, "{case}");
            for &(name, frame, x, y, color, skin) in &landmarks {
                if name != case {
                    continue;
                }
                let x = x + frame * 80;
                assert_eq!(
                    before.get_pixel(x, y).0,
                    rgba(color),
                    "source {case} [{x},{y}]"
                );
                let expected = if skin {
                    target[source.iter().position(|c| *c == color).unwrap()]
                } else {
                    color
                };
                assert_eq!(
                    after.get_pixel(x, y).0,
                    rgba(expected),
                    "{id} {case} [{x},{y}] skin={skin}"
                );
            }
        }
        for region in prior {
            let asset = region["asset"].as_str().unwrap();
            for file in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(
                    fs::read(variant.join(&file)).unwrap(),
                    fs::read(baseline.join("variants").join(id).join(&file)).unwrap(),
                    "previous output changed: {id}/{file}"
                );
            }
        }
    }
    for region in prior {
        let asset = region["asset"].as_str().unwrap();
        for file in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
            assert_eq!(
                fs::read(original.join(&file)).unwrap(),
                fs::read(baseline.join("original").join(&file)).unwrap(),
                "previous source changed: {file}"
            );
        }
    }
}
