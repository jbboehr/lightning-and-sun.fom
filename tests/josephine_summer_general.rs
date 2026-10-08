use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/josephine and the local accepted Josephine world baseline"]
fn josephine_summer_general_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/josephine");
    let baseline = root.join("generated/slice-118-build/characters/josephine");
    let set = std::env::var_os("FOM_JOSEPHINE_SUMMER_GENERAL_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/josephine-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/josephine-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..85];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..85], prior);
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
    // Face, exposed arms and ankles change; hair, eyes, flower, clothing and
    // shoes stay original. This oracle does not use the component seeds.
    let landmarks = [
        ("action_east", 0, 40, 34, 0x713222, true),
        ("action_east", 0, 39, 42, 0x57271D, true),
        ("action_east", 0, 40, 42, 0x713222, true),
        ("action_east", 1, 42, 33, 0x713222, true),
        ("action_east", 1, 45, 43, 0x7E442F, true),
        ("action_east", 1, 37, 51, 0x713222, true),
        ("action_east", 2, 42, 33, 0x713222, true),
        ("action_east", 2, 44, 42, 0x57271D, true),
        ("action_east", 2, 37, 51, 0x713222, true),
        ("action_east", 3, 42, 33, 0x713222, true),
        ("action_east", 3, 45, 43, 0x7E442F, true),
        ("action_east", 3, 37, 51, 0x713222, true),
        ("action_east", 4, 42, 33, 0x713222, true),
        ("action_east", 4, 44, 42, 0x57271D, true),
        ("action_east", 4, 37, 51, 0x713222, true),
        ("action_east", 5, 40, 34, 0x713222, true),
        ("action_east", 5, 39, 42, 0x57271D, true),
        ("action_east", 5, 40, 42, 0x713222, true),
        ("action_east", 6, 39, 33, 0x713222, true),
        ("action_east", 6, 39, 42, 0x57271D, true),
        ("action_east", 6, 40, 42, 0x57271D, true),
        ("action_east", 6, 38, 52, 0x713222, true),
        ("action_east", 0, 38, 29, 0x5F3737, false),
        ("action_east", 0, 37, 44, 0xE7866A, false),
        ("action_east", 0, 40, 43, 0xF6B486, false),
        ("action_east", 0, 39, 43, 0xFAD8B8, false),
        ("action_east", 0, 43, 45, 0xFFF9F0, false),
        ("action_east", 0, 37, 39, 0xEABF67, false),
        ("action_north", 0, 33, 43, 0x713222, true),
        ("action_north", 0, 46, 42, 0x713222, true),
        ("action_north", 0, 37, 52, 0x7E442F, true),
        ("action_north", 1, 35, 43, 0x57271D, true),
        ("action_north", 1, 37, 51, 0x7E442F, true),
        ("action_north", 2, 35, 43, 0x57271D, true),
        ("action_north", 2, 37, 51, 0x7E442F, true),
        ("action_north", 3, 35, 43, 0x57271D, true),
        ("action_north", 3, 37, 51, 0x7E442F, true),
        ("action_north", 4, 35, 43, 0x57271D, true),
        ("action_north", 4, 37, 51, 0x7E442F, true),
        ("action_north", 5, 34, 44, 0x57271D, true),
        ("action_north", 5, 46, 42, 0x713222, true),
        ("action_north", 5, 37, 52, 0x7E442F, true),
        ("action_north", 6, 34, 44, 0x57271D, true),
        ("action_north", 6, 45, 44, 0x57271D, true),
        ("action_north", 6, 37, 52, 0x7E442F, true),
        ("action_north", 0, 42, 28, 0x5F3737, false),
        ("action_north", 0, 37, 46, 0xE7866A, false),
        ("action_north", 0, 37, 44, 0xF6B486, false),
        ("action_north", 0, 39, 45, 0xFAD8B8, false),
        ("action_north", 0, 41, 44, 0xFFF9F0, false),
        ("action_north", 0, 38, 53, 0xEABF67, false),
        ("action_south", 0, 38, 34, 0x713222, true),
        ("action_south", 0, 37, 42, 0x57271D, true),
        ("action_south", 0, 40, 42, 0x7E442F, true),
        ("action_south", 1, 38, 33, 0x713222, true),
        ("action_south", 1, 38, 42, 0x57271D, true),
        ("action_south", 1, 40, 42, 0x57271D, true),
        ("action_south", 1, 37, 52, 0x713222, true),
        ("action_south", 2, 38, 33, 0x713222, true),
        ("action_south", 2, 38, 42, 0x57271D, true),
        ("action_south", 2, 40, 42, 0x57271D, true),
        ("action_south", 2, 37, 52, 0x713222, true),
        ("action_south", 3, 38, 33, 0x713222, true),
        ("action_south", 3, 38, 42, 0x57271D, true),
        ("action_south", 3, 40, 42, 0x57271D, true),
        ("action_south", 3, 37, 52, 0x713222, true),
        ("action_south", 4, 38, 33, 0x713222, true),
        ("action_south", 4, 38, 42, 0x57271D, true),
        ("action_south", 4, 40, 42, 0x57271D, true),
        ("action_south", 4, 37, 52, 0x713222, true),
        ("action_south", 5, 38, 34, 0x713222, true),
        ("action_south", 5, 37, 42, 0x57271D, true),
        ("action_south", 5, 40, 42, 0x7E442F, true),
        ("action_south", 6, 38, 33, 0x713222, true),
        ("action_south", 6, 38, 42, 0x57271D, true),
        ("action_south", 6, 40, 42, 0x57271D, true),
        ("action_south", 6, 37, 52, 0x713222, true),
        ("action_south", 0, 36, 29, 0x5F3737, false),
        ("action_south", 0, 34, 44, 0xE7866A, false),
        ("action_south", 0, 37, 43, 0xF6B486, false),
        ("action_south", 0, 36, 43, 0xFAD8B8, false),
        ("action_south", 0, 38, 44, 0xFFF9F0, false),
        ("action_south", 0, 35, 39, 0xEABF67, false),
        ("action_east", 0, 39, 36, 0xC2B9BE, false),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 39, 37, 0xECF0E9, false),
        ("action_east", 0, 43, 43, 0xD53E6D, false),
        ("action_east", 0, 44, 43, 0xEF667C, false),
        ("action_east", 0, 40, 53, 0xD68D47, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[49, 51, 51, 51, 51, 49, 56]),
        ("action_north", &[20, 14, 14, 14, 14, 21, 24]),
        ("action_south", &[61, 62, 62, 62, 62, 61, 69]),
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
            let prefix = "spr_npc_josephine_summer";
            let asset =
                format!("assets/animations/NPCs/Josephine/Sprites/Summer/{prefix}_{case}.png");
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
                // Exposed ankles below the hem are skin too; shoes stay original.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Josephine Summer material mismatch: {id} {case} [{x},{y}]"
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
