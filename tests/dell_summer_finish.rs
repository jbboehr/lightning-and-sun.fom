use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/dell and the local accepted Dell world baseline"]
fn dell_summer_finish_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/dell");
    let baseline = root.join("local/fixtures/slice-162-build/characters/dell");
    let set = std::env::var_os("FOM_DELL_SUMMER_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/dell-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/dell-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..85];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..85], prior);
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
    let source = [0xECC889, 0xDF8D4B, 0xB65932, 0x762E21];
    // Faces, closed eyelids, exposed hands, arms and legs change. Shared-color
    // blonde hair, eye details, clothing, boots, stick and trail stay original.
    let landmarks = [
        ("scratch_head_start_south", 0, 35, 38, 0xECC889, true),
        ("scratch_head_start_south", 0, 34, 43, 0xDF8D4B, true),
        ("scratch_head_start_south", 0, 44, 45, 0xB65932, true),
        ("scratch_head_start_south", 0, 37, 50, 0xDF8D4B, true),
        ("scratch_head_start_south", 0, 41, 50, 0xECC889, true),
        ("scratch_head_start_south", 0, 35, 32, 0xDF8D4B, false),
        ("scratch_head_start_south", 0, 38, 33, 0xB65932, false),
        ("scratch_head_start_south", 0, 34, 31, 0xFFDC90, false),
        ("scratch_head_start_south", 0, 40, 33, 0xF5F5F5, false),
        ("scratch_head_start_south", 0, 34, 40, 0xDF8D4B, false),
        ("scratch_head_start_south", 0, 45, 40, 0xDF8D4B, false),
        ("scratch_head_start_south", 0, 42, 43, 0x804637, false),
        ("scratch_head_start_south", 0, 38, 43, 0xB17557, false),
        ("scratch_head_start_south", 0, 43, 43, 0x6D91AF, false),
        ("scratch_head_start_south", 0, 39, 43, 0xB5DEF2, false),
        ("scratch_head_start_south", 0, 37, 48, 0xD8A474, false),
        ("scratch_head_start_south", 0, 37, 52, 0xC44748, false),
        ("scratch_head_loop_south", 0, 35, 38, 0xECC889, true),
        ("scratch_head_loop_south", 0, 34, 42, 0xECC889, true),
        ("scratch_head_loop_south", 0, 44, 45, 0xB65932, true),
        ("scratch_head_loop_south", 0, 37, 50, 0xDF8D4B, true),
        ("scratch_head_loop_south", 0, 41, 50, 0xECC889, true),
        ("scratch_head_loop_south", 0, 35, 32, 0xDF8D4B, false),
        ("scratch_head_loop_south", 0, 38, 33, 0xB65932, false),
        ("scratch_head_loop_south", 0, 34, 31, 0xFFDC90, false),
        ("scratch_head_loop_south", 0, 40, 33, 0xF5F5F5, false),
        ("scratch_head_loop_south", 0, 38, 38, 0xECF0E9, false),
        ("scratch_head_loop_south", 0, 37, 37, 0xC2B9BE, false),
        ("scratch_head_loop_south", 0, 34, 40, 0xDF8D4B, false),
        ("scratch_head_loop_south", 0, 45, 40, 0xDF8D4B, false),
        ("scratch_head_loop_south", 0, 37, 43, 0x804637, false),
        ("scratch_head_loop_south", 0, 38, 43, 0xB17557, false),
        ("scratch_head_loop_south", 0, 36, 43, 0x6D91AF, false),
        ("scratch_head_loop_south", 0, 39, 43, 0xB5DEF2, false),
        ("scratch_head_loop_south", 0, 37, 48, 0xD8A474, false),
        ("scratch_head_loop_south", 0, 37, 52, 0xC44748, false),
        ("scratch_head_loop_south", 1, 35, 38, 0xECC889, true),
        ("scratch_head_loop_south", 1, 34, 42, 0xECC889, true),
        ("scratch_head_loop_south", 1, 44, 45, 0xB65932, true),
        ("scratch_head_loop_south", 1, 37, 50, 0xDF8D4B, true),
        ("scratch_head_loop_south", 1, 41, 50, 0xECC889, true),
        ("scratch_head_loop_south", 1, 35, 32, 0xDF8D4B, false),
        ("scratch_head_loop_south", 1, 38, 33, 0xB65932, false),
        ("scratch_head_loop_south", 1, 34, 31, 0xFFDC90, false),
        ("scratch_head_loop_south", 1, 40, 33, 0xF5F5F5, false),
        ("scratch_head_loop_south", 1, 38, 38, 0xECF0E9, false),
        ("scratch_head_loop_south", 1, 37, 37, 0xC2B9BE, false),
        ("scratch_head_loop_south", 1, 34, 40, 0xDF8D4B, false),
        ("scratch_head_loop_south", 1, 45, 40, 0xDF8D4B, false),
        ("scratch_head_loop_south", 1, 37, 43, 0x804637, false),
        ("scratch_head_loop_south", 1, 38, 43, 0xB17557, false),
        ("scratch_head_loop_south", 1, 36, 43, 0x6D91AF, false),
        ("scratch_head_loop_south", 1, 39, 43, 0xB5DEF2, false),
        ("scratch_head_loop_south", 1, 37, 48, 0xD8A474, false),
        ("scratch_head_loop_south", 1, 37, 52, 0xC44748, false),
        ("scratch_head_end_south", 0, 35, 38, 0xECC889, true),
        ("scratch_head_end_south", 0, 34, 43, 0xDF8D4B, true),
        ("scratch_head_end_south", 0, 44, 45, 0xB65932, true),
        ("scratch_head_end_south", 0, 37, 50, 0xDF8D4B, true),
        ("scratch_head_end_south", 0, 41, 50, 0xECC889, true),
        ("scratch_head_end_south", 0, 35, 32, 0xDF8D4B, false),
        ("scratch_head_end_south", 0, 38, 33, 0xB65932, false),
        ("scratch_head_end_south", 0, 34, 31, 0xFFDC90, false),
        ("scratch_head_end_south", 0, 40, 33, 0xF5F5F5, false),
        ("scratch_head_end_south", 0, 34, 40, 0xDF8D4B, false),
        ("scratch_head_end_south", 0, 45, 40, 0xDF8D4B, false),
        ("scratch_head_end_south", 0, 42, 43, 0x804637, false),
        ("scratch_head_end_south", 0, 38, 43, 0xB17557, false),
        ("scratch_head_end_south", 0, 43, 43, 0x6D91AF, false),
        ("scratch_head_end_south", 0, 39, 43, 0xB5DEF2, false),
        ("scratch_head_end_south", 0, 37, 48, 0xD8A474, false),
        ("scratch_head_end_south", 0, 37, 52, 0xC44748, false),
        ("swing_stick_east", 0, 40, 39, 0xECC889, true),
        ("swing_stick_east", 0, 36, 45, 0x762E21, true),
        ("swing_stick_east", 0, 44, 46, 0xB65932, true),
        ("swing_stick_east", 0, 38, 50, 0xB65932, true),
        ("swing_stick_east", 0, 37, 32, 0xDF8D4B, false),
        ("swing_stick_east", 0, 39, 34, 0xB65932, false),
        ("swing_stick_east", 0, 34, 32, 0xFFDC90, false),
        ("swing_stick_east", 0, 41, 34, 0xF5F5F5, false),
        ("swing_stick_east", 0, 38, 39, 0xECF0E9, false),
        ("swing_stick_east", 0, 38, 38, 0xC2B9BE, false),
        ("swing_stick_east", 0, 38, 44, 0x804637, false),
        ("swing_stick_east", 0, 39, 44, 0xB17557, false),
        ("swing_stick_east", 0, 37, 44, 0x6D91AF, false),
        ("swing_stick_east", 0, 40, 44, 0xB5DEF2, false),
        ("swing_stick_east", 0, 38, 49, 0xD8A474, false),
        ("swing_stick_east", 0, 39, 53, 0xE16667, false),
        ("swing_stick_east", 1, 38, 40, 0xECC889, true),
        ("swing_stick_east", 1, 39, 46, 0xECC889, true),
        ("swing_stick_east", 1, 40, 47, 0xECC889, true),
        ("swing_stick_east", 1, 44, 50, 0xDF8D4B, true),
        ("swing_stick_east", 1, 37, 33, 0xDF8D4B, false),
        ("swing_stick_east", 1, 39, 35, 0xB65932, false),
        ("swing_stick_east", 1, 34, 33, 0xFFDC90, false),
        ("swing_stick_east", 1, 41, 35, 0xF5F5F5, false),
        ("swing_stick_east", 1, 43, 38, 0xA8571C, false),
        ("swing_stick_east", 1, 45, 38, 0x712922, false),
        ("swing_stick_east", 1, 38, 47, 0x804637, false),
        ("swing_stick_east", 1, 39, 49, 0xB17557, false),
        ("swing_stick_east", 1, 39, 45, 0x6D91AF, false),
        ("swing_stick_east", 1, 38, 45, 0xB5DEF2, false),
        ("swing_stick_east", 1, 38, 50, 0xD8A474, false),
        ("swing_stick_east", 1, 39, 53, 0xE16667, false),
        ("swing_stick_east", 2, 40, 39, 0xECC889, true),
        ("swing_stick_east", 2, 39, 45, 0xECC889, true),
        ("swing_stick_east", 2, 40, 45, 0xECC889, true),
        ("swing_stick_east", 2, 38, 50, 0xB65932, true),
        ("swing_stick_east", 2, 37, 32, 0xDF8D4B, false),
        ("swing_stick_east", 2, 39, 34, 0xB65932, false),
        ("swing_stick_east", 2, 34, 32, 0xFFDC90, false),
        ("swing_stick_east", 2, 41, 34, 0xF5F5F5, false),
        ("swing_stick_east", 2, 38, 39, 0xECF0E9, false),
        ("swing_stick_east", 2, 38, 38, 0xC2B9BE, false),
        ("swing_stick_east", 2, 48, 38, 0xA8571C, false),
        ("swing_stick_east", 2, 48, 39, 0x712922, false),
        ("swing_stick_east", 2, 38, 46, 0x804637, false),
        ("swing_stick_east", 2, 39, 48, 0xB17557, false),
        ("swing_stick_east", 2, 39, 44, 0x6D91AF, false),
        ("swing_stick_east", 2, 38, 44, 0xB5DEF2, false),
        ("swing_stick_east", 2, 38, 49, 0xD8A474, false),
        ("swing_stick_east", 2, 39, 53, 0xE16667, false),
        ("swing_stick_east", 3, 41, 39, 0xECC889, true),
        ("swing_stick_east", 3, 37, 45, 0xB65932, true),
        ("swing_stick_east", 3, 45, 46, 0xB65932, true),
        ("swing_stick_east", 3, 38, 32, 0xDF8D4B, false),
        ("swing_stick_east", 3, 40, 34, 0xB65932, false),
        ("swing_stick_east", 3, 35, 32, 0xFFDC90, false),
        ("swing_stick_east", 3, 42, 34, 0xF5F5F5, false),
        ("swing_stick_east", 3, 39, 39, 0xECF0E9, false),
        ("swing_stick_east", 3, 39, 38, 0xC2B9BE, false),
        ("swing_stick_east", 3, 53, 46, 0xA8571C, false),
        ("swing_stick_east", 3, 52, 41, 0x712922, false),
        ("swing_stick_east", 3, 39, 44, 0x804637, false),
        ("swing_stick_east", 3, 40, 44, 0xB17557, false),
        ("swing_stick_east", 3, 38, 44, 0x6D91AF, false),
        ("swing_stick_east", 3, 41, 44, 0xB5DEF2, false),
        ("swing_stick_east", 3, 39, 53, 0xE16667, false),
        ("swing_stick_east", 4, 44, 40, 0xECC889, true),
        ("swing_stick_east", 4, 37, 45, 0xECC889, true),
        ("swing_stick_east", 4, 48, 45, 0xDF8D4B, true),
        ("swing_stick_east", 4, 40, 50, 0xB65932, true),
        ("swing_stick_east", 4, 41, 33, 0xDF8D4B, false),
        ("swing_stick_east", 4, 43, 35, 0xB65932, false),
        ("swing_stick_east", 4, 38, 33, 0xFFDC90, false),
        ("swing_stick_east", 4, 45, 35, 0xF5F5F5, false),
        ("swing_stick_east", 4, 42, 40, 0xECF0E9, false),
        ("swing_stick_east", 4, 42, 39, 0xC2B9BE, false),
        ("swing_stick_east", 4, 36, 46, 0xA8571C, false),
        ("swing_stick_east", 4, 36, 45, 0x712922, false),
        ("swing_stick_east", 4, 41, 45, 0x804637, false),
        ("swing_stick_east", 4, 42, 45, 0xB17557, false),
        ("swing_stick_east", 4, 40, 46, 0x6D91AF, false),
        ("swing_stick_east", 4, 40, 45, 0xB5DEF2, false),
        ("swing_stick_east", 4, 40, 49, 0xD8A474, false),
        ("swing_stick_east", 4, 39, 53, 0xE16667, false),
        ("swing_stick_east", 5, 44, 39, 0xECC889, true),
        ("swing_stick_east", 5, 36, 44, 0xDF8D4B, true),
        ("swing_stick_east", 5, 48, 44, 0xDF8D4B, true),
        ("swing_stick_east", 5, 40, 50, 0xB65932, true),
        ("swing_stick_east", 5, 41, 32, 0xDF8D4B, false),
        ("swing_stick_east", 5, 43, 34, 0xB65932, false),
        ("swing_stick_east", 5, 38, 32, 0xFFDC90, false),
        ("swing_stick_east", 5, 45, 34, 0xF5F5F5, false),
        ("swing_stick_east", 5, 42, 39, 0xECF0E9, false),
        ("swing_stick_east", 5, 42, 38, 0xC2B9BE, false),
        ("swing_stick_east", 5, 37, 45, 0xA8571C, false),
        ("swing_stick_east", 5, 36, 45, 0x712922, false),
        ("swing_stick_east", 5, 41, 44, 0x804637, false),
        ("swing_stick_east", 5, 42, 44, 0xB17557, false),
        ("swing_stick_east", 5, 40, 45, 0x6D91AF, false),
        ("swing_stick_east", 5, 40, 44, 0xB5DEF2, false),
        ("swing_stick_east", 5, 40, 49, 0xD8A474, false),
        ("swing_stick_east", 5, 39, 53, 0xE16667, false),
        ("swing_stick_east", 6, 44, 39, 0xECC889, true),
        ("swing_stick_east", 6, 36, 44, 0xDF8D4B, true),
        ("swing_stick_east", 6, 48, 44, 0xDF8D4B, true),
        ("swing_stick_east", 6, 40, 50, 0xB65932, true),
        ("swing_stick_east", 6, 41, 32, 0xDF8D4B, false),
        ("swing_stick_east", 6, 43, 34, 0xB65932, false),
        ("swing_stick_east", 6, 38, 32, 0xFFDC90, false),
        ("swing_stick_east", 6, 45, 34, 0xF5F5F5, false),
        ("swing_stick_east", 6, 42, 39, 0xECF0E9, false),
        ("swing_stick_east", 6, 42, 38, 0xC2B9BE, false),
        ("swing_stick_east", 6, 37, 45, 0xA8571C, false),
        ("swing_stick_east", 6, 36, 45, 0x712922, false),
        ("swing_stick_east", 6, 41, 44, 0x804637, false),
        ("swing_stick_east", 6, 42, 44, 0xB17557, false),
        ("swing_stick_east", 6, 40, 45, 0x6D91AF, false),
        ("swing_stick_east", 6, 40, 44, 0xB5DEF2, false),
        ("swing_stick_east", 6, 40, 49, 0xD8A474, false),
        ("swing_stick_east", 6, 39, 53, 0xE16667, false),
        ("swing_stick_east", 0, 36, 44, 0x000000, false),
        ("swing_stick_east", 0, 37, 44, 0x6D91AF, false),
        ("swing_stick_east", 0, 45, 42, 0x000000, false),
        ("swing_stick_east", 5, 38, 43, 0xECC889, true),
        ("swing_stick_east", 6, 38, 43, 0xECC889, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("scratch_head_start_south", &[68]),
        ("scratch_head_loop_south", &[59, 62]),
        ("scratch_head_end_south", &[68]),
        ("swing_stick_east", &[47, 39, 49, 37, 47, 54, 54]),
    ];
    // Bounds reviewed against original grids, independently of component seeds.
    let skin_at = |case: &str, x: u32, y: u32| {
        let frame = (x / 80) as usize;
        let x = x % 80;
        if case.starts_with("scratch_head") {
            let rows = [
                (37, 36, 44),
                (38, 35, 45),
                (39, 35, 45),
                (40, 36, 44),
                (41, 37, 43),
            ];
            return y >= 42
                || rows
                    .iter()
                    .any(|&(row, lo, hi)| y == row && (lo..hi).contains(&x));
        }
        let (dx, dy, body) = [
            (0, 0, 44),
            (0, 1, 45),
            (0, 0, 44),
            (1, 0, 44),
            (4, 1, 44),
            (4, 0, 43),
            (4, 0, 43),
        ][frame];
        if y >= body {
            return true;
        }
        let face = if frame == 1 {
            [
                (38, 37, 45),
                (39, 36, 45),
                (40, 36, 45),
                (41, 37, 44),
                (42, 38, 43),
                (43, 40, 42),
            ]
        } else {
            [
                (38, 40, 42),
                (39, 36, 45),
                (40, 36, 45),
                (41, 37, 44),
                (42, 38, 43),
                (43, 40, 42),
            ]
        };
        face.iter()
            .any(|&(row, lo, hi)| y == row + dy && (lo + dx..hi + dx).contains(&x))
    };
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [12, 13, 14, 15]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_dell_specialanimation_summer";
            let asset = format!("assets/animations/NPCs/Dell/Sprites/Summer/{prefix}_{case}.png");
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
                // The reviewed face rows separate skin from matching blonde hair.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| {
                        if skin_at(case, x, y) {
                            rgba(target[i])
                        } else {
                            p.0
                        }
                    });
                assert_eq!(
                    q.0, expected,
                    "Dell Summer material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin independently of target colors.
                if skin_at(case, x, y) && source.iter().any(|c| rgba(*c) == p.0) {
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
