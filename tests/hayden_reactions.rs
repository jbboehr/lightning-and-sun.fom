use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 168 local animations in extracted/hayden-special-study"]
fn hayden_reaction_masks_preserve_books_and_shirt_edges_around_visible_skin() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-special-study");
    let set = std::env::var_os("FOM_HAYDEN_REACTIONS_PRESETS")
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
        ("read_sit_start_south", 1, 35, 41, 0x815A2E, false),
        ("read_sit_start_south", 1, 34, 43, 0x523C26, true),
        ("read_sit_start_south", 1, 37, 40, 0xCC7F21, false),
        ("read_sit_start_south", 1, 39, 40, 0xF6E4D7, false),
        ("read_sit_start_south", 1, 39, 47, 0xFCA804, false),
        ("read_sit_loop_south", 0, 37, 38, 0xE7B172, true),
        ("read_sit_loop_south", 0, 45, 38, 0xAB7E3F, false),
        ("read_sit_loop_south", 0, 39, 43, 0xC9AF9C, false),
        ("read_sit_loop_south", 0, 44, 44, 0x9A5F1B, false),
        ("read_sit_loop_south", 2, 42, 38, 0xE7B172, true),
        ("read_sit_loop_south", 2, 44, 38, 0x815A2E, false),
        ("read_sit_end_south", 0, 33, 44, 0x523C26, true),
        ("read_sit_end_south", 1, 44, 41, 0x815A2E, false),
        ("shocked_start_south", 0, 34, 43, 0x815A2E, true),
        ("shocked_start_south", 0, 36, 43, 0x815A2E, false),
        ("shocked_loop_south", 0, 34, 35, 0x523C26, true),
        ("shocked_loop_south", 0, 35, 35, 0x815A2E, false),
        ("shocked_loop_south", 0, 45, 35, 0x523C26, true),
        ("shocked_loop_south", 0, 44, 35, 0x815A2E, false),
        ("shocked_loop_south", 0, 37, 33, 0xE8B271, true),
        ("shocked_loop_south", 0, 42, 33, 0xE8B271, true),
        ("shocked_loop_south", 0, 39, 35, 0x9E2626, false),
        ("shocked_loop_south", 0, 39, 36, 0x3F332D, false),
        ("shocked_end_south", 0, 34, 42, 0x523C26, true),
    ];
    let cases = [
        ("read_sit_start_south", 3, 119),
        ("read_sit_loop_south", 4, 88),
        ("read_sit_end_south", 3, 119),
        ("shocked_start_south", 1, 62),
        ("shocked_loop_south", 1, 60),
        ("shocked_end_south", 1, 62),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let mut target: Vec<_> = preset["colors"].as_array().unwrap()[11..15]
            .iter()
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        // The shocked-loop cheeks reuse two existing portrait highlight pixels.
        let portrait_highlight =
            u32::from_str_radix(&preset["colors"][0].as_str().unwrap()[1..7], 16).unwrap();
        target.push(portrait_highlight);
        let mut mask = Vec::new();
        for (case, frames, expected_changed) in cases {
            let family = if case.starts_with("read_sit_") {
                "specialanimation_spring"
            } else {
                "spring"
            };
            let asset = format!(
                "assets/animations/NPCs/Hayden/Sprites/Spring/spr_npc_hayden_{family}_{case}.png"
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
                        .expect("hair, clothing, book, mouth or another non-skin color changed");
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
