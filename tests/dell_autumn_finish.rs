use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/dell and the local accepted Dell world baseline"]
fn dell_autumn_finish_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/dell");
    let baseline = root.join("local/fixtures/slice-167-build/characters/dell");
    let set = std::env::var_os("FOM_DELL_AUTUMN_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/dell-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/dell-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..109];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..109], prior);
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
    let source = [0xECC889, 0xDF8D4B, 0xB65932, 0x762E21];
    // Faces, closed eyelids and exposed hands change. Blonde hair, eye details,
    // clothing, boots, stick and motion trail stay original.
    let landmarks = [
        ("swing_stick_east", 0, 40, 39, 0xECC889, true),
        ("swing_stick_east", 0, 35, 48, 0xECC889, true),
        ("swing_stick_east", 0, 36, 48, 0x762E21, true),
        ("swing_stick_east", 0, 44, 48, 0x762E21, true),
        ("swing_stick_east", 0, 45, 48, 0xDF8D4B, true),
        ("swing_stick_east", 0, 37, 32, 0xDF8D4B, false),
        ("swing_stick_east", 0, 38, 34, 0xB65932, false),
        ("swing_stick_east", 0, 34, 32, 0xFFDC90, false),
        ("swing_stick_east", 0, 38, 33, 0xF5F5F5, false),
        ("swing_stick_east", 0, 38, 39, 0xECF0E9, false),
        ("swing_stick_east", 0, 38, 38, 0xC2B9BE, false),
        ("swing_stick_east", 0, 37, 45, 0x444D6E, false),
        ("swing_stick_east", 0, 38, 44, 0x626A8C, false),
        ("swing_stick_east", 0, 38, 45, 0x80AABA, false),
        ("swing_stick_east", 0, 39, 44, 0x561830, false),
        ("swing_stick_east", 0, 40, 45, 0x7F2748, false),
        ("swing_stick_east", 0, 40, 46, 0xDD4655, false),
        ("swing_stick_east", 0, 39, 46, 0xA02A3D, false),
        ("swing_stick_east", 0, 38, 51, 0x653127, false),
        ("swing_stick_east", 0, 39, 52, 0x935436, false),
        ("swing_stick_east", 0, 39, 51, 0xBC8A54, false),
        ("swing_stick_east", 1, 38, 40, 0xECC889, true),
        ("swing_stick_east", 1, 41, 47, 0xECC889, true),
        ("swing_stick_east", 1, 42, 47, 0xECC889, true),
        ("swing_stick_east", 1, 41, 48, 0xDF8D4B, true),
        ("swing_stick_east", 1, 42, 48, 0xB65932, true),
        ("swing_stick_east", 1, 44, 50, 0xDF8D4B, true),
        ("swing_stick_east", 1, 37, 33, 0xDF8D4B, false),
        ("swing_stick_east", 1, 38, 35, 0xB65932, false),
        ("swing_stick_east", 1, 34, 33, 0xFFDC90, false),
        ("swing_stick_east", 1, 38, 34, 0xF5F5F5, false),
        ("swing_stick_east", 1, 38, 45, 0x444D6E, false),
        ("swing_stick_east", 1, 39, 45, 0x626A8C, false),
        ("swing_stick_east", 1, 39, 46, 0x80AABA, false),
        ("swing_stick_east", 1, 39, 50, 0xDD4655, false),
        ("swing_stick_east", 1, 39, 49, 0xA02A3D, false),
        ("swing_stick_east", 1, 38, 51, 0x653127, false),
        ("swing_stick_east", 1, 39, 52, 0x935436, false),
        ("swing_stick_east", 1, 39, 51, 0xBC8A54, false),
        ("swing_stick_east", 1, 43, 36, 0xA8571C, false),
        ("swing_stick_east", 1, 45, 38, 0x712922, false),
        ("swing_stick_east", 2, 40, 39, 0xECC889, true),
        ("swing_stick_east", 2, 41, 45, 0x762E21, true),
        ("swing_stick_east", 2, 41, 46, 0xECC889, true),
        ("swing_stick_east", 2, 42, 46, 0xECC889, true),
        ("swing_stick_east", 2, 41, 47, 0xDF8D4B, true),
        ("swing_stick_east", 2, 42, 47, 0xB65932, true),
        ("swing_stick_east", 2, 44, 49, 0xDF8D4B, true),
        ("swing_stick_east", 2, 37, 32, 0xDF8D4B, false),
        ("swing_stick_east", 2, 38, 34, 0xB65932, false),
        ("swing_stick_east", 2, 34, 32, 0xFFDC90, false),
        ("swing_stick_east", 2, 38, 33, 0xF5F5F5, false),
        ("swing_stick_east", 2, 38, 39, 0xECF0E9, false),
        ("swing_stick_east", 2, 38, 38, 0xC2B9BE, false),
        ("swing_stick_east", 2, 39, 44, 0x444D6E, false),
        ("swing_stick_east", 2, 38, 44, 0x626A8C, false),
        ("swing_stick_east", 2, 38, 45, 0x80AABA, false),
        ("swing_stick_east", 2, 39, 50, 0xDD4655, false),
        ("swing_stick_east", 2, 39, 48, 0xA02A3D, false),
        ("swing_stick_east", 2, 38, 51, 0x653127, false),
        ("swing_stick_east", 2, 39, 52, 0x935436, false),
        ("swing_stick_east", 2, 39, 51, 0xBC8A54, false),
        ("swing_stick_east", 2, 48, 37, 0xA8571C, false),
        ("swing_stick_east", 2, 48, 39, 0x712922, false),
        ("swing_stick_east", 3, 41, 39, 0xECC889, true),
        ("swing_stick_east", 3, 34, 46, 0x762E21, true),
        ("swing_stick_east", 3, 34, 47, 0xDF8D4B, true),
        ("swing_stick_east", 3, 35, 47, 0xECC889, true),
        ("swing_stick_east", 3, 36, 47, 0xECC889, true),
        ("swing_stick_east", 3, 45, 47, 0xDF8D4B, true),
        ("swing_stick_east", 3, 38, 32, 0xDF8D4B, false),
        ("swing_stick_east", 3, 39, 34, 0xB65932, false),
        ("swing_stick_east", 3, 35, 32, 0xFFDC90, false),
        ("swing_stick_east", 3, 39, 33, 0xF5F5F5, false),
        ("swing_stick_east", 3, 39, 39, 0xECF0E9, false),
        ("swing_stick_east", 3, 39, 38, 0xC2B9BE, false),
        ("swing_stick_east", 3, 38, 45, 0x444D6E, false),
        ("swing_stick_east", 3, 39, 44, 0x626A8C, false),
        ("swing_stick_east", 3, 39, 45, 0x80AABA, false),
        ("swing_stick_east", 3, 40, 44, 0x561830, false),
        ("swing_stick_east", 3, 41, 45, 0x7F2748, false),
        ("swing_stick_east", 3, 41, 46, 0xDD4655, false),
        ("swing_stick_east", 3, 40, 46, 0xA02A3D, false),
        ("swing_stick_east", 3, 38, 52, 0x653127, false),
        ("swing_stick_east", 3, 39, 52, 0x935436, false),
        ("swing_stick_east", 3, 39, 53, 0xBC8A54, false),
        ("swing_stick_east", 3, 53, 46, 0xA8571C, false),
        ("swing_stick_east", 3, 52, 41, 0x712922, false),
        ("swing_stick_east", 4, 44, 40, 0xECC889, true),
        ("swing_stick_east", 4, 36, 44, 0x762E21, true),
        ("swing_stick_east", 4, 37, 44, 0xDF8D4B, true),
        ("swing_stick_east", 4, 44, 44, 0xB65932, true),
        ("swing_stick_east", 4, 45, 44, 0xB65932, true),
        ("swing_stick_east", 4, 37, 45, 0xECC889, true),
        ("swing_stick_east", 4, 38, 45, 0xECC889, true),
        ("swing_stick_east", 4, 49, 45, 0xDF8D4B, true),
        ("swing_stick_east", 4, 50, 45, 0xB65932, true),
        ("swing_stick_east", 4, 38, 46, 0xECC889, true),
        ("swing_stick_east", 4, 49, 46, 0x762E21, true),
        ("swing_stick_east", 4, 50, 46, 0xDF8D4B, true),
        ("swing_stick_east", 4, 49, 47, 0xDF8D4B, true),
        ("swing_stick_east", 4, 50, 47, 0xB65932, true),
        ("swing_stick_east", 4, 41, 33, 0xDF8D4B, false),
        ("swing_stick_east", 4, 42, 35, 0xB65932, false),
        ("swing_stick_east", 4, 38, 33, 0xFFDC90, false),
        ("swing_stick_east", 4, 42, 34, 0xF5F5F5, false),
        ("swing_stick_east", 4, 42, 40, 0xECF0E9, false),
        ("swing_stick_east", 4, 42, 39, 0xC2B9BE, false),
        ("swing_stick_east", 4, 41, 45, 0x444D6E, false),
        ("swing_stick_east", 4, 38, 44, 0x626A8C, false),
        ("swing_stick_east", 4, 39, 45, 0x80AABA, false),
        ("swing_stick_east", 4, 43, 45, 0x561830, false),
        ("swing_stick_east", 4, 43, 46, 0x7F2748, false),
        ("swing_stick_east", 4, 43, 47, 0xDD4655, false),
        ("swing_stick_east", 4, 42, 47, 0xA02A3D, false),
        ("swing_stick_east", 4, 44, 51, 0x653127, false),
        ("swing_stick_east", 4, 39, 52, 0x935436, false),
        ("swing_stick_east", 4, 45, 51, 0xBC8A54, false),
        ("swing_stick_east", 4, 36, 46, 0xA8571C, false),
        ("swing_stick_east", 4, 36, 45, 0x712922, false),
        ("swing_stick_east", 5, 44, 39, 0xECC889, true),
        ("swing_stick_east", 5, 36, 43, 0x762E21, true),
        ("swing_stick_east", 5, 37, 43, 0xDF8D4B, true),
        ("swing_stick_east", 5, 44, 43, 0xB65932, true),
        ("swing_stick_east", 5, 45, 43, 0xB65932, true),
        ("swing_stick_east", 5, 36, 44, 0xDF8D4B, true),
        ("swing_stick_east", 5, 37, 44, 0xECC889, true),
        ("swing_stick_east", 5, 38, 44, 0xECC889, true),
        ("swing_stick_east", 5, 49, 44, 0xDF8D4B, true),
        ("swing_stick_east", 5, 50, 44, 0xB65932, true),
        ("swing_stick_east", 5, 38, 45, 0xECC889, true),
        ("swing_stick_east", 5, 49, 45, 0x762E21, true),
        ("swing_stick_east", 5, 50, 45, 0xDF8D4B, true),
        ("swing_stick_east", 5, 49, 46, 0xDF8D4B, true),
        ("swing_stick_east", 5, 50, 46, 0xB65932, true),
        ("swing_stick_east", 5, 41, 51, 0xB65932, true),
        ("swing_stick_east", 5, 41, 32, 0xDF8D4B, false),
        ("swing_stick_east", 5, 42, 34, 0xB65932, false),
        ("swing_stick_east", 5, 38, 32, 0xFFDC90, false),
        ("swing_stick_east", 5, 42, 33, 0xF5F5F5, false),
        ("swing_stick_east", 5, 42, 39, 0xECF0E9, false),
        ("swing_stick_east", 5, 42, 38, 0xC2B9BE, false),
        ("swing_stick_east", 5, 41, 44, 0x444D6E, false),
        ("swing_stick_east", 5, 38, 43, 0x626A8C, false),
        ("swing_stick_east", 5, 39, 44, 0x80AABA, false),
        ("swing_stick_east", 5, 43, 44, 0x561830, false),
        ("swing_stick_east", 5, 43, 45, 0x7F2748, false),
        ("swing_stick_east", 5, 43, 46, 0xDD4655, false),
        ("swing_stick_east", 5, 42, 46, 0xA02A3D, false),
        ("swing_stick_east", 5, 38, 51, 0x653127, false),
        ("swing_stick_east", 5, 40, 51, 0x935436, false),
        ("swing_stick_east", 5, 39, 51, 0xBC8A54, false),
        ("swing_stick_east", 5, 37, 45, 0xA8571C, false),
        ("swing_stick_east", 5, 36, 45, 0x712922, false),
        ("swing_stick_east", 6, 44, 39, 0xECC889, true),
        ("swing_stick_east", 6, 36, 43, 0x762E21, true),
        ("swing_stick_east", 6, 37, 43, 0xDF8D4B, true),
        ("swing_stick_east", 6, 44, 43, 0xB65932, true),
        ("swing_stick_east", 6, 45, 43, 0xB65932, true),
        ("swing_stick_east", 6, 36, 44, 0xDF8D4B, true),
        ("swing_stick_east", 6, 37, 44, 0xECC889, true),
        ("swing_stick_east", 6, 38, 44, 0xECC889, true),
        ("swing_stick_east", 6, 49, 44, 0xDF8D4B, true),
        ("swing_stick_east", 6, 50, 44, 0xB65932, true),
        ("swing_stick_east", 6, 38, 45, 0xECC889, true),
        ("swing_stick_east", 6, 49, 45, 0x762E21, true),
        ("swing_stick_east", 6, 50, 45, 0xDF8D4B, true),
        ("swing_stick_east", 6, 49, 46, 0xDF8D4B, true),
        ("swing_stick_east", 6, 50, 46, 0xB65932, true),
        ("swing_stick_east", 6, 41, 51, 0xB65932, true),
        ("swing_stick_east", 6, 41, 32, 0xDF8D4B, false),
        ("swing_stick_east", 6, 42, 34, 0xB65932, false),
        ("swing_stick_east", 6, 38, 32, 0xFFDC90, false),
        ("swing_stick_east", 6, 42, 33, 0xF5F5F5, false),
        ("swing_stick_east", 6, 42, 39, 0xECF0E9, false),
        ("swing_stick_east", 6, 42, 38, 0xC2B9BE, false),
        ("swing_stick_east", 6, 41, 44, 0x444D6E, false),
        ("swing_stick_east", 6, 38, 43, 0x626A8C, false),
        ("swing_stick_east", 6, 39, 44, 0x80AABA, false),
        ("swing_stick_east", 6, 43, 44, 0x561830, false),
        ("swing_stick_east", 6, 43, 45, 0x7F2748, false),
        ("swing_stick_east", 6, 43, 46, 0xDD4655, false),
        ("swing_stick_east", 6, 42, 46, 0xA02A3D, false),
        ("swing_stick_east", 6, 38, 51, 0x653127, false),
        ("swing_stick_east", 6, 40, 51, 0x935436, false),
        ("swing_stick_east", 6, 39, 51, 0xBC8A54, false),
        ("swing_stick_east", 6, 37, 45, 0xA8571C, false),
        ("swing_stick_east", 6, 36, 45, 0x712922, false),
    ];
    let cases: &[(&str, &[usize])] = &[("swing_stick_east", &[30, 27, 32, 31, 37, 39, 39])];
    // Bounds reviewed against the original grids, independently of component seeds.
    let skin_at = |case: &str, x: u32, y: u32| {
        assert_eq!(case, "swing_stick_east");
        let frame = (x / 80) as usize;
        let x = x % 80;
        let (dx, dy, body) = [
            (0, 0, 44),
            (0, 1, 45),
            (0, 0, 44),
            (1, 0, 44),
            (4, 1, 44),
            (4, 0, 43),
            (4, 0, 43),
        ][frame];
        if y >= body {
            return true;
        }
        let face = if frame == 1 {
            [
                (38, 37, 45),
                (39, 36, 45),
                (40, 36, 45),
                (41, 37, 44),
                (42, 38, 43),
                (43, 40, 42),
            ]
        } else {
            [
                (38, 40, 42),
                (39, 36, 45),
                (40, 36, 45),
                (41, 37, 44),
                (42, 38, 43),
                (43, 40, 42),
            ]
        };
        face.iter()
            .any(|&(row, lo, hi)| y == row + dy && (lo + dx..hi + dx).contains(&x))
    };
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [12, 13, 14, 15]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_dell_specialanimation_autumn";
            let asset = format!("assets/animations/NPCs/Dell/Sprites/Autumn/{prefix}_{case}.png");
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
                // The reviewed face rows separate skin from matching blonde hair.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| {
                        if skin_at(case, x, y) {
                            rgba(target[i])
                        } else {
                            p.0
                        }
                    });
                assert_eq!(
                    q.0, expected,
                    "Dell Autumn material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin independently of target colors.
                if skin_at(case, x, y) && source.iter().any(|c| rgba(*c) == p.0) {
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
