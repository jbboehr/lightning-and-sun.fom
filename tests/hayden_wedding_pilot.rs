use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/hayden-wedding-finish-study and the local accepted Beach swim baseline"]
fn hayden_wedding_masks_cover_faces_and_hands_and_preserve_suit_and_shoes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/hayden-wedding-finish-study");
    let baseline = root.join("generated/characters-world-beach-swim-trial/characters/hayden");
    let set = std::env::var_os("FOM_HAYDEN_WEDDING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/hayden-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/hayden-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..290];
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
    // Five isolated East cheek pixels use the existing portrait highlight. The
    // darkest world-ramp color is used ONLY for shoes here and stays unchanged.
    // Cuffs, collar, suit, flower, vest, hair and beard retain their materials.
    let landmarks = [
        ("idle_east", 0, 40, 33, 0xE7B172, true),
        ("idle_east", 0, 43, 35, 0xE8B271, true),
        ("idle_east", 0, 42, 31, 0x815A2E, true),
        ("idle_east", 0, 40, 29, 0x3F332D, false),
        ("idle_east", 0, 40, 38, 0x3F332D, false),
        ("idle_east", 0, 40, 40, 0xFDF7E0, false),
        ("idle_east", 0, 34, 45, 0xE7B172, true),
        ("idle_east", 0, 35, 46, 0x815A2E, true),
        ("idle_east", 0, 34, 44, 0xFDF7E0, false),
        ("idle_east", 0, 38, 52, 0x523C26, false),
        ("idle_east", 0, 41, 53, 0x000000, false),
        ("idle_east", 0, 44, 40, 0x904E2F, false),
        ("walk_east", 1, 40, 34, 0xE7B172, true),
        ("walk_east", 1, 43, 36, 0xE8B271, true),
        ("walk_east", 1, 42, 32, 0x815A2E, true),
        ("walk_east", 1, 40, 41, 0xFDF7E0, false),
        ("walk_east", 1, 32, 45, 0xE7B172, true),
        ("walk_east", 1, 33, 46, 0x815A2E, true),
        ("walk_east", 1, 34, 44, 0xFDF7E0, false),
        ("walk_east", 1, 37, 49, 0x523C26, false),
        ("walk_east", 1, 42, 52, 0x523C26, false),
        ("walk_east", 1, 43, 41, 0xEB8244, false),
        ("walk_east", 3, 40, 34, 0xE7B172, true),
        ("walk_east", 3, 43, 36, 0xE8B271, true),
        ("walk_east", 3, 42, 32, 0x815A2E, true),
        ("walk_east", 3, 40, 41, 0xFDF7E0, false),
        ("walk_east", 3, 35, 46, 0xE7B172, true),
        ("walk_east", 3, 37, 47, 0x815A2E, true),
        ("walk_east", 3, 36, 45, 0xFDF7E0, false),
        ("walk_east", 3, 36, 51, 0x523C26, false),
        ("walk_east", 3, 40, 44, 0xE3B449, false),
        ("idle_north", 0, 40, 29, 0x66534A, false),
        ("idle_north", 0, 38, 35, 0x66534A, false),
        ("idle_north", 0, 39, 40, 0xDDD5AC, false),
        ("idle_north", 0, 38, 44, 0xDDD5AC, false),
        ("idle_north", 0, 32, 45, 0xE7B172, true),
        ("idle_north", 0, 34, 46, 0x815A2E, true),
        ("idle_north", 0, 46, 45, 0xAB7E3F, true),
        ("idle_north", 0, 32, 44, 0xFDF7E0, false),
        ("idle_north", 0, 37, 52, 0x523C26, false),
        ("idle_south", 0, 40, 29, 0x3F332D, false),
        ("idle_south", 0, 38, 35, 0x3F332D, false),
        ("idle_south", 0, 39, 40, 0xFDF7E0, false),
        ("idle_south", 0, 38, 44, 0xC98A33, false),
        ("idle_south", 0, 32, 45, 0xE7B172, true),
        ("idle_south", 0, 34, 46, 0x815A2E, true),
        ("idle_south", 0, 46, 45, 0xE7B172, true),
        ("idle_south", 0, 32, 44, 0xFDF7E0, false),
        ("idle_south", 0, 37, 52, 0x523C26, false),
        ("walk_north", 1, 39, 41, 0xDDD5AC, false),
        ("walk_north", 1, 34, 46, 0xE7B172, true),
        ("walk_north", 1, 46, 44, 0xAB7E3F, true),
        ("walk_north", 1, 34, 45, 0xFDF7E0, false),
        ("walk_north", 1, 41, 53, 0x523C26, false),
        ("walk_north", 3, 39, 41, 0xDDD5AC, false),
        ("walk_north", 3, 45, 46, 0xE7B172, true),
        ("walk_north", 3, 45, 45, 0xFDF7E0, false),
        ("walk_north", 3, 38, 53, 0x523C26, false),
        ("walk_south", 1, 39, 41, 0xFDF7E0, false),
        ("walk_south", 1, 34, 46, 0xE7B172, true),
        ("walk_south", 1, 46, 44, 0xAB7E3F, true),
        ("walk_south", 1, 34, 45, 0xFDF7E0, false),
        ("walk_south", 1, 41, 53, 0x523C26, false),
        ("walk_south", 3, 39, 41, 0xFDF7E0, false),
        ("walk_south", 3, 45, 46, 0xE7B172, true),
        ("walk_south", 3, 45, 45, 0xFDF7E0, false),
        ("walk_south", 3, 38, 53, 0x523C26, false),
    ];
    let cases = [
        ("idle_east", 1, 23),
        ("idle_north", 1, 12),
        ("idle_south", 1, 28),
        ("walk_east", 4, 93),
        ("walk_north", 4, 46),
        ("walk_south", 4, 108),
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
        for (case, frames, expected_changed) in cases {
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
                // including all 63 shoe pixels sharing the darkest world shade.
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
