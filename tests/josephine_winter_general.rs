use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/josephine and the local fixed Josephine world baseline"]
fn josephine_winter_general_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/josephine");
    let baseline = root.join("local/fixtures/slice-134-build/characters/josephine");
    let set = std::env::var_os("FOM_JOSEPHINE_WINTER_GENERAL_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/josephine-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/josephine-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..151];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..151], prior);
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
    // These three pinned sources use the four world shades exclusively for skin.
    // Face and exposed hands change; hair, eyes, gold earrings, pink clothing and
    // dark leggings and boots stay original. This oracle does not use the component seeds.
    let landmarks = [
        ("action_east", 0, 40, 33, 0x713222, true),
        ("action_east", 0, 39, 47, 0x9B643A, true),
        ("action_east", 0, 41, 42, 0x7E442F, true),
        ("action_east", 0, 44, 44, 0x3A1D24, false),
        ("action_east", 1, 41, 33, 0x713222, true),
        ("action_east", 1, 47, 43, 0x9B643A, true),
        ("action_east", 1, 40, 46, 0x3A1D24, false),
        ("action_east", 2, 41, 33, 0x713222, true),
        ("action_east", 2, 44, 42, 0x9B643A, true),
        ("action_east", 2, 40, 46, 0x3A1D24, false),
        ("action_east", 3, 41, 33, 0x713222, true),
        ("action_east", 3, 47, 43, 0x9B643A, true),
        ("action_east", 3, 40, 46, 0x3A1D24, false),
        ("action_east", 4, 41, 33, 0x713222, true),
        ("action_east", 4, 44, 42, 0x9B643A, true),
        ("action_east", 4, 40, 46, 0x3A1D24, false),
        ("action_east", 5, 40, 33, 0x713222, true),
        ("action_east", 5, 39, 47, 0x9B643A, true),
        ("action_east", 5, 41, 42, 0x7E442F, true),
        ("action_east", 5, 44, 44, 0x3A1D24, false),
        ("action_east", 6, 38, 33, 0x713222, true),
        ("action_east", 6, 34, 46, 0x9B643A, true),
        ("action_east", 6, 40, 42, 0x9B643A, true),
        ("action_east", 6, 38, 46, 0x3A1D24, false),
        ("action_east", 0, 41, 27, 0x5F3737, false),
        ("action_east", 0, 38, 43, 0xE18A8A, false),
        ("action_east", 0, 37, 33, 0xFAC0B5, false),
        ("action_east", 0, 38, 44, 0xA15663, false),
        ("action_east", 0, 42, 46, 0x633438, false),
        ("action_east", 0, 39, 53, 0xEE678F, false),
        ("action_north", 0, 32, 44, 0x9B643A, true),
        ("action_north", 0, 37, 46, 0x3A1D24, false),
        ("action_north", 1, 34, 45, 0x9B643A, true),
        ("action_north", 1, 42, 45, 0x3A1D24, false),
        ("action_north", 2, 34, 45, 0x9B643A, true),
        ("action_north", 2, 42, 45, 0x3A1D24, false),
        ("action_north", 3, 34, 45, 0x9B643A, true),
        ("action_north", 3, 42, 45, 0x3A1D24, false),
        ("action_north", 4, 34, 45, 0x9B643A, true),
        ("action_north", 4, 42, 45, 0x3A1D24, false),
        ("action_north", 5, 33, 46, 0x9B643A, true),
        ("action_north", 5, 37, 46, 0x3A1D24, false),
        ("action_north", 6, 32, 46, 0x9B643A, true),
        ("action_north", 6, 45, 46, 0x9B643A, true),
        ("action_north", 6, 37, 46, 0x3A1D24, false),
        ("action_north", 0, 39, 26, 0x5F3737, false),
        ("action_north", 0, 36, 41, 0xE18A8A, false),
        ("action_north", 0, 35, 34, 0xFAC0B5, false),
        ("action_north", 0, 35, 42, 0xA15663, false),
        ("action_north", 0, 37, 45, 0x633438, false),
        ("action_north", 0, 38, 52, 0xEE678F, false),
        ("action_south", 0, 38, 33, 0x713222, true),
        ("action_south", 0, 39, 42, 0x7E442F, true),
        ("action_south", 0, 40, 42, 0x7E442F, true),
        ("action_south", 0, 37, 47, 0x3A1D24, false),
        ("action_south", 1, 37, 33, 0x713222, true),
        ("action_south", 1, 39, 42, 0x9B643A, true),
        ("action_south", 1, 40, 42, 0x9B643A, true),
        ("action_south", 1, 42, 46, 0x3A1D24, false),
        ("action_south", 2, 37, 33, 0x713222, true),
        ("action_south", 2, 39, 42, 0x9B643A, true),
        ("action_south", 2, 40, 42, 0x9B643A, true),
        ("action_south", 2, 42, 46, 0x3A1D24, false),
        ("action_south", 3, 37, 33, 0x713222, true),
        ("action_south", 3, 39, 42, 0x9B643A, true),
        ("action_south", 3, 40, 42, 0x9B643A, true),
        ("action_south", 3, 42, 46, 0x3A1D24, false),
        ("action_south", 4, 37, 33, 0x713222, true),
        ("action_south", 4, 39, 42, 0x9B643A, true),
        ("action_south", 4, 40, 42, 0x9B643A, true),
        ("action_south", 4, 42, 46, 0x3A1D24, false),
        ("action_south", 5, 38, 33, 0x713222, true),
        ("action_south", 5, 39, 42, 0x7E442F, true),
        ("action_south", 5, 40, 42, 0x7E442F, true),
        ("action_south", 5, 37, 47, 0x3A1D24, false),
        ("action_south", 6, 37, 33, 0x713222, true),
        ("action_south", 6, 39, 42, 0x9B643A, true),
        ("action_south", 6, 40, 42, 0x9B643A, true),
        ("action_south", 6, 37, 46, 0x3A1D24, false),
        ("action_south", 0, 39, 27, 0x5F3737, false),
        ("action_south", 0, 35, 43, 0xE18A8A, false),
        ("action_south", 0, 35, 33, 0xFAC0B5, false),
        ("action_south", 0, 36, 42, 0xA15663, false),
        ("action_south", 0, 37, 46, 0x633438, false),
        ("action_south", 0, 37, 53, 0xEE678F, false),
        ("action_east", 0, 39, 36, 0xC2B9BE, false),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 39, 37, 0xECF0E9, false),
        ("action_east", 0, 37, 40, 0xDD934C, false),
        ("action_east", 0, 37, 41, 0xEDC57E, false),
        ("action_east", 0, 37, 39, 0xFFEAB3, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[41, 42, 42, 42, 42, 41, 45]),
        ("action_north", &[8, 6, 7, 6, 7, 8, 12]),
        ("action_south", &[50, 47, 47, 47, 47, 50, 52]),
    ];
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
            let prefix = "spr_npc_josephine_winter";
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
