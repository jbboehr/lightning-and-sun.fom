use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/landen and the local accepted Landen world baseline"]
fn landen_spring_reactions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/landen");
    let baseline = root.join("generated/slice-049-build/characters/landen");
    let set = std::env::var_os("FOM_LANDEN_SPRING_REACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/landen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/landen-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..49];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..49], prior);
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
    let source = [0x9F6250, 0x854D3C, 0x63342A, 0x491F1B];
    // Reviewed grids distinguish moving skin from hair, scarf, shoes and mouth interior.
    // In these pinned strips skin ends above row 49, including the shocked jump.
    let landmarks = [
        ("action_east", 0, 44, 48, 0x000000, false),
        ("action_east", 0, 42, 50, 0x332D2C, false),
        ("action_east", 0, 40, 47, 0x491F1B, true),
        ("action_east", 0, 43, 45, 0x553658, false),
        ("action_east", 0, 40, 49, 0x605451, false),
        ("action_east", 0, 39, 52, 0x63342A, false),
        ("action_east", 0, 38, 46, 0x63342A, true),
        ("action_east", 0, 38, 45, 0x854D3C, true),
        ("action_east", 0, 43, 49, 0x857874, false),
        ("action_east", 0, 43, 41, 0x88558D, false),
        ("action_east", 0, 41, 41, 0x9D76A5, false),
        ("action_east", 0, 43, 51, 0x9F6250, false),
        ("action_east", 0, 40, 46, 0x9F6250, true),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 41, 30, 0xB2A0A4, false),
        ("action_east", 0, 39, 36, 0xC2B9BE, false),
        ("action_east", 0, 38, 53, 0xE3A190, false),
        ("action_east", 0, 44, 37, 0xECF0E9, false),
        ("action_east", 0, 43, 42, 0xFFE2D6, false),
        ("action_east", 1, 44, 52, 0x491F1B, false),
        ("action_east", 1, 48, 44, 0x491F1B, true),
        ("action_east", 1, 38, 52, 0x63342A, false),
        ("action_east", 1, 49, 44, 0x63342A, true),
        ("action_east", 1, 38, 51, 0x854D3C, false),
        ("action_east", 1, 46, 43, 0x854D3C, true),
        ("action_east", 1, 43, 51, 0x9F6250, false),
        ("action_east", 1, 47, 43, 0x9F6250, true),
        ("action_east", 2, 44, 52, 0x491F1B, false),
        ("action_east", 2, 38, 52, 0x63342A, false),
        ("action_east", 2, 38, 51, 0x854D3C, false),
        ("action_east", 2, 43, 51, 0x9F6250, false),
        ("action_east", 3, 44, 52, 0x491F1B, false),
        ("action_east", 3, 38, 52, 0x63342A, false),
        ("action_east", 3, 38, 51, 0x854D3C, false),
        ("action_east", 3, 43, 51, 0x9F6250, false),
        ("action_east", 4, 44, 52, 0x491F1B, false),
        ("action_east", 4, 38, 52, 0x63342A, false),
        ("action_east", 4, 38, 51, 0x854D3C, false),
        ("action_east", 4, 43, 51, 0x9F6250, false),
        ("action_east", 5, 39, 52, 0x63342A, false),
        ("action_east", 5, 43, 51, 0x9F6250, false),
        ("action_east", 6, 39, 52, 0x63342A, false),
        ("action_east", 6, 42, 51, 0x9F6250, false),
        ("action_north", 0, 36, 48, 0x000000, false),
        ("action_north", 0, 41, 50, 0x332D2C, false),
        ("action_north", 0, 33, 44, 0x491F1B, true),
        ("action_north", 0, 37, 46, 0x553658, false),
        ("action_north", 0, 38, 48, 0x605451, false),
        ("action_north", 0, 41, 52, 0x63342A, false),
        ("action_north", 0, 46, 41, 0x63342A, true),
        ("action_north", 0, 42, 51, 0x854D3C, false),
        ("action_north", 0, 45, 43, 0x854D3C, true),
        ("action_north", 0, 38, 49, 0x857874, false),
        ("action_north", 0, 37, 45, 0x88558D, false),
        ("action_north", 0, 39, 46, 0x9D76A5, false),
        ("action_north", 0, 41, 51, 0x9F6250, false),
        ("action_north", 0, 34, 43, 0x9F6250, true),
        ("action_north", 0, 38, 32, 0xB2A0A4, false),
        ("action_north", 0, 35, 43, 0xE3A190, false),
        ("action_north", 0, 42, 41, 0xFFE2D6, false),
        ("action_north", 1, 35, 45, 0x491F1B, true),
        ("action_north", 1, 41, 51, 0x63342A, false),
        ("action_north", 1, 34, 43, 0x63342A, true),
        ("action_north", 1, 42, 50, 0x854D3C, false),
        ("action_north", 1, 44, 38, 0x854D3C, true),
        ("action_north", 1, 38, 53, 0x9F6250, false),
        ("action_north", 1, 35, 44, 0x9F6250, true),
        ("action_north", 2, 41, 51, 0x63342A, false),
        ("action_north", 2, 42, 50, 0x854D3C, false),
        ("action_north", 2, 38, 53, 0x9F6250, false),
        ("action_north", 3, 41, 51, 0x63342A, false),
        ("action_north", 3, 42, 50, 0x854D3C, false),
        ("action_north", 3, 38, 53, 0x9F6250, false),
        ("action_north", 4, 41, 51, 0x63342A, false),
        ("action_north", 4, 42, 50, 0x854D3C, false),
        ("action_north", 4, 38, 53, 0x9F6250, false),
        ("action_north", 5, 41, 52, 0x63342A, false),
        ("action_north", 5, 42, 51, 0x854D3C, false),
        ("action_north", 5, 41, 51, 0x9F6250, false),
        ("action_north", 6, 41, 52, 0x63342A, false),
        ("action_north", 6, 42, 51, 0x854D3C, false),
        ("action_north", 6, 41, 51, 0x9F6250, false),
        ("action_south", 0, 46, 47, 0x000000, false),
        ("action_south", 0, 41, 51, 0x332D2C, false),
        ("action_south", 0, 40, 47, 0x403C3C, false),
        ("action_south", 0, 46, 46, 0x491F1B, true),
        ("action_south", 0, 42, 44, 0x553658, false),
        ("action_south", 0, 42, 48, 0x605451, false),
        ("action_south", 0, 41, 52, 0x63342A, false),
        ("action_south", 0, 45, 45, 0x63342A, true),
        ("action_south", 0, 40, 41, 0x854D3C, true),
        ("action_south", 0, 38, 50, 0x857874, false),
        ("action_south", 0, 41, 44, 0x88558D, false),
        ("action_south", 0, 38, 46, 0x9D76A5, false),
        ("action_south", 0, 33, 46, 0x9F6250, true),
        ("action_south", 0, 35, 32, 0xB2A0A4, false),
        ("action_south", 0, 42, 36, 0xC2B9BE, false),
        ("action_south", 0, 39, 44, 0xE3A190, false),
        ("action_south", 0, 42, 37, 0xECF0E9, false),
        ("action_south", 0, 38, 43, 0xFFE2D6, false),
        ("action_south", 1, 35, 46, 0x491F1B, true),
        ("action_south", 1, 41, 52, 0x63342A, false),
        ("action_south", 1, 44, 44, 0x63342A, true),
        ("action_south", 1, 42, 51, 0x854D3C, false),
        ("action_south", 1, 40, 40, 0x854D3C, true),
        ("action_south", 1, 41, 51, 0x9F6250, false),
        ("action_south", 1, 34, 45, 0x9F6250, true),
        ("action_south", 2, 41, 52, 0x63342A, false),
        ("action_south", 2, 42, 51, 0x854D3C, false),
        ("action_south", 2, 41, 51, 0x9F6250, false),
        ("action_south", 3, 41, 52, 0x63342A, false),
        ("action_south", 3, 42, 51, 0x854D3C, false),
        ("action_south", 3, 41, 51, 0x9F6250, false),
        ("action_south", 4, 41, 52, 0x63342A, false),
        ("action_south", 4, 42, 51, 0x854D3C, false),
        ("action_south", 4, 41, 51, 0x9F6250, false),
        ("action_south", 5, 41, 52, 0x63342A, false),
        ("action_south", 6, 41, 52, 0x63342A, false),
        ("action_south", 6, 42, 51, 0x854D3C, false),
        ("action_south", 6, 41, 51, 0x9F6250, false),
        ("shocked_end_south", 0, 33, 48, 0x000000, false),
        ("shocked_end_south", 0, 41, 51, 0x332D2C, false),
        ("shocked_end_south", 0, 40, 47, 0x403C3C, false),
        ("shocked_end_south", 0, 46, 47, 0x491F1B, true),
        ("shocked_end_south", 0, 42, 44, 0x553658, false),
        ("shocked_end_south", 0, 42, 48, 0x605451, false),
        ("shocked_end_south", 0, 41, 52, 0x63342A, false),
        ("shocked_end_south", 0, 35, 45, 0x63342A, true),
        ("shocked_end_south", 0, 40, 41, 0x854D3C, true),
        ("shocked_end_south", 0, 38, 50, 0x857874, false),
        ("shocked_end_south", 0, 37, 45, 0x88558D, false),
        ("shocked_end_south", 0, 38, 46, 0x9D76A5, false),
        ("shocked_end_south", 0, 45, 46, 0x9F6250, true),
        ("shocked_end_south", 0, 35, 32, 0xB2A0A4, false),
        ("shocked_end_south", 0, 39, 44, 0xE3A190, false),
        ("shocked_end_south", 0, 38, 43, 0xFFE2D6, false),
        ("shocked_loop_south", 0, 38, 47, 0x000000, false),
        ("shocked_loop_south", 0, 35, 49, 0x332D2C, false),
        ("shocked_loop_south", 0, 40, 45, 0x403C3C, false),
        ("shocked_loop_south", 0, 40, 36, 0x410808, false),
        ("shocked_loop_south", 0, 42, 37, 0x491F1B, true),
        ("shocked_loop_south", 0, 37, 42, 0x553658, false),
        ("shocked_loop_south", 0, 35, 47, 0x605451, false),
        ("shocked_loop_south", 0, 34, 50, 0x63342A, false),
        ("shocked_loop_south", 0, 48, 36, 0x63342A, true),
        ("shocked_loop_south", 0, 49, 34, 0x854D3C, true),
        ("shocked_loop_south", 0, 43, 47, 0x857874, false),
        ("shocked_loop_south", 0, 42, 43, 0x88558D, false),
        ("shocked_loop_south", 0, 37, 45, 0x9D76A5, false),
        ("shocked_loop_south", 0, 45, 49, 0x9F6250, false),
        ("shocked_loop_south", 0, 40, 40, 0x9F6250, true),
        ("shocked_loop_south", 0, 41, 28, 0xB2A0A4, false),
        ("shocked_loop_south", 0, 41, 33, 0xC2B9BE, false),
        ("shocked_loop_south", 0, 40, 37, 0xC83E37, false),
        ("shocked_loop_south", 0, 32, 50, 0xE3A190, false),
        ("shocked_loop_south", 0, 38, 35, 0xECF0E9, false),
        ("shocked_loop_south", 0, 40, 41, 0xFFE2D6, false),
        ("shocked_start_south", 0, 33, 48, 0x000000, false),
        ("shocked_start_south", 0, 41, 51, 0x332D2C, false),
        ("shocked_start_south", 0, 40, 47, 0x403C3C, false),
        ("shocked_start_south", 0, 46, 47, 0x491F1B, true),
        ("shocked_start_south", 0, 42, 44, 0x553658, false),
        ("shocked_start_south", 0, 42, 48, 0x605451, false),
        ("shocked_start_south", 0, 41, 52, 0x63342A, false),
        ("shocked_start_south", 0, 35, 45, 0x63342A, true),
        ("shocked_start_south", 0, 40, 41, 0x854D3C, true),
        ("shocked_start_south", 0, 38, 50, 0x857874, false),
        ("shocked_start_south", 0, 37, 45, 0x88558D, false),
        ("shocked_start_south", 0, 38, 46, 0x9D76A5, false),
        ("shocked_start_south", 0, 45, 46, 0x9F6250, true),
        ("shocked_start_south", 0, 35, 32, 0xB2A0A4, false),
        ("shocked_start_south", 0, 39, 44, 0xE3A190, false),
        ("shocked_start_south", 0, 38, 43, 0xFFE2D6, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[48, 47, 45, 47, 45, 48, 53]),
        ("action_north", &[13, 12, 13, 12, 13, 16, 20]),
        ("action_south", &[59, 58, 58, 58, 58, 59, 62]),
        ("shocked_end_south", &[70]),
        ("shocked_loop_south", &[72]),
        ("shocked_start_south", &[70]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [0, 1, 2, 3]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_landen_spring";
            let asset = format!("assets/animations/NPCs/Landen/Sprites/Spring/{prefix}_{case}.png");
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
                // In these six pinned strips, exposed skin ends above row 49.
                // The shoes below it reuse the same source shades.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && y < 49)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Landen Spring material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin even when a Ryis target equals its source.
                if y < 49 && source.iter().any(|c| rgba(*c) == p.0) {
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
