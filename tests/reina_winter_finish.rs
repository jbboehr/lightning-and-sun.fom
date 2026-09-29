use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/reina-wedding-finish-study and the local accepted Reina world baseline"]
fn reina_winter_finish_covers_skin_and_preserves_kitchen_tools_and_outfit() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/reina-wedding-finish-study");
    let baseline = root
        .join("generated/characters-reina-juniper-march-winter-specials-trial/characters/reina");
    let set = std::env::var_os("FOM_REINA_WINTER_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/reina-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/reina-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..242];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..242], prior);
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
    let source = [0xB36844, 0x8E4538, 0x712922, 0x571D1F];
    // Fresh literal landmarks distinguish exposed fingers and face from knife,
    // green effects, glass, cloth, Winter sleeves and scarf-covered nape gaps.
    let landmarks = [
        ("chop_north", 0, 44, 46, 0x93524E, false),
        ("chop_north", 0, 45, 46, 0x93524E, false),
        ("chop_north", 0, 34, 43, 0x8E4538, true),
        ("chop_north", 0, 38, 50, 0x70645F, false),
        ("chop_north", 0, 41, 50, 0x70645F, false),
        ("chop_north", 0, 40, 43, 0x93524E, false),
        ("chop_north", 0, 40, 37, 0x63413B, false),
        ("chop_north", 1, 44, 42, 0x682E2E, false),
        ("chop_north", 1, 44, 43, 0x000000, false),
        ("chop_north", 1, 37, 49, 0x524642, false),
        ("chop_north", 1, 41, 49, 0x70645F, false),
        ("chop_north", 1, 40, 44, 0x93524E, false),
        ("chop_north", 3, 38, 50, 0x70645F, false),
        ("chop_north", 3, 41, 50, 0x70645F, false),
        ("chop_north", 3, 40, 44, 0x93524E, false),
        ("chop_north", 3, 45, 42, 0x682E2E, false),
        ("chop_north", 6, 37, 49, 0x524642, false),
        ("chop_north", 6, 42, 49, 0x524642, false),
        ("chop_north", 6, 40, 44, 0x93524E, false),
        ("chop_north", 10, 37, 49, 0x524642, false),
        ("chop_north", 10, 42, 49, 0x524642, false),
        ("chop_north", 10, 40, 44, 0x93524E, false),
        ("chop_north", 14, 37, 49, 0x524642, false),
        ("chop_north", 14, 42, 49, 0x524642, false),
        ("chop_north", 14, 40, 44, 0x93524E, false),
        ("chop_north", 18, 44, 46, 0x93524E, false),
        ("chop_north", 18, 45, 46, 0x93524E, false),
        ("chop_north", 18, 38, 50, 0x70645F, false),
        ("chop_north", 18, 41, 50, 0x70645F, false),
        ("chop_north", 18, 40, 43, 0x93524E, false),
        ("chop_north", 19, 33, 46, 0xB36844, true),
        ("chop_north", 19, 34, 47, 0xB36844, true),
        ("chop_north", 19, 45, 46, 0xB36844, true),
        ("chop_north", 19, 37, 49, 0x524642, false),
        ("chop_north", 19, 42, 49, 0x524642, false),
        ("chop_north", 19, 39, 43, 0x93524E, false),
        ("polish_start_east", 0, 40, 36, 0xB36844, true),
        ("polish_start_east", 0, 42, 42, 0xEABF67, false),
        ("polish_start_east", 0, 37, 44, 0x93524E, false),
        ("polish_start_east", 0, 35, 46, 0x93524E, false),
        ("polish_start_east", 0, 36, 48, 0xB36844, true),
        ("polish_start_east", 0, 40, 46, 0x3C3431, false),
        ("polish_start_east", 0, 39, 50, 0x70645F, false),
        ("polish_start_east", 0, 45, 46, 0xFFFFFF, false),
        ("polish_start_east", 0, 46, 46, 0xFFFFFF, false),
        ("polish_start_east", 0, 43, 43, 0x682E2E, false),
        ("polish_start_east", 0, 40, 31, 0x63413B, false),
        ("polish_start_east", 1, 40, 35, 0xB36844, true),
        ("polish_start_east", 1, 41, 41, 0xEABF67, false),
        ("polish_start_east", 1, 36, 43, 0x93524E, false),
        ("polish_start_east", 1, 38, 45, 0x3C3431, false),
        ("polish_start_east", 1, 38, 49, 0x70645F, false),
        ("polish_start_east", 1, 41, 49, 0x70645F, false),
        ("polish_start_east", 1, 43, 43, 0x85CAEA, false),
        ("polish_start_east", 1, 44, 45, 0x85CAEA, false),
        ("polish_start_east", 1, 32, 43, 0xF5F5F5, false),
        ("polish_start_east", 1, 33, 43, 0xC3D1DD, false),
        ("polish_start_east", 1, 35, 45, 0xC3D1DD, false),
        ("polish_start_east", 1, 36, 45, 0x000000, false),
        ("polish_start_east", 2, 40, 35, 0xB36844, true),
        ("polish_start_east", 2, 41, 41, 0xEABF67, false),
        ("polish_start_east", 2, 37, 43, 0x93524E, false),
        ("polish_start_east", 2, 38, 49, 0x70645F, false),
        ("polish_start_east", 2, 41, 49, 0x70645F, false),
        ("polish_start_east", 2, 43, 43, 0x85CAEA, false),
        ("polish_start_east", 2, 32, 45, 0xC3D1DD, false),
        ("polish_start_east", 2, 34, 46, 0xF5F5F5, false),
        ("polish_loop_east", 0, 40, 35, 0xB36844, true),
        ("polish_loop_east", 0, 41, 41, 0xEABF67, false),
        ("polish_loop_east", 0, 38, 42, 0x3A1A1A, false),
        ("polish_loop_east", 0, 36, 44, 0x682E2E, false),
        ("polish_loop_east", 0, 40, 44, 0xC3D1DD, false),
        ("polish_loop_east", 0, 41, 44, 0x547BB0, false),
        ("polish_loop_east", 0, 42, 45, 0x000000, false),
        ("polish_loop_east", 0, 44, 45, 0x85CAEA, false),
        ("polish_loop_east", 0, 40, 50, 0x000000, false),
        ("polish_loop_east", 0, 42, 50, 0x000000, false),
        ("polish_loop_east", 1, 40, 35, 0xB36844, true),
        ("polish_loop_east", 1, 41, 41, 0xEABF67, false),
        ("polish_loop_east", 1, 38, 42, 0x3A1A1A, false),
        ("polish_loop_east", 1, 36, 44, 0x682E2E, false),
        ("polish_loop_east", 1, 40, 44, 0xC3D1DD, false),
        ("polish_loop_east", 1, 41, 44, 0x547BB0, false),
        ("polish_loop_east", 1, 42, 45, 0x000000, false),
        ("polish_loop_east", 1, 44, 45, 0x85CAEA, false),
        ("polish_loop_east", 1, 40, 50, 0x547BB0, false),
        ("polish_loop_east", 1, 42, 50, 0x000000, false),
        ("polish_loop_east", 2, 40, 35, 0xB36844, true),
        ("polish_loop_east", 2, 41, 41, 0xEABF67, false),
        ("polish_loop_east", 2, 37, 42, 0x3A1A1A, false),
        ("polish_loop_east", 2, 36, 44, 0x000000, false),
        ("polish_loop_east", 2, 40, 44, 0x000000, false),
        ("polish_loop_east", 2, 41, 44, 0xFFFFFF, false),
        ("polish_loop_east", 2, 42, 45, 0xFFFFFF, false),
        ("polish_loop_east", 2, 44, 45, 0x85CAEA, false),
        ("polish_loop_east", 2, 40, 50, 0x000000, false),
        ("polish_loop_east", 2, 42, 50, 0x000000, false),
        ("polish_loop_east", 3, 40, 35, 0xB36844, true),
        ("polish_loop_east", 3, 41, 41, 0xEABF67, false),
        ("polish_loop_east", 3, 37, 42, 0x3A1A1A, false),
        ("polish_loop_east", 3, 36, 44, 0x000000, false),
        ("polish_loop_east", 3, 40, 44, 0x000000, false),
        ("polish_loop_east", 3, 41, 44, 0xFFFFFF, false),
        ("polish_loop_east", 3, 42, 45, 0xFFFFFF, false),
        ("polish_loop_east", 3, 44, 45, 0x85CAEA, false),
        ("polish_loop_east", 3, 40, 50, 0xD0B992, false),
        ("polish_loop_east", 3, 42, 50, 0x000000, false),
        ("polish_end_east", 0, 40, 35, 0xB36844, true),
        ("polish_end_east", 0, 41, 41, 0xEABF67, false),
        ("polish_end_east", 0, 37, 44, 0x93524E, false),
        ("polish_end_east", 0, 38, 44, 0x000000, false),
        ("polish_end_east", 0, 41, 48, 0x3C3431, false),
        ("polish_end_east", 0, 44, 44, 0x5A9DDA, false),
        ("polish_end_east", 0, 45, 46, 0xFFFFFF, false),
        ("polish_end_east", 0, 38, 49, 0x70645F, false),
        ("polish_end_east", 1, 40, 36, 0xB36844, true),
        ("polish_end_east", 1, 42, 42, 0xEABF67, false),
        ("polish_end_east", 1, 37, 44, 0x93524E, false),
        ("polish_end_east", 1, 35, 46, 0x93524E, false),
        ("polish_end_east", 1, 36, 48, 0xB36844, true),
        ("polish_end_east", 1, 40, 46, 0x3C3431, false),
        ("polish_end_east", 1, 39, 50, 0x70645F, false),
        ("polish_end_east", 1, 40, 44, 0xD68D47, false),
        ("chop_north", 0, 47, 40, 0xA1BBC0, false),
        ("chop_north", 0, 46, 42, 0x423C39, false),
        ("chop_north", 1, 48, 30, 0xCEE9EE, false),
        ("chop_north", 1, 47, 32, 0x8A9B9E, false),
        ("chop_north", 2, 49, 35, 0x3B6D5B, false),
        ("chop_north", 2, 49, 36, 0x68CCA7, false),
        ("chop_north", 2, 50, 36, 0x82E5C0, false),
        ("chop_north", 0, 39, 42, 0xFFF5D8, false),
        ("chop_north", 0, 40, 42, 0xFFF5D8, false),
        ("chop_north", 0, 37, 42, 0xFFF5D8, false),
        ("chop_north", 0, 38, 43, 0x93524E, false),
        ("chop_north", 0, 39, 44, 0x93524E, false),
        ("chop_north", 0, 39, 45, 0x93524E, false),
        ("chop_north", 0, 38, 53, 0x3C3431, false),
        ("chop_north", 0, 38, 52, 0x3C3431, false),
        ("chop_north", 2, 39, 40, 0xA27F4E, false),
        ("chop_north", 2, 39, 41, 0xFFF5D8, false),
        ("chop_north", 2, 39, 42, 0x93524E, false),
        ("chop_north", 2, 39, 43, 0x93524E, false),
        ("chop_north", 2, 39, 44, 0x93524E, false),
        ("chop_north", 2, 37, 50, 0xFFF5D8, false),
        ("chop_north", 2, 38, 53, 0x000000, false),
        ("polish_start_east", 0, 38, 42, 0xFFF5D8, false),
        ("polish_start_east", 0, 39, 43, 0x9E512F, false),
        ("polish_start_east", 0, 40, 44, 0xD68D47, false),
        ("polish_start_east", 0, 40, 45, 0xD68D47, false),
        ("polish_start_east", 0, 39, 47, 0x3C3431, false),
        ("polish_start_east", 0, 40, 50, 0x000000, false),
        ("polish_start_east", 0, 40, 53, 0x70645F, false),
        ("polish_loop_east", 0, 38, 43, 0x000000, false),
        ("polish_loop_east", 0, 39, 44, 0x547BB0, false),
        ("polish_loop_east", 0, 41, 45, 0xC3D1DD, false),
        ("polish_loop_east", 0, 41, 53, 0x3C3431, false),
        ("polish_end_east", 0, 38, 42, 0x3A1A1A, false),
        ("polish_end_east", 0, 38, 43, 0x000000, false),
        ("polish_end_east", 0, 38, 45, 0x682E2E, false),
        ("polish_end_east", 0, 37, 46, 0xB36844, true),
        ("polish_end_east", 0, 38, 47, 0x571D1F, true),
        ("polish_end_east", 0, 40, 53, 0x000000, false),
        ("polish_start_east", 1, 38, 42, 0x9E512F, false),
        ("polish_start_east", 2, 38, 42, 0x9E512F, false),
        ("polish_start_east", 0, 42, 43, 0x9E512F, false),
        ("polish_loop_east", 2, 38, 42, 0x9E512F, false),
        ("polish_loop_east", 3, 38, 42, 0x9E512F, false),
        ("polish_end_east", 0, 41, 42, 0x9E512F, false),
        ("polish_end_east", 1, 39, 43, 0x9E512F, false),
        ("polish_end_east", 1, 42, 43, 0x9E512F, false),
        ("polish_start_east", 0, 34, 47, 0xB36844, true),
        ("polish_start_east", 0, 34, 46, 0x682E2E, false),
        ("polish_start_east", 0, 35, 48, 0x571D1F, true),
        ("chop_north", 6, 39, 40, 0xA27F4E, false),
        ("chop_north", 10, 39, 40, 0xA27F4E, false),
        ("chop_north", 14, 39, 40, 0xA27F4E, false),
    ];
    let cases: [(&str, &[usize]); 4] = [
        (
            "chop_north",
            &[9, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 9, 12],
        ),
        ("polish_start_east", &[37, 35, 35]),
        ("polish_loop_east", &[35, 35, 35, 35]),
        ("polish_end_east", &[41, 54]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = preset["colors"].as_array().unwrap()[7..11]
            .iter()
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for (case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_reina_specialanimation_winter";
            let asset = format!("assets/animations/NPCs/Reina/Sprites/Winter/{prefix}_{case}.png");
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
                // All four reviewed world shades are skin in these Winter strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Reina Winter material mismatch: {id} {case} [{x},{y}]"
                );
                mask.push(p != q);
                if p != q {
                    let index = source.iter().position(|c| rgba(*c) == p.0).expect(
                        "kitchen tools, effects, hair, clothes or another non-skin color changed",
                    );
                    assert_eq!(q.0, rgba(target[index]));
                    per_frame[x as usize / 80] += 1;
                }
            }
            // The Winter scarf and sleeves conceal all skin in four chop poses.
            let empty: Vec<_> = per_frame
                .iter()
                .enumerate()
                .filter_map(|(frame, n)| (*n == 0).then_some(frame))
                .collect();
            assert_eq!(
                empty,
                if case == "chop_north" {
                    vec![2, 6, 10, 14]
                } else {
                    vec![]
                }
            );
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
