use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/eiland and the local accepted Eiland world baseline"]
fn eiland_wedding_finish_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/eiland");
    let baseline = root.join("generated/slice-014-build/characters/eiland");
    let set = std::env::var_os("FOM_EILAND_WEDDING_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_WEDDING_FINISH_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..268];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..268], prior);
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
        ("action_east", 0, 40, 49, 0x000000, false),
        ("action_east", 0, 39, 50, 0x685979, false),
        ("action_east", 0, 43, 43, 0x6A3126, false),
        ("action_east", 0, 36, 37, 0x6C2859, false),
        ("action_east", 0, 39, 40, 0x7D3B14, true),
        ("action_east", 0, 41, 44, 0x8B2240, false),
        ("action_east", 0, 40, 48, 0x927D96, false),
        ("action_east", 0, 38, 44, 0x9C5241, false),
        ("action_east", 0, 41, 34, 0x9C5241, true),
        ("action_east", 0, 47, 33, 0xA54E7F, false),
        ("action_east", 0, 41, 41, 0xBA6A4C, true),
        ("action_east", 0, 40, 50, 0xBBB5C7, false),
        ("action_east", 0, 40, 36, 0xC2B9BE, false),
        ("action_east", 0, 36, 48, 0xC9785A, false),
        ("action_east", 0, 43, 47, 0xDE8F5D, false),
        ("action_east", 0, 42, 40, 0xDE8F5D, true),
        ("action_east", 0, 42, 43, 0xE64372, false),
        ("action_east", 0, 39, 33, 0xE797AC, false),
        ("action_east", 0, 37, 38, 0xE9A980, true),
        ("action_east", 0, 39, 38, 0xECF0E9, false),
        ("action_east", 0, 42, 46, 0xF0BC70, false),
        ("action_east", 0, 41, 48, 0xF4F4F4, false),
        ("action_east", 0, 43, 33, 0xFCDAE0, false),
        ("action_east", 0, 45, 32, 0xFFFFFF, false),
        ("action_east", 1, 44, 40, 0xBA6A4C, true),
        ("action_south", 1, 40, 40, 0xBA6A4C, true),
        ("blink_east", 1, 41, 40, 0xBA6A4C, true),
        ("blink_south", 0, 40, 49, 0x000000, false),
        ("blink_south", 0, 37, 49, 0x685979, false),
        ("blink_south", 0, 41, 42, 0x6A3126, false),
        ("blink_south", 0, 34, 36, 0x6C2859, false),
        ("blink_south", 0, 39, 43, 0x8B2240, false),
        ("blink_south", 0, 43, 47, 0x927D96, false),
        ("blink_south", 0, 43, 43, 0x9C5241, false),
        ("blink_south", 0, 39, 33, 0x9C5241, true),
        ("blink_south", 0, 45, 32, 0xA54E7F, false),
        ("blink_south", 0, 40, 40, 0xBA6A4C, true),
        ("blink_south", 0, 37, 46, 0xBBB5C7, false),
        ("blink_south", 0, 42, 36, 0xC2B9BE, false),
        ("blink_south", 0, 44, 46, 0xC9785A, false),
        ("blink_south", 0, 39, 39, 0xDE8F5D, true),
        ("blink_south", 0, 37, 47, 0xE4D5E6, false),
        ("blink_south", 0, 40, 42, 0xE64372, false),
        ("blink_south", 0, 37, 32, 0xE797AC, false),
        ("blink_south", 0, 35, 37, 0xE9A980, true),
        ("blink_south", 0, 42, 37, 0xECF0E9, false),
        ("blink_south", 0, 40, 45, 0xF0BC70, false),
        ("blink_south", 0, 39, 47, 0xF4F4F4, false),
        ("blink_south", 0, 41, 32, 0xFCDAE0, false),
        ("blink_south", 0, 43, 31, 0xFFFFFF, false),
        ("blink_south", 1, 40, 40, 0xBA6A4C, true),
        ("kiss_east", 1, 40, 40, 0xBA6A4C, true),
        ("kiss_east", 1, 42, 40, 0xDE8F5D, true),
        ("sit_south", 0, 44, 47, 0x000000, false),
        ("sit_south", 0, 38, 46, 0x685979, false),
        ("sit_south", 0, 41, 42, 0x6A3126, false),
        ("sit_south", 0, 36, 36, 0x6C2859, false),
        ("sit_south", 0, 43, 44, 0x7D3B14, false),
        ("sit_south", 0, 39, 43, 0x8B2240, false),
        ("sit_south", 0, 36, 46, 0x927D96, false),
        ("sit_south", 0, 43, 43, 0x9C5241, false),
        ("sit_south", 0, 39, 33, 0x9C5241, true),
        ("sit_south", 0, 45, 32, 0xA54E7F, false),
        ("sit_south", 0, 40, 40, 0xBA6A4C, true),
        ("sit_south", 0, 36, 47, 0xBBB5C7, false),
        ("sit_south", 0, 38, 35, 0xC2B9BE, false),
        ("sit_south", 0, 41, 44, 0xC9785A, false),
        ("sit_south", 0, 39, 39, 0xDE8F5D, true),
        ("sit_south", 0, 42, 46, 0xE4D5E6, false),
        ("sit_south", 0, 40, 42, 0xE64372, false),
        ("sit_south", 0, 37, 32, 0xE797AC, false),
        ("sit_south", 0, 35, 37, 0xE9A980, true),
        ("sit_south", 0, 37, 37, 0xECF0E9, false),
        ("sit_south", 0, 43, 42, 0xF0BC70, false),
        ("sit_south", 0, 46, 46, 0xF4F4F4, false),
        ("sit_south", 0, 41, 32, 0xFCDAE0, false),
        ("sit_south", 0, 43, 31, 0xFFFFFF, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[24, 22, 22, 22, 22, 24, 23]),
        ("action_north", &[0, 0, 0, 0, 0, 0, 0]),
        ("action_south", &[21, 23, 23, 23, 23, 23, 23]),
        ("blink_east", &[24, 31, 24]),
        ("blink_south", &[24, 31, 24]),
        ("kiss_east", &[21, 25, 31, 31]),
        ("sit_east", &[24]),
        ("sit_north", &[0]),
        ("sit_south", &[23]),
    ];
    // Wedding gold trim shares a skin shadow; preserve each reviewed coordinate.
    let trim: &[(&str, u32, u32, u32, u32)] = &[
        ("action_east", 1, 41, 42, 0x9C5241),
        ("action_east", 2, 41, 42, 0x9C5241),
        ("action_east", 2, 42, 42, 0x9C5241),
        ("action_east", 3, 41, 42, 0x9C5241),
        ("action_east", 4, 41, 42, 0x9C5241),
        ("action_east", 4, 42, 42, 0x9C5241),
        ("action_east", 1, 39, 43, 0x9C5241),
        ("action_east", 1, 40, 43, 0x9C5241),
        ("action_east", 2, 39, 43, 0x9C5241),
        ("action_east", 3, 39, 43, 0x9C5241),
        ("action_east", 3, 40, 43, 0x9C5241),
        ("action_east", 4, 39, 43, 0x9C5241),
        ("action_east", 6, 36, 43, 0x9C5241),
        ("action_east", 6, 37, 43, 0x9C5241),
        ("action_east", 6, 44, 43, 0x9C5241),
        ("action_east", 0, 37, 44, 0x9C5241),
        ("action_east", 0, 38, 44, 0x9C5241),
        ("action_east", 1, 40, 44, 0xBA6A4C),
        ("action_east", 2, 40, 44, 0xBA6A4C),
        ("action_east", 3, 40, 44, 0xBA6A4C),
        ("action_east", 4, 40, 44, 0xBA6A4C),
        ("action_east", 5, 37, 44, 0x9C5241),
        ("action_east", 5, 38, 44, 0x9C5241),
        ("action_east", 1, 40, 45, 0xBA6A4C),
        ("action_east", 2, 40, 45, 0xBA6A4C),
        ("action_east", 3, 40, 45, 0xBA6A4C),
        ("action_east", 4, 40, 45, 0xBA6A4C),
        ("action_east", 0, 43, 47, 0xDE8F5D),
        ("action_east", 5, 43, 47, 0xDE8F5D),
        ("action_east", 1, 44, 52, 0x7D3B14),
        ("action_east", 2, 44, 52, 0x7D3B14),
        ("action_east", 3, 44, 52, 0x7D3B14),
        ("action_east", 4, 44, 52, 0x7D3B14),
        ("action_north", 1, 37, 40, 0x9C5241),
        ("action_north", 1, 42, 40, 0x9C5241),
        ("action_north", 2, 37, 40, 0x9C5241),
        ("action_north", 2, 42, 40, 0x9C5241),
        ("action_north", 3, 37, 40, 0x9C5241),
        ("action_north", 3, 42, 40, 0x9C5241),
        ("action_north", 4, 37, 40, 0x9C5241),
        ("action_north", 4, 42, 40, 0x9C5241),
        ("action_north", 0, 37, 41, 0x9C5241),
        ("action_north", 0, 42, 41, 0x9C5241),
        ("action_north", 5, 37, 41, 0x9C5241),
        ("action_north", 5, 42, 41, 0x9C5241),
        ("action_north", 6, 37, 41, 0x9C5241),
        ("action_north", 6, 42, 41, 0x9C5241),
        ("action_north", 1, 36, 42, 0x9C5241),
        ("action_north", 1, 37, 42, 0x9C5241),
        ("action_north", 1, 42, 42, 0x9C5241),
        ("action_north", 1, 43, 42, 0x9C5241),
        ("action_north", 2, 36, 42, 0x9C5241),
        ("action_north", 2, 37, 42, 0x9C5241),
        ("action_north", 2, 42, 42, 0x9C5241),
        ("action_north", 2, 43, 42, 0x9C5241),
        ("action_north", 3, 36, 42, 0x9C5241),
        ("action_north", 3, 37, 42, 0x9C5241),
        ("action_north", 3, 42, 42, 0x9C5241),
        ("action_north", 3, 43, 42, 0x9C5241),
        ("action_north", 4, 36, 42, 0x9C5241),
        ("action_north", 4, 37, 42, 0x9C5241),
        ("action_north", 4, 42, 42, 0x9C5241),
        ("action_north", 4, 43, 42, 0x9C5241),
        ("action_north", 0, 36, 43, 0x9C5241),
        ("action_north", 0, 37, 43, 0x9C5241),
        ("action_north", 0, 42, 43, 0x9C5241),
        ("action_north", 0, 43, 43, 0x9C5241),
        ("action_north", 1, 38, 43, 0x9C5241),
        ("action_north", 1, 39, 43, 0x9C5241),
        ("action_north", 1, 40, 43, 0x9C5241),
        ("action_north", 1, 41, 43, 0x9C5241),
        ("action_north", 2, 38, 43, 0x9C5241),
        ("action_north", 2, 39, 43, 0x9C5241),
        ("action_north", 2, 40, 43, 0x9C5241),
        ("action_north", 2, 41, 43, 0x9C5241),
        ("action_north", 3, 38, 43, 0x9C5241),
        ("action_north", 3, 39, 43, 0x9C5241),
        ("action_north", 3, 40, 43, 0x9C5241),
        ("action_north", 3, 41, 43, 0x9C5241),
        ("action_north", 4, 38, 43, 0x9C5241),
        ("action_north", 4, 39, 43, 0x9C5241),
        ("action_north", 4, 40, 43, 0x9C5241),
        ("action_north", 4, 41, 43, 0x9C5241),
        ("action_north", 5, 36, 43, 0x9C5241),
        ("action_north", 5, 37, 43, 0x9C5241),
        ("action_north", 5, 42, 43, 0x9C5241),
        ("action_north", 5, 43, 43, 0x9C5241),
        ("action_north", 6, 36, 43, 0x9C5241),
        ("action_north", 6, 37, 43, 0x9C5241),
        ("action_north", 6, 42, 43, 0x9C5241),
        ("action_north", 6, 43, 43, 0x9C5241),
        ("action_north", 0, 38, 44, 0x9C5241),
        ("action_north", 0, 39, 44, 0x9C5241),
        ("action_north", 0, 40, 44, 0x9C5241),
        ("action_north", 0, 41, 44, 0x9C5241),
        ("action_north", 5, 38, 44, 0x9C5241),
        ("action_north", 5, 39, 44, 0x9C5241),
        ("action_north", 5, 40, 44, 0x9C5241),
        ("action_north", 5, 41, 44, 0x9C5241),
        ("action_north", 6, 38, 44, 0x9C5241),
        ("action_north", 6, 39, 44, 0x9C5241),
        ("action_north", 6, 40, 44, 0x9C5241),
        ("action_north", 6, 41, 44, 0x9C5241),
        ("action_south", 0, 39, 41, 0xBA6A4C),
        ("action_south", 0, 40, 41, 0xBA6A4C),
        ("action_south", 1, 35, 43, 0x9C5241),
        ("action_south", 1, 36, 43, 0x9C5241),
        ("action_south", 1, 43, 43, 0x9C5241),
        ("action_south", 2, 35, 43, 0x9C5241),
        ("action_south", 2, 36, 43, 0x9C5241),
        ("action_south", 2, 43, 43, 0x9C5241),
        ("action_south", 3, 35, 43, 0x9C5241),
        ("action_south", 3, 36, 43, 0x9C5241),
        ("action_south", 3, 43, 43, 0x9C5241),
        ("action_south", 4, 35, 43, 0x9C5241),
        ("action_south", 4, 36, 43, 0x9C5241),
        ("action_south", 4, 43, 43, 0x9C5241),
        ("action_south", 6, 35, 43, 0x9C5241),
        ("action_south", 6, 36, 43, 0x9C5241),
        ("action_south", 6, 43, 43, 0x9C5241),
        ("action_south", 6, 44, 43, 0x9C5241),
        ("action_south", 0, 35, 44, 0x9C5241),
        ("action_south", 0, 36, 44, 0x9C5241),
        ("action_south", 0, 43, 44, 0x9C5241),
        ("action_south", 0, 44, 44, 0x9C5241),
        ("action_south", 1, 44, 44, 0x9C5241),
        ("action_south", 2, 44, 44, 0x9C5241),
        ("action_south", 3, 44, 44, 0x9C5241),
        ("action_south", 4, 44, 44, 0x9C5241),
        ("action_south", 5, 35, 44, 0x9C5241),
        ("action_south", 5, 36, 44, 0x9C5241),
        ("action_south", 5, 43, 44, 0x9C5241),
        ("action_south", 5, 44, 44, 0x9C5241),
        ("action_south", 2, 34, 45, 0x9C5241),
        ("action_south", 4, 34, 45, 0x9C5241),
        ("action_south", 0, 44, 46, 0x9C5241),
        ("action_south", 5, 44, 46, 0x9C5241),
        ("blink_east", 0, 36, 43, 0x9C5241),
        ("blink_east", 0, 37, 43, 0x9C5241),
        ("blink_east", 0, 44, 43, 0x9C5241),
        ("blink_east", 1, 36, 43, 0x9C5241),
        ("blink_east", 1, 37, 43, 0x9C5241),
        ("blink_east", 1, 44, 43, 0x9C5241),
        ("blink_east", 2, 36, 43, 0x9C5241),
        ("blink_east", 2, 37, 43, 0x9C5241),
        ("blink_east", 2, 44, 43, 0x9C5241),
        ("blink_south", 0, 35, 43, 0x9C5241),
        ("blink_south", 0, 36, 43, 0x9C5241),
        ("blink_south", 0, 43, 43, 0x9C5241),
        ("blink_south", 0, 44, 43, 0x9C5241),
        ("blink_south", 1, 35, 43, 0x9C5241),
        ("blink_south", 1, 36, 43, 0x9C5241),
        ("blink_south", 1, 43, 43, 0x9C5241),
        ("blink_south", 1, 44, 43, 0x9C5241),
        ("blink_south", 2, 35, 43, 0x9C5241),
        ("blink_south", 2, 36, 43, 0x9C5241),
        ("blink_south", 2, 43, 43, 0x9C5241),
        ("blink_south", 2, 44, 43, 0x9C5241),
        ("kiss_east", 0, 39, 41, 0xBA6A4C),
        ("kiss_east", 0, 40, 41, 0xBA6A4C),
        ("kiss_east", 1, 41, 41, 0xBA6A4C),
        ("kiss_east", 1, 42, 41, 0xBA6A4C),
        ("kiss_east", 2, 38, 43, 0x9C5241),
        ("kiss_east", 2, 39, 43, 0x9C5241),
        ("kiss_east", 0, 35, 44, 0x9C5241),
        ("kiss_east", 0, 36, 44, 0x9C5241),
        ("kiss_east", 0, 43, 44, 0x9C5241),
        ("kiss_east", 1, 37, 44, 0x9C5241),
        ("kiss_east", 1, 38, 44, 0x9C5241),
        ("kiss_east", 3, 37, 44, 0x9C5241),
        ("kiss_east", 3, 38, 44, 0x9C5241),
        ("sit_east", 0, 36, 43, 0x9C5241),
        ("sit_east", 0, 37, 43, 0x9C5241),
        ("sit_north", 0, 37, 41, 0x9C5241),
        ("sit_north", 0, 42, 41, 0x9C5241),
        ("sit_north", 0, 36, 43, 0x9C5241),
        ("sit_north", 0, 37, 43, 0x9C5241),
        ("sit_north", 0, 42, 43, 0x9C5241),
        ("sit_north", 0, 43, 43, 0x9C5241),
        ("sit_north", 0, 38, 44, 0x9C5241),
        ("sit_north", 0, 39, 44, 0x9C5241),
        ("sit_north", 0, 40, 44, 0x9C5241),
        ("sit_north", 0, 41, 44, 0x9C5241),
        ("sit_south", 0, 35, 43, 0x9C5241),
        ("sit_south", 0, 36, 43, 0x9C5241),
        ("sit_south", 0, 43, 43, 0x9C5241),
        ("sit_south", 0, 44, 43, 0x9C5241),
        ("sit_south", 0, 36, 44, 0x7D3B14),
        ("sit_south", 0, 43, 44, 0x7D3B14),
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
            let prefix = "spr_npc_eiland_wedding";
            let asset =
                format!("assets/animations/NPCs/Eiland/Sprites/Wedding/{prefix}_{case}.png");
            let before = image::open(original.join(&asset)).unwrap().to_rgba8();
            let after = image::open(variant.join(&asset)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), (80 * frames, 80));
            assert_eq!(before.dimensions(), after.dimensions());
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(variant.join(meta)).unwrap()
            );
            for &(name, frame, x, y, color) in trim {
                if name == case {
                    assert_eq!(before.get_pixel(frame * 80 + x, y).0, rgba(color));
                }
            }
            let mut per_frame = vec![0; frames as usize];
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                // Preserve the reviewed gold-trim pixels that share a skin shade.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| {
                        !trim
                            .iter()
                            .any(|t| (t.0, t.1, t.2, t.3) == (case, x / 80, x % 80, y))
                    })
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Eiland Wedding material mismatch: {id} {case} [{x},{y}]"
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
            // North shows only hair and clothing; retain the original PNG bytes.
            if case.ends_with("_north") {
                assert_eq!(
                    fs::read(original.join(&asset)).unwrap(),
                    fs::read(variant.join(&asset)).unwrap()
                );
            } else {
                assert!(per_frame.iter().all(|n| *n > 0), "empty frame in {case}");
            }
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
