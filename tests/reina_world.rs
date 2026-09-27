use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/reina-autumn-actions-study and the local accepted Reina portraits baseline"]
fn reina_world_masks_cover_skin_and_preserve_shared_color_jacket_folds() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/reina-autumn-actions-study");
    let baseline = root.join("generated/characters-world-wedding-finish-trial/characters/reina");
    let set = std::env::var_os("FOM_REINA_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/reina-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/reina-portraits.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..103];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..103], prior);
    assert_eq!(
        &candidate["source_colors"].as_array().unwrap()[..8],
        profile["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &candidate["color_groups"].as_array().unwrap()[..1],
        profile["color_groups"].as_array().unwrap()
    );
    let accepted_set: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/sets/reina-portraits-trial.json")).unwrap(),
    )
    .unwrap();
    for (new, old) in presets["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(accepted_set["presets"].as_array().unwrap())
    {
        assert_eq!(
            &new["colors"].as_array().unwrap()[8..11],
            &old["colors"].as_array().unwrap()[1..4]
        );
        assert_eq!(new["id"], old["id"]);
        assert_eq!(new["label"], old["label"]);
        assert_eq!(
            &new["colors"].as_array().unwrap()[..8],
            old["colors"].as_array().unwrap()
        );
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
    let source = [0xB36844, 0x8E4538, 0x712922, 0x571D1F];
    // Literal frame-local material landmarks, independently inspected in the source.
    // The exposed midriff changes; the blouse neckline, belt and jacket folds do not.
    let landmarks = [
        ("idle_east", 0, 40, 35, 0xB36844, true),
        ("idle_east", 0, 42, 41, 0xB36844, true),
        ("idle_east", 0, 39, 45, 0xB36844, true),
        ("idle_east", 0, 35, 46, 0xB36844, true),
        ("idle_east", 0, 36, 47, 0xB36844, true),
        ("idle_east", 0, 40, 49, 0x000000, false),
        ("idle_east", 0, 37, 43, 0xB36844, false),
        ("idle_east", 0, 39, 41, 0xF0B0A3, false),
        ("idle_east", 0, 40, 46, 0xF7E29C, false),
        ("idle_east", 0, 40, 53, 0xA13761, false),
        ("idle_east", 0, 40, 30, 0x63413B, false),
        ("idle_north", 0, 32, 46, 0xB36844, true),
        ("idle_north", 0, 33, 47, 0x571D1F, true),
        ("idle_north", 0, 45, 45, 0xB36844, true),
        ("idle_north", 0, 37, 49, 0x8E4538, true),
        ("idle_north", 0, 34, 44, 0xD2962F, false),
        ("idle_north", 0, 39, 43, 0xF7E29C, false),
        ("idle_north", 0, 39, 36, 0x452929, false),
        ("idle_north", 0, 40, 52, 0x000000, false),
        ("idle_south", 0, 39, 35, 0xB36844, true),
        ("idle_south", 0, 35, 38, 0x000000, false),
        ("idle_south", 0, 39, 41, 0x8E4538, true),
        ("idle_south", 0, 38, 45, 0xB36844, true),
        ("idle_south", 0, 32, 46, 0xB36844, true),
        ("idle_south", 0, 33, 47, 0x571D1F, true),
        ("idle_south", 0, 37, 49, 0x8E4538, true),
        ("idle_south", 0, 36, 43, 0xB36844, false),
        ("idle_south", 0, 43, 43, 0xB36844, false),
        ("idle_south", 0, 40, 42, 0xF0B0A3, false),
        ("idle_south", 0, 34, 44, 0xD2962F, false),
        ("idle_south", 0, 38, 31, 0x63413B, false),
        ("walk_east", 1, 38, 44, 0xB36844, false),
        ("walk_east", 1, 42, 42, 0xB36844, true),
        ("walk_east", 1, 39, 46, 0xB36844, true),
        ("walk_east", 1, 32, 46, 0xB36844, true),
        ("walk_east", 1, 34, 47, 0xB36844, true),
        ("walk_east", 1, 42, 50, 0x000000, false),
        ("walk_east", 1, 40, 44, 0xF5F5F5, false),
        ("walk_east", 1, 40, 47, 0xF7E29C, false),
        ("walk_east", 3, 42, 42, 0xB36844, true),
        ("walk_east", 3, 39, 46, 0xB36844, true),
        ("walk_east", 3, 36, 47, 0xB36844, true),
        ("walk_east", 3, 36, 50, 0x000000, false),
        ("walk_east", 3, 37, 45, 0xD2962F, false),
        ("walk_east", 3, 43, 51, 0x581831, false),
        ("walk_north", 1, 34, 46, 0xB36844, true),
        ("walk_north", 1, 34, 48, 0x571D1F, true),
        ("walk_north", 1, 46, 46, 0x712922, true),
        ("walk_north", 1, 37, 49, 0x712922, true),
        ("walk_north", 1, 42, 51, 0x8E4538, true),
        ("walk_north", 1, 36, 45, 0xD2962F, false),
        ("walk_north", 1, 39, 45, 0xF7E29C, false),
        ("walk_north", 3, 33, 46, 0x712922, true),
        ("walk_north", 3, 45, 46, 0xB36844, true),
        ("walk_north", 3, 45, 48, 0x571D1F, true),
        ("walk_north", 3, 37, 51, 0x8E4538, true),
        ("walk_north", 3, 42, 49, 0x712922, true),
        ("walk_north", 3, 43, 45, 0xD2962F, false),
        ("walk_south", 1, 36, 44, 0xB36844, false),
        ("walk_south", 1, 43, 44, 0xB36844, false),
        ("walk_south", 1, 38, 46, 0xB36844, true),
        ("walk_south", 1, 34, 47, 0xB36844, true),
        ("walk_south", 1, 46, 46, 0x712922, true),
        ("walk_south", 1, 39, 43, 0xF0B0A3, false),
        ("walk_south", 1, 37, 49, 0x712922, true),
        ("walk_south", 3, 36, 44, 0xB36844, false),
        ("walk_south", 3, 43, 44, 0xB36844, false),
        ("walk_south", 3, 38, 46, 0xB36844, true),
        ("walk_south", 3, 33, 46, 0x712922, true),
        ("walk_south", 3, 45, 47, 0xB36844, true),
        ("walk_south", 3, 37, 51, 0x8E4538, true),
        ("walk_south", 3, 40, 43, 0xF0B0A3, false),
    ];
    let cases: [(&str, &[usize]); 6] = [
        ("idle_east", &[60]),
        ("idle_north", &[26]),
        ("idle_south", &[68]),
        ("walk_east", &[60, 64, 60, 57]),
        ("walk_north", &[26, 19, 26, 19]),
        ("walk_south", &[68, 61, 68, 61]),
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
                "idle_east" => &[[37, 43]],
                "idle_south" => &[[36, 43], [43, 43]],
                "walk_east" => &[[37, 43], [197, 43], [118, 44]],
                "walk_south" => &[
                    [36, 43],
                    [43, 43],
                    [196, 43],
                    [203, 43],
                    [116, 44],
                    [123, 44],
                    [276, 44],
                    [283, 44],
                ],
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
                // The four reviewed skin shades also occur in 14 isolated jacket
                // folds. Guard both their preservation and every omitted skin pixel.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !jacket_folds.contains(&[x, y]))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Reina world material mismatch: {id} {case} [{x},{y}]"
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
