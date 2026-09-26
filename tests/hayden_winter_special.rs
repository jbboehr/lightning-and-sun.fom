use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/hayden-beach-swim-study and the local accepted Winter standard baseline"]
fn hayden_winter_special_masks_cover_moved_fingers_and_preserve_props() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-beach-swim-study");
    let baseline = root.join("generated/characters-world-winter-standard-trial/characters/hayden");
    let set = root.join("palettes/sets/hayden-world-trial.json");
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..268];
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
    // Independently reviewed, literal frame-local boundaries. Frames are zero-based.
    // Gold Winter sleeves share mid/dark skin shades. Keep their shadows and
    // cream cuffs, while detached reading fingers and exposed necklines change.
    let landmarks = [
        ("hammer_east", 0, 39, 43, 0xC6994D, false),
        ("hammer_east", 0, 53, 39, 0x85CED4, false),
        ("hammer_east", 1, 35, 33, 0xAB7E3F, true),
        ("hammer_east", 1, 37, 33, 0x815A2E, true),
        ("hammer_east", 1, 38, 44, 0xC6994D, false),
        ("hammer_east", 2, 51, 28, 0xC5E3E4, false),
        ("hammer_east", 2, 47, 43, 0x3D3C3A, false),
        ("harvest_east", 2, 42, 48, 0xE5BA5D, false),
        ("harvest_east", 2, 47, 52, 0x815A2E, true),
        ("harvest_east", 3, 48, 53, 0x815A2E, true),
        ("harvest_east", 3, 43, 46, 0xFFF5DA, false),
        ("read_sit_start_south", 0, 32, 43, 0xFFF5DA, false),
        ("read_sit_start_south", 0, 36, 43, 0xFFF5DA, false),
        ("read_sit_start_south", 1, 39, 39, 0xF6E4D7, false),
        ("read_sit_start_south", 1, 44, 43, 0xFFF5DA, false),
        ("read_sit_start_south", 2, 46, 44, 0xFFF5DA, false),
        ("read_sit_loop_south", 0, 38, 39, 0xE7B172, true),
        ("read_sit_loop_south", 0, 39, 40, 0xE7B172, true),
        ("read_sit_loop_south", 0, 34, 39, 0xC9AF9C, false),
        ("read_sit_loop_south", 0, 33, 40, 0x693F22, false),
        ("read_sit_loop_south", 0, 34, 41, 0x9A5F1B, false),
        ("read_sit_loop_south", 2, 42, 38, 0x384C92, false),
        ("read_sit_end_south", 0, 46, 44, 0xFFF5DA, false),
        ("read_sit_end_south", 0, 35, 38, 0xF6E4D7, false),
        ("read_sit_end_south", 1, 38, 45, 0x9A5F1B, false),
        ("read_sit_end_south", 2, 43, 43, 0xFFF5DA, false),
        ("till_east", 0, 41, 47, 0x815A2E, true),
        ("till_east", 1, 50, 43, 0xA57333, false),
        ("till_east", 1, 51, 47, 0x7C4A53, false),
        ("till_east", 1, 56, 47, 0xDDEAF6, false),
        ("water_east", 0, 38, 44, 0xC6994D, false),
        ("water_east", 0, 47, 39, 0xBE6D44, false),
        ("water_east", 1, 39, 44, 0xC6994D, false),
        ("water_east", 1, 51, 37, 0xDD9D3E, false),
        ("water_east", 1, 52, 37, 0xFFF672, false),
        ("wipebrow_south", 0, 39, 31, 0xE7B172, true),
        ("wipebrow_south", 0, 40, 35, 0xD8BB9C, false),
        ("wipebrow_south", 0, 40, 28, 0x66534A, false),
        ("wipebrow_south", 1, 43, 35, 0xFFF5DA, false),
        ("wipebrow_south", 2, 44, 35, 0xD8BB9C, false),
        ("wipebrow_south", 2, 36, 45, 0xFFF5DA, false),
        ("wipebrow_south", 4, 36, 44, 0xFFF5DA, false),
        ("wipebrow_south", 5, 45, 46, 0x815A2E, true),
        ("read_sit_start_south", 0, 31, 44, 0xAB7E3F, true),
        ("read_sit_end_south", 2, 31, 44, 0xAB7E3F, true),
        ("water_east", 0, 40, 44, 0xC6994D, false),
        ("water_east", 2, 40, 44, 0xC6994D, false),
        ("hammer_east", 0, 44, 40, 0xE7B172, true),
        ("hammer_east", 2, 40, 42, 0x815A2E, false),
        ("hammer_east", 2, 41, 42, 0xAB7E3F, false),
        ("hammer_east", 2, 39, 44, 0x815A2E, false),
        ("hammer_east", 3, 40, 46, 0xAB7E3F, false),
        ("hammer_east", 4, 41, 46, 0x815A2E, false),
        ("hammer_east", 5, 42, 43, 0x6E4922, false),
        ("harvest_east", 0, 41, 44, 0xE7B172, true),
        ("harvest_east", 8, 42, 44, 0xE7B172, true),
        ("read_sit_start_south", 0, 39, 40, 0xE7B172, true),
        ("read_sit_start_south", 0, 44, 41, 0x6E4922, false),
        ("read_sit_loop_south", 1, 39, 41, 0xE7B172, true),
        ("read_sit_loop_south", 2, 39, 40, 0xE7B172, true),
        ("read_sit_end_south", 2, 39, 40, 0xE7B172, true),
        ("till_east", 0, 44, 41, 0xE7B172, true),
        ("till_east", 1, 42, 42, 0xAB7E3F, false),
        ("till_east", 1, 42, 43, 0x815A2E, false),
        ("till_east", 1, 39, 45, 0x815A2E, false),
        ("till_east", 1, 41, 46, 0x815A2E, false),
        ("till_east", 2, 41, 45, 0xAB7E3F, false),
        ("till_east", 3, 38, 44, 0xAB7E3F, false),
        ("till_east", 3, 42, 40, 0xE7B172, true),
        ("till_east", 4, 37, 42, 0x6E4922, false),
        ("wipebrow_south", 0, 39, 41, 0xE7B172, true),
        ("wipebrow_south", 1, 42, 39, 0xAB7E3F, false),
        ("wipebrow_south", 2, 39, 41, 0xE7B172, true),
        ("wipebrow_south", 3, 39, 41, 0xE7B172, true),
        ("wipebrow_south", 4, 39, 40, 0xE7B172, true),
        ("wipebrow_south", 5, 44, 41, 0x6E4922, false),
        ("water_east", 0, 41, 41, 0xAB7E3F, true),
        ("water_east", 0, 42, 41, 0xE7B172, true),
        ("water_east", 1, 49, 39, 0xAB7E3F, true),
        ("read_sit_loop_south", 2, 41, 39, 0xE7B172, true),
    ];
    let cases = [
        ("hammer_east", 6, 146),
        ("harvest_east", 10, 270),
        ("read_sit_start_south", 3, 88),
        ("read_sit_loop_south", 4, 86),
        ("read_sit_end_south", 3, 88),
        ("till_east", 5, 116),
        ("water_east", 4, 91),
        ("wipebrow_south", 6, 206),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = preset["colors"].as_array().unwrap()[11..15]
            .iter()
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for (case, frames, expected_changed) in cases {
            let asset = format!(
                "assets/animations/NPCs/Hayden/Sprites/Winter/spr_npc_hayden_specialanimation_winter_{case}.png"
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
                mask.push(p != q);
                if p != q {
                    let index = source
                        .iter()
                        .position(|c| rgba(*c) == p.0)
                        .expect("hat, hair, clothing or another non-skin color changed");
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
