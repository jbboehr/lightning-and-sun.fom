use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/reina-winter-standard-study and the local accepted Reina world baseline"]
fn reina_winter_world_covers_skin_and_preserves_outfit_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/reina-winter-standard-study");
    let baseline = root.join("generated/characters-march-autumn-injured-trial/characters/reina");
    let set = std::env::var_os("FOM_REINA_WINTER_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/reina-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/reina-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..211];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..211], prior);
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
    // Literal source landmarks distinguish the face and bare hands from brown
    // sleeves, trousers, boots and scarf shading that reuses portrait skin #9E512F.
    let landmarks = [
        ("idle_east", 0, 40, 35, 0xB36844, true),
        ("idle_east", 0, 40, 42, 0xD68D47, false),
        ("idle_east", 0, 34, 45, 0x682E2E, false),
        ("idle_east", 0, 35, 46, 0xB36844, true),
        ("idle_east", 0, 40, 41, 0xEABF67, false),
        ("idle_east", 0, 40, 43, 0xD68D47, false),
        ("idle_east", 0, 40, 44, 0xD68D47, false),
        ("idle_east", 0, 38, 46, 0x682E2E, false),
        ("idle_east", 0, 38, 49, 0x524642, false),
        ("idle_east", 0, 38, 52, 0x271F1D, false),
        ("idle_east", 0, 39, 52, 0x3C3431, false),
        ("idle_east", 0, 39, 53, 0x70645F, false),
        ("idle_east", 0, 40, 30, 0x63413B, false),
        ("idle_north", 0, 39, 40, 0xA27F4E, false),
        ("idle_north", 0, 34, 45, 0x93524E, false),
        ("idle_north", 0, 45, 45, 0x93524E, false),
        ("idle_north", 0, 33, 47, 0x571D1F, true),
        ("idle_north", 0, 46, 47, 0x571D1F, true),
        ("idle_north", 0, 39, 41, 0xFFF5D8, false),
        ("idle_north", 0, 39, 42, 0x93524E, false),
        ("idle_north", 0, 39, 43, 0x93524E, false),
        ("idle_north", 0, 39, 46, 0x682E2E, false),
        ("idle_north", 0, 37, 52, 0x271F1D, false),
        ("idle_south", 0, 38, 33, 0x712922, true),
        ("idle_south", 0, 39, 35, 0xB36844, true),
        ("idle_south", 0, 36, 38, 0x571D1F, true),
        ("idle_south", 0, 39, 42, 0xD68D47, false),
        ("idle_south", 0, 34, 45, 0x93524E, false),
        ("idle_south", 0, 45, 45, 0x93524E, false),
        ("idle_south", 0, 39, 41, 0xEABF67, false),
        ("idle_south", 0, 36, 44, 0x3A1A1A, false),
        ("idle_south", 0, 39, 43, 0xD68D47, false),
        ("idle_south", 0, 39, 44, 0xD68D47, false),
        ("idle_south", 0, 37, 49, 0x524642, false),
        ("idle_south", 0, 38, 50, 0xFFF5D8, false),
        ("idle_south", 0, 37, 51, 0x271F1D, false),
        ("idle_south", 0, 37, 52, 0x271F1D, false),
        ("idle_south", 0, 37, 53, 0x70645F, false),
        ("walk_east", 1, 40, 36, 0xB36844, true),
        ("walk_east", 1, 40, 43, 0xD68D47, false),
        ("walk_east", 1, 32, 46, 0xB36844, true),
        ("walk_east", 1, 47, 46, 0xB36844, true),
        ("walk_east", 1, 36, 45, 0x682E2E, false),
        ("walk_east", 1, 42, 51, 0x271F1D, false),
        ("walk_east", 3, 36, 47, 0xB36844, true),
        ("walk_east", 3, 44, 46, 0x8E4538, true),
        ("walk_east", 3, 36, 52, 0x271F1D, false),
        ("walk_east", 3, 37, 52, 0x3C3431, false),
        ("walk_north", 1, 39, 41, 0xA27F4E, false),
        ("walk_north", 1, 34, 47, 0xB36844, true),
        ("walk_north", 1, 46, 47, 0x571D1F, true),
        ("walk_north", 1, 37, 50, 0xA27F4E, false),
        ("walk_north", 1, 37, 51, 0x271F1D, false),
        ("walk_north", 1, 38, 51, 0x271F1D, false),
        ("walk_north", 1, 37, 52, 0x271F1D, false),
        ("walk_north", 3, 39, 41, 0xA27F4E, false),
        ("walk_north", 3, 33, 47, 0x571D1F, true),
        ("walk_north", 3, 45, 47, 0xB36844, true),
        ("walk_north", 3, 41, 51, 0x271F1D, false),
        ("walk_north", 3, 42, 51, 0x271F1D, false),
        ("walk_south", 1, 39, 43, 0xD68D47, false),
        ("walk_south", 1, 34, 47, 0xB36844, true),
        ("walk_south", 1, 46, 47, 0x571D1F, true),
        ("walk_south", 1, 37, 51, 0x271F1D, false),
        ("walk_south", 1, 38, 51, 0x271F1D, false),
        ("walk_south", 3, 39, 43, 0xD68D47, false),
        ("walk_south", 3, 33, 47, 0x571D1F, true),
        ("walk_south", 3, 45, 47, 0xB36844, true),
        ("walk_south", 3, 41, 51, 0x271F1D, false),
        ("walk_south", 3, 42, 51, 0x271F1D, false),
        ("idle_east", 0, 39, 42, 0x9E512F, false),
        ("idle_east", 0, 42, 42, 0x9E512F, false),
        ("idle_east", 0, 38, 41, 0xFFF5D8, false),
        ("idle_east", 0, 39, 43, 0xEABF67, false),
        ("idle_east", 0, 38, 45, 0x3A1A1A, false),
        ("idle_east", 0, 35, 45, 0x93524E, false),
        ("idle_east", 0, 36, 46, 0xB36844, true),
        ("idle_east", 0, 45, 46, 0x8E4538, true),
        ("idle_south", 0, 38, 42, 0x9E512F, false),
        ("idle_south", 0, 41, 42, 0x9E512F, false),
        ("idle_south", 0, 37, 41, 0xFFF5D8, false),
        ("idle_south", 0, 33, 45, 0x682E2E, false),
        ("idle_south", 0, 32, 46, 0xB36844, true),
        ("idle_south", 0, 33, 47, 0x571D1F, true),
        ("idle_south", 0, 39, 45, 0x3C3431, false),
        ("idle_south", 0, 38, 46, 0x3C3431, false),
        ("idle_south", 0, 37, 50, 0xD0B992, false),
        ("idle_north", 0, 38, 41, 0xFFF5D8, false),
        ("idle_north", 0, 36, 44, 0x3A1A1A, false),
        ("idle_north", 0, 35, 45, 0x3A1A1A, false),
        ("idle_north", 0, 32, 46, 0xB36844, true),
        ("idle_north", 0, 45, 46, 0xB36844, true),
        ("idle_north", 0, 38, 50, 0xFFF5D8, false),
        ("walk_east", 0, 39, 42, 0x9E512F, false),
        ("walk_east", 0, 42, 42, 0x9E512F, false),
        ("walk_east", 1, 39, 43, 0x9E512F, false),
        ("walk_east", 1, 42, 43, 0x9E512F, false),
        ("walk_east", 2, 39, 42, 0x9E512F, false),
        ("walk_east", 2, 42, 42, 0x9E512F, false),
        ("walk_east", 3, 39, 43, 0x9E512F, false),
        ("walk_east", 3, 42, 43, 0x9E512F, false),
        ("walk_south", 0, 38, 42, 0x9E512F, false),
        ("walk_south", 0, 41, 42, 0x9E512F, false),
        ("walk_south", 1, 38, 43, 0x9E512F, false),
        ("walk_south", 1, 41, 43, 0x9E512F, false),
        ("walk_south", 2, 38, 42, 0x9E512F, false),
        ("walk_south", 2, 41, 42, 0x9E512F, false),
        ("walk_south", 3, 38, 43, 0x9E512F, false),
        ("walk_south", 3, 41, 43, 0x9E512F, false),
    ];
    let cases: [(&str, &[usize]); 6] = [
        ("idle_east", &[40]),
        ("idle_north", &[12]),
        ("idle_south", &[46]),
        ("walk_east", &[40, 43, 40, 39]),
        ("walk_north", &[12, 10, 12, 10]),
        ("walk_south", &[46, 44, 46, 44]),
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
            let prefix = "spr_npc_reina_winter";
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
                        "hair, jacket, scarf, trousers, boots or another non-skin color changed",
                    );
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
