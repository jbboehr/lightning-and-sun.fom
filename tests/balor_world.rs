use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/balor-winter-world-study and the local accepted Balor portrait baseline"]
fn balor_world_covers_skin_and_preserves_outfit_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-winter-world-study");
    let baseline =
        root.join("generated/characters-reina-juniper-march-wedding-finish-trial/characters/balor");
    let set = std::env::var_os("FOM_BALOR_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/balor-portraits.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..110];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..110], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 207);
    assert_eq!(
        &candidate["source_colors"].as_array().unwrap()[..7],
        profile["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &candidate["color_groups"].as_array().unwrap()[..4],
        profile["color_groups"].as_array().unwrap()
    );
    let portrait_set: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/sets/balor-portraits-trial.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(presets["presets"].as_array().unwrap().len(), 4);
    for (world, portrait) in presets["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(portrait_set["presets"].as_array().unwrap())
    {
        assert_eq!(world["id"], portrait["id"]);
        assert_eq!(world["label"], portrait["label"]);
        let colors = world["colors"].as_array().unwrap();
        assert_eq!(&colors[..7], portrait["colors"].as_array().unwrap());
        assert_eq!(&colors[7..], &colors[..4]);
    }
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
    // Literal landmarks distinguish exposed face/neck/hands from purple hair,
    // blue scarf, shirt, bag, trousers and boots. Trouser #612026 also appears
    // in the accepted portrait skin palette and must remain untouched here.
    let landmarks = [
        ("idle_east", 0, 39, 31, 0x686589, false),
        ("idle_east", 0, 40, 36, 0xFCD9B3, true),
        ("idle_east", 0, 34, 45, 0xFCD9B3, true),
        ("idle_east", 0, 40, 41, 0x5C72D8, false),
        ("idle_east", 0, 39, 46, 0x8F4A54, false),
        ("idle_east", 0, 38, 53, 0x686589, false),
        ("idle_north", 0, 39, 31, 0x9B83B7, false),
        ("idle_north", 0, 40, 36, 0x686589, false),
        ("idle_north", 0, 34, 45, 0x525A62, false),
        ("idle_north", 0, 40, 41, 0x2E2E64, false),
        ("idle_north", 0, 39, 46, 0x6F8396, false),
        ("idle_north", 0, 38, 53, 0x686589, false),
        ("idle_south", 0, 39, 31, 0x9B83B7, false),
        ("idle_south", 0, 40, 36, 0xFCD9B3, true),
        ("idle_south", 0, 34, 45, 0x525A62, false),
        ("idle_south", 0, 40, 41, 0x424E88, false),
        ("idle_south", 0, 39, 46, 0xECF0E9, false),
        ("idle_south", 0, 38, 53, 0x686589, false),
        ("walk_east", 0, 39, 31, 0x686589, false),
        ("walk_east", 0, 40, 36, 0xFCD9B3, true),
        ("walk_east", 0, 34, 45, 0xFCD9B3, true),
        ("walk_east", 0, 40, 41, 0x5C72D8, false),
        ("walk_east", 0, 39, 46, 0x8F4A54, false),
        ("walk_east", 0, 38, 53, 0x686589, false),
        ("walk_east", 1, 39, 31, 0x9B83B7, false),
        ("walk_east", 1, 40, 36, 0xF0B988, true),
        ("walk_east", 1, 34, 45, 0xFCD9B3, true),
        ("walk_east", 1, 40, 41, 0x672115, true),
        ("walk_east", 1, 39, 46, 0x77405D, false),
        ("walk_east", 2, 39, 31, 0x686589, false),
        ("walk_east", 2, 40, 36, 0xFCD9B3, true),
        ("walk_east", 2, 34, 45, 0xFCD9B3, true),
        ("walk_east", 2, 40, 41, 0x5C72D8, false),
        ("walk_east", 2, 39, 46, 0x8F4A54, false),
        ("walk_east", 2, 38, 53, 0x686589, false),
        ("walk_east", 3, 39, 31, 0x9B83B7, false),
        ("walk_east", 3, 40, 36, 0xF0B988, true),
        ("walk_east", 3, 34, 45, 0x000000, false),
        ("walk_east", 3, 40, 41, 0x672115, true),
        ("walk_east", 3, 39, 46, 0x77405D, false),
        ("walk_east", 3, 38, 53, 0x686589, false),
        ("walk_north", 0, 39, 31, 0x9B83B7, false),
        ("walk_north", 0, 40, 36, 0x686589, false),
        ("walk_north", 0, 34, 45, 0x525A62, false),
        ("walk_north", 0, 40, 41, 0x2E2E64, false),
        ("walk_north", 0, 39, 46, 0x6F8396, false),
        ("walk_north", 0, 38, 53, 0x686589, false),
        ("walk_north", 1, 39, 31, 0x686589, false),
        ("walk_north", 1, 40, 36, 0x686589, false),
        ("walk_north", 1, 34, 45, 0xFCD9B3, true),
        ("walk_north", 1, 40, 41, 0x000000, false),
        ("walk_north", 1, 39, 46, 0x525A62, false),
        ("walk_north", 1, 38, 53, 0x000000, false),
        ("walk_north", 2, 39, 31, 0x9B83B7, false),
        ("walk_north", 2, 40, 36, 0x686589, false),
        ("walk_north", 2, 34, 45, 0x525A62, false),
        ("walk_north", 2, 40, 41, 0x2E2E64, false),
        ("walk_north", 2, 39, 46, 0x6F8396, false),
        ("walk_north", 2, 38, 53, 0x686589, false),
        ("walk_north", 3, 39, 31, 0x686589, false),
        ("walk_north", 3, 40, 36, 0x686589, false),
        ("walk_north", 3, 34, 45, 0x374050, false),
        ("walk_north", 3, 40, 41, 0x000000, false),
        ("walk_north", 3, 39, 46, 0x6F8396, false),
        ("walk_north", 3, 38, 53, 0x686589, false),
        ("walk_south", 0, 39, 31, 0x9B83B7, false),
        ("walk_south", 0, 40, 36, 0xFCD9B3, true),
        ("walk_south", 0, 34, 45, 0x525A62, false),
        ("walk_south", 0, 40, 41, 0x424E88, false),
        ("walk_south", 0, 39, 46, 0xECF0E9, false),
        ("walk_south", 0, 38, 53, 0x686589, false),
        ("walk_south", 1, 39, 31, 0x9B83B7, false),
        ("walk_south", 1, 40, 36, 0xFCD9B3, true),
        ("walk_south", 1, 34, 45, 0x525A62, false),
        ("walk_south", 1, 40, 41, 0x672115, true),
        ("walk_south", 1, 39, 46, 0xA56B81, false),
        ("walk_south", 1, 38, 53, 0x000000, false),
        ("walk_south", 2, 39, 31, 0x9B83B7, false),
        ("walk_south", 2, 40, 36, 0xFCD9B3, true),
        ("walk_south", 2, 34, 45, 0x525A62, false),
        ("walk_south", 2, 40, 41, 0x424E88, false),
        ("walk_south", 2, 39, 46, 0xECF0E9, false),
        ("walk_south", 2, 38, 53, 0x686589, false),
        ("walk_south", 3, 39, 31, 0x9B83B7, false),
        ("walk_south", 3, 40, 36, 0xF0B988, true),
        ("walk_south", 3, 34, 45, 0xD37A57, true),
        ("walk_south", 3, 40, 41, 0x672115, true),
        ("walk_south", 3, 39, 46, 0xA56B81, false),
        ("walk_south", 3, 38, 53, 0x4A3D66, false),
        ("idle_south", 0, 39, 40, 0x672115, true),
        ("idle_south", 0, 40, 40, 0x672115, true),
        ("idle_south", 0, 36, 46, 0x612026, false),
        ("idle_south", 0, 37, 46, 0x612026, false),
        ("idle_south", 0, 36, 47, 0x612026, false),
        ("idle_south", 0, 35, 48, 0x8F4A54, false),
        ("idle_south", 0, 38, 44, 0xB3AFBD, false),
        ("idle_south", 0, 37, 35, 0xC2B9BE, false),
        ("idle_south", 0, 38, 36, 0x000000, false),
        ("idle_south", 0, 39, 37, 0xFCD9B3, true),
        ("idle_east", 0, 40, 40, 0x672115, true),
        ("idle_east", 0, 41, 40, 0x672115, true),
        ("idle_east", 0, 41, 46, 0xECF0E9, false),
        ("idle_east", 0, 42, 46, 0x8F4A54, false),
        ("idle_east", 0, 43, 46, 0x612026, false),
        ("idle_east", 0, 39, 47, 0xA56B81, false),
        ("idle_north", 0, 32, 45, 0xFCD9B3, true),
        ("idle_north", 0, 33, 46, 0x672115, true),
        ("idle_north", 0, 34, 46, 0xFCD9B3, true),
        ("idle_north", 0, 39, 44, 0x374050, false),
        ("idle_north", 0, 38, 51, 0x4A3D66, false),
    ];
    let cases: [(&str, &[usize]); 6] = [
        ("idle_east", &[33]),
        ("idle_north", &[12]),
        ("idle_south", &[41]),
        ("walk_east", &[33, 36, 33, 33]),
        ("walk_north", &[12, 11, 12, 9]),
        ("walk_south", &[41, 39, 41, 39]),
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
                    "idle_east" => 4,
                    "idle_north" | "walk_north" => 0,
                    "idle_south" => 5,
                    "walk_east" => 12,
                    "walk_south" => 17,
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
