use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/reina-beach-finish-study and the local accepted Reina world baseline"]
fn reina_beach_pilot_covers_skin_and_preserves_outfit_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/reina-beach-finish-study");
    let baseline = root.join("generated/characters-march-winter-injured-trial/characters/reina");
    let set = std::env::var_os("FOM_REINA_BEACH_PILOT_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/reina-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/reina-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..246];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..246], prior);
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
    // Fresh literal landmarks distinguish exposed torso, arms, hands and legs
    // from green swimsuit fabric, coral sandal straps, eyes and hair.
    let landmarks = [
        ("idle_east", 0, 40, 30, 0x63413B, false),
        ("idle_east", 0, 39, 36, 0x000000, false),
        ("idle_east", 0, 39, 40, 0x000000, false),
        ("idle_east", 0, 39, 41, 0xB36844, true),
        ("idle_east", 0, 39, 42, 0x47C744, false),
        ("idle_east", 0, 38, 42, 0x30792E, false),
        ("idle_east", 0, 38, 43, 0x30792E, false),
        ("idle_east", 0, 39, 43, 0x9EF671, false),
        ("idle_east", 0, 39, 45, 0xB36844, true),
        ("idle_east", 0, 39, 46, 0x9EF671, false),
        ("idle_east", 0, 38, 50, 0x712922, true),
        ("idle_east", 0, 38, 52, 0xBE5B52, false),
        ("idle_east", 0, 38, 53, 0x712922, true),
        ("idle_east", 0, 34, 44, 0x000000, false),
        ("idle_east", 0, 34, 46, 0xB36844, true),
        ("idle_north", 0, 40, 30, 0x63413B, false),
        ("idle_north", 0, 39, 36, 0x452929, false),
        ("idle_north", 0, 39, 40, 0x712922, true),
        ("idle_north", 0, 39, 41, 0x47C744, false),
        ("idle_north", 0, 39, 42, 0xB36844, true),
        ("idle_north", 0, 38, 42, 0xB36844, true),
        ("idle_north", 0, 38, 43, 0x9EF671, false),
        ("idle_north", 0, 39, 43, 0x9EF671, false),
        ("idle_north", 0, 39, 45, 0xB36844, true),
        ("idle_north", 0, 39, 46, 0x9EF671, false),
        ("idle_north", 0, 38, 50, 0xB36844, true),
        ("idle_north", 0, 38, 52, 0xF5897F, false),
        ("idle_north", 0, 38, 53, 0xB36844, true),
        ("idle_north", 0, 34, 44, 0x712922, true),
        ("idle_north", 0, 34, 46, 0xB36844, true),
        ("idle_south", 0, 40, 30, 0x63413B, false),
        ("idle_south", 0, 39, 36, 0xB36844, true),
        ("idle_south", 0, 39, 40, 0x712922, true),
        ("idle_south", 0, 39, 41, 0x8E4538, true),
        ("idle_south", 0, 39, 42, 0xB36844, true),
        ("idle_south", 0, 38, 42, 0x47C744, false),
        ("idle_south", 0, 38, 43, 0x9EF671, false),
        ("idle_south", 0, 39, 43, 0x47C744, false),
        ("idle_south", 0, 39, 45, 0xB36844, true),
        ("idle_south", 0, 39, 46, 0x9EF671, false),
        ("idle_south", 0, 38, 50, 0xB36844, true),
        ("idle_south", 0, 38, 52, 0xF5897F, false),
        ("idle_south", 0, 38, 53, 0xB36844, true),
        ("idle_south", 0, 34, 44, 0x712922, true),
        ("idle_south", 0, 34, 46, 0xB36844, true),
        ("walk_east", 0, 40, 30, 0x63413B, false),
        ("walk_east", 0, 39, 36, 0x000000, false),
        ("walk_east", 0, 39, 40, 0x000000, false),
        ("walk_east", 0, 39, 41, 0xB36844, true),
        ("walk_east", 0, 39, 42, 0x47C744, false),
        ("walk_east", 0, 38, 42, 0x30792E, false),
        ("walk_east", 0, 38, 43, 0x30792E, false),
        ("walk_east", 0, 39, 43, 0x9EF671, false),
        ("walk_east", 0, 39, 45, 0xB36844, true),
        ("walk_east", 0, 39, 46, 0x9EF671, false),
        ("walk_east", 0, 38, 50, 0x712922, true),
        ("walk_east", 0, 38, 52, 0xBE5B52, false),
        ("walk_east", 0, 38, 53, 0x712922, true),
        ("walk_east", 0, 34, 44, 0x000000, false),
        ("walk_east", 0, 34, 46, 0xB36844, true),
        ("walk_east", 1, 40, 30, 0x63413B, false),
        ("walk_east", 1, 39, 36, 0xC2B9BE, false),
        ("walk_east", 1, 39, 40, 0x712922, true),
        ("walk_east", 1, 39, 41, 0x000000, false),
        ("walk_east", 1, 39, 42, 0xB36844, true),
        ("walk_east", 1, 38, 42, 0x30792E, false),
        ("walk_east", 1, 38, 43, 0x30792E, false),
        ("walk_east", 1, 39, 43, 0x47C744, false),
        ("walk_east", 1, 39, 45, 0x30792E, false),
        ("walk_east", 1, 39, 46, 0xB36844, true),
        ("walk_east", 1, 38, 50, 0x000000, false),
        ("walk_east", 1, 38, 52, 0x000000, false),
        ("walk_east", 1, 34, 44, 0x000000, false),
        ("walk_east", 1, 34, 46, 0xB36844, true),
        ("walk_east", 2, 40, 30, 0x63413B, false),
        ("walk_east", 2, 39, 36, 0x000000, false),
        ("walk_east", 2, 39, 40, 0x000000, false),
        ("walk_east", 2, 39, 41, 0xB36844, true),
        ("walk_east", 2, 39, 42, 0x47C744, false),
        ("walk_east", 2, 38, 42, 0x30792E, false),
        ("walk_east", 2, 38, 43, 0x30792E, false),
        ("walk_east", 2, 39, 43, 0x9EF671, false),
        ("walk_east", 2, 39, 45, 0xB36844, true),
        ("walk_east", 2, 39, 46, 0x9EF671, false),
        ("walk_east", 2, 38, 50, 0x712922, true),
        ("walk_east", 2, 38, 52, 0xBE5B52, false),
        ("walk_east", 2, 38, 53, 0x712922, true),
        ("walk_east", 2, 34, 44, 0x000000, false),
        ("walk_east", 2, 34, 46, 0xB36844, true),
        ("walk_east", 3, 40, 30, 0x63413B, false),
        ("walk_east", 3, 39, 36, 0xC2B9BE, false),
        ("walk_east", 3, 39, 40, 0x712922, true),
        ("walk_east", 3, 39, 41, 0x000000, false),
        ("walk_east", 3, 39, 42, 0xB36844, true),
        ("walk_east", 3, 38, 42, 0x30792E, false),
        ("walk_east", 3, 38, 43, 0x30792E, false),
        ("walk_east", 3, 39, 43, 0x47C744, false),
        ("walk_east", 3, 39, 45, 0x30792E, false),
        ("walk_east", 3, 39, 46, 0xB36844, true),
        ("walk_east", 3, 38, 50, 0xB36844, true),
        ("walk_east", 3, 38, 52, 0xBE5B52, false),
        ("walk_east", 3, 38, 53, 0xB36844, true),
        ("walk_east", 3, 34, 46, 0x000000, false),
        ("walk_north", 0, 40, 30, 0x63413B, false),
        ("walk_north", 0, 39, 36, 0x452929, false),
        ("walk_north", 0, 39, 40, 0x712922, true),
        ("walk_north", 0, 39, 41, 0x47C744, false),
        ("walk_north", 0, 39, 42, 0xB36844, true),
        ("walk_north", 0, 38, 42, 0xB36844, true),
        ("walk_north", 0, 38, 43, 0x9EF671, false),
        ("walk_north", 0, 39, 43, 0x9EF671, false),
        ("walk_north", 0, 39, 45, 0xB36844, true),
        ("walk_north", 0, 39, 46, 0x9EF671, false),
        ("walk_north", 0, 38, 50, 0xB36844, true),
        ("walk_north", 0, 38, 52, 0xF5897F, false),
        ("walk_north", 0, 38, 53, 0xB36844, true),
        ("walk_north", 0, 34, 44, 0x712922, true),
        ("walk_north", 0, 34, 46, 0xB36844, true),
        ("walk_north", 1, 40, 30, 0x63413B, false),
        ("walk_north", 1, 39, 36, 0x63413B, false),
        ("walk_north", 1, 39, 40, 0x000000, false),
        ("walk_north", 1, 39, 41, 0x712922, true),
        ("walk_north", 1, 39, 42, 0x47C744, false),
        ("walk_north", 1, 38, 42, 0x30792E, false),
        ("walk_north", 1, 38, 43, 0xB36844, true),
        ("walk_north", 1, 39, 43, 0xB36844, true),
        ("walk_north", 1, 39, 45, 0x9EF671, false),
        ("walk_north", 1, 39, 46, 0xB36844, true),
        ("walk_north", 1, 38, 50, 0x8E4538, true),
        ("walk_north", 1, 38, 52, 0x8E4538, true),
        ("walk_north", 1, 38, 53, 0x000000, false),
        ("walk_north", 1, 34, 44, 0x000000, false),
        ("walk_north", 1, 34, 46, 0xB36844, true),
        ("walk_north", 2, 40, 30, 0x63413B, false),
        ("walk_north", 2, 39, 36, 0x452929, false),
        ("walk_north", 2, 39, 40, 0x712922, true),
        ("walk_north", 2, 39, 41, 0x47C744, false),
        ("walk_north", 2, 39, 42, 0xB36844, true),
        ("walk_north", 2, 38, 42, 0xB36844, true),
        ("walk_north", 2, 38, 43, 0x9EF671, false),
        ("walk_north", 2, 39, 43, 0x9EF671, false),
        ("walk_north", 2, 39, 45, 0xB36844, true),
        ("walk_north", 2, 39, 46, 0x9EF671, false),
        ("walk_north", 2, 38, 50, 0xB36844, true),
        ("walk_north", 2, 38, 52, 0xF5897F, false),
        ("walk_north", 2, 38, 53, 0xB36844, true),
        ("walk_north", 2, 34, 44, 0x712922, true),
        ("walk_north", 2, 34, 46, 0xB36844, true),
        ("walk_north", 3, 40, 30, 0x63413B, false),
        ("walk_north", 3, 39, 36, 0x63413B, false),
        ("walk_north", 3, 39, 40, 0x000000, false),
        ("walk_north", 3, 39, 41, 0x712922, true),
        ("walk_north", 3, 39, 42, 0x47C744, false),
        ("walk_north", 3, 38, 42, 0x30792E, false),
        ("walk_north", 3, 38, 43, 0xB36844, true),
        ("walk_north", 3, 39, 43, 0xB36844, true),
        ("walk_north", 3, 39, 45, 0x9EF671, false),
        ("walk_north", 3, 39, 46, 0xB36844, true),
        ("walk_north", 3, 38, 50, 0xB36844, true),
        ("walk_north", 3, 38, 52, 0xB36844, true),
        ("walk_north", 3, 38, 53, 0xF5897F, false),
        ("walk_north", 3, 34, 44, 0x000000, false),
        ("walk_north", 3, 34, 46, 0x712922, true),
        ("walk_south", 0, 40, 30, 0x63413B, false),
        ("walk_south", 0, 39, 36, 0xB36844, true),
        ("walk_south", 0, 39, 40, 0x712922, true),
        ("walk_south", 0, 39, 41, 0x8E4538, true),
        ("walk_south", 0, 39, 42, 0xB36844, true),
        ("walk_south", 0, 38, 42, 0x47C744, false),
        ("walk_south", 0, 38, 43, 0x9EF671, false),
        ("walk_south", 0, 39, 43, 0x47C744, false),
        ("walk_south", 0, 39, 45, 0xB36844, true),
        ("walk_south", 0, 39, 46, 0x9EF671, false),
        ("walk_south", 0, 38, 50, 0xB36844, true),
        ("walk_south", 0, 38, 52, 0xF5897F, false),
        ("walk_south", 0, 38, 53, 0xB36844, true),
        ("walk_south", 0, 34, 44, 0x712922, true),
        ("walk_south", 0, 34, 46, 0xB36844, true),
        ("walk_south", 1, 40, 30, 0x63413B, false),
        ("walk_south", 1, 39, 36, 0xB36844, true),
        ("walk_south", 1, 39, 40, 0x8E4538, true),
        ("walk_south", 1, 39, 41, 0x712922, true),
        ("walk_south", 1, 39, 42, 0x8E4538, true),
        ("walk_south", 1, 38, 42, 0xB36844, true),
        ("walk_south", 1, 38, 43, 0x47C744, false),
        ("walk_south", 1, 39, 43, 0xB36844, true),
        ("walk_south", 1, 39, 45, 0x30792E, false),
        ("walk_south", 1, 39, 46, 0xB36844, true),
        ("walk_south", 1, 38, 50, 0x8E4538, true),
        ("walk_south", 1, 38, 52, 0x8E4538, true),
        ("walk_south", 1, 38, 53, 0x000000, false),
        ("walk_south", 1, 34, 44, 0x000000, false),
        ("walk_south", 1, 34, 46, 0xB36844, true),
        ("walk_south", 2, 40, 30, 0x63413B, false),
        ("walk_south", 2, 39, 36, 0xB36844, true),
        ("walk_south", 2, 39, 40, 0x712922, true),
        ("walk_south", 2, 39, 41, 0x8E4538, true),
        ("walk_south", 2, 39, 42, 0xB36844, true),
        ("walk_south", 2, 38, 42, 0x47C744, false),
        ("walk_south", 2, 38, 43, 0x9EF671, false),
        ("walk_south", 2, 39, 43, 0x47C744, false),
        ("walk_south", 2, 39, 45, 0xB36844, true),
        ("walk_south", 2, 39, 46, 0x9EF671, false),
        ("walk_south", 2, 38, 50, 0xB36844, true),
        ("walk_south", 2, 38, 52, 0xF5897F, false),
        ("walk_south", 2, 38, 53, 0xB36844, true),
        ("walk_south", 2, 34, 44, 0x712922, true),
        ("walk_south", 2, 34, 46, 0xB36844, true),
        ("walk_south", 3, 40, 30, 0x63413B, false),
        ("walk_south", 3, 39, 36, 0xB36844, true),
        ("walk_south", 3, 39, 40, 0x8E4538, true),
        ("walk_south", 3, 39, 41, 0x712922, true),
        ("walk_south", 3, 39, 42, 0x8E4538, true),
        ("walk_south", 3, 38, 42, 0xB36844, true),
        ("walk_south", 3, 38, 43, 0x47C744, false),
        ("walk_south", 3, 39, 43, 0xB36844, true),
        ("walk_south", 3, 39, 45, 0x30792E, false),
        ("walk_south", 3, 39, 46, 0xB36844, true),
        ("walk_south", 3, 38, 50, 0xB36844, true),
        ("walk_south", 3, 38, 52, 0xB36844, true),
        ("walk_south", 3, 38, 53, 0xF5897F, false),
        ("walk_south", 3, 34, 44, 0x000000, false),
        ("walk_south", 3, 34, 46, 0x712922, true),
        ("idle_east", 0, 39, 52, 0xF5897F, false),
        ("idle_east", 0, 42, 52, 0xF5897F, false),
        ("idle_north", 0, 38, 49, 0xB36844, true),
        ("idle_south", 0, 38, 44, 0x30792E, false),
        ("idle_south", 0, 36, 44, 0x571D1F, true),
        ("idle_south", 0, 37, 45, 0x712922, true),
        ("idle_south", 0, 37, 46, 0x47C744, false),
        ("idle_south", 0, 38, 47, 0x30792E, false),
    ];
    let cases: [(&str, &[usize]); 6] = [
        ("idle_east", &[85]),
        ("idle_north", &[65]),
        ("idle_south", &[100]),
        ("walk_east", &[85, 91, 85, 77]),
        ("walk_north", &[65, 56, 65, 56]),
        ("walk_south", &[100, 91, 100, 91]),
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
            let prefix = "spr_npc_reina_beach";
            let asset = format!("assets/animations/NPCs/Reina/Sprites/Beach/{prefix}_{case}.png");
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
                // All four reviewed world shades are skin in these Beach strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Reina Beach material mismatch: {id} {case} [{x},{y}]"
                );
                mask.push(p != q);
                if p != q {
                    let index = source.iter().position(|c| rgba(*c) == p.0).expect(
                        "hair, green swimsuit, coral sandal straps, eyes or another non-skin color changed",
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
