use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/valen and the local accepted Valen world baseline"]
fn valen_beach_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/valen");
    let baseline = root.join("generated/slice-004-build/characters/valen");
    let set = std::env::var_os("FOM_VALEN_BEACH_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/valen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_VALEN_BEACH_WORLD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/valen-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..228];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..228], prior);
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
        ("idle_east", 0, 34, 48, 0x000000, false),
        ("idle_east", 0, 38, 44, 0x2E2D38, false),
        ("idle_east", 0, 40, 44, 0x4D4C53, false),
        ("idle_east", 0, 41, 52, 0x6264A0, false),
        ("idle_east", 0, 37, 40, 0x6E578A, false),
        ("idle_east", 0, 35, 47, 0x762E21, true),
        ("idle_east", 0, 37, 48, 0x874B9A, false),
        ("idle_east", 0, 41, 47, 0x8BBDBE, false),
        ("idle_east", 0, 42, 41, 0x8D9FB8, false),
        ("idle_east", 0, 40, 31, 0xA385B9, false),
        ("idle_east", 0, 43, 35, 0xA59DA2, false),
        ("idle_east", 0, 42, 45, 0xB571AF, false),
        ("idle_east", 0, 39, 35, 0xC2B9BE, false),
        ("idle_east", 0, 44, 44, 0xC37555, true),
        ("idle_east", 0, 39, 31, 0xD6C1DD, false),
        ("idle_east", 0, 40, 43, 0xD6DCE5, false),
        ("idle_east", 0, 38, 37, 0xECF0E9, false),
        ("idle_east", 0, 44, 45, 0xEFA67A, true),
        ("idle_east", 0, 41, 46, 0xF0B1DE, false),
        ("idle_east", 0, 37, 32, 0xF5F5F5, false),
        ("idle_east", 0, 36, 47, 0xFBD3A7, true),
        ("walk_east", 0, 34, 48, 0x000000, false),
        ("walk_east", 0, 38, 44, 0x2E2D38, false),
        ("walk_east", 0, 40, 44, 0x4D4C53, false),
        ("walk_east", 0, 41, 52, 0x6264A0, false),
        ("walk_east", 0, 37, 40, 0x6E578A, false),
        ("walk_east", 0, 35, 47, 0x762E21, true),
        ("walk_east", 0, 37, 48, 0x874B9A, false),
        ("walk_east", 0, 41, 47, 0x8BBDBE, false),
        ("walk_east", 0, 42, 41, 0x8D9FB8, false),
        ("walk_east", 0, 40, 31, 0xA385B9, false),
        ("walk_east", 0, 43, 35, 0xA59DA2, false),
        ("walk_east", 0, 42, 45, 0xB571AF, false),
        ("walk_east", 0, 39, 35, 0xC2B9BE, false),
        ("walk_east", 0, 44, 44, 0xC37555, true),
        ("walk_east", 0, 39, 31, 0xD6C1DD, false),
        ("walk_east", 0, 40, 43, 0xD6DCE5, false),
        ("walk_east", 0, 38, 37, 0xECF0E9, false),
        ("walk_east", 0, 44, 45, 0xEFA67A, true),
        ("walk_east", 0, 41, 46, 0xF0B1DE, false),
        ("walk_east", 0, 37, 32, 0xF5F5F5, false),
        ("walk_east", 0, 36, 47, 0xFBD3A7, true),
        ("walk_east", 1, 46, 47, 0x762E21, true),
        ("walk_east", 1, 36, 45, 0xC37555, true),
        ("walk_east", 1, 44, 43, 0xEFA67A, true),
        ("walk_east", 1, 45, 46, 0xFBD3A7, true),
        ("walk_north", 1, 46, 47, 0x762E21, true),
        ("walk_north", 1, 45, 46, 0xC37555, true),
        ("walk_north", 1, 41, 51, 0xEFA67A, true),
        ("walk_north", 1, 33, 47, 0xFBD3A7, true),
        ("walk_south", 0, 44, 48, 0x000000, false),
        ("walk_south", 0, 42, 43, 0x2E2D38, false),
        ("walk_south", 0, 39, 44, 0x4D4C53, false),
        ("walk_south", 0, 41, 52, 0x6264A0, false),
        ("walk_south", 0, 43, 40, 0x6E578A, false),
        ("walk_south", 0, 33, 47, 0x762E21, true),
        ("walk_south", 0, 43, 47, 0x874B9A, false),
        ("walk_south", 0, 39, 47, 0x8BBDBE, false),
        ("walk_south", 0, 41, 41, 0x8D9FB8, false),
        ("walk_south", 0, 39, 31, 0xA385B9, false),
        ("walk_south", 0, 38, 46, 0xB571AF, false),
        ("walk_south", 0, 41, 35, 0xC2B9BE, false),
        ("walk_south", 0, 34, 44, 0xC37555, true),
        ("walk_south", 0, 36, 31, 0xD6C1DD, false),
        ("walk_south", 0, 39, 43, 0xD6DCE5, false),
        ("walk_south", 0, 37, 37, 0xECF0E9, false),
        ("walk_south", 0, 42, 49, 0xEFA67A, true),
        ("walk_south", 0, 39, 46, 0xF0B1DE, false),
        ("walk_south", 0, 37, 32, 0xF5F5F5, false),
        ("walk_south", 0, 47, 46, 0xFBD3A7, true),
        ("walk_south", 1, 44, 46, 0x762E21, true),
        ("walk_south", 1, 44, 45, 0xC37555, true),
        ("walk_south", 1, 44, 44, 0xEFA67A, true),
        ("walk_south", 1, 33, 47, 0xFBD3A7, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[72]),
        ("idle_north", &[42]),
        ("idle_south", &[86]),
        ("walk_east", &[72, 82, 72, 67]),
        ("walk_north", &[42, 33, 42, 33]),
        ("walk_south", &[86, 72, 86, 74]),
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
            let prefix = "spr_npc_valen_beach";
            let asset = format!("assets/animations/NPCs/Valen/Sprites/Beach/{prefix}_{case}.png");
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
                // All reviewed world shades are skin in these Beach strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Valen Beach material mismatch: {id} {case} [{x},{y}]"
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
