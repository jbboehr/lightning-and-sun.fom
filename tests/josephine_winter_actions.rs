use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/josephine and the local fixed Josephine world baseline"]
fn josephine_winter_actions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/josephine");
    let baseline = root.join("generated/slice-133-build/characters/josephine");
    let set = std::env::var_os("FOM_JOSEPHINE_WINTER_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/josephine-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/josephine-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..140];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..140], prior);
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
    // These eleven pinned sources use the four world shades exclusively for skin.
    // Eyelids and moving hands change; lashes and red/dark mouth interiors stay
    // original. The portrait shade #3A1D24 is dark clothing here and is preserved.
    // This whole-image oracle and literal landmarks do not use component seeds.
    let landmarks = [
        ("blink_east", 0, 40, 40, 0x713222, true),
        ("blink_east", 0, 38, 46, 0x3A1D24, false),
        ("blink_east", 1, 40, 40, 0x713222, true),
        ("blink_east", 1, 38, 46, 0x3A1D24, false),
        ("blink_east", 2, 40, 40, 0x713222, true),
        ("blink_east", 2, 38, 46, 0x3A1D24, false),
        ("blink_east", 0, 40, 26, 0x5F3737, false),
        ("blink_east", 0, 36, 42, 0xE18A8A, false),
        ("blink_east", 0, 36, 32, 0xFAC0B5, false),
        ("blink_east", 0, 37, 41, 0xA15663, false),
        ("blink_south", 0, 39, 40, 0x713222, true),
        ("blink_south", 0, 37, 46, 0x3A1D24, false),
        ("blink_south", 1, 39, 40, 0x713222, true),
        ("blink_south", 1, 37, 46, 0x3A1D24, false),
        ("blink_south", 2, 39, 40, 0x713222, true),
        ("blink_south", 2, 37, 46, 0x3A1D24, false),
        ("blink_south", 0, 39, 26, 0x5F3737, false),
        ("blink_south", 0, 35, 42, 0xE18A8A, false),
        ("blink_south", 0, 35, 32, 0xFAC0B5, false),
        ("blink_south", 0, 36, 41, 0xA15663, false),
        ("drink_east", 0, 40, 40, 0x713222, true),
        ("drink_east", 0, 44, 45, 0x3A1D24, false),
        ("drink_east", 1, 39, 40, 0x9B643A, true),
        ("drink_east", 1, 44, 45, 0x3A1D24, false),
        ("drink_east", 2, 40, 40, 0x713222, true),
        ("drink_east", 2, 44, 45, 0x3A1D24, false),
        ("drink_east", 0, 40, 26, 0x5F3737, false),
        ("drink_east", 0, 37, 43, 0xE18A8A, false),
        ("drink_east", 0, 36, 32, 0xFAC0B5, false),
        ("drink_east", 0, 37, 44, 0xA15663, false),
        ("drink_north", 0, 46, 42, 0x713222, true),
        ("drink_north", 0, 37, 46, 0x3A1D24, false),
        ("drink_north", 1, 33, 46, 0x7E442F, true),
        ("drink_north", 1, 37, 46, 0x3A1D24, false),
        ("drink_north", 2, 46, 42, 0x713222, true),
        ("drink_north", 2, 37, 46, 0x3A1D24, false),
        ("drink_north", 0, 39, 26, 0x5F3737, false),
        ("drink_north", 0, 36, 41, 0xE18A8A, false),
        ("drink_north", 0, 35, 34, 0xFAC0B5, false),
        ("drink_north", 0, 35, 43, 0xA15663, false),
        ("drink_south", 0, 39, 40, 0x713222, true),
        ("drink_south", 0, 39, 46, 0x3A1D24, false),
        ("drink_south", 1, 35, 40, 0x9B643A, true),
        ("drink_south", 1, 39, 46, 0x3A1D24, false),
        ("drink_south", 2, 39, 40, 0x713222, true),
        ("drink_south", 2, 39, 46, 0x3A1D24, false),
        ("drink_south", 0, 39, 26, 0x5F3737, false),
        ("drink_south", 0, 44, 42, 0xE18A8A, false),
        ("drink_south", 0, 35, 32, 0xFAC0B5, false),
        ("drink_south", 0, 36, 41, 0xA15663, false),
        ("eat_east", 0, 40, 40, 0x713222, true),
        ("eat_east", 0, 44, 45, 0x3A1D24, false),
        ("eat_east", 1, 45, 40, 0x7E442F, true),
        ("eat_east", 1, 44, 45, 0x3A1D24, false),
        ("eat_east", 2, 43, 40, 0x9B643A, true),
        ("eat_east", 2, 44, 45, 0x3A1D24, false),
        ("eat_east", 3, 39, 40, 0x713222, true),
        ("eat_east", 3, 42, 46, 0x3A1D24, false),
        ("eat_east", 4, 40, 40, 0x713222, true),
        ("eat_east", 4, 40, 47, 0x3A1D24, false),
        ("eat_east", 0, 40, 26, 0x5F3737, false),
        ("eat_east", 0, 40, 42, 0xE18A8A, false),
        ("eat_east", 0, 36, 32, 0xFAC0B5, false),
        ("eat_east", 0, 40, 43, 0xA15663, false),
        ("eat_north", 0, 46, 42, 0x713222, true),
        ("eat_north", 0, 37, 46, 0x3A1D24, false),
        ("eat_north", 1, 33, 46, 0x7E442F, true),
        ("eat_north", 1, 37, 46, 0x3A1D24, false),
        ("eat_north", 2, 46, 42, 0x713222, true),
        ("eat_north", 2, 37, 46, 0x3A1D24, false),
        ("eat_north", 0, 39, 26, 0x5F3737, false),
        ("eat_north", 0, 36, 41, 0xE18A8A, false),
        ("eat_north", 0, 35, 34, 0xFAC0B5, false),
        ("eat_north", 0, 35, 43, 0xA15663, false),
        ("eat_south", 0, 39, 40, 0x713222, true),
        ("eat_south", 0, 39, 46, 0x3A1D24, false),
        ("eat_south", 1, 38, 40, 0x713222, true),
        ("eat_south", 1, 39, 46, 0x3A1D24, false),
        ("eat_south", 2, 36, 40, 0x57271D, true),
        ("eat_south", 2, 39, 46, 0x3A1D24, false),
        ("eat_south", 3, 38, 40, 0x713222, true),
        ("eat_south", 3, 39, 46, 0x3A1D24, false),
        ("eat_south", 4, 39, 40, 0x713222, true),
        ("eat_south", 4, 39, 46, 0x3A1D24, false),
        ("eat_south", 0, 39, 26, 0x5F3737, false),
        ("eat_south", 0, 35, 42, 0xE18A8A, false),
        ("eat_south", 0, 35, 32, 0xFAC0B5, false),
        ("eat_south", 0, 36, 41, 0xA15663, false),
        ("sit_east", 0, 40, 40, 0x713222, true),
        ("sit_east", 0, 44, 45, 0x3A1D24, false),
        ("sit_east", 0, 40, 26, 0x5F3737, false),
        ("sit_east", 0, 36, 42, 0xE18A8A, false),
        ("sit_east", 0, 36, 32, 0xFAC0B5, false),
        ("sit_east", 0, 37, 41, 0xA15663, false),
        ("sit_north", 0, 33, 46, 0x7E442F, true),
        ("sit_north", 0, 37, 46, 0x3A1D24, false),
        ("sit_north", 0, 39, 26, 0x5F3737, false),
        ("sit_north", 0, 36, 41, 0xE18A8A, false),
        ("sit_north", 0, 35, 34, 0xFAC0B5, false),
        ("sit_north", 0, 35, 43, 0xA15663, false),
        ("sit_south", 0, 39, 40, 0x713222, true),
        ("sit_south", 0, 39, 46, 0x3A1D24, false),
        ("sit_south", 0, 39, 26, 0x5F3737, false),
        ("sit_south", 0, 35, 42, 0xE18A8A, false),
        ("sit_south", 0, 35, 32, 0xFAC0B5, false),
        ("sit_south", 0, 36, 41, 0xA15663, false),
        ("blink_east", 1, 38, 35, 0x9B643A, true),
        ("blink_east", 1, 42, 35, 0x9B643A, true),
        ("blink_east", 1, 38, 37, 0x000000, false),
        ("blink_east", 1, 39, 53, 0xEE678F, false),
        ("blink_south", 1, 38, 35, 0x9B643A, true),
        ("blink_south", 1, 42, 35, 0x9B643A, true),
        ("blink_south", 1, 37, 37, 0x000000, false),
        ("blink_south", 1, 37, 53, 0xEE678F, false),
        ("eat_east", 2, 40, 36, 0x9E2626, false),
        ("eat_east", 2, 40, 35, 0x410808, false),
        ("eat_south", 2, 38, 36, 0x9E2626, false),
        ("eat_south", 2, 38, 35, 0x410808, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[49, 57, 49]),
        ("blink_south", &[56, 64, 56]),
        ("drink_east", &[40, 47, 40]),
        ("drink_north", &[4, 3, 4]),
        ("drink_south", &[51, 63, 51]),
        ("eat_east", &[37, 44, 34, 45, 40]),
        ("eat_north", &[4, 3, 4]),
        ("eat_south", &[51, 59, 47, 66, 50]),
        ("sit_east", &[42]),
        ("sit_north", &[6]),
        ("sit_south", &[50]),
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
