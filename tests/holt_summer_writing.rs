use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/holt and the local accepted Holt world baseline"]
fn holt_summer_writing_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/holt");
    let baseline = root.join("generated/slice-093-build/characters/holt");
    let set = std::env::var_os("FOM_HOLT_SUMMER_WRITING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/holt-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/holt-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..82];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..82], prior);
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
    // These pinned writing strips have skin above row 51; footwear starts below it.
    // Include the lowered hand under the board through row 49.
    // Hair, moustache, eye detail, clothes, board and pen stay original.
    let landmarks = [
        ("write_end_south", 0, 35, 46, 0x000000, false),
        ("write_end_south", 0, 36, 49, 0x296552, false),
        ("write_end_south", 0, 32, 42, 0x373357, false),
        ("write_end_south", 0, 37, 48, 0x429E81, false),
        ("write_end_south", 0, 43, 46, 0x57342B, false),
        ("write_end_south", 0, 33, 42, 0x6B5C7C, false),
        ("write_end_south", 0, 39, 30, 0x762E21, false),
        ("write_end_south", 0, 44, 45, 0x7D4F3D, false),
        ("write_end_south", 0, 45, 43, 0x837CA0, false),
        ("write_end_south", 0, 31, 45, 0x96523A, true),
        ("write_end_south", 0, 46, 43, 0xA8A5B5, false),
        ("write_end_south", 0, 44, 35, 0xAD6945, true),
        ("write_end_south", 0, 42, 32, 0xB65932, false),
        ("write_end_south", 0, 31, 37, 0xBA7F3E, false),
        ("write_end_south", 0, 37, 44, 0xC44748, false),
        ("write_end_south", 0, 42, 44, 0xC5895C, false),
        ("write_end_south", 0, 39, 41, 0xCC6B73, false),
        ("write_end_south", 0, 42, 51, 0xCD9067, false),
        ("write_end_south", 0, 30, 43, 0xCD9067, true),
        ("write_end_south", 0, 44, 30, 0xD2793F, false),
        ("write_end_south", 0, 41, 50, 0xE17154, false),
        ("write_end_south", 0, 33, 44, 0xE3779D, false),
        ("write_end_south", 0, 38, 53, 0xF0CC8E, false),
        ("write_end_south", 0, 42, 47, 0xF0CC8E, true),
        ("write_end_south", 0, 37, 30, 0xF1A354, false),
        ("write_end_south", 0, 47, 42, 0xF5F5F5, false),
        ("write_end_south", 0, 39, 43, 0xFF9CA1, false),
        ("write_end_south", 0, 41, 42, 0xFFC5C8, false),
        ("write_end_south", 0, 32, 37, 0xFFDA6A, false),
        ("write_end_south", 0, 40, 40, 0xFFF0F0, false),
        ("write_end_south", 1, 39, 31, 0x762E21, false),
        ("write_end_south", 1, 42, 39, 0x96523A, true),
        ("write_end_south", 1, 32, 48, 0xCD9067, true),
        ("write_end_south", 1, 46, 48, 0xF0CC8E, true),
        ("write_loop_south", 0, 37, 47, 0x000000, false),
        ("write_loop_south", 0, 38, 49, 0x296552, false),
        ("write_loop_south", 0, 36, 41, 0x373357, false),
        ("write_loop_south", 0, 37, 48, 0x429E81, false),
        ("write_loop_south", 0, 43, 46, 0x57342B, false),
        ("write_loop_south", 0, 38, 43, 0x6B5C7C, false),
        ("write_loop_south", 0, 39, 30, 0x762E21, false),
        ("write_loop_south", 0, 44, 45, 0x7D4F3D, false),
        ("write_loop_south", 0, 45, 43, 0x837CA0, false),
        ("write_loop_south", 0, 38, 38, 0x96523A, true),
        ("write_loop_south", 0, 43, 35, 0xA59DA2, false),
        ("write_loop_south", 0, 46, 43, 0xA8A5B5, false),
        ("write_loop_south", 0, 44, 35, 0xAD6945, true),
        ("write_loop_south", 0, 38, 32, 0xB65932, false),
        ("write_loop_south", 0, 35, 40, 0xBA7F3E, false),
        ("write_loop_south", 0, 38, 35, 0xC2B9BE, false),
        ("write_loop_south", 0, 37, 50, 0xC44748, false),
        ("write_loop_south", 0, 42, 44, 0xC5895C, false),
        ("write_loop_south", 0, 40, 41, 0xCC6B73, false),
        ("write_loop_south", 0, 42, 51, 0xCD9067, false),
        ("write_loop_south", 0, 36, 45, 0xCD9067, true),
        ("write_loop_south", 0, 44, 30, 0xD2793F, false),
        ("write_loop_south", 0, 38, 52, 0xE17154, false),
        ("write_loop_south", 0, 39, 44, 0xE3779D, false),
        ("write_loop_south", 0, 43, 36, 0xECF0E9, false),
        ("write_loop_south", 0, 38, 53, 0xF0CC8E, false),
        ("write_loop_south", 0, 42, 47, 0xF0CC8E, true),
        ("write_loop_south", 0, 37, 30, 0xF1A354, false),
        ("write_loop_south", 0, 47, 42, 0xF5F5F5, false),
        ("write_loop_south", 0, 39, 42, 0xFF9CA1, false),
        ("write_loop_south", 0, 41, 42, 0xFFC5C8, false),
        ("write_loop_south", 0, 34, 40, 0xFFDA6A, false),
        ("write_loop_south", 0, 40, 40, 0xFFF0F0, false),
        ("write_loop_south", 0, 35, 45, 0xFFFFFF, false),
        ("write_loop_south", 1, 39, 30, 0x762E21, false),
        ("write_loop_south", 1, 38, 38, 0x96523A, true),
        ("write_loop_south", 1, 44, 35, 0xAD6945, true),
        ("write_loop_south", 1, 36, 45, 0xCD9067, true),
        ("write_loop_south", 1, 42, 47, 0xF0CC8E, true),
        ("write_loop_south", 2, 38, 38, 0x96523A, true),
        ("write_loop_south", 2, 44, 35, 0xAD6945, true),
        ("write_loop_south", 2, 35, 44, 0xCD9067, true),
        ("write_loop_south", 2, 42, 48, 0xF0CC8E, true),
        ("write_loop_south", 3, 38, 38, 0x96523A, true),
        ("write_loop_south", 3, 44, 35, 0xAD6945, true),
        ("write_loop_south", 3, 35, 45, 0xCD9067, true),
        ("write_loop_south", 3, 42, 47, 0xF0CC8E, true),
        ("write_start_south", 0, 31, 47, 0x000000, false),
        ("write_start_south", 0, 38, 50, 0x296552, false),
        ("write_start_south", 0, 34, 45, 0x373357, false),
        ("write_start_south", 0, 38, 49, 0x429E81, false),
        ("write_start_south", 0, 46, 47, 0x57342B, false),
        ("write_start_south", 0, 34, 46, 0x6B5C7C, false),
        ("write_start_south", 0, 39, 31, 0x762E21, false),
        ("write_start_south", 0, 47, 46, 0x7D4F3D, false),
        ("write_start_south", 0, 48, 44, 0x837CA0, false),
        ("write_start_south", 0, 42, 39, 0x96523A, true),
        ("write_start_south", 0, 49, 44, 0xA8A5B5, false),
        ("write_start_south", 0, 35, 34, 0xB65932, false),
        ("write_start_south", 0, 32, 41, 0xBA7F3E, false),
        ("write_start_south", 0, 42, 45, 0xC44748, false),
        ("write_start_south", 0, 45, 45, 0xC5895C, false),
        ("write_start_south", 0, 39, 42, 0xCC6B73, false),
        ("write_start_south", 0, 42, 51, 0xCD9067, false),
        ("write_start_south", 0, 32, 48, 0xCD9067, true),
        ("write_start_south", 0, 43, 31, 0xD2793F, false),
        ("write_start_south", 0, 39, 47, 0xE17154, false),
        ("write_start_south", 0, 34, 48, 0xE3779D, false),
        ("write_start_south", 0, 38, 53, 0xF0CC8E, false),
        ("write_start_south", 0, 46, 48, 0xF0CC8E, true),
        ("write_start_south", 0, 36, 31, 0xF1A354, false),
        ("write_start_south", 0, 50, 43, 0xF5F5F5, false),
        ("write_start_south", 0, 40, 43, 0xFF9CA1, false),
        ("write_start_south", 0, 45, 43, 0xFFC5C8, false),
        ("write_start_south", 0, 33, 41, 0xFFDA6A, false),
        ("write_start_south", 0, 40, 41, 0xFFF0F0, false),
        ("write_start_south", 1, 39, 30, 0x762E21, false),
        ("write_start_south", 1, 31, 45, 0x96523A, true),
        ("write_start_south", 1, 44, 35, 0xAD6945, true),
        ("write_start_south", 1, 30, 43, 0xCD9067, true),
        ("write_start_south", 1, 42, 47, 0xF0CC8E, true),
        ("write_start_south", 0, 45, 49, 0xF0CC8E, true),
        ("write_start_south", 0, 44, 49, 0xCD9067, true),
        ("write_start_south", 0, 37, 53, 0xF0CC8E, false),
        ("write_start_south", 0, 46, 44, 0xC5895C, false),
        ("write_start_south", 0, 34, 48, 0xE3779D, false),
        ("write_loop_south", 0, 37, 37, 0x762E21, false),
        ("write_loop_south", 0, 37, 44, 0xCD9067, true),
        ("write_loop_south", 0, 44, 45, 0x7D4F3D, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("write_end_south", &[48, 45]),
        ("write_loop_south", &[41, 42, 42, 42]),
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
            let prefix = "spr_npc_holt_specialanimation_summer";
            let asset = format!("assets/animations/NPCs/Holt/Sprites/Summer/{prefix}_{case}.png");
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
                // Literal source bounds are independent of mask seeds.
                let in_skin_area = y < 51;
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && in_skin_area)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Holt Summer material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin even when a Ryis target equals its source.
                if in_skin_area && source.iter().any(|c| rgba(*c) == p.0) {
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
