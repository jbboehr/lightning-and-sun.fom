use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/landen and the local accepted Landen world baseline"]
fn landen_spring_actions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/landen");
    let baseline = root.join("generated/slice-048-build/characters/landen");
    let set = std::env::var_os("FOM_LANDEN_SPRING_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/landen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/landen-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..38];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..38], prior);
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
    // Reviewed grids distinguish skin from shoes, scarf, clothing and the apple.
    // The hand reaches over the left shoe in eat_south frame 1 (zero-based).
    let landmarks = [
        ("blink_east", 0, 43, 47, 0x000000, false),
        ("blink_east", 0, 41, 50, 0x332D2C, false),
        ("blink_east", 0, 41, 46, 0x403C3C, false),
        ("blink_east", 0, 35, 46, 0x491F1B, true),
        ("blink_east", 0, 42, 44, 0x553658, false),
        ("blink_east", 0, 39, 48, 0x605451, false),
        ("blink_east", 0, 39, 52, 0x63342A, false),
        ("blink_east", 0, 34, 46, 0x63342A, true),
        ("blink_east", 0, 44, 45, 0x854D3C, true),
        ("blink_east", 0, 39, 49, 0x857874, false),
        ("blink_east", 0, 38, 44, 0x88558D, false),
        ("blink_east", 0, 39, 45, 0x9D76A5, false),
        ("blink_east", 0, 42, 51, 0x9F6250, false),
        ("blink_east", 0, 34, 45, 0x9F6250, true),
        ("blink_east", 0, 43, 36, 0xA59DA2, false),
        ("blink_east", 0, 40, 29, 0xB2A0A4, false),
        ("blink_east", 0, 38, 36, 0xC2B9BE, false),
        ("blink_east", 0, 44, 43, 0xE3A190, false),
        ("blink_east", 0, 39, 42, 0xFFE2D6, false),
        ("blink_east", 1, 35, 46, 0x491F1B, true),
        ("blink_east", 1, 39, 52, 0x63342A, false),
        ("blink_east", 1, 34, 46, 0x63342A, true),
        ("blink_east", 1, 44, 45, 0x854D3C, true),
        ("blink_east", 1, 42, 51, 0x9F6250, false),
        ("blink_east", 1, 34, 45, 0x9F6250, true),
        ("blink_east", 2, 39, 52, 0x63342A, false),
        ("blink_east", 2, 42, 51, 0x9F6250, false),
        ("blink_south", 0, 45, 47, 0x000000, false),
        ("blink_south", 0, 41, 50, 0x332D2C, false),
        ("blink_south", 0, 40, 46, 0x403C3C, false),
        ("blink_south", 0, 46, 46, 0x491F1B, true),
        ("blink_south", 0, 42, 43, 0x553658, false),
        ("blink_south", 0, 42, 47, 0x605451, false),
        ("blink_south", 0, 41, 52, 0x63342A, false),
        ("blink_south", 0, 44, 44, 0x63342A, true),
        ("blink_south", 0, 42, 51, 0x854D3C, false),
        ("blink_south", 0, 40, 40, 0x854D3C, true),
        ("blink_south", 0, 38, 49, 0x857874, false),
        ("blink_south", 0, 37, 44, 0x88558D, false),
        ("blink_south", 0, 38, 45, 0x9D76A5, false),
        ("blink_south", 0, 41, 51, 0x9F6250, false),
        ("blink_south", 0, 45, 45, 0x9F6250, true),
        ("blink_south", 0, 35, 31, 0xB2A0A4, false),
        ("blink_south", 0, 42, 36, 0xC2B9BE, false),
        ("blink_south", 0, 39, 43, 0xE3A190, false),
        ("blink_south", 0, 38, 42, 0xFFE2D6, false),
        ("blink_south", 1, 46, 46, 0x491F1B, true),
        ("blink_south", 1, 41, 52, 0x63342A, false),
        ("blink_south", 1, 44, 44, 0x63342A, true),
        ("blink_south", 1, 42, 51, 0x854D3C, false),
        ("blink_south", 1, 40, 40, 0x854D3C, true),
        ("blink_south", 1, 41, 51, 0x9F6250, false),
        ("blink_south", 1, 45, 45, 0x9F6250, true),
        ("blink_south", 2, 41, 52, 0x63342A, false),
        ("blink_south", 2, 42, 51, 0x854D3C, false),
        ("blink_south", 2, 41, 51, 0x9F6250, false),
        ("drink_east", 1, 41, 49, 0x63342A, false),
        ("drink_east", 1, 39, 43, 0x63342A, true),
        ("drink_east", 1, 39, 42, 0x854D3C, true),
        ("drink_east", 1, 42, 48, 0x9F6250, false),
        ("drink_east", 1, 38, 42, 0x9F6250, true),
        ("drink_east", 2, 41, 49, 0x63342A, false),
        ("drink_east", 2, 42, 48, 0x9F6250, false),
        ("drink_north", 1, 34, 47, 0x491F1B, true),
        ("drink_north", 1, 40, 41, 0x63342A, true),
        ("drink_north", 1, 34, 46, 0x854D3C, true),
        ("drink_south", 1, 35, 43, 0x491F1B, true),
        ("drink_south", 1, 41, 50, 0x63342A, false),
        ("drink_south", 1, 40, 41, 0x63342A, true),
        ("drink_south", 1, 45, 45, 0x854D3C, true),
        ("drink_south", 1, 35, 42, 0x9F6250, true),
        ("drink_south", 2, 41, 50, 0x63342A, false),
        ("eat_east", 0, 42, 45, 0x000000, false),
        ("eat_east", 0, 38, 47, 0x332D2C, false),
        ("eat_east", 0, 38, 39, 0x491F1B, true),
        ("eat_east", 0, 38, 44, 0x553658, false),
        ("eat_east", 0, 42, 46, 0x605451, false),
        ("eat_east", 0, 41, 49, 0x63342A, false),
        ("eat_east", 0, 41, 40, 0x63342A, true),
        ("eat_east", 0, 41, 43, 0x854D3C, true),
        ("eat_east", 0, 40, 46, 0x857874, false),
        ("eat_east", 0, 38, 43, 0x88558D, false),
        ("eat_east", 0, 39, 45, 0x9D76A5, false),
        ("eat_east", 0, 42, 48, 0x9F6250, false),
        ("eat_east", 0, 42, 43, 0x9F6250, true),
        ("eat_east", 0, 43, 36, 0xA59DA2, false),
        ("eat_east", 0, 40, 30, 0xB2A0A4, false),
        ("eat_east", 0, 38, 36, 0xC2B9BE, false),
        ("eat_east", 0, 44, 49, 0xE3A190, false),
        ("eat_east", 0, 43, 37, 0xECF0E9, false),
        ("eat_east", 0, 38, 42, 0xFFE2D6, false),
        ("eat_east", 1, 46, 42, 0x491F1B, true),
        ("eat_east", 1, 41, 49, 0x63342A, false),
        ("eat_east", 1, 47, 41, 0x63342A, true),
        ("eat_east", 1, 43, 42, 0x854D3C, true),
        ("eat_east", 1, 42, 48, 0x9F6250, false),
        ("eat_east", 1, 44, 41, 0x9F6250, true),
        ("eat_east", 2, 41, 49, 0x63342A, false),
        ("eat_east", 2, 42, 48, 0x9F6250, false),
        ("eat_east", 3, 41, 49, 0x63342A, false),
        ("eat_east", 3, 42, 48, 0x9F6250, false),
        ("eat_east", 4, 41, 49, 0x63342A, false),
        ("eat_east", 4, 42, 48, 0x9F6250, false),
        ("eat_north", 1, 34, 47, 0x491F1B, true),
        ("eat_north", 1, 40, 41, 0x63342A, true),
        ("eat_north", 1, 34, 46, 0x854D3C, true),
        ("eat_south", 0, 46, 47, 0x000000, false),
        ("eat_south", 0, 41, 49, 0x332D2C, false),
        ("eat_south", 0, 45, 47, 0x491F1B, true),
        ("eat_south", 0, 43, 43, 0x553658, false),
        ("eat_south", 0, 41, 47, 0x605451, false),
        ("eat_south", 0, 41, 50, 0x63342A, false),
        ("eat_south", 0, 40, 40, 0x63342A, true),
        ("eat_south", 0, 37, 46, 0x854D3C, true),
        ("eat_south", 0, 41, 48, 0x857874, false),
        ("eat_south", 0, 38, 44, 0x88558D, false),
        ("eat_south", 0, 38, 45, 0x9D76A5, false),
        ("eat_south", 0, 35, 46, 0x9F6250, true),
        ("eat_south", 0, 35, 32, 0xB2A0A4, false),
        ("eat_south", 0, 42, 36, 0xC2B9BE, false),
        ("eat_south", 0, 44, 44, 0xE3A190, false),
        ("eat_south", 0, 42, 37, 0xECF0E9, false),
        ("eat_south", 0, 39, 43, 0xFFE2D6, false),
        ("eat_south", 1, 42, 40, 0x491F1B, true),
        ("eat_south", 1, 42, 50, 0x63342A, false),
        ("eat_south", 1, 39, 41, 0x63342A, true),
        ("eat_south", 1, 44, 46, 0x854D3C, true),
        ("eat_south", 1, 37, 47, 0x9F6250, true),
        ("eat_south", 2, 41, 50, 0x63342A, false),
        ("eat_south", 3, 41, 50, 0x63342A, false),
        ("eat_south", 4, 41, 50, 0x63342A, false),
        ("sit_north", 0, 43, 46, 0x000000, false),
        ("sit_north", 0, 45, 47, 0x491F1B, true),
        ("sit_north", 0, 37, 45, 0x553658, false),
        ("sit_north", 0, 36, 35, 0x605451, false),
        ("sit_north", 0, 40, 40, 0x63342A, true),
        ("sit_north", 0, 35, 46, 0x854D3C, true),
        ("sit_north", 0, 37, 36, 0x857874, false),
        ("sit_north", 0, 38, 46, 0x88558D, false),
        ("sit_north", 0, 40, 47, 0x9D76A5, false),
        ("sit_north", 0, 38, 33, 0xB2A0A4, false),
        ("sit_north", 0, 42, 43, 0xE3A190, false),
        ("sit_north", 0, 41, 42, 0xFFE2D6, false),
        ("eat_south", 1, 36, 49, 0x63342A, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[57, 61, 57]),
        ("blink_south", &[66, 70, 66]),
        ("drink_east", &[47, 50, 47]),
        ("drink_north", &[14, 9, 14]),
        ("drink_south", &[57, 66, 57]),
        ("eat_east", &[44, 49, 44, 48, 47]),
        ("eat_north", &[14, 9, 14]),
        ("eat_south", &[59, 64, 55, 68, 58]),
        ("sit_east", &[47]),
        ("sit_north", &[16]),
        ("sit_south", &[58]),
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
            // Literal shoe coordinates come from the source grids, independent of seeds.
            let mut shoes = Vec::new();
            for frame in 0..frames {
                let points: Vec<(u32, u32)> = if case.starts_with("blink_") {
                    let columns = if case.ends_with("east") {
                        [38, 39, 41, 42]
                    } else {
                        [37, 38, 41, 42]
                    };
                    columns
                        .into_iter()
                        .flat_map(|x| [(x, 51), (x, 52)])
                        .collect()
                } else if case.ends_with("east") {
                    vec![(44, 47), (41, 48), (42, 48), (44, 48), (41, 49), (42, 49)]
                } else if case.ends_with("south") {
                    // The second eating frame occludes the left shoe with his hand.
                    if case == "eat_south" && frame == 1 {
                        vec![(41, 50), (42, 50)]
                    } else {
                        vec![(37, 50), (38, 50), (41, 50), (42, 50)]
                    }
                } else {
                    Vec::new()
                };
                for (x, y) in points {
                    let x = x + frame * 80;
                    let pixel = before.get_pixel(x, y).0;
                    assert!(
                        source.iter().any(|c| rgba(*c) == pixel),
                        "source shoe {case} [{x},{y}]"
                    );
                    shoes.push((x, y));
                }
            }
            let mut per_frame = vec![0; frames as usize];
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                let skin = !shoes.contains(&(x, y));
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && skin)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Landen Spring material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin even when a Ryis target equals its source.
                if skin && source.iter().any(|c| rgba(*c) == p.0) {
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
