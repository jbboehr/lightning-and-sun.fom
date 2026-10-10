use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/darcy and the local accepted Darcy world baseline"]
fn darcy_summer_general_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/darcy");
    let baseline = root.join("generated/slice-145-build/characters/darcy");
    let set = std::env::var_os("FOM_DARCY_SUMMER_GENERAL_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/darcy-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/darcy-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..59];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..59], prior);
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
    let source = [0xFCD9B3, 0xF6996C, 0xC76539, 0x7B2806];
    // These three pinned sources use the four world shades exclusively for skin.
    // Face, ears, front neck, arms and moving hands change; gray hair, eye
    // details, green blouse and rear collar, coral clothing and cream footwear
    // stay original. The oracle and literal landmarks do not use mask seeds.
    let landmarks = [
        ("action_east", 0, 37, 36, 0x7B2806, true),
        ("action_east", 0, 41, 42, 0xF6996C, true),
        ("action_east", 0, 40, 42, 0x66BA88, false),
        ("action_east", 0, 38, 53, 0xE6C0A4, false),
        ("action_east", 0, 39, 53, 0xFEF3DE, false),
        ("action_east", 1, 39, 35, 0x7B2806, true),
        ("action_east", 1, 44, 43, 0xF6996C, true),
        ("action_east", 1, 42, 41, 0x66BA88, false),
        ("action_east", 1, 38, 50, 0xE6C0A4, false),
        ("action_east", 1, 38, 51, 0xFEF3DE, false),
        ("action_east", 2, 39, 35, 0x7B2806, true),
        ("action_east", 2, 42, 44, 0xF6996C, true),
        ("action_east", 2, 42, 41, 0x66BA88, false),
        ("action_east", 2, 38, 50, 0xE6C0A4, false),
        ("action_east", 2, 38, 51, 0xFEF3DE, false),
        ("action_east", 3, 39, 35, 0x7B2806, true),
        ("action_east", 3, 44, 43, 0xF6996C, true),
        ("action_east", 3, 42, 41, 0x66BA88, false),
        ("action_east", 3, 38, 50, 0xE6C0A4, false),
        ("action_east", 3, 38, 51, 0xFEF3DE, false),
        ("action_east", 4, 39, 35, 0x7B2806, true),
        ("action_east", 4, 42, 44, 0xF6996C, true),
        ("action_east", 4, 42, 41, 0x66BA88, false),
        ("action_east", 4, 38, 50, 0xE6C0A4, false),
        ("action_east", 4, 38, 51, 0xFEF3DE, false),
        ("action_east", 5, 37, 36, 0x7B2806, true),
        ("action_east", 5, 41, 42, 0xF6996C, true),
        ("action_east", 5, 40, 42, 0x66BA88, false),
        ("action_east", 5, 38, 53, 0xE6C0A4, false),
        ("action_east", 5, 39, 53, 0xFEF3DE, false),
        ("action_east", 6, 36, 35, 0x7B2806, true),
        ("action_east", 6, 35, 44, 0xC76539, true),
        ("action_east", 6, 39, 41, 0x66BA88, false),
        ("action_east", 6, 38, 51, 0xE6C0A4, false),
        ("action_east", 6, 39, 51, 0xFEF3DE, false),
        ("action_north", 0, 46, 42, 0xC76539, true),
        ("action_north", 0, 45, 41, 0x7B2806, true),
        ("action_north", 0, 38, 40, 0x66BA88, false),
        ("action_north", 0, 37, 51, 0xE6C0A4, false),
        ("action_north", 0, 38, 51, 0xFEF3DE, false),
        ("action_north", 1, 35, 43, 0xC76539, true),
        ("action_north", 1, 44, 39, 0xF6996C, true),
        ("action_north", 1, 38, 40, 0x66BA88, false),
        ("action_north", 1, 37, 50, 0xE6C0A4, false),
        ("action_north", 1, 38, 50, 0xFEF3DE, false),
        ("action_north", 2, 35, 43, 0xC76539, true),
        ("action_north", 2, 44, 39, 0xC76539, true),
        ("action_north", 2, 38, 40, 0x66BA88, false),
        ("action_north", 2, 37, 50, 0xE6C0A4, false),
        ("action_north", 2, 38, 50, 0xFEF3DE, false),
        ("action_north", 3, 35, 43, 0xC76539, true),
        ("action_north", 3, 44, 39, 0xF6996C, true),
        ("action_north", 3, 38, 40, 0x66BA88, false),
        ("action_north", 3, 37, 50, 0xE6C0A4, false),
        ("action_north", 3, 38, 50, 0xFEF3DE, false),
        ("action_north", 4, 35, 43, 0xC76539, true),
        ("action_north", 4, 44, 39, 0xC76539, true),
        ("action_north", 4, 38, 40, 0x66BA88, false),
        ("action_north", 4, 37, 50, 0xE6C0A4, false),
        ("action_north", 4, 38, 50, 0xFEF3DE, false),
        ("action_north", 5, 46, 42, 0xC76539, true),
        ("action_north", 5, 45, 41, 0x7B2806, true),
        ("action_north", 5, 38, 40, 0x66BA88, false),
        ("action_north", 5, 37, 51, 0xE6C0A4, false),
        ("action_north", 5, 38, 51, 0xFEF3DE, false),
        ("action_north", 6, 34, 44, 0xC76539, true),
        ("action_north", 6, 44, 44, 0xF6996C, true),
        ("action_north", 6, 38, 40, 0x66BA88, false),
        ("action_north", 6, 37, 51, 0xE6C0A4, false),
        ("action_north", 6, 38, 51, 0xFEF3DE, false),
        ("action_south", 0, 35, 36, 0x7B2806, true),
        ("action_south", 0, 39, 42, 0xF6996C, true),
        ("action_south", 0, 44, 45, 0xF6996C, true),
        ("action_south", 0, 41, 42, 0x66BA88, false),
        ("action_south", 0, 37, 53, 0xFEF3DE, false),
        ("action_south", 1, 35, 35, 0x7B2806, true),
        ("action_south", 1, 35, 44, 0xF6996C, true),
        ("action_south", 1, 44, 45, 0xF6996C, true),
        ("action_south", 1, 41, 41, 0x66BA88, false),
        ("action_south", 1, 37, 51, 0xE6C0A4, false),
        ("action_south", 1, 38, 51, 0xFEF3DE, false),
        ("action_south", 2, 35, 35, 0x7B2806, true),
        ("action_south", 2, 36, 45, 0xF6996C, true),
        ("action_south", 2, 44, 45, 0xF6996C, true),
        ("action_south", 2, 41, 41, 0x66BA88, false),
        ("action_south", 2, 37, 51, 0xE6C0A4, false),
        ("action_south", 2, 38, 51, 0xFEF3DE, false),
        ("action_south", 3, 35, 35, 0x7B2806, true),
        ("action_south", 3, 35, 44, 0xF6996C, true),
        ("action_south", 3, 44, 45, 0xF6996C, true),
        ("action_south", 3, 41, 41, 0x66BA88, false),
        ("action_south", 3, 37, 51, 0xE6C0A4, false),
        ("action_south", 3, 38, 51, 0xFEF3DE, false),
        ("action_south", 4, 35, 35, 0x7B2806, true),
        ("action_south", 4, 36, 45, 0xF6996C, true),
        ("action_south", 4, 44, 45, 0xF6996C, true),
        ("action_south", 4, 41, 41, 0x66BA88, false),
        ("action_south", 4, 37, 51, 0xE6C0A4, false),
        ("action_south", 4, 38, 51, 0xFEF3DE, false),
        ("action_south", 5, 35, 36, 0x7B2806, true),
        ("action_south", 5, 39, 42, 0xF6996C, true),
        ("action_south", 5, 44, 45, 0xF6996C, true),
        ("action_south", 5, 41, 42, 0x66BA88, false),
        ("action_south", 5, 37, 53, 0xFEF3DE, false),
        ("action_south", 6, 35, 35, 0x7B2806, true),
        ("action_south", 6, 34, 44, 0xC76539, true),
        ("action_south", 6, 44, 44, 0xF6996C, true),
        ("action_south", 6, 38, 41, 0x66BA88, false),
        ("action_south", 6, 37, 51, 0xE6C0A4, false),
        ("action_south", 6, 38, 51, 0xFEF3DE, false),
        ("action_east", 0, 40, 31, 0x4F4040, false),
        ("action_east", 0, 39, 29, 0xF26253, false),
        ("action_east", 0, 38, 31, 0x9C1D1D, false),
        ("action_north", 0, 37, 36, 0x4F4040, false),
        ("action_north", 0, 39, 28, 0xF26253, false),
        ("action_north", 0, 39, 30, 0x9C1D1D, false),
        ("action_south", 0, 38, 31, 0x4F4040, false),
        ("action_south", 0, 38, 29, 0xF26253, false),
        ("action_south", 0, 36, 31, 0x9C1D1D, false),
        ("action_east", 1, 47, 43, 0xFCD9B3, true),
        ("action_east", 1, 49, 43, 0xF6996C, true),
        ("action_east", 1, 49, 45, 0xC76539, true),
        ("action_east", 1, 48, 45, 0x7B2806, true),
        ("action_east", 1, 41, 35, 0xC2B9BE, false),
        ("action_east", 1, 41, 36, 0xECF0E9, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[32, 34, 33, 34, 33, 32, 38]),
        ("action_north", &[19, 9, 12, 9, 12, 17, 20]),
        ("action_south", &[43, 39, 39, 39, 39, 43, 47]),
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
            let prefix = "spr_npc_darcy_summer";
            let asset = format!("assets/animations/NPCs/Darcy/Sprites/Summer/{prefix}_{case}.png");
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
                // Cream footwear uses separate material shades and stays original.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Darcy Summer material mismatch: {id} {case} [{x},{y}]"
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
