use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/darcy and the local accepted Darcy world baseline"]
fn darcy_autumn_actions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/darcy");
    let baseline = root.join("local/fixtures/slice-147-build/characters/darcy");
    let set = std::env::var_os("FOM_DARCY_AUTUMN_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/darcy-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/darcy-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..68];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..68], prior);
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
    // These six pinned sources use the four world shades exclusively for skin.
    // Face, eyelids, exposed neck and hands change. Hair, lashes, eye details, tan
    // coat, cuffs, teal outfit, boots and mouth interiors stay original. The rear
    // neck is visible above the coat. The oracle and landmarks do not use seeds.
    let landmarks = [
        ("blink_east", 0, 36, 35, 0x7B2806, true),
        ("blink_east", 0, 34, 45, 0xC76539, true),
        ("blink_east", 0, 44, 45, 0xF6996C, true),
        ("blink_east", 0, 40, 42, 0xC69D7C, false),
        ("blink_east", 0, 40, 41, 0xDEC1A8, false),
        ("blink_east", 0, 40, 43, 0xF5F5F5, false),
        ("blink_east", 0, 38, 51, 0x2C1E1E, false),
        ("blink_east", 1, 36, 35, 0x7B2806, true),
        ("blink_east", 1, 34, 45, 0xC76539, true),
        ("blink_east", 1, 44, 45, 0xF6996C, true),
        ("blink_east", 1, 40, 42, 0xC69D7C, false),
        ("blink_east", 1, 40, 41, 0xDEC1A8, false),
        ("blink_east", 1, 40, 43, 0xF5F5F5, false),
        ("blink_east", 1, 38, 51, 0x2C1E1E, false),
        ("blink_east", 2, 36, 35, 0x7B2806, true),
        ("blink_east", 2, 34, 45, 0xC76539, true),
        ("blink_east", 2, 44, 45, 0xF6996C, true),
        ("blink_east", 2, 40, 42, 0xC69D7C, false),
        ("blink_east", 2, 40, 41, 0xDEC1A8, false),
        ("blink_east", 2, 40, 43, 0xF5F5F5, false),
        ("blink_east", 2, 38, 51, 0x2C1E1E, false),
        ("blink_south", 0, 35, 35, 0x7B2806, true),
        ("blink_south", 0, 33, 45, 0xC76539, true),
        ("blink_south", 0, 44, 45, 0xC76539, true),
        ("blink_south", 0, 39, 42, 0xC69D7C, false),
        ("blink_south", 0, 39, 41, 0xDEC1A8, false),
        ("blink_south", 0, 39, 43, 0xF5F5F5, false),
        ("blink_south", 0, 38, 51, 0x4F4040, false),
        ("blink_south", 1, 35, 35, 0x7B2806, true),
        ("blink_south", 1, 33, 45, 0xC76539, true),
        ("blink_south", 1, 44, 45, 0xC76539, true),
        ("blink_south", 1, 39, 42, 0xC69D7C, false),
        ("blink_south", 1, 39, 41, 0xDEC1A8, false),
        ("blink_south", 1, 39, 43, 0xF5F5F5, false),
        ("blink_south", 1, 38, 51, 0x4F4040, false),
        ("blink_south", 2, 35, 35, 0x7B2806, true),
        ("blink_south", 2, 33, 45, 0xC76539, true),
        ("blink_south", 2, 44, 45, 0xC76539, true),
        ("blink_south", 2, 39, 42, 0xC69D7C, false),
        ("blink_south", 2, 39, 41, 0xDEC1A8, false),
        ("blink_south", 2, 39, 43, 0xF5F5F5, false),
        ("blink_south", 2, 38, 51, 0x4F4040, false),
        ("eat_south", 0, 35, 35, 0x7B2806, true),
        ("eat_south", 0, 35, 46, 0xF6996C, true),
        ("eat_south", 0, 44, 45, 0xC76539, true),
        ("eat_south", 0, 39, 42, 0xC69D7C, false),
        ("eat_south", 0, 39, 41, 0xDEC1A8, false),
        ("eat_south", 0, 39, 43, 0xF5F5F5, false),
        ("eat_south", 0, 38, 49, 0x2C1E1E, false),
        ("eat_south", 1, 35, 36, 0x7B2806, true),
        ("eat_south", 1, 36, 47, 0xF6996C, true),
        ("eat_south", 1, 44, 45, 0xC76539, true),
        ("eat_south", 1, 39, 43, 0xC69D7C, false),
        ("eat_south", 1, 39, 42, 0xDEC1A8, false),
        ("eat_south", 1, 39, 44, 0xF5F5F5, false),
        ("eat_south", 1, 41, 49, 0x2C1E1E, false),
        ("eat_south", 2, 37, 33, 0xC76539, true),
        ("eat_south", 2, 36, 40, 0x7B2806, true),
        ("eat_south", 2, 44, 45, 0xC76539, true),
        ("eat_south", 2, 40, 42, 0xC69D7C, false),
        ("eat_south", 2, 40, 41, 0xDEC1A8, false),
        ("eat_south", 2, 39, 43, 0xF5F5F5, false),
        ("eat_south", 2, 38, 49, 0x2C1E1E, false),
        ("eat_south", 3, 35, 36, 0x7B2806, true),
        ("eat_south", 3, 35, 43, 0x7B2806, true),
        ("eat_south", 3, 44, 45, 0xC76539, true),
        ("eat_south", 3, 39, 43, 0xC69D7C, false),
        ("eat_south", 3, 39, 42, 0xDEC1A8, false),
        ("eat_south", 3, 39, 44, 0xF5F5F5, false),
        ("eat_south", 3, 38, 49, 0x2C1E1E, false),
        ("eat_south", 4, 35, 35, 0x7B2806, true),
        ("eat_south", 4, 34, 45, 0xC76539, true),
        ("eat_south", 4, 44, 45, 0xC76539, true),
        ("eat_south", 4, 39, 42, 0xC69D7C, false),
        ("eat_south", 4, 39, 41, 0xDEC1A8, false),
        ("eat_south", 4, 39, 43, 0xF5F5F5, false),
        ("eat_south", 4, 38, 49, 0x2C1E1E, false),
        ("sit_east", 0, 36, 35, 0x7B2806, true),
        ("sit_east", 0, 35, 45, 0xF6996C, true),
        ("sit_east", 0, 40, 42, 0xC69D7C, false),
        ("sit_east", 0, 40, 41, 0xDEC1A8, false),
        ("sit_east", 0, 40, 43, 0xF5F5F5, false),
        ("sit_east", 0, 42, 48, 0x2C1E1E, false),
        ("sit_north", 0, 39, 40, 0xC76539, true),
        ("sit_north", 0, 34, 45, 0xC76539, true),
        ("sit_north", 0, 45, 45, 0xC76539, true),
        ("sit_north", 0, 39, 41, 0xC69D7C, false),
        ("sit_north", 0, 38, 42, 0xDEC1A8, false),
        ("sit_north", 0, 35, 44, 0xC9C9CC, false),
        ("sit_south", 0, 35, 35, 0x7B2806, true),
        ("sit_south", 0, 34, 45, 0xC76539, true),
        ("sit_south", 0, 44, 45, 0xC76539, true),
        ("sit_south", 0, 39, 42, 0xC69D7C, false),
        ("sit_south", 0, 39, 41, 0xDEC1A8, false),
        ("sit_south", 0, 39, 43, 0xF5F5F5, false),
        ("sit_south", 0, 38, 49, 0x2C1E1E, false),
        ("blink_east", 0, 39, 30, 0x4F4040, false),
        ("blink_east", 0, 38, 28, 0x609BA5, false),
        ("blink_south", 0, 38, 30, 0x4F4040, false),
        ("blink_south", 0, 38, 28, 0x609BA5, false),
        ("eat_south", 0, 38, 30, 0x4F4040, false),
        ("eat_south", 0, 38, 28, 0x609BA5, false),
        ("sit_east", 0, 39, 30, 0x4F4040, false),
        ("sit_east", 0, 38, 28, 0x609BA5, false),
        ("sit_north", 0, 37, 36, 0x4F4040, false),
        ("sit_north", 0, 39, 28, 0x609BA5, false),
        ("sit_south", 0, 38, 30, 0x4F4040, false),
        ("sit_south", 0, 38, 28, 0x609BA5, false),
        ("blink_east", 1, 39, 36, 0xF6996C, true),
        ("blink_east", 1, 38, 37, 0x000000, false),
        ("blink_south", 1, 39, 36, 0xF6996C, true),
        ("blink_south", 1, 38, 37, 0x000000, false),
        ("eat_south", 2, 39, 35, 0x410808, false),
        ("eat_south", 2, 39, 36, 0x9E2626, false),
        ("eat_south", 2, 36, 42, 0xF6996C, true),
        ("sit_north", 0, 39, 40, 0xC76539, true),
        ("sit_north", 0, 40, 40, 0xC76539, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[35, 39, 35]),
        ("blink_south", &[45, 49, 45]),
        ("eat_south", &[36, 41, 43, 47, 37]),
        ("sit_east", &[27]),
        ("sit_north", &[10]),
        ("sit_south", &[37]),
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
            let prefix = "spr_npc_darcy_autumn";
            let asset = format!("assets/animations/NPCs/Darcy/Sprites/Autumn/{prefix}_{case}.png");
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
                // Tan coat, white cuffs, teal clothing and dark boots stay original.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Darcy Autumn material mismatch: {id} {case} [{x},{y}]"
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
