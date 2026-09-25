use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 238 local animations in extracted/hayden-winter-standard-study"]
fn hayden_action_masks_follow_raised_hands_without_recoloring_shirt_folds() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-winter-standard-study");
    let set = std::env::var_os("FOM_HAYDEN_ACTIONS_PRESETS")
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
    let source = [0xE7B172, 0xAB7E3F, 0x815A2E, 0x523C26];
    // Literal source landmarks distinguish hands/forearms from adjacent same-ramp
    // sleeve and torso folds. Frames are zero-based, coordinates are frame-local.
    let landmarks = [
        ("blink_east", 1, 38, 32, 0x815A2E, true),
        ("blink_south", 1, 37, 33, 0xAB7E3F, true),
        ("blink_east", 0, 37, 42, 0x815A2E, false),
        ("sit_east", 0, 35, 44, 0x523C26, true),
        ("sit_north", 0, 43, 42, 0x815A2E, false),
        ("sit_south", 0, 35, 38, 0x815A2E, false),
        ("drink_east", 1, 39, 30, 0x815A2E, true),
        ("drink_east", 1, 34, 41, 0xAB7E3F, false),
        ("drink_east", 1, 35, 41, 0x523C26, true),
        ("drink_north", 1, 46, 40, 0x523C26, true),
        ("drink_north", 1, 44, 41, 0xAB7E3F, false),
        ("drink_south", 1, 36, 39, 0x815A2E, true),
        ("drink_south", 1, 44, 41, 0x815A2E, false),
        ("eat_east", 1, 44, 38, 0x523C26, true),
        ("eat_east", 1, 39, 40, 0xAB7E3F, false),
        ("eat_east", 2, 43, 38, 0x523C26, true),
        ("eat_east", 2, 38, 41, 0x523C26, false),
        ("eat_north", 0, 47, 37, 0x815A2E, true),
        ("eat_north", 0, 44, 40, 0xAB7E3F, false),
        ("eat_south", 2, 35, 38, 0x523C26, true),
        ("eat_south", 2, 35, 41, 0xAB7E3F, false),
        ("eat_south", 3, 34, 41, 0x523C26, true),
        ("eat_south", 3, 35, 41, 0xAB7E3F, false),
    ];
    let cases = [
        ("blink_east", 3, 163),
        ("blink_south", 3, 178),
        ("drink_east", 3, 116),
        ("drink_north", 3, 97),
        ("drink_south", 3, 163),
        ("eat_east", 5, 186),
        ("eat_north", 3, 97),
        ("eat_south", 5, 279),
        ("sit_east", 1, 33),
        ("sit_north", 1, 38),
        ("sit_south", 1, 54),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = preset["colors"].as_array().unwrap()[11..15]
            .iter()
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
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
