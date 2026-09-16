use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 180 local animations in extracted/hayden-outfit-actions-study"]
fn hayden_standard_masks_preserve_shirt_folds_and_cover_kissing_cheek_skin() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-outfit-actions-study");
    let set = std::env::var_os("FOM_HAYDEN_STANDARD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/hayden-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
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
    let source = [0xE7B172, 0xAB7E3F, 0x815A2E, 0x523C26, 0xE8B271];
    // Literal source landmarks distinguish hands/forearms from adjacent same-ramp
    // sleeve and torso folds. Frames are zero-based, coordinates are frame-local.
    let landmarks = [
        ("action_east", 0, 36, 42, 0x815A2E, true),
        ("action_east", 0, 40, 43, 0xAB7E3F, false),
        ("action_east", 1, 42, 40, 0x815A2E, true),
        ("action_east", 1, 39, 41, 0x815A2E, false),
        ("action_east", 2, 41, 41, 0x815A2E, true),
        ("action_east", 2, 39, 43, 0x815A2E, false),
        ("action_north", 0, 32, 40, 0x815A2E, true),
        ("action_north", 0, 35, 40, 0xAB7E3F, false),
        ("action_north", 1, 45, 38, 0xAB7E3F, false),
        ("action_north", 2, 46, 39, 0x815A2E, true),
        ("action_north", 2, 44, 39, 0xAB7E3F, false),
        ("action_south", 1, 36, 43, 0x815A2E, true),
        ("action_south", 1, 37, 42, 0x815A2E, false),
        ("action_south", 2, 36, 41, 0x815A2E, true),
        ("action_south", 2, 37, 40, 0xAB7E3F, false),
        ("kiss_east", 0, 36, 45, 0x815A2E, true),
        ("kiss_east", 0, 37, 43, 0x815A2E, false),
        ("kiss_east", 2, 41, 35, 0xE8B271, true),
        ("kiss_east", 2, 41, 38, 0xE7B172, true),
        ("kiss_east", 2, 40, 41, 0xAB7E3F, false),
        ("sleep_east", 0, 38, 39, 0x815A2E, true),
        ("sleep_east", 0, 38, 41, 0x523C26, false),
    ];
    let cases = [
        ("action_east", 7, 267),
        ("action_north", 7, 191),
        ("action_south", 7, 346),
        ("kiss_east", 4, 166),
        ("sleep_east", 1, 45),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let mut target: Vec<_> = preset["colors"].as_array().unwrap()[11..15]
            .iter()
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        // The existing portrait highlight occurs once in the kissing cheek.
        let portrait_highlight =
            u32::from_str_radix(&preset["colors"][0].as_str().unwrap()[1..7], 16).unwrap();
        target.push(portrait_highlight);
        let mut mask = Vec::new();
        for (case, frames, expected_changed) in cases {
            let asset = format!(
                "assets/animations/NPCs/Hayden/Sprites/Spring/spr_npc_hayden_spring_{case}.png"
            );
            let before = image::open(original.join(&asset)).unwrap().to_rgba8();
            let variant = output.join("variants").join(id);
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
                mask.push(p != q);
                if p != q {
                    let index = source
                        .iter()
                        .position(|c| rgba(*c) == p.0)
                        .expect("hair, clothing, mouth or another non-skin color changed");
                    assert_eq!(q.0, rgba(target[index]));
                    per_frame[x as usize / 80] += 1;
                }
            }
            assert!(per_frame.iter().all(|n| *n > 0), "empty frame in {case}");
            assert_eq!(per_frame.iter().sum::<usize>(), expected_changed, "{case}");
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
    }
}
