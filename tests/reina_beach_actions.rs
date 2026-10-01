use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/reina and the local accepted Reina world baseline"]
fn reina_beach_actions_covers_skin_and_preserves_outfit_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/reina");
    let baseline =
        root.join("generated/characters-reina-juniper-march-beach-pilot-trial/characters/reina");
    let set = std::env::var_os("FOM_REINA_BEACH_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/reina-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/reina-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..252];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..252], prior);
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
    // Fresh literal landmarks distinguish moving hands, bare limbs, closed
    // eyelids and kiss-mouth shading from swimsuit fabric, sandals and hair.
    let landmarks = [
        ("action_east", 0, 40, 30, 0x63413B, false),
        ("action_east", 0, 39, 39, 0x712922, true),
        ("action_east", 0, 38, 50, 0x000000, false),
        ("action_east", 1, 40, 30, 0x63413B, false),
        ("action_east", 1, 39, 39, 0x2B1919, false),
        ("action_east", 1, 38, 50, 0x8E4538, true),
        ("action_east", 2, 40, 30, 0x63413B, false),
        ("action_east", 2, 39, 39, 0x2B1919, false),
        ("action_east", 2, 38, 50, 0x8E4538, true),
        ("action_east", 3, 40, 30, 0x63413B, false),
        ("action_east", 3, 39, 39, 0x2B1919, false),
        ("action_east", 3, 38, 50, 0x8E4538, true),
        ("action_east", 4, 40, 30, 0x63413B, false),
        ("action_east", 4, 39, 39, 0x2B1919, false),
        ("action_east", 4, 38, 50, 0x8E4538, true),
        ("action_east", 5, 40, 30, 0x63413B, false),
        ("action_east", 5, 39, 39, 0x712922, true),
        ("action_east", 5, 38, 50, 0x000000, false),
        ("action_east", 6, 40, 30, 0x63413B, false),
        ("action_east", 6, 39, 39, 0x712922, true),
        ("action_east", 6, 38, 50, 0x712922, true),
        ("action_north", 0, 40, 30, 0x63413B, false),
        ("action_north", 0, 39, 39, 0x000000, false),
        ("action_north", 0, 38, 50, 0xB36844, true),
        ("action_north", 1, 40, 30, 0x63413B, false),
        ("action_north", 1, 39, 39, 0x712922, true),
        ("action_north", 1, 38, 50, 0xB36844, true),
        ("action_north", 2, 40, 30, 0x63413B, false),
        ("action_north", 2, 39, 39, 0x712922, true),
        ("action_north", 2, 38, 50, 0xB36844, true),
        ("action_north", 3, 40, 30, 0x63413B, false),
        ("action_north", 3, 39, 39, 0x712922, true),
        ("action_north", 3, 38, 50, 0xB36844, true),
        ("action_north", 4, 40, 30, 0x63413B, false),
        ("action_north", 4, 39, 39, 0x712922, true),
        ("action_north", 4, 38, 50, 0xB36844, true),
        ("action_north", 5, 40, 30, 0x63413B, false),
        ("action_north", 5, 39, 39, 0x000000, false),
        ("action_north", 5, 38, 50, 0xB36844, true),
        ("action_north", 6, 40, 30, 0x63413B, false),
        ("action_north", 6, 39, 39, 0x000000, false),
        ("action_north", 6, 38, 50, 0xB36844, true),
        ("action_south", 0, 40, 30, 0x63413B, false),
        ("action_south", 0, 39, 39, 0xB36844, true),
        ("action_south", 0, 38, 50, 0xB36844, true),
        ("action_south", 1, 40, 30, 0x63413B, false),
        ("action_south", 1, 39, 39, 0x8E4538, true),
        ("action_south", 1, 38, 50, 0xB36844, true),
        ("action_south", 2, 40, 30, 0x63413B, false),
        ("action_south", 2, 39, 39, 0x8E4538, true),
        ("action_south", 2, 38, 50, 0xB36844, true),
        ("action_south", 3, 40, 30, 0x63413B, false),
        ("action_south", 3, 39, 39, 0x8E4538, true),
        ("action_south", 3, 38, 50, 0xB36844, true),
        ("action_south", 4, 40, 30, 0x63413B, false),
        ("action_south", 4, 39, 39, 0x8E4538, true),
        ("action_south", 4, 38, 50, 0xB36844, true),
        ("action_south", 5, 40, 30, 0x63413B, false),
        ("action_south", 5, 39, 39, 0xB36844, true),
        ("action_south", 5, 38, 50, 0xB36844, true),
        ("action_south", 6, 40, 30, 0x63413B, false),
        ("action_south", 6, 39, 39, 0x8E4538, true),
        ("action_south", 6, 38, 50, 0xB36844, true),
        ("blink_east", 0, 40, 30, 0x63413B, false),
        ("blink_east", 0, 39, 39, 0x712922, true),
        ("blink_east", 0, 38, 50, 0x712922, true),
        ("blink_east", 1, 40, 30, 0x63413B, false),
        ("blink_east", 1, 39, 39, 0x712922, true),
        ("blink_east", 1, 38, 50, 0x712922, true),
        ("blink_east", 2, 40, 30, 0x63413B, false),
        ("blink_east", 2, 39, 39, 0x712922, true),
        ("blink_east", 2, 38, 50, 0x712922, true),
        ("blink_south", 0, 40, 30, 0x63413B, false),
        ("blink_south", 0, 39, 39, 0x8E4538, true),
        ("blink_south", 0, 38, 50, 0xB36844, true),
        ("blink_south", 1, 40, 30, 0x63413B, false),
        ("blink_south", 1, 39, 39, 0x8E4538, true),
        ("blink_south", 1, 38, 50, 0xB36844, true),
        ("blink_south", 2, 40, 30, 0x63413B, false),
        ("blink_south", 2, 39, 39, 0x8E4538, true),
        ("blink_south", 2, 38, 50, 0xB36844, true),
        ("kiss_east", 0, 40, 30, 0x452929, false),
        ("kiss_east", 0, 39, 39, 0xB36844, true),
        ("kiss_east", 0, 38, 50, 0xB36844, true),
        ("kiss_east", 1, 40, 30, 0x63413B, false),
        ("kiss_east", 1, 39, 39, 0x712922, true),
        ("kiss_east", 1, 38, 50, 0x712922, true),
        ("kiss_east", 2, 40, 30, 0x63413B, false),
        ("kiss_east", 2, 39, 39, 0x2B1919, false),
        ("kiss_east", 2, 38, 50, 0xB36844, true),
        ("kiss_east", 3, 40, 30, 0x63413B, false),
        ("kiss_east", 3, 39, 39, 0x712922, true),
        ("kiss_east", 3, 38, 50, 0x712922, true),
        ("action_east", 1, 47, 43, 0xB36844, true),
        ("action_east", 1, 48, 43, 0xB36844, true),
        ("action_east", 1, 49, 43, 0x8E4538, true),
        ("action_east", 1, 48, 44, 0xB36844, true),
        ("action_east", 1, 48, 45, 0x571D1F, true),
        ("action_east", 1, 43, 42, 0xB36844, true),
        ("action_east", 1, 39, 43, 0x000000, false),
        ("action_east", 1, 41, 46, 0x47C744, false),
        ("action_east", 1, 42, 52, 0xBE5B52, false),
        ("action_east", 1, 43, 52, 0xF5897F, false),
        ("action_north", 0, 34, 44, 0xB36844, true),
        ("action_north", 0, 38, 43, 0x9EF671, false),
        ("action_north", 0, 39, 41, 0x47C744, false),
        ("action_north", 0, 38, 52, 0xF5897F, false),
        ("action_north", 0, 39, 40, 0x712922, true),
        ("action_south", 2, 38, 43, 0x9EF671, false),
        ("action_south", 2, 40, 44, 0x30792E, false),
        ("action_south", 2, 41, 44, 0x30792E, false),
        ("action_south", 2, 42, 46, 0x47C744, false),
        ("action_south", 2, 38, 52, 0xF5897F, false),
        ("blink_east", 1, 38, 36, 0xB36844, true),
        ("blink_east", 1, 39, 37, 0x000000, false),
        ("blink_east", 1, 40, 37, 0xB36844, true),
        ("blink_east", 1, 39, 41, 0xB36844, true),
        ("blink_east", 1, 39, 43, 0x9EF671, false),
        ("blink_east", 1, 38, 52, 0xBE5B52, false),
        ("blink_south", 1, 37, 36, 0xB36844, true),
        ("blink_south", 1, 39, 37, 0xB36844, true),
        ("blink_south", 1, 40, 37, 0xB36844, true),
        ("blink_south", 1, 38, 43, 0x9EF671, false),
        ("blink_south", 1, 39, 46, 0x9EF671, false),
        ("blink_south", 1, 38, 52, 0xF5897F, false),
        ("kiss_east", 2, 37, 37, 0x452929, false),
        ("kiss_east", 2, 38, 37, 0x000000, false),
        ("kiss_east", 2, 39, 37, 0xB36844, true),
        ("kiss_east", 2, 40, 37, 0x712922, true),
        ("kiss_east", 2, 41, 37, 0x8E4538, true),
        ("kiss_east", 2, 42, 37, 0xB36844, true),
        ("kiss_east", 2, 41, 39, 0x571D1F, true),
        ("kiss_east", 2, 39, 42, 0x712922, true),
        ("kiss_east", 2, 38, 43, 0x712922, true),
        ("kiss_east", 2, 38, 52, 0x000000, false),
        ("kiss_east", 3, 38, 37, 0x712922, true),
        ("kiss_east", 3, 39, 37, 0x000000, false),
        ("kiss_east", 3, 40, 37, 0x000000, false),
        ("kiss_east", 3, 41, 37, 0xB36844, true),
        ("kiss_east", 3, 42, 37, 0xB36844, true),
        ("kiss_east", 3, 39, 39, 0x712922, true),
        ("kiss_east", 3, 39, 43, 0x30792E, false),
        ("kiss_east", 3, 38, 52, 0xF5897F, false),
    ];
    let cases: [(&str, &[usize]); 6] = [
        ("action_east", &[69, 79, 76, 79, 76, 69, 83]),
        ("action_north", &[61, 56, 57, 56, 57, 63, 65]),
        ("action_south", &[90, 90, 87, 90, 87, 90, 100]),
        ("blink_east", &[89, 99, 89]),
        ("blink_south", &[104, 114, 104]),
        ("kiss_east", &[77, 85, 91, 89]),
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
