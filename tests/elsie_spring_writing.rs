use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/elsie and the local accepted Elsie world baseline"]
fn elsie_spring_writing_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/elsie");
    let baseline = root.join("generated/slice-178-build/characters/elsie");
    let set = std::env::var_os("FOM_ELSIE_SPRING_WRITING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/elsie-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/elsie-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..59];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..59], prior);
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
    // The four world shades occur only in skin in these three pinned sources.
    // Forehead, face, neckline and the moving writing hand follow the palette.
    // Check every pixel independently of component seeds; hair, eye details,
    // cosmetics, earrings, gown, shoes, book and quill keep their original colors.
    let landmarks = [
        ("write_start_south", 0, 40, 34, 0xFBDCB9, true),
        ("write_start_south", 0, 40, 33, 0xF3C193, true),
        ("write_start_south", 0, 40, 32, 0xE08C6A, true),
        ("write_start_south", 0, 35, 36, 0xA45838, true),
        ("write_start_south", 0, 39, 41, 0xE08C6A, true),
        ("write_start_south", 0, 32, 47, 0xFBDCB9, true),
        ("write_start_south", 0, 33, 48, 0xA45838, true),
        ("write_start_south", 0, 45, 36, 0x000000, false),
        ("write_start_south", 0, 36, 29, 0x5B566E, false),
        ("write_start_south", 0, 37, 28, 0x91879B, false),
        ("write_start_south", 0, 38, 28, 0xD7C8CE, false),
        ("write_start_south", 0, 35, 40, 0xF4AB50, false),
        ("write_start_south", 0, 44, 41, 0xFFE990, false),
        ("write_start_south", 0, 38, 48, 0x700815, false),
        ("write_start_south", 0, 39, 50, 0x9B212C, false),
        ("write_start_south", 0, 33, 51, 0xA03E59, false),
        ("write_start_south", 0, 38, 49, 0xEB8C8C, false),
        ("write_start_south", 0, 37, 48, 0xFBC9BB, false),
        ("write_start_south", 0, 40, 50, 0xD84B3A, false),
        ("write_start_south", 0, 44, 45, 0xFAEADD, false),
        ("write_start_south", 0, 46, 45, 0x896D5D, false),
        ("write_start_south", 0, 48, 43, 0xC9AF9C, false),
        ("write_start_south", 0, 48, 44, 0x6F2240, false),
        ("write_start_south", 0, 49, 44, 0xAD455F, false),
        ("write_start_south", 0, 32, 41, 0xEE5C91, false),
        ("write_start_south", 0, 33, 41, 0xFB8BA6, false),
        ("write_start_south", 0, 32, 40, 0xFFC9D3, false),
        ("write_start_south", 1, 41, 33, 0xFBDCB9, true),
        ("write_start_south", 1, 41, 32, 0xF3C193, true),
        ("write_start_south", 1, 41, 31, 0xE08C6A, true),
        ("write_start_south", 1, 36, 35, 0xA45838, true),
        ("write_start_south", 1, 38, 41, 0xE08C6A, true),
        ("write_start_south", 1, 32, 45, 0xA45838, true),
        ("write_start_south", 1, 35, 36, 0x000000, false),
        ("write_start_south", 1, 38, 37, 0xECF0E9, false),
        ("write_start_south", 1, 37, 27, 0x5B566E, false),
        ("write_start_south", 1, 38, 27, 0x91879B, false),
        ("write_start_south", 1, 39, 27, 0xD7C8CE, false),
        ("write_start_south", 1, 36, 39, 0xF4AB50, false),
        ("write_start_south", 1, 36, 40, 0xFFE990, false),
        ("write_start_south", 1, 37, 50, 0x700815, false),
        ("write_start_south", 1, 33, 50, 0xA03E59, false),
        ("write_start_south", 1, 34, 48, 0xEB8C8C, false),
        ("write_start_south", 1, 36, 48, 0xFBC9BB, false),
        ("write_start_south", 1, 42, 45, 0xFAEADD, false),
        ("write_start_south", 1, 44, 45, 0x896D5D, false),
        ("write_start_south", 1, 46, 43, 0xC9AF9C, false),
        ("write_start_south", 1, 46, 44, 0x6F2240, false),
        ("write_start_south", 1, 47, 44, 0xAD455F, false),
        ("write_start_south", 1, 31, 38, 0xEE5C91, false),
        ("write_start_south", 1, 32, 38, 0xFB8BA6, false),
        ("write_start_south", 1, 31, 37, 0xFFC9D3, false),
        ("write_loop_south", 0, 41, 33, 0xFBDCB9, true),
        ("write_loop_south", 0, 41, 32, 0xF3C193, true),
        ("write_loop_south", 0, 41, 31, 0xE08C6A, true),
        ("write_loop_south", 0, 36, 35, 0xA45838, true),
        ("write_loop_south", 0, 41, 42, 0xF3C193, true),
        ("write_loop_south", 0, 35, 36, 0x000000, false),
        ("write_loop_south", 0, 38, 37, 0xECF0E9, false),
        ("write_loop_south", 0, 37, 27, 0x5B566E, false),
        ("write_loop_south", 0, 38, 27, 0x91879B, false),
        ("write_loop_south", 0, 39, 27, 0xD7C8CE, false),
        ("write_loop_south", 0, 44, 39, 0xF4AB50, false),
        ("write_loop_south", 0, 44, 40, 0xFFE990, false),
        ("write_loop_south", 0, 37, 50, 0x700815, false),
        ("write_loop_south", 0, 33, 50, 0xA03E59, false),
        ("write_loop_south", 0, 34, 48, 0xEB8C8C, false),
        ("write_loop_south", 0, 36, 48, 0xFBC9BB, false),
        ("write_loop_south", 0, 40, 50, 0xD84B3A, false),
        ("write_loop_south", 0, 38, 42, 0xF4F2F6, false),
        ("write_loop_south", 0, 41, 44, 0xFAEADD, false),
        ("write_loop_south", 0, 43, 44, 0x896D5D, false),
        ("write_loop_south", 0, 45, 42, 0xC9AF9C, false),
        ("write_loop_south", 0, 45, 43, 0x6F2240, false),
        ("write_loop_south", 0, 46, 43, 0xAD455F, false),
        ("write_loop_south", 0, 33, 39, 0xEE5C91, false),
        ("write_loop_south", 0, 33, 38, 0xFB8BA6, false),
        ("write_loop_south", 0, 35, 38, 0xFFC9D3, false),
        ("write_loop_south", 1, 41, 33, 0xFBDCB9, true),
        ("write_loop_south", 1, 41, 32, 0xF3C193, true),
        ("write_loop_south", 1, 41, 31, 0xE08C6A, true),
        ("write_loop_south", 1, 36, 35, 0xA45838, true),
        ("write_loop_south", 1, 39, 41, 0xE08C6A, true),
        ("write_loop_south", 1, 35, 46, 0xF3C193, true),
        ("write_loop_south", 1, 35, 47, 0xE08C6A, true),
        ("write_loop_south", 1, 35, 36, 0x000000, false),
        ("write_loop_south", 1, 38, 37, 0xECF0E9, false),
        ("write_loop_south", 1, 37, 27, 0x5B566E, false),
        ("write_loop_south", 1, 38, 27, 0x91879B, false),
        ("write_loop_south", 1, 39, 27, 0xD7C8CE, false),
        ("write_loop_south", 1, 44, 39, 0xF4AB50, false),
        ("write_loop_south", 1, 44, 40, 0xFFE990, false),
        ("write_loop_south", 1, 37, 50, 0x700815, false),
        ("write_loop_south", 1, 33, 50, 0xA03E59, false),
        ("write_loop_south", 1, 34, 48, 0xEB8C8C, false),
        ("write_loop_south", 1, 35, 49, 0xFBC9BB, false),
        ("write_loop_south", 1, 40, 50, 0xD84B3A, false),
        ("write_loop_south", 1, 41, 44, 0xFAEADD, false),
        ("write_loop_south", 1, 43, 44, 0x896D5D, false),
        ("write_loop_south", 1, 45, 42, 0xC9AF9C, false),
        ("write_loop_south", 1, 45, 43, 0x6F2240, false),
        ("write_loop_south", 1, 46, 43, 0xAD455F, false),
        ("write_loop_south", 1, 32, 40, 0xEE5C91, false),
        ("write_loop_south", 1, 32, 39, 0xFB8BA6, false),
        ("write_loop_south", 1, 34, 39, 0xFFC9D3, false),
        ("write_loop_south", 2, 41, 33, 0xFBDCB9, true),
        ("write_loop_south", 2, 41, 32, 0xF3C193, true),
        ("write_loop_south", 2, 41, 31, 0xE08C6A, true),
        ("write_loop_south", 2, 36, 35, 0xA45838, true),
        ("write_loop_south", 2, 39, 41, 0xE08C6A, true),
        ("write_loop_south", 2, 35, 45, 0xFBDCB9, true),
        ("write_loop_south", 2, 34, 45, 0xF3C193, true),
        ("write_loop_south", 2, 34, 46, 0xE08C6A, true),
        ("write_loop_south", 2, 35, 36, 0x000000, false),
        ("write_loop_south", 2, 38, 37, 0xECF0E9, false),
        ("write_loop_south", 2, 37, 27, 0x5B566E, false),
        ("write_loop_south", 2, 38, 27, 0x91879B, false),
        ("write_loop_south", 2, 39, 27, 0xD7C8CE, false),
        ("write_loop_south", 2, 44, 39, 0xF4AB50, false),
        ("write_loop_south", 2, 44, 40, 0xFFE990, false),
        ("write_loop_south", 2, 37, 50, 0x700815, false),
        ("write_loop_south", 2, 33, 50, 0xA03E59, false),
        ("write_loop_south", 2, 34, 48, 0xEB8C8C, false),
        ("write_loop_south", 2, 36, 48, 0xFBC9BB, false),
        ("write_loop_south", 2, 41, 45, 0xFAEADD, false),
        ("write_loop_south", 2, 43, 45, 0x896D5D, false),
        ("write_loop_south", 2, 45, 43, 0xC9AF9C, false),
        ("write_loop_south", 2, 45, 44, 0x6F2240, false),
        ("write_loop_south", 2, 46, 44, 0xAD455F, false),
        ("write_loop_south", 2, 31, 39, 0xEE5C91, false),
        ("write_loop_south", 2, 31, 38, 0xFB8BA6, false),
        ("write_loop_south", 2, 33, 38, 0xFFC9D3, false),
        ("write_loop_south", 3, 41, 33, 0xFBDCB9, true),
        ("write_loop_south", 3, 41, 32, 0xF3C193, true),
        ("write_loop_south", 3, 41, 31, 0xE08C6A, true),
        ("write_loop_south", 3, 36, 35, 0xA45838, true),
        ("write_loop_south", 3, 38, 41, 0xE08C6A, true),
        ("write_loop_south", 3, 34, 46, 0xFBDCB9, true),
        ("write_loop_south", 3, 34, 47, 0xF3C193, true),
        ("write_loop_south", 3, 35, 36, 0x000000, false),
        ("write_loop_south", 3, 38, 37, 0xECF0E9, false),
        ("write_loop_south", 3, 37, 27, 0x5B566E, false),
        ("write_loop_south", 3, 38, 27, 0x91879B, false),
        ("write_loop_south", 3, 39, 27, 0xD7C8CE, false),
        ("write_loop_south", 3, 36, 39, 0xF4AB50, false),
        ("write_loop_south", 3, 44, 40, 0xFFE990, false),
        ("write_loop_south", 3, 37, 50, 0x700815, false),
        ("write_loop_south", 3, 33, 50, 0xA03E59, false),
        ("write_loop_south", 3, 45, 48, 0xEB8C8C, false),
        ("write_loop_south", 3, 35, 49, 0xFBC9BB, false),
        ("write_loop_south", 3, 40, 50, 0xD84B3A, false),
        ("write_loop_south", 3, 41, 44, 0xFAEADD, false),
        ("write_loop_south", 3, 43, 44, 0x896D5D, false),
        ("write_loop_south", 3, 45, 42, 0xC9AF9C, false),
        ("write_loop_south", 3, 45, 43, 0x6F2240, false),
        ("write_loop_south", 3, 46, 43, 0xAD455F, false),
        ("write_loop_south", 3, 31, 40, 0xEE5C91, false),
        ("write_loop_south", 3, 31, 39, 0xFB8BA6, false),
        ("write_loop_south", 3, 33, 39, 0xFFC9D3, false),
        ("write_end_south", 0, 41, 33, 0xFBDCB9, true),
        ("write_end_south", 0, 41, 32, 0xF3C193, true),
        ("write_end_south", 0, 41, 31, 0xE08C6A, true),
        ("write_end_south", 0, 36, 35, 0xA45838, true),
        ("write_end_south", 0, 38, 41, 0xE08C6A, true),
        ("write_end_south", 0, 32, 45, 0xA45838, true),
        ("write_end_south", 0, 35, 36, 0x000000, false),
        ("write_end_south", 0, 38, 37, 0xECF0E9, false),
        ("write_end_south", 0, 37, 27, 0x5B566E, false),
        ("write_end_south", 0, 38, 27, 0x91879B, false),
        ("write_end_south", 0, 39, 27, 0xD7C8CE, false),
        ("write_end_south", 0, 36, 39, 0xF4AB50, false),
        ("write_end_south", 0, 36, 40, 0xFFE990, false),
        ("write_end_south", 0, 37, 50, 0x700815, false),
        ("write_end_south", 0, 33, 50, 0xA03E59, false),
        ("write_end_south", 0, 34, 48, 0xEB8C8C, false),
        ("write_end_south", 0, 36, 48, 0xFBC9BB, false),
        ("write_end_south", 0, 42, 45, 0xFAEADD, false),
        ("write_end_south", 0, 44, 45, 0x896D5D, false),
        ("write_end_south", 0, 46, 43, 0xC9AF9C, false),
        ("write_end_south", 0, 46, 44, 0x6F2240, false),
        ("write_end_south", 0, 47, 44, 0xAD455F, false),
        ("write_end_south", 0, 31, 38, 0xEE5C91, false),
        ("write_end_south", 0, 32, 38, 0xFB8BA6, false),
        ("write_end_south", 0, 31, 37, 0xFFC9D3, false),
        ("write_end_south", 1, 40, 34, 0xFBDCB9, true),
        ("write_end_south", 1, 40, 33, 0xF3C193, true),
        ("write_end_south", 1, 40, 32, 0xE08C6A, true),
        ("write_end_south", 1, 35, 36, 0xA45838, true),
        ("write_end_south", 1, 39, 41, 0xE08C6A, true),
        ("write_end_south", 1, 32, 47, 0xFBDCB9, true),
        ("write_end_south", 1, 33, 48, 0xA45838, true),
        ("write_end_south", 1, 45, 36, 0x000000, false),
        ("write_end_south", 1, 36, 29, 0x5B566E, false),
        ("write_end_south", 1, 37, 28, 0x91879B, false),
        ("write_end_south", 1, 38, 28, 0xD7C8CE, false),
        ("write_end_south", 1, 35, 40, 0xF4AB50, false),
        ("write_end_south", 1, 44, 41, 0xFFE990, false),
        ("write_end_south", 1, 38, 48, 0x700815, false),
        ("write_end_south", 1, 39, 50, 0x9B212C, false),
        ("write_end_south", 1, 33, 51, 0xA03E59, false),
        ("write_end_south", 1, 38, 49, 0xEB8C8C, false),
        ("write_end_south", 1, 37, 48, 0xFBC9BB, false),
        ("write_end_south", 1, 40, 50, 0xD84B3A, false),
        ("write_end_south", 1, 44, 45, 0xFAEADD, false),
        ("write_end_south", 1, 46, 45, 0x896D5D, false),
        ("write_end_south", 1, 48, 43, 0xC9AF9C, false),
        ("write_end_south", 1, 48, 44, 0x6F2240, false),
        ("write_end_south", 1, 49, 44, 0xAD455F, false),
        ("write_end_south", 1, 32, 41, 0xEE5C91, false),
        ("write_end_south", 1, 33, 41, 0xFB8BA6, false),
        ("write_end_south", 1, 32, 40, 0xFFC9D3, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("write_start_south", &[68, 57]),
        ("write_loop_south", &[46, 53, 58, 58]),
        ("write_end_south", &[57, 68]),
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
            let prefix = "spr_npc_elsie_specialanimation_spring";
            let asset = format!("assets/animations/NPCs/Elsie/Sprites/Spring/{prefix}_{case}.png");
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
                    "Elsie Spring material mismatch: {id} {case} [{x},{y}]"
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
