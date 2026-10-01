use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/eiland-autumn-writing-study and the local accepted Eiland world baseline"]
fn eiland_autumn_writing_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/eiland-autumn-writing-study");
    let baseline = root.join("generated/slice-001-build/characters/eiland");
    let set = std::env::var_os("FOM_EILAND_AUTUMN_WRITING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_AUTUMN_WRITING_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..188];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..188], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 194);
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
    let source = [0xE9A980, 0xDE8F5D, 0xBA6A4C, 0x9C5241, 0x7D3B14];
    // Reviewed landmarks distinguish moving skin from protected materials.
    // Literal per-frame counts guard brief and occluded skin exposure.
    let landmarks = [
        ("read_sit_end_south", 0, 46, 46, 0x000000, false),
        ("read_sit_end_south", 0, 37, 44, 0x405F70, false),
        ("read_sit_end_south", 0, 44, 45, 0x5D878E, false),
        ("read_sit_end_south", 0, 42, 49, 0x685979, false),
        ("read_sit_end_south", 0, 37, 35, 0x6C2859, false),
        ("read_sit_end_south", 0, 35, 44, 0x7BACB5, false),
        ("read_sit_end_south", 0, 35, 47, 0x7D3B14, true),
        ("read_sit_end_south", 0, 40, 34, 0x9C5241, true),
        ("read_sit_end_south", 0, 38, 33, 0xA54E7F, false),
        ("read_sit_end_south", 0, 42, 45, 0xABD4CF, false),
        ("read_sit_end_south", 0, 39, 41, 0xBA6A4C, true),
        ("read_sit_end_south", 0, 38, 44, 0xC9AF9C, false),
        ("read_sit_end_south", 0, 40, 40, 0xDE8F5D, true),
        ("read_sit_end_south", 0, 36, 33, 0xE797AC, false),
        ("read_sit_end_south", 0, 34, 47, 0xE9A980, true),
        ("read_sit_end_south", 0, 41, 50, 0xEDE0EF, false),
        ("read_sit_end_south", 0, 38, 42, 0xF6E4D7, false),
        ("read_sit_end_south", 0, 41, 33, 0xFCDAE0, false),
        ("read_sit_end_south", 0, 43, 32, 0xFFFFFF, false),
        ("read_sit_end_south", 1, 44, 47, 0x7D3B14, true),
        ("read_sit_end_south", 1, 45, 46, 0xE9A980, true),
        ("read_sit_loop_south", 1, 42, 40, 0x7D3B14, true),
        ("read_sit_loop_south", 1, 39, 41, 0xBA6A4C, true),
        ("read_sit_loop_south", 1, 40, 40, 0xDE8F5D, true),
        ("read_sit_start_south", 1, 44, 47, 0x7D3B14, true),
        ("read_sit_start_south", 1, 45, 46, 0xE9A980, true),
        ("write_end_south", 0, 36, 47, 0x000000, false),
        ("write_end_south", 0, 38, 49, 0x2B2432, false),
        ("write_end_south", 0, 37, 49, 0x473C52, false),
        ("write_end_south", 0, 44, 47, 0x57342B, false),
        ("write_end_south", 0, 37, 44, 0x5D457D, false),
        ("write_end_south", 0, 33, 46, 0x663409, false),
        ("write_end_south", 0, 42, 52, 0x685979, false),
        ("write_end_south", 0, 37, 36, 0x6C2859, false),
        ("write_end_south", 0, 41, 44, 0x735E90, false),
        ("write_end_south", 0, 38, 39, 0x7D3B14, true),
        ("write_end_south", 0, 37, 50, 0x927D96, false),
        ("write_end_south", 0, 41, 33, 0x9C5241, true),
        ("write_end_south", 0, 38, 43, 0xA54E7F, false),
        ("write_end_south", 0, 43, 36, 0xA59DA2, false),
        ("write_end_south", 0, 35, 49, 0xAA93C8, false),
        ("write_end_south", 0, 37, 45, 0xAF97B4, false),
        ("write_end_south", 0, 44, 46, 0xB28159, false),
        ("write_end_south", 0, 37, 47, 0xBA6A4C, false),
        ("write_end_south", 0, 41, 45, 0xBBB5C7, false),
        ("write_end_south", 0, 38, 51, 0xC1BDC8, false),
        ("write_end_south", 0, 38, 36, 0xC2B9BE, false),
        ("write_end_south", 0, 40, 48, 0xC3D1DD, false),
        ("write_end_south", 0, 35, 47, 0xC9785A, false),
        ("write_end_south", 0, 44, 42, 0xD2CCDE, false),
        ("write_end_south", 0, 34, 43, 0xD36A0E, false),
        ("write_end_south", 0, 40, 43, 0xDB5C81, false),
        ("write_end_south", 0, 43, 49, 0xDE8F5D, true),
        ("write_end_south", 0, 37, 32, 0xE797AC, false),
        ("write_end_south", 0, 43, 48, 0xE9A980, true),
        ("write_end_south", 0, 43, 37, 0xECF0E9, false),
        ("write_end_south", 0, 37, 53, 0xEDE0EF, false),
        ("write_end_south", 0, 34, 49, 0xF0BC70, false),
        ("write_end_south", 0, 43, 42, 0xF4F4F4, false),
        ("write_end_south", 0, 42, 46, 0xF5F5F5, false),
        ("write_end_south", 0, 33, 43, 0xF9AB6C, false),
        ("write_end_south", 0, 34, 45, 0xFAB680, false),
        ("write_end_south", 0, 42, 32, 0xFCDAE0, false),
        ("write_end_south", 0, 33, 45, 0xFFD8D1, false),
        ("write_end_south", 0, 44, 31, 0xFFFFFF, false),
        ("write_end_south", 1, 42, 40, 0x7D3B14, true),
        ("write_end_south", 1, 39, 41, 0xBA6A4C, true),
        ("write_end_south", 1, 40, 40, 0xDE8F5D, true),
        ("write_loop_south", 1, 41, 40, 0xBA6A4C, true),
        ("write_loop_south", 1, 41, 47, 0xDE8F5D, true),
        ("write_loop_south", 1, 42, 47, 0xE9A980, true),
        ("write_start_south", 0, 42, 48, 0x000000, false),
        ("write_start_south", 0, 38, 50, 0x2B2432, false),
        ("write_start_south", 0, 42, 47, 0x473C52, false),
        ("write_start_south", 0, 42, 45, 0x5D457D, false),
        ("write_start_south", 0, 34, 49, 0x663409, false),
        ("write_start_south", 0, 42, 52, 0x685979, false),
        ("write_start_south", 0, 36, 37, 0x6C2859, false),
        ("write_start_south", 0, 39, 48, 0x735E90, false),
        ("write_start_south", 0, 42, 40, 0x7D3B14, true),
        ("write_start_south", 0, 41, 47, 0x927D96, false),
        ("write_start_south", 0, 40, 34, 0x9C5241, true),
        ("write_start_south", 0, 38, 44, 0xA54E7F, false),
        ("write_start_south", 0, 44, 50, 0xAA93C8, false),
        ("write_start_south", 0, 42, 46, 0xAF97B4, false),
        ("write_start_south", 0, 44, 48, 0xB28159, false),
        ("write_start_south", 0, 39, 41, 0xBA6A4C, true),
        ("write_start_south", 0, 41, 46, 0xBBB5C7, false),
        ("write_start_south", 0, 38, 51, 0xC1BDC8, false),
        ("write_start_south", 0, 42, 49, 0xC3D1DD, false),
        ("write_start_south", 0, 43, 42, 0xC9785A, false),
        ("write_start_south", 0, 44, 43, 0xD2CCDE, false),
        ("write_start_south", 0, 35, 46, 0xD36A0E, false),
        ("write_start_south", 0, 40, 44, 0xDB5C81, false),
        ("write_start_south", 0, 40, 40, 0xDE8F5D, true),
        ("write_start_south", 0, 36, 33, 0xE797AC, false),
        ("write_start_south", 0, 40, 37, 0xE9A980, true),
        ("write_start_south", 0, 37, 53, 0xEDE0EF, false),
        ("write_start_south", 0, 39, 50, 0xF0BC70, false),
        ("write_start_south", 0, 43, 43, 0xF4F4F4, false),
        ("write_start_south", 0, 44, 47, 0xF5F5F5, false),
        ("write_start_south", 0, 34, 46, 0xF9AB6C, false),
        ("write_start_south", 0, 35, 48, 0xFAB680, false),
        ("write_start_south", 0, 41, 33, 0xFCDAE0, false),
        ("write_start_south", 0, 34, 48, 0xFFD8D1, false),
        ("write_start_south", 0, 43, 32, 0xFFFFFF, false),
        ("write_start_south", 1, 43, 49, 0xDE8F5D, true),
        ("write_start_south", 1, 43, 48, 0xE9A980, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("read_sit_end_south", &[40, 25, 33]),
        ("read_sit_loop_south", &[26, 34, 26, 34]),
        ("read_sit_start_south", &[33, 32, 33]),
        ("write_end_south", &[31, 34]),
        ("write_loop_south", &[36, 37, 37, 37]),
        ("write_start_south", &[34, 31]),
    ];
    // Nine individually reviewed clothing pixels share the skin shadow color.
    let clothing: &[(&str, u32, u32, u32)] = &[
        ("read_sit_end_south", 2, 37, 46),
        ("read_sit_end_south", 2, 42, 46),
        ("read_sit_start_south", 0, 37, 46),
        ("read_sit_start_south", 0, 42, 46),
        ("write_end_south", 0, 37, 47),
        ("write_end_south", 1, 37, 48),
        ("write_loop_south", 2, 37, 47),
        ("write_start_south", 0, 37, 48),
        ("write_start_south", 1, 37, 47),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [0, 9, 10, 3, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_eiland_specialanimation_autumn";
            let asset = format!("assets/animations/NPCs/Eiland/Sprites/Autumn/{prefix}_{case}.png");
            let before = image::open(original.join(&asset)).unwrap().to_rgba8();
            let after = image::open(variant.join(&asset)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), (80 * frames, 80));
            assert_eq!(before.dimensions(), after.dimensions());
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(variant.join(meta)).unwrap()
            );
            for &(name, frame, x, y) in clothing {
                if name == case {
                    assert_eq!(before.get_pixel(frame * 80 + x, y).0, rgba(0xBA6A4C));
                }
            }
            let mut per_frame = vec![0; frames as usize];
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                // The shared shadow shade also occurs in the nine clothing pixels above.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !clothing.contains(&(case, x / 80, x % 80, y)))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Eiland Autumn material mismatch: {id} {case} [{x},{y}]"
                );
                mask.push(p != q);
                if p != q {
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
