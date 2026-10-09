use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/josephine and the local accepted Josephine world baseline"]
fn josephine_summer_polishing_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/josephine");
    let baseline = root.join("generated/slice-120-build/characters/josephine");
    let set = std::env::var_os("FOM_JOSEPHINE_SUMMER_POLISHING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/josephine-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/josephine-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..91];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..91], prior);
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
    let source = [0x9B643A, 0x7E442F, 0x713222, 0x57271D];
    // These three pinned sources use the four world shades exclusively for skin.
    // Exposed hands in start/end and ankles change. The loop's hands are hidden;
    // its cloth and glass stay original. The oracle does not use mask seeds.
    let landmarks = [
        ("polish_end_east", 0, 38, 33, 0x713222, true),
        ("polish_end_east", 0, 37, 52, 0x713222, true),
        ("polish_end_east", 0, 36, 28, 0x5F3737, false),
        ("polish_end_east", 0, 37, 42, 0xF6B486, false),
        ("polish_end_east", 0, 45, 44, 0x85CAEA, false),
        ("polish_end_east", 1, 39, 34, 0x713222, true),
        ("polish_end_east", 1, 38, 52, 0x713222, true),
        ("polish_end_east", 1, 37, 29, 0x5F3737, false),
        ("polish_end_east", 1, 38, 43, 0xF6B486, false),
        ("polish_loop_east", 0, 38, 33, 0x713222, true),
        ("polish_loop_east", 0, 37, 52, 0x713222, true),
        ("polish_loop_east", 0, 36, 28, 0x5F3737, false),
        ("polish_loop_east", 0, 37, 42, 0xF6B486, false),
        ("polish_loop_east", 0, 42, 43, 0x85CAEA, false),
        ("polish_loop_east", 0, 39, 44, 0x547BB0, false),
        ("polish_loop_east", 1, 38, 33, 0x713222, true),
        ("polish_loop_east", 1, 37, 52, 0x713222, true),
        ("polish_loop_east", 1, 36, 28, 0x5F3737, false),
        ("polish_loop_east", 1, 37, 42, 0xF6B486, false),
        ("polish_loop_east", 1, 42, 43, 0x85CAEA, false),
        ("polish_loop_east", 1, 39, 44, 0x547BB0, false),
        ("polish_loop_east", 2, 38, 33, 0x713222, true),
        ("polish_loop_east", 2, 37, 52, 0x713222, true),
        ("polish_loop_east", 2, 36, 28, 0x5F3737, false),
        ("polish_loop_east", 2, 37, 42, 0xF6B486, false),
        ("polish_loop_east", 2, 42, 43, 0x85CAEA, false),
        ("polish_loop_east", 2, 37, 44, 0x547BB0, false),
        ("polish_loop_east", 3, 38, 33, 0x713222, true),
        ("polish_loop_east", 3, 37, 52, 0x713222, true),
        ("polish_loop_east", 3, 36, 28, 0x5F3737, false),
        ("polish_loop_east", 3, 37, 42, 0xF6B486, false),
        ("polish_loop_east", 3, 42, 43, 0x85CAEA, false),
        ("polish_loop_east", 3, 37, 44, 0x547BB0, false),
        ("polish_start_east", 0, 39, 34, 0x713222, true),
        ("polish_start_east", 0, 38, 52, 0x713222, true),
        ("polish_start_east", 0, 37, 29, 0x5F3737, false),
        ("polish_start_east", 0, 38, 43, 0xF6B486, false),
        ("polish_start_east", 0, 45, 44, 0x85CAEA, false),
        ("polish_start_east", 1, 38, 33, 0x713222, true),
        ("polish_start_east", 1, 37, 52, 0x713222, true),
        ("polish_start_east", 1, 36, 28, 0x5F3737, false),
        ("polish_start_east", 1, 37, 42, 0xF6B486, false),
        ("polish_start_east", 1, 42, 43, 0x85CAEA, false),
        ("polish_start_east", 1, 31, 42, 0x547BB0, false),
        ("polish_start_east", 2, 38, 33, 0x713222, true),
        ("polish_start_east", 2, 37, 52, 0x713222, true),
        ("polish_start_east", 2, 36, 28, 0x5F3737, false),
        ("polish_start_east", 2, 37, 42, 0xF6B486, false),
        ("polish_start_east", 2, 42, 43, 0x85CAEA, false),
        ("polish_start_east", 2, 35, 44, 0x547BB0, false),
        ("polish_end_east", 0, 37, 46, 0x9B643A, true),
        ("polish_end_east", 0, 36, 45, 0x57271D, true),
        ("polish_end_east", 1, 34, 47, 0x9B643A, true),
        ("polish_end_east", 1, 36, 46, 0x57271D, true),
        ("polish_start_east", 0, 34, 47, 0x9B643A, true),
        ("polish_start_east", 0, 36, 46, 0x57271D, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("polish_end_east", &[59, 70]),
        ("polish_loop_east", &[49, 51, 49, 49]),
        ("polish_start_east", &[54, 48, 48]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [0, 9, 10, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_josephine_specialanimation_summer";
            let asset =
                format!("assets/animations/NPCs/Josephine/Sprites/Summer/{prefix}_{case}.png");
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
                // Exposed ankles below the hem are skin too; shoes stay original.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Josephine Summer material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin independently of the target colors.
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
