use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/josephine and the local fixed Josephine world baseline"]
fn josephine_winter_chopping_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/josephine");
    let baseline = root.join("local/fixtures/slice-138-build/characters/josephine");
    let set = std::env::var_os("FOM_JOSEPHINE_WINTER_CHOPPING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/josephine-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/josephine-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..163];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..163], prior);
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
    let source = [0x9B643A, 0x7E442F, 0x713222, 0x57271D];
    // This pinned source uses the four world shades exclusively for skin.
    // Visible hands change; hair, earrings, clothing, boots, knife and
    // food particles stay original. The oracle does not use mask seeds.
    let landmarks = [
        ("chop_north", 0, 45, 44, 0x7E442F, true),
        ("chop_north", 0, 37, 51, 0x3A1D24, false),
        ("chop_north", 1, 46, 38, 0x7E442F, true),
        ("chop_north", 1, 36, 50, 0x3A1D24, false),
        ("chop_north", 2, 36, 50, 0x3A1D24, false),
        ("chop_north", 3, 46, 41, 0x713222, true),
        ("chop_north", 3, 36, 51, 0x3A1D24, false),
        ("chop_north", 4, 45, 42, 0x7E442F, true),
        ("chop_north", 4, 36, 51, 0x3A1D24, false),
        ("chop_north", 5, 45, 41, 0x7E442F, true),
        ("chop_north", 5, 36, 50, 0x3A1D24, false),
        ("chop_north", 6, 36, 50, 0x3A1D24, false),
        ("chop_north", 7, 46, 41, 0x713222, true),
        ("chop_north", 7, 36, 51, 0x3A1D24, false),
        ("chop_north", 8, 45, 42, 0x7E442F, true),
        ("chop_north", 8, 36, 51, 0x3A1D24, false),
        ("chop_north", 9, 45, 41, 0x7E442F, true),
        ("chop_north", 9, 36, 50, 0x3A1D24, false),
        ("chop_north", 10, 36, 50, 0x3A1D24, false),
        ("chop_north", 11, 46, 41, 0x713222, true),
        ("chop_north", 11, 36, 51, 0x3A1D24, false),
        ("chop_north", 12, 45, 42, 0x7E442F, true),
        ("chop_north", 12, 36, 51, 0x3A1D24, false),
        ("chop_north", 13, 45, 41, 0x7E442F, true),
        ("chop_north", 13, 36, 50, 0x3A1D24, false),
        ("chop_north", 14, 36, 50, 0x3A1D24, false),
        ("chop_north", 15, 46, 41, 0x713222, true),
        ("chop_north", 15, 36, 51, 0x3A1D24, false),
        ("chop_north", 16, 45, 42, 0x7E442F, true),
        ("chop_north", 16, 36, 51, 0x3A1D24, false),
        ("chop_north", 17, 45, 41, 0x7E442F, true),
        ("chop_north", 17, 36, 50, 0x3A1D24, false),
        ("chop_north", 18, 45, 44, 0x7E442F, true),
        ("chop_north", 18, 37, 51, 0x3A1D24, false),
        ("chop_north", 19, 45, 46, 0x9B643A, true),
        ("chop_north", 19, 37, 50, 0x3A1D24, false),
        ("chop_north", 0, 33, 42, 0x713222, true),
        ("chop_north", 18, 33, 42, 0x713222, true),
        ("chop_north", 19, 32, 46, 0x9B643A, true),
        ("chop_north", 0, 39, 27, 0x5F3737, false),
        ("chop_north", 0, 36, 42, 0xE18A8A, false),
        ("chop_north", 0, 37, 42, 0xFAC0B5, false),
        ("chop_north", 0, 37, 52, 0xC34272, false),
        ("chop_north", 0, 35, 41, 0xEDC57E, false),
        ("chop_north", 0, 35, 40, 0xDD934C, false),
        ("chop_north", 1, 47, 29, 0x8A9B9E, false),
        ("chop_north", 1, 48, 30, 0xCEE9EE, false),
        ("chop_north", 1, 47, 35, 0x423C39, false),
        ("chop_north", 3, 49, 34, 0x3B6D5B, false),
        ("chop_north", 3, 50, 35, 0x68CCA7, false),
        ("chop_north", 3, 51, 35, 0x82E5C0, false),
    ];
    let cases: &[(&str, &[usize])] = &[(
        "chop_north",
        &[8, 5, 0, 2, 3, 3, 0, 2, 3, 3, 0, 2, 3, 3, 0, 2, 3, 3, 8, 12],
    )];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [0, 9, 10, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_josephine_specialanimation_winter";
            let asset =
                format!("assets/animations/NPCs/Josephine/Sprites/Winter/{prefix}_{case}.png");
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
                // Winter leggings and boots stay original; no ankles are exposed.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Josephine Winter material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin independently of the target colors.
                if source.iter().any(|c| rgba(*c) == p.0) {
                    let index = source
                        .iter()
                        .position(|c| rgba(*c) == p.0)
                        .expect("non-skin material changed");
                    assert_eq!(q.0, rgba(target[index]));
                    per_frame[x as usize / 80] += 1;
                }
            }
            // Four chopping frames have no exposed skin in the Winter outfit.
            // Their zero counts and every unchanged pixel are checked independently.
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
