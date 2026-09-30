use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/valen-summer-standard-study and the local accepted Valen world baseline"]
fn valen_summer_standard_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-summer-standard-study");
    let baseline = root
        .join("generated/characters-balor-valen-eiland-summer-expansion-trial/characters/valen");
    let set = std::env::var_os("FOM_VALEN_SUMMER_STANDARD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/valen-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_VALEN_SUMMER_STANDARD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/valen-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..149];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..149], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 154);
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
    let source = [0xFBD3A7, 0xEFA67A, 0xC37555, 0x762E21];
    // Literal source-grid landmarks include exposed skin and protected materials.
    // These expectations come from the source art, independently of recipe seeds.
    let landmarks = [
        ("action_east", 0, 45, 48, 0x000000, false),
        ("action_east", 0, 43, 46, 0x3F3935, false),
        ("action_east", 0, 38, 43, 0x542F3A, false),
        ("action_east", 0, 42, 52, 0x6264A0, false),
        ("action_east", 0, 40, 28, 0x6D4C12, false),
        ("action_east", 0, 38, 47, 0x6D70AF, false),
        ("action_east", 0, 45, 40, 0x6E578A, false),
        ("action_east", 0, 44, 48, 0x72665E, false),
        ("action_east", 0, 40, 48, 0x762E21, true),
        ("action_east", 0, 36, 34, 0xA385B9, false),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 38, 44, 0xAB615F, false),
        ("action_east", 0, 39, 46, 0xAEB0DF, false),
        ("action_east", 0, 39, 50, 0xAFA190, false),
        ("action_east", 0, 40, 30, 0xBD8E19, false),
        ("action_east", 0, 40, 36, 0xC2B9BE, false),
        ("action_east", 0, 39, 29, 0xC2D5E4, false),
        ("action_east", 0, 38, 51, 0xC37555, true),
        ("action_east", 0, 42, 44, 0xCD8D8B, false),
        ("action_east", 0, 36, 33, 0xD6C1DD, false),
        ("action_east", 0, 40, 50, 0xE3DACA, false),
        ("action_east", 0, 39, 38, 0xECF0E9, false),
        ("action_east", 0, 41, 42, 0xEFA67A, true),
        ("action_east", 0, 37, 33, 0xF5F5F5, false),
        ("action_east", 0, 41, 48, 0xFBD3A7, true),
        ("action_east", 1, 48, 45, 0x762E21, true),
        ("action_east", 1, 49, 45, 0xC37555, true),
        ("action_east", 1, 47, 45, 0xEFA67A, true),
        ("action_east", 1, 39, 50, 0xFBD3A7, true),
        ("action_east", 2, 45, 46, 0x762E21, true),
        ("action_east", 2, 44, 46, 0xC37555, true),
        ("action_east", 2, 44, 45, 0xEFA67A, true),
        ("action_east", 2, 43, 51, 0xFBD3A7, true),
        ("action_east", 3, 48, 45, 0x762E21, true),
        ("action_east", 3, 49, 45, 0xC37555, true),
        ("action_east", 3, 47, 45, 0xEFA67A, true),
        ("action_east", 3, 39, 50, 0xFBD3A7, true),
        ("action_east", 4, 45, 46, 0x762E21, true),
        ("action_east", 4, 44, 46, 0xC37555, true),
        ("action_east", 4, 44, 45, 0xEFA67A, true),
        ("action_east", 4, 43, 51, 0xFBD3A7, true),
        ("action_east", 5, 40, 48, 0x762E21, true),
        ("action_east", 5, 38, 51, 0xC37555, true),
        ("action_east", 5, 41, 42, 0xEFA67A, true),
        ("action_east", 5, 41, 48, 0xFBD3A7, true),
        ("action_east", 6, 35, 47, 0x762E21, true),
        ("action_east", 6, 34, 47, 0xC37555, true),
        ("action_east", 6, 44, 45, 0xEFA67A, true),
        ("action_east", 6, 36, 47, 0xFBD3A7, true),
        ("action_north", 0, 36, 47, 0x000000, false),
        ("action_north", 0, 42, 45, 0x3F3935, false),
        ("action_north", 0, 43, 43, 0x542F3A, false),
        ("action_north", 0, 41, 52, 0x6264A0, false),
        ("action_north", 0, 43, 27, 0x6D4C12, false),
        ("action_north", 0, 46, 42, 0x6D70AF, false),
        ("action_north", 0, 40, 40, 0x6E578A, false),
        ("action_north", 0, 40, 45, 0x72665E, false),
        ("action_north", 0, 33, 45, 0x762E21, true),
        ("action_north", 0, 35, 34, 0xA385B9, false),
        ("action_north", 0, 42, 42, 0xAB615F, false),
        ("action_north", 0, 37, 48, 0xAFA190, false),
        ("action_north", 0, 44, 28, 0xBD8E19, false),
        ("action_north", 0, 35, 44, 0xC37555, true),
        ("action_north", 0, 39, 43, 0xCD8D8B, false),
        ("action_north", 0, 35, 35, 0xD6C1DD, false),
        ("action_north", 0, 38, 48, 0xE3DACA, false),
        ("action_north", 0, 42, 51, 0xEFA67A, true),
        ("action_north", 0, 40, 32, 0xF5F5F5, false),
        ("action_north", 0, 34, 44, 0xFBD3A7, true),
        ("action_north", 1, 35, 46, 0x762E21, true),
        ("action_north", 1, 34, 44, 0xC37555, true),
        ("action_north", 1, 42, 50, 0xEFA67A, true),
        ("action_north", 1, 34, 46, 0xFBD3A7, true),
        ("action_north", 2, 35, 46, 0x762E21, true),
        ("action_north", 2, 34, 44, 0xC37555, true),
        ("action_north", 2, 42, 50, 0xEFA67A, true),
        ("action_north", 2, 34, 46, 0xFBD3A7, true),
        ("action_north", 3, 35, 46, 0x762E21, true),
        ("action_north", 3, 34, 44, 0xC37555, true),
        ("action_north", 3, 42, 50, 0xEFA67A, true),
        ("action_north", 3, 34, 46, 0xFBD3A7, true),
        ("action_north", 4, 35, 46, 0x762E21, true),
        ("action_north", 4, 34, 44, 0xC37555, true),
        ("action_north", 4, 42, 50, 0xEFA67A, true),
        ("action_north", 4, 34, 46, 0xFBD3A7, true),
        ("action_north", 5, 34, 47, 0x762E21, true),
        ("action_north", 5, 46, 44, 0xC37555, true),
        ("action_north", 5, 42, 51, 0xEFA67A, true),
        ("action_north", 5, 34, 46, 0xFBD3A7, true),
        ("action_north", 6, 46, 47, 0x762E21, true),
        ("action_north", 6, 33, 45, 0xC37555, true),
        ("action_north", 6, 37, 53, 0xEFA67A, true),
        ("action_north", 6, 47, 46, 0xFBD3A7, true),
        ("action_south", 0, 44, 48, 0x000000, false),
        ("action_south", 0, 42, 46, 0x3F3935, false),
        ("action_south", 0, 36, 45, 0x542F3A, false),
        ("action_south", 0, 41, 52, 0x6264A0, false),
        ("action_south", 0, 41, 28, 0x6D4C12, false),
        ("action_south", 0, 35, 46, 0x6D70AF, false),
        ("action_south", 0, 36, 41, 0x6E578A, false),
        ("action_south", 0, 39, 48, 0x72665E, false),
        ("action_south", 0, 44, 46, 0x762E21, true),
        ("action_south", 0, 34, 33, 0xA385B9, false),
        ("action_south", 0, 37, 44, 0xAB615F, false),
        ("action_south", 0, 34, 46, 0xAEB0DF, false),
        ("action_south", 0, 37, 50, 0xAFA190, false),
        ("action_south", 0, 35, 30, 0xBD8E19, false),
        ("action_south", 0, 41, 36, 0xC2B9BE, false),
        ("action_south", 0, 37, 29, 0xC2D5E4, false),
        ("action_south", 0, 44, 45, 0xC37555, true),
        ("action_south", 0, 41, 44, 0xCD8D8B, false),
        ("action_south", 0, 44, 32, 0xD6C1DD, false),
        ("action_south", 0, 41, 48, 0xE3DACA, false),
        ("action_south", 0, 37, 38, 0xECF0E9, false),
        ("action_south", 0, 40, 42, 0xEFA67A, true),
        ("action_south", 0, 36, 33, 0xF5F5F5, false),
        ("action_south", 0, 35, 47, 0xFBD3A7, true),
        ("action_south", 1, 35, 47, 0x762E21, true),
        ("action_south", 1, 44, 45, 0xC37555, true),
        ("action_south", 1, 35, 44, 0xEFA67A, true),
        ("action_south", 1, 34, 47, 0xFBD3A7, true),
        ("action_south", 2, 38, 47, 0x762E21, true),
        ("action_south", 2, 44, 45, 0xC37555, true),
        ("action_south", 2, 36, 45, 0xEFA67A, true),
        ("action_south", 2, 39, 46, 0xFBD3A7, true),
        ("action_south", 3, 35, 47, 0x762E21, true),
        ("action_south", 3, 44, 45, 0xC37555, true),
        ("action_south", 3, 35, 44, 0xEFA67A, true),
        ("action_south", 3, 34, 47, 0xFBD3A7, true),
        ("action_south", 4, 38, 47, 0x762E21, true),
        ("action_south", 4, 44, 45, 0xC37555, true),
        ("action_south", 4, 36, 45, 0xEFA67A, true),
        ("action_south", 4, 39, 46, 0xFBD3A7, true),
        ("action_south", 5, 44, 46, 0x762E21, true),
        ("action_south", 5, 44, 45, 0xC37555, true),
        ("action_south", 5, 40, 42, 0xEFA67A, true),
        ("action_south", 5, 35, 47, 0xFBD3A7, true),
        ("action_south", 6, 46, 47, 0x762E21, true),
        ("action_south", 6, 45, 44, 0xC37555, true),
        ("action_south", 6, 37, 51, 0xEFA67A, true),
        ("action_south", 6, 46, 46, 0xFBD3A7, true),
        ("kiss_east", 0, 42, 48, 0x000000, false),
        ("kiss_east", 0, 41, 46, 0x3F3935, false),
        ("kiss_east", 0, 42, 44, 0x542F3A, false),
        ("kiss_east", 0, 41, 52, 0x6264A0, false),
        ("kiss_east", 0, 38, 28, 0x6D4C12, false),
        ("kiss_east", 0, 36, 46, 0x6D70AF, false),
        ("kiss_east", 0, 43, 40, 0x6E578A, false),
        ("kiss_east", 0, 38, 48, 0x72665E, false),
        ("kiss_east", 0, 35, 48, 0x762E21, true),
        ("kiss_east", 0, 34, 34, 0xA385B9, false),
        ("kiss_east", 0, 42, 36, 0xA59DA2, false),
        ("kiss_east", 0, 37, 44, 0xAB615F, false),
        ("kiss_east", 0, 35, 46, 0xAEB0DF, false),
        ("kiss_east", 0, 37, 50, 0xAFA190, false),
        ("kiss_east", 0, 38, 30, 0xBD8E19, false),
        ("kiss_east", 0, 38, 36, 0xC2B9BE, false),
        ("kiss_east", 0, 37, 29, 0xC2D5E4, false),
        ("kiss_east", 0, 34, 48, 0xC37555, true),
        ("kiss_east", 0, 40, 44, 0xCD8D8B, false),
        ("kiss_east", 0, 34, 33, 0xD6C1DD, false),
        ("kiss_east", 0, 41, 49, 0xE3DACA, false),
        ("kiss_east", 0, 37, 38, 0xECF0E9, false),
        ("kiss_east", 0, 39, 42, 0xEFA67A, true),
        ("kiss_east", 0, 35, 33, 0xF5F5F5, false),
        ("kiss_east", 0, 36, 48, 0xFBD3A7, true),
        ("kiss_east", 1, 36, 48, 0x762E21, true),
        ("kiss_east", 1, 35, 48, 0xC37555, true),
        ("kiss_east", 1, 43, 40, 0xEFA67A, true),
        ("kiss_east", 1, 37, 48, 0xFBD3A7, true),
        ("kiss_east", 2, 36, 46, 0x762E21, true),
        ("kiss_east", 2, 41, 51, 0xC37555, true),
        ("kiss_east", 2, 43, 41, 0xEFA67A, true),
        ("kiss_east", 2, 37, 45, 0xFBD3A7, true),
        ("kiss_east", 3, 36, 48, 0x762E21, true),
        ("kiss_east", 3, 35, 48, 0xC37555, true),
        ("kiss_east", 3, 43, 40, 0xEFA67A, true),
        ("kiss_east", 3, 37, 48, 0xFBD3A7, true),
        ("sleep_east", 0, 37, 48, 0x000000, false),
        ("sleep_east", 0, 42, 45, 0x3F3935, false),
        ("sleep_east", 0, 39, 43, 0x542F3A, false),
        ("sleep_east", 0, 41, 52, 0x6264A0, false),
        ("sleep_east", 0, 39, 27, 0x6D4C12, false),
        ("sleep_east", 0, 42, 40, 0x6D70AF, false),
        ("sleep_east", 0, 37, 40, 0x6E578A, false),
        ("sleep_east", 0, 41, 47, 0x72665E, false),
        ("sleep_east", 0, 37, 38, 0x762E21, true),
        ("sleep_east", 0, 35, 33, 0xA385B9, false),
        ("sleep_east", 0, 40, 42, 0xAB615F, false),
        ("sleep_east", 0, 44, 41, 0xAEB0DF, false),
        ("sleep_east", 0, 38, 49, 0xAFA190, false),
        ("sleep_east", 0, 39, 29, 0xBD8E19, false),
        ("sleep_east", 0, 38, 28, 0xC2D5E4, false),
        ("sleep_east", 0, 41, 51, 0xC37555, true),
        ("sleep_east", 0, 40, 44, 0xCD8D8B, false),
        ("sleep_east", 0, 35, 32, 0xD6C1DD, false),
        ("sleep_east", 0, 39, 49, 0xE3DACA, false),
        ("sleep_east", 0, 42, 42, 0xEFA67A, true),
        ("sleep_east", 0, 36, 32, 0xF5F5F5, false),
        ("sleep_east", 0, 39, 51, 0xFBD3A7, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[55, 53, 53, 53, 53, 55, 57]),
        ("action_north", &[26, 19, 20, 19, 20, 27, 27]),
        ("action_south", &[64, 62, 63, 62, 63, 64, 68]),
        ("kiss_east", &[54, 61, 65, 67]),
        ("sleep_east", &[59]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [8, 9, 10, 4]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_valen_summer";
            let asset = format!("assets/animations/NPCs/Valen/Sprites/Summer/{prefix}_{case}.png");
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
                // All reviewed world shades are skin in these Summer strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Valen Summer material mismatch: {id} {case} [{x},{y}]"
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
