use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/reina-summer-specials-study and the local accepted Reina world baseline"]
fn reina_spring_actions_cover_skin_and_preserve_jacket_and_mouth_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/reina-summer-specials-study");
    let baseline =
        root.join("generated/characters-reina-juniper-march-world-trial/characters/reina");
    let set = std::env::var_os("FOM_REINA_SPRING_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/reina-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/reina-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..109];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..109], prior);
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
    // Literal source landmarks distinguish exposed necks, hands and midriff
    // from same-colored jacket folds, mouth interiors and blouse trim.
    let landmarks = [
        ("blink_east", 1, 39, 34, 0x8E4538, true),
        ("blink_east", 1, 40, 35, 0xB36844, true),
        ("blink_east", 1, 38, 36, 0xB36844, true),
        ("blink_east", 1, 37, 43, 0xB36844, false),
        ("blink_east", 1, 39, 45, 0xB36844, true),
        ("blink_east", 1, 42, 41, 0xB36844, true),
        ("blink_east", 1, 40, 30, 0x63413B, false),
        ("blink_south", 1, 39, 34, 0x8E4538, true),
        ("blink_south", 1, 39, 35, 0xB36844, true),
        ("blink_south", 1, 37, 36, 0xB36844, true),
        ("blink_south", 1, 36, 43, 0xB36844, false),
        ("blink_south", 1, 43, 43, 0xB36844, false),
        ("blink_south", 1, 38, 45, 0xB36844, true),
        ("blink_south", 1, 40, 42, 0xF0B0A3, false),
        ("drink_east", 0, 40, 42, 0xB36844, true),
        ("drink_east", 0, 39, 43, 0x712922, true),
        ("drink_east", 0, 37, 43, 0xB36844, false),
        ("drink_east", 0, 38, 44, 0xB36844, false),
        ("drink_east", 0, 42, 46, 0xB36844, true),
        ("drink_east", 0, 39, 41, 0xF0B0A3, false),
        ("drink_east", 1, 39, 40, 0xB36844, true),
        ("drink_east", 1, 38, 42, 0xB36844, true),
        ("drink_east", 1, 37, 43, 0xB36844, false),
        ("drink_east", 1, 42, 44, 0xB36844, false),
        ("drink_east", 1, 38, 35, 0xB36844, true),
        ("drink_east", 1, 35, 45, 0x010101, false),
        ("drink_north", 0, 45, 44, 0xB36844, true),
        ("drink_north", 0, 33, 46, 0x8E4538, true),
        ("drink_north", 0, 40, 43, 0xF7E29C, false),
        ("drink_north", 0, 40, 36, 0x63413B, false),
        ("drink_north", 1, 46, 43, 0x000000, false),
        ("drink_north", 1, 33, 46, 0x8E4538, true),
        ("drink_south", 0, 34, 42, 0xB36844, true),
        ("drink_south", 0, 43, 43, 0xB36844, false),
        ("drink_south", 0, 39, 41, 0x8E4538, true),
        ("drink_south", 0, 37, 47, 0xB36844, true),
        ("drink_south", 1, 35, 40, 0xB36844, true),
        ("drink_south", 1, 35, 42, 0xB36844, true),
        ("drink_south", 1, 36, 44, 0xB36844, false),
        ("drink_south", 1, 39, 42, 0xF0B0A3, false),
        ("drink_south", 1, 37, 47, 0xB36844, true),
        ("eat_east", 0, 40, 41, 0xB36844, false),
        ("eat_east", 0, 39, 43, 0xB36844, false),
        ("eat_east", 0, 42, 42, 0xB36844, true),
        ("eat_east", 0, 44, 42, 0x8E4538, true),
        ("eat_east", 0, 42, 46, 0xB36844, true),
        ("eat_east", 1, 38, 42, 0xB36844, false),
        ("eat_east", 1, 40, 43, 0xB36844, false),
        ("eat_east", 1, 46, 40, 0xB36844, true),
        ("eat_east", 1, 48, 41, 0x000000, false),
        ("eat_east", 1, 40, 39, 0x712922, true),
        ("eat_east", 2, 39, 42, 0xB36844, false),
        ("eat_east", 2, 43, 36, 0xC83E37, false),
        ("eat_east", 2, 42, 35, 0x410808, false),
        ("eat_east", 2, 44, 39, 0xB36844, true),
        ("eat_east", 2, 44, 40, 0x8E4538, true),
        ("eat_east", 3, 38, 42, 0xB36844, false),
        ("eat_east", 3, 39, 44, 0xB36844, false),
        ("eat_east", 3, 38, 45, 0xB36844, false),
        ("eat_east", 3, 42, 43, 0xB36844, true),
        ("eat_east", 3, 43, 44, 0xB36844, true),
        ("eat_east", 4, 38, 43, 0xB36844, false),
        ("eat_east", 4, 40, 42, 0xB36844, true),
        ("eat_east", 4, 42, 41, 0xB36844, true),
        ("eat_east", 4, 41, 44, 0xB36844, true),
        ("eat_east", 4, 40, 45, 0xD2962F, false),
        ("eat_north", 0, 45, 44, 0xB36844, true),
        ("eat_north", 0, 33, 46, 0x8E4538, true),
        ("eat_north", 0, 40, 43, 0xF7E29C, false),
        ("eat_north", 1, 46, 43, 0x000000, false),
        ("eat_south", 0, 36, 43, 0xB36844, false),
        ("eat_south", 0, 43, 43, 0xB36844, false),
        ("eat_south", 0, 35, 45, 0xB36844, true),
        ("eat_south", 0, 35, 46, 0xB36844, true),
        ("eat_south", 0, 41, 47, 0xB36844, true),
        ("eat_south", 1, 37, 43, 0xB36844, false),
        ("eat_south", 1, 38, 44, 0xB36844, false),
        ("eat_south", 1, 43, 43, 0xB36844, false),
        ("eat_south", 1, 36, 47, 0xB36844, true),
        ("eat_south", 1, 37, 49, 0xB36844, true),
        ("eat_south", 1, 38, 40, 0x712922, true),
        ("eat_south", 2, 43, 43, 0xB36844, false),
        ("eat_south", 2, 38, 36, 0xC83E37, false),
        ("eat_south", 2, 38, 35, 0x410808, false),
        ("eat_south", 2, 37, 40, 0xB36844, true),
        ("eat_south", 2, 37, 47, 0xB36844, true),
        ("eat_south", 3, 43, 43, 0xB36844, false),
        ("eat_south", 3, 36, 43, 0xB36844, true),
        ("eat_south", 3, 36, 44, 0xB36844, true),
        ("eat_south", 3, 37, 47, 0xB36844, true),
        ("sit_east", 0, 37, 43, 0xB36844, false),
        ("sit_east", 0, 42, 41, 0xB36844, true),
        ("sit_east", 0, 35, 45, 0xB36844, true),
        ("sit_east", 0, 35, 47, 0x571D1F, true),
        ("sit_east", 0, 42, 46, 0xB36844, true),
        ("sit_east", 0, 40, 44, 0xD8D8DC, false),
        ("sit_north", 0, 33, 46, 0x8E4538, true),
        ("sit_north", 0, 33, 47, 0x000000, false),
        ("sit_north", 0, 46, 46, 0x8E4538, true),
        ("sit_north", 0, 46, 47, 0x000000, false),
        ("sit_north", 0, 39, 43, 0xF7E29C, false),
        ("sit_south", 0, 36, 43, 0xB36844, false),
        ("sit_south", 0, 43, 43, 0xB36844, false),
        ("sit_south", 0, 33, 46, 0x8E4538, true),
        ("sit_south", 0, 33, 47, 0x000000, false),
        ("sit_south", 0, 37, 47, 0xB36844, true),
        ("sit_south", 0, 40, 42, 0xF0B0A3, false),
    ];
    let cases: [(&str, &[usize]); 11] = [
        ("blink_east", &[64, 74, 64]),
        ("blink_south", &[72, 82, 72]),
        ("drink_east", &[43, 52, 43]),
        ("drink_north", &[14, 9, 14]),
        ("drink_south", &[52, 64, 52]),
        ("eat_east", &[38, 49, 40, 46, 40]),
        ("eat_north", &[14, 9, 14]),
        ("eat_south", &[52, 61, 58, 69, 50]),
        ("sit_east", &[44]),
        ("sit_north", &[14]),
        ("sit_south", &[50]),
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
                "blink_east" => &[[37, 43], [117, 43], [197, 43]],
                "blink_south" => &[
                    [36, 43],
                    [43, 43],
                    [116, 43],
                    [123, 43],
                    [196, 43],
                    [203, 43],
                ],
                "drink_east" => &[
                    [37, 43],
                    [117, 43],
                    [197, 43],
                    [38, 44],
                    [122, 44],
                    [198, 44],
                ],
                "drink_south" => &[[43, 43], [123, 43], [203, 43], [116, 44]],
                "eat_east" => &[
                    [40, 41],
                    [39, 43],
                    [118, 42],
                    [120, 43],
                    [199, 42],
                    [278, 42],
                    [279, 44],
                    [278, 45],
                    [358, 43],
                ],
                "eat_south" => &[
                    [36, 43],
                    [43, 43],
                    [117, 43],
                    [123, 43],
                    [203, 43],
                    [283, 43],
                    [356, 43],
                    [363, 43],
                    [118, 44],
                ],
                "sit_east" => &[[37, 43]],
                "sit_south" => &[[36, 43], [43, 43]],
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
                // The four reviewed skin shades also occur in 40 isolated jacket
                // folds. Guard both their preservation and every omitted skin pixel.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !jacket_folds.contains(&[x, y]))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Reina Spring action material mismatch: {id} {case} [{x},{y}]"
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
