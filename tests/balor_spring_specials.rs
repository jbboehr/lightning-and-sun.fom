use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/balor-winter-world-study and the local accepted Balor world baseline"]
fn balor_spring_specials_cover_skin_and_preserve_props() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-winter-world-study");
    let baseline = root
        .join("generated/characters-balor-valen-eiland-spring-reactions-trial/characters/balor");
    let set = std::env::var_os("FOM_BALOR_SPRING_SPECIALS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/balor-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..138];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..138], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 207);
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
    // Fixed source-material landmarks cover every translated jump frame and
    // coin, gem and hair-flip phase. Coin/gem facets, sparkles, hair highlights
    // and trousers are distinct from the four reviewed world skin shades.
    let landmarks = [
        ("coin_flip_south", 0, 38, 35, 0xFCD9B3, true),
        ("coin_flip_south", 0, 36, 37, 0x672115, true),
        ("coin_flip_south", 0, 39, 28, 0x686589, false),
        ("coin_flip_south", 0, 36, 46, 0x612026, false),
        ("coin_flip_south", 1, 38, 35, 0xFCD9B3, true),
        ("coin_flip_south", 1, 36, 37, 0x672115, true),
        ("coin_flip_south", 1, 39, 28, 0x686589, false),
        ("coin_flip_south", 1, 36, 46, 0x612026, false),
        ("coin_flip_south", 2, 38, 35, 0xFCD9B3, true),
        ("coin_flip_south", 2, 36, 37, 0x672115, true),
        ("coin_flip_south", 2, 39, 28, 0x686589, false),
        ("coin_flip_south", 2, 37, 46, 0x612026, false),
        ("coin_flip_south", 3, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 3, 36, 38, 0x672115, true),
        ("coin_flip_south", 3, 39, 29, 0x686589, false),
        ("coin_flip_south", 3, 37, 47, 0x612026, false),
        ("coin_flip_south", 4, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 4, 36, 38, 0x672115, true),
        ("coin_flip_south", 4, 39, 29, 0x686589, false),
        ("coin_flip_south", 4, 37, 47, 0x612026, false),
        ("coin_flip_south", 5, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 5, 36, 38, 0x672115, true),
        ("coin_flip_south", 5, 39, 29, 0x686589, false),
        ("coin_flip_south", 5, 37, 47, 0x612026, false),
        ("coin_flip_south", 5, 47, 37, 0xFFD942, false),
        ("coin_flip_south", 6, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 6, 36, 38, 0x672115, true),
        ("coin_flip_south", 6, 39, 29, 0x686589, false),
        ("coin_flip_south", 6, 37, 47, 0x612026, false),
        ("coin_flip_south", 6, 48, 30, 0xFFD942, false),
        ("coin_flip_south", 6, 48, 31, 0xB77F3A, false),
        ("coin_flip_south", 6, 47, 30, 0xFFFFFF, false),
        ("coin_flip_south", 7, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 7, 36, 38, 0x672115, true),
        ("coin_flip_south", 7, 39, 29, 0x686589, false),
        ("coin_flip_south", 7, 37, 47, 0x612026, false),
        ("coin_flip_south", 7, 48, 27, 0xFFD942, false),
        ("coin_flip_south", 7, 47, 27, 0xFFFFFF, false),
        ("coin_flip_south", 8, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 8, 36, 38, 0x672115, true),
        ("coin_flip_south", 8, 39, 29, 0x686589, false),
        ("coin_flip_south", 8, 37, 47, 0x612026, false),
        ("coin_flip_south", 8, 47, 25, 0xFFD942, false),
        ("coin_flip_south", 8, 48, 25, 0xB77F3A, false),
        ("coin_flip_south", 9, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 9, 36, 38, 0x672115, true),
        ("coin_flip_south", 9, 39, 29, 0x686589, false),
        ("coin_flip_south", 9, 37, 47, 0x612026, false),
        ("coin_flip_south", 9, 47, 24, 0xFFD942, false),
        ("coin_flip_south", 9, 48, 24, 0xB77F3A, false),
        ("coin_flip_south", 10, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 10, 36, 38, 0x672115, true),
        ("coin_flip_south", 10, 39, 29, 0x686589, false),
        ("coin_flip_south", 10, 37, 47, 0x612026, false),
        ("coin_flip_south", 10, 48, 23, 0xFFD942, false),
        ("coin_flip_south", 10, 48, 24, 0xB77F3A, false),
        ("coin_flip_south", 10, 47, 23, 0xFFFFFF, false),
        ("coin_flip_south", 11, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 11, 36, 38, 0x672115, true),
        ("coin_flip_south", 11, 39, 29, 0x686589, false),
        ("coin_flip_south", 11, 37, 47, 0x612026, false),
        ("coin_flip_south", 11, 48, 25, 0xFFD942, false),
        ("coin_flip_south", 11, 47, 25, 0xFFFFFF, false),
        ("coin_flip_south", 12, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 12, 36, 38, 0x672115, true),
        ("coin_flip_south", 12, 39, 29, 0x686589, false),
        ("coin_flip_south", 12, 37, 47, 0x612026, false),
        ("coin_flip_south", 12, 47, 30, 0xFFD942, false),
        ("coin_flip_south", 12, 48, 30, 0xB77F3A, false),
        ("coin_flip_south", 12, 47, 36, 0xFFFFFF, false),
        ("coin_flip_south", 13, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 13, 36, 38, 0x672115, true),
        ("coin_flip_south", 13, 39, 29, 0x686589, false),
        ("coin_flip_south", 13, 37, 47, 0x612026, false),
        ("coin_flip_south", 13, 47, 36, 0xFFD942, false),
        ("coin_flip_south", 13, 48, 36, 0xB77F3A, false),
        ("coin_flip_south", 14, 38, 36, 0xFCD9B3, true),
        ("coin_flip_south", 14, 36, 38, 0x672115, true),
        ("coin_flip_south", 14, 39, 29, 0x686589, false),
        ("coin_flip_south", 14, 37, 47, 0x612026, false),
        ("coin_flip_south", 15, 38, 35, 0xFCD9B3, true),
        ("coin_flip_south", 15, 36, 37, 0x672115, true),
        ("coin_flip_south", 15, 39, 28, 0x686589, false),
        ("coin_flip_south", 15, 37, 46, 0x612026, false),
        ("coin_flip_south", 16, 40, 35, 0xFCD9B3, true),
        ("coin_flip_south", 16, 36, 37, 0x672115, true),
        ("coin_flip_south", 16, 39, 28, 0x686589, false),
        ("coin_flip_south", 16, 36, 46, 0x612026, false),
        ("hair_flip_south", 0, 35, 36, 0xFCD9B3, true),
        ("hair_flip_south", 0, 36, 38, 0x672115, true),
        ("hair_flip_south", 0, 39, 29, 0x686589, false),
        ("hair_flip_south", 0, 37, 46, 0x612026, false),
        ("hair_flip_south", 0, 42, 39, 0xB395D6, false),
        ("hair_flip_south", 0, 42, 40, 0xDEDAE9, false),
        ("hair_flip_south", 1, 43, 34, 0xFCD9B3, true),
        ("hair_flip_south", 1, 36, 37, 0x672115, true),
        ("hair_flip_south", 1, 39, 28, 0x686589, false),
        ("hair_flip_south", 1, 37, 46, 0x612026, false),
        ("hair_flip_south", 1, 45, 40, 0xB395D6, false),
        ("hair_flip_south", 2, 49, 34, 0xFCD9B3, true),
        ("hair_flip_south", 2, 36, 37, 0x672115, true),
        ("hair_flip_south", 2, 39, 28, 0x686589, false),
        ("hair_flip_south", 2, 36, 46, 0x612026, false),
        ("hair_flip_south", 2, 45, 40, 0xB395D6, false),
        ("hair_flip_south", 3, 38, 36, 0xFCD9B3, true),
        ("hair_flip_south", 3, 39, 38, 0x672115, true),
        ("hair_flip_south", 3, 42, 29, 0x686589, false),
        ("hair_flip_south", 3, 45, 46, 0x612026, false),
        ("hair_flip_south", 3, 39, 39, 0xB395D6, false),
        ("hair_flip_south", 4, 36, 35, 0xFCD9B3, true),
        ("hair_flip_south", 4, 37, 37, 0x672115, true),
        ("hair_flip_south", 4, 40, 28, 0x686589, false),
        ("hair_flip_south", 4, 37, 46, 0x612026, false),
        ("hair_flip_south", 4, 35, 41, 0xB395D6, false),
        ("jump_east", 0, 26, 38, 0xFCD9B3, true),
        ("jump_east", 0, 22, 40, 0x672115, true),
        ("jump_east", 0, 24, 31, 0x686589, false),
        ("jump_east", 0, 22, 47, 0x612026, false),
        ("jump_east", 1, 27, 33, 0xFCD9B3, true),
        ("jump_east", 1, 23, 35, 0x672115, true),
        ("jump_east", 1, 25, 26, 0x686589, false),
        ("jump_east", 1, 23, 42, 0x612026, false),
        ("jump_east", 2, 29, 27, 0xFCD9B3, true),
        ("jump_east", 2, 25, 29, 0x672115, true),
        ("jump_east", 2, 27, 20, 0x686589, false),
        ("jump_east", 2, 25, 36, 0x612026, false),
        ("jump_east", 3, 31, 24, 0xFCD9B3, true),
        ("jump_east", 3, 27, 26, 0x672115, true),
        ("jump_east", 3, 29, 17, 0x686589, false),
        ("jump_east", 3, 27, 33, 0x612026, false),
        ("jump_east", 4, 33, 21, 0xFCD9B3, true),
        ("jump_east", 4, 29, 23, 0x672115, true),
        ("jump_east", 4, 31, 14, 0x686589, false),
        ("jump_east", 4, 29, 30, 0x612026, false),
        ("jump_east", 5, 35, 20, 0xFCD9B3, true),
        ("jump_east", 5, 31, 22, 0x672115, true),
        ("jump_east", 5, 33, 13, 0x686589, false),
        ("jump_east", 5, 31, 29, 0x612026, false),
        ("jump_east", 6, 37, 18, 0xFCD9B3, true),
        ("jump_east", 6, 34, 21, 0x672115, true),
        ("jump_east", 6, 35, 11, 0x686589, false),
        ("jump_east", 6, 32, 26, 0x612026, false),
        ("jump_east", 7, 39, 17, 0xFCD9B3, true),
        ("jump_east", 7, 36, 20, 0x672115, true),
        ("jump_east", 7, 37, 10, 0x686589, false),
        ("jump_east", 7, 34, 25, 0x612026, false),
        ("jump_east", 8, 41, 17, 0xFCD9B3, true),
        ("jump_east", 8, 38, 20, 0x672115, true),
        ("jump_east", 8, 39, 10, 0x686589, false),
        ("jump_east", 8, 36, 25, 0x612026, false),
        ("jump_east", 9, 43, 16, 0xFCD9B3, true),
        ("jump_east", 9, 39, 18, 0x672115, true),
        ("jump_east", 9, 41, 9, 0x686589, false),
        ("jump_east", 9, 38, 24, 0x612026, false),
        ("jump_east", 10, 47, 16, 0xFCD9B3, true),
        ("jump_east", 10, 43, 18, 0x672115, true),
        ("jump_east", 10, 45, 9, 0x686589, false),
        ("jump_east", 10, 42, 24, 0x612026, false),
        ("jump_east", 11, 49, 16, 0xFCD9B3, true),
        ("jump_east", 11, 45, 18, 0x672115, true),
        ("jump_east", 11, 47, 9, 0x686589, false),
        ("jump_east", 11, 44, 24, 0x612026, false),
        ("jump_east", 12, 49, 16, 0xFCD9B3, true),
        ("jump_east", 12, 45, 18, 0x672115, true),
        ("jump_east", 12, 47, 9, 0x686589, false),
        ("jump_east", 12, 44, 24, 0x612026, false),
        ("jump_east", 13, 51, 17, 0xFCD9B3, true),
        ("jump_east", 13, 47, 19, 0x672115, true),
        ("jump_east", 13, 49, 10, 0x686589, false),
        ("jump_east", 13, 46, 25, 0x612026, false),
        ("jump_east", 14, 53, 17, 0xFCD9B3, true),
        ("jump_east", 14, 49, 19, 0x672115, true),
        ("jump_east", 14, 51, 10, 0x686589, false),
        ("jump_east", 14, 48, 25, 0x612026, false),
        ("jump_east", 15, 55, 18, 0xFCD9B3, true),
        ("jump_east", 15, 51, 20, 0x672115, true),
        ("jump_east", 15, 53, 11, 0x686589, false),
        ("jump_east", 15, 50, 27, 0x612026, false),
        ("jump_east", 16, 57, 20, 0xFCD9B3, true),
        ("jump_east", 16, 53, 22, 0x672115, true),
        ("jump_east", 16, 55, 13, 0x686589, false),
        ("jump_east", 16, 52, 29, 0x612026, false),
        ("jump_east", 17, 59, 21, 0xFCD9B3, true),
        ("jump_east", 17, 55, 23, 0x672115, true),
        ("jump_east", 17, 57, 14, 0x686589, false),
        ("jump_east", 17, 54, 30, 0x612026, false),
        ("jump_east", 18, 61, 23, 0xFCD9B3, true),
        ("jump_east", 18, 57, 25, 0x672115, true),
        ("jump_east", 18, 59, 16, 0x686589, false),
        ("jump_east", 18, 56, 32, 0x612026, false),
        ("jump_east", 19, 63, 27, 0xFCD9B3, true),
        ("jump_east", 19, 59, 29, 0x672115, true),
        ("jump_east", 19, 61, 20, 0x686589, false),
        ("jump_east", 19, 58, 36, 0x612026, false),
        ("jump_east", 20, 65, 32, 0xFCD9B3, true),
        ("jump_east", 20, 61, 34, 0x672115, true),
        ("jump_east", 20, 63, 25, 0x686589, false),
        ("jump_east", 20, 60, 41, 0x612026, false),
        ("jump_east", 21, 65, 40, 0xFCD9B3, true),
        ("jump_east", 21, 61, 42, 0x672115, true),
        ("jump_east", 21, 63, 33, 0x686589, false),
        ("jump_east", 21, 61, 49, 0x612026, false),
        ("inspect_gem_start_south", 0, 38, 35, 0xFCD9B3, true),
        ("inspect_gem_start_south", 0, 36, 37, 0x672115, true),
        ("inspect_gem_start_south", 0, 39, 28, 0x686589, false),
        ("inspect_gem_start_south", 0, 36, 46, 0x612026, false),
        ("inspect_gem_start_south", 1, 38, 35, 0xFCD9B3, true),
        ("inspect_gem_start_south", 1, 36, 37, 0x672115, true),
        ("inspect_gem_start_south", 1, 39, 28, 0x686589, false),
        ("inspect_gem_start_south", 1, 36, 46, 0x612026, false),
        ("inspect_gem_start_south", 2, 38, 35, 0xFCD9B3, true),
        ("inspect_gem_start_south", 2, 36, 37, 0x672115, true),
        ("inspect_gem_start_south", 2, 39, 28, 0x686589, false),
        ("inspect_gem_start_south", 2, 36, 46, 0x612026, false),
        ("inspect_gem_start_south", 3, 38, 36, 0xFCD9B3, true),
        ("inspect_gem_start_south", 3, 36, 38, 0x672115, true),
        ("inspect_gem_start_south", 3, 39, 29, 0x686589, false),
        ("inspect_gem_start_south", 3, 37, 47, 0x612026, false),
        ("inspect_gem_loop_south", 0, 38, 36, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 0, 36, 38, 0x672115, true),
        ("inspect_gem_loop_south", 0, 39, 29, 0x686589, false),
        ("inspect_gem_loop_south", 0, 37, 47, 0x612026, false),
        ("inspect_gem_loop_south", 0, 46, 39, 0x3959A4, false),
        ("inspect_gem_loop_south", 0, 47, 39, 0x52A1E8, false),
        ("inspect_gem_loop_south", 0, 48, 39, 0x7CF8FF, false),
        ("inspect_gem_loop_south", 0, 48, 40, 0xFFFFFF, false),
        ("inspect_gem_loop_south", 1, 38, 36, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 1, 36, 38, 0x672115, true),
        ("inspect_gem_loop_south", 1, 39, 29, 0x686589, false),
        ("inspect_gem_loop_south", 1, 37, 47, 0x612026, false),
        ("inspect_gem_loop_south", 1, 46, 39, 0x3959A4, false),
        ("inspect_gem_loop_south", 1, 47, 39, 0x52A1E8, false),
        ("inspect_gem_loop_south", 1, 48, 39, 0x7CF8FF, false),
        ("inspect_gem_loop_south", 1, 50, 37, 0xFFFFFF, false),
        ("inspect_gem_loop_south", 2, 38, 36, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 2, 36, 38, 0x672115, true),
        ("inspect_gem_loop_south", 2, 39, 29, 0x686589, false),
        ("inspect_gem_loop_south", 2, 37, 47, 0x612026, false),
        ("inspect_gem_loop_south", 2, 46, 39, 0x3959A4, false),
        ("inspect_gem_loop_south", 2, 47, 39, 0x52A1E8, false),
        ("inspect_gem_loop_south", 2, 49, 37, 0x7CF8FF, false),
        ("inspect_gem_loop_south", 2, 50, 35, 0xFFFFFF, false),
        ("inspect_gem_loop_south", 3, 38, 36, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 3, 36, 38, 0x672115, true),
        ("inspect_gem_loop_south", 3, 39, 29, 0x686589, false),
        ("inspect_gem_loop_south", 3, 37, 47, 0x612026, false),
        ("inspect_gem_loop_south", 3, 46, 39, 0x3959A4, false),
        ("inspect_gem_loop_south", 3, 47, 39, 0x52A1E8, false),
        ("inspect_gem_loop_south", 3, 50, 36, 0x7CF8FF, false),
        ("inspect_gem_loop_south", 3, 51, 35, 0xFFFFFF, false),
        ("inspect_gem_loop_south", 4, 38, 36, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 4, 36, 38, 0x672115, true),
        ("inspect_gem_loop_south", 4, 39, 29, 0x686589, false),
        ("inspect_gem_loop_south", 4, 37, 47, 0x612026, false),
        ("inspect_gem_loop_south", 4, 46, 39, 0x3959A4, false),
        ("inspect_gem_loop_south", 4, 47, 39, 0x52A1E8, false),
        ("inspect_gem_loop_south", 4, 48, 36, 0x7CF8FF, false),
        ("inspect_gem_loop_south", 4, 49, 37, 0xFFFFFF, false),
        ("inspect_gem_loop_south", 5, 38, 36, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 5, 36, 38, 0x672115, true),
        ("inspect_gem_loop_south", 5, 39, 29, 0x686589, false),
        ("inspect_gem_loop_south", 5, 37, 47, 0x612026, false),
        ("inspect_gem_loop_south", 5, 46, 39, 0x3959A4, false),
        ("inspect_gem_loop_south", 5, 47, 39, 0x52A1E8, false),
        ("inspect_gem_loop_south", 5, 50, 37, 0x7CF8FF, false),
        ("inspect_gem_loop_south", 5, 50, 38, 0xFFFFFF, false),
        ("inspect_gem_loop_south", 6, 38, 36, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 6, 36, 38, 0x672115, true),
        ("inspect_gem_loop_south", 6, 39, 29, 0x686589, false),
        ("inspect_gem_loop_south", 6, 37, 47, 0x612026, false),
        ("inspect_gem_loop_south", 6, 46, 39, 0x3959A4, false),
        ("inspect_gem_loop_south", 6, 47, 39, 0x52A1E8, false),
        ("inspect_gem_loop_south", 6, 48, 39, 0x7CF8FF, false),
        ("inspect_gem_loop_south", 6, 48, 40, 0xFFFFFF, false),
        ("inspect_gem_end_south", 0, 38, 36, 0xFCD9B3, true),
        ("inspect_gem_end_south", 0, 36, 38, 0x672115, true),
        ("inspect_gem_end_south", 0, 39, 29, 0x686589, false),
        ("inspect_gem_end_south", 0, 37, 47, 0x612026, false),
        ("inspect_gem_end_south", 1, 38, 35, 0xFCD9B3, true),
        ("inspect_gem_end_south", 1, 36, 37, 0x672115, true),
        ("inspect_gem_end_south", 1, 39, 28, 0x686589, false),
        ("inspect_gem_end_south", 1, 36, 46, 0x612026, false),
        ("inspect_gem_end_south", 2, 38, 35, 0xFCD9B3, true),
        ("inspect_gem_end_south", 2, 36, 37, 0x672115, true),
        ("inspect_gem_end_south", 2, 39, 28, 0x686589, false),
        ("inspect_gem_end_south", 2, 36, 46, 0x612026, false),
        ("inspect_gem_end_south", 3, 38, 35, 0xFCD9B3, true),
        ("inspect_gem_end_south", 3, 36, 37, 0x672115, true),
        ("inspect_gem_end_south", 3, 39, 28, 0x686589, false),
        ("inspect_gem_end_south", 3, 36, 46, 0x612026, false),
        ("inspect_gem_loop_south", 0, 49, 43, 0xD37A57, true),
        ("coin_flip_south", 0, 36, 46, 0x612026, false),
    ];
    let cases: [(&str, &[usize]); 6] = [
        (
            "coin_flip_south",
            &[
                46, 43, 45, 47, 44, 47, 48, 48, 48, 48, 48, 48, 48, 48, 47, 45, 41,
            ],
        ),
        ("hair_flip_south", &[24, 49, 49, 54, 47]),
        (
            "jump_east",
            &[
                33, 33, 33, 33, 33, 33, 36, 36, 36, 37, 37, 37, 37, 37, 37, 35, 35, 35, 35, 35, 35,
                33,
            ],
        ),
        ("inspect_gem_start_south", &[46, 43, 46, 50]),
        ("inspect_gem_loop_south", &[38, 38, 38, 38, 38, 38, 38]),
        ("inspect_gem_end_south", &[50, 46, 43, 46]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let mut common_mask = None;
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = preset["colors"].as_array().unwrap()[7..11]
            .iter()
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        let mut mask = Vec::new();
        for (case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_balor_specialanimation_spring";
            let asset = format!("assets/animations/NPCs/Balor/Sprites/Spring/{prefix}_{case}.png");
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
            let mut trouser_shadows = 0;
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                if p.0 == rgba(0x612026) {
                    assert_eq!(p, q, "trouser shadow changed: {id} {case} [{x},{y}]");
                    trouser_shadows += 1;
                }
                // All four reviewed world shades are skin in these Spring strips.
                // Independently guard every omitted skin pixel and non-skin material.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Balor Spring material mismatch: {id} {case} [{x},{y}]"
                );
                mask.push(p != q);
                if p != q {
                    let index = source.iter().position(|c| rgba(*c) == p.0).expect(
                        "hair, scarf, shirt, bag, trousers, eyes or another non-skin color changed",
                    );
                    assert_eq!(q.0, rgba(target[index]));
                    per_frame[x as usize / 80] += 1;
                }
            }
            assert_eq!(
                trouser_shadows,
                match case {
                    "coin_flip_south" => 71,
                    "hair_flip_south" => 24,
                    "jump_east" => 100,
                    "inspect_gem_start_south" | "inspect_gem_end_south" => 19,
                    "inspect_gem_loop_south" => 28,
                    _ => unreachable!(),
                }
            );
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
