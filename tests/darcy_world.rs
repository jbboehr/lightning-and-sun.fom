use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/darcy and the local accepted Darcy portrait baseline"]
fn darcy_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/darcy");
    let baseline = root.join("generated/slice-140-build/characters/darcy");
    let set = std::env::var_os("FOM_DARCY_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/darcy-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/darcy-portraits.json")).unwrap(),
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
        &candidate["source_colors"].as_array().unwrap()[..10],
        profile["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &candidate["source_colors"].as_array().unwrap()[10..],
        &["#FCD9B3", "#F6996C", "#C76539"]
    );
    assert_eq!(
        &candidate["color_groups"].as_array().unwrap()[..10],
        profile["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        candidate["color_groups"][10],
        serde_json::json!(["#FCD9B3", "#F6996C", "#C76539"])
    );
    let portrait_set: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/sets/darcy-portraits-trial.json")).unwrap(),
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
        assert_eq!(&colors[..10], portrait["colors"].as_array().unwrap());
        assert_eq!(
            &colors[10..],
            &[colors[0].clone(), colors[1].clone(), colors[2].clone()]
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
    let source = [0xFCD9B3, 0xF6996C, 0xC76539, 0x7B2806];
    // In these six pinned sources the four world shades belong only to skin.
    // Face, ears, neck and hands change; hair, eyes and clothing stay original.
    // The whole-image oracle and literal landmarks do not use component seeds.
    let landmarks = [
        ("idle_east", 0, 36, 35, 0x7B2806, true),
        ("idle_east", 0, 34, 45, 0xC76539, true),
        ("idle_east", 0, 45, 46, 0xF6996C, true),
        ("idle_north", 0, 39, 40, 0xC76539, true),
        ("idle_north", 0, 33, 45, 0xC76539, true),
        ("idle_north", 0, 45, 45, 0xF6996C, true),
        ("idle_south", 0, 35, 35, 0x7B2806, true),
        ("idle_south", 0, 33, 45, 0xC76539, true),
        ("idle_south", 0, 44, 45, 0xC76539, true),
        ("walk_east", 0, 36, 35, 0x7B2806, true),
        ("walk_east", 0, 34, 45, 0xC76539, true),
        ("walk_east", 0, 45, 46, 0xF6996C, true),
        ("walk_east", 1, 36, 36, 0x7B2806, true),
        ("walk_east", 1, 32, 46, 0xFCD9B3, true),
        ("walk_east", 1, 45, 46, 0xC76539, true),
        ("walk_east", 2, 36, 35, 0x7B2806, true),
        ("walk_east", 2, 34, 45, 0xC76539, true),
        ("walk_east", 2, 45, 46, 0xF6996C, true),
        ("walk_east", 3, 36, 36, 0x7B2806, true),
        ("walk_east", 3, 35, 46, 0xC76539, true),
        ("walk_east", 3, 44, 46, 0x7B2806, true),
        ("walk_north", 0, 39, 40, 0xC76539, true),
        ("walk_north", 0, 33, 45, 0xC76539, true),
        ("walk_north", 0, 45, 45, 0xF6996C, true),
        ("walk_north", 1, 39, 41, 0xC76539, true),
        ("walk_north", 1, 33, 46, 0xC76539, true),
        ("walk_north", 1, 45, 45, 0xC76539, true),
        ("walk_north", 2, 39, 40, 0xC76539, true),
        ("walk_north", 2, 33, 45, 0xC76539, true),
        ("walk_north", 2, 45, 45, 0xF6996C, true),
        ("walk_north", 3, 39, 41, 0xC76539, true),
        ("walk_north", 3, 34, 45, 0xC76539, true),
        ("walk_north", 3, 45, 46, 0xF6996C, true),
        ("walk_south", 0, 35, 35, 0x7B2806, true),
        ("walk_south", 0, 33, 45, 0xC76539, true),
        ("walk_south", 0, 44, 45, 0xC76539, true),
        ("walk_south", 1, 35, 36, 0x7B2806, true),
        ("walk_south", 1, 33, 47, 0xFCD9B3, true),
        ("walk_south", 1, 45, 45, 0xF6996C, true),
        ("walk_south", 2, 35, 35, 0x7B2806, true),
        ("walk_south", 2, 33, 45, 0xC76539, true),
        ("walk_south", 2, 44, 45, 0xC76539, true),
        ("walk_south", 3, 35, 36, 0x7B2806, true),
        ("walk_south", 3, 33, 46, 0xC76539, true),
        ("walk_south", 3, 44, 47, 0xC76539, true),
        ("idle_east", 0, 39, 30, 0x4F4040, false),
        ("idle_east", 0, 38, 28, 0xE73B4F, false),
        ("idle_east", 0, 40, 42, 0x86DCC7, false),
        ("idle_east", 0, 38, 51, 0x2C1E1E, false),
        ("idle_north", 0, 37, 36, 0x4F4040, false),
        ("idle_north", 0, 39, 28, 0xE73B4F, false),
        ("idle_north", 0, 39, 41, 0x86DCC7, false),
        ("idle_north", 0, 38, 52, 0x2C1E1E, false),
        ("idle_south", 0, 38, 30, 0x4F4040, false),
        ("idle_south", 0, 38, 28, 0xE73B4F, false),
        ("idle_south", 0, 39, 42, 0x86DCC7, false),
        ("idle_south", 0, 38, 52, 0x2C1E1E, false),
        ("idle_south", 0, 35, 36, 0xFCD9B3, true),
        ("idle_south", 0, 40, 35, 0xF6996C, true),
        ("idle_south", 0, 37, 38, 0xC76539, true),
        ("idle_south", 0, 35, 35, 0x7B2806, true),
        ("idle_south", 0, 37, 36, 0xECF0E9, false),
        ("idle_south", 0, 37, 35, 0xC2B9BE, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[30]),
        ("idle_north", &[18]),
        ("idle_south", &[41]),
        ("walk_east", &[30, 33, 30, 30]),
        ("walk_north", &[18, 14, 18, 14]),
        ("walk_south", &[41, 35, 41, 34]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [10, 11, 12, 9]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_darcy_spring";
            let asset = format!("assets/animations/NPCs/Darcy/Sprites/Spring/{prefix}_{case}.png");
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
                // Dark leggings and boots are clothing; no ankles are exposed.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Darcy Spring material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin independently of target colors.
                if source.iter().any(|c| rgba(*c) == p.0) {
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
