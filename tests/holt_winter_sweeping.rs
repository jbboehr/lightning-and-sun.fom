use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/holt and the local accepted Holt world baseline"]
fn holt_winter_sweeping_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/holt");
    let baseline = root.join("generated/slice-107-build/characters/holt");
    let set = std::env::var_os("FOM_HOLT_WINTER_SWEEPING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/holt-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/holt-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..140];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..140], prior);
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
    // In these pinned Winter sweeping strips, all four shades are skin only.
    // Check the gripping hand and adjacent sleeve in every loop frame. Hair,
    // moustache, eyes, clothing, boots, broom and dust stay original.
    // These literal landmarks and the whole-image oracle do not use mask seeds.
    let landmarks = [
        ("sweep_end_south", 0, 33, 48, 0x96523A, true),
        ("sweep_end_south", 0, 37, 51, 0x1D684E, false),
        ("sweep_loop_south", 0, 36, 37, 0x762E21, false),
        ("sweep_loop_south", 0, 34, 41, 0xB45732, false),
        ("sweep_loop_south", 0, 40, 40, 0x215259, false),
        ("sweep_loop_south", 0, 37, 36, 0x000000, false),
        ("sweep_loop_south", 0, 42, 47, 0xBB8151, false),
        ("sweep_loop_south", 0, 46, 53, 0xE9B03A, false),
        ("sweep_loop_south", 1, 38, 47, 0xCD9067, true),
        ("sweep_loop_south", 1, 55, 53, 0xA0A0A0, false),
        ("sweep_loop_south", 5, 40, 44, 0xF0CC8E, true),
        ("sweep_loop_south", 6, 39, 47, 0xCD9067, true),
        ("sweep_start_south", 0, 34, 48, 0xF0CC8E, true),
        ("sweep_loop_south", 0, 37, 53, 0xD37B52, false),
        ("sweep_loop_south", 0, 40, 42, 0xF0CC8E, true),
        ("sweep_loop_south", 0, 42, 42, 0x622A10, false),
        ("sweep_loop_south", 1, 40, 43, 0xF0CC8E, true),
        ("sweep_loop_south", 1, 42, 43, 0x622A10, false),
        ("sweep_loop_south", 2, 40, 43, 0xF0CC8E, true),
        ("sweep_loop_south", 2, 42, 43, 0x622A10, false),
        ("sweep_loop_south", 3, 40, 42, 0xF0CC8E, true),
        ("sweep_loop_south", 3, 42, 42, 0x622A10, false),
        ("sweep_loop_south", 4, 41, 42, 0xF0CC8E, true),
        ("sweep_loop_south", 4, 43, 42, 0x622A10, false),
        ("sweep_loop_south", 5, 41, 42, 0xF0CC8E, true),
        ("sweep_loop_south", 5, 43, 42, 0x622A10, false),
        ("sweep_loop_south", 6, 43, 43, 0xCD9067, true),
        ("sweep_loop_south", 6, 45, 43, 0xE37D45, false),
        ("sweep_loop_south", 7, 43, 43, 0xCD9067, true),
        ("sweep_loop_south", 7, 45, 43, 0xE37D45, false),
        ("sweep_loop_south", 8, 41, 42, 0xF0CC8E, true),
        ("sweep_loop_south", 8, 43, 42, 0x622A10, false),
        ("sweep_loop_south", 9, 41, 42, 0xF0CC8E, true),
        ("sweep_loop_south", 9, 43, 42, 0x622A10, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("sweep_end_south", &[42]),
        (
            "sweep_loop_south",
            &[48, 54, 50, 48, 51, 53, 59, 55, 53, 51],
        ),
        ("sweep_start_south", &[37]),
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
