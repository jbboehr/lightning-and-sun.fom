use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/elsie and the local accepted Elsie world baseline"]
fn elsie_spring_reactions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/elsie");
    let baseline = root.join("local/fixtures/slice-177-build/characters/elsie");
    let set = std::env::var_os("FOM_ELSIE_SPRING_REACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/elsie-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/elsie-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..56];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..56], prior);
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
    // Forehead, face, neckline and moving hands follow the palette.
    // Check every pixel independently of component seeds; mouth interiors,
    // hair, cosmetics, earrings, gown and shoes keep their original colors.
    let landmarks = [
        ("shocked_start_south", 0, 40, 34, 0xFBDCB9, true),
        ("shocked_start_south", 0, 40, 33, 0xF3C193, true),
        ("shocked_start_south", 0, 40, 32, 0xE08C6A, true),
        ("shocked_start_south", 0, 35, 36, 0xA45838, true),
        ("shocked_start_south", 0, 38, 42, 0xE08C6A, true),
        ("shocked_start_south", 0, 32, 47, 0xFBDCB9, true),
        ("shocked_start_south", 0, 33, 48, 0xA45838, true),
        ("shocked_start_south", 0, 45, 47, 0xFBDCB9, true),
        ("shocked_start_south", 0, 46, 48, 0xA45838, true),
        ("shocked_start_south", 0, 37, 34, 0x000000, false),
        ("shocked_start_south", 0, 36, 29, 0x5B566E, false),
        ("shocked_start_south", 0, 37, 28, 0x91879B, false),
        ("shocked_start_south", 0, 38, 28, 0xD7C8CE, false),
        ("shocked_start_south", 0, 35, 40, 0xF4AB50, false),
        ("shocked_start_south", 0, 35, 41, 0xFFE990, false),
        ("shocked_start_south", 0, 39, 46, 0x700815, false),
        ("shocked_start_south", 0, 35, 45, 0x721943, false),
        ("shocked_start_south", 0, 37, 45, 0x9B212C, false),
        ("shocked_start_south", 0, 36, 42, 0xA03E59, false),
        ("shocked_start_south", 0, 34, 45, 0xEB8C8C, false),
        ("shocked_start_south", 0, 36, 43, 0xFBC9BB, false),
        ("shocked_start_south", 0, 37, 44, 0xD84B3A, false),
        ("shocked_loop_south", 0, 40, 31, 0xFBDCB9, true),
        ("shocked_loop_south", 0, 40, 30, 0xF3C193, true),
        ("shocked_loop_south", 0, 39, 30, 0xE08C6A, true),
        ("shocked_loop_south", 0, 35, 33, 0xA45838, true),
        ("shocked_loop_south", 0, 38, 40, 0xF3C193, true),
        ("shocked_loop_south", 0, 31, 33, 0xFBDCB9, true),
        ("shocked_loop_south", 0, 30, 34, 0xF3C193, true),
        ("shocked_loop_south", 0, 32, 33, 0xE08C6A, true),
        ("shocked_loop_south", 0, 46, 33, 0xFBDCB9, true),
        ("shocked_loop_south", 0, 47, 34, 0xF3C193, true),
        ("shocked_loop_south", 0, 47, 33, 0xE08C6A, true),
        ("shocked_loop_south", 0, 34, 33, 0x000000, false),
        ("shocked_loop_south", 0, 37, 33, 0xC2B9BE, false),
        ("shocked_loop_south", 0, 37, 34, 0xECF0E9, false),
        ("shocked_loop_south", 0, 39, 36, 0x410808, false),
        ("shocked_loop_south", 0, 36, 26, 0x5B566E, false),
        ("shocked_loop_south", 0, 37, 25, 0x91879B, false),
        ("shocked_loop_south", 0, 38, 25, 0xD7C8CE, false),
        ("shocked_loop_south", 0, 35, 36, 0xF4AB50, false),
        ("shocked_loop_south", 0, 39, 43, 0x700815, false),
        ("shocked_loop_south", 0, 35, 41, 0x721943, false),
        ("shocked_loop_south", 0, 37, 42, 0x9B212C, false),
        ("shocked_loop_south", 0, 34, 43, 0xA03E59, false),
        ("shocked_loop_south", 0, 35, 42, 0xEB8C8C, false),
        ("shocked_loop_south", 0, 35, 43, 0xFBC9BB, false),
        ("shocked_loop_south", 0, 37, 41, 0xD84B3A, false),
        ("shocked_end_south", 0, 40, 34, 0xFBDCB9, true),
        ("shocked_end_south", 0, 40, 33, 0xF3C193, true),
        ("shocked_end_south", 0, 40, 32, 0xE08C6A, true),
        ("shocked_end_south", 0, 35, 36, 0xA45838, true),
        ("shocked_end_south", 0, 38, 42, 0xE08C6A, true),
        ("shocked_end_south", 0, 32, 47, 0xFBDCB9, true),
        ("shocked_end_south", 0, 33, 48, 0xA45838, true),
        ("shocked_end_south", 0, 45, 47, 0xFBDCB9, true),
        ("shocked_end_south", 0, 46, 48, 0xA45838, true),
        ("shocked_end_south", 0, 37, 34, 0x000000, false),
        ("shocked_end_south", 0, 36, 29, 0x5B566E, false),
        ("shocked_end_south", 0, 37, 28, 0x91879B, false),
        ("shocked_end_south", 0, 38, 28, 0xD7C8CE, false),
        ("shocked_end_south", 0, 35, 40, 0xF4AB50, false),
        ("shocked_end_south", 0, 35, 41, 0xFFE990, false),
        ("shocked_end_south", 0, 39, 46, 0x700815, false),
        ("shocked_end_south", 0, 35, 45, 0x721943, false),
        ("shocked_end_south", 0, 37, 45, 0x9B212C, false),
        ("shocked_end_south", 0, 36, 42, 0xA03E59, false),
        ("shocked_end_south", 0, 34, 45, 0xEB8C8C, false),
        ("shocked_end_south", 0, 36, 43, 0xFBC9BB, false),
        ("shocked_end_south", 0, 37, 44, 0xD84B3A, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("shocked_start_south", &[77]),
        ("shocked_loop_south", &[85]),
        ("shocked_end_south", &[77]),
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
            let prefix = "spr_npc_elsie_spring";
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
