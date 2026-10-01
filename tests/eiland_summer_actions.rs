use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/eiland and the local accepted Eiland world baseline"]
fn eiland_summer_actions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/eiland");
    let baseline = root
        .join("generated/characters-balor-valen-eiland-summer-expansion-trial/characters/eiland");
    let set = std::env::var_os("FOM_EILAND_SUMMER_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile_path = std::env::var_os("FOM_EILAND_SUMMER_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile_path).unwrap()).unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..131];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..131], prior);
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
    // Literal source-grid landmarks include exposed skin and protected materials.
    // These expectations come from the source art, independently of recipe seeds.
    let landmarks = [
        ("blink_east", 0, 36, 48, 0x000000, false),
        ("blink_east", 0, 40, 46, 0x533061, false),
        ("blink_east", 0, 37, 36, 0x6C2859, false),
        ("blink_east", 0, 38, 44, 0x724E80, false),
        ("blink_east", 0, 41, 50, 0x756279, false),
        ("blink_east", 0, 35, 47, 0x7D3B14, true),
        ("blink_east", 0, 41, 49, 0x927D96, false),
        ("blink_east", 0, 41, 33, 0x9C5241, true),
        ("blink_east", 0, 38, 43, 0xA54E7F, false),
        ("blink_east", 0, 43, 36, 0xA59DA2, false),
        ("blink_east", 0, 39, 45, 0xB475BA, false),
        ("blink_east", 0, 34, 45, 0xBA6A4C, true),
        ("blink_east", 0, 44, 44, 0xBBB5C7, false),
        ("blink_east", 0, 38, 52, 0xC1BDC8, false),
        ("blink_east", 0, 38, 36, 0xC2B9BE, false),
        ("blink_east", 0, 42, 43, 0xDA8B36, false),
        ("blink_east", 0, 43, 42, 0xDB5C81, false),
        ("blink_east", 0, 35, 45, 0xDE8F5D, true),
        ("blink_east", 0, 37, 32, 0xE797AC, false),
        ("blink_east", 0, 35, 46, 0xE9A980, true),
        ("blink_east", 0, 43, 37, 0xECF0E9, false),
        ("blink_east", 0, 42, 51, 0xEDE0EF, false),
        ("blink_east", 0, 42, 44, 0xF9C94D, false),
        ("blink_east", 0, 42, 32, 0xFCDAE0, false),
        ("blink_east", 0, 44, 31, 0xFFFFFF, false),
        ("blink_east", 1, 35, 47, 0x7D3B14, true),
        ("blink_east", 1, 41, 33, 0x9C5241, true),
        ("blink_east", 1, 34, 45, 0xBA6A4C, true),
        ("blink_east", 1, 35, 45, 0xDE8F5D, true),
        ("blink_east", 1, 35, 46, 0xE9A980, true),
        ("blink_east", 2, 35, 47, 0x7D3B14, true),
        ("blink_east", 2, 41, 33, 0x9C5241, true),
        ("blink_east", 2, 34, 45, 0xBA6A4C, true),
        ("blink_east", 2, 35, 45, 0xDE8F5D, true),
        ("blink_east", 2, 35, 46, 0xE9A980, true),
        ("blink_south", 0, 39, 48, 0x000000, false),
        ("blink_south", 0, 41, 46, 0x533061, false),
        ("blink_south", 0, 36, 36, 0x6C2859, false),
        ("blink_south", 0, 42, 44, 0x724E80, false),
        ("blink_south", 0, 42, 50, 0x756279, false),
        ("blink_south", 0, 46, 47, 0x7D3B14, true),
        ("blink_south", 0, 38, 50, 0x927D96, false),
        ("blink_south", 0, 40, 33, 0x9C5241, true),
        ("blink_south", 0, 42, 43, 0xA54E7F, false),
        ("blink_south", 0, 38, 45, 0xB475BA, false),
        ("blink_south", 0, 35, 45, 0xBA6A4C, true),
        ("blink_south", 0, 44, 44, 0xBBB5C7, false),
        ("blink_south", 0, 37, 52, 0xC1BDC8, false),
        ("blink_south", 0, 42, 36, 0xC2B9BE, false),
        ("blink_south", 0, 41, 43, 0xDA8B36, false),
        ("blink_south", 0, 36, 43, 0xDB5C81, false),
        ("blink_south", 0, 34, 45, 0xDE8F5D, true),
        ("blink_south", 0, 36, 32, 0xE797AC, false),
        ("blink_south", 0, 47, 46, 0xE9A980, true),
        ("blink_south", 0, 42, 37, 0xECF0E9, false),
        ("blink_south", 0, 41, 51, 0xEDE0EF, false),
        ("blink_south", 0, 41, 44, 0xF9C94D, false),
        ("blink_south", 0, 41, 32, 0xFCDAE0, false),
        ("blink_south", 0, 43, 31, 0xFFFFFF, false),
        ("blink_south", 1, 46, 47, 0x7D3B14, true),
        ("blink_south", 1, 40, 33, 0x9C5241, true),
        ("blink_south", 1, 35, 45, 0xBA6A4C, true),
        ("blink_south", 1, 34, 45, 0xDE8F5D, true),
        ("blink_south", 1, 47, 46, 0xE9A980, true),
        ("blink_south", 2, 46, 47, 0x7D3B14, true),
        ("blink_south", 2, 40, 33, 0x9C5241, true),
        ("blink_south", 2, 35, 45, 0xBA6A4C, true),
        ("blink_south", 2, 34, 45, 0xDE8F5D, true),
        ("blink_south", 2, 47, 46, 0xE9A980, true),
        ("drink_east", 0, 45, 45, 0x000000, false),
        ("drink_east", 0, 36, 44, 0x010101, false),
        ("drink_east", 0, 43, 44, 0x533061, false),
        ("drink_east", 0, 37, 36, 0x6C2859, false),
        ("drink_east", 0, 39, 41, 0x724E80, false),
        ("drink_east", 0, 38, 47, 0x756279, false),
        ("drink_east", 0, 38, 39, 0x7D3B14, true),
        ("drink_east", 0, 44, 48, 0x927D96, false),
        ("drink_east", 0, 41, 33, 0x9C5241, true),
        ("drink_east", 0, 39, 32, 0xA54E7F, false),
        ("drink_east", 0, 41, 40, 0xBA6A4C, true),
        ("drink_east", 0, 44, 45, 0xBBB5C7, false),
        ("drink_east", 0, 41, 48, 0xC1BDC8, false),
        ("drink_east", 0, 39, 35, 0xC2B9BE, false),
        ("drink_east", 0, 39, 47, 0xDA8B36, false),
        ("drink_east", 0, 37, 43, 0xDB5C81, false),
        ("drink_east", 0, 42, 43, 0xDE8F5D, true),
        ("drink_east", 0, 37, 32, 0xE797AC, false),
        ("drink_east", 0, 41, 42, 0xE9A980, true),
        ("drink_east", 0, 38, 37, 0xECF0E9, false),
        ("drink_east", 0, 44, 47, 0xEDE0EF, false),
        ("drink_east", 0, 39, 46, 0xF9C94D, false),
        ("drink_east", 0, 42, 32, 0xFCDAE0, false),
        ("drink_east", 0, 44, 31, 0xFFFFFF, false),
        ("drink_east", 1, 36, 39, 0x7D3B14, true),
        ("drink_east", 1, 39, 33, 0x9C5241, true),
        ("drink_east", 1, 39, 43, 0xBA6A4C, true),
        ("drink_east", 1, 40, 41, 0xDE8F5D, true),
        ("drink_east", 1, 39, 41, 0xE9A980, true),
        ("drink_east", 2, 38, 39, 0x7D3B14, true),
        ("drink_east", 2, 41, 33, 0x9C5241, true),
        ("drink_east", 2, 41, 40, 0xBA6A4C, true),
        ("drink_east", 2, 42, 43, 0xDE8F5D, true),
        ("drink_east", 2, 41, 42, 0xE9A980, true),
        ("drink_north", 0, 47, 45, 0x000000, false),
        ("drink_north", 0, 41, 46, 0x533061, false),
        ("drink_north", 0, 44, 38, 0x6C2859, false),
        ("drink_north", 0, 37, 45, 0x724E80, false),
        ("drink_north", 0, 34, 47, 0x7D3B14, true),
        ("drink_north", 0, 42, 47, 0x927D96, false),
        ("drink_north", 0, 39, 43, 0xA54E7F, false),
        ("drink_north", 0, 40, 45, 0xB475BA, false),
        ("drink_north", 0, 35, 45, 0xBA6A4C, true),
        ("drink_north", 0, 42, 44, 0xBBB5C7, false),
        ("drink_north", 0, 41, 42, 0xDB5C81, false),
        ("drink_north", 0, 33, 46, 0xDE8F5D, true),
        ("drink_north", 0, 44, 34, 0xE797AC, false),
        ("drink_north", 0, 45, 44, 0xE9A980, true),
        ("drink_north", 0, 43, 42, 0xEDE0EF, false),
        ("drink_north", 0, 34, 35, 0xFCDAE0, false),
        ("drink_north", 0, 37, 33, 0xFFFFFF, false),
        ("drink_north", 1, 34, 47, 0x7D3B14, true),
        ("drink_north", 1, 34, 45, 0xBA6A4C, true),
        ("drink_north", 1, 34, 46, 0xDE8F5D, true),
        ("drink_north", 2, 34, 47, 0x7D3B14, true),
        ("drink_north", 2, 35, 45, 0xBA6A4C, true),
        ("drink_north", 2, 33, 46, 0xDE8F5D, true),
        ("drink_north", 2, 45, 44, 0xE9A980, true),
        ("drink_south", 0, 36, 47, 0x000000, false),
        ("drink_south", 0, 40, 46, 0x533061, false),
        ("drink_south", 0, 36, 36, 0x6C2859, false),
        ("drink_south", 0, 42, 44, 0x724E80, false),
        ("drink_south", 0, 42, 48, 0x756279, false),
        ("drink_south", 0, 45, 47, 0x7D3B14, true),
        ("drink_south", 0, 37, 50, 0x927D96, false),
        ("drink_south", 0, 40, 33, 0x9C5241, true),
        ("drink_south", 0, 42, 43, 0xA54E7F, false),
        ("drink_south", 0, 38, 45, 0xB475BA, false),
        ("drink_south", 0, 35, 44, 0xBA6A4C, true),
        ("drink_south", 0, 44, 44, 0xBBB5C7, false),
        ("drink_south", 0, 42, 49, 0xC1BDC8, false),
        ("drink_south", 0, 42, 36, 0xC2B9BE, false),
        ("drink_south", 0, 38, 43, 0xDA8B36, false),
        ("drink_south", 0, 42, 42, 0xDB5C81, false),
        ("drink_south", 0, 45, 46, 0xDE8F5D, true),
        ("drink_south", 0, 36, 32, 0xE797AC, false),
        ("drink_south", 0, 34, 43, 0xE9A980, true),
        ("drink_south", 0, 42, 37, 0xECF0E9, false),
        ("drink_south", 0, 38, 49, 0xEDE0EF, false),
        ("drink_south", 0, 41, 44, 0xF9C94D, false),
        ("drink_south", 0, 41, 32, 0xFCDAE0, false),
        ("drink_south", 0, 43, 31, 0xFFFFFF, false),
        ("drink_south", 1, 45, 47, 0x7D3B14, true),
        ("drink_south", 1, 40, 34, 0x9C5241, true),
        ("drink_south", 1, 39, 41, 0xBA6A4C, true),
        ("drink_south", 1, 40, 42, 0xDE8F5D, true),
        ("drink_south", 1, 35, 41, 0xE9A980, true),
        ("drink_south", 2, 45, 47, 0x7D3B14, true),
        ("drink_south", 2, 40, 33, 0x9C5241, true),
        ("drink_south", 2, 35, 44, 0xBA6A4C, true),
        ("drink_south", 2, 45, 46, 0xDE8F5D, true),
        ("drink_south", 2, 34, 43, 0xE9A980, true),
        ("eat_east", 0, 45, 45, 0x000000, false),
        ("eat_east", 0, 41, 45, 0x533061, false),
        ("eat_east", 0, 37, 36, 0x6C2859, false),
        ("eat_east", 0, 38, 45, 0x724E80, false),
        ("eat_east", 0, 38, 47, 0x756279, false),
        ("eat_east", 0, 38, 39, 0x7D3B14, true),
        ("eat_east", 0, 44, 48, 0x927D96, false),
        ("eat_east", 0, 41, 33, 0x9C5241, true),
        ("eat_east", 0, 38, 43, 0xA54E7F, false),
        ("eat_east", 0, 41, 40, 0xBA6A4C, true),
        ("eat_east", 0, 44, 45, 0xBBB5C7, false),
        ("eat_east", 0, 41, 48, 0xC1BDC8, false),
        ("eat_east", 0, 39, 35, 0xC2B9BE, false),
        ("eat_east", 0, 39, 47, 0xDA8B36, false),
        ("eat_east", 0, 39, 42, 0xDB5C81, false),
        ("eat_east", 0, 41, 43, 0xDE8F5D, true),
        ("eat_east", 0, 37, 32, 0xE797AC, false),
        ("eat_east", 0, 42, 43, 0xE9A980, true),
        ("eat_east", 0, 38, 37, 0xECF0E9, false),
        ("eat_east", 0, 44, 47, 0xEDE0EF, false),
        ("eat_east", 0, 39, 44, 0xF9C94D, false),
        ("eat_east", 0, 42, 32, 0xFCDAE0, false),
        ("eat_east", 0, 44, 31, 0xFFFFFF, false),
        ("eat_east", 1, 46, 42, 0x7D3B14, true),
        ("eat_east", 1, 42, 33, 0x9C5241, true),
        ("eat_east", 1, 43, 42, 0xBA6A4C, true),
        ("eat_east", 1, 44, 41, 0xDE8F5D, true),
        ("eat_east", 1, 45, 41, 0xE9A980, true),
        ("eat_east", 2, 39, 39, 0x7D3B14, true),
        ("eat_east", 2, 41, 32, 0x9C5241, true),
        ("eat_east", 2, 42, 42, 0xBA6A4C, true),
        ("eat_east", 2, 42, 41, 0xDE8F5D, true),
        ("eat_east", 2, 43, 40, 0xE9A980, true),
        ("eat_east", 3, 38, 40, 0x7D3B14, true),
        ("eat_east", 3, 41, 34, 0x9C5241, true),
        ("eat_east", 3, 40, 41, 0xBA6A4C, true),
        ("eat_east", 3, 42, 40, 0xDE8F5D, true),
        ("eat_east", 3, 42, 44, 0xE9A980, true),
        ("eat_east", 4, 38, 39, 0x7D3B14, true),
        ("eat_east", 4, 41, 33, 0x9C5241, true),
        ("eat_east", 4, 41, 40, 0xBA6A4C, true),
        ("eat_east", 4, 41, 41, 0xDE8F5D, true),
        ("eat_east", 4, 41, 45, 0xE9A980, true),
        ("eat_north", 0, 47, 45, 0x000000, false),
        ("eat_north", 0, 41, 46, 0x533061, false),
        ("eat_north", 0, 44, 38, 0x6C2859, false),
        ("eat_north", 0, 37, 45, 0x724E80, false),
        ("eat_north", 0, 34, 47, 0x7D3B14, true),
        ("eat_north", 0, 42, 47, 0x927D96, false),
        ("eat_north", 0, 39, 43, 0xA54E7F, false),
        ("eat_north", 0, 40, 45, 0xB475BA, false),
        ("eat_north", 0, 35, 45, 0xBA6A4C, true),
        ("eat_north", 0, 42, 44, 0xBBB5C7, false),
        ("eat_north", 0, 41, 42, 0xDB5C81, false),
        ("eat_north", 0, 33, 46, 0xDE8F5D, true),
        ("eat_north", 0, 44, 34, 0xE797AC, false),
        ("eat_north", 0, 45, 44, 0xE9A980, true),
        ("eat_north", 0, 43, 42, 0xEDE0EF, false),
        ("eat_north", 0, 34, 35, 0xFCDAE0, false),
        ("eat_north", 0, 37, 33, 0xFFFFFF, false),
        ("eat_north", 1, 34, 47, 0x7D3B14, true),
        ("eat_north", 1, 34, 45, 0xBA6A4C, true),
        ("eat_north", 1, 34, 46, 0xDE8F5D, true),
        ("eat_north", 2, 34, 47, 0x7D3B14, true),
        ("eat_north", 2, 35, 45, 0xBA6A4C, true),
        ("eat_north", 2, 33, 46, 0xDE8F5D, true),
        ("eat_north", 2, 45, 44, 0xE9A980, true),
        ("eat_south", 0, 46, 47, 0x000000, false),
        ("eat_south", 0, 41, 46, 0x533061, false),
        ("eat_south", 0, 34, 37, 0x6C2859, false),
        ("eat_south", 0, 37, 44, 0x724E80, false),
        ("eat_south", 0, 42, 48, 0x756279, false),
        ("eat_south", 0, 45, 47, 0x7D3B14, true),
        ("eat_south", 0, 37, 50, 0x927D96, false),
        ("eat_south", 0, 40, 33, 0x9C5241, true),
        ("eat_south", 0, 42, 43, 0xA54E7F, false),
        ("eat_south", 0, 38, 45, 0xB475BA, false),
        ("eat_south", 0, 45, 45, 0xBA6A4C, true),
        ("eat_south", 0, 44, 44, 0xBBB5C7, false),
        ("eat_south", 0, 42, 49, 0xC1BDC8, false),
        ("eat_south", 0, 38, 35, 0xC2B9BE, false),
        ("eat_south", 0, 38, 43, 0xDA8B36, false),
        ("eat_south", 0, 36, 43, 0xDB5C81, false),
        ("eat_south", 0, 37, 46, 0xDE8F5D, true),
        ("eat_south", 0, 36, 32, 0xE797AC, false),
        ("eat_south", 0, 37, 47, 0xE9A980, true),
        ("eat_south", 0, 37, 37, 0xECF0E9, false),
        ("eat_south", 0, 35, 45, 0xEDE0EF, false),
        ("eat_south", 0, 38, 44, 0xF9C94D, false),
        ("eat_south", 0, 41, 32, 0xFCDAE0, false),
        ("eat_south", 0, 43, 31, 0xFFFFFF, false),
        ("eat_south", 1, 42, 40, 0x7D3B14, true),
        ("eat_south", 1, 40, 34, 0x9C5241, true),
        ("eat_south", 1, 39, 41, 0xBA6A4C, true),
        ("eat_south", 1, 36, 47, 0xDE8F5D, true),
        ("eat_south", 1, 38, 48, 0xE9A980, true),
        ("eat_south", 2, 45, 47, 0x7D3B14, true),
        ("eat_south", 2, 40, 32, 0x9C5241, true),
        ("eat_south", 2, 38, 42, 0xBA6A4C, true),
        ("eat_south", 2, 37, 42, 0xDE8F5D, true),
        ("eat_south", 2, 36, 41, 0xE9A980, true),
        ("eat_south", 3, 35, 43, 0x7D3B14, true),
        ("eat_south", 3, 40, 34, 0x9C5241, true),
        ("eat_south", 3, 40, 41, 0xBA6A4C, true),
        ("eat_south", 3, 37, 44, 0xDE8F5D, true),
        ("eat_south", 3, 35, 44, 0xE9A980, true),
        ("eat_south", 4, 45, 47, 0x7D3B14, true),
        ("eat_south", 4, 40, 33, 0x9C5241, true),
        ("eat_south", 4, 35, 45, 0xBA6A4C, true),
        ("eat_south", 4, 35, 46, 0xDE8F5D, true),
        ("eat_south", 4, 35, 37, 0xE9A980, true),
        ("sit_east", 0, 37, 47, 0x000000, false),
        ("sit_east", 0, 40, 45, 0x533061, false),
        ("sit_east", 0, 37, 36, 0x6C2859, false),
        ("sit_east", 0, 41, 43, 0x724E80, false),
        ("sit_east", 0, 38, 47, 0x756279, false),
        ("sit_east", 0, 35, 47, 0x7D3B14, true),
        ("sit_east", 0, 44, 48, 0x927D96, false),
        ("sit_east", 0, 41, 33, 0x9C5241, true),
        ("sit_east", 0, 38, 43, 0xA54E7F, false),
        ("sit_east", 0, 35, 45, 0xBA6A4C, true),
        ("sit_east", 0, 44, 45, 0xBBB5C7, false),
        ("sit_east", 0, 41, 49, 0xC1BDC8, false),
        ("sit_east", 0, 39, 35, 0xC2B9BE, false),
        ("sit_east", 0, 39, 43, 0xDA8B36, false),
        ("sit_east", 0, 43, 42, 0xDB5C81, false),
        ("sit_east", 0, 41, 41, 0xDE8F5D, true),
        ("sit_east", 0, 37, 32, 0xE797AC, false),
        ("sit_east", 0, 35, 46, 0xE9A980, true),
        ("sit_east", 0, 38, 37, 0xECF0E9, false),
        ("sit_east", 0, 44, 47, 0xEDE0EF, false),
        ("sit_east", 0, 40, 44, 0xF9C94D, false),
        ("sit_east", 0, 42, 32, 0xFCDAE0, false),
        ("sit_east", 0, 44, 31, 0xFFFFFF, false),
        ("sit_north", 0, 43, 46, 0x000000, false),
        ("sit_north", 0, 41, 46, 0x533061, false),
        ("sit_north", 0, 44, 38, 0x6C2859, false),
        ("sit_north", 0, 37, 45, 0x724E80, false),
        ("sit_north", 0, 45, 47, 0x7D3B14, true),
        ("sit_north", 0, 42, 47, 0x927D96, false),
        ("sit_north", 0, 39, 43, 0xA54E7F, false),
        ("sit_north", 0, 40, 45, 0xB475BA, false),
        ("sit_north", 0, 44, 45, 0xBA6A4C, true),
        ("sit_north", 0, 44, 44, 0xBBB5C7, false),
        ("sit_north", 0, 41, 42, 0xDB5C81, false),
        ("sit_north", 0, 45, 46, 0xDE8F5D, true),
        ("sit_north", 0, 44, 34, 0xE797AC, false),
        ("sit_north", 0, 43, 42, 0xEDE0EF, false),
        ("sit_north", 0, 34, 35, 0xFCDAE0, false),
        ("sit_north", 0, 37, 33, 0xFFFFFF, false),
        ("sit_south", 0, 46, 47, 0x000000, false),
        ("sit_south", 0, 40, 46, 0x533061, false),
        ("sit_south", 0, 34, 37, 0x6C2859, false),
        ("sit_south", 0, 42, 44, 0x724E80, false),
        ("sit_south", 0, 42, 48, 0x756279, false),
        ("sit_south", 0, 45, 47, 0x7D3B14, true),
        ("sit_south", 0, 37, 50, 0x927D96, false),
        ("sit_south", 0, 40, 33, 0x9C5241, true),
        ("sit_south", 0, 42, 43, 0xA54E7F, false),
        ("sit_south", 0, 38, 45, 0xB475BA, false),
        ("sit_south", 0, 35, 45, 0xBA6A4C, true),
        ("sit_south", 0, 35, 44, 0xBBB5C7, false),
        ("sit_south", 0, 42, 49, 0xC1BDC8, false),
        ("sit_south", 0, 38, 35, 0xC2B9BE, false),
        ("sit_south", 0, 38, 43, 0xDA8B36, false),
        ("sit_south", 0, 36, 43, 0xDB5C81, false),
        ("sit_south", 0, 35, 46, 0xDE8F5D, true),
        ("sit_south", 0, 36, 32, 0xE797AC, false),
        ("sit_south", 0, 35, 37, 0xE9A980, true),
        ("sit_south", 0, 37, 37, 0xECF0E9, false),
        ("sit_south", 0, 38, 49, 0xEDE0EF, false),
        ("sit_south", 0, 41, 44, 0xF9C94D, false),
        ("sit_south", 0, 41, 32, 0xFCDAE0, false),
        ("sit_south", 0, 43, 31, 0xFFFFFF, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[39, 46, 39]),
        ("blink_south", &[47, 54, 47]),
        ("drink_east", &[33, 38, 33]),
        ("drink_north", &[12, 7, 12]),
        ("drink_south", &[40, 44, 40]),
        ("eat_east", &[32, 37, 27, 36, 33]),
        ("eat_north", &[12, 7, 12]),
        ("eat_south", &[39, 42, 33, 48, 40]),
        ("sit_east", &[34]),
        ("sit_north", &[12]),
        ("sit_south", &[40]),
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
            let prefix = "spr_npc_eiland_summer";
            let asset = format!("assets/animations/NPCs/Eiland/Sprites/Summer/{prefix}_{case}.png");
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
                    "Eiland Summer material mismatch: {id} {case} [{x},{y}]"
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
