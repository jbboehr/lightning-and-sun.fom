use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/reina-winter-reading-study and the local accepted Reina world baseline"]
fn reina_spring_standard_covers_skin_and_preserves_jacket_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/reina-winter-reading-study");
    let baseline =
        root.join("generated/characters-reina-juniper-march-actions-trial/characters/reina");
    let set = std::env::var_os("FOM_REINA_SPRING_STANDARD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/reina-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/reina-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..120];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..120], prior);
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
    let source = [0xB36844, 0x8E4538, 0x712922, 0x571D1F];
    // Independently inspected source landmarks distinguish exposed hands,
    // necks and midriff from same-colored jacket folds and hem pixels.
    let landmarks = [
        ("action_east", 0, 42, 36, 0xB36844, true),
        ("action_east", 0, 43, 38, 0x000000, false),
        ("action_east", 0, 42, 42, 0x8E4538, true),
        ("action_east", 0, 43, 42, 0xB36844, true),
        ("action_east", 0, 38, 47, 0xD2962F, false),
        ("action_east", 0, 40, 50, 0xB36844, true),
        ("action_east", 0, 40, 45, 0x000000, false),
        ("action_east", 0, 40, 31, 0x63413B, false),
        ("action_east", 1, 43, 35, 0xB36844, true),
        ("action_east", 1, 45, 41, 0xB36844, true),
        ("action_east", 1, 40, 41, 0xB36844, false),
        ("action_east", 1, 39, 45, 0xB36844, false),
        ("action_east", 1, 47, 43, 0xB36844, true),
        ("action_east", 1, 49, 43, 0x8E4538, true),
        ("action_east", 1, 48, 45, 0x571D1F, true),
        ("action_east", 1, 41, 45, 0xB36844, true),
        ("action_east", 1, 42, 45, 0xB36844, true),
        ("action_east", 1, 40, 49, 0xB36844, true),
        ("action_east", 1, 44, 43, 0xF7E29C, false),
        ("action_east", 1, 42, 44, 0xA59DA2, false),
        ("action_east", 2, 40, 41, 0xB36844, false),
        ("action_east", 2, 39, 45, 0xB36844, false),
        ("action_east", 2, 41, 45, 0xB36844, true),
        ("action_east", 2, 45, 44, 0xB36844, true),
        ("action_east", 2, 45, 45, 0xB36844, true),
        ("action_east", 2, 44, 46, 0x712922, true),
        ("action_east", 2, 45, 41, 0xB36844, true),
        ("action_east", 2, 44, 42, 0xF0B0A3, false),
        ("action_east", 6, 37, 43, 0xB36844, false),
        ("action_east", 6, 42, 41, 0xB36844, true),
        ("action_east", 6, 39, 45, 0xB36844, true),
        ("action_east", 6, 35, 46, 0xB36844, true),
        ("action_east", 6, 36, 47, 0xB36844, true),
        ("action_east", 6, 40, 42, 0xF0B0A3, false),
        ("action_north", 0, 45, 43, 0xB36844, true),
        ("action_north", 0, 46, 43, 0x8E4538, true),
        ("action_north", 0, 34, 44, 0xB36844, true),
        ("action_north", 0, 37, 49, 0x8E4538, true),
        ("action_north", 0, 42, 49, 0x8E4538, true),
        ("action_north", 0, 40, 43, 0xF7E29C, false),
        ("action_north", 0, 39, 37, 0x2B1919, false),
        ("action_north", 1, 34, 44, 0x712922, true),
        ("action_north", 1, 34, 46, 0xB36844, true),
        ("action_north", 1, 37, 49, 0x8E4538, true),
        ("action_north", 1, 42, 49, 0x8E4538, true),
        ("action_north", 1, 38, 42, 0xF7E29C, false),
        ("action_north", 2, 34, 44, 0x712922, true),
        ("action_north", 2, 34, 45, 0xB36844, true),
        ("action_north", 2, 37, 49, 0x8E4538, true),
        ("action_north", 2, 42, 49, 0x8E4538, true),
        ("action_north", 2, 40, 44, 0xEABF67, false),
        ("action_south", 0, 36, 44, 0xB36844, false),
        ("action_south", 0, 43, 44, 0xB36844, false),
        ("action_south", 0, 39, 35, 0x8E4538, true),
        ("action_south", 0, 38, 46, 0xB36844, true),
        ("action_south", 0, 32, 46, 0x000000, false),
        ("action_south", 0, 38, 50, 0xB36844, true),
        ("action_south", 0, 40, 43, 0xF0B0A3, false),
        ("action_south", 1, 37, 43, 0xB36844, false),
        ("action_south", 1, 43, 43, 0xB36844, false),
        ("action_south", 1, 35, 45, 0xB36844, true),
        ("action_south", 1, 35, 47, 0x571D1F, true),
        ("action_south", 1, 40, 45, 0xB36844, true),
        ("action_south", 1, 38, 49, 0xB36844, true),
        ("action_south", 1, 42, 42, 0xEABF67, false),
        ("action_south", 2, 43, 43, 0xB36844, false),
        ("action_south", 2, 37, 43, 0xEABF67, false),
        ("action_south", 2, 37, 44, 0xD2962F, false),
        ("action_south", 2, 39, 45, 0x000000, false),
        ("action_south", 2, 41, 45, 0xB36844, true),
        ("action_south", 2, 38, 49, 0xB36844, true),
        ("kiss_east", 0, 37, 44, 0xB36844, false),
        ("kiss_east", 0, 41, 42, 0xB36844, true),
        ("kiss_east", 0, 35, 47, 0xB36844, true),
        ("kiss_east", 0, 38, 50, 0xB36844, true),
        ("kiss_east", 0, 39, 43, 0xF0B0A3, false),
        ("kiss_east", 1, 38, 44, 0xB36844, false),
        ("kiss_east", 1, 43, 42, 0xB36844, true),
        ("kiss_east", 1, 36, 46, 0xB36844, true),
        ("kiss_east", 1, 41, 46, 0xB36844, true),
        ("kiss_east", 1, 41, 39, 0xB36844, true),
        ("kiss_east", 2, 40, 43, 0xB36844, false),
        ("kiss_east", 2, 47, 44, 0xB36844, false),
        ("kiss_east", 2, 46, 45, 0xB36844, false),
        ("kiss_east", 2, 45, 41, 0xB36844, true),
        ("kiss_east", 2, 36, 44, 0xB36844, true),
        ("kiss_east", 2, 37, 46, 0xB36844, true),
        ("kiss_east", 2, 42, 45, 0xB36844, true),
        ("kiss_east", 2, 43, 45, 0xB36844, true),
        ("kiss_east", 2, 43, 36, 0xB36844, true),
        ("kiss_east", 2, 41, 37, 0x8E4538, true),
        ("kiss_east", 2, 43, 44, 0xD8D8DC, false),
        ("kiss_east", 2, 39, 31, 0x63413B, false),
        ("kiss_east", 3, 38, 44, 0xB36844, false),
        ("kiss_east", 3, 43, 42, 0xB36844, true),
        ("kiss_east", 3, 36, 46, 0xB36844, true),
        ("kiss_east", 3, 41, 46, 0xB36844, true),
        ("kiss_east", 3, 41, 39, 0xB36844, true),
        ("sleep_east", 0, 38, 43, 0xB36844, false),
        ("sleep_east", 0, 40, 35, 0xB36844, true),
        ("sleep_east", 0, 38, 37, 0x000000, false),
        ("sleep_east", 0, 43, 39, 0x8E4538, true),
        ("sleep_east", 0, 44, 40, 0xB36844, true),
        ("sleep_east", 0, 43, 41, 0xB36844, true),
        ("sleep_east", 0, 39, 45, 0xB36844, true),
        ("sleep_east", 0, 41, 45, 0xB36844, true),
        ("sleep_east", 0, 39, 49, 0xB36844, true),
        ("sleep_east", 0, 42, 41, 0xD2962F, false),
        ("sleep_east", 0, 40, 44, 0xD8D8DC, false),
        ("sleep_east", 0, 40, 47, 0xA13761, false),
    ];
    let cases: [(&str, &[usize]); 5] = [
        ("action_east", &[47, 53, 50, 53, 50, 47, 60]),
        ("action_north", &[25, 17, 18, 17, 18, 25, 26]),
        ("action_south", &[60, 62, 61, 62, 61, 60, 68]),
        ("kiss_east", &[54, 62, 71, 66]),
        ("sleep_east", &[61]),
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
            let jacket_folds: &[[u32; 2]] = match case {
                "action_east" => &[
                    [120, 41],
                    [200, 41],
                    [280, 41],
                    [360, 41],
                    [517, 43],
                    [119, 45],
                    [199, 45],
                    [279, 45],
                    [359, 45],
                ],
                "action_south" => &[
                    [117, 43],
                    [123, 43],
                    [203, 43],
                    [277, 43],
                    [283, 43],
                    [363, 43],
                    [516, 43],
                    [523, 43],
                    [36, 44],
                    [43, 44],
                    [436, 44],
                    [443, 44],
                ],
                "kiss_east" => &[
                    [200, 43],
                    [37, 44],
                    [118, 44],
                    [207, 44],
                    [278, 44],
                    [206, 45],
                ],
                "sleep_east" => &[[38, 43]],
                _ => &[],
            };
            let frames = expected_per_frame.len() as u32;
            let asset = format!(
                "assets/animations/NPCs/Reina/Sprites/Spring/spr_npc_reina_spring_{case}.png"
            );
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
                // The four reviewed skin shades also occur in 28 isolated jacket
                // folds. Guard both their preservation and every omitted skin pixel.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !jacket_folds.contains(&[x, y]))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Reina Spring standard material mismatch: {id} {case} [{x},{y}]"
                );
                mask.push(p != q);
                if p != q {
                    let index = source.iter().position(|c| rgba(*c) == p.0).expect(
                        "hair, blouse, jacket, shorts, shoes or another non-skin color changed",
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
