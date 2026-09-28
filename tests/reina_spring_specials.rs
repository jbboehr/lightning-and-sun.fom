use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/reina-winter-standard-study and the local accepted Reina world baseline"]
fn reina_spring_specials_cover_skin_and_preserve_jacket_and_writing_props() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/reina-winter-standard-study");
    let baseline =
        root.join("generated/characters-reina-juniper-march-reactions-trial/characters/reina");
    let set = std::env::var_os("FOM_REINA_SPRING_SPECIALS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/reina-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/reina-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..131];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..131], prior);
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
    // faces from jacket folds, pencil colors, clipboard backing and paper.
    let landmarks = [
        ("write_start_south", 0, 39, 36, 0xB36844, true),
        ("write_start_south", 0, 39, 42, 0x8E4538, true),
        ("write_start_south", 0, 36, 44, 0xB36844, false),
        ("write_start_south", 0, 43, 44, 0xB36844, false),
        ("write_start_south", 0, 33, 45, 0xD36A0E, false),
        ("write_start_south", 0, 34, 45, 0xF9AB6C, false),
        ("write_start_south", 0, 34, 48, 0xFFD8D1, false),
        ("write_start_south", 0, 34, 49, 0x663409, false),
        ("write_start_south", 0, 46, 46, 0xB28159, false),
        ("write_start_south", 0, 43, 49, 0xB28159, false),
        ("write_start_south", 0, 39, 46, 0xB36844, true),
        ("write_start_south", 0, 40, 43, 0xF0B0A3, false),
        ("write_start_south", 0, 39, 31, 0x63413B, false),
        ("write_start_south", 1, 39, 35, 0x000000, false),
        ("write_start_south", 1, 36, 43, 0xB36844, false),
        ("write_start_south", 1, 38, 45, 0xB36844, true),
        ("write_start_south", 1, 42, 48, 0xB36844, true),
        ("write_start_south", 1, 43, 48, 0xB36844, true),
        ("write_start_south", 1, 42, 49, 0xB36844, true),
        ("write_start_south", 1, 34, 42, 0xD36A0E, false),
        ("write_start_south", 1, 33, 45, 0xFFD8D1, false),
        ("write_start_south", 1, 40, 48, 0xC3D1DD, false),
        ("write_start_south", 1, 43, 45, 0xF5F5F5, false),
        ("write_start_south", 1, 40, 41, 0x8E4538, true),
        ("write_sit_start_south", 0, 39, 36, 0xB36844, true),
        ("write_sit_start_south", 0, 39, 42, 0xF0B0A3, false),
        ("write_sit_start_south", 0, 36, 44, 0xB36844, false),
        ("write_sit_start_south", 0, 43, 44, 0xB36844, false),
        ("write_sit_start_south", 0, 33, 45, 0xD36A0E, false),
        ("write_sit_start_south", 0, 34, 45, 0xF9AB6C, false),
        ("write_sit_start_south", 0, 34, 48, 0xFFD8D1, false),
        ("write_sit_start_south", 0, 34, 49, 0x663409, false),
        ("write_sit_start_south", 0, 46, 46, 0xB28159, false),
        ("write_sit_start_south", 0, 43, 49, 0xB28159, false),
        ("write_sit_start_south", 0, 39, 46, 0xA13761, false),
        ("write_sit_start_south", 0, 40, 43, 0xF5F5F5, false),
        ("write_sit_start_south", 0, 39, 31, 0x63413B, false),
        ("write_sit_start_south", 1, 39, 35, 0x000000, false),
        ("write_sit_start_south", 1, 36, 43, 0xB36844, false),
        ("write_sit_start_south", 1, 38, 45, 0x581831, false),
        ("write_sit_start_south", 1, 42, 48, 0xB36844, true),
        ("write_sit_start_south", 1, 43, 48, 0xB36844, true),
        ("write_sit_start_south", 1, 42, 49, 0xB36844, true),
        ("write_sit_start_south", 1, 34, 42, 0xD36A0E, false),
        ("write_sit_start_south", 1, 33, 45, 0xFFD8D1, false),
        ("write_sit_start_south", 1, 40, 48, 0xC3D1DD, false),
        ("write_sit_start_south", 1, 43, 45, 0xF5F5F5, false),
        ("write_sit_start_south", 1, 40, 41, 0x8E4538, true),
        ("write_loop_south", 0, 39, 35, 0x000000, false),
        ("write_loop_south", 0, 39, 41, 0x8E4538, true),
        ("write_loop_south", 0, 37, 45, 0xB36844, true),
        ("write_loop_south", 0, 36, 46, 0x712922, true),
        ("write_loop_south", 0, 43, 47, 0xB36844, true),
        ("write_loop_south", 0, 42, 48, 0xB36844, true),
        ("write_loop_south", 0, 43, 44, 0xB28159, false),
        ("write_loop_south", 0, 39, 47, 0xC3D1DD, false),
        ("write_loop_south", 0, 37, 42, 0xF9AB6C, false),
        ("write_loop_south", 0, 39, 44, 0xFFD8D1, false),
        ("write_loop_south", 0, 40, 44, 0xFAB680, false),
        ("write_loop_south", 1, 39, 35, 0x000000, false),
        ("write_loop_south", 1, 36, 46, 0xB36844, true),
        ("write_loop_south", 1, 35, 47, 0x712922, true),
        ("write_loop_south", 1, 43, 47, 0xB36844, true),
        ("write_loop_south", 1, 42, 48, 0xB36844, true),
        ("write_loop_south", 1, 36, 43, 0xF9AB6C, false),
        ("write_loop_south", 1, 38, 45, 0xFFD8D1, false),
        ("write_loop_south", 1, 40, 46, 0xF5F5F5, false),
        ("write_loop_south", 2, 39, 35, 0x000000, false),
        ("write_loop_south", 2, 34, 44, 0x8E4538, true),
        ("write_loop_south", 2, 35, 45, 0xB36844, true),
        ("write_loop_south", 2, 34, 46, 0x712922, true),
        ("write_loop_south", 2, 37, 47, 0x581831, false),
        ("write_loop_south", 2, 40, 45, 0xB36844, true),
        ("write_loop_south", 2, 42, 48, 0xB36844, true),
        ("write_loop_south", 2, 42, 49, 0xB36844, true),
        ("write_loop_south", 2, 41, 47, 0x000000, false),
        ("write_loop_south", 2, 40, 48, 0x000000, false),
        ("write_loop_south", 2, 36, 42, 0xD36A0E, false),
        ("write_loop_south", 2, 38, 44, 0xFAB680, false),
        ("write_loop_south", 2, 39, 45, 0x000000, false),
        ("write_loop_south", 2, 45, 45, 0xB28159, false),
        ("write_loop_south", 3, 39, 35, 0x000000, false),
        ("write_loop_south", 3, 34, 45, 0x8E4538, true),
        ("write_loop_south", 3, 34, 46, 0xB36844, true),
        ("write_loop_south", 3, 35, 47, 0xB36844, true),
        ("write_loop_south", 3, 43, 47, 0xB36844, true),
        ("write_loop_south", 3, 42, 48, 0xB36844, true),
        ("write_loop_south", 3, 35, 43, 0xF9AB6C, false),
        ("write_loop_south", 3, 37, 45, 0xFFD8D1, false),
        ("write_loop_south", 3, 38, 46, 0x663409, false),
        ("write_loop_south", 3, 41, 46, 0x000000, false),
        ("write_sit_loop_south", 0, 39, 35, 0x000000, false),
        ("write_sit_loop_south", 0, 39, 41, 0x8E4538, true),
        ("write_sit_loop_south", 0, 37, 45, 0xB36844, true),
        ("write_sit_loop_south", 0, 36, 46, 0x712922, true),
        ("write_sit_loop_south", 0, 43, 47, 0xB36844, true),
        ("write_sit_loop_south", 0, 42, 48, 0xB36844, true),
        ("write_sit_loop_south", 0, 43, 44, 0xB28159, false),
        ("write_sit_loop_south", 0, 39, 47, 0xC3D1DD, false),
        ("write_sit_loop_south", 0, 37, 42, 0xF9AB6C, false),
        ("write_sit_loop_south", 0, 39, 44, 0xFFD8D1, false),
        ("write_sit_loop_south", 0, 40, 44, 0xFAB680, false),
        ("write_sit_loop_south", 1, 39, 35, 0x000000, false),
        ("write_sit_loop_south", 1, 36, 46, 0xB36844, true),
        ("write_sit_loop_south", 1, 35, 47, 0x712922, true),
        ("write_sit_loop_south", 1, 43, 47, 0xB36844, true),
        ("write_sit_loop_south", 1, 42, 48, 0xB36844, true),
        ("write_sit_loop_south", 1, 36, 43, 0xF9AB6C, false),
        ("write_sit_loop_south", 1, 38, 45, 0xFFD8D1, false),
        ("write_sit_loop_south", 1, 40, 46, 0xF5F5F5, false),
        ("write_sit_loop_south", 2, 39, 35, 0x000000, false),
        ("write_sit_loop_south", 2, 34, 44, 0x8E4538, true),
        ("write_sit_loop_south", 2, 35, 45, 0xB36844, true),
        ("write_sit_loop_south", 2, 34, 46, 0x712922, true),
        ("write_sit_loop_south", 2, 37, 47, 0xB36844, true),
        ("write_sit_loop_south", 2, 40, 45, 0xD2962F, false),
        ("write_sit_loop_south", 2, 42, 48, 0xB36844, true),
        ("write_sit_loop_south", 2, 42, 49, 0xB36844, true),
        ("write_sit_loop_south", 2, 41, 47, 0x57342B, false),
        ("write_sit_loop_south", 2, 40, 48, 0x57342B, false),
        ("write_sit_loop_south", 2, 36, 42, 0xD36A0E, false),
        ("write_sit_loop_south", 2, 38, 44, 0xFAB680, false),
        ("write_sit_loop_south", 2, 39, 45, 0x000000, false),
        ("write_sit_loop_south", 2, 45, 45, 0xB28159, false),
        ("write_sit_loop_south", 3, 39, 35, 0x000000, false),
        ("write_sit_loop_south", 3, 34, 45, 0x8E4538, true),
        ("write_sit_loop_south", 3, 34, 46, 0xB36844, true),
        ("write_sit_loop_south", 3, 35, 47, 0xB36844, true),
        ("write_sit_loop_south", 3, 43, 47, 0xB36844, true),
        ("write_sit_loop_south", 3, 42, 48, 0xB36844, true),
        ("write_sit_loop_south", 3, 35, 43, 0xF9AB6C, false),
        ("write_sit_loop_south", 3, 37, 45, 0xFFD8D1, false),
        ("write_sit_loop_south", 3, 38, 46, 0x663409, false),
        ("write_sit_loop_south", 3, 41, 46, 0x57342B, false),
        ("write_end_south", 0, 39, 35, 0x000000, false),
        ("write_end_south", 0, 36, 43, 0xB36844, false),
        ("write_end_south", 0, 38, 45, 0xB36844, true),
        ("write_end_south", 0, 42, 48, 0xB36844, true),
        ("write_end_south", 0, 43, 48, 0xB36844, true),
        ("write_end_south", 0, 34, 42, 0xD36A0E, false),
        ("write_end_south", 0, 33, 45, 0xFFD8D1, false),
        ("write_end_south", 0, 43, 45, 0xF5F5F5, false),
        ("write_end_south", 1, 39, 36, 0xB36844, true),
        ("write_end_south", 1, 36, 44, 0xB36844, false),
        ("write_end_south", 1, 43, 44, 0xB36844, false),
        ("write_end_south", 1, 33, 45, 0xD36A0E, false),
        ("write_end_south", 1, 34, 48, 0xFFD8D1, false),
        ("write_end_south", 1, 46, 46, 0xB28159, false),
        ("write_end_south", 1, 39, 46, 0xB36844, true),
        ("write_sit_end_south", 0, 39, 35, 0x000000, false),
        ("write_sit_end_south", 0, 36, 43, 0xB36844, false),
        ("write_sit_end_south", 0, 38, 45, 0x581831, false),
        ("write_sit_end_south", 0, 42, 48, 0xB36844, true),
        ("write_sit_end_south", 0, 43, 48, 0xB36844, true),
        ("write_sit_end_south", 0, 34, 42, 0xD36A0E, false),
        ("write_sit_end_south", 0, 33, 45, 0xFFD8D1, false),
        ("write_sit_end_south", 0, 43, 45, 0xF5F5F5, false),
        ("write_sit_end_south", 1, 39, 36, 0xB36844, true),
        ("write_sit_end_south", 1, 36, 44, 0xB36844, false),
        ("write_sit_end_south", 1, 43, 44, 0xB36844, false),
        ("write_sit_end_south", 1, 33, 45, 0xD36A0E, false),
        ("write_sit_end_south", 1, 34, 48, 0xFFD8D1, false),
        ("write_sit_end_south", 1, 46, 46, 0xB28159, false),
        ("write_sit_end_south", 1, 39, 46, 0xA13761, false),
    ];
    let cases: [(&str, &[usize]); 6] = [
        ("write_start_south", &[58, 52]),
        ("write_loop_south", &[55, 55, 53, 56]),
        ("write_end_south", &[52, 58]),
        ("write_sit_start_south", &[54, 45]),
        ("write_sit_loop_south", &[48, 48, 50, 49]),
        ("write_sit_end_south", &[45, 54]),
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
                "write_start_south" => &[[116, 43], [36, 44], [43, 44]],
                "write_end_south" => &[[36, 43], [116, 44], [123, 44]],
                "write_sit_start_south" => &[[116, 43], [36, 44], [43, 44]],
                "write_sit_end_south" => &[[36, 43], [116, 44], [123, 44]],
                _ => &[],
            };
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_reina_specialanimation_spring";
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
                // The four reviewed skin shades also occur in 12 isolated jacket
                // folds. Guard both their preservation and every omitted skin pixel.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !jacket_folds.contains(&[x, y]))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Reina Spring writing material mismatch: {id} {case} [{x},{y}]"
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
