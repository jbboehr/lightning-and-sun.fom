use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/landen and the local accepted Landen world baseline"]
fn landen_summer_finish_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/landen");
    let baseline = root.join("local/fixtures/slice-053-build/characters/landen");
    let set = std::env::var_os("FOM_LANDEN_SUMMER_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/landen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/landen-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..73];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..73], prior);
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
    let source = [0x9F6250, 0x854D3C, 0x63342A, 0x491F1B];
    // Reviewed grids separate face, hands and exposed knees from hair and clothing.
    // All four world shades represent skin in these four pinned Summer strips.
    let landmarks = [
        ("hammer_east", 0, 45, 47, 0x000000, false),
        ("hammer_east", 0, 37, 51, 0x19243A, false),
        ("hammer_east", 0, 43, 42, 0x1E1C54, false),
        ("hammer_east", 0, 38, 51, 0x293A5D, false),
        ("hammer_east", 0, 51, 46, 0x305064, false),
        ("hammer_east", 0, 41, 43, 0x333D83, false),
        ("hammer_east", 0, 42, 43, 0x4760AD, false),
        ("hammer_east", 0, 43, 47, 0x491F1B, true),
        ("hammer_east", 0, 37, 33, 0x605451, false),
        ("hammer_east", 0, 52, 46, 0x6099A8, false),
        ("hammer_east", 0, 44, 43, 0x63342A, true),
        ("hammer_east", 0, 42, 42, 0x6F8ADB, false),
        ("hammer_east", 0, 40, 45, 0x854D3C, true),
        ("hammer_east", 0, 35, 33, 0x857874, false),
        ("hammer_east", 0, 51, 42, 0x85CED4, false),
        ("hammer_east", 0, 48, 45, 0x936244, false),
        ("hammer_east", 0, 41, 46, 0x9F6250, true),
        ("hammer_east", 0, 43, 36, 0xA59DA2, false),
        ("hammer_east", 0, 44, 52, 0xA94A39, false),
        ("hammer_east", 0, 40, 30, 0xB2A0A4, false),
        ("hammer_east", 0, 47, 45, 0xBB8151, false),
        ("hammer_east", 0, 38, 36, 0xC2B9BE, false),
        ("hammer_east", 0, 36, 53, 0xD8614C, false),
        ("hammer_east", 0, 38, 49, 0xD96B45, false),
        ("hammer_east", 0, 46, 45, 0xE3B57F, false),
        ("hammer_east", 0, 43, 37, 0xECF0E9, false),
        ("hammer_east", 0, 39, 49, 0xF8A27B, false),
        ("hammer_east", 0, 45, 53, 0xFF9577, false),
        ("hammer_east", 1, 39, 40, 0x491F1B, true),
        ("hammer_east", 1, 38, 40, 0x63342A, true),
        ("hammer_east", 1, 39, 42, 0x854D3C, true),
        ("hammer_east", 1, 38, 41, 0x9F6250, true),
        ("action_east", 0, 44, 48, 0x000000, false),
        ("action_east", 0, 38, 51, 0x19243A, false),
        ("action_east", 0, 38, 44, 0x1E1C54, false),
        ("action_east", 0, 39, 51, 0x293A5D, false),
        ("action_east", 0, 42, 43, 0x333D83, false),
        ("action_east", 0, 43, 43, 0x4760AD, false),
        ("action_east", 0, 40, 47, 0x491F1B, true),
        ("action_east", 0, 38, 33, 0x605451, false),
        ("action_east", 0, 38, 45, 0x63342A, true),
        ("action_east", 0, 39, 43, 0x6F8ADB, false),
        ("action_east", 0, 39, 45, 0x854D3C, true),
        ("action_east", 0, 36, 33, 0x857874, false),
        ("action_east", 0, 40, 46, 0x9F6250, true),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 42, 52, 0xA94A39, false),
        ("action_east", 0, 41, 30, 0xB2A0A4, false),
        ("action_east", 0, 39, 36, 0xC2B9BE, false),
        ("action_east", 0, 38, 53, 0xD8614C, false),
        ("action_east", 0, 42, 49, 0xD96B45, false),
        ("action_east", 0, 44, 37, 0xECF0E9, false),
        ("action_east", 0, 40, 49, 0xF8A27B, false),
        ("action_east", 0, 43, 53, 0xFF9577, false),
        ("action_east", 1, 48, 44, 0x491F1B, true),
        ("action_east", 1, 49, 44, 0x63342A, true),
        ("action_east", 1, 46, 43, 0x854D3C, true),
        ("action_east", 1, 48, 43, 0x9F6250, true),
        ("action_north", 0, 36, 48, 0x000000, false),
        ("action_north", 0, 37, 51, 0x19243A, false),
        ("action_north", 0, 43, 42, 0x1E1C54, false),
        ("action_north", 0, 38, 51, 0x293A5D, false),
        ("action_north", 0, 44, 42, 0x333D83, false),
        ("action_north", 0, 38, 43, 0x4760AD, false),
        ("action_north", 0, 33, 44, 0x491F1B, true),
        ("action_north", 0, 36, 34, 0x605451, false),
        ("action_north", 0, 35, 43, 0x63342A, true),
        ("action_north", 0, 44, 41, 0x6F8ADB, false),
        ("action_north", 0, 45, 43, 0x854D3C, true),
        ("action_north", 0, 37, 35, 0x857874, false),
        ("action_north", 0, 32, 44, 0x9F6250, true),
        ("action_north", 0, 42, 52, 0xA94A39, false),
        ("action_north", 0, 38, 32, 0xB2A0A4, false),
        ("action_north", 0, 37, 53, 0xD8614C, false),
        ("action_north", 0, 37, 48, 0xD96B45, false),
        ("action_north", 0, 41, 47, 0xF8A27B, false),
        ("action_north", 0, 41, 53, 0xFF9577, false),
        ("action_north", 1, 35, 45, 0x491F1B, true),
        ("action_north", 1, 34, 43, 0x63342A, true),
        ("action_north", 1, 37, 48, 0x854D3C, true),
        ("action_north", 1, 34, 45, 0x9F6250, true),
        ("action_north", 2, 37, 48, 0x854D3C, true),
        ("action_north", 3, 37, 48, 0x854D3C, true),
        ("action_north", 4, 37, 48, 0x854D3C, true),
        ("action_north", 6, 37, 49, 0x854D3C, true),
        ("action_south", 0, 35, 48, 0x000000, false),
        ("action_south", 0, 37, 51, 0x19243A, false),
        ("action_south", 0, 43, 44, 0x1E1C54, false),
        ("action_south", 0, 38, 51, 0x293A5D, false),
        ("action_south", 0, 35, 44, 0x333D83, false),
        ("action_south", 0, 38, 44, 0x4760AD, false),
        ("action_south", 0, 46, 46, 0x491F1B, true),
        ("action_south", 0, 43, 32, 0x605451, false),
        ("action_south", 0, 33, 45, 0x63342A, true),
        ("action_south", 0, 39, 44, 0x6F8ADB, false),
        ("action_south", 0, 34, 45, 0x854D3C, true),
        ("action_south", 0, 39, 32, 0x857874, false),
        ("action_south", 0, 33, 46, 0x9F6250, true),
        ("action_south", 0, 42, 52, 0xA94A39, false),
        ("action_south", 0, 35, 32, 0xB2A0A4, false),
        ("action_south", 0, 42, 36, 0xC2B9BE, false),
        ("action_south", 0, 41, 52, 0xD8614C, false),
        ("action_south", 0, 37, 49, 0xD96B45, false),
        ("action_south", 0, 42, 37, 0xECF0E9, false),
        ("action_south", 0, 41, 48, 0xF8A27B, false),
        ("action_south", 0, 41, 53, 0xFF9577, false),
        ("action_south", 1, 35, 46, 0x491F1B, true),
        ("action_south", 1, 44, 44, 0x63342A, true),
        ("action_south", 1, 45, 44, 0x854D3C, true),
        ("action_south", 1, 35, 45, 0x9F6250, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("hammer_east", &[49, 53, 53, 43, 48, 50, 50]),
        ("action_east", &[48, 52, 52, 52, 52, 48, 56]),
        ("action_north", &[21, 13, 15, 13, 15, 22, 24]),
        ("action_south", &[58, 59, 60, 59, 60, 58, 65]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [0, 1, 2, 3]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = if case.starts_with("hammer_") {
                "spr_npc_landen_specialanimation_summer"
            } else {
                "spr_npc_landen_summer"
            };
            let asset = format!("assets/animations/NPCs/Landen/Sprites/Summer/{prefix}_{case}.png");
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
                // Include the exposed knees below the shorts as well as upper skin.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Landen Summer material mismatch: {id} {case} [{x},{y}]"
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
