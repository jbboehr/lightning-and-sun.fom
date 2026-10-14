use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/holt and the local accepted Holt world baseline"]
fn holt_winter_actions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/holt");
    let baseline = root.join("local/fixtures/slice-103-build/characters/holt");
    let set = std::env::var_os("FOM_HOLT_WINTER_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/holt-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/holt-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..122];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..122], prior);
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
    let source = [0xF0CC8E, 0xCD9067, 0x96523A, 0xAD6945];
    // These Winter strips use all four shades exclusively for skin. Unlike
    // Autumn, their footwear does not reuse them and no dark hand contours
    // need #762E21. Hair, moustache, clothing and mouth interiors stay original.
    // South eating frame 2 lowers the hand beside a boot, down to row 49.
    let landmarks = [
        ("blink_east", 1, 39, 35, 0xF0CC8E, true),
        ("blink_east", 0, 44, 35, 0xAD6945, true),
        ("blink_east", 0, 45, 45, 0x622A10, false),
        ("blink_south", 1, 38, 35, 0xF0CC8E, true),
        ("drink_east", 1, 39, 35, 0xF0CC8E, true),
        ("drink_east", 0, 41, 44, 0xF0CC8E, true),
        ("drink_north", 0, 33, 47, 0xCD9067, true),
        ("drink_south", 1, 45, 47, 0xCD9067, true),
        ("eat_east", 1, 41, 38, 0x9E2626, false),
        ("eat_east", 1, 38, 37, 0x762E21, false),
        ("eat_east", 2, 40, 38, 0xF0CC8E, true),
        ("eat_east", 3, 40, 43, 0xCD9067, true),
        ("eat_north", 1, 33, 47, 0xCD9067, true),
        ("eat_south", 0, 36, 48, 0x96523A, true),
        ("eat_south", 1, 37, 49, 0x96523A, true),
        ("eat_south", 1, 38, 50, 0xD37B52, false),
        ("eat_south", 1, 34, 46, 0xE37D45, false),
        ("eat_south", 1, 35, 48, 0xF0CC8E, true),
        ("eat_south", 2, 40, 38, 0xCD9067, true),
        ("sit_east", 0, 41, 48, 0x1D684E, false),
        ("sit_east", 0, 42, 50, 0xD37B52, false),
        ("sit_north", 0, 32, 44, 0xB45732, false),
        ("sit_north", 0, 35, 47, 0x000000, false),
        ("sit_south", 0, 45, 47, 0xCD9067, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[37, 45, 37]),
        ("blink_south", &[42, 50, 42]),
        ("drink_east", &[35, 46, 35]),
        ("drink_north", &[17, 17, 17]),
        ("drink_south", &[45, 56, 45]),
        ("eat_east", &[34, 39, 44, 42, 34]),
        ("eat_north", &[17, 17, 17]),
        ("eat_south", &[45, 50, 54, 59, 45]),
        ("sit_east", &[35]),
        ("sit_north", &[24]),
        ("sit_south", &[45]),
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
            let prefix = "spr_npc_holt_winter";
            let asset = format!("assets/animations/NPCs/Holt/Sprites/Winter/{prefix}_{case}.png");
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
                // Reviewed literal material shades are independent of mask seeds.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Holt Winter material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin even when a Ryis target equals its source.
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
