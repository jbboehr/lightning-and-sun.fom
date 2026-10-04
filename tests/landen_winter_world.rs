use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/landen and the local accepted Landen world baseline"]
fn landen_winter_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/landen");
    let baseline = root.join("generated/slice-057-build/characters/landen");
    let set = std::env::var_os("FOM_LANDEN_WINTER_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/landen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/landen-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..98];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..98], prior);
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
    // Reviewed grids separate face and hands from hair and clothing.
    // All four world shades represent skin in these six pinned Winter strips.
    let landmarks = [
        ("idle_east", 0, 37, 48, 0x000000, false),
        ("idle_east", 0, 41, 51, 0x2B1428, false),
        ("idle_east", 0, 41, 42, 0x331F4D, false),
        ("idle_east", 0, 42, 51, 0x462140, false),
        ("idle_east", 0, 35, 46, 0x491F1B, true),
        ("idle_east", 0, 41, 52, 0x533D3E, false),
        ("idle_east", 0, 38, 49, 0x5B2B53, false),
        ("idle_east", 0, 39, 41, 0x5B3672, false),
        ("idle_east", 0, 37, 32, 0x605451, false),
        ("idle_east", 0, 34, 46, 0x63342A, true),
        ("idle_east", 0, 43, 43, 0x6C5849, false),
        ("idle_east", 0, 42, 52, 0x715655, false),
        ("idle_east", 0, 39, 50, 0x814367, false),
        ("idle_east", 0, 39, 43, 0x834A99, false),
        ("idle_east", 0, 45, 45, 0x854D3C, true),
        ("idle_east", 0, 35, 32, 0x857874, false),
        ("idle_east", 0, 38, 44, 0x917A68, false),
        ("idle_east", 0, 42, 53, 0x937772, false),
        ("idle_east", 0, 36, 45, 0x9F6250, true),
        ("idle_east", 0, 43, 35, 0xA59DA2, false),
        ("idle_east", 0, 40, 29, 0xB2A0A4, false),
        ("idle_east", 0, 38, 35, 0xC2B9BE, false),
        ("idle_east", 0, 39, 45, 0xC3AC9B, false),
        ("idle_east", 0, 44, 44, 0xDBC9B1, false),
        ("idle_east", 0, 39, 48, 0xE3A190, false),
        ("idle_east", 0, 43, 36, 0xECF0E9, false),
        ("idle_east", 0, 35, 44, 0xF2E8D6, false),
        ("idle_east", 0, 42, 47, 0xFFE2D6, false),
        ("idle_north", 0, 46, 47, 0x000000, false),
        ("idle_north", 0, 42, 51, 0x2B1428, false),
        ("idle_north", 0, 39, 41, 0x331F4D, false),
        ("idle_north", 0, 41, 51, 0x462140, false),
        ("idle_north", 0, 46, 46, 0x491F1B, true),
        ("idle_north", 0, 37, 53, 0x533D3E, false),
        ("idle_north", 0, 42, 49, 0x5B2B53, false),
        ("idle_north", 0, 40, 40, 0x5B3672, false),
        ("idle_north", 0, 44, 34, 0x605451, false),
        ("idle_north", 0, 43, 43, 0x6C5849, false),
        ("idle_north", 0, 38, 53, 0x715655, false),
        ("idle_north", 0, 38, 50, 0x814367, false),
        ("idle_north", 0, 40, 39, 0x834A99, false),
        ("idle_north", 0, 34, 35, 0x857874, false),
        ("idle_north", 0, 42, 43, 0x917A68, false),
        ("idle_north", 0, 47, 45, 0x9F6250, true),
        ("idle_north", 0, 38, 32, 0xB2A0A4, false),
        ("idle_north", 0, 38, 44, 0xC3AC9B, false),
        ("idle_north", 0, 44, 45, 0xDBC9B1, false),
        ("idle_north", 0, 38, 48, 0xE3A190, false),
        ("idle_north", 0, 45, 44, 0xF2E8D6, false),
        ("idle_north", 0, 41, 47, 0xFFE2D6, false),
        ("idle_south", 0, 47, 47, 0x000000, false),
        ("idle_south", 0, 42, 51, 0x2B1428, false),
        ("idle_south", 0, 40, 42, 0x331F4D, false),
        ("idle_south", 0, 41, 51, 0x462140, false),
        ("idle_south", 0, 46, 46, 0x491F1B, true),
        ("idle_south", 0, 42, 52, 0x533D3E, false),
        ("idle_south", 0, 37, 49, 0x5B2B53, false),
        ("idle_south", 0, 38, 41, 0x5B3672, false),
        ("idle_south", 0, 43, 31, 0x605451, false),
        ("idle_south", 0, 41, 38, 0x63342A, true),
        ("idle_south", 0, 35, 44, 0x6C5849, false),
        ("idle_south", 0, 41, 52, 0x715655, false),
        ("idle_south", 0, 38, 50, 0x814367, false),
        ("idle_south", 0, 38, 43, 0x834A99, false),
        ("idle_south", 0, 37, 33, 0x854D3C, true),
        ("idle_south", 0, 39, 31, 0x857874, false),
        ("idle_south", 0, 37, 44, 0x917A68, false),
        ("idle_south", 0, 41, 53, 0x937772, false),
        ("idle_south", 0, 47, 45, 0x9F6250, true),
        ("idle_south", 0, 35, 31, 0xB2A0A4, false),
        ("idle_south", 0, 42, 35, 0xC2B9BE, false),
        ("idle_south", 0, 44, 43, 0xC3AC9B, false),
        ("idle_south", 0, 44, 45, 0xDBC9B1, false),
        ("idle_south", 0, 38, 48, 0xE3A190, false),
        ("idle_south", 0, 42, 36, 0xECF0E9, false),
        ("idle_south", 0, 45, 44, 0xF2E8D6, false),
        ("idle_south", 0, 41, 47, 0xFFE2D6, false),
        ("walk_east", 0, 37, 48, 0x000000, false),
        ("walk_east", 0, 41, 51, 0x2B1428, false),
        ("walk_east", 0, 41, 42, 0x331F4D, false),
        ("walk_east", 0, 42, 51, 0x462140, false),
        ("walk_east", 0, 35, 46, 0x491F1B, true),
        ("walk_east", 0, 41, 52, 0x533D3E, false),
        ("walk_east", 0, 38, 49, 0x5B2B53, false),
        ("walk_east", 0, 39, 41, 0x5B3672, false),
        ("walk_east", 0, 37, 32, 0x605451, false),
        ("walk_east", 0, 34, 46, 0x63342A, true),
        ("walk_east", 0, 43, 43, 0x6C5849, false),
        ("walk_east", 0, 42, 52, 0x715655, false),
        ("walk_east", 0, 39, 50, 0x814367, false),
        ("walk_east", 0, 39, 43, 0x834A99, false),
        ("walk_east", 0, 45, 45, 0x854D3C, true),
        ("walk_east", 0, 35, 32, 0x857874, false),
        ("walk_east", 0, 38, 44, 0x917A68, false),
        ("walk_east", 0, 42, 53, 0x937772, false),
        ("walk_east", 0, 36, 45, 0x9F6250, true),
        ("walk_east", 0, 43, 35, 0xA59DA2, false),
        ("walk_east", 0, 40, 29, 0xB2A0A4, false),
        ("walk_east", 0, 38, 35, 0xC2B9BE, false),
        ("walk_east", 0, 39, 45, 0xC3AC9B, false),
        ("walk_east", 0, 44, 44, 0xDBC9B1, false),
        ("walk_east", 0, 39, 48, 0xE3A190, false),
        ("walk_east", 0, 43, 36, 0xECF0E9, false),
        ("walk_east", 0, 35, 44, 0xF2E8D6, false),
        ("walk_east", 0, 42, 47, 0xFFE2D6, false),
        ("walk_east", 1, 46, 46, 0x491F1B, true),
        ("walk_east", 1, 32, 46, 0x63342A, true),
        ("walk_east", 1, 45, 46, 0x854D3C, true),
        ("walk_east", 1, 45, 45, 0x9F6250, true),
        ("walk_north", 0, 46, 47, 0x000000, false),
        ("walk_north", 0, 42, 51, 0x2B1428, false),
        ("walk_north", 0, 39, 41, 0x331F4D, false),
        ("walk_north", 0, 41, 51, 0x462140, false),
        ("walk_north", 0, 46, 46, 0x491F1B, true),
        ("walk_north", 0, 37, 53, 0x533D3E, false),
        ("walk_north", 0, 42, 49, 0x5B2B53, false),
        ("walk_north", 0, 40, 40, 0x5B3672, false),
        ("walk_north", 0, 44, 34, 0x605451, false),
        ("walk_north", 0, 43, 43, 0x6C5849, false),
        ("walk_north", 0, 38, 53, 0x715655, false),
        ("walk_north", 0, 38, 50, 0x814367, false),
        ("walk_north", 0, 40, 39, 0x834A99, false),
        ("walk_north", 0, 34, 35, 0x857874, false),
        ("walk_north", 0, 42, 43, 0x917A68, false),
        ("walk_north", 0, 47, 45, 0x9F6250, true),
        ("walk_north", 0, 38, 32, 0xB2A0A4, false),
        ("walk_north", 0, 38, 44, 0xC3AC9B, false),
        ("walk_north", 0, 44, 45, 0xDBC9B1, false),
        ("walk_north", 0, 38, 48, 0xE3A190, false),
        ("walk_north", 0, 45, 44, 0xF2E8D6, false),
        ("walk_north", 0, 41, 47, 0xFFE2D6, false),
        ("walk_north", 1, 46, 46, 0x491F1B, true),
        ("walk_north", 1, 45, 46, 0x63342A, true),
        ("walk_north", 1, 34, 46, 0x9F6250, true),
        ("walk_south", 0, 47, 47, 0x000000, false),
        ("walk_south", 0, 42, 51, 0x2B1428, false),
        ("walk_south", 0, 40, 42, 0x331F4D, false),
        ("walk_south", 0, 41, 51, 0x462140, false),
        ("walk_south", 0, 46, 46, 0x491F1B, true),
        ("walk_south", 0, 42, 52, 0x533D3E, false),
        ("walk_south", 0, 37, 49, 0x5B2B53, false),
        ("walk_south", 0, 38, 41, 0x5B3672, false),
        ("walk_south", 0, 43, 31, 0x605451, false),
        ("walk_south", 0, 41, 38, 0x63342A, true),
        ("walk_south", 0, 35, 44, 0x6C5849, false),
        ("walk_south", 0, 41, 52, 0x715655, false),
        ("walk_south", 0, 38, 50, 0x814367, false),
        ("walk_south", 0, 38, 43, 0x834A99, false),
        ("walk_south", 0, 37, 33, 0x854D3C, true),
        ("walk_south", 0, 39, 31, 0x857874, false),
        ("walk_south", 0, 37, 44, 0x917A68, false),
        ("walk_south", 0, 41, 53, 0x937772, false),
        ("walk_south", 0, 47, 45, 0x9F6250, true),
        ("walk_south", 0, 35, 31, 0xB2A0A4, false),
        ("walk_south", 0, 42, 35, 0xC2B9BE, false),
        ("walk_south", 0, 44, 43, 0xC3AC9B, false),
        ("walk_south", 0, 44, 45, 0xDBC9B1, false),
        ("walk_south", 0, 38, 48, 0xE3A190, false),
        ("walk_south", 0, 42, 36, 0xECF0E9, false),
        ("walk_south", 0, 45, 44, 0xF2E8D6, false),
        ("walk_south", 0, 41, 47, 0xFFE2D6, false),
        ("walk_south", 1, 46, 46, 0x491F1B, true),
        ("walk_south", 1, 46, 45, 0x63342A, true),
        ("walk_south", 1, 37, 34, 0x854D3C, true),
        ("walk_south", 1, 34, 46, 0x9F6250, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[45]),
        ("idle_north", &[12]),
        ("idle_south", &[52]),
        ("walk_east", &[45, 48, 45, 44]),
        ("walk_north", &[12, 10, 12, 10]),
        ("walk_south", &[52, 50, 52, 50]),
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
            let prefix = "spr_npc_landen_winter";
            let asset = format!("assets/animations/NPCs/Landen/Sprites/Winter/{prefix}_{case}.png");
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
                // The reviewed Winter shades belong only to the face and hands.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Landen Winter material mismatch: {id} {case} [{x},{y}]"
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
