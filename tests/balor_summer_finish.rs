use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/balor-summer-finish-study and the local accepted Balor world baseline"]
fn balor_summer_finish_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-summer-finish-study");
    let baseline = root
        .join("generated/characters-balor-valen-eiland-summer-expansion-trial/characters/balor");
    let set = std::env::var_os("FOM_BALOR_SUMMER_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_BALOR_SUMMER_FINISH_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/balor-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..166];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..166], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 173);
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
    // Literal source-grid landmarks include exposed skin and protected materials.
    // These expectations come from the source art, independently of recipe seeds.
    let landmarks = [
        ("coin_flip_south", 0, 40, 47, 0x000000, false),
        ("coin_flip_south", 0, 44, 38, 0x281846, false),
        ("coin_flip_south", 0, 33, 44, 0x374050, false),
        ("coin_flip_south", 0, 42, 48, 0x494288, false),
        ("coin_flip_south", 0, 38, 33, 0x4A3D66, false),
        ("coin_flip_south", 0, 34, 45, 0x525A62, false),
        ("coin_flip_south", 0, 37, 52, 0x612026, false),
        ("coin_flip_south", 0, 33, 46, 0x672115, true),
        ("coin_flip_south", 0, 35, 42, 0x686589, false),
        ("coin_flip_south", 0, 38, 48, 0x6960AD, false),
        ("coin_flip_south", 0, 37, 53, 0x81454F, false),
        ("coin_flip_south", 0, 40, 30, 0x9B83B7, false),
        ("coin_flip_south", 0, 38, 45, 0xA56B81, false),
        ("coin_flip_south", 0, 35, 43, 0xB395D6, false),
        ("coin_flip_south", 0, 41, 46, 0xB3AFBD, false),
        ("coin_flip_south", 0, 35, 44, 0xD37A57, true),
        ("coin_flip_south", 0, 38, 43, 0xDEDAE9, false),
        ("coin_flip_south", 0, 37, 51, 0xF0B988, true),
        ("coin_flip_south", 0, 32, 45, 0xFCD9B3, true),
        ("coin_flip_south", 0, 40, 46, 0xFFFFFF, false),
        ("coin_flip_south", 1, 33, 46, 0x672115, true),
        ("coin_flip_south", 1, 35, 44, 0xD37A57, true),
        ("coin_flip_south", 1, 44, 45, 0xF0B988, true),
        ("coin_flip_south", 1, 33, 45, 0xFCD9B3, true),
        ("coin_flip_south", 2, 34, 46, 0x672115, true),
        ("coin_flip_south", 2, 35, 44, 0xD37A57, true),
        ("coin_flip_south", 2, 37, 51, 0xF0B988, true),
        ("coin_flip_south", 2, 34, 45, 0xFCD9B3, true),
        ("coin_flip_south", 3, 46, 41, 0x672115, true),
        ("coin_flip_south", 3, 40, 40, 0xD37A57, true),
        ("coin_flip_south", 3, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 3, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 4, 46, 41, 0x672115, true),
        ("coin_flip_south", 4, 40, 40, 0xD37A57, true),
        ("coin_flip_south", 4, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 4, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 5, 36, 45, 0x672115, true),
        ("coin_flip_south", 5, 45, 40, 0xD37A57, true),
        ("coin_flip_south", 5, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 5, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 6, 36, 45, 0x672115, true),
        ("coin_flip_south", 6, 45, 40, 0xD37A57, true),
        ("coin_flip_south", 6, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 6, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 7, 36, 45, 0x672115, true),
        ("coin_flip_south", 7, 45, 40, 0xD37A57, true),
        ("coin_flip_south", 7, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 7, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 8, 36, 45, 0x672115, true),
        ("coin_flip_south", 8, 45, 40, 0xD37A57, true),
        ("coin_flip_south", 8, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 8, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 9, 36, 45, 0x672115, true),
        ("coin_flip_south", 9, 45, 40, 0xD37A57, true),
        ("coin_flip_south", 9, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 9, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 10, 36, 45, 0x672115, true),
        ("coin_flip_south", 10, 45, 40, 0xD37A57, true),
        ("coin_flip_south", 10, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 10, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 11, 36, 45, 0x672115, true),
        ("coin_flip_south", 11, 45, 40, 0xD37A57, true),
        ("coin_flip_south", 11, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 11, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 12, 36, 45, 0x672115, true),
        ("coin_flip_south", 12, 45, 40, 0xD37A57, true),
        ("coin_flip_south", 12, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 12, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 13, 36, 45, 0x672115, true),
        ("coin_flip_south", 13, 45, 40, 0xD37A57, true),
        ("coin_flip_south", 13, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 13, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 14, 46, 41, 0x672115, true),
        ("coin_flip_south", 14, 40, 40, 0xD37A57, true),
        ("coin_flip_south", 14, 44, 42, 0xF0B988, true),
        ("coin_flip_south", 14, 33, 44, 0xFCD9B3, true),
        ("coin_flip_south", 15, 34, 46, 0x672115, true),
        ("coin_flip_south", 15, 35, 44, 0xD37A57, true),
        ("coin_flip_south", 15, 37, 51, 0xF0B988, true),
        ("coin_flip_south", 15, 34, 45, 0xFCD9B3, true),
        ("coin_flip_south", 16, 46, 46, 0x672115, true),
        ("coin_flip_south", 16, 35, 44, 0xD37A57, true),
        ("coin_flip_south", 16, 37, 51, 0xF0B988, true),
        ("coin_flip_south", 16, 47, 45, 0xFCD9B3, true),
        ("inspect_gem_end_south", 0, 37, 46, 0x000000, false),
        ("inspect_gem_end_south", 0, 35, 40, 0x281846, false),
        ("inspect_gem_end_south", 0, 46, 43, 0x374050, false),
        ("inspect_gem_end_south", 0, 42, 49, 0x494288, false),
        ("inspect_gem_end_south", 0, 38, 34, 0x4A3D66, false),
        ("inspect_gem_end_south", 0, 34, 45, 0x525A62, false),
        ("inspect_gem_end_south", 0, 38, 52, 0x612026, false),
        ("inspect_gem_end_south", 0, 36, 45, 0x672115, true),
        ("inspect_gem_end_south", 0, 41, 41, 0x686589, false),
        ("inspect_gem_end_south", 0, 38, 49, 0x6960AD, false),
        ("inspect_gem_end_south", 0, 38, 53, 0x81454F, false),
        ("inspect_gem_end_south", 0, 40, 31, 0x9B83B7, false),
        ("inspect_gem_end_south", 0, 39, 46, 0xA56B81, false),
        ("inspect_gem_end_south", 0, 42, 43, 0xB395D6, false),
        ("inspect_gem_end_south", 0, 41, 47, 0xB3AFBD, false),
        ("inspect_gem_end_south", 0, 44, 43, 0xD37A57, true),
        ("inspect_gem_end_south", 0, 41, 43, 0xDEDAE9, false),
        ("inspect_gem_end_south", 0, 47, 41, 0xF0B988, true),
        ("inspect_gem_end_south", 0, 33, 44, 0xFCD9B3, true),
        ("inspect_gem_end_south", 0, 40, 47, 0xFFFFFF, false),
        ("inspect_gem_end_south", 1, 34, 46, 0x672115, true),
        ("inspect_gem_end_south", 1, 35, 44, 0xD37A57, true),
        ("inspect_gem_end_south", 1, 37, 51, 0xF0B988, true),
        ("inspect_gem_end_south", 1, 34, 45, 0xFCD9B3, true),
        ("inspect_gem_end_south", 2, 33, 46, 0x672115, true),
        ("inspect_gem_end_south", 2, 35, 44, 0xD37A57, true),
        ("inspect_gem_end_south", 2, 44, 45, 0xF0B988, true),
        ("inspect_gem_end_south", 2, 33, 45, 0xFCD9B3, true),
        ("inspect_gem_end_south", 3, 33, 46, 0x672115, true),
        ("inspect_gem_end_south", 3, 35, 44, 0xD37A57, true),
        ("inspect_gem_end_south", 3, 37, 51, 0xF0B988, true),
        ("inspect_gem_end_south", 3, 32, 45, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 0, 45, 45, 0x000000, false),
        ("inspect_gem_loop_south", 0, 44, 38, 0x281846, false),
        ("inspect_gem_loop_south", 0, 36, 43, 0x374050, false),
        ("inspect_gem_loop_south", 0, 46, 41, 0x3959A4, false),
        ("inspect_gem_loop_south", 0, 42, 49, 0x494288, false),
        ("inspect_gem_loop_south", 0, 40, 33, 0x4A3D66, false),
        ("inspect_gem_loop_south", 0, 47, 42, 0x52A1E8, false),
        ("inspect_gem_loop_south", 0, 37, 52, 0x612026, false),
        ("inspect_gem_loop_south", 0, 35, 43, 0x672115, true),
        ("inspect_gem_loop_south", 0, 44, 42, 0x686589, false),
        ("inspect_gem_loop_south", 0, 38, 49, 0x6960AD, false),
        ("inspect_gem_loop_south", 0, 48, 41, 0x7CF8FF, false),
        ("inspect_gem_loop_south", 0, 37, 53, 0x81454F, false),
        ("inspect_gem_loop_south", 0, 40, 31, 0x9B83B7, false),
        ("inspect_gem_loop_south", 0, 38, 46, 0xA56B81, false),
        ("inspect_gem_loop_south", 0, 42, 44, 0xB395D6, false),
        ("inspect_gem_loop_south", 0, 41, 47, 0xB3AFBD, false),
        ("inspect_gem_loop_south", 0, 42, 35, 0xC2B9BE, false),
        ("inspect_gem_loop_south", 0, 34, 43, 0xD37A57, true),
        ("inspect_gem_loop_south", 0, 39, 44, 0xDEDAE9, false),
        ("inspect_gem_loop_south", 0, 41, 37, 0xECF0E9, false),
        ("inspect_gem_loop_south", 0, 46, 44, 0xF0B988, true),
        ("inspect_gem_loop_south", 0, 38, 41, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 0, 41, 46, 0xFFFFFF, false),
        ("inspect_gem_loop_south", 1, 35, 43, 0x672115, true),
        ("inspect_gem_loop_south", 1, 34, 43, 0xD37A57, true),
        ("inspect_gem_loop_south", 1, 46, 44, 0xF0B988, true),
        ("inspect_gem_loop_south", 1, 38, 41, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 2, 35, 43, 0x672115, true),
        ("inspect_gem_loop_south", 2, 34, 43, 0xD37A57, true),
        ("inspect_gem_loop_south", 2, 46, 44, 0xF0B988, true),
        ("inspect_gem_loop_south", 2, 38, 41, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 3, 35, 43, 0x672115, true),
        ("inspect_gem_loop_south", 3, 34, 43, 0xD37A57, true),
        ("inspect_gem_loop_south", 3, 46, 44, 0xF0B988, true),
        ("inspect_gem_loop_south", 3, 38, 41, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 4, 35, 43, 0x672115, true),
        ("inspect_gem_loop_south", 4, 34, 43, 0xD37A57, true),
        ("inspect_gem_loop_south", 4, 46, 44, 0xF0B988, true),
        ("inspect_gem_loop_south", 4, 38, 41, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 5, 35, 43, 0x672115, true),
        ("inspect_gem_loop_south", 5, 34, 43, 0xD37A57, true),
        ("inspect_gem_loop_south", 5, 46, 44, 0xF0B988, true),
        ("inspect_gem_loop_south", 5, 38, 41, 0xFCD9B3, true),
        ("inspect_gem_loop_south", 6, 35, 43, 0x672115, true),
        ("inspect_gem_loop_south", 6, 34, 43, 0xD37A57, true),
        ("inspect_gem_loop_south", 6, 46, 44, 0xF0B988, true),
        ("inspect_gem_loop_south", 6, 38, 41, 0xFCD9B3, true),
        ("inspect_gem_start_south", 0, 40, 47, 0x000000, false),
        ("inspect_gem_start_south", 0, 44, 38, 0x281846, false),
        ("inspect_gem_start_south", 0, 33, 44, 0x374050, false),
        ("inspect_gem_start_south", 0, 42, 48, 0x494288, false),
        ("inspect_gem_start_south", 0, 38, 33, 0x4A3D66, false),
        ("inspect_gem_start_south", 0, 34, 45, 0x525A62, false),
        ("inspect_gem_start_south", 0, 37, 52, 0x612026, false),
        ("inspect_gem_start_south", 0, 33, 46, 0x672115, true),
        ("inspect_gem_start_south", 0, 35, 42, 0x686589, false),
        ("inspect_gem_start_south", 0, 38, 48, 0x6960AD, false),
        ("inspect_gem_start_south", 0, 37, 53, 0x81454F, false),
        ("inspect_gem_start_south", 0, 40, 30, 0x9B83B7, false),
        ("inspect_gem_start_south", 0, 38, 45, 0xA56B81, false),
        ("inspect_gem_start_south", 0, 35, 43, 0xB395D6, false),
        ("inspect_gem_start_south", 0, 41, 46, 0xB3AFBD, false),
        ("inspect_gem_start_south", 0, 35, 44, 0xD37A57, true),
        ("inspect_gem_start_south", 0, 38, 43, 0xDEDAE9, false),
        ("inspect_gem_start_south", 0, 37, 51, 0xF0B988, true),
        ("inspect_gem_start_south", 0, 32, 45, 0xFCD9B3, true),
        ("inspect_gem_start_south", 0, 40, 46, 0xFFFFFF, false),
        ("inspect_gem_start_south", 1, 33, 46, 0x672115, true),
        ("inspect_gem_start_south", 1, 35, 44, 0xD37A57, true),
        ("inspect_gem_start_south", 1, 44, 45, 0xF0B988, true),
        ("inspect_gem_start_south", 1, 33, 45, 0xFCD9B3, true),
        ("inspect_gem_start_south", 2, 34, 46, 0x672115, true),
        ("inspect_gem_start_south", 2, 35, 44, 0xD37A57, true),
        ("inspect_gem_start_south", 2, 37, 51, 0xF0B988, true),
        ("inspect_gem_start_south", 2, 34, 45, 0xFCD9B3, true),
        ("inspect_gem_start_south", 3, 36, 45, 0x672115, true),
        ("inspect_gem_start_south", 3, 44, 43, 0xD37A57, true),
        ("inspect_gem_start_south", 3, 47, 41, 0xF0B988, true),
        ("inspect_gem_start_south", 3, 33, 44, 0xFCD9B3, true),
        ("read_sit_end_south", 0, 44, 46, 0x000000, false),
        ("read_sit_end_south", 0, 42, 30, 0x281846, false),
        ("read_sit_end_south", 0, 38, 35, 0x4A3D66, false),
        ("read_sit_end_south", 0, 37, 45, 0x606C76, false),
        ("read_sit_end_south", 0, 41, 50, 0x612026, false),
        ("read_sit_end_south", 0, 40, 41, 0x672115, true),
        ("read_sit_end_south", 0, 41, 34, 0x686589, false),
        ("read_sit_end_south", 0, 41, 51, 0x81454F, false),
        ("read_sit_end_south", 0, 38, 46, 0x98A2A6, false),
        ("read_sit_end_south", 0, 40, 32, 0x9B83B7, false),
        ("read_sit_end_south", 0, 42, 49, 0xB395D6, false),
        ("read_sit_end_south", 0, 38, 44, 0xC9AF9C, false),
        ("read_sit_end_south", 0, 41, 40, 0xD37A57, true),
        ("read_sit_end_south", 0, 42, 38, 0xECF0E9, false),
        ("read_sit_end_south", 0, 40, 40, 0xF0B988, true),
        ("read_sit_end_south", 0, 38, 42, 0xF6E4D7, false),
        ("read_sit_end_south", 0, 34, 47, 0xFCD9B3, true),
        ("read_sit_end_south", 1, 44, 47, 0x672115, true),
        ("read_sit_end_south", 1, 44, 38, 0xD37A57, true),
        ("read_sit_end_south", 1, 44, 44, 0xF0B988, true),
        ("read_sit_end_south", 1, 45, 46, 0xFCD9B3, true),
        ("read_sit_end_south", 2, 34, 47, 0x672115, true),
        ("read_sit_end_south", 2, 40, 40, 0xD37A57, true),
        ("read_sit_end_south", 2, 33, 46, 0xF0B988, true),
        ("read_sit_end_south", 2, 40, 42, 0xFCD9B3, true),
        ("read_sit_loop_south", 0, 43, 45, 0x000000, false),
        ("read_sit_loop_south", 0, 44, 38, 0x26031D, false),
        ("read_sit_loop_south", 0, 36, 40, 0x281846, false),
        ("read_sit_loop_south", 0, 39, 34, 0x4A3D66, false),
        ("read_sit_loop_south", 0, 38, 47, 0x606C76, false),
        ("read_sit_loop_south", 0, 42, 50, 0x612026, false),
        ("read_sit_loop_south", 0, 41, 40, 0x672115, true),
        ("read_sit_loop_south", 0, 41, 41, 0x686589, false),
        ("read_sit_loop_south", 0, 41, 51, 0x81454F, false),
        ("read_sit_loop_south", 0, 45, 46, 0x98A2A6, false),
        ("read_sit_loop_south", 0, 41, 31, 0x9B83B7, false),
        ("read_sit_loop_south", 0, 43, 36, 0xA59DA2, false),
        ("read_sit_loop_south", 0, 41, 42, 0xB395D6, false),
        ("read_sit_loop_south", 0, 38, 36, 0xC2B9BE, false),
        ("read_sit_loop_south", 0, 41, 45, 0xC9AF9C, false),
        ("read_sit_loop_south", 0, 40, 40, 0xD37A57, true),
        ("read_sit_loop_south", 0, 43, 42, 0xDEDAE9, false),
        ("read_sit_loop_south", 0, 43, 37, 0xECF0E9, false),
        ("read_sit_loop_south", 0, 40, 41, 0xF0B988, true),
        ("read_sit_loop_south", 0, 42, 44, 0xF6E4D7, false),
        ("read_sit_loop_south", 0, 40, 42, 0xFCD9B3, true),
        ("read_sit_loop_south", 1, 42, 40, 0x672115, true),
        ("read_sit_loop_south", 1, 39, 41, 0xD37A57, true),
        ("read_sit_loop_south", 1, 39, 42, 0xF0B988, true),
        ("read_sit_loop_south", 1, 39, 38, 0xFCD9B3, true),
        ("read_sit_loop_south", 2, 38, 40, 0x672115, true),
        ("read_sit_loop_south", 2, 39, 40, 0xD37A57, true),
        ("read_sit_loop_south", 2, 40, 41, 0xF0B988, true),
        ("read_sit_loop_south", 2, 40, 42, 0xFCD9B3, true),
        ("read_sit_loop_south", 3, 42, 40, 0x672115, true),
        ("read_sit_loop_south", 3, 39, 41, 0xD37A57, true),
        ("read_sit_loop_south", 3, 39, 42, 0xF0B988, true),
        ("read_sit_loop_south", 3, 39, 38, 0xFCD9B3, true),
        ("read_sit_start_south", 0, 35, 47, 0x000000, false),
        ("read_sit_start_south", 0, 43, 40, 0x281846, false),
        ("read_sit_start_south", 0, 44, 45, 0x374050, false),
        ("read_sit_start_south", 0, 40, 47, 0x494288, false),
        ("read_sit_start_south", 0, 40, 33, 0x4A3D66, false),
        ("read_sit_start_south", 0, 37, 50, 0x612026, false),
        ("read_sit_start_south", 0, 34, 47, 0x672115, true),
        ("read_sit_start_south", 0, 44, 42, 0x686589, false),
        ("read_sit_start_south", 0, 41, 48, 0x6960AD, false),
        ("read_sit_start_south", 0, 37, 51, 0x81454F, false),
        ("read_sit_start_south", 0, 40, 31, 0x9B83B7, false),
        ("read_sit_start_south", 0, 38, 46, 0xA56B81, false),
        ("read_sit_start_south", 0, 35, 44, 0xB395D6, false),
        ("read_sit_start_south", 0, 40, 46, 0xB3AFBD, false),
        ("read_sit_start_south", 0, 42, 36, 0xC2B9BE, false),
        ("read_sit_start_south", 0, 40, 40, 0xD37A57, true),
        ("read_sit_start_south", 0, 38, 44, 0xDEDAE9, false),
        ("read_sit_start_south", 0, 42, 37, 0xECF0E9, false),
        ("read_sit_start_south", 0, 33, 46, 0xF0B988, true),
        ("read_sit_start_south", 0, 40, 42, 0xFCD9B3, true),
        ("read_sit_start_south", 0, 41, 46, 0xFFFFFF, false),
        ("read_sit_start_south", 1, 44, 47, 0x672115, true),
        ("read_sit_start_south", 1, 44, 38, 0xD37A57, true),
        ("read_sit_start_south", 1, 44, 44, 0xF0B988, true),
        ("read_sit_start_south", 1, 45, 46, 0xFCD9B3, true),
        ("read_sit_start_south", 2, 40, 41, 0x672115, true),
        ("read_sit_start_south", 2, 41, 40, 0xD37A57, true),
        ("read_sit_start_south", 2, 40, 40, 0xF0B988, true),
        ("read_sit_start_south", 2, 34, 47, 0xFCD9B3, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        (
            "coin_flip_south",
            &[
                54, 53, 53, 50, 47, 50, 51, 51, 51, 51, 51, 51, 51, 51, 50, 53, 49,
            ],
        ),
        ("inspect_gem_end_south", &[51, 53, 53, 54]),
        ("inspect_gem_loop_south", &[41, 41, 41, 41, 41, 41, 41]),
        ("inspect_gem_start_south", &[54, 53, 53, 51]),
        ("read_sit_end_south", &[31, 29, 36]),
        ("read_sit_loop_south", &[25, 33, 25, 33]),
        ("read_sit_start_south", &[36, 29, 31]),
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
            let prefix = "spr_npc_balor_specialanimation_summer";
            let asset = format!("assets/animations/NPCs/Balor/Sprites/Summer/{prefix}_{case}.png");
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
                    "Balor Summer material mismatch: {id} {case} [{x},{y}]"
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
