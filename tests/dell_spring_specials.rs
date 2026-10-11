use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/dell and the local accepted Dell world baseline"]
fn dell_spring_specials_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/dell");
    let baseline = root.join("generated/slice-156-build/characters/dell");
    let set = std::env::var_os("FOM_DELL_SPRING_SPECIALS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/dell-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/dell-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..55];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..55], prior);
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
    // Faces, eyelids, exposed wrists and ankles change. Matching blonde hair,
    // lashes, eye details, red mouth interiors and clothing remain original.
    let landmarks = [
        ("scratch_head_start_south", 0, 35, 38, 0xECC889, true),
        ("scratch_head_start_south", 0, 34, 43, 0xDF8D4B, true),
        ("scratch_head_start_south", 0, 45, 45, 0x762E21, true),
        ("scratch_head_start_south", 0, 37, 50, 0xDF8D4B, true),
        ("scratch_head_start_south", 0, 41, 50, 0xECC889, true),
        ("scratch_head_start_south", 0, 37, 31, 0xDF8D4B, false),
        ("scratch_head_start_south", 0, 38, 33, 0xB65932, false),
        ("scratch_head_start_south", 0, 34, 31, 0xFFDC90, false),
        ("scratch_head_start_south", 0, 38, 32, 0xF5F5F5, false),
        ("scratch_head_start_south", 0, 38, 43, 0xF96F8C, false),
        ("scratch_head_start_south", 0, 42, 43, 0x616BBE, false),
        ("scratch_head_start_south", 0, 37, 52, 0xA1121D, false),
        ("scratch_head_start_south", 0, 34, 40, 0xDF8D4B, false),
        ("scratch_head_start_south", 0, 45, 40, 0xDF8D4B, false),
        ("scratch_head_loop_south", 0, 35, 38, 0xECC889, true),
        ("scratch_head_loop_south", 0, 34, 42, 0xECC889, true),
        ("scratch_head_loop_south", 0, 45, 45, 0x762E21, true),
        ("scratch_head_loop_south", 0, 37, 50, 0xDF8D4B, true),
        ("scratch_head_loop_south", 0, 41, 50, 0xECC889, true),
        ("scratch_head_loop_south", 0, 37, 31, 0xDF8D4B, false),
        ("scratch_head_loop_south", 0, 38, 33, 0xB65932, false),
        ("scratch_head_loop_south", 0, 34, 31, 0xFFDC90, false),
        ("scratch_head_loop_south", 0, 38, 32, 0xF5F5F5, false),
        ("scratch_head_loop_south", 0, 38, 43, 0xF96F8C, false),
        ("scratch_head_loop_south", 0, 37, 43, 0x616BBE, false),
        ("scratch_head_loop_south", 0, 37, 52, 0xA1121D, false),
        ("scratch_head_loop_south", 0, 38, 38, 0xECF0E9, false),
        ("scratch_head_loop_south", 0, 37, 37, 0xC2B9BE, false),
        ("scratch_head_loop_south", 0, 34, 40, 0xB65932, false),
        ("scratch_head_loop_south", 0, 45, 40, 0xDF8D4B, false),
        ("scratch_head_loop_south", 1, 35, 38, 0xECC889, true),
        ("scratch_head_loop_south", 1, 34, 42, 0xECC889, true),
        ("scratch_head_loop_south", 1, 45, 45, 0x762E21, true),
        ("scratch_head_loop_south", 1, 37, 50, 0xDF8D4B, true),
        ("scratch_head_loop_south", 1, 41, 50, 0xECC889, true),
        ("scratch_head_loop_south", 1, 37, 31, 0xDF8D4B, false),
        ("scratch_head_loop_south", 1, 38, 33, 0xB65932, false),
        ("scratch_head_loop_south", 1, 34, 31, 0xFFDC90, false),
        ("scratch_head_loop_south", 1, 38, 32, 0xF5F5F5, false),
        ("scratch_head_loop_south", 1, 38, 43, 0xF96F8C, false),
        ("scratch_head_loop_south", 1, 37, 43, 0x616BBE, false),
        ("scratch_head_loop_south", 1, 37, 52, 0xA1121D, false),
        ("scratch_head_loop_south", 1, 38, 38, 0xECF0E9, false),
        ("scratch_head_loop_south", 1, 37, 37, 0xC2B9BE, false),
        ("scratch_head_loop_south", 1, 34, 40, 0xDF8D4B, false),
        ("scratch_head_loop_south", 1, 45, 40, 0xDF8D4B, false),
        ("scratch_head_end_south", 0, 35, 38, 0xECC889, true),
        ("scratch_head_end_south", 0, 34, 43, 0xDF8D4B, true),
        ("scratch_head_end_south", 0, 45, 45, 0x762E21, true),
        ("scratch_head_end_south", 0, 37, 50, 0xDF8D4B, true),
        ("scratch_head_end_south", 0, 41, 50, 0xECC889, true),
        ("scratch_head_end_south", 0, 37, 31, 0xDF8D4B, false),
        ("scratch_head_end_south", 0, 38, 33, 0xB65932, false),
        ("scratch_head_end_south", 0, 34, 31, 0xFFDC90, false),
        ("scratch_head_end_south", 0, 38, 32, 0xF5F5F5, false),
        ("scratch_head_end_south", 0, 38, 43, 0xF96F8C, false),
        ("scratch_head_end_south", 0, 42, 43, 0x616BBE, false),
        ("scratch_head_end_south", 0, 37, 52, 0xA1121D, false),
        ("scratch_head_end_south", 0, 34, 40, 0xDF8D4B, false),
        ("scratch_head_end_south", 0, 45, 40, 0xDF8D4B, false),
        ("yawn_start_south", 0, 35, 38, 0xECC889, true),
        ("yawn_start_south", 0, 33, 46, 0xDF8D4B, true),
        ("yawn_start_south", 0, 45, 45, 0x762E21, true),
        ("yawn_start_south", 0, 37, 50, 0xDF8D4B, true),
        ("yawn_start_south", 0, 41, 50, 0xECC889, true),
        ("yawn_start_south", 0, 37, 31, 0xDF8D4B, false),
        ("yawn_start_south", 0, 38, 33, 0xB65932, false),
        ("yawn_start_south", 0, 34, 31, 0xFFDC90, false),
        ("yawn_start_south", 0, 38, 32, 0xF5F5F5, false),
        ("yawn_start_south", 0, 38, 43, 0xF96F8C, false),
        ("yawn_start_south", 0, 37, 43, 0x616BBE, false),
        ("yawn_start_south", 0, 37, 52, 0xA1121D, false),
        ("yawn_start_south", 0, 39, 40, 0xA1121D, false),
        ("yawn_start_south", 0, 34, 40, 0xDF8D4B, false),
        ("yawn_start_south", 0, 45, 40, 0xDF8D4B, false),
        ("yawn_start_south", 1, 35, 38, 0xECC889, true),
        ("yawn_start_south", 1, 38, 40, 0xDF8D4B, true),
        ("yawn_start_south", 1, 45, 45, 0x762E21, true),
        ("yawn_start_south", 1, 37, 50, 0xDF8D4B, true),
        ("yawn_start_south", 1, 41, 50, 0xECC889, true),
        ("yawn_start_south", 1, 37, 31, 0xDF8D4B, false),
        ("yawn_start_south", 1, 38, 33, 0xB65932, false),
        ("yawn_start_south", 1, 34, 31, 0xFFDC90, false),
        ("yawn_start_south", 1, 38, 32, 0xF5F5F5, false),
        ("yawn_start_south", 1, 36, 43, 0xF96F8C, false),
        ("yawn_start_south", 1, 42, 43, 0x616BBE, false),
        ("yawn_start_south", 1, 37, 52, 0xA1121D, false),
        ("yawn_start_south", 1, 40, 39, 0xA1121D, false),
        ("yawn_start_south", 1, 34, 40, 0xDF8D4B, false),
        ("yawn_start_south", 1, 45, 40, 0xDF8D4B, false),
        ("yawn_start_south", 2, 35, 38, 0xECC889, true),
        ("yawn_start_south", 2, 38, 40, 0xECC889, true),
        ("yawn_start_south", 2, 45, 45, 0x762E21, true),
        ("yawn_start_south", 2, 37, 50, 0xDF8D4B, true),
        ("yawn_start_south", 2, 41, 50, 0xECC889, true),
        ("yawn_start_south", 2, 37, 31, 0xDF8D4B, false),
        ("yawn_start_south", 2, 38, 33, 0xB65932, false),
        ("yawn_start_south", 2, 34, 31, 0xFFDC90, false),
        ("yawn_start_south", 2, 38, 32, 0xF5F5F5, false),
        ("yawn_start_south", 2, 36, 42, 0xF96F8C, false),
        ("yawn_start_south", 2, 42, 43, 0x616BBE, false),
        ("yawn_start_south", 2, 37, 52, 0xA1121D, false),
        ("yawn_start_south", 2, 40, 38, 0xA1121D, false),
        ("yawn_start_south", 2, 34, 40, 0xDF8D4B, false),
        ("yawn_start_south", 2, 45, 40, 0xDF8D4B, false),
        ("yawn_loop_south", 0, 35, 38, 0xECC889, true),
        ("yawn_loop_south", 0, 38, 40, 0xDF8D4B, true),
        ("yawn_loop_south", 0, 44, 46, 0xB65932, true),
        ("yawn_loop_south", 0, 37, 50, 0xDF8D4B, true),
        ("yawn_loop_south", 0, 41, 50, 0xECC889, true),
        ("yawn_loop_south", 0, 34, 31, 0xDF8D4B, false),
        ("yawn_loop_south", 0, 42, 31, 0xB65932, false),
        ("yawn_loop_south", 0, 38, 31, 0xFFDC90, false),
        ("yawn_loop_south", 0, 41, 31, 0xF5F5F5, false),
        ("yawn_loop_south", 0, 36, 43, 0xF96F8C, false),
        ("yawn_loop_south", 0, 42, 43, 0x616BBE, false),
        ("yawn_loop_south", 0, 37, 52, 0xA1121D, false),
        ("yawn_loop_south", 0, 38, 37, 0xA1121D, false),
        ("yawn_loop_south", 0, 34, 40, 0xB65932, false),
        ("yawn_loop_south", 0, 45, 40, 0xB65932, false),
        ("yawn_end_south", 0, 35, 38, 0xECC889, true),
        ("yawn_end_south", 0, 38, 40, 0xDF8D4B, true),
        ("yawn_end_south", 0, 44, 46, 0xB65932, true),
        ("yawn_end_south", 0, 37, 50, 0xDF8D4B, true),
        ("yawn_end_south", 0, 41, 50, 0xECC889, true),
        ("yawn_end_south", 0, 37, 31, 0xDF8D4B, false),
        ("yawn_end_south", 0, 38, 33, 0xB65932, false),
        ("yawn_end_south", 0, 34, 31, 0xFFDC90, false),
        ("yawn_end_south", 0, 38, 32, 0xF5F5F5, false),
        ("yawn_end_south", 0, 36, 43, 0xF96F8C, false),
        ("yawn_end_south", 0, 42, 43, 0x616BBE, false),
        ("yawn_end_south", 0, 37, 52, 0xA1121D, false),
        ("yawn_end_south", 0, 38, 38, 0xA1121D, false),
        ("yawn_end_south", 0, 34, 40, 0xDF8D4B, false),
        ("yawn_end_south", 0, 45, 40, 0xDF8D4B, false),
        ("yawn_end_south", 1, 35, 38, 0xECC889, true),
        ("yawn_end_south", 1, 33, 46, 0xDF8D4B, true),
        ("yawn_end_south", 1, 45, 45, 0x762E21, true),
        ("yawn_end_south", 1, 37, 50, 0xDF8D4B, true),
        ("yawn_end_south", 1, 41, 50, 0xECC889, true),
        ("yawn_end_south", 1, 37, 31, 0xDF8D4B, false),
        ("yawn_end_south", 1, 38, 33, 0xB65932, false),
        ("yawn_end_south", 1, 34, 31, 0xFFDC90, false),
        ("yawn_end_south", 1, 38, 32, 0xF5F5F5, false),
        ("yawn_end_south", 1, 38, 43, 0xF96F8C, false),
        ("yawn_end_south", 1, 37, 43, 0x616BBE, false),
        ("yawn_end_south", 1, 37, 52, 0xA1121D, false),
        ("yawn_end_south", 1, 37, 39, 0xECF0E9, false),
        ("yawn_end_south", 1, 39, 40, 0xA1121D, false),
        ("yawn_end_south", 1, 34, 40, 0xDF8D4B, false),
        ("yawn_end_south", 1, 45, 40, 0xDF8D4B, false),
        ("yawn_end_south", 2, 35, 38, 0xECC889, true),
        ("yawn_end_south", 2, 34, 46, 0xDF8D4B, true),
        ("yawn_end_south", 2, 45, 45, 0x762E21, true),
        ("yawn_end_south", 2, 37, 50, 0xDF8D4B, true),
        ("yawn_end_south", 2, 41, 50, 0xECC889, true),
        ("yawn_end_south", 2, 37, 31, 0xDF8D4B, false),
        ("yawn_end_south", 2, 38, 33, 0xB65932, false),
        ("yawn_end_south", 2, 34, 31, 0xFFDC90, false),
        ("yawn_end_south", 2, 38, 32, 0xF5F5F5, false),
        ("yawn_end_south", 2, 38, 43, 0xF96F8C, false),
        ("yawn_end_south", 2, 36, 43, 0x382A61, false),
        ("yawn_end_south", 2, 37, 52, 0xA1121D, false),
        ("yawn_end_south", 2, 37, 39, 0xECF0E9, false),
        ("yawn_end_south", 2, 37, 38, 0xC2B9BE, false),
        ("yawn_end_south", 2, 34, 40, 0xDF8D4B, false),
        ("yawn_end_south", 2, 45, 40, 0xDF8D4B, false),
        ("scratch_head_loop_south", 0, 35, 41, 0x762E21, true),
        ("scratch_head_loop_south", 0, 33, 40, 0xDF8D4B, false),
        ("scratch_head_loop_south", 0, 33, 38, 0xDF8D4B, false),
        ("scratch_head_loop_south", 0, 32, 38, 0xFFDC90, false),
        ("scratch_head_loop_south", 1, 34, 42, 0xECC889, true),
        ("scratch_head_loop_south", 0, 41, 37, 0xECC889, true),
        ("scratch_head_loop_south", 0, 42, 37, 0xECC889, true),
        ("scratch_head_loop_south", 0, 42, 39, 0xDF8D4B, true),
        ("yawn_loop_south", 0, 39, 36, 0xECC889, true),
        ("yawn_loop_south", 0, 43, 36, 0xB65932, true),
        ("yawn_loop_south", 0, 44, 36, 0xB65932, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("scratch_head_start_south", &[63]),
        ("scratch_head_loop_south", &[55, 55]),
        ("scratch_head_end_south", &[63]),
        ("yawn_start_south", &[58, 47, 43]),
        ("yawn_loop_south", &[43]),
        ("yawn_end_south", &[41, 50, 50]),
    ];
    // Bounds reviewed against original grids, independently of component seeds.
    let face = [
        (37, 36, 44),
        (38, 35, 45),
        (39, 35, 45),
        (40, 36, 44),
        (41, 37, 43),
    ];
    let skin_at = |case: &str, x: u32, y: u32| {
        let frame = x / 80;
        let x = x % 80;
        // Hair ends above the exposed wrist and neck on row 42.
        if y >= 42 {
            return true;
        }
        if case == "scratch_head_loop_south" && frame == 0 && (x, y) == (35, 41) {
            return true;
        }
        // The yawn raises the nose and joins two pixels at its right cheek edge.
        if case == "yawn_loop_south" && [(39, 36), (40, 36), (43, 36), (44, 36)].contains(&(x, y)) {
            return true;
        }
        face.iter()
            .any(|&(row, lo, hi)| y == row && (lo..hi).contains(&x))
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
            let prefix = "spr_npc_dell_specialanimation_spring";
            let asset = format!("assets/animations/NPCs/Dell/Sprites/Spring/{prefix}_{case}.png");
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
                    "Dell Spring material mismatch: {id} {case} [{x},{y}]"
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
