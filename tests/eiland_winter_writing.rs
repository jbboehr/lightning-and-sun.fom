use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/eiland and the local accepted Eiland world baseline"]
fn eiland_winter_writing_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/eiland");
    let baseline = root.join("generated/slice-007-build/characters/eiland");
    let set = std::env::var_os("FOM_EILAND_WINTER_WRITING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_WINTER_WRITING_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..229];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..229], prior);
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
        ("read_sit_end_south", 0, 44, 47, 0x927D96, false),
        ("read_sit_end_south", 0, 40, 34, 0x9C5241, true),
        ("read_sit_end_south", 0, 38, 33, 0xA54E7F, false),
        ("read_sit_end_south", 0, 42, 45, 0xABD4CF, false),
        ("read_sit_end_south", 0, 39, 41, 0xBA6A4C, true),
        ("read_sit_end_south", 0, 45, 47, 0xBBB5C7, false),
        ("read_sit_end_south", 0, 38, 44, 0xC9AF9C, false),
        ("read_sit_end_south", 0, 40, 40, 0xDE8F5D, true),
        ("read_sit_end_south", 0, 36, 33, 0xE797AC, false),
        ("read_sit_end_south", 0, 40, 37, 0xE9A980, true),
        ("read_sit_end_south", 0, 38, 50, 0xEDE0EF, false),
        ("read_sit_end_south", 0, 38, 42, 0xF6E4D7, false),
        ("read_sit_end_south", 0, 41, 33, 0xFCDAE0, false),
        ("read_sit_end_south", 0, 43, 32, 0xFFFFFF, false),
        ("read_sit_loop_south", 1, 39, 41, 0xBA6A4C, true),
        ("read_sit_loop_south", 1, 40, 40, 0xDE8F5D, true),
        ("write_end_south", 0, 40, 47, 0x000000, false),
        ("write_end_south", 0, 38, 48, 0x2B2432, false),
        ("write_end_south", 0, 35, 48, 0x40384A, false),
        ("write_end_south", 0, 38, 49, 0x473C52, false),
        ("write_end_south", 0, 44, 47, 0x57342B, false),
        ("write_end_south", 0, 35, 49, 0x585061, false),
        ("write_end_south", 0, 33, 46, 0x663409, false),
        ("write_end_south", 0, 37, 44, 0x685979, false),
        ("write_end_south", 0, 37, 36, 0x6C2859, false),
        ("write_end_south", 0, 38, 39, 0x7D3B14, true),
        ("write_end_south", 0, 37, 50, 0x927D96, false),
        ("write_end_south", 0, 41, 33, 0x9C5241, true),
        ("write_end_south", 0, 39, 32, 0xA54E7F, false),
        ("write_end_south", 0, 43, 36, 0xA59DA2, false),
        ("write_end_south", 0, 44, 46, 0xB28159, false),
        ("write_end_south", 0, 39, 45, 0xBA6A4C, false),
        ("write_end_south", 0, 41, 40, 0xBA6A4C, true),
        ("write_end_south", 0, 37, 45, 0xBBB5C7, false),
        ("write_end_south", 0, 38, 36, 0xC2B9BE, false),
        ("write_end_south", 0, 40, 48, 0xC3D1DD, false),
        ("write_end_south", 0, 44, 42, 0xD2CCDE, false),
        ("write_end_south", 0, 34, 43, 0xD36A0E, false),
        ("write_end_south", 0, 40, 39, 0xDE8F5D, true),
        ("write_end_south", 0, 37, 32, 0xE797AC, false),
        ("write_end_south", 0, 36, 37, 0xE9A980, true),
        ("write_end_south", 0, 43, 37, 0xECF0E9, false),
        ("write_end_south", 0, 38, 46, 0xEDE0EF, false),
        ("write_end_south", 0, 40, 44, 0xF0BC70, false),
        ("write_end_south", 0, 36, 42, 0xF4F4F4, false),
        ("write_end_south", 0, 42, 46, 0xF5F5F5, false),
        ("write_end_south", 0, 33, 43, 0xF9AB6C, false),
        ("write_end_south", 0, 34, 45, 0xFAB680, false),
        ("write_end_south", 0, 42, 32, 0xFCDAE0, false),
        ("write_end_south", 0, 33, 45, 0xFFD8D1, false),
        ("write_end_south", 0, 44, 31, 0xFFFFFF, false),
        ("write_end_south", 1, 39, 41, 0xBA6A4C, true),
        ("write_end_south", 1, 40, 40, 0xDE8F5D, true),
        ("write_loop_south", 1, 41, 40, 0xBA6A4C, true),
        ("write_start_south", 0, 49, 48, 0x000000, false),
        ("write_start_south", 0, 38, 49, 0x2B2432, false),
        ("write_start_south", 0, 40, 49, 0x473C52, false),
        ("write_start_south", 0, 44, 50, 0x585061, false),
        ("write_start_south", 0, 34, 49, 0x663409, false),
        ("write_start_south", 0, 37, 45, 0x685979, false),
        ("write_start_south", 0, 36, 37, 0x6C2859, false),
        ("write_start_south", 0, 37, 51, 0x927D96, false),
        ("write_start_south", 0, 40, 34, 0x9C5241, true),
        ("write_start_south", 0, 38, 33, 0xA54E7F, false),
        ("write_start_south", 0, 44, 48, 0xB28159, false),
        ("write_start_south", 0, 44, 44, 0xBA6A4C, false),
        ("write_start_south", 0, 39, 41, 0xBA6A4C, true),
        ("write_start_south", 0, 41, 44, 0xBBB5C7, false),
        ("write_start_south", 0, 42, 49, 0xC3D1DD, false),
        ("write_start_south", 0, 44, 43, 0xD2CCDE, false),
        ("write_start_south", 0, 35, 46, 0xD36A0E, false),
        ("write_start_south", 0, 40, 40, 0xDE8F5D, true),
        ("write_start_south", 0, 36, 33, 0xE797AC, false),
        ("write_start_south", 0, 40, 37, 0xE9A980, true),
        ("write_start_south", 0, 38, 47, 0xEDE0EF, false),
        ("write_start_south", 0, 40, 45, 0xF0BC70, false),
        ("write_start_south", 0, 36, 43, 0xF4F4F4, false),
        ("write_start_south", 0, 44, 47, 0xF5F5F5, false),
        ("write_start_south", 0, 34, 46, 0xF9AB6C, false),
        ("write_start_south", 0, 35, 48, 0xFAB680, false),
        ("write_start_south", 0, 41, 33, 0xFCDAE0, false),
        ("write_start_south", 0, 34, 48, 0xFFD8D1, false),
        ("write_start_south", 0, 43, 32, 0xFFFFFF, false),
        ("write_start_south", 1, 41, 40, 0xBA6A4C, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("read_sit_end_south", &[32, 17, 24]),
        ("read_sit_loop_south", &[25, 32, 25, 32]),
        ("read_sit_start_south", &[24, 24, 25]),
        ("write_end_south", &[26, 32]),
        ("write_loop_south", &[26, 26, 26, 26]),
        ("write_start_south", &[32, 26]),
    ];
    // Reviewed Winter coat and gold-trim pixels share the skin shadow shade.
    let clothing: &[(&str, u32, u32, u32)] = &[
        ("read_sit_end_south", 2, 36, 41),
        ("read_sit_end_south", 2, 43, 41),
        ("read_sit_end_south", 2, 35, 43),
        ("read_sit_end_south", 2, 44, 43),
        ("read_sit_end_south", 1, 35, 44),
        ("read_sit_end_south", 1, 44, 44),
        ("read_sit_end_south", 2, 40, 44),
        ("read_sit_loop_south", 0, 36, 41),
        ("read_sit_loop_south", 0, 43, 41),
        ("read_sit_loop_south", 2, 36, 41),
        ("read_sit_loop_south", 2, 43, 41),
        ("read_sit_loop_south", 1, 36, 42),
        ("read_sit_loop_south", 1, 43, 42),
        ("read_sit_loop_south", 3, 36, 42),
        ("read_sit_loop_south", 3, 43, 42),
        ("read_sit_loop_south", 1, 40, 44),
        ("read_sit_loop_south", 3, 40, 44),
        ("read_sit_start_south", 0, 36, 41),
        ("read_sit_start_south", 0, 43, 41),
        ("read_sit_start_south", 0, 35, 43),
        ("read_sit_start_south", 0, 44, 43),
        ("read_sit_start_south", 0, 40, 44),
        ("read_sit_start_south", 1, 35, 44),
        ("read_sit_start_south", 1, 44, 44),
        ("write_end_south", 0, 35, 41),
        ("write_end_south", 0, 36, 41),
        ("write_end_south", 0, 43, 41),
        ("write_end_south", 1, 36, 42),
        ("write_end_south", 1, 43, 42),
        ("write_end_south", 0, 44, 43),
        ("write_end_south", 1, 44, 44),
        ("write_end_south", 0, 39, 45),
        ("write_end_south", 1, 39, 46),
        ("write_end_south", 0, 35, 47),
        ("write_end_south", 0, 34, 48),
        ("write_end_south", 0, 37, 48),
        ("write_end_south", 1, 37, 49),
        ("write_loop_south", 0, 36, 41),
        ("write_loop_south", 0, 43, 41),
        ("write_loop_south", 1, 36, 41),
        ("write_loop_south", 1, 43, 41),
        ("write_loop_south", 2, 43, 41),
        ("write_loop_south", 3, 36, 41),
        ("write_loop_south", 3, 43, 41),
        ("write_loop_south", 2, 44, 43),
        ("write_loop_south", 0, 35, 47),
        ("write_loop_south", 0, 34, 48),
        ("write_loop_south", 0, 37, 48),
        ("write_loop_south", 0, 45, 48),
        ("write_loop_south", 1, 34, 48),
        ("write_loop_south", 1, 45, 48),
        ("write_loop_south", 2, 34, 48),
        ("write_loop_south", 2, 37, 48),
        ("write_loop_south", 3, 37, 48),
        ("write_loop_south", 3, 45, 48),
        ("write_start_south", 1, 35, 41),
        ("write_start_south", 1, 36, 41),
        ("write_start_south", 1, 43, 41),
        ("write_start_south", 0, 36, 42),
        ("write_start_south", 0, 43, 42),
        ("write_start_south", 1, 44, 43),
        ("write_start_south", 0, 44, 44),
        ("write_start_south", 1, 39, 45),
        ("write_start_south", 0, 39, 46),
        ("write_start_south", 1, 35, 47),
        ("write_start_south", 1, 34, 48),
        ("write_start_south", 1, 37, 48),
        ("write_start_south", 0, 37, 49),
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
            let prefix = "spr_npc_eiland_specialanimation_winter";
            let asset = format!("assets/animations/NPCs/Eiland/Sprites/Winter/{prefix}_{case}.png");
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
                // Preserve the individually reviewed shared-color clothing pixels.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !clothing.contains(&(case, x / 80, x % 80, y)))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Eiland Winter material mismatch: {id} {case} [{x},{y}]"
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
