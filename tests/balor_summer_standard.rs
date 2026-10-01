use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/balor-winter-standard-study and the local accepted Balor world baseline"]
fn balor_summer_standard_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-winter-standard-study");
    let baseline = root.join(
        "generated/characters-balor-valen-summer-eiland-spring-finish-trial/characters/balor",
    );
    let set = std::env::var_os("FOM_BALOR_SUMMER_STANDARD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/balor-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..161];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..161], prior);
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
    // Literal source landmarks cover every Summer action, sleep and kiss
    // frame, moving hands, wrist details, shirt, hair, belt/shoes and jaw edges.
    // All four reviewed world shades are skin in these five strips.
    let landmarks = [
        ("action_north", 0, 35, 42, 0xFCD9B3, true),
        ("action_north", 0, 46, 40, 0xD37A57, true),
        ("action_north", 0, 45, 40, 0x672115, true),
        ("action_north", 0, 39, 28, 0x686589, false),
        ("action_north", 0, 42, 45, 0x612026, false),
        ("action_north", 0, 35, 41, 0xB395D6, false),
        ("action_north", 0, 36, 41, 0xDEDAE9, false),
        ("action_north", 0, 46, 41, 0x374050, false),
        ("action_north", 1, 36, 43, 0xFCD9B3, true),
        ("action_north", 1, 35, 45, 0x672115, true),
        ("action_north", 1, 39, 27, 0x686589, false),
        ("action_north", 1, 42, 44, 0x612026, false),
        ("action_north", 1, 36, 40, 0xB395D6, false),
        ("action_north", 1, 43, 39, 0xDEDAE9, false),
        ("action_north", 1, 36, 44, 0x374050, false),
        ("action_north", 2, 36, 43, 0xFCD9B3, true),
        ("action_north", 2, 45, 38, 0xD37A57, true),
        ("action_north", 2, 35, 45, 0x672115, true),
        ("action_north", 2, 39, 27, 0x686589, false),
        ("action_north", 2, 42, 44, 0x612026, false),
        ("action_north", 2, 43, 39, 0xB395D6, false),
        ("action_north", 2, 37, 40, 0xDEDAE9, false),
        ("action_north", 2, 36, 44, 0x374050, false),
        ("action_north", 3, 36, 43, 0xFCD9B3, true),
        ("action_north", 3, 35, 45, 0x672115, true),
        ("action_north", 3, 39, 27, 0x686589, false),
        ("action_north", 3, 42, 44, 0x612026, false),
        ("action_north", 3, 36, 40, 0xB395D6, false),
        ("action_north", 3, 43, 39, 0xDEDAE9, false),
        ("action_north", 3, 36, 44, 0x374050, false),
        ("action_north", 4, 36, 43, 0xFCD9B3, true),
        ("action_north", 4, 45, 38, 0xD37A57, true),
        ("action_north", 4, 35, 45, 0x672115, true),
        ("action_north", 4, 39, 27, 0x686589, false),
        ("action_north", 4, 42, 44, 0x612026, false),
        ("action_north", 4, 43, 39, 0xB395D6, false),
        ("action_north", 4, 37, 40, 0xDEDAE9, false),
        ("action_north", 4, 36, 44, 0x374050, false),
        ("action_north", 5, 45, 42, 0xFCD9B3, true),
        ("action_north", 5, 46, 40, 0xD37A57, true),
        ("action_north", 5, 45, 40, 0x672115, true),
        ("action_north", 5, 39, 28, 0x686589, false),
        ("action_north", 5, 42, 45, 0x612026, false),
        ("action_north", 5, 43, 40, 0xB395D6, false),
        ("action_north", 5, 37, 41, 0xDEDAE9, false),
        ("action_north", 5, 46, 41, 0x374050, false),
        ("action_north", 6, 34, 44, 0xFCD9B3, true),
        ("action_north", 6, 34, 43, 0xD37A57, true),
        ("action_north", 6, 33, 46, 0x672115, true),
        ("action_north", 6, 39, 28, 0x686589, false),
        ("action_north", 6, 42, 45, 0x612026, false),
        ("action_north", 6, 36, 41, 0xB395D6, false),
        ("action_north", 6, 37, 41, 0xDEDAE9, false),
        ("action_north", 6, 33, 44, 0x374050, false),
        ("action_south", 0, 40, 36, 0xFCD9B3, true),
        ("action_south", 0, 35, 37, 0xD37A57, true),
        ("action_south", 0, 36, 38, 0x672115, true),
        ("action_south", 0, 39, 29, 0x686589, false),
        ("action_south", 0, 37, 46, 0x612026, false),
        ("action_south", 0, 37, 41, 0xB395D6, false),
        ("action_south", 0, 37, 42, 0xDEDAE9, false),
        ("action_south", 0, 45, 44, 0x374050, false),
        ("action_south", 1, 40, 35, 0xFCD9B3, true),
        ("action_south", 1, 35, 36, 0xD37A57, true),
        ("action_south", 1, 36, 37, 0x672115, true),
        ("action_south", 1, 39, 28, 0x686589, false),
        ("action_south", 1, 37, 47, 0x612026, false),
        ("action_south", 1, 37, 40, 0xB395D6, false),
        ("action_south", 1, 37, 41, 0xDEDAE9, false),
        ("action_south", 1, 44, 43, 0x374050, false),
        ("action_south", 2, 40, 35, 0xFCD9B3, true),
        ("action_south", 2, 35, 36, 0xD37A57, true),
        ("action_south", 2, 36, 37, 0x672115, true),
        ("action_south", 2, 39, 28, 0x686589, false),
        ("action_south", 2, 36, 47, 0x612026, false),
        ("action_south", 2, 37, 40, 0xB395D6, false),
        ("action_south", 2, 37, 41, 0xDEDAE9, false),
        ("action_south", 2, 44, 43, 0x374050, false),
        ("action_south", 3, 40, 35, 0xFCD9B3, true),
        ("action_south", 3, 35, 36, 0xD37A57, true),
        ("action_south", 3, 36, 37, 0x672115, true),
        ("action_south", 3, 39, 28, 0x686589, false),
        ("action_south", 3, 37, 47, 0x612026, false),
        ("action_south", 3, 37, 40, 0xB395D6, false),
        ("action_south", 3, 37, 41, 0xDEDAE9, false),
        ("action_south", 3, 44, 43, 0x374050, false),
        ("action_south", 4, 40, 35, 0xFCD9B3, true),
        ("action_south", 4, 35, 36, 0xD37A57, true),
        ("action_south", 4, 36, 37, 0x672115, true),
        ("action_south", 4, 39, 28, 0x686589, false),
        ("action_south", 4, 36, 47, 0x612026, false),
        ("action_south", 4, 37, 40, 0xB395D6, false),
        ("action_south", 4, 37, 41, 0xDEDAE9, false),
        ("action_south", 4, 44, 43, 0x374050, false),
        ("action_south", 5, 40, 36, 0xFCD9B3, true),
        ("action_south", 5, 35, 37, 0xD37A57, true),
        ("action_south", 5, 36, 38, 0x672115, true),
        ("action_south", 5, 39, 29, 0x686589, false),
        ("action_south", 5, 37, 46, 0x612026, false),
        ("action_south", 5, 37, 41, 0xB395D6, false),
        ("action_south", 5, 37, 42, 0xDEDAE9, false),
        ("action_south", 5, 45, 44, 0x374050, false),
        ("action_south", 6, 40, 35, 0xFCD9B3, true),
        ("action_south", 6, 35, 36, 0xD37A57, true),
        ("action_south", 6, 36, 37, 0x672115, true),
        ("action_south", 6, 39, 28, 0x686589, false),
        ("action_south", 6, 37, 45, 0x612026, false),
        ("action_south", 6, 37, 40, 0xB395D6, false),
        ("action_south", 6, 37, 41, 0xDEDAE9, false),
        ("action_south", 6, 33, 44, 0x374050, false),
        ("action_east", 0, 42, 36, 0xFCD9B3, true),
        ("action_east", 0, 37, 37, 0xD37A57, true),
        ("action_east", 0, 38, 38, 0x672115, true),
        ("action_east", 0, 40, 29, 0x686589, false),
        ("action_east", 0, 38, 52, 0x612026, false),
        ("action_east", 0, 39, 41, 0xB395D6, false),
        ("action_east", 0, 39, 42, 0xDEDAE9, false),
        ("action_east", 0, 38, 45, 0x374050, false),
        ("action_east", 1, 44, 35, 0xFCD9B3, true),
        ("action_east", 1, 39, 36, 0xD37A57, true),
        ("action_east", 1, 40, 37, 0x672115, true),
        ("action_east", 1, 42, 28, 0x686589, false),
        ("action_east", 1, 39, 45, 0x612026, false),
        ("action_east", 1, 41, 40, 0xB395D6, false),
        ("action_east", 1, 42, 40, 0xDEDAE9, false),
        ("action_east", 1, 46, 43, 0x374050, false),
        ("action_east", 2, 44, 35, 0xFCD9B3, true),
        ("action_east", 2, 39, 36, 0xD37A57, true),
        ("action_east", 2, 40, 37, 0x672115, true),
        ("action_east", 2, 42, 28, 0x686589, false),
        ("action_east", 2, 39, 45, 0x612026, false),
        ("action_east", 2, 41, 40, 0xB395D6, false),
        ("action_east", 2, 41, 41, 0xDEDAE9, false),
        ("action_east", 2, 43, 44, 0x374050, false),
        ("action_east", 3, 44, 35, 0xFCD9B3, true),
        ("action_east", 3, 39, 36, 0xD37A57, true),
        ("action_east", 3, 40, 37, 0x672115, true),
        ("action_east", 3, 42, 28, 0x686589, false),
        ("action_east", 3, 39, 45, 0x612026, false),
        ("action_east", 3, 41, 40, 0xB395D6, false),
        ("action_east", 3, 42, 40, 0xDEDAE9, false),
        ("action_east", 3, 46, 43, 0x374050, false),
        ("action_east", 4, 44, 35, 0xFCD9B3, true),
        ("action_east", 4, 39, 36, 0xD37A57, true),
        ("action_east", 4, 40, 37, 0x672115, true),
        ("action_east", 4, 42, 28, 0x686589, false),
        ("action_east", 4, 39, 45, 0x612026, false),
        ("action_east", 4, 41, 40, 0xB395D6, false),
        ("action_east", 4, 41, 41, 0xDEDAE9, false),
        ("action_east", 4, 43, 44, 0x374050, false),
        ("action_east", 5, 42, 36, 0xFCD9B3, true),
        ("action_east", 5, 37, 37, 0xD37A57, true),
        ("action_east", 5, 38, 38, 0x672115, true),
        ("action_east", 5, 40, 29, 0x686589, false),
        ("action_east", 5, 38, 52, 0x612026, false),
        ("action_east", 5, 39, 41, 0xB395D6, false),
        ("action_east", 5, 39, 42, 0xDEDAE9, false),
        ("action_east", 5, 38, 45, 0x374050, false),
        ("action_east", 6, 41, 35, 0xFCD9B3, true),
        ("action_east", 6, 36, 36, 0xD37A57, true),
        ("action_east", 6, 37, 37, 0x672115, true),
        ("action_east", 6, 39, 28, 0x686589, false),
        ("action_east", 6, 38, 45, 0x612026, false),
        ("action_east", 6, 38, 40, 0xB395D6, false),
        ("action_east", 6, 38, 41, 0xDEDAE9, false),
        ("action_east", 6, 34, 44, 0x374050, false),
        ("sleep_east", 0, 39, 35, 0xFCD9B3, true),
        ("sleep_east", 0, 36, 36, 0xD37A57, true),
        ("sleep_east", 0, 37, 37, 0x672115, true),
        ("sleep_east", 0, 39, 28, 0x686589, false),
        ("sleep_east", 0, 38, 45, 0x612026, false),
        ("sleep_east", 0, 40, 40, 0xB395D6, false),
        ("sleep_east", 0, 39, 40, 0xDEDAE9, false),
        ("sleep_east", 0, 42, 39, 0x374050, false),
        ("kiss_east", 0, 40, 36, 0xFCD9B3, true),
        ("kiss_east", 0, 35, 37, 0xD37A57, true),
        ("kiss_east", 0, 36, 38, 0x672115, true),
        ("kiss_east", 0, 38, 29, 0x686589, false),
        ("kiss_east", 0, 37, 48, 0x612026, false),
        ("kiss_east", 0, 37, 41, 0xB395D6, false),
        ("kiss_east", 0, 37, 42, 0xDEDAE9, false),
        ("kiss_east", 0, 34, 45, 0x374050, false),
        ("kiss_east", 1, 42, 36, 0xFCD9B3, true),
        ("kiss_east", 1, 37, 37, 0xD37A57, true),
        ("kiss_east", 1, 38, 38, 0x672115, true),
        ("kiss_east", 1, 40, 29, 0x686589, false),
        ("kiss_east", 1, 38, 48, 0x612026, false),
        ("kiss_east", 1, 39, 41, 0xB395D6, false),
        ("kiss_east", 1, 39, 42, 0xDEDAE9, false),
        ("kiss_east", 1, 35, 45, 0x374050, false),
        ("kiss_east", 2, 44, 35, 0xFCD9B3, true),
        ("kiss_east", 2, 39, 36, 0xD37A57, true),
        ("kiss_east", 2, 40, 37, 0x672115, true),
        ("kiss_east", 2, 42, 28, 0x686589, false),
        ("kiss_east", 2, 39, 45, 0x612026, false),
        ("kiss_east", 2, 40, 40, 0xB395D6, false),
        ("kiss_east", 2, 40, 41, 0xDEDAE9, false),
        ("kiss_east", 2, 38, 44, 0x374050, false),
        ("kiss_east", 3, 42, 36, 0xFCD9B3, true),
        ("kiss_east", 3, 37, 37, 0xD37A57, true),
        ("kiss_east", 3, 38, 38, 0x672115, true),
        ("kiss_east", 3, 40, 29, 0x686589, false),
        ("kiss_east", 3, 38, 48, 0x612026, false),
        ("kiss_east", 3, 39, 41, 0xB395D6, false),
        ("kiss_east", 3, 39, 42, 0xDEDAE9, false),
        ("kiss_east", 3, 35, 45, 0x374050, false),
        ("kiss_east", 2, 43, 39, 0xD37A57, true),
        ("kiss_east", 2, 44, 39, 0xD37A57, true),
        ("kiss_east", 2, 45, 39, 0x000000, false),
        ("kiss_east", 2, 39, 45, 0x612026, false),
    ];
    let cases: [(&str, &[usize]); 5] = [
        ("action_north", &[17, 11, 13, 11, 13, 17, 20]),
        ("action_south", &[45, 44, 45, 44, 45, 45, 49]),
        ("action_east", &[38, 37, 38, 37, 38, 38, 41]),
        ("sleep_east", &[37]),
        ("kiss_east", &[38, 39, 43, 43]),
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
            let prefix = "spr_npc_balor_summer";
            let asset = format!("assets/animations/NPCs/Balor/Sprites/Summer/{prefix}_{case}.png");
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
            let mut belt_and_shoe_shadows = 0;
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                if p.0 == rgba(0x612026) {
                    assert_eq!(p, q, "belt or shoe shadow changed: {id} {case} [{x},{y}]");
                    belt_and_shoe_shadows += 1;
                }
                // All four reviewed world shades are skin in these Summer strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Balor Summer material mismatch: {id} {case} [{x},{y}]"
                );
                mask.push(p != q);
                if p != q {
                    let index = source.iter().position(|c| rgba(*c) == p.0).expect(
                        "hair, shirt, belt, shoes, cuffs, eyes or another non-skin color changed",
                    );
                    assert_eq!(q.0, rgba(target[index]));
                    per_frame[x as usize / 80] += 1;
                }
            }
            assert_eq!(
                belt_and_shoe_shadows,
                match case {
                    "action_east" => 61,
                    "action_north" => 65,
                    "action_south" => 41,
                    "kiss_east" => 22,
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
