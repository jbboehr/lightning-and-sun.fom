use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/holt and the local accepted Holt world baseline"]
fn holt_summer_whittling_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/holt");
    let baseline = root.join("generated/slice-094-build/characters/holt");
    let set = std::env::var_os("FOM_HOLT_SUMMER_WHITTLING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/holt-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/holt-world-trial.json")).unwrap(),
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
    let source = [0xF0CC8E, 0xCD9067, 0x96523A, 0xAD6945];
    // In this pinned strip, skin is above row 51; footwear reuses skin shades below.
    // The wood's pale highlights and flying chips are separate from skin.
    // Hair, moustache, eye detail, clothes, footwear, wood and knife stay original.
    let landmarks = [
        ("whittle_east", 0, 38, 48, 0x000000, false),
        ("whittle_east", 0, 41, 50, 0x296552, false),
        ("whittle_east", 0, 46, 46, 0x3E2516, false),
        ("whittle_east", 0, 40, 49, 0x429E81, false),
        ("whittle_east", 0, 43, 47, 0x653D25, false),
        ("whittle_east", 0, 43, 44, 0x6C7781, false),
        ("whittle_east", 0, 40, 31, 0x762E21, false),
        ("whittle_east", 0, 44, 47, 0x936244, false),
        ("whittle_east", 0, 38, 53, 0x96523A, false),
        ("whittle_east", 0, 40, 45, 0x96523A, true),
        ("whittle_east", 0, 44, 35, 0xA59DA2, false),
        ("whittle_east", 0, 45, 36, 0xAD6945, true),
        ("whittle_east", 0, 39, 33, 0xB65932, false),
        ("whittle_east", 0, 45, 44, 0xBB8151, false),
        ("whittle_east", 0, 40, 35, 0xC2B9BE, false),
        ("whittle_east", 0, 41, 48, 0xC44748, false),
        ("whittle_east", 0, 41, 42, 0xCC6B73, false),
        ("whittle_east", 0, 43, 51, 0xCD9067, false),
        ("whittle_east", 0, 38, 46, 0xCD9067, true),
        ("whittle_east", 0, 45, 31, 0xD2793F, false),
        ("whittle_east", 0, 42, 44, 0xDAE2E7, false),
        ("whittle_east", 0, 44, 52, 0xE17154, false),
        ("whittle_east", 0, 39, 37, 0xECF0E9, false),
        ("whittle_east", 0, 40, 53, 0xF0CC8E, false),
        ("whittle_east", 0, 40, 46, 0xF0CC8E, true),
        ("whittle_east", 0, 38, 31, 0xF1A354, false),
        ("whittle_east", 0, 37, 43, 0xFF9CA1, false),
        ("whittle_east", 0, 38, 43, 0xFFC5C8, false),
        ("whittle_east", 0, 42, 41, 0xFFF0F0, false),
        ("whittle_east", 0, 37, 46, 0xFFFFFF, false),
        ("whittle_east", 1, 40, 44, 0x96523A, true),
        ("whittle_east", 1, 45, 35, 0xAD6945, true),
        ("whittle_east", 1, 38, 46, 0xCD9067, true),
        ("whittle_east", 1, 40, 45, 0xF0CC8E, true),
        ("whittle_east", 2, 39, 38, 0x96523A, true),
        ("whittle_east", 2, 45, 35, 0xAD6945, true),
        ("whittle_east", 2, 38, 43, 0xCD9067, true),
        ("whittle_east", 2, 40, 43, 0xF0CC8E, true),
        ("whittle_east", 3, 40, 45, 0x96523A, true),
        ("whittle_east", 3, 45, 36, 0xAD6945, true),
        ("whittle_east", 3, 38, 46, 0xCD9067, true),
        ("whittle_east", 3, 40, 46, 0xF0CC8E, true),
        ("whittle_east", 4, 40, 45, 0x96523A, true),
        ("whittle_east", 4, 46, 35, 0xAD6945, true),
        ("whittle_east", 4, 38, 47, 0xCD9067, true),
        ("whittle_east", 4, 40, 46, 0xF0CC8E, true),
        ("whittle_east", 5, 40, 45, 0x96523A, true),
        ("whittle_east", 5, 46, 35, 0xAD6945, true),
        ("whittle_east", 5, 38, 47, 0xCD9067, true),
        ("whittle_east", 5, 40, 46, 0xF0CC8E, true),
        ("whittle_east", 6, 40, 45, 0x96523A, true),
        ("whittle_east", 6, 46, 35, 0xAD6945, true),
        ("whittle_east", 6, 38, 47, 0xCD9067, true),
        ("whittle_east", 6, 40, 46, 0xF0CC8E, true),
        ("whittle_east", 7, 40, 45, 0x96523A, true),
        ("whittle_east", 7, 46, 35, 0xAD6945, true),
        ("whittle_east", 7, 38, 47, 0xCD9067, true),
        ("whittle_east", 7, 40, 46, 0xF0CC8E, true),
        ("whittle_east", 8, 40, 45, 0x96523A, true),
        ("whittle_east", 8, 46, 35, 0xAD6945, true),
        ("whittle_east", 8, 38, 47, 0xCD9067, true),
        ("whittle_east", 8, 40, 46, 0xF0CC8E, true),
        ("whittle_east", 0, 40, 45, 0x96523A, true),
        ("whittle_east", 6, 45, 41, 0xE7BA83, false),
        ("whittle_east", 6, 54, 44, 0xC09165, false),
        ("whittle_east", 6, 43, 46, 0xDAE2E7, false),
        ("whittle_east", 8, 39, 51, 0xF0CC8E, false),
        ("whittle_east", 0, 38, 38, 0x762E21, false),
    ];
    let cases: &[(&str, &[usize])] = &[("whittle_east", &[34, 35, 31, 34, 34, 34, 34, 34, 34])];
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
