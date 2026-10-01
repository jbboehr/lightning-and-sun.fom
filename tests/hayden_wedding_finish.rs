use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/hayden and the local accepted Wedding pilot baseline"]
fn hayden_remaining_wedding_masks_preserve_materials_in_blinks_sitting_actions_and_kisses() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/hayden");
    let baseline = root.join("generated/characters-world-wedding-pilot-trial/characters/hayden");
    let set = std::env::var_os("FOM_HAYDEN_WEDDING_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/hayden-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..296];
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
    let source = [0xE7B172, 0xAB7E3F, 0x815A2E, 0xE8B271];
    // Independently reviewed, literal frame-local boundaries. Frames are zero-based.
    // Fourteen isolated East face pixels use the existing portrait highlight. The
    // darkest world-ramp color is used ONLY for shoes here and stays unchanged.
    // Cuffs, collar, suit, flower, vest, hair and beard retain their materials.
    let landmarks = [
        ("blink_east", 1, 38, 32, 0x815A2E, true),
        ("blink_east", 1, 39, 33, 0xE7B172, true),
        ("blink_east", 1, 38, 34, 0x000000, false),
        ("blink_east", 1, 43, 35, 0xE8B271, true),
        ("blink_east", 1, 40, 29, 0x3F332D, false),
        ("blink_east", 1, 34, 44, 0xFDF7E0, false),
        ("blink_east", 1, 34, 45, 0xE7B172, true),
        ("blink_east", 1, 38, 52, 0x523C26, false),
        ("blink_south", 1, 37, 32, 0x815A2E, true),
        ("blink_south", 1, 38, 33, 0xE7B172, true),
        ("blink_south", 1, 37, 34, 0x000000, false),
        ("blink_south", 1, 40, 29, 0x3F332D, false),
        ("blink_south", 1, 32, 44, 0xFDF7E0, false),
        ("blink_south", 1, 32, 45, 0xE7B172, true),
        ("blink_south", 1, 39, 40, 0xFDF7E0, false),
        ("blink_south", 1, 37, 52, 0x523C26, false),
        ("sit_east", 0, 43, 35, 0xE8B271, true),
        ("sit_east", 0, 32, 44, 0xAB7E3F, true),
        ("sit_east", 0, 35, 46, 0xAB7E3F, true),
        ("sit_east", 0, 33, 43, 0xFDF7E0, false),
        ("sit_east", 0, 41, 48, 0x523C26, false),
        ("sit_east", 0, 44, 47, 0x523C26, false),
        ("sit_east", 0, 40, 40, 0xFDF7E0, false),
        ("sit_east", 0, 39, 42, 0x585654, false),
        ("sit_north", 0, 31, 44, 0xAB7E3F, true),
        ("sit_north", 0, 34, 46, 0xE7B172, true),
        ("sit_north", 0, 45, 46, 0xE7B172, true),
        ("sit_north", 0, 48, 44, 0xAB7E3F, true),
        ("sit_north", 0, 32, 43, 0xFDF7E0, false),
        ("sit_north", 0, 40, 35, 0x28201C, false),
        ("sit_north", 0, 39, 40, 0xDDD5AC, false),
        ("sit_north", 0, 39, 46, 0x816F55, false),
        ("sit_south", 0, 31, 44, 0xAB7E3F, true),
        ("sit_south", 0, 34, 46, 0xAB7E3F, true),
        ("sit_south", 0, 45, 46, 0xAB7E3F, true),
        ("sit_south", 0, 48, 44, 0xAB7E3F, true),
        ("sit_south", 0, 47, 43, 0xFDF7E0, false),
        ("sit_south", 0, 37, 49, 0x523C26, false),
        ("sit_south", 0, 39, 40, 0xFDF7E0, false),
        ("sit_south", 0, 39, 44, 0xC98A33, false),
        ("action_east", 0, 44, 36, 0xE8B271, true),
        ("action_east", 0, 40, 46, 0xE7B172, true),
        ("action_east", 0, 41, 47, 0xE7B172, true),
        ("action_east", 0, 39, 45, 0xFDF7E0, false),
        ("action_east", 1, 46, 35, 0xE8B271, true),
        ("action_east", 1, 47, 41, 0xE7B172, true),
        ("action_east", 1, 47, 43, 0x815A2E, true),
        ("action_east", 1, 45, 41, 0xADA278, false),
        ("action_east", 1, 37, 51, 0x523C26, false),
        ("action_east", 1, 42, 52, 0x523C26, false),
        ("action_east", 2, 45, 42, 0xE7B172, true),
        ("action_east", 2, 45, 44, 0x815A2E, true),
        ("action_east", 2, 43, 43, 0xFDF7E0, false),
        ("action_east", 2, 42, 40, 0x000000, false),
        ("action_north", 0, 31, 44, 0xE7B172, true),
        ("action_north", 0, 32, 45, 0x815A2E, true),
        ("action_north", 0, 47, 37, 0x815A2E, true),
        ("action_north", 0, 48, 37, 0x000000, false),
        ("action_north", 0, 32, 43, 0xFDF7E0, false),
        ("action_north", 0, 47, 38, 0xDDD5AC, false),
        ("action_north", 1, 33, 44, 0xE7B172, true),
        ("action_north", 1, 35, 45, 0x815A2E, true),
        ("action_north", 1, 34, 43, 0xFDF7E0, false),
        ("action_north", 1, 37, 53, 0x523C26, false),
        ("action_north", 2, 46, 36, 0xADA278, false),
        ("action_north", 2, 40, 40, 0xDDD5AC, false),
        ("action_south", 2, 38, 45, 0xE7B172, true),
        ("action_south", 2, 39, 46, 0x815A2E, true),
        ("action_south", 2, 45, 44, 0xAB7E3F, true),
        ("action_south", 2, 46, 45, 0x815A2E, true),
        ("action_south", 2, 37, 44, 0xADA278, false),
        ("action_south", 2, 40, 40, 0xFDF7E0, false),
        ("action_south", 2, 37, 52, 0x523C26, false),
        ("action_south", 3, 35, 45, 0xE7B172, true),
        ("action_south", 3, 35, 46, 0x815A2E, true),
        ("action_south", 3, 34, 44, 0xFDF7E0, false),
        ("kiss_east", 0, 42, 36, 0xE8B271, true),
        ("kiss_east", 0, 36, 46, 0xE7B172, true),
        ("kiss_east", 0, 37, 52, 0x523C26, false),
        ("kiss_east", 1, 44, 36, 0xE8B271, true),
        ("kiss_east", 1, 36, 46, 0xE7B172, true),
        ("kiss_east", 2, 42, 35, 0xE8B271, true),
        ("kiss_east", 2, 40, 32, 0x3F332D, false),
        ("kiss_east", 2, 42, 33, 0x000000, false),
        ("kiss_east", 2, 41, 34, 0xAB7E3F, true),
        ("kiss_east", 2, 44, 36, 0x28201C, false),
        ("kiss_east", 2, 35, 45, 0xE7B172, true),
        ("kiss_east", 2, 35, 44, 0xFDF7E0, false),
        ("kiss_east", 2, 36, 51, 0x523C26, false),
        ("kiss_east", 3, 42, 36, 0x66534A, false),
        ("kiss_east", 3, 40, 37, 0x66534A, false),
        ("kiss_east", 3, 35, 46, 0xE7B172, true),
        ("kiss_east", 3, 35, 45, 0xFDF7E0, false),
    ];
    let cases: [(&str, &[usize]); 9] = [
        ("blink_east", &[26, 30, 26]),
        ("blink_south", &[31, 35, 31]),
        ("sit_east", &[23]),
        ("sit_north", &[18]),
        ("sit_south", &[32]),
        ("action_east", &[22, 23, 23, 23, 23, 22, 23]),
        ("action_north", &[8, 6, 6, 6, 6, 8, 12]),
        ("action_south", &[25, 26, 28, 26, 28, 25, 28]),
        ("kiss_east", &[21, 24, 28, 28]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = preset["colors"].as_array().unwrap()[11..14]
            .iter()
            .chain(std::iter::once(&preset["colors"][0]))
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for (case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let asset = format!(
                "assets/animations/NPCs/Hayden/Sprites/Wedding/spr_npc_hayden_wedding_{case}.png"
            );
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
                // Review established these three world shades and the isolated
                // portrait highlight are skin. Every other pixel must stay intact,
                // including all 152 shoe pixels sharing the darkest world shade.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Wedding material mismatch: {id} {case} [{x},{y}]"
                );
                mask.push(p != q);
                if p != q {
                    let index = source
                        .iter()
                        .position(|c| rgba(*c) == p.0)
                        .expect("hair, suit, shoes or another non-skin color changed");
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
        if let Some(first) = &common_mask {
            assert_eq!(first, &mask);
        } else {
            common_mask = Some(mask);
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
