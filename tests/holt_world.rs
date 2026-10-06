use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/holt and the local accepted Holt portrait baseline"]
fn holt_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/holt");
    let baseline = root.join("generated/slice-084-build/characters/holt");
    let set = std::env::var_os("FOM_HOLT_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/holt-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/holt-portraits.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..32];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..32], prior);
    assert_eq!(
        &candidate["source_colors"].as_array().unwrap()[..9],
        profile["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &candidate["source_colors"].as_array().unwrap()[9..],
        &["#F0CC8E", "#96523A", "#AD6945"]
    );
    assert_eq!(
        &candidate["color_groups"].as_array().unwrap()[..9],
        profile["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        candidate["color_groups"][9],
        serde_json::json!(["#F0CC8E", "#96523A", "#AD6945"])
    );
    let portrait_set: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/sets/holt-portraits-trial.json")).unwrap(),
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
        assert_eq!(&colors[..9], portrait["colors"].as_array().unwrap());
        assert_eq!(
            &colors[9..],
            &[colors[0].clone(), colors[2].clone(), colors[2].clone()]
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
    let source = [0xF0CC8E, 0xCD9067, 0x96523A, 0xAD6945];
    // These pinned strips separate skin from hair, moustache, scarf and clothing.
    // Boots below row 49 reuse three skin shades and remain original.
    // The extra #AD6945 shade is the side-view nose edge.
    let landmarks = [
        ("idle_east", 0, 35, 48, 0x000000, false),
        ("idle_east", 0, 37, 49, 0x163253, false),
        ("idle_east", 0, 39, 48, 0x3B5370, false),
        ("idle_east", 0, 44, 43, 0x55908C, false),
        ("idle_east", 0, 41, 52, 0x762E21, false),
        ("idle_east", 0, 34, 43, 0x7AB99F, false),
        ("idle_east", 0, 41, 50, 0x837D60, false),
        ("idle_east", 0, 41, 51, 0x96523A, false),
        ("idle_east", 0, 35, 46, 0x96523A, true),
        ("idle_east", 0, 34, 44, 0x9FDDCF, false),
        ("idle_east", 0, 43, 34, 0xA59DA2, false),
        ("idle_east", 0, 44, 35, 0xAD6945, true),
        ("idle_east", 0, 42, 43, 0xB65932, false),
        ("idle_east", 0, 42, 50, 0xBEBC91, false),
        ("idle_east", 0, 39, 34, 0xC2B9BE, false),
        ("idle_east", 0, 33, 47, 0xCD9067, true),
        ("idle_east", 0, 44, 30, 0xD2793F, false),
        ("idle_east", 0, 38, 36, 0xECF0E9, false),
        ("idle_east", 0, 40, 53, 0xF0CC8E, false),
        ("idle_east", 0, 34, 45, 0xF0CC8E, true),
        ("idle_east", 0, 39, 45, 0xF1A354, false),
        ("idle_east", 0, 42, 41, 0xFCF2DE, false),
        ("idle_north", 0, 44, 48, 0x000000, false),
        ("idle_north", 0, 41, 47, 0x163253, false),
        ("idle_north", 0, 41, 46, 0x3B5370, false),
        ("idle_north", 0, 45, 43, 0x55908C, false),
        ("idle_north", 0, 41, 52, 0x762E21, false),
        ("idle_north", 0, 38, 44, 0x7AB99F, false),
        ("idle_north", 0, 42, 50, 0x837D60, false),
        ("idle_north", 0, 45, 46, 0x96523A, true),
        ("idle_north", 0, 45, 44, 0x9FDDCF, false),
        ("idle_north", 0, 37, 42, 0xB65932, false),
        ("idle_north", 0, 41, 50, 0xBEBC91, false),
        ("idle_north", 0, 37, 53, 0xCD9067, false),
        ("idle_north", 0, 34, 47, 0xCD9067, true),
        ("idle_north", 0, 36, 33, 0xD2793F, false),
        ("idle_north", 0, 38, 53, 0xF0CC8E, false),
        ("idle_north", 0, 33, 46, 0xF0CC8E, true),
        ("idle_north", 0, 36, 41, 0xF1A354, false),
        ("idle_north", 0, 45, 40, 0xFCF2DE, false),
        ("idle_south", 0, 44, 48, 0x000000, false),
        ("idle_south", 0, 36, 48, 0x163253, false),
        ("idle_south", 0, 37, 48, 0x3B5370, false),
        ("idle_south", 0, 43, 43, 0x55908C, false),
        ("idle_south", 0, 41, 52, 0x762E21, false),
        ("idle_south", 0, 33, 43, 0x7AB99F, false),
        ("idle_south", 0, 42, 50, 0x837D60, false),
        ("idle_south", 0, 33, 47, 0x96523A, true),
        ("idle_south", 0, 45, 44, 0x9FDDCF, false),
        ("idle_south", 0, 37, 43, 0xB65932, false),
        ("idle_south", 0, 41, 50, 0xBEBC91, false),
        ("idle_south", 0, 41, 34, 0xC2B9BE, false),
        ("idle_south", 0, 42, 51, 0xCD9067, false),
        ("idle_south", 0, 45, 46, 0xCD9067, true),
        ("idle_south", 0, 43, 30, 0xD2793F, false),
        ("idle_south", 0, 37, 36, 0xECF0E9, false),
        ("idle_south", 0, 38, 53, 0xF0CC8E, false),
        ("idle_south", 0, 33, 46, 0xF0CC8E, true),
        ("idle_south", 0, 41, 42, 0xF1A354, false),
        ("idle_south", 0, 38, 41, 0xFCF2DE, false),
        ("walk_east", 0, 35, 48, 0x000000, false),
        ("walk_east", 0, 37, 49, 0x163253, false),
        ("walk_east", 0, 39, 48, 0x3B5370, false),
        ("walk_east", 0, 44, 43, 0x55908C, false),
        ("walk_east", 0, 41, 52, 0x762E21, false),
        ("walk_east", 0, 34, 43, 0x7AB99F, false),
        ("walk_east", 0, 41, 50, 0x837D60, false),
        ("walk_east", 0, 41, 51, 0x96523A, false),
        ("walk_east", 0, 35, 46, 0x96523A, true),
        ("walk_east", 0, 34, 44, 0x9FDDCF, false),
        ("walk_east", 0, 43, 34, 0xA59DA2, false),
        ("walk_east", 0, 44, 35, 0xAD6945, true),
        ("walk_east", 0, 42, 43, 0xB65932, false),
        ("walk_east", 0, 42, 50, 0xBEBC91, false),
        ("walk_east", 0, 39, 34, 0xC2B9BE, false),
        ("walk_east", 0, 33, 47, 0xCD9067, true),
        ("walk_east", 0, 44, 30, 0xD2793F, false),
        ("walk_east", 0, 38, 36, 0xECF0E9, false),
        ("walk_east", 0, 40, 53, 0xF0CC8E, false),
        ("walk_east", 0, 34, 45, 0xF0CC8E, true),
        ("walk_east", 0, 39, 45, 0xF1A354, false),
        ("walk_east", 0, 42, 41, 0xFCF2DE, false),
        ("walk_east", 1, 46, 45, 0x96523A, true),
        ("walk_east", 1, 44, 36, 0xAD6945, true),
        ("walk_east", 1, 41, 52, 0xCD9067, false),
        ("walk_east", 1, 47, 44, 0xCD9067, true),
        ("walk_east", 1, 42, 53, 0xF0CC8E, false),
        ("walk_east", 1, 32, 45, 0xF0CC8E, true),
        ("walk_east", 2, 41, 51, 0x96523A, false),
        ("walk_east", 2, 35, 46, 0x96523A, true),
        ("walk_east", 2, 44, 35, 0xAD6945, true),
        ("walk_east", 2, 33, 47, 0xCD9067, true),
        ("walk_east", 2, 40, 53, 0xF0CC8E, false),
        ("walk_east", 2, 34, 45, 0xF0CC8E, true),
        ("walk_east", 3, 37, 46, 0x96523A, true),
        ("walk_east", 3, 44, 36, 0xAD6945, true),
        ("walk_east", 3, 42, 51, 0xCD9067, false),
        ("walk_east", 3, 37, 45, 0xCD9067, true),
        ("walk_east", 3, 44, 52, 0xF0CC8E, false),
        ("walk_east", 3, 36, 45, 0xF0CC8E, true),
        ("walk_north", 0, 44, 48, 0x000000, false),
        ("walk_north", 0, 41, 47, 0x163253, false),
        ("walk_north", 0, 41, 46, 0x3B5370, false),
        ("walk_north", 0, 45, 43, 0x55908C, false),
        ("walk_north", 0, 41, 52, 0x762E21, false),
        ("walk_north", 0, 38, 44, 0x7AB99F, false),
        ("walk_north", 0, 42, 50, 0x837D60, false),
        ("walk_north", 0, 45, 46, 0x96523A, true),
        ("walk_north", 0, 45, 44, 0x9FDDCF, false),
        ("walk_north", 0, 37, 42, 0xB65932, false),
        ("walk_north", 0, 41, 50, 0xBEBC91, false),
        ("walk_north", 0, 37, 53, 0xCD9067, false),
        ("walk_north", 0, 34, 47, 0xCD9067, true),
        ("walk_north", 0, 36, 33, 0xD2793F, false),
        ("walk_north", 0, 38, 53, 0xF0CC8E, false),
        ("walk_north", 0, 33, 46, 0xF0CC8E, true),
        ("walk_north", 0, 36, 41, 0xF1A354, false),
        ("walk_north", 0, 45, 40, 0xFCF2DE, false),
        ("walk_north", 1, 37, 52, 0x96523A, false),
        ("walk_north", 1, 46, 46, 0x96523A, true),
        ("walk_north", 1, 42, 52, 0xCD9067, false),
        ("walk_north", 1, 36, 46, 0xCD9067, true),
        ("walk_north", 1, 41, 54, 0xF0CC8E, false),
        ("walk_north", 1, 34, 47, 0xF0CC8E, true),
        ("walk_north", 2, 45, 46, 0x96523A, true),
        ("walk_north", 2, 37, 53, 0xCD9067, false),
        ("walk_north", 2, 34, 47, 0xCD9067, true),
        ("walk_north", 2, 38, 53, 0xF0CC8E, false),
        ("walk_north", 2, 33, 46, 0xF0CC8E, true),
        ("walk_north", 3, 42, 52, 0x96523A, false),
        ("walk_north", 3, 33, 46, 0x96523A, true),
        ("walk_north", 3, 41, 52, 0xCD9067, false),
        ("walk_north", 3, 34, 46, 0xCD9067, true),
        ("walk_north", 3, 38, 54, 0xF0CC8E, false),
        ("walk_north", 3, 44, 47, 0xF0CC8E, true),
        ("walk_south", 0, 44, 48, 0x000000, false),
        ("walk_south", 0, 36, 48, 0x163253, false),
        ("walk_south", 0, 37, 48, 0x3B5370, false),
        ("walk_south", 0, 43, 43, 0x55908C, false),
        ("walk_south", 0, 41, 52, 0x762E21, false),
        ("walk_south", 0, 33, 43, 0x7AB99F, false),
        ("walk_south", 0, 42, 50, 0x837D60, false),
        ("walk_south", 0, 33, 47, 0x96523A, true),
        ("walk_south", 0, 45, 44, 0x9FDDCF, false),
        ("walk_south", 0, 37, 43, 0xB65932, false),
        ("walk_south", 0, 41, 50, 0xBEBC91, false),
        ("walk_south", 0, 41, 34, 0xC2B9BE, false),
        ("walk_south", 0, 42, 51, 0xCD9067, false),
        ("walk_south", 0, 45, 46, 0xCD9067, true),
        ("walk_south", 0, 43, 30, 0xD2793F, false),
        ("walk_south", 0, 37, 36, 0xECF0E9, false),
        ("walk_south", 0, 38, 53, 0xF0CC8E, false),
        ("walk_south", 0, 33, 46, 0xF0CC8E, true),
        ("walk_south", 0, 41, 42, 0xF1A354, false),
        ("walk_south", 0, 38, 41, 0xFCF2DE, false),
        ("walk_south", 1, 46, 46, 0x96523A, true),
        ("walk_south", 1, 38, 52, 0xCD9067, false),
        ("walk_south", 1, 46, 45, 0xCD9067, true),
        ("walk_south", 1, 41, 54, 0xF0CC8E, false),
        ("walk_south", 1, 34, 47, 0xF0CC8E, true),
        ("walk_south", 2, 33, 47, 0x96523A, true),
        ("walk_south", 2, 42, 51, 0xCD9067, false),
        ("walk_south", 2, 45, 46, 0xCD9067, true),
        ("walk_south", 2, 38, 53, 0xF0CC8E, false),
        ("walk_south", 2, 33, 46, 0xF0CC8E, true),
        ("walk_south", 3, 43, 46, 0x96523A, true),
        ("walk_south", 3, 41, 52, 0xCD9067, false),
        ("walk_south", 3, 34, 45, 0xCD9067, true),
        ("walk_south", 3, 37, 54, 0xF0CC8E, false),
        ("walk_south", 3, 44, 47, 0xF0CC8E, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[40]),
        ("idle_north", &[22]),
        ("idle_south", &[47]),
        ("walk_east", &[40, 44, 40, 37]),
        ("walk_north", &[22, 20, 22, 20]),
        ("walk_south", &[47, 44, 47, 44]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [9, 1, 10, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_holt_spring";
            let asset = format!("assets/animations/NPCs/Holt/Sprites/Spring/{prefix}_{case}.png");
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
                // In these six pinned strips, exposed skin ends above row 49.
                // The shoes below it reuse the same source shades.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && y < 49)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Holt Spring material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin even when a Ryis target equals its source.
                if y < 49 && source.iter().any(|c| rgba(*c) == p.0) {
                    let index = source
                        .iter()
                        .position(|c| rgba(*c) == p.0)
                        .expect("non-skin material changed");
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
