use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/reina-wedding-finish-study and the local accepted Reina world baseline"]
fn reina_beach_swim_covers_face_and_preserves_water() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/reina-wedding-finish-study");
    let baseline =
        root.join("generated/characters-reina-juniper-march-beach-actions-trial/characters/reina");
    let set = std::env::var_os("FOM_REINA_BEACH_SWIM_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/reina-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/reina-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..258];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..258], prior);
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
    // Fresh literal landmarks distinguish the bobbing face and submerged jaw
    // boundary from hair, eyes, the waterline and disconnected splash droplets.
    let landmarks = [
        ("bath_swim_east", 0, 40, 50, 0xB36844, true),
        ("bath_swim_east", 0, 39, 53, 0x8E4538, true),
        ("bath_swim_east", 0, 38, 54, 0x571D1F, true),
        ("bath_swim_east", 0, 40, 44, 0x63413B, false),
        ("bath_swim_east", 0, 37, 51, 0x712922, true),
        ("bath_swim_east", 0, 40, 55, 0x9DEBFC, false),
        ("bath_swim_east", 0, 35, 56, 0x328BC9, false),
        ("bath_swim_east", 1, 40, 50, 0xB36844, true),
        ("bath_swim_east", 1, 39, 53, 0x8E4538, true),
        ("bath_swim_east", 1, 38, 54, 0x571D1F, true),
        ("bath_swim_east", 1, 40, 44, 0x63413B, false),
        ("bath_swim_east", 1, 37, 51, 0x712922, true),
        ("bath_swim_east", 1, 40, 55, 0x9DEBFC, false),
        ("bath_swim_east", 1, 35, 56, 0x328BC9, false),
        ("bath_swim_east", 2, 40, 51, 0xB36844, true),
        ("bath_swim_east", 2, 39, 54, 0x8E4538, true),
        ("bath_swim_east", 2, 38, 54, 0x712922, true),
        ("bath_swim_east", 2, 40, 45, 0x63413B, false),
        ("bath_swim_east", 2, 37, 52, 0x712922, true),
        ("bath_swim_east", 2, 40, 55, 0x9DEBFC, false),
        ("bath_swim_east", 2, 35, 56, 0x328BC9, false),
        ("bath_swim_east", 3, 40, 51, 0xB36844, true),
        ("bath_swim_east", 3, 39, 54, 0x8E4538, true),
        ("bath_swim_east", 3, 38, 54, 0x712922, true),
        ("bath_swim_east", 3, 40, 45, 0x63413B, false),
        ("bath_swim_east", 3, 37, 52, 0x712922, true),
        ("bath_swim_east", 3, 40, 55, 0x9DEBFC, false),
        ("bath_swim_east", 3, 35, 56, 0x328BC9, false),
        ("bath_swim_south", 0, 40, 50, 0xB36844, true),
        ("bath_swim_south", 0, 39, 53, 0xB36844, true),
        ("bath_swim_south", 0, 38, 54, 0x712922, true),
        ("bath_swim_south", 0, 40, 44, 0x63413B, false),
        ("bath_swim_south", 0, 37, 51, 0xECF0E9, false),
        ("bath_swim_south", 0, 40, 55, 0x9DEBFC, false),
        ("bath_swim_south", 0, 35, 56, 0x328BC9, false),
        ("bath_swim_south", 1, 40, 50, 0xB36844, true),
        ("bath_swim_south", 1, 39, 53, 0xB36844, true),
        ("bath_swim_south", 1, 38, 54, 0x712922, true),
        ("bath_swim_south", 1, 40, 44, 0x63413B, false),
        ("bath_swim_south", 1, 37, 51, 0xECF0E9, false),
        ("bath_swim_south", 1, 40, 55, 0x9DEBFC, false),
        ("bath_swim_south", 1, 35, 56, 0x328BC9, false),
        ("bath_swim_south", 2, 40, 51, 0xB36844, true),
        ("bath_swim_south", 2, 39, 54, 0xB36844, true),
        ("bath_swim_south", 2, 38, 54, 0x8E4538, true),
        ("bath_swim_south", 2, 40, 45, 0x63413B, false),
        ("bath_swim_south", 2, 37, 52, 0xECF0E9, false),
        ("bath_swim_south", 2, 40, 55, 0x9DEBFC, false),
        ("bath_swim_south", 2, 35, 56, 0x328BC9, false),
        ("bath_swim_south", 3, 40, 51, 0xB36844, true),
        ("bath_swim_south", 3, 39, 54, 0xB36844, true),
        ("bath_swim_south", 3, 38, 54, 0x8E4538, true),
        ("bath_swim_south", 3, 40, 45, 0x63413B, false),
        ("bath_swim_south", 3, 37, 52, 0xECF0E9, false),
        ("bath_swim_south", 3, 40, 55, 0x9DEBFC, false),
        ("bath_swim_south", 3, 35, 56, 0x328BC9, false),
        ("bath_swim_east", 0, 48, 59, 0x9DEBFC, false),
        ("bath_swim_east", 2, 47, 57, 0x9DEBFC, false),
        ("bath_swim_south", 0, 30, 59, 0x9DEBFC, false),
        ("bath_swim_south", 2, 35, 60, 0x9DEBFC, false),
        ("bath_swim_south", 0, 39, 54, 0x8E4538, true),
    ];
    let cases: [(&str, &[usize]); 2] = [
        ("bath_swim_east", &[30, 30, 25, 25]),
        ("bath_swim_south", &[34, 34, 28, 28]),
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
                        "hair, water, splash droplets, eyes or another non-skin color changed",
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
