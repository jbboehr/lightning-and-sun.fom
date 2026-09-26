use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/reina-spring-finish-study and the local accepted Reina world baseline"]
fn reina_spring_reactions_cover_skin_and_preserve_jacket_book_and_mouth() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/reina-spring-finish-study");
    let baseline =
        root.join("generated/characters-reina-juniper-march-standard-trial/characters/reina");
    let set = std::env::var_os("FOM_REINA_SPRING_REACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/reina-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/reina-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..125];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..125], prior);
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
    // Independently inspected source landmarks distinguish exposed hands and
    // faces from jacket folds, book covers/pages and shocked mouth details.
    let landmarks = [
        ("shocked_start_south", 0, 39, 36, 0xB36844, true),
        ("shocked_start_south", 0, 40, 38, 0xB36844, true),
        ("shocked_start_south", 0, 39, 42, 0x8E4538, true),
        ("shocked_start_south", 0, 36, 44, 0xB36844, false),
        ("shocked_start_south", 0, 43, 44, 0xB36844, false),
        ("shocked_start_south", 0, 33, 47, 0xB36844, true),
        ("shocked_start_south", 0, 34, 48, 0xB36844, true),
        ("shocked_start_south", 0, 46, 47, 0xB36844, true),
        ("shocked_start_south", 0, 38, 46, 0xB36844, true),
        ("shocked_start_south", 0, 41, 50, 0xB36844, true),
        ("shocked_start_south", 0, 40, 43, 0xF0B0A3, false),
        ("shocked_start_south", 0, 38, 30, 0x2B1919, false),
        ("shocked_end_south", 0, 39, 36, 0xB36844, true),
        ("shocked_end_south", 0, 40, 38, 0xB36844, true),
        ("shocked_end_south", 0, 39, 42, 0x8E4538, true),
        ("shocked_end_south", 0, 36, 44, 0xB36844, false),
        ("shocked_end_south", 0, 43, 44, 0xB36844, false),
        ("shocked_end_south", 0, 33, 47, 0xB36844, true),
        ("shocked_end_south", 0, 34, 48, 0xB36844, true),
        ("shocked_end_south", 0, 46, 47, 0xB36844, true),
        ("shocked_end_south", 0, 38, 46, 0xB36844, true),
        ("shocked_end_south", 0, 41, 50, 0xB36844, true),
        ("shocked_end_south", 0, 40, 43, 0xF0B0A3, false),
        ("shocked_end_south", 0, 38, 30, 0x2B1919, false),
        ("shocked_loop_south", 0, 32, 33, 0x712922, true),
        ("shocked_loop_south", 0, 32, 37, 0xB36844, true),
        ("shocked_loop_south", 0, 47, 37, 0xB36844, true),
        ("shocked_loop_south", 0, 39, 34, 0xB36844, true),
        ("shocked_loop_south", 0, 39, 36, 0x410808, false),
        ("shocked_loop_south", 0, 39, 37, 0x9E2626, false),
        ("shocked_loop_south", 0, 39, 39, 0x8E4538, true),
        ("shocked_loop_south", 0, 34, 38, 0xB36844, false),
        ("shocked_loop_south", 0, 45, 38, 0xB36844, false),
        ("shocked_loop_south", 0, 33, 39, 0xB36844, false),
        ("shocked_loop_south", 0, 46, 39, 0xB36844, false),
        ("shocked_loop_south", 0, 36, 40, 0xB36844, false),
        ("shocked_loop_south", 0, 43, 40, 0xB36844, false),
        ("shocked_loop_south", 0, 38, 43, 0xB36844, true),
        ("shocked_loop_south", 0, 35, 48, 0xB36844, true),
        ("shocked_loop_south", 0, 44, 48, 0xB36844, true),
        ("shocked_loop_south", 0, 34, 39, 0xEABF67, false),
        ("shocked_loop_south", 0, 38, 29, 0x63413B, false),
        ("read_sit_start_south", 0, 39, 35, 0xB36844, true),
        ("read_sit_start_south", 0, 36, 43, 0xB36844, false),
        ("read_sit_start_south", 0, 43, 43, 0xB36844, false),
        ("read_sit_start_south", 0, 34, 46, 0x8E4538, true),
        ("read_sit_start_south", 0, 34, 47, 0x571D1F, true),
        ("read_sit_start_south", 0, 37, 47, 0xB36844, true),
        ("read_sit_start_south", 0, 42, 47, 0xB36844, true),
        ("read_sit_start_south", 0, 40, 42, 0xF0B0A3, false),
        ("read_sit_start_south", 1, 39, 36, 0xB36844, true),
        ("read_sit_start_south", 1, 35, 45, 0xB36844, true),
        ("read_sit_start_south", 1, 34, 45, 0x712922, true),
        ("read_sit_start_south", 1, 35, 47, 0x571D1F, true),
        ("read_sit_start_south", 1, 44, 45, 0xB36844, true),
        ("read_sit_start_south", 1, 37, 42, 0xDF7175, false),
        ("read_sit_start_south", 1, 39, 42, 0xF6E4D7, false),
        ("read_sit_start_south", 1, 36, 42, 0x000000, false),
        ("read_sit_start_south", 1, 37, 48, 0xAF4F52, false),
        ("read_sit_start_south", 1, 39, 49, 0xFD9CA0, false),
        ("read_sit_start_south", 2, 39, 36, 0xB36844, true),
        ("read_sit_start_south", 2, 39, 41, 0x712922, true),
        ("read_sit_start_south", 2, 35, 46, 0x000000, false),
        ("read_sit_start_south", 2, 36, 47, 0x000000, false),
        ("read_sit_start_south", 2, 38, 42, 0xF6E4D7, false),
        ("read_sit_start_south", 2, 39, 43, 0xF6E4D7, false),
        ("read_sit_start_south", 2, 39, 46, 0xAF4F52, false),
        ("read_sit_start_south", 2, 38, 48, 0xAF4F52, false),
        ("read_sit_loop_south", 0, 40, 35, 0xB36844, true),
        ("read_sit_loop_south", 0, 40, 39, 0x8E4538, true),
        ("read_sit_loop_south", 0, 39, 41, 0x8E4538, true),
        ("read_sit_loop_south", 0, 37, 42, 0xB36844, false),
        ("read_sit_loop_south", 0, 42, 42, 0xB36844, false),
        ("read_sit_loop_south", 0, 33, 42, 0xF6E4D7, false),
        ("read_sit_loop_south", 0, 34, 43, 0xF6E4D7, false),
        ("read_sit_loop_south", 0, 32, 44, 0xFD9CA0, false),
        ("read_sit_loop_south", 0, 35, 46, 0xDF7175, false),
        ("read_sit_loop_south", 0, 40, 48, 0xFD9CA0, false),
        ("read_sit_loop_south", 1, 39, 36, 0xB36844, true),
        ("read_sit_loop_south", 1, 39, 41, 0x712922, true),
        ("read_sit_loop_south", 1, 37, 43, 0xB36844, false),
        ("read_sit_loop_south", 1, 42, 43, 0xB36844, false),
        ("read_sit_loop_south", 1, 34, 44, 0xF6E4D7, false),
        ("read_sit_loop_south", 1, 36, 45, 0xC9AF9C, false),
        ("read_sit_loop_south", 1, 34, 46, 0x863A3C, false),
        ("read_sit_loop_south", 1, 38, 47, 0xAF4F52, false),
        ("read_sit_loop_south", 1, 39, 49, 0xFD9CA0, false),
        ("read_sit_loop_south", 2, 38, 35, 0xB36844, true),
        ("read_sit_loop_south", 2, 38, 39, 0x8E4538, true),
        ("read_sit_loop_south", 2, 39, 41, 0x8E4538, true),
        ("read_sit_loop_south", 2, 37, 42, 0xB36844, false),
        ("read_sit_loop_south", 2, 42, 42, 0xB36844, false),
        ("read_sit_loop_south", 2, 46, 42, 0xF6E4D7, false),
        ("read_sit_loop_south", 2, 45, 43, 0xF6E4D7, false),
        ("read_sit_loop_south", 2, 47, 44, 0xFD9CA0, false),
        ("read_sit_loop_south", 2, 44, 46, 0xDF7175, false),
        ("read_sit_loop_south", 2, 39, 48, 0xFD9CA0, false),
        ("read_sit_loop_south", 3, 39, 36, 0xB36844, true),
        ("read_sit_loop_south", 3, 39, 41, 0x712922, true),
        ("read_sit_loop_south", 3, 37, 43, 0xB36844, false),
        ("read_sit_loop_south", 3, 42, 43, 0xB36844, false),
        ("read_sit_loop_south", 3, 34, 44, 0xF6E4D7, false),
        ("read_sit_loop_south", 3, 36, 45, 0xC9AF9C, false),
        ("read_sit_loop_south", 3, 34, 46, 0x863A3C, false),
        ("read_sit_loop_south", 3, 38, 47, 0xAF4F52, false),
        ("read_sit_loop_south", 3, 39, 49, 0xFD9CA0, false),
        ("read_sit_end_south", 0, 39, 36, 0xB36844, true),
        ("read_sit_end_south", 0, 35, 46, 0x000000, false),
        ("read_sit_end_south", 0, 36, 47, 0x000000, false),
        ("read_sit_end_south", 0, 39, 43, 0xF6E4D7, false),
        ("read_sit_end_south", 0, 39, 46, 0xAF4F52, false),
        ("read_sit_end_south", 1, 39, 36, 0xB36844, true),
        ("read_sit_end_south", 1, 35, 45, 0xB36844, true),
        ("read_sit_end_south", 1, 35, 47, 0x571D1F, true),
        ("read_sit_end_south", 1, 39, 42, 0xF6E4D7, false),
        ("read_sit_end_south", 1, 39, 49, 0xFD9CA0, false),
        ("read_sit_end_south", 2, 39, 35, 0xB36844, true),
        ("read_sit_end_south", 2, 36, 43, 0xB36844, false),
        ("read_sit_end_south", 2, 43, 43, 0xB36844, false),
        ("read_sit_end_south", 2, 34, 46, 0x8E4538, true),
        ("read_sit_end_south", 2, 37, 47, 0xB36844, true),
        ("read_sit_end_south", 2, 40, 42, 0xF0B0A3, false),
    ];
    let cases: [(&str, &[usize]); 6] = [
        ("shocked_start_south", &[78]),
        ("shocked_loop_south", &[86]),
        ("shocked_end_south", &[78]),
        ("read_sit_start_south", &[46, 50, 44]),
        ("read_sit_loop_south", &[37, 50, 37, 50]),
        ("read_sit_end_south", &[54, 40, 46]),
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
                "shocked_start_south" => &[[36, 44], [43, 44]],
                "shocked_loop_south" => {
                    &[[34, 38], [45, 38], [33, 39], [46, 39], [36, 40], [43, 40]]
                }
                "shocked_end_south" => &[[36, 44], [43, 44]],
                "read_sit_start_south" => &[[36, 43], [43, 43]],
                "read_sit_loop_south" => &[
                    [37, 42],
                    [42, 42],
                    [197, 42],
                    [202, 42],
                    [117, 43],
                    [122, 43],
                    [277, 43],
                    [282, 43],
                ],
                "read_sit_end_south" => &[[196, 43], [203, 43]],
                _ => &[],
            };
            let frames = expected_per_frame.len() as u32;
            let prefix = if case.starts_with("read_sit_") {
                "spr_npc_reina_specialanimation_spring"
            } else {
                "spr_npc_reina_spring"
            };
            let asset = format!("assets/animations/NPCs/Reina/Sprites/Spring/{prefix}_{case}.png");
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
                // The four reviewed skin shades also occur in 22 isolated jacket
                // folds. Guard both their preservation and every omitted skin pixel.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !jacket_folds.contains(&[x, y]))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Reina Spring reaction material mismatch: {id} {case} [{x},{y}]"
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
