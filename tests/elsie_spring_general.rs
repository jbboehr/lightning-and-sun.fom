use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/elsie and the local accepted Elsie world baseline"]
fn elsie_spring_general_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/elsie");
    let baseline = root.join("local/fixtures/slice-176-build/characters/elsie");
    let set = std::env::var_os("FOM_ELSIE_SPRING_GENERAL_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/elsie-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/elsie-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..53];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..53], prior);
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
    let source = [0xFBDCB9, 0xF3C193, 0xE08C6A, 0xA45838];
    // The four world shades occur only in skin in these three pinned sources.
    // Forehead, face, neckline and moving hands follow the palette.
    // Check every pixel independently of component seeds; mouth interiors,
    // hair, cosmetics, earrings, gown and shoes keep their original colors.
    let landmarks = [
        ("action_east", 0, 42, 34, 0xFBDCB9, true),
        ("action_east", 0, 42, 33, 0xF3C193, true),
        ("action_east", 0, 42, 32, 0xE08C6A, true),
        ("action_east", 0, 37, 36, 0xA45838, true),
        ("action_east", 0, 39, 40, 0xA45838, true),
        ("action_east", 0, 34, 33, 0x000000, false),
        ("action_east", 0, 39, 36, 0xC2B9BE, false),
        ("action_east", 0, 39, 37, 0xECF0E9, false),
        ("action_east", 0, 38, 39, 0xA45838, true),
        ("action_east", 0, 38, 28, 0x5B566E, false),
        ("action_east", 0, 39, 28, 0x91879B, false),
        ("action_east", 0, 40, 28, 0xD7C8CE, false),
        ("action_east", 0, 37, 40, 0xF4AB50, false),
        ("action_east", 0, 37, 41, 0xFFE990, false),
        ("action_east", 0, 42, 46, 0x700815, false),
        ("action_east", 0, 40, 42, 0x721943, false),
        ("action_east", 0, 45, 45, 0x9B212C, false),
        ("action_east", 0, 38, 42, 0xA03E59, false),
        ("action_east", 0, 39, 42, 0xEB8C8C, false),
        ("action_east", 0, 38, 43, 0xFBC9BB, false),
        ("action_east", 0, 41, 44, 0xD84B3A, false),
        ("action_east", 1, 44, 33, 0xFBDCB9, true),
        ("action_east", 1, 44, 32, 0xF3C193, true),
        ("action_east", 1, 44, 31, 0xE08C6A, true),
        ("action_east", 1, 39, 35, 0xA45838, true),
        ("action_east", 1, 43, 40, 0xE08C6A, true),
        ("action_east", 1, 35, 33, 0x000000, false),
        ("action_east", 1, 41, 35, 0xC2B9BE, false),
        ("action_east", 1, 41, 36, 0xECF0E9, false),
        ("action_east", 1, 41, 39, 0xA45838, true),
        ("action_east", 1, 40, 27, 0x5B566E, false),
        ("action_east", 1, 41, 27, 0x91879B, false),
        ("action_east", 1, 42, 27, 0xD7C8CE, false),
        ("action_east", 1, 39, 39, 0xF4AB50, false),
        ("action_east", 1, 39, 40, 0xFFE990, false),
        ("action_east", 1, 42, 46, 0x700815, false),
        ("action_east", 1, 45, 43, 0x721943, false),
        ("action_east", 1, 43, 48, 0x9B212C, false),
        ("action_east", 1, 41, 41, 0xA03E59, false),
        ("action_east", 1, 41, 42, 0xEB8C8C, false),
        ("action_east", 1, 42, 41, 0xFBC9BB, false),
        ("action_east", 1, 43, 49, 0xD84B3A, false),
        ("action_east", 2, 44, 33, 0xFBDCB9, true),
        ("action_east", 2, 44, 32, 0xF3C193, true),
        ("action_east", 2, 44, 31, 0xE08C6A, true),
        ("action_east", 2, 39, 35, 0xA45838, true),
        ("action_east", 2, 43, 40, 0xE08C6A, true),
        ("action_east", 2, 35, 33, 0x000000, false),
        ("action_east", 2, 41, 35, 0xC2B9BE, false),
        ("action_east", 2, 41, 36, 0xECF0E9, false),
        ("action_east", 2, 41, 39, 0xA45838, true),
        ("action_east", 2, 40, 27, 0x5B566E, false),
        ("action_east", 2, 41, 27, 0x91879B, false),
        ("action_east", 2, 42, 27, 0xD7C8CE, false),
        ("action_east", 2, 39, 39, 0xF4AB50, false),
        ("action_east", 2, 39, 40, 0xFFE990, false),
        ("action_east", 2, 41, 47, 0x700815, false),
        ("action_east", 2, 43, 43, 0x721943, false),
        ("action_east", 2, 47, 44, 0x9B212C, false),
        ("action_east", 2, 41, 41, 0xA03E59, false),
        ("action_east", 2, 41, 42, 0xEB8C8C, false),
        ("action_east", 2, 42, 41, 0xFBC9BB, false),
        ("action_east", 2, 47, 43, 0xD84B3A, false),
        ("action_east", 3, 44, 33, 0xFBDCB9, true),
        ("action_east", 3, 44, 32, 0xF3C193, true),
        ("action_east", 3, 44, 31, 0xE08C6A, true),
        ("action_east", 3, 39, 35, 0xA45838, true),
        ("action_east", 3, 43, 40, 0xE08C6A, true),
        ("action_east", 3, 35, 33, 0x000000, false),
        ("action_east", 3, 41, 35, 0xC2B9BE, false),
        ("action_east", 3, 41, 36, 0xECF0E9, false),
        ("action_east", 3, 41, 39, 0xA45838, true),
        ("action_east", 3, 40, 27, 0x5B566E, false),
        ("action_east", 3, 41, 27, 0x91879B, false),
        ("action_east", 3, 42, 27, 0xD7C8CE, false),
        ("action_east", 3, 39, 39, 0xF4AB50, false),
        ("action_east", 3, 39, 40, 0xFFE990, false),
        ("action_east", 3, 42, 46, 0x700815, false),
        ("action_east", 3, 45, 43, 0x721943, false),
        ("action_east", 3, 43, 48, 0x9B212C, false),
        ("action_east", 3, 41, 41, 0xA03E59, false),
        ("action_east", 3, 41, 42, 0xEB8C8C, false),
        ("action_east", 3, 42, 41, 0xFBC9BB, false),
        ("action_east", 3, 43, 49, 0xD84B3A, false),
        ("action_east", 4, 44, 33, 0xFBDCB9, true),
        ("action_east", 4, 44, 32, 0xF3C193, true),
        ("action_east", 4, 44, 31, 0xE08C6A, true),
        ("action_east", 4, 39, 35, 0xA45838, true),
        ("action_east", 4, 43, 40, 0xE08C6A, true),
        ("action_east", 4, 35, 33, 0x000000, false),
        ("action_east", 4, 41, 35, 0xC2B9BE, false),
        ("action_east", 4, 41, 36, 0xECF0E9, false),
        ("action_east", 4, 41, 39, 0xA45838, true),
        ("action_east", 4, 40, 27, 0x5B566E, false),
        ("action_east", 4, 41, 27, 0x91879B, false),
        ("action_east", 4, 42, 27, 0xD7C8CE, false),
        ("action_east", 4, 39, 39, 0xF4AB50, false),
        ("action_east", 4, 39, 40, 0xFFE990, false),
        ("action_east", 4, 41, 47, 0x700815, false),
        ("action_east", 4, 43, 43, 0x721943, false),
        ("action_east", 4, 47, 44, 0x9B212C, false),
        ("action_east", 4, 41, 41, 0xA03E59, false),
        ("action_east", 4, 41, 42, 0xEB8C8C, false),
        ("action_east", 4, 42, 41, 0xFBC9BB, false),
        ("action_east", 4, 47, 43, 0xD84B3A, false),
        ("action_east", 5, 42, 34, 0xFBDCB9, true),
        ("action_east", 5, 42, 33, 0xF3C193, true),
        ("action_east", 5, 42, 32, 0xE08C6A, true),
        ("action_east", 5, 37, 36, 0xA45838, true),
        ("action_east", 5, 39, 40, 0xA45838, true),
        ("action_east", 5, 34, 33, 0x000000, false),
        ("action_east", 5, 39, 36, 0xC2B9BE, false),
        ("action_east", 5, 39, 37, 0xECF0E9, false),
        ("action_east", 5, 38, 39, 0xA45838, true),
        ("action_east", 5, 38, 28, 0x5B566E, false),
        ("action_east", 5, 39, 28, 0x91879B, false),
        ("action_east", 5, 40, 28, 0xD7C8CE, false),
        ("action_east", 5, 37, 40, 0xF4AB50, false),
        ("action_east", 5, 37, 41, 0xFFE990, false),
        ("action_east", 5, 42, 46, 0x700815, false),
        ("action_east", 5, 40, 42, 0x721943, false),
        ("action_east", 5, 45, 45, 0x9B212C, false),
        ("action_east", 5, 38, 42, 0xA03E59, false),
        ("action_east", 5, 39, 42, 0xEB8C8C, false),
        ("action_east", 5, 38, 43, 0xFBC9BB, false),
        ("action_east", 5, 41, 44, 0xD84B3A, false),
        ("action_east", 6, 41, 33, 0xFBDCB9, true),
        ("action_east", 6, 41, 32, 0xF3C193, true),
        ("action_east", 6, 41, 31, 0xE08C6A, true),
        ("action_east", 6, 36, 35, 0xA45838, true),
        ("action_east", 6, 40, 40, 0xE08C6A, true),
        ("action_east", 6, 38, 33, 0x000000, false),
        ("action_east", 6, 38, 35, 0xC2B9BE, false),
        ("action_east", 6, 38, 36, 0xECF0E9, false),
        ("action_east", 6, 38, 39, 0xA45838, true),
        ("action_east", 6, 37, 27, 0x5B566E, false),
        ("action_east", 6, 38, 27, 0x91879B, false),
        ("action_east", 6, 39, 27, 0xD7C8CE, false),
        ("action_east", 6, 36, 39, 0xF4AB50, false),
        ("action_east", 6, 36, 40, 0xFFE990, false),
        ("action_east", 6, 41, 45, 0x700815, false),
        ("action_east", 6, 39, 41, 0x721943, false),
        ("action_east", 6, 39, 44, 0x9B212C, false),
        ("action_east", 6, 37, 41, 0xA03E59, false),
        ("action_east", 6, 38, 41, 0xEB8C8C, false),
        ("action_east", 6, 37, 42, 0xFBC9BB, false),
        ("action_east", 6, 39, 43, 0xD84B3A, false),
        ("action_north", 0, 45, 41, 0xA45838, true),
        ("action_north", 0, 36, 28, 0x5B566E, false),
        ("action_north", 0, 38, 27, 0x91879B, false),
        ("action_north", 0, 39, 27, 0xD7C8CE, false),
        ("action_north", 0, 35, 40, 0xF4AB50, false),
        ("action_north", 0, 35, 42, 0x721943, false),
        ("action_north", 0, 36, 41, 0xA03E59, false),
        ("action_north", 0, 37, 41, 0xEB8C8C, false),
        ("action_north", 0, 36, 42, 0xFBC9BB, false),
        ("action_north", 1, 34, 45, 0xFBDCB9, true),
        ("action_north", 1, 36, 27, 0x5B566E, false),
        ("action_north", 1, 38, 26, 0x91879B, false),
        ("action_north", 1, 39, 26, 0xD7C8CE, false),
        ("action_north", 1, 35, 39, 0xF4AB50, false),
        ("action_north", 1, 35, 42, 0x721943, false),
        ("action_north", 1, 43, 41, 0xA03E59, false),
        ("action_north", 1, 37, 41, 0xEB8C8C, false),
        ("action_north", 1, 36, 41, 0xFBC9BB, false),
        ("action_north", 2, 34, 45, 0xFBDCB9, true),
        ("action_north", 2, 36, 27, 0x5B566E, false),
        ("action_north", 2, 38, 26, 0x91879B, false),
        ("action_north", 2, 39, 26, 0xD7C8CE, false),
        ("action_north", 2, 35, 39, 0xF4AB50, false),
        ("action_north", 2, 35, 42, 0x721943, false),
        ("action_north", 2, 37, 42, 0xA03E59, false),
        ("action_north", 2, 37, 41, 0xEB8C8C, false),
        ("action_north", 2, 36, 41, 0xFBC9BB, false),
        ("action_north", 3, 34, 45, 0xFBDCB9, true),
        ("action_north", 3, 36, 27, 0x5B566E, false),
        ("action_north", 3, 38, 26, 0x91879B, false),
        ("action_north", 3, 39, 26, 0xD7C8CE, false),
        ("action_north", 3, 35, 39, 0xF4AB50, false),
        ("action_north", 3, 35, 42, 0x721943, false),
        ("action_north", 3, 43, 41, 0xA03E59, false),
        ("action_north", 3, 37, 41, 0xEB8C8C, false),
        ("action_north", 3, 36, 41, 0xFBC9BB, false),
        ("action_north", 4, 34, 45, 0xFBDCB9, true),
        ("action_north", 4, 36, 27, 0x5B566E, false),
        ("action_north", 4, 38, 26, 0x91879B, false),
        ("action_north", 4, 39, 26, 0xD7C8CE, false),
        ("action_north", 4, 35, 39, 0xF4AB50, false),
        ("action_north", 4, 35, 42, 0x721943, false),
        ("action_north", 4, 37, 42, 0xA03E59, false),
        ("action_north", 4, 37, 41, 0xEB8C8C, false),
        ("action_north", 4, 36, 41, 0xFBC9BB, false),
        ("action_north", 5, 45, 41, 0xA45838, true),
        ("action_north", 5, 36, 28, 0x5B566E, false),
        ("action_north", 5, 38, 27, 0x91879B, false),
        ("action_north", 5, 39, 27, 0xD7C8CE, false),
        ("action_north", 5, 35, 40, 0xF4AB50, false),
        ("action_north", 5, 35, 43, 0x721943, false),
        ("action_north", 5, 36, 41, 0xA03E59, false),
        ("action_north", 5, 37, 41, 0xEB8C8C, false),
        ("action_north", 5, 36, 42, 0xFBC9BB, false),
        ("action_north", 6, 32, 46, 0xFBDCB9, true),
        ("action_north", 6, 36, 28, 0x5B566E, false),
        ("action_north", 6, 38, 27, 0x91879B, false),
        ("action_north", 6, 39, 27, 0xD7C8CE, false),
        ("action_north", 6, 35, 40, 0xF4AB50, false),
        ("action_north", 6, 35, 44, 0x721943, false),
        ("action_north", 6, 36, 41, 0xA03E59, false),
        ("action_north", 6, 37, 41, 0xEB8C8C, false),
        ("action_north", 6, 36, 42, 0xFBC9BB, false),
        ("action_south", 0, 40, 34, 0xFBDCB9, true),
        ("action_south", 0, 40, 33, 0xF3C193, true),
        ("action_south", 0, 40, 32, 0xE08C6A, true),
        ("action_south", 0, 35, 36, 0xA45838, true),
        ("action_south", 0, 37, 40, 0xA45838, true),
        ("action_south", 0, 37, 34, 0x000000, false),
        ("action_south", 0, 37, 36, 0xC2B9BE, false),
        ("action_south", 0, 37, 37, 0xECF0E9, false),
        ("action_south", 0, 36, 39, 0xA45838, true),
        ("action_south", 0, 36, 29, 0x5B566E, false),
        ("action_south", 0, 37, 28, 0x91879B, false),
        ("action_south", 0, 38, 28, 0xD7C8CE, false),
        ("action_south", 0, 35, 40, 0xF4AB50, false),
        ("action_south", 0, 35, 41, 0xFFE990, false),
        ("action_south", 0, 39, 46, 0x700815, false),
        ("action_south", 0, 35, 44, 0x721943, false),
        ("action_south", 0, 37, 45, 0x9B212C, false),
        ("action_south", 0, 37, 42, 0xA03E59, false),
        ("action_south", 0, 36, 42, 0xEB8C8C, false),
        ("action_south", 0, 36, 43, 0xFBC9BB, false),
        ("action_south", 0, 37, 44, 0xD84B3A, false),
        ("action_south", 1, 40, 33, 0xFBDCB9, true),
        ("action_south", 1, 40, 32, 0xF3C193, true),
        ("action_south", 1, 40, 31, 0xE08C6A, true),
        ("action_south", 1, 35, 35, 0xA45838, true),
        ("action_south", 1, 39, 40, 0xE08C6A, true),
        ("action_south", 1, 37, 33, 0x000000, false),
        ("action_south", 1, 37, 35, 0xC2B9BE, false),
        ("action_south", 1, 37, 36, 0xECF0E9, false),
        ("action_south", 1, 37, 39, 0xA45838, true),
        ("action_south", 1, 36, 28, 0x5B566E, false),
        ("action_south", 1, 37, 27, 0x91879B, false),
        ("action_south", 1, 38, 27, 0xD7C8CE, false),
        ("action_south", 1, 35, 39, 0xF4AB50, false),
        ("action_south", 1, 35, 40, 0xFFE990, false),
        ("action_south", 1, 39, 45, 0x700815, false),
        ("action_south", 1, 42, 41, 0x721943, false),
        ("action_south", 1, 42, 44, 0x9B212C, false),
        ("action_south", 1, 37, 41, 0xA03E59, false),
        ("action_south", 1, 36, 41, 0xEB8C8C, false),
        ("action_south", 1, 36, 42, 0xFBC9BB, false),
        ("action_south", 1, 38, 43, 0xD84B3A, false),
        ("action_south", 2, 40, 33, 0xFBDCB9, true),
        ("action_south", 2, 40, 32, 0xF3C193, true),
        ("action_south", 2, 40, 31, 0xE08C6A, true),
        ("action_south", 2, 35, 35, 0xA45838, true),
        ("action_south", 2, 39, 40, 0xE08C6A, true),
        ("action_south", 2, 37, 33, 0x000000, false),
        ("action_south", 2, 37, 35, 0xC2B9BE, false),
        ("action_south", 2, 37, 36, 0xECF0E9, false),
        ("action_south", 2, 37, 39, 0xA45838, true),
        ("action_south", 2, 36, 28, 0x5B566E, false),
        ("action_south", 2, 37, 27, 0x91879B, false),
        ("action_south", 2, 38, 27, 0xD7C8CE, false),
        ("action_south", 2, 35, 39, 0xF4AB50, false),
        ("action_south", 2, 35, 40, 0xFFE990, false),
        ("action_south", 2, 40, 45, 0x700815, false),
        ("action_south", 2, 42, 41, 0x721943, false),
        ("action_south", 2, 42, 44, 0x9B212C, false),
        ("action_south", 2, 37, 41, 0xA03E59, false),
        ("action_south", 2, 36, 41, 0xEB8C8C, false),
        ("action_south", 2, 36, 42, 0xFBC9BB, false),
        ("action_south", 2, 41, 43, 0xD84B3A, false),
        ("action_south", 3, 40, 33, 0xFBDCB9, true),
        ("action_south", 3, 40, 32, 0xF3C193, true),
        ("action_south", 3, 40, 31, 0xE08C6A, true),
        ("action_south", 3, 35, 35, 0xA45838, true),
        ("action_south", 3, 39, 40, 0xE08C6A, true),
        ("action_south", 3, 37, 33, 0x000000, false),
        ("action_south", 3, 37, 35, 0xC2B9BE, false),
        ("action_south", 3, 37, 36, 0xECF0E9, false),
        ("action_south", 3, 37, 39, 0xA45838, true),
        ("action_south", 3, 36, 28, 0x5B566E, false),
        ("action_south", 3, 37, 27, 0x91879B, false),
        ("action_south", 3, 38, 27, 0xD7C8CE, false),
        ("action_south", 3, 35, 39, 0xF4AB50, false),
        ("action_south", 3, 35, 40, 0xFFE990, false),
        ("action_south", 3, 39, 45, 0x700815, false),
        ("action_south", 3, 42, 41, 0x721943, false),
        ("action_south", 3, 42, 44, 0x9B212C, false),
        ("action_south", 3, 37, 41, 0xA03E59, false),
        ("action_south", 3, 36, 41, 0xEB8C8C, false),
        ("action_south", 3, 36, 42, 0xFBC9BB, false),
        ("action_south", 3, 38, 43, 0xD84B3A, false),
        ("action_south", 4, 40, 33, 0xFBDCB9, true),
        ("action_south", 4, 40, 32, 0xF3C193, true),
        ("action_south", 4, 40, 31, 0xE08C6A, true),
        ("action_south", 4, 35, 35, 0xA45838, true),
        ("action_south", 4, 39, 40, 0xE08C6A, true),
        ("action_south", 4, 37, 33, 0x000000, false),
        ("action_south", 4, 37, 35, 0xC2B9BE, false),
        ("action_south", 4, 37, 36, 0xECF0E9, false),
        ("action_south", 4, 37, 39, 0xA45838, true),
        ("action_south", 4, 36, 28, 0x5B566E, false),
        ("action_south", 4, 37, 27, 0x91879B, false),
        ("action_south", 4, 38, 27, 0xD7C8CE, false),
        ("action_south", 4, 35, 39, 0xF4AB50, false),
        ("action_south", 4, 35, 40, 0xFFE990, false),
        ("action_south", 4, 40, 45, 0x700815, false),
        ("action_south", 4, 42, 41, 0x721943, false),
        ("action_south", 4, 42, 44, 0x9B212C, false),
        ("action_south", 4, 37, 41, 0xA03E59, false),
        ("action_south", 4, 36, 41, 0xEB8C8C, false),
        ("action_south", 4, 36, 42, 0xFBC9BB, false),
        ("action_south", 4, 41, 43, 0xD84B3A, false),
        ("action_south", 5, 40, 34, 0xFBDCB9, true),
        ("action_south", 5, 40, 33, 0xF3C193, true),
        ("action_south", 5, 40, 32, 0xE08C6A, true),
        ("action_south", 5, 35, 36, 0xA45838, true),
        ("action_south", 5, 37, 40, 0xA45838, true),
        ("action_south", 5, 37, 34, 0x000000, false),
        ("action_south", 5, 37, 36, 0xC2B9BE, false),
        ("action_south", 5, 37, 37, 0xECF0E9, false),
        ("action_south", 5, 36, 39, 0xA45838, true),
        ("action_south", 5, 36, 29, 0x5B566E, false),
        ("action_south", 5, 37, 28, 0x91879B, false),
        ("action_south", 5, 38, 28, 0xD7C8CE, false),
        ("action_south", 5, 35, 40, 0xF4AB50, false),
        ("action_south", 5, 35, 41, 0xFFE990, false),
        ("action_south", 5, 39, 46, 0x700815, false),
        ("action_south", 5, 35, 44, 0x721943, false),
        ("action_south", 5, 37, 45, 0x9B212C, false),
        ("action_south", 5, 37, 42, 0xA03E59, false),
        ("action_south", 5, 36, 42, 0xEB8C8C, false),
        ("action_south", 5, 36, 43, 0xFBC9BB, false),
        ("action_south", 5, 37, 44, 0xD84B3A, false),
        ("action_south", 6, 40, 33, 0xFBDCB9, true),
        ("action_south", 6, 40, 32, 0xF3C193, true),
        ("action_south", 6, 40, 31, 0xE08C6A, true),
        ("action_south", 6, 35, 35, 0xA45838, true),
        ("action_south", 6, 39, 40, 0xE08C6A, true),
        ("action_south", 6, 37, 33, 0x000000, false),
        ("action_south", 6, 37, 35, 0xC2B9BE, false),
        ("action_south", 6, 37, 36, 0xECF0E9, false),
        ("action_south", 6, 37, 39, 0xA45838, true),
        ("action_south", 6, 36, 28, 0x5B566E, false),
        ("action_south", 6, 37, 27, 0x91879B, false),
        ("action_south", 6, 38, 27, 0xD7C8CE, false),
        ("action_south", 6, 35, 39, 0xF4AB50, false),
        ("action_south", 6, 35, 40, 0xFFE990, false),
        ("action_south", 6, 39, 45, 0x700815, false),
        ("action_south", 6, 35, 44, 0x721943, false),
        ("action_south", 6, 37, 44, 0x9B212C, false),
        ("action_south", 6, 37, 41, 0xA03E59, false),
        ("action_south", 6, 36, 41, 0xEB8C8C, false),
        ("action_south", 6, 36, 42, 0xFBC9BB, false),
        ("action_south", 6, 37, 43, 0xD84B3A, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[53, 46, 46, 46, 46, 53, 53]),
        ("action_north", &[8, 6, 6, 6, 6, 7, 10]),
        ("action_south", &[60, 59, 60, 59, 60, 60, 63]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [8, 9, 10, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_elsie_spring";
            let asset = format!("assets/animations/NPCs/Elsie/Sprites/Spring/{prefix}_{case}.png");
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
                // All four reviewed source shades are skin in this pinned batch.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Elsie Spring material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin independently of target colors.
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
