use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/balor-spring-standard-study and the local accepted Balor world baseline"]
fn balor_spring_actions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-spring-standard-study");
    let baseline =
        root.join("generated/characters-balor-valen-eiland-world-trial/characters/balor");
    let set = std::env::var_os("FOM_BALOR_SPRING_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/balor-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..116];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..116], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 132);
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
    // Fixed landmarks cover all source frames, moving hands/cuffs, eye details,
    // mouth interiors, scarf, bag, trousers and boots. Full-pixel expectations
    // independently select the four reviewed skin shades in this corpus.
    let landmarks = [
        ("blink_east", 0, 39, 31, 0x686589, false),
        ("blink_east", 0, 40, 36, 0xFCD9B3, true),
        ("blink_east", 0, 39, 43, 0xA59DA2, false),
        ("blink_east", 0, 39, 46, 0x8F4A54, false),
        ("blink_east", 0, 38, 49, 0x77405D, false),
        ("blink_east", 1, 39, 31, 0x686589, false),
        ("blink_east", 1, 40, 36, 0xFCD9B3, true),
        ("blink_east", 1, 39, 43, 0xA59DA2, false),
        ("blink_east", 1, 39, 46, 0x8F4A54, false),
        ("blink_east", 1, 38, 49, 0x77405D, false),
        ("blink_east", 2, 39, 31, 0x686589, false),
        ("blink_east", 2, 40, 36, 0xFCD9B3, true),
        ("blink_east", 2, 39, 43, 0xA59DA2, false),
        ("blink_east", 2, 39, 46, 0x8F4A54, false),
        ("blink_east", 2, 38, 49, 0x77405D, false),
        ("blink_south", 0, 39, 31, 0x9B83B7, false),
        ("blink_south", 0, 40, 36, 0xFCD9B3, true),
        ("blink_south", 0, 39, 43, 0x2E2E64, false),
        ("blink_south", 0, 39, 46, 0xECF0E9, false),
        ("blink_south", 0, 38, 49, 0xA56B81, false),
        ("blink_south", 1, 39, 31, 0x9B83B7, false),
        ("blink_south", 1, 40, 36, 0xFCD9B3, true),
        ("blink_south", 1, 39, 43, 0x2E2E64, false),
        ("blink_south", 1, 39, 46, 0xECF0E9, false),
        ("blink_south", 1, 38, 49, 0xA56B81, false),
        ("blink_south", 2, 39, 31, 0x9B83B7, false),
        ("blink_south", 2, 40, 36, 0xFCD9B3, true),
        ("blink_south", 2, 39, 43, 0x2E2E64, false),
        ("blink_south", 2, 39, 46, 0xECF0E9, false),
        ("blink_south", 2, 38, 49, 0xA56B81, false),
        ("drink_east", 0, 39, 31, 0x9B83B7, false),
        ("drink_east", 0, 40, 36, 0xF0B988, true),
        ("drink_east", 0, 39, 43, 0x374050, false),
        ("drink_east", 0, 39, 46, 0xA56B81, false),
        ("drink_east", 1, 39, 31, 0x686589, false),
        ("drink_east", 1, 40, 36, 0xFCD9B3, true),
        ("drink_east", 1, 39, 43, 0x374050, false),
        ("drink_east", 1, 39, 46, 0xA56B81, false),
        ("drink_east", 2, 39, 31, 0x9B83B7, false),
        ("drink_east", 2, 40, 36, 0xF0B988, true),
        ("drink_east", 2, 39, 43, 0x374050, false),
        ("drink_east", 2, 39, 46, 0xA56B81, false),
        ("drink_north", 0, 39, 31, 0x686589, false),
        ("drink_north", 0, 40, 36, 0x686589, false),
        ("drink_north", 0, 39, 43, 0x5C72D8, false),
        ("drink_north", 0, 39, 46, 0x525A62, false),
        ("drink_north", 0, 38, 49, 0x374050, false),
        ("drink_north", 0, 33, 46, 0xF0B988, true),
        ("drink_north", 0, 34, 47, 0x672115, true),
        ("drink_north", 0, 45, 42, 0x000000, false),
        ("drink_north", 1, 39, 31, 0x686589, false),
        ("drink_north", 1, 40, 36, 0x686589, false),
        ("drink_north", 1, 39, 43, 0x2E2E64, false),
        ("drink_north", 1, 39, 46, 0x374050, false),
        ("drink_north", 1, 38, 49, 0x525A62, false),
        ("drink_north", 1, 33, 46, 0xF0B988, true),
        ("drink_north", 1, 34, 47, 0x672115, true),
        ("drink_north", 1, 45, 41, 0xD37A57, true),
        ("drink_north", 2, 39, 31, 0x686589, false),
        ("drink_north", 2, 40, 36, 0x686589, false),
        ("drink_north", 2, 39, 43, 0x5C72D8, false),
        ("drink_north", 2, 39, 46, 0x525A62, false),
        ("drink_north", 2, 38, 49, 0x374050, false),
        ("drink_north", 2, 33, 46, 0xF0B988, true),
        ("drink_north", 2, 34, 47, 0x672115, true),
        ("drink_north", 2, 45, 42, 0x000000, false),
        ("drink_south", 0, 39, 31, 0x9B83B7, false),
        ("drink_south", 0, 40, 36, 0xFCD9B3, true),
        ("drink_south", 0, 39, 43, 0x424E88, false),
        ("drink_south", 0, 39, 46, 0xECF0E9, false),
        ("drink_south", 0, 38, 49, 0x77405D, false),
        ("drink_south", 1, 39, 31, 0x686589, false),
        ("drink_south", 1, 40, 36, 0xF0B988, true),
        ("drink_south", 1, 39, 43, 0x5C72D8, false),
        ("drink_south", 1, 39, 46, 0xA56B81, false),
        ("drink_south", 1, 38, 49, 0x77405D, false),
        ("drink_south", 2, 39, 31, 0x9B83B7, false),
        ("drink_south", 2, 40, 36, 0xFCD9B3, true),
        ("drink_south", 2, 39, 43, 0x424E88, false),
        ("drink_south", 2, 39, 46, 0xECF0E9, false),
        ("drink_south", 2, 38, 49, 0x77405D, false),
        ("eat_east", 0, 39, 31, 0x9B83B7, false),
        ("eat_east", 0, 40, 36, 0xF0B988, true),
        ("eat_east", 0, 39, 43, 0x000000, false),
        ("eat_east", 0, 39, 46, 0xA56B81, false),
        ("eat_east", 1, 39, 31, 0x9B83B7, false),
        ("eat_east", 1, 40, 36, 0x000000, false),
        ("eat_east", 1, 39, 43, 0x374050, false),
        ("eat_east", 1, 39, 46, 0xA56B81, false),
        ("eat_east", 2, 39, 31, 0x686589, false),
        ("eat_east", 2, 40, 36, 0x9E2626, false),
        ("eat_east", 2, 39, 43, 0x000000, false),
        ("eat_east", 2, 39, 46, 0xA56B81, false),
        ("eat_east", 3, 39, 31, 0x9B83B7, false),
        ("eat_east", 3, 40, 36, 0x4A3D66, false),
        ("eat_east", 3, 39, 43, 0x525A62, false),
        ("eat_east", 3, 39, 46, 0xA56B81, false),
        ("eat_east", 4, 39, 31, 0x9B83B7, false),
        ("eat_east", 4, 40, 36, 0xF0B988, true),
        ("eat_east", 4, 39, 43, 0xFCD9B3, true),
        ("eat_east", 4, 39, 46, 0xA56B81, false),
        ("eat_north", 0, 39, 31, 0x686589, false),
        ("eat_north", 0, 40, 36, 0x686589, false),
        ("eat_north", 0, 39, 43, 0x5C72D8, false),
        ("eat_north", 0, 39, 46, 0x525A62, false),
        ("eat_north", 0, 38, 49, 0x374050, false),
        ("eat_north", 0, 33, 46, 0xF0B988, true),
        ("eat_north", 0, 34, 47, 0x672115, true),
        ("eat_north", 0, 45, 42, 0x000000, false),
        ("eat_north", 1, 39, 31, 0x686589, false),
        ("eat_north", 1, 40, 36, 0x686589, false),
        ("eat_north", 1, 39, 43, 0x2E2E64, false),
        ("eat_north", 1, 39, 46, 0x374050, false),
        ("eat_north", 1, 38, 49, 0x525A62, false),
        ("eat_north", 1, 33, 46, 0xF0B988, true),
        ("eat_north", 1, 34, 47, 0x672115, true),
        ("eat_north", 1, 45, 41, 0xD37A57, true),
        ("eat_north", 2, 39, 31, 0x686589, false),
        ("eat_north", 2, 40, 36, 0x686589, false),
        ("eat_north", 2, 39, 43, 0x5C72D8, false),
        ("eat_north", 2, 39, 46, 0x525A62, false),
        ("eat_north", 2, 38, 49, 0x374050, false),
        ("eat_north", 2, 33, 46, 0xF0B988, true),
        ("eat_north", 2, 34, 47, 0x672115, true),
        ("eat_north", 2, 45, 42, 0x000000, false),
        ("eat_south", 0, 39, 31, 0x9B83B7, false),
        ("eat_south", 0, 40, 36, 0xFCD9B3, true),
        ("eat_south", 0, 39, 43, 0x424E88, false),
        ("eat_south", 0, 39, 46, 0xECF0E9, false),
        ("eat_south", 0, 38, 49, 0x77405D, false),
        ("eat_south", 1, 39, 31, 0x686589, false),
        ("eat_south", 1, 40, 36, 0xF0B988, true),
        ("eat_south", 1, 39, 43, 0x5C72D8, false),
        ("eat_south", 1, 39, 46, 0xA56B81, false),
        ("eat_south", 1, 38, 49, 0xF0B988, true),
        ("eat_south", 2, 39, 31, 0x686589, false),
        ("eat_south", 2, 40, 36, 0x9E2626, false),
        ("eat_south", 2, 39, 43, 0x424E88, false),
        ("eat_south", 2, 39, 46, 0xECF0E9, false),
        ("eat_south", 2, 38, 49, 0x77405D, false),
        ("eat_south", 3, 39, 31, 0x686589, false),
        ("eat_south", 3, 40, 36, 0xF0B988, true),
        ("eat_south", 3, 39, 43, 0x5C72D8, false),
        ("eat_south", 3, 39, 46, 0xA56B81, false),
        ("eat_south", 3, 38, 49, 0x77405D, false),
        ("eat_south", 4, 39, 31, 0x9B83B7, false),
        ("eat_south", 4, 40, 36, 0xFCD9B3, true),
        ("eat_south", 4, 39, 43, 0x424E88, false),
        ("eat_south", 4, 39, 46, 0xECF0E9, false),
        ("eat_south", 4, 38, 49, 0x77405D, false),
        ("sit_east", 0, 39, 31, 0x9B83B7, false),
        ("sit_east", 0, 40, 36, 0xF0B988, true),
        ("sit_east", 0, 39, 43, 0x2E2E64, false),
        ("sit_east", 0, 39, 46, 0xA56B81, false),
        ("sit_north", 0, 39, 31, 0x686589, false),
        ("sit_north", 0, 40, 36, 0x686589, false),
        ("sit_north", 0, 39, 43, 0x5C72D8, false),
        ("sit_north", 0, 39, 46, 0x525A62, false),
        ("sit_north", 0, 38, 49, 0x374050, false),
        ("sit_north", 0, 33, 46, 0xF0B988, true),
        ("sit_north", 0, 34, 47, 0x672115, true),
        ("sit_north", 0, 45, 42, 0x000000, false),
        ("sit_south", 0, 39, 31, 0x9B83B7, false),
        ("sit_south", 0, 40, 36, 0xFCD9B3, true),
        ("sit_south", 0, 39, 43, 0x424E88, false),
        ("sit_south", 0, 39, 46, 0xECF0E9, false),
        ("sit_south", 0, 38, 49, 0x77405D, false),
        ("drink_east", 0, 40, 42, 0xFCD9B3, true),
        ("drink_east", 0, 39, 44, 0xFCD9B3, true),
        ("drink_east", 0, 39, 43, 0x374050, false),
        ("drink_east", 0, 41, 44, 0x374050, false),
        ("drink_east", 1, 39, 40, 0xFCD9B3, true),
        ("drink_east", 1, 40, 42, 0xD37A57, true),
        ("drink_east", 1, 38, 43, 0xF0B988, true),
        ("drink_east", 1, 37, 43, 0xB3AFBD, false),
        ("drink_east", 1, 38, 45, 0x612026, false),
        ("eat_east", 2, 41, 35, 0x410808, false),
        ("eat_east", 2, 41, 36, 0x9E2626, false),
        ("eat_east", 2, 42, 37, 0x9E2626, false),
        ("eat_east", 2, 44, 39, 0xFCD9B3, true),
        ("eat_east", 2, 44, 40, 0xF0B988, true),
        ("eat_east", 2, 39, 41, 0x6F8396, false),
        ("eat_east", 2, 43, 41, 0x525A62, false),
        ("eat_east", 2, 42, 42, 0xF0B988, true),
        ("eat_east", 2, 38, 45, 0x612026, false),
        ("eat_south", 1, 40, 40, 0x9E2626, false),
        ("eat_south", 2, 38, 35, 0x410808, false),
        ("eat_south", 2, 40, 37, 0x9E2626, false),
        ("eat_south", 2, 36, 40, 0x672115, true),
        ("eat_south", 2, 37, 40, 0xFCD9B3, true),
        ("eat_south", 2, 37, 41, 0xFCD9B3, true),
        ("eat_south", 2, 37, 42, 0xF0B988, true),
        ("eat_south", 2, 40, 41, 0x672115, true),
        ("eat_south", 2, 43, 41, 0x374050, false),
        ("eat_south", 2, 36, 46, 0x612026, false),
        ("eat_south", 2, 39, 46, 0xECF0E9, false),
        ("eat_south", 3, 36, 43, 0xFCD9B3, true),
        ("eat_south", 3, 37, 43, 0xD37A57, true),
        ("eat_south", 3, 35, 44, 0xFCD9B3, true),
        ("eat_south", 3, 43, 46, 0x000000, false),
        ("sit_east", 0, 38, 45, 0x612026, false),
        ("sit_east", 0, 35, 46, 0xFCD9B3, true),
        ("sit_east", 0, 36, 47, 0xF0B988, true),
        ("sit_east", 0, 42, 42, 0x424E88, false),
        ("blink_south", 1, 37, 36, 0x000000, false),
        ("blink_south", 1, 39, 37, 0xFCD9B3, true),
        ("blink_south", 1, 36, 46, 0x612026, false),
        ("drink_north", 0, 46, 42, 0xD37A57, true),
        ("drink_north", 2, 46, 42, 0xD37A57, true),
        ("eat_north", 0, 46, 42, 0xD37A57, true),
        ("eat_north", 2, 46, 42, 0xD37A57, true),
    ];
    let cases: [(&str, &[usize]); 11] = [
        ("blink_east", &[35, 39, 35]),
        ("blink_south", &[43, 47, 43]),
        ("drink_east", &[28, 36, 28]),
        ("drink_north", &[7, 5, 7]),
        ("drink_south", &[37, 42, 37]),
        ("eat_east", &[27, 29, 26, 29, 32]),
        ("eat_north", &[7, 5, 7]),
        ("eat_south", &[37, 40, 36, 46, 33]),
        ("sit_east", &[28]),
        ("sit_north", &[6]),
        ("sit_south", &[33]),
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
                    "blink_east" => 12,
                    "blink_south" => 15,
                    "drink_east" | "sit_east" => 1,
                    "drink_north" | "eat_north" | "sit_north" => 0,
                    "drink_south" => 17,
                    "eat_east" | "sit_south" => 5,
                    "eat_south" => 16,
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
