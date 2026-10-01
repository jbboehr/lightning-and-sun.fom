use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/valen and the local accepted Valen world baseline"]
fn valen_wedding_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/valen");
    let baseline = root.join("generated/slice-007-build/characters/valen");
    let set = std::env::var_os("FOM_VALEN_WEDDING_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/valen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_VALEN_WEDDING_WORLD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/valen-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..242];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..242], prior);
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
    let source = [0xFBD3A7, 0xEFA67A, 0xC37555, 0x762E21];
    // Reviewed landmarks distinguish moving skin from protected materials.
    // Literal per-frame counts guard brief and occluded skin exposure.
    let landmarks = [
        ("idle_east", 0, 46, 47, 0x000000, false),
        ("idle_east", 0, 40, 42, 0x3B3F5D, false),
        ("idle_east", 0, 38, 47, 0x49495B, false),
        ("idle_east", 0, 37, 34, 0x6E578A, false),
        ("idle_east", 0, 35, 47, 0x762E21, true),
        ("idle_east", 0, 41, 41, 0x787CA5, false),
        ("idle_east", 0, 40, 46, 0x9696A8, false),
        ("idle_east", 0, 43, 42, 0x9E528C, false),
        ("idle_east", 0, 40, 31, 0xA385B9, false),
        ("idle_east", 0, 43, 35, 0xA59DA2, false),
        ("idle_east", 0, 41, 48, 0xB3B3C1, false),
        ("idle_east", 0, 39, 35, 0xC2B9BE, false),
        ("idle_east", 0, 34, 47, 0xC37555, true),
        ("idle_east", 0, 37, 31, 0xD6C1DD, false),
        ("idle_east", 0, 38, 46, 0xD9D4D8, false),
        ("idle_east", 0, 38, 37, 0xECF0E9, false),
        ("idle_east", 0, 39, 52, 0xEFA67A, true),
        ("idle_east", 0, 42, 42, 0xF2A7E0, false),
        ("idle_east", 0, 37, 32, 0xF5F5F5, false),
        ("idle_east", 0, 36, 46, 0xFBD3A7, true),
        ("idle_east", 0, 34, 45, 0xFFFAF6, false),
        ("walk_east", 0, 46, 47, 0x000000, false),
        ("walk_east", 0, 40, 42, 0x3B3F5D, false),
        ("walk_east", 0, 38, 47, 0x49495B, false),
        ("walk_east", 0, 37, 34, 0x6E578A, false),
        ("walk_east", 0, 35, 47, 0x762E21, true),
        ("walk_east", 0, 41, 41, 0x787CA5, false),
        ("walk_east", 0, 40, 46, 0x9696A8, false),
        ("walk_east", 0, 43, 42, 0x9E528C, false),
        ("walk_east", 0, 40, 31, 0xA385B9, false),
        ("walk_east", 0, 43, 35, 0xA59DA2, false),
        ("walk_east", 0, 41, 48, 0xB3B3C1, false),
        ("walk_east", 0, 39, 35, 0xC2B9BE, false),
        ("walk_east", 0, 34, 47, 0xC37555, true),
        ("walk_east", 0, 37, 31, 0xD6C1DD, false),
        ("walk_east", 0, 38, 46, 0xD9D4D8, false),
        ("walk_east", 0, 38, 37, 0xECF0E9, false),
        ("walk_east", 0, 39, 52, 0xEFA67A, true),
        ("walk_east", 0, 42, 42, 0xF2A7E0, false),
        ("walk_east", 0, 37, 32, 0xF5F5F5, false),
        ("walk_east", 0, 36, 46, 0xFBD3A7, true),
        ("walk_east", 0, 34, 45, 0xFFFAF6, false),
        ("walk_east", 1, 33, 47, 0x762E21, true),
        ("walk_east", 1, 32, 47, 0xC37555, true),
        ("walk_east", 1, 42, 40, 0xEFA67A, true),
        ("walk_east", 1, 45, 46, 0xFBD3A7, true),
        ("walk_north", 1, 34, 48, 0x762E21, true),
        ("walk_north", 1, 45, 47, 0xC37555, true),
        ("walk_north", 1, 42, 53, 0xEFA67A, true),
        ("walk_north", 1, 35, 48, 0xFBD3A7, true),
        ("walk_south", 0, 40, 48, 0x000000, false),
        ("walk_south", 0, 39, 42, 0x3B3F5D, false),
        ("walk_south", 0, 38, 46, 0x49495B, false),
        ("walk_south", 0, 36, 34, 0x6E578A, false),
        ("walk_south", 0, 46, 47, 0x762E21, true),
        ("walk_south", 0, 40, 41, 0x787CA5, false),
        ("walk_south", 0, 40, 46, 0x9696A8, false),
        ("walk_south", 0, 42, 42, 0x9E528C, false),
        ("walk_south", 0, 42, 31, 0xA385B9, false),
        ("walk_south", 0, 38, 48, 0xB3B3C1, false),
        ("walk_south", 0, 41, 35, 0xC2B9BE, false),
        ("walk_south", 0, 37, 52, 0xC37555, true),
        ("walk_south", 0, 35, 31, 0xD6C1DD, false),
        ("walk_south", 0, 42, 46, 0xD9D4D8, false),
        ("walk_south", 0, 37, 37, 0xECF0E9, false),
        ("walk_south", 0, 41, 52, 0xEFA67A, true),
        ("walk_south", 0, 41, 42, 0xF2A7E0, false),
        ("walk_south", 0, 36, 32, 0xF5F5F5, false),
        ("walk_south", 0, 47, 46, 0xFBD3A7, true),
        ("walk_south", 0, 34, 45, 0xFFFAF6, false),
        ("walk_south", 1, 46, 47, 0x762E21, true),
        ("walk_south", 1, 35, 47, 0xC37555, true),
        ("walk_south", 1, 40, 40, 0xEFA67A, true),
        ("walk_south", 1, 34, 47, 0xFBD3A7, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[43]),
        ("idle_north", &[16]),
        ("idle_south", &[52]),
        ("walk_east", &[43, 46, 43, 41]),
        ("walk_north", &[16, 11, 16, 11]),
        ("walk_south", &[52, 49, 52, 49]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [8, 9, 10, 4]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_valen_wedding";
            let asset = format!("assets/animations/NPCs/Valen/Sprites/Wedding/{prefix}_{case}.png");
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
                // All reviewed world shades are skin in these Wedding strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Valen Wedding material mismatch: {id} {case} [{x},{y}]"
                );
                mask.push(p != q);
                if p != q {
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
        if let Some(first) = &common_mask {
            assert_eq!(first, &mask);
        } else {
            common_mask = Some(mask);
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
