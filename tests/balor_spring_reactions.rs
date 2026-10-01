use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/balor-winter-standard-study and the local accepted Balor world baseline"]
fn balor_spring_reactions_cover_skin_and_preserve_book() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-winter-standard-study");
    let baseline =
        root.join("generated/characters-balor-valen-eiland-spring-standard-trial/characters/balor");
    let set = std::env::var_os("FOM_BALOR_SPRING_REACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/balor-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..132];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..132], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 223);
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
    let source = [0xFCD9B3, 0xF0B988, 0xD37A57, 0x672115];
    // Literal landmarks guard all source frames, exposed fingers beside the
    // book, pale pages, cover, facial features, scarf, clothes and trousers.
    // The four world shades are exclusively skin in these six strips.
    let landmarks = [
        ("shocked_start_south", 0, 40, 31, 0x9B83B7, false),
        ("shocked_start_south", 0, 40, 37, 0xFCD9B3, true),
        ("shocked_start_south", 0, 39, 43, 0x424E88, false),
        ("shocked_start_south", 0, 40, 47, 0xECF0E9, false),
        ("shocked_start_south", 0, 38, 51, 0x686589, false),
        ("shocked_loop_south", 0, 40, 31, 0x4A3D66, false),
        ("shocked_loop_south", 0, 40, 37, 0x9E2626, false),
        ("shocked_loop_south", 0, 39, 43, 0xB3AFBD, false),
        ("shocked_loop_south", 0, 40, 47, 0x000000, false),
        ("shocked_end_south", 0, 40, 31, 0x9B83B7, false),
        ("shocked_end_south", 0, 40, 37, 0xFCD9B3, true),
        ("shocked_end_south", 0, 39, 43, 0x424E88, false),
        ("shocked_end_south", 0, 40, 47, 0xECF0E9, false),
        ("shocked_end_south", 0, 38, 51, 0x686589, false),
        ("read_sit_start_south", 0, 40, 31, 0x9B83B7, false),
        ("read_sit_start_south", 0, 40, 37, 0xFCD9B3, true),
        ("read_sit_start_south", 0, 39, 43, 0x424E88, false),
        ("read_sit_start_south", 0, 40, 47, 0x461839, false),
        ("read_sit_start_south", 0, 38, 51, 0x686589, false),
        ("read_sit_start_south", 1, 40, 31, 0x9B83B7, false),
        ("read_sit_start_south", 1, 40, 37, 0xFCD9B3, true),
        ("read_sit_start_south", 1, 39, 43, 0xF6E4D7, false),
        ("read_sit_start_south", 1, 40, 47, 0xC9AF9C, false),
        ("read_sit_start_south", 1, 38, 51, 0x000000, false),
        ("read_sit_start_south", 2, 40, 31, 0x9B83B7, false),
        ("read_sit_start_south", 2, 40, 37, 0xFCD9B3, true),
        ("read_sit_start_south", 2, 39, 43, 0xF6E4D7, false),
        ("read_sit_start_south", 2, 40, 47, 0x98A2A6, false),
        ("read_sit_start_south", 2, 38, 51, 0x686589, false),
        ("read_sit_loop_south", 0, 40, 31, 0x9B83B7, false),
        ("read_sit_loop_south", 0, 40, 37, 0xFCD9B3, true),
        ("read_sit_loop_south", 0, 39, 43, 0x424E88, false),
        ("read_sit_loop_south", 0, 40, 47, 0x98A2A6, false),
        ("read_sit_loop_south", 0, 38, 51, 0x686589, false),
        ("read_sit_loop_south", 1, 40, 31, 0x9B83B7, false),
        ("read_sit_loop_south", 1, 40, 37, 0xFCD9B3, true),
        ("read_sit_loop_south", 1, 39, 43, 0x5C72D8, false),
        ("read_sit_loop_south", 1, 40, 47, 0xC9AF9C, false),
        ("read_sit_loop_south", 1, 38, 51, 0x000000, false),
        ("read_sit_loop_south", 2, 40, 31, 0x9B83B7, false),
        ("read_sit_loop_south", 2, 40, 37, 0x000000, false),
        ("read_sit_loop_south", 2, 39, 43, 0x424E88, false),
        ("read_sit_loop_south", 2, 40, 47, 0x606C76, false),
        ("read_sit_loop_south", 2, 38, 51, 0x686589, false),
        ("read_sit_loop_south", 3, 40, 31, 0x9B83B7, false),
        ("read_sit_loop_south", 3, 40, 37, 0xFCD9B3, true),
        ("read_sit_loop_south", 3, 39, 43, 0x5C72D8, false),
        ("read_sit_loop_south", 3, 40, 47, 0xC9AF9C, false),
        ("read_sit_loop_south", 3, 38, 51, 0x000000, false),
        ("read_sit_end_south", 0, 40, 31, 0x9B83B7, false),
        ("read_sit_end_south", 0, 40, 37, 0xFCD9B3, true),
        ("read_sit_end_south", 0, 39, 43, 0xF6E4D7, false),
        ("read_sit_end_south", 0, 40, 47, 0x98A2A6, false),
        ("read_sit_end_south", 0, 38, 51, 0x686589, false),
        ("read_sit_end_south", 1, 40, 31, 0x9B83B7, false),
        ("read_sit_end_south", 1, 40, 37, 0xFCD9B3, true),
        ("read_sit_end_south", 1, 39, 43, 0xF6E4D7, false),
        ("read_sit_end_south", 1, 40, 47, 0xC9AF9C, false),
        ("read_sit_end_south", 1, 38, 51, 0x000000, false),
        ("read_sit_end_south", 2, 40, 31, 0x9B83B7, false),
        ("read_sit_end_south", 2, 40, 37, 0xFCD9B3, true),
        ("read_sit_end_south", 2, 39, 43, 0x424E88, false),
        ("read_sit_end_south", 2, 40, 47, 0x461839, false),
        ("read_sit_end_south", 2, 38, 51, 0x686589, false),
        ("shocked_loop_south", 0, 31, 35, 0xFCD9B3, true),
        ("shocked_loop_south", 0, 32, 36, 0x525A62, false),
        ("shocked_loop_south", 0, 33, 37, 0xF0B988, true),
        ("shocked_loop_south", 0, 47, 35, 0xFCD9B3, true),
        ("shocked_loop_south", 0, 48, 36, 0x374050, false),
        ("shocked_loop_south", 0, 40, 37, 0x9E2626, false),
        ("shocked_loop_south", 0, 39, 38, 0xD37A57, true),
        ("shocked_loop_south", 0, 40, 39, 0x672115, true),
        ("shocked_loop_south", 0, 37, 47, 0xA56B81, false),
        ("read_sit_start_south", 1, 35, 46, 0xFCD9B3, true),
        ("read_sit_start_south", 1, 35, 47, 0x672115, true),
        ("read_sit_start_south", 1, 44, 46, 0xFCD9B3, true),
        ("read_sit_start_south", 1, 44, 47, 0x672115, true),
        ("read_sit_start_south", 1, 37, 45, 0x606C76, false),
        ("read_sit_start_south", 1, 38, 44, 0xC9AF9C, false),
        ("read_sit_start_south", 1, 39, 44, 0xF6E4D7, false),
        ("read_sit_start_south", 1, 40, 44, 0xF6E4D7, false),
        ("read_sit_start_south", 1, 41, 44, 0xC9AF9C, false),
        ("read_sit_start_south", 1, 42, 44, 0x606C76, false),
        ("read_sit_start_south", 2, 34, 47, 0xFCD9B3, true),
        ("read_sit_start_south", 2, 35, 47, 0x672115, true),
        ("read_sit_start_south", 2, 36, 47, 0x000000, false),
        ("read_sit_start_south", 2, 44, 47, 0x672115, true),
        ("read_sit_start_south", 2, 39, 44, 0xC9AF9C, false),
        ("read_sit_loop_south", 0, 32, 42, 0xF6E4D7, false),
        ("read_sit_loop_south", 0, 34, 42, 0xC9AF9C, false),
        ("read_sit_loop_south", 0, 39, 45, 0xF6E4D7, false),
        ("read_sit_loop_south", 0, 40, 45, 0xF6E4D7, false),
        ("read_sit_loop_south", 0, 45, 44, 0xF6E4D7, false),
        ("read_sit_loop_south", 0, 46, 46, 0x98A2A6, false),
        ("read_sit_loop_south", 1, 33, 44, 0xF6E4D7, false),
        ("read_sit_loop_south", 1, 36, 45, 0xC9AF9C, false),
        ("read_sit_loop_south", 1, 37, 46, 0x000000, false),
        ("read_sit_loop_south", 1, 40, 46, 0xF6E4D7, false),
        ("read_sit_loop_south", 1, 40, 47, 0xC9AF9C, false),
        ("read_sit_loop_south", 1, 40, 48, 0x606C76, false),
        ("read_sit_loop_south", 1, 40, 49, 0x98A2A6, false),
        ("read_sit_loop_south", 2, 48, 42, 0x000000, false),
        ("read_sit_end_south", 0, 34, 47, 0xFCD9B3, true),
        ("read_sit_end_south", 0, 35, 47, 0x672115, true),
        ("read_sit_end_south", 0, 39, 44, 0xC9AF9C, false),
        ("read_sit_end_south", 1, 35, 46, 0xFCD9B3, true),
        ("read_sit_end_south", 1, 35, 47, 0x672115, true),
        ("read_sit_end_south", 1, 39, 44, 0xF6E4D7, false),
        ("read_sit_end_south", 1, 42, 44, 0x606C76, false),
        ("shocked_loop_south", 0, 36, 45, 0x612026, false),
    ];
    let cases: [(&str, &[usize]); 6] = [
        ("shocked_start_south", &[47]),
        ("shocked_loop_south", &[49]),
        ("shocked_end_south", &[47]),
        ("read_sit_start_south", &[31, 27, 29]),
        ("read_sit_loop_south", &[23, 33, 23, 33]),
        ("read_sit_end_south", &[29, 27, 31]),
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
            let prefix = if case.starts_with("read_sit_") {
                "spr_npc_balor_specialanimation_spring"
            } else {
                "spr_npc_balor_spring"
            };
            let asset = format!("assets/animations/NPCs/Balor/Sprites/Spring/{prefix}_{case}.png");
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
            let mut trouser_shadows = 0;
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                if p.0 == rgba(0x612026) {
                    assert_eq!(p, q, "trouser shadow changed: {id} {case} [{x},{y}]");
                    trouser_shadows += 1;
                }
                // All four reviewed world shades are skin in these Spring strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Balor Spring material mismatch: {id} {case} [{x},{y}]"
                );
                mask.push(p != q);
                if p != q {
                    let index = source.iter().position(|c| rgba(*c) == p.0).expect(
                        "hair, scarf, shirt, bag, trousers, eyes or another non-skin color changed",
                    );
                    assert_eq!(q.0, rgba(target[index]));
                    per_frame[x as usize / 80] += 1;
                }
            }
            assert_eq!(
                trouser_shadows,
                match case {
                    "shocked_start_south"
                    | "shocked_end_south"
                    | "read_sit_start_south"
                    | "read_sit_end_south" => 5,
                    "shocked_loop_south" => 6,
                    "read_sit_loop_south" => 0,
                    _ => unreachable!(),
                }
            );
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
