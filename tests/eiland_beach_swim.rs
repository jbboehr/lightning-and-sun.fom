use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/eiland and the local accepted Eiland world baseline"]
fn eiland_beach_swim_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/eiland");
    let baseline = root.join("generated/slice-012-build/characters/eiland");
    let set = std::env::var_os("FOM_EILAND_BEACH_SWIM_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_BEACH_SWIM_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..260];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..260], prior);
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
    let source = [0xE9A980, 0xDE8F5D, 0xBA6A4C, 0x9C5241, 0x7D3B14];
    // Reviewed landmarks distinguish moving skin from protected materials.
    // Literal per-frame counts guard brief and occluded skin exposure.
    let landmarks = [
        ("bath_swim_east", 0, 39, 49, 0x000000, false),
        ("bath_swim_east", 0, 35, 56, 0x328BC9, false),
        ("bath_swim_east", 0, 37, 51, 0x6C2859, false),
        ("bath_swim_east", 0, 38, 54, 0x7D3B14, true),
        ("bath_swim_east", 0, 41, 48, 0x9C5241, true),
        ("bath_swim_east", 0, 43, 55, 0x9DEBFC, false),
        ("bath_swim_east", 0, 39, 47, 0xA54E7F, false),
        ("bath_swim_east", 0, 38, 53, 0xBA6A4C, true),
        ("bath_swim_east", 0, 39, 50, 0xC2B9BE, false),
        ("bath_swim_east", 0, 40, 54, 0xDE8F5D, true),
        ("bath_swim_east", 0, 37, 47, 0xE797AC, false),
        ("bath_swim_east", 0, 36, 52, 0xE9A980, true),
        ("bath_swim_east", 0, 38, 52, 0xECF0E9, false),
        ("bath_swim_east", 0, 42, 47, 0xFCDAE0, false),
        ("bath_swim_east", 0, 44, 46, 0xFFFFFF, false),
        ("bath_swim_east", 1, 38, 54, 0x7D3B14, true),
        ("bath_swim_east", 1, 41, 48, 0x9C5241, true),
        ("bath_swim_east", 1, 38, 53, 0xBA6A4C, true),
        ("bath_swim_east", 1, 40, 54, 0xDE8F5D, true),
        ("bath_swim_east", 1, 36, 52, 0xE9A980, true),
        ("bath_swim_south", 0, 37, 49, 0x000000, false),
        ("bath_swim_south", 0, 36, 56, 0x328BC9, false),
        ("bath_swim_south", 0, 34, 52, 0x6C2859, false),
        ("bath_swim_south", 0, 42, 54, 0x7D3B14, true),
        ("bath_swim_south", 0, 40, 48, 0x9C5241, true),
        ("bath_swim_south", 0, 31, 59, 0x9DEBFC, false),
        ("bath_swim_south", 0, 38, 47, 0xA54E7F, false),
        ("bath_swim_south", 0, 42, 53, 0xBA6A4C, true),
        ("bath_swim_south", 0, 38, 50, 0xC2B9BE, false),
        ("bath_swim_south", 0, 39, 54, 0xDE8F5D, true),
        ("bath_swim_south", 0, 36, 47, 0xE797AC, false),
        ("bath_swim_south", 0, 35, 52, 0xE9A980, true),
        ("bath_swim_south", 0, 37, 52, 0xECF0E9, false),
        ("bath_swim_south", 0, 41, 47, 0xFCDAE0, false),
        ("bath_swim_south", 0, 43, 46, 0xFFFFFF, false),
        ("bath_swim_south", 1, 42, 54, 0x7D3B14, true),
        ("bath_swim_south", 1, 40, 48, 0x9C5241, true),
        ("bath_swim_south", 1, 42, 53, 0xBA6A4C, true),
        ("bath_swim_south", 1, 39, 54, 0xDE8F5D, true),
        ("bath_swim_south", 1, 35, 52, 0xE9A980, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("bath_swim_east", &[23, 23, 18, 18]),
        ("bath_swim_south", &[24, 24, 18, 18]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [0, 9, 10, 3, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_eiland_beach";
            let asset = format!("assets/animations/NPCs/Eiland/Sprites/Beach/{prefix}_{case}.png");
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
                    "Eiland Beach material mismatch: {id} {case} [{x},{y}]"
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
