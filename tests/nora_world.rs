use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/nora and the local accepted Nora portrait baseline"]
fn nora_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/nora");
    let baseline = root.join("generated/slice-060-build/characters/nora");
    let set = std::env::var_os("FOM_NORA_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/nora-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/nora-portraits.json")).unwrap(),
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
        &["#FCD9B3", "#F0B988", "#D37A57"]
    );
    assert_eq!(
        &candidate["color_groups"].as_array().unwrap()[..5],
        profile["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        candidate["color_groups"][5],
        serde_json::json!(["#FCD9B3", "#F0B988", "#D37A57"])
    );
    let portrait_set: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/sets/nora-portraits-trial.json")).unwrap(),
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
        assert_eq!(&colors[10..], &colors[..3]);
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
    let source = [0xFCD9B3, 0xF0B988, 0xD37A57, 0x762E21];
    // Source grids separate exposed skin from hair, blouse, overalls and shoes.
    // The deepest shade belongs to hair above row 44 and to hands below it.
    // The three other world shades are skin, including the exposed ankles.
    let landmarks = [
        ("idle_east", 0, 34, 48, 0x000000, false),
        ("idle_east", 0, 38, 48, 0x163253, false),
        ("idle_east", 0, 38, 50, 0x254397, false),
        ("idle_east", 0, 40, 48, 0x3B5370, false),
        ("idle_east", 0, 40, 45, 0x5078E7, false),
        ("idle_east", 0, 34, 31, 0x762E21, false),
        ("idle_east", 0, 35, 47, 0x762E21, true),
        ("idle_east", 0, 42, 42, 0x91AEFF, false),
        ("idle_east", 0, 43, 35, 0xA59DA2, false),
        ("idle_east", 0, 42, 32, 0xB65932, false),
        ("idle_east", 0, 43, 43, 0xBEBC91, false),
        ("idle_east", 0, 39, 35, 0xC2B9BE, false),
        ("idle_east", 0, 35, 31, 0xCC9155, false),
        ("idle_east", 0, 34, 47, 0xD37A57, true),
        ("idle_east", 0, 38, 37, 0xECF0E9, false),
        ("idle_east", 0, 40, 31, 0xEFCC86, false),
        ("idle_east", 0, 45, 46, 0xF0B988, true),
        ("idle_east", 0, 38, 32, 0xFAECB2, false),
        ("idle_east", 0, 35, 46, 0xFCD9B3, true),
        ("idle_east", 0, 40, 42, 0xFFFDD3, false),
        ("idle_north", 0, 35, 48, 0x000000, false),
        ("idle_north", 0, 42, 48, 0x163253, false),
        ("idle_north", 0, 37, 45, 0x254397, false),
        ("idle_north", 0, 39, 48, 0x3B5370, false),
        ("idle_north", 0, 38, 44, 0x5078E7, false),
        ("idle_north", 0, 40, 34, 0x762E21, false),
        ("idle_north", 0, 46, 47, 0x762E21, true),
        ("idle_north", 0, 40, 41, 0x91AEFF, false),
        ("idle_north", 0, 45, 32, 0xB65932, false),
        ("idle_north", 0, 36, 44, 0xBEBC91, false),
        ("idle_north", 0, 43, 34, 0xCC9155, false),
        ("idle_north", 0, 46, 45, 0xD37A57, true),
        ("idle_north", 0, 42, 32, 0xEFCC86, false),
        ("idle_north", 0, 42, 52, 0xF0B988, true),
        ("idle_north", 0, 40, 31, 0xFAECB2, false),
        ("idle_north", 0, 47, 46, 0xFCD9B3, true),
        ("idle_north", 0, 43, 42, 0xFFFDD3, false),
        ("idle_south", 0, 34, 48, 0x000000, false),
        ("idle_south", 0, 37, 48, 0x163253, false),
        ("idle_south", 0, 42, 45, 0x254397, false),
        ("idle_south", 0, 39, 48, 0x3B5370, false),
        ("idle_south", 0, 39, 45, 0x5078E7, false),
        ("idle_south", 0, 40, 29, 0x762E21, false),
        ("idle_south", 0, 46, 47, 0x762E21, true),
        ("idle_south", 0, 41, 42, 0x91AEFF, false),
        ("idle_south", 0, 40, 32, 0xB65932, false),
        ("idle_south", 0, 36, 44, 0xBEBC91, false),
        ("idle_south", 0, 41, 35, 0xC2B9BE, false),
        ("idle_south", 0, 35, 31, 0xCC9155, false),
        ("idle_south", 0, 46, 45, 0xD37A57, true),
        ("idle_south", 0, 37, 37, 0xECF0E9, false),
        ("idle_south", 0, 41, 31, 0xEFCC86, false),
        ("idle_south", 0, 38, 52, 0xF0B988, true),
        ("idle_south", 0, 42, 32, 0xFAECB2, false),
        ("idle_south", 0, 46, 46, 0xFCD9B3, true),
        ("idle_south", 0, 40, 42, 0xFFFDD3, false),
        ("walk_east", 0, 34, 48, 0x000000, false),
        ("walk_east", 0, 38, 48, 0x163253, false),
        ("walk_east", 0, 38, 50, 0x254397, false),
        ("walk_east", 0, 40, 48, 0x3B5370, false),
        ("walk_east", 0, 40, 45, 0x5078E7, false),
        ("walk_east", 0, 34, 31, 0x762E21, false),
        ("walk_east", 0, 35, 47, 0x762E21, true),
        ("walk_east", 0, 42, 42, 0x91AEFF, false),
        ("walk_east", 0, 43, 35, 0xA59DA2, false),
        ("walk_east", 0, 42, 32, 0xB65932, false),
        ("walk_east", 0, 43, 43, 0xBEBC91, false),
        ("walk_east", 0, 39, 35, 0xC2B9BE, false),
        ("walk_east", 0, 35, 31, 0xCC9155, false),
        ("walk_east", 0, 34, 47, 0xD37A57, true),
        ("walk_east", 0, 38, 37, 0xECF0E9, false),
        ("walk_east", 0, 40, 31, 0xEFCC86, false),
        ("walk_east", 0, 45, 46, 0xF0B988, true),
        ("walk_east", 0, 38, 32, 0xFAECB2, false),
        ("walk_east", 0, 35, 46, 0xFCD9B3, true),
        ("walk_east", 0, 40, 42, 0xFFFDD3, false),
        ("walk_east", 1, 43, 40, 0x762E21, false),
        ("walk_east", 1, 46, 47, 0x762E21, true),
        ("walk_east", 1, 33, 45, 0xD37A57, true),
        ("walk_east", 1, 41, 42, 0xF0B988, true),
        ("walk_east", 1, 45, 46, 0xFCD9B3, true),
        ("walk_east", 2, 34, 31, 0x762E21, false),
        ("walk_east", 2, 35, 47, 0x762E21, true),
        ("walk_east", 2, 34, 47, 0xD37A57, true),
        ("walk_east", 2, 45, 46, 0xF0B988, true),
        ("walk_east", 2, 35, 46, 0xFCD9B3, true),
        ("walk_east", 3, 43, 40, 0x762E21, false),
        ("walk_east", 3, 44, 51, 0x762E21, true),
        ("walk_east", 3, 35, 46, 0xD37A57, true),
        ("walk_east", 3, 40, 42, 0xF0B988, true),
        ("walk_east", 3, 36, 47, 0xFCD9B3, true),
        ("walk_north", 0, 35, 48, 0x000000, false),
        ("walk_north", 0, 42, 48, 0x163253, false),
        ("walk_north", 0, 37, 45, 0x254397, false),
        ("walk_north", 0, 39, 48, 0x3B5370, false),
        ("walk_north", 0, 38, 44, 0x5078E7, false),
        ("walk_north", 0, 40, 34, 0x762E21, false),
        ("walk_north", 0, 46, 47, 0x762E21, true),
        ("walk_north", 0, 40, 41, 0x91AEFF, false),
        ("walk_north", 0, 45, 32, 0xB65932, false),
        ("walk_north", 0, 36, 44, 0xBEBC91, false),
        ("walk_north", 0, 43, 34, 0xCC9155, false),
        ("walk_north", 0, 46, 45, 0xD37A57, true),
        ("walk_north", 0, 42, 32, 0xEFCC86, false),
        ("walk_north", 0, 42, 52, 0xF0B988, true),
        ("walk_north", 0, 40, 31, 0xFAECB2, false),
        ("walk_north", 0, 47, 46, 0xFCD9B3, true),
        ("walk_north", 0, 43, 42, 0xFFFDD3, false),
        ("walk_north", 1, 42, 40, 0x762E21, false),
        ("walk_north", 1, 46, 47, 0x762E21, true),
        ("walk_north", 1, 35, 47, 0xD37A57, true),
        ("walk_north", 1, 42, 53, 0xF0B988, true),
        ("walk_north", 1, 34, 47, 0xFCD9B3, true),
        ("walk_north", 2, 40, 34, 0x762E21, false),
        ("walk_north", 2, 46, 47, 0x762E21, true),
        ("walk_north", 2, 46, 45, 0xD37A57, true),
        ("walk_north", 2, 42, 52, 0xF0B988, true),
        ("walk_north", 2, 47, 46, 0xFCD9B3, true),
        ("walk_north", 3, 42, 40, 0x762E21, false),
        ("walk_north", 3, 33, 47, 0x762E21, true),
        ("walk_north", 3, 34, 47, 0xD37A57, true),
        ("walk_north", 3, 37, 53, 0xF0B988, true),
        ("walk_north", 3, 46, 47, 0xFCD9B3, true),
        ("walk_south", 0, 34, 48, 0x000000, false),
        ("walk_south", 0, 37, 48, 0x163253, false),
        ("walk_south", 0, 42, 45, 0x254397, false),
        ("walk_south", 0, 39, 48, 0x3B5370, false),
        ("walk_south", 0, 39, 45, 0x5078E7, false),
        ("walk_south", 0, 40, 29, 0x762E21, false),
        ("walk_south", 0, 46, 47, 0x762E21, true),
        ("walk_south", 0, 41, 42, 0x91AEFF, false),
        ("walk_south", 0, 40, 32, 0xB65932, false),
        ("walk_south", 0, 36, 44, 0xBEBC91, false),
        ("walk_south", 0, 41, 35, 0xC2B9BE, false),
        ("walk_south", 0, 35, 31, 0xCC9155, false),
        ("walk_south", 0, 46, 45, 0xD37A57, true),
        ("walk_south", 0, 37, 37, 0xECF0E9, false),
        ("walk_south", 0, 41, 31, 0xEFCC86, false),
        ("walk_south", 0, 38, 52, 0xF0B988, true),
        ("walk_south", 0, 42, 32, 0xFAECB2, false),
        ("walk_south", 0, 46, 46, 0xFCD9B3, true),
        ("walk_south", 0, 40, 42, 0xFFFDD3, false),
        ("walk_south", 1, 42, 40, 0x762E21, false),
        ("walk_south", 1, 34, 48, 0x762E21, true),
        ("walk_south", 1, 46, 46, 0xD37A57, true),
        ("walk_south", 1, 39, 42, 0xF0B988, true),
        ("walk_south", 1, 34, 47, 0xFCD9B3, true),
        ("walk_south", 2, 40, 29, 0x762E21, false),
        ("walk_south", 2, 46, 47, 0x762E21, true),
        ("walk_south", 2, 46, 45, 0xD37A57, true),
        ("walk_south", 2, 38, 52, 0xF0B988, true),
        ("walk_south", 2, 46, 46, 0xFCD9B3, true),
        ("walk_south", 3, 42, 40, 0x762E21, false),
        ("walk_south", 3, 45, 48, 0x762E21, true),
        ("walk_south", 3, 46, 46, 0xD37A57, true),
        ("walk_south", 3, 39, 42, 0xF0B988, true),
        ("walk_south", 3, 46, 47, 0xFCD9B3, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[41]),
        ("idle_north", &[22]),
        ("idle_south", &[46]),
        ("walk_east", &[41, 42, 41, 41]),
        ("walk_north", &[22, 19, 22, 19]),
        ("walk_south", &[46, 43, 46, 43]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [10, 11, 12, 3]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_nora_spring";
            let asset = format!("assets/animations/NPCs/Nora/Sprites/Spring/{prefix}_{case}.png");
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
                // Preserve dark hair contours, including the lower bobbing tips.
                // Ankles use skin shades; adjacent blue shoes remain unchanged.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 44))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Nora Spring material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin separately from changed pixels.
                if source
                    .iter()
                    .any(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 44))
                {
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
