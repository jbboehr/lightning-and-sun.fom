use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/josephine and the local fixed Josephine world baseline"]
fn josephine_winter_polishing_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/josephine");
    let baseline = root.join("generated/slice-136-build/characters/josephine");
    let set = std::env::var_os("FOM_JOSEPHINE_WINTER_POLISHING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/josephine-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/josephine-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..157];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..157], prior);
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
    // Exposed hands in start/end change. Winter leggings stay original;
    // the loop hides the hands behind its cloth and glass. The oracle ignores mask seeds.
    let landmarks = [
        ("polish_end_east", 0, 38, 32, 0x713222, true),
        ("polish_end_east", 0, 37, 50, 0x3A1D24, false),
        ("polish_end_east", 0, 39, 26, 0x5F3737, false),
        ("polish_end_east", 0, 36, 42, 0xFAC0B5, false),
        ("polish_end_east", 0, 45, 44, 0x85CAEA, false),
        ("polish_end_east", 1, 39, 33, 0x713222, true),
        ("polish_end_east", 1, 38, 51, 0x3A1D24, false),
        ("polish_end_east", 1, 40, 27, 0x5F3737, false),
        ("polish_end_east", 1, 37, 43, 0xFAC0B5, false),
        ("polish_loop_east", 0, 38, 32, 0x713222, true),
        ("polish_loop_east", 0, 40, 51, 0x3A1D24, false),
        ("polish_loop_east", 0, 39, 26, 0x5F3737, false),
        ("polish_loop_east", 0, 36, 42, 0xFAC0B5, false),
        ("polish_loop_east", 0, 42, 43, 0x85CAEA, false),
        ("polish_loop_east", 0, 39, 44, 0x547BB0, false),
        ("polish_loop_east", 1, 38, 32, 0x713222, true),
        ("polish_loop_east", 1, 37, 50, 0x3A1D24, false),
        ("polish_loop_east", 1, 39, 26, 0x5F3737, false),
        ("polish_loop_east", 1, 36, 42, 0xFAC0B5, false),
        ("polish_loop_east", 1, 42, 43, 0x85CAEA, false),
        ("polish_loop_east", 1, 39, 44, 0x547BB0, false),
        ("polish_loop_east", 2, 38, 32, 0x713222, true),
        ("polish_loop_east", 2, 37, 51, 0x3A1D24, false),
        ("polish_loop_east", 2, 39, 26, 0x5F3737, false),
        ("polish_loop_east", 2, 36, 42, 0xFAC0B5, false),
        ("polish_loop_east", 2, 42, 43, 0x85CAEA, false),
        ("polish_loop_east", 2, 37, 44, 0x547BB0, false),
        ("polish_loop_east", 3, 38, 32, 0x713222, true),
        ("polish_loop_east", 3, 40, 50, 0x3A1D24, false),
        ("polish_loop_east", 3, 39, 26, 0x5F3737, false),
        ("polish_loop_east", 3, 36, 42, 0xFAC0B5, false),
        ("polish_loop_east", 3, 42, 43, 0x85CAEA, false),
        ("polish_loop_east", 3, 37, 44, 0x547BB0, false),
        ("polish_start_east", 0, 39, 33, 0x713222, true),
        ("polish_start_east", 0, 38, 51, 0x3A1D24, false),
        ("polish_start_east", 0, 40, 27, 0x5F3737, false),
        ("polish_start_east", 0, 37, 43, 0xFAC0B5, false),
        ("polish_start_east", 0, 45, 44, 0x85CAEA, false),
        ("polish_start_east", 1, 38, 32, 0x713222, true),
        ("polish_start_east", 1, 37, 50, 0x3A1D24, false),
        ("polish_start_east", 1, 39, 26, 0x5F3737, false),
        ("polish_start_east", 1, 36, 42, 0xFAC0B5, false),
        ("polish_start_east", 1, 42, 43, 0x85CAEA, false),
        ("polish_start_east", 1, 31, 42, 0x547BB0, false),
        ("polish_start_east", 2, 38, 32, 0x713222, true),
        ("polish_start_east", 2, 37, 50, 0x3A1D24, false),
        ("polish_start_east", 2, 39, 26, 0x5F3737, false),
        ("polish_start_east", 2, 36, 42, 0xFAC0B5, false),
        ("polish_start_east", 2, 42, 43, 0x85CAEA, false),
        ("polish_start_east", 2, 35, 44, 0x547BB0, false),
        ("polish_end_east", 0, 37, 46, 0x9B643A, true),
        ("polish_end_east", 0, 38, 47, 0x57271D, true),
        ("polish_end_east", 1, 34, 47, 0x9B643A, true),
        ("polish_end_east", 1, 35, 48, 0x57271D, true),
        ("polish_start_east", 0, 34, 47, 0x9B643A, true),
        ("polish_start_east", 0, 35, 48, 0x57271D, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("polish_end_east", &[47, 57]),
        ("polish_loop_east", &[41, 41, 41, 41]),
        ("polish_start_east", &[43, 41, 41]),
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
            let prefix = "spr_npc_josephine_specialanimation_winter";
            let asset =
                format!("assets/animations/NPCs/Josephine/Sprites/Winter/{prefix}_{case}.png");
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
                // Winter leggings and boots stay original; no ankles are exposed.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Josephine Winter material mismatch: {id} {case} [{x},{y}]"
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
