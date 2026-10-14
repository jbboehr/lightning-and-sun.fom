use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/holt and the local accepted Holt world baseline"]
fn holt_winter_writing_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/holt");
    let baseline = root.join("local/fixtures/slice-105-build/characters/holt");
    let set = std::env::var_os("FOM_HOLT_WINTER_WRITING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/holt-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/holt-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..136];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..136], prior);
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
    let source = [0xF0CC8E, 0xCD9067, 0x96523A, 0xAD6945];
    // In these pinned Winter writing strips, all four shades are skin only.
    // Include the hand below the board through row 49. Board, pen, moustache,
    // plaid sleeves, scarf, apron and boots stay original.
    // These literal landmarks and the whole-image oracle do not use mask seeds.
    let landmarks = [
        ("write_start_south", 0, 39, 35, 0xF0CC8E, true),
        ("write_start_south", 0, 36, 38, 0x762E21, false),
        ("write_start_south", 0, 45, 48, 0xF0CC8E, true),
        ("write_start_south", 0, 44, 49, 0xCD9067, true),
        ("write_start_south", 0, 45, 49, 0xF0CC8E, true),
        ("write_start_south", 0, 42, 48, 0xC5895C, false),
        ("write_start_south", 0, 47, 44, 0xC5895C, false),
        ("write_start_south", 0, 32, 48, 0xCD9067, true),
        ("write_start_south", 0, 34, 45, 0x373357, false),
        ("write_start_south", 0, 38, 41, 0x7AB99F, false),
        ("write_start_south", 0, 38, 52, 0x984E2C, false),
        ("write_start_south", 0, 38, 53, 0xD37B52, false),
        ("write_start_south", 0, 36, 41, 0xB45732, false),
        ("write_loop_south", 2, 34, 46, 0xCD9067, true),
        ("write_loop_south", 2, 34, 45, 0xF0CC8E, true),
        ("write_loop_south", 2, 35, 46, 0xF0CC8E, true),
        ("write_loop_south", 2, 44, 47, 0x7D4F3D, false),
        ("write_loop_south", 2, 44, 35, 0xAD6945, true),
        ("write_loop_south", 2, 38, 37, 0xCD9067, true),
        ("write_loop_south", 2, 37, 44, 0xE3779D, false),
        ("write_end_south", 0, 30, 44, 0xF0CC8E, true),
        ("write_end_south", 0, 30, 43, 0xCD9067, true),
        ("write_end_south", 1, 46, 48, 0xF0CC8E, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("write_end_south", &[48, 45]),
        ("write_loop_south", &[41, 40, 41, 40]),
        ("write_start_south", &[45, 48]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [9, 1, 10, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_holt_specialanimation_winter";
            let asset = format!("assets/animations/NPCs/Holt/Sprites/Winter/{prefix}_{case}.png");
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
                // Reviewed literal material shades are independent of mask seeds.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Holt Winter material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin even when a Ryis target equals its source.
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
