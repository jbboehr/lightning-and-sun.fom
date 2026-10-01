use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/balor and the local accepted Balor world baseline"]
fn balor_spring_standard_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/balor");
    let baseline =
        root.join("generated/characters-balor-valen-eiland-spring-actions-trial/characters/balor");
    let set = std::env::var_os("FOM_BALOR_SPRING_STANDARD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/balor-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..127];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..127], prior);
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
    // Reviewed literal landmarks cover moving hands/cuffs, two-pixel North
    // hands, kiss and sleep faces, hair, scarf, bag, trousers and boots.
    // The four world shades are exclusively skin in these five strips.
    let landmarks = [
        ("action_east", 0, 40, 31, 0x9B83B7, false),
        ("action_east", 0, 42, 36, 0xFCD9B3, true),
        ("action_east", 0, 39, 43, 0x525A62, false),
        ("action_east", 0, 40, 47, 0x672115, true),
        ("action_east", 0, 38, 52, 0x4A3D66, false),
        ("action_east", 1, 40, 31, 0x686589, false),
        ("action_east", 1, 42, 36, 0x000000, false),
        ("action_east", 1, 39, 43, 0x2E2E64, false),
        ("action_east", 1, 40, 47, 0x612026, false),
        ("action_east", 1, 38, 52, 0x4A3D66, false),
        ("action_east", 2, 40, 31, 0x686589, false),
        ("action_east", 2, 42, 36, 0x000000, false),
        ("action_east", 2, 39, 43, 0x2E2E64, false),
        ("action_east", 2, 40, 47, 0x612026, false),
        ("action_east", 2, 38, 52, 0x4A3D66, false),
        ("action_east", 3, 40, 31, 0x686589, false),
        ("action_east", 3, 42, 36, 0x000000, false),
        ("action_east", 3, 39, 43, 0x2E2E64, false),
        ("action_east", 3, 40, 47, 0x612026, false),
        ("action_east", 3, 38, 52, 0x4A3D66, false),
        ("action_east", 4, 40, 31, 0x686589, false),
        ("action_east", 4, 42, 36, 0x000000, false),
        ("action_east", 4, 39, 43, 0x2E2E64, false),
        ("action_east", 4, 40, 47, 0x612026, false),
        ("action_east", 4, 38, 52, 0x4A3D66, false),
        ("action_east", 5, 40, 31, 0x9B83B7, false),
        ("action_east", 5, 42, 36, 0xFCD9B3, true),
        ("action_east", 5, 39, 43, 0x525A62, false),
        ("action_east", 5, 40, 47, 0x672115, true),
        ("action_east", 5, 38, 52, 0x4A3D66, false),
        ("action_east", 6, 40, 31, 0x9B83B7, false),
        ("action_east", 6, 42, 36, 0x000000, false),
        ("action_east", 6, 39, 43, 0xA59DA2, false),
        ("action_east", 6, 40, 47, 0xA56B81, false),
        ("action_east", 6, 38, 52, 0x4A3D66, false),
        ("action_north", 0, 40, 31, 0x9B83B7, false),
        ("action_north", 0, 42, 36, 0x686589, false),
        ("action_north", 0, 39, 43, 0x424E88, false),
        ("action_north", 0, 40, 47, 0x525A62, false),
        ("action_north", 0, 38, 52, 0x686589, false),
        ("action_north", 1, 40, 31, 0x9B83B7, false),
        ("action_north", 1, 42, 36, 0x4A3D66, false),
        ("action_north", 1, 39, 43, 0x374050, false),
        ("action_north", 1, 40, 47, 0x374050, false),
        ("action_north", 1, 38, 52, 0x686589, false),
        ("action_north", 2, 40, 31, 0x9B83B7, false),
        ("action_north", 2, 42, 36, 0x4A3D66, false),
        ("action_north", 2, 39, 43, 0x374050, false),
        ("action_north", 2, 40, 47, 0x374050, false),
        ("action_north", 2, 38, 52, 0x686589, false),
        ("action_north", 3, 40, 31, 0x9B83B7, false),
        ("action_north", 3, 42, 36, 0x4A3D66, false),
        ("action_north", 3, 39, 43, 0x374050, false),
        ("action_north", 3, 40, 47, 0x374050, false),
        ("action_north", 3, 38, 52, 0x686589, false),
        ("action_north", 4, 40, 31, 0x9B83B7, false),
        ("action_north", 4, 42, 36, 0x4A3D66, false),
        ("action_north", 4, 39, 43, 0x374050, false),
        ("action_north", 4, 40, 47, 0x374050, false),
        ("action_north", 4, 38, 52, 0x686589, false),
        ("action_north", 5, 40, 31, 0x9B83B7, false),
        ("action_north", 5, 42, 36, 0x686589, false),
        ("action_north", 5, 39, 43, 0x424E88, false),
        ("action_north", 5, 40, 47, 0x525A62, false),
        ("action_north", 5, 38, 52, 0x686589, false),
        ("action_north", 6, 40, 31, 0x9B83B7, false),
        ("action_north", 6, 42, 36, 0x686589, false),
        ("action_north", 6, 39, 43, 0x424E88, false),
        ("action_north", 6, 40, 47, 0x525A62, false),
        ("action_north", 6, 38, 52, 0x686589, false),
        ("action_south", 0, 40, 31, 0x9B83B7, false),
        ("action_south", 0, 42, 36, 0xC2B9BE, false),
        ("action_south", 0, 39, 43, 0x424E88, false),
        ("action_south", 0, 40, 47, 0xECF0E9, false),
        ("action_south", 0, 38, 52, 0x4A3D66, false),
        ("action_south", 1, 40, 31, 0x686589, false),
        ("action_south", 1, 42, 36, 0xECF0E9, false),
        ("action_south", 1, 39, 43, 0x2E2E64, false),
        ("action_south", 1, 40, 47, 0xA56B81, false),
        ("action_south", 1, 38, 52, 0x4A3D66, false),
        ("action_south", 2, 40, 31, 0x686589, false),
        ("action_south", 2, 42, 36, 0xECF0E9, false),
        ("action_south", 2, 39, 43, 0x2E2E64, false),
        ("action_south", 2, 40, 47, 0xA56B81, false),
        ("action_south", 2, 38, 52, 0x4A3D66, false),
        ("action_south", 3, 40, 31, 0x686589, false),
        ("action_south", 3, 42, 36, 0xECF0E9, false),
        ("action_south", 3, 39, 43, 0x2E2E64, false),
        ("action_south", 3, 40, 47, 0xA56B81, false),
        ("action_south", 3, 38, 52, 0x4A3D66, false),
        ("action_south", 4, 40, 31, 0x686589, false),
        ("action_south", 4, 42, 36, 0xECF0E9, false),
        ("action_south", 4, 39, 43, 0x2E2E64, false),
        ("action_south", 4, 40, 47, 0xA56B81, false),
        ("action_south", 4, 38, 52, 0x4A3D66, false),
        ("action_south", 5, 40, 31, 0x9B83B7, false),
        ("action_south", 5, 42, 36, 0xC2B9BE, false),
        ("action_south", 5, 39, 43, 0x424E88, false),
        ("action_south", 5, 40, 47, 0xECF0E9, false),
        ("action_south", 5, 38, 52, 0x4A3D66, false),
        ("action_south", 6, 40, 31, 0x686589, false),
        ("action_south", 6, 42, 36, 0xECF0E9, false),
        ("action_south", 6, 39, 43, 0x2E2E64, false),
        ("action_south", 6, 40, 47, 0xA56B81, false),
        ("action_south", 6, 38, 52, 0x4A3D66, false),
        ("kiss_east", 0, 40, 31, 0x9B83B7, false),
        ("kiss_east", 0, 42, 36, 0xA59DA2, false),
        ("kiss_east", 0, 39, 43, 0x424E88, false),
        ("kiss_east", 0, 40, 47, 0xECF0E9, false),
        ("kiss_east", 0, 38, 52, 0x4A3D66, false),
        ("kiss_east", 1, 40, 31, 0x9B83B7, false),
        ("kiss_east", 1, 42, 36, 0xFCD9B3, true),
        ("kiss_east", 1, 39, 43, 0x2E2E64, false),
        ("kiss_east", 1, 40, 47, 0xECF0E9, false),
        ("kiss_east", 1, 38, 52, 0x4A3D66, false),
        ("kiss_east", 2, 40, 31, 0x686589, false),
        ("kiss_east", 2, 42, 36, 0xFCD9B3, true),
        ("kiss_east", 2, 39, 43, 0x766871, false),
        ("kiss_east", 2, 40, 47, 0xA56B81, false),
        ("kiss_east", 2, 38, 52, 0x000000, false),
        ("kiss_east", 3, 40, 31, 0x9B83B7, false),
        ("kiss_east", 3, 42, 36, 0xFCD9B3, true),
        ("kiss_east", 3, 39, 43, 0x2E2E64, false),
        ("kiss_east", 3, 40, 47, 0xECF0E9, false),
        ("kiss_east", 3, 38, 52, 0x4A3D66, false),
        ("sleep_east", 0, 40, 31, 0x9B83B7, false),
        ("sleep_east", 0, 42, 36, 0x000000, false),
        ("sleep_east", 0, 39, 43, 0xA59DA2, false),
        ("sleep_east", 0, 40, 47, 0xA56B81, false),
        ("sleep_east", 0, 38, 52, 0x4A3D66, false),
        ("action_east", 1, 48, 42, 0xFCD9B3, true),
        ("action_east", 1, 49, 43, 0xFCD9B3, true),
        ("action_east", 1, 48, 44, 0x672115, true),
        ("action_east", 1, 47, 42, 0x525A62, false),
        ("action_east", 1, 45, 43, 0x766871, false),
        ("action_east", 1, 39, 46, 0x612026, false),
        ("action_east", 1, 40, 47, 0x612026, false),
        ("action_east", 2, 45, 43, 0xFCD9B3, true),
        ("action_east", 2, 46, 44, 0xFCD9B3, true),
        ("action_east", 2, 45, 45, 0x672115, true),
        ("action_north", 0, 32, 43, 0xFCD9B3, true),
        ("action_north", 0, 33, 44, 0x672115, true),
        ("action_north", 1, 34, 44, 0xD37A57, true),
        ("action_north", 1, 34, 45, 0xF0B988, true),
        ("action_north", 2, 34, 44, 0xD37A57, true),
        ("action_north", 2, 34, 45, 0xF0B988, true),
        ("action_north", 3, 34, 44, 0xD37A57, true),
        ("action_north", 3, 34, 45, 0xF0B988, true),
        ("action_north", 4, 34, 44, 0xD37A57, true),
        ("action_north", 4, 34, 45, 0xF0B988, true),
        ("action_north", 5, 32, 44, 0x000000, false),
        ("action_north", 6, 32, 45, 0xFCD9B3, true),
        ("kiss_east", 0, 36, 47, 0xFCD9B3, true),
        ("kiss_east", 0, 42, 47, 0x612026, false),
        ("kiss_east", 1, 41, 40, 0xD37A57, true),
        ("kiss_east", 2, 41, 35, 0x000000, false),
        ("kiss_east", 2, 44, 35, 0xFCD9B3, true),
        ("kiss_east", 2, 42, 36, 0xFCD9B3, true),
        ("kiss_east", 2, 44, 36, 0xFCD9B3, true),
        ("kiss_east", 2, 42, 37, 0xF0B988, true),
        ("kiss_east", 2, 43, 39, 0xD37A57, true),
        ("kiss_east", 2, 44, 39, 0xD37A57, true),
        ("kiss_east", 2, 35, 44, 0xFCD9B3, true),
        ("kiss_east", 2, 36, 45, 0x672115, true),
        ("kiss_east", 2, 44, 46, 0x612026, false),
        ("kiss_east", 3, 40, 37, 0xFCD9B3, true),
        ("sleep_east", 0, 40, 36, 0xFCD9B3, true),
        ("sleep_east", 0, 42, 36, 0x000000, false),
        ("sleep_east", 0, 43, 38, 0xF0B988, true),
        ("sleep_east", 0, 44, 38, 0x672115, true),
        ("sleep_east", 0, 45, 38, 0xF0B988, true),
        ("sleep_east", 0, 44, 39, 0xFCD9B3, true),
        ("sleep_east", 0, 45, 39, 0xFCD9B3, true),
        ("sleep_east", 0, 42, 40, 0xFCD9B3, true),
        ("sleep_east", 0, 43, 40, 0x525A62, false),
        ("sleep_east", 0, 39, 39, 0xB3AFBD, false),
        ("sleep_east", 0, 37, 46, 0x8F4A54, false),
    ];
    let cases: [(&str, &[usize]); 5] = [
        ("action_east", &[31, 30, 31, 30, 31, 31, 33]),
        ("action_north", &[10, 2, 4, 2, 4, 9, 12]),
        ("action_south", &[38, 36, 37, 36, 37, 38, 41]),
        ("kiss_east", &[32, 34, 37, 38]),
        ("sleep_east", &[29]),
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
            let prefix = "spr_npc_balor_spring";
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
                    "action_east" => 34,
                    "action_north" => 0,
                    "action_south" => 23,
                    "kiss_east" => 10,
                    "sleep_east" => 7,
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
