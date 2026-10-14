use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/landen and the local accepted Landen world baseline"]
fn landen_spring_hammer_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/landen");
    let baseline = root.join("local/fixtures/slice-050-build/characters/landen");
    let set = std::env::var_os("FOM_LANDEN_SPRING_HAMMER_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/landen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/landen-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..55];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..55], prior);
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
    // Reviewed grids distinguish moving skin from hair, scarf, shoes, hammer and swing effects.
    // Skin ends above row 49 in this pinned hammer strip.
    let landmarks = [
        ("hammer_east", 0, 45, 47, 0x000000, false),
        ("hammer_east", 0, 51, 46, 0x305064, false),
        ("hammer_east", 0, 43, 50, 0x332D2C, false),
        ("hammer_east", 0, 43, 47, 0x491F1B, true),
        ("hammer_east", 0, 38, 44, 0x553658, false),
        ("hammer_east", 0, 44, 48, 0x605451, false),
        ("hammer_east", 0, 52, 46, 0x6099A8, false),
        ("hammer_east", 0, 37, 52, 0x63342A, false),
        ("hammer_east", 0, 41, 40, 0x63342A, true),
        ("hammer_east", 0, 42, 45, 0x854D3C, true),
        ("hammer_east", 0, 39, 49, 0x857874, false),
        ("hammer_east", 0, 51, 42, 0x85CED4, false),
        ("hammer_east", 0, 40, 44, 0x88558D, false),
        ("hammer_east", 0, 48, 45, 0x936244, false),
        ("hammer_east", 0, 39, 46, 0x9D76A5, false),
        ("hammer_east", 0, 45, 51, 0x9F6250, false),
        ("hammer_east", 0, 41, 46, 0x9F6250, true),
        ("hammer_east", 0, 43, 36, 0xA59DA2, false),
        ("hammer_east", 0, 40, 30, 0xB2A0A4, false),
        ("hammer_east", 0, 47, 45, 0xBB8151, false),
        ("hammer_east", 0, 38, 36, 0xC2B9BE, false),
        ("hammer_east", 0, 37, 53, 0xE3A190, false),
        ("hammer_east", 0, 46, 45, 0xE3B57F, false),
        ("hammer_east", 0, 43, 37, 0xECF0E9, false),
        ("hammer_east", 0, 38, 42, 0xFFE2D6, false),
        ("hammer_east", 1, 35, 38, 0x491F1B, true),
        ("hammer_east", 1, 37, 52, 0x63342A, false),
        ("hammer_east", 1, 38, 40, 0x63342A, true),
        ("hammer_east", 1, 45, 51, 0x9F6250, false),
        ("hammer_east", 2, 37, 52, 0x63342A, false),
        ("hammer_east", 2, 45, 51, 0x9F6250, false),
        ("hammer_east", 3, 37, 52, 0x63342A, false),
        ("hammer_east", 3, 45, 51, 0x9F6250, false),
        ("hammer_east", 4, 37, 52, 0x63342A, false),
        ("hammer_east", 4, 45, 51, 0x9F6250, false),
        ("hammer_east", 5, 37, 52, 0x63342A, false),
        ("hammer_east", 5, 45, 51, 0x9F6250, false),
        ("hammer_east", 6, 37, 52, 0x63342A, false),
        ("hammer_east", 6, 45, 51, 0x9F6250, false),
    ];
    let cases: &[(&str, &[usize])] = &[("hammer_east", &[48, 45, 45, 40, 45, 45, 45])];
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
            let prefix = "spr_npc_landen_specialanimation_spring";
            let asset = format!("assets/animations/NPCs/Landen/Sprites/Spring/{prefix}_{case}.png");
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
                // In this pinned hammer strip, exposed skin ends above row 49.
                // The shoes below it reuse the same source shades.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && y < 49)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Landen Spring material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin even when a Ryis target equals its source.
                if y < 49 && source.iter().any(|c| rgba(*c) == p.0) {
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
