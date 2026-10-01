use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/balor and the local accepted Balor world baseline"]
fn balor_beach_finish_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/balor");
    let baseline = root.join("generated/slice-004-build/characters/balor");
    let set = std::env::var_os("FOM_BALOR_BEACH_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_BALOR_BEACH_FINISH_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/balor-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..244];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..244], prior);
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
    let source = [0xFCD9B3, 0xF0B988, 0xD37A57, 0x672115];
    // Reviewed landmarks distinguish moving skin from protected materials.
    // Literal per-frame counts guard brief and occluded skin exposure.
    let landmarks = [
        ("beach_bath_swim_east", 0, 38, 49, 0x000000, false),
        ("beach_bath_swim_east", 0, 36, 53, 0x281846, false),
        ("beach_bath_swim_east", 0, 46, 55, 0x328BC9, false),
        ("beach_bath_swim_east", 0, 39, 48, 0x4A3D66, false),
        ("beach_bath_swim_east", 0, 38, 53, 0x672115, true),
        ("beach_bath_swim_east", 0, 38, 47, 0x686589, false),
        ("beach_bath_swim_east", 0, 41, 45, 0x9B83B7, false),
        ("beach_bath_swim_east", 0, 41, 55, 0x9DEBFC, false),
        ("beach_bath_swim_east", 0, 43, 50, 0xA59DA2, false),
        ("beach_bath_swim_east", 0, 38, 50, 0xC2B9BE, false),
        ("beach_bath_swim_east", 0, 39, 53, 0xD37A57, true),
        ("beach_bath_swim_east", 0, 43, 51, 0xECF0E9, false),
        ("beach_bath_swim_east", 0, 43, 52, 0xF0B988, true),
        ("beach_bath_swim_east", 0, 40, 52, 0xFCD9B3, true),
        ("beach_bath_swim_east", 1, 38, 53, 0x672115, true),
        ("beach_bath_swim_east", 1, 39, 53, 0xD37A57, true),
        ("beach_bath_swim_east", 1, 43, 52, 0xF0B988, true),
        ("beach_bath_swim_east", 1, 40, 52, 0xFCD9B3, true),
        ("beach_bath_swim_south", 0, 42, 49, 0x000000, false),
        ("beach_bath_swim_south", 0, 44, 53, 0x281846, false),
        ("beach_bath_swim_south", 0, 35, 56, 0x328BC9, false),
        ("beach_bath_swim_south", 0, 40, 47, 0x4A3D66, false),
        ("beach_bath_swim_south", 0, 37, 53, 0x672115, true),
        ("beach_bath_swim_south", 0, 41, 47, 0x686589, false),
        ("beach_bath_swim_south", 0, 40, 45, 0x9B83B7, false),
        ("beach_bath_swim_south", 0, 31, 59, 0x9DEBFC, false),
        ("beach_bath_swim_south", 0, 42, 50, 0xC2B9BE, false),
        ("beach_bath_swim_south", 0, 38, 53, 0xD37A57, true),
        ("beach_bath_swim_south", 0, 42, 51, 0xECF0E9, false),
        ("beach_bath_swim_south", 0, 38, 52, 0xF0B988, true),
        ("beach_bath_swim_south", 0, 40, 51, 0xFCD9B3, true),
        ("beach_bath_swim_south", 1, 37, 53, 0x672115, true),
        ("beach_bath_swim_south", 1, 38, 53, 0xD37A57, true),
        ("beach_bath_swim_south", 1, 38, 52, 0xF0B988, true),
        ("beach_bath_swim_south", 1, 40, 51, 0xFCD9B3, true),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            35,
            48,
            0x000000,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            35,
            38,
            0x281846,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            42,
            47,
            0x2E2E64,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            35,
            44,
            0x374050,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            41,
            46,
            0x424E88,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            36,
            47,
            0x461839,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            38,
            34,
            0x4A3D66,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            34,
            44,
            0x525A62,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            41,
            48,
            0x6472B6,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            35,
            46,
            0x672115,
            true,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            36,
            42,
            0x686589,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            37,
            46,
            0x77405D,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            40,
            31,
            0x9B83B7,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            40,
            45,
            0xA56B81,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            37,
            42,
            0xB395D6,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            39,
            43,
            0xD37A57,
            true,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            36,
            41,
            0xDEDAE9,
            false,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            37,
            49,
            0xF0B988,
            true,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            0,
            41,
            49,
            0xFCD9B3,
            true,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            1,
            35,
            46,
            0x672115,
            true,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            1,
            39,
            43,
            0xD37A57,
            true,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            1,
            34,
            46,
            0xF0B988,
            true,
        ),
        (
            "specialanimation_beach_hair_flip_south",
            1,
            35,
            45,
            0xFCD9B3,
            true,
        ),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("beach_bath_swim_east", &[21, 21, 19, 19]),
        ("beach_bath_swim_south", &[25, 25, 23, 23]),
        (
            "specialanimation_beach_hair_flip_south",
            &[59, 87, 87, 88, 85],
        ),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [7, 8, 9, 10]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_balor";
            let asset = format!("assets/animations/NPCs/Balor/Sprites/Beach/{prefix}_{case}.png");
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
                // All reviewed world shades are skin in these Beach strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Balor Beach material mismatch: {id} {case} [{x},{y}]"
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
