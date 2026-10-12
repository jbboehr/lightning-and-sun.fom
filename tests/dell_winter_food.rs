use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/dell and the local accepted Dell world baseline"]
fn dell_winter_food_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/dell");
    let baseline = root.join("generated/slice-170-build/characters/dell");
    let set = std::env::var_os("FOM_DELL_WINTER_FOOD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/dell-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/dell-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..121];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..121], prior);
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
    // Moving faces and hands change; mouth interiors, clothing and blonde hair
    // stay original except the reviewed dark edges connected to closed East eyes.
    let landmarks = [
        ("drink_east", 0, 36, 36, 0xECC889, true),
        ("drink_east", 0, 40, 42, 0xECC889, true),
        ("drink_east", 0, 37, 29, 0xDF8D4B, false),
        ("drink_east", 0, 38, 31, 0xB65932, false),
        ("drink_east", 0, 34, 29, 0xFFDC90, false),
        ("drink_east", 0, 38, 30, 0xF5F5F5, false),
        ("drink_east", 0, 39, 41, 0x212249, false),
        ("drink_east", 0, 38, 41, 0x353777, false),
        ("drink_east", 0, 38, 42, 0x4E51A2, false),
        ("drink_east", 0, 39, 46, 0x444D6E, false),
        ("drink_east", 0, 44, 44, 0x6A8BBA, false),
        ("drink_east", 0, 43, 44, 0x82BCD1, false),
        ("drink_east", 0, 41, 47, 0x351E1A, false),
        ("drink_east", 0, 42, 47, 0x58372A, false),
        ("drink_east", 0, 44, 47, 0x7F5C3F, false),
        ("drink_east", 1, 34, 36, 0xECC889, true),
        ("drink_east", 1, 39, 40, 0xECC889, true),
        ("drink_east", 1, 35, 29, 0xDF8D4B, false),
        ("drink_east", 1, 36, 31, 0xB65932, false),
        ("drink_east", 1, 32, 29, 0xFFDC90, false),
        ("drink_east", 1, 36, 30, 0xF5F5F5, false),
        ("drink_east", 1, 37, 41, 0x212249, false),
        ("drink_east", 1, 36, 41, 0x353777, false),
        ("drink_east", 1, 38, 42, 0x4E51A2, false),
        ("drink_east", 1, 41, 44, 0x444D6E, false),
        ("drink_east", 1, 40, 44, 0x6A8BBA, false),
        ("drink_east", 1, 43, 44, 0x82BCD1, false),
        ("drink_east", 1, 41, 43, 0x653818, false),
        ("drink_east", 1, 41, 47, 0x351E1A, false),
        ("drink_east", 1, 42, 47, 0x58372A, false),
        ("drink_east", 1, 44, 47, 0x7F5C3F, false),
        ("drink_east", 2, 36, 36, 0xECC889, true),
        ("drink_east", 2, 40, 42, 0xECC889, true),
        ("drink_east", 2, 37, 29, 0xDF8D4B, false),
        ("drink_east", 2, 38, 31, 0xB65932, false),
        ("drink_east", 2, 34, 29, 0xFFDC90, false),
        ("drink_east", 2, 38, 30, 0xF5F5F5, false),
        ("drink_east", 2, 39, 41, 0x212249, false),
        ("drink_east", 2, 38, 41, 0x353777, false),
        ("drink_east", 2, 38, 42, 0x4E51A2, false),
        ("drink_east", 2, 39, 46, 0x444D6E, false),
        ("drink_east", 2, 44, 44, 0x6A8BBA, false),
        ("drink_east", 2, 43, 44, 0x82BCD1, false),
        ("drink_east", 2, 41, 47, 0x351E1A, false),
        ("drink_east", 2, 42, 47, 0x58372A, false),
        ("drink_east", 2, 44, 47, 0x7F5C3F, false),
        ("drink_north", 0, 33, 45, 0xDF8D4B, true),
        ("drink_north", 0, 45, 41, 0x762E21, true),
        ("drink_north", 0, 37, 29, 0xDF8D4B, false),
        ("drink_north", 0, 33, 35, 0xB65932, false),
        ("drink_north", 0, 38, 29, 0xFFDC90, false),
        ("drink_north", 0, 36, 31, 0xF5F5F5, false),
        ("drink_north", 0, 36, 41, 0x212249, false),
        ("drink_north", 0, 38, 41, 0x353777, false),
        ("drink_north", 0, 39, 41, 0x4E51A2, false),
        ("drink_north", 1, 33, 45, 0xDF8D4B, true),
        ("drink_north", 1, 37, 30, 0xDF8D4B, false),
        ("drink_north", 1, 33, 36, 0xB65932, false),
        ("drink_north", 1, 38, 30, 0xFFDC90, false),
        ("drink_north", 1, 36, 32, 0xF5F5F5, false),
        ("drink_north", 1, 36, 42, 0x212249, false),
        ("drink_north", 1, 44, 41, 0x353777, false),
        ("drink_north", 1, 39, 42, 0x4E51A2, false),
        ("drink_north", 2, 33, 45, 0xDF8D4B, true),
        ("drink_north", 2, 45, 41, 0x762E21, true),
        ("drink_north", 2, 37, 29, 0xDF8D4B, false),
        ("drink_north", 2, 33, 35, 0xB65932, false),
        ("drink_north", 2, 38, 29, 0xFFDC90, false),
        ("drink_north", 2, 36, 31, 0xF5F5F5, false),
        ("drink_north", 2, 36, 41, 0x212249, false),
        ("drink_north", 2, 38, 41, 0x353777, false),
        ("drink_north", 2, 39, 41, 0x4E51A2, false),
        ("drink_south", 0, 35, 36, 0xECC889, true),
        ("drink_south", 0, 34, 42, 0xECC889, true),
        ("drink_south", 0, 44, 45, 0xDF8D4B, true),
        ("drink_south", 0, 37, 29, 0xDF8D4B, false),
        ("drink_south", 0, 38, 31, 0xB65932, false),
        ("drink_south", 0, 34, 29, 0xFFDC90, false),
        ("drink_south", 0, 38, 30, 0xF5F5F5, false),
        ("drink_south", 0, 36, 41, 0x212249, false),
        ("drink_south", 0, 37, 41, 0x353777, false),
        ("drink_south", 0, 37, 42, 0x4E51A2, false),
        ("drink_south", 0, 38, 45, 0x6A8BBA, false),
        ("drink_south", 0, 39, 45, 0x82BCD1, false),
        ("drink_south", 0, 39, 41, 0x653818, false),
        ("drink_south", 0, 38, 43, 0xA8571C, false),
        ("drink_south", 0, 39, 42, 0xD47D36, false),
        ("drink_south", 0, 39, 43, 0xF49B43, false),
        ("drink_south", 0, 37, 49, 0x351E1A, false),
        ("drink_south", 0, 37, 48, 0x58372A, false),
        ("drink_south", 0, 38, 48, 0x7F5C3F, false),
        ("drink_south", 1, 35, 37, 0xECC889, true),
        ("drink_south", 1, 35, 40, 0xECC889, true),
        ("drink_south", 1, 44, 45, 0xDF8D4B, true),
        ("drink_south", 1, 37, 30, 0xDF8D4B, false),
        ("drink_south", 1, 38, 32, 0xB65932, false),
        ("drink_south", 1, 34, 30, 0xFFDC90, false),
        ("drink_south", 1, 38, 31, 0xF5F5F5, false),
        ("drink_south", 1, 38, 42, 0x212249, false),
        ("drink_south", 1, 34, 42, 0x353777, false),
        ("drink_south", 1, 34, 43, 0x4E51A2, false),
        ("drink_south", 1, 38, 45, 0x6A8BBA, false),
        ("drink_south", 1, 39, 45, 0x82BCD1, false),
        ("drink_south", 1, 39, 42, 0x653818, false),
        ("drink_south", 1, 38, 44, 0xA8571C, false),
        ("drink_south", 1, 39, 43, 0xD47D36, false),
        ("drink_south", 1, 39, 44, 0xF49B43, false),
        ("drink_south", 1, 37, 49, 0x351E1A, false),
        ("drink_south", 1, 37, 48, 0x58372A, false),
        ("drink_south", 1, 38, 48, 0x7F5C3F, false),
        ("drink_south", 2, 35, 36, 0xECC889, true),
        ("drink_south", 2, 34, 42, 0xECC889, true),
        ("drink_south", 2, 44, 45, 0xDF8D4B, true),
        ("drink_south", 2, 37, 29, 0xDF8D4B, false),
        ("drink_south", 2, 38, 31, 0xB65932, false),
        ("drink_south", 2, 34, 29, 0xFFDC90, false),
        ("drink_south", 2, 38, 30, 0xF5F5F5, false),
        ("drink_south", 2, 36, 41, 0x212249, false),
        ("drink_south", 2, 37, 41, 0x353777, false),
        ("drink_south", 2, 37, 42, 0x4E51A2, false),
        ("drink_south", 2, 38, 45, 0x6A8BBA, false),
        ("drink_south", 2, 39, 45, 0x82BCD1, false),
        ("drink_south", 2, 39, 41, 0x653818, false),
        ("drink_south", 2, 38, 43, 0xA8571C, false),
        ("drink_south", 2, 39, 42, 0xD47D36, false),
        ("drink_south", 2, 39, 43, 0xF49B43, false),
        ("drink_south", 2, 37, 49, 0x351E1A, false),
        ("drink_south", 2, 37, 48, 0x58372A, false),
        ("drink_south", 2, 38, 48, 0x7F5C3F, false),
        ("eat_east", 0, 36, 36, 0xECC889, true),
        ("eat_east", 0, 43, 42, 0xECC889, true),
        ("eat_east", 0, 37, 29, 0xDF8D4B, false),
        ("eat_east", 0, 38, 31, 0xB65932, false),
        ("eat_east", 0, 34, 29, 0xFFDC90, false),
        ("eat_east", 0, 38, 30, 0xF5F5F5, false),
        ("eat_east", 0, 38, 41, 0x212249, false),
        ("eat_east", 0, 39, 41, 0x353777, false),
        ("eat_east", 0, 39, 42, 0x4E51A2, false),
        ("eat_east", 0, 39, 46, 0x444D6E, false),
        ("eat_east", 0, 39, 45, 0x6A8BBA, false),
        ("eat_east", 0, 39, 44, 0x82BCD1, false),
        ("eat_east", 0, 41, 47, 0x351E1A, false),
        ("eat_east", 0, 42, 47, 0x58372A, false),
        ("eat_east", 0, 44, 47, 0x7F5C3F, false),
        ("eat_east", 1, 37, 36, 0xECC889, true),
        ("eat_east", 1, 45, 40, 0xDF8D4B, true),
        ("eat_east", 1, 38, 29, 0xDF8D4B, false),
        ("eat_east", 1, 39, 31, 0xB65932, false),
        ("eat_east", 1, 35, 29, 0xFFDC90, false),
        ("eat_east", 1, 39, 30, 0xF5F5F5, false),
        ("eat_east", 1, 39, 41, 0x212249, false),
        ("eat_east", 1, 44, 40, 0x353777, false),
        ("eat_east", 1, 41, 41, 0x4E51A2, false),
        ("eat_east", 1, 41, 44, 0x444D6E, false),
        ("eat_east", 1, 40, 44, 0x6A8BBA, false),
        ("eat_east", 1, 43, 44, 0x82BCD1, false),
        ("eat_east", 1, 41, 47, 0x351E1A, false),
        ("eat_east", 1, 42, 47, 0x58372A, false),
        ("eat_east", 1, 44, 47, 0x7F5C3F, false),
        ("eat_east", 2, 36, 35, 0xECC889, true),
        ("eat_east", 2, 41, 39, 0xB65932, true),
        ("eat_east", 2, 37, 28, 0xDF8D4B, false),
        ("eat_east", 2, 38, 30, 0xB65932, false),
        ("eat_east", 2, 34, 28, 0xFFDC90, false),
        ("eat_east", 2, 38, 29, 0xF5F5F5, false),
        ("eat_east", 2, 38, 41, 0x212249, false),
        ("eat_east", 2, 39, 41, 0x353777, false),
        ("eat_east", 2, 42, 40, 0x4E51A2, false),
        ("eat_east", 2, 41, 44, 0x444D6E, false),
        ("eat_east", 2, 40, 44, 0x6A8BBA, false),
        ("eat_east", 2, 39, 44, 0x82BCD1, false),
        ("eat_east", 2, 39, 43, 0x653818, false),
        ("eat_east", 2, 41, 47, 0x351E1A, false),
        ("eat_east", 2, 42, 47, 0x58372A, false),
        ("eat_east", 2, 44, 47, 0x7F5C3F, false),
        ("eat_east", 3, 36, 37, 0xECC889, true),
        ("eat_east", 3, 43, 43, 0xECC889, true),
        ("eat_east", 3, 37, 30, 0xDF8D4B, false),
        ("eat_east", 3, 38, 32, 0xB65932, false),
        ("eat_east", 3, 34, 30, 0xFFDC90, false),
        ("eat_east", 3, 38, 31, 0xF5F5F5, false),
        ("eat_east", 3, 38, 42, 0x212249, false),
        ("eat_east", 3, 39, 42, 0x353777, false),
        ("eat_east", 3, 39, 43, 0x4E51A2, false),
        ("eat_east", 3, 39, 46, 0x444D6E, false),
        ("eat_east", 3, 39, 45, 0x6A8BBA, false),
        ("eat_east", 3, 42, 46, 0x82BCD1, false),
        ("eat_east", 3, 41, 47, 0x351E1A, false),
        ("eat_east", 3, 42, 47, 0x58372A, false),
        ("eat_east", 3, 44, 47, 0x7F5C3F, false),
        ("eat_east", 4, 36, 36, 0xECC889, true),
        ("eat_east", 4, 42, 43, 0xECC889, true),
        ("eat_east", 4, 37, 29, 0xDF8D4B, false),
        ("eat_east", 4, 38, 31, 0xB65932, false),
        ("eat_east", 4, 34, 29, 0xFFDC90, false),
        ("eat_east", 4, 38, 30, 0xF5F5F5, false),
        ("eat_east", 4, 42, 41, 0x212249, false),
        ("eat_east", 4, 38, 41, 0x353777, false),
        ("eat_east", 4, 39, 41, 0x4E51A2, false),
        ("eat_east", 4, 39, 46, 0x444D6E, false),
        ("eat_east", 4, 39, 45, 0x6A8BBA, false),
        ("eat_east", 4, 44, 45, 0x82BCD1, false),
        ("eat_east", 4, 41, 41, 0x653818, false),
        ("eat_east", 4, 41, 47, 0x351E1A, false),
        ("eat_east", 4, 42, 47, 0x58372A, false),
        ("eat_east", 4, 44, 47, 0x7F5C3F, false),
        ("eat_north", 0, 33, 45, 0xDF8D4B, true),
        ("eat_north", 0, 45, 41, 0x762E21, true),
        ("eat_north", 0, 37, 29, 0xDF8D4B, false),
        ("eat_north", 0, 33, 35, 0xB65932, false),
        ("eat_north", 0, 38, 29, 0xFFDC90, false),
        ("eat_north", 0, 36, 31, 0xF5F5F5, false),
        ("eat_north", 0, 36, 41, 0x212249, false),
        ("eat_north", 0, 38, 41, 0x353777, false),
        ("eat_north", 0, 39, 41, 0x4E51A2, false),
        ("eat_north", 1, 33, 45, 0xDF8D4B, true),
        ("eat_north", 1, 37, 30, 0xDF8D4B, false),
        ("eat_north", 1, 33, 36, 0xB65932, false),
        ("eat_north", 1, 38, 30, 0xFFDC90, false),
        ("eat_north", 1, 36, 32, 0xF5F5F5, false),
        ("eat_north", 1, 36, 42, 0x212249, false),
        ("eat_north", 1, 44, 41, 0x353777, false),
        ("eat_north", 1, 39, 42, 0x4E51A2, false),
        ("eat_north", 2, 33, 45, 0xDF8D4B, true),
        ("eat_north", 2, 45, 41, 0x762E21, true),
        ("eat_north", 2, 37, 29, 0xDF8D4B, false),
        ("eat_north", 2, 33, 35, 0xB65932, false),
        ("eat_north", 2, 38, 29, 0xFFDC90, false),
        ("eat_north", 2, 36, 31, 0xF5F5F5, false),
        ("eat_north", 2, 36, 41, 0x212249, false),
        ("eat_north", 2, 38, 41, 0x353777, false),
        ("eat_north", 2, 39, 41, 0x4E51A2, false),
        ("eat_south", 0, 35, 36, 0xECC889, true),
        ("eat_south", 0, 36, 46, 0xECC889, true),
        ("eat_south", 0, 44, 45, 0xDF8D4B, true),
        ("eat_south", 0, 37, 29, 0xDF8D4B, false),
        ("eat_south", 0, 38, 31, 0xB65932, false),
        ("eat_south", 0, 34, 29, 0xFFDC90, false),
        ("eat_south", 0, 38, 30, 0xF5F5F5, false),
        ("eat_south", 0, 38, 41, 0x212249, false),
        ("eat_south", 0, 37, 41, 0x353777, false),
        ("eat_south", 0, 37, 42, 0x4E51A2, false),
        ("eat_south", 0, 38, 45, 0x6A8BBA, false),
        ("eat_south", 0, 39, 45, 0x82BCD1, false),
        ("eat_south", 0, 39, 41, 0x653818, false),
        ("eat_south", 0, 38, 43, 0xA8571C, false),
        ("eat_south", 0, 39, 42, 0xD47D36, false),
        ("eat_south", 0, 39, 43, 0xF49B43, false),
        ("eat_south", 0, 37, 49, 0x351E1A, false),
        ("eat_south", 0, 42, 48, 0x58372A, false),
        ("eat_south", 0, 38, 48, 0x7F5C3F, false),
        ("eat_south", 1, 35, 37, 0xECC889, true),
        ("eat_south", 1, 37, 47, 0xECC889, true),
        ("eat_south", 1, 44, 45, 0xDF8D4B, true),
        ("eat_south", 1, 37, 30, 0xDF8D4B, false),
        ("eat_south", 1, 38, 32, 0xB65932, false),
        ("eat_south", 1, 34, 30, 0xFFDC90, false),
        ("eat_south", 1, 38, 31, 0xF5F5F5, false),
        ("eat_south", 1, 38, 42, 0x212249, false),
        ("eat_south", 1, 37, 42, 0x353777, false),
        ("eat_south", 1, 37, 43, 0x4E51A2, false),
        ("eat_south", 1, 41, 45, 0x6A8BBA, false),
        ("eat_south", 1, 39, 45, 0x82BCD1, false),
        ("eat_south", 1, 39, 42, 0x653818, false),
        ("eat_south", 1, 41, 44, 0xA8571C, false),
        ("eat_south", 1, 39, 43, 0xD47D36, false),
        ("eat_south", 1, 39, 44, 0xF49B43, false),
        ("eat_south", 1, 42, 49, 0x351E1A, false),
        ("eat_south", 1, 42, 48, 0x58372A, false),
        ("eat_south", 1, 41, 48, 0x7F5C3F, false),
        ("eat_south", 2, 35, 35, 0xECC889, true),
        ("eat_south", 2, 36, 40, 0x762E21, true),
        ("eat_south", 2, 44, 45, 0xDF8D4B, true),
        ("eat_south", 2, 37, 28, 0xDF8D4B, false),
        ("eat_south", 2, 38, 30, 0xB65932, false),
        ("eat_south", 2, 34, 28, 0xFFDC90, false),
        ("eat_south", 2, 38, 29, 0xF5F5F5, false),
        ("eat_south", 2, 41, 41, 0x212249, false),
        ("eat_south", 2, 42, 41, 0x353777, false),
        ("eat_south", 2, 42, 42, 0x4E51A2, false),
        ("eat_south", 2, 38, 45, 0x6A8BBA, false),
        ("eat_south", 2, 39, 45, 0x82BCD1, false),
        ("eat_south", 2, 40, 41, 0x653818, false),
        ("eat_south", 2, 41, 43, 0xA8571C, false),
        ("eat_south", 2, 40, 42, 0xD47D36, false),
        ("eat_south", 2, 39, 43, 0xF49B43, false),
        ("eat_south", 2, 37, 49, 0x351E1A, false),
        ("eat_south", 2, 37, 48, 0x58372A, false),
        ("eat_south", 2, 38, 48, 0x7F5C3F, false),
        ("eat_south", 3, 35, 37, 0xECC889, true),
        ("eat_south", 3, 35, 43, 0x762E21, true),
        ("eat_south", 3, 44, 45, 0xDF8D4B, true),
        ("eat_south", 3, 37, 30, 0xDF8D4B, false),
        ("eat_south", 3, 38, 32, 0xB65932, false),
        ("eat_south", 3, 34, 30, 0xFFDC90, false),
        ("eat_south", 3, 38, 31, 0xF5F5F5, false),
        ("eat_south", 3, 38, 42, 0x212249, false),
        ("eat_south", 3, 42, 42, 0x353777, false),
        ("eat_south", 3, 42, 43, 0x4E51A2, false),
        ("eat_south", 3, 41, 45, 0x6A8BBA, false),
        ("eat_south", 3, 39, 45, 0x82BCD1, false),
        ("eat_south", 3, 39, 42, 0x653818, false),
        ("eat_south", 3, 41, 44, 0xA8571C, false),
        ("eat_south", 3, 39, 43, 0xD47D36, false),
        ("eat_south", 3, 39, 44, 0xF49B43, false),
        ("eat_south", 3, 37, 49, 0x351E1A, false),
        ("eat_south", 3, 37, 48, 0x58372A, false),
        ("eat_south", 3, 38, 48, 0x7F5C3F, false),
        ("eat_south", 4, 35, 36, 0xECC889, true),
        ("eat_south", 4, 33, 45, 0xDF8D4B, true),
        ("eat_south", 4, 44, 45, 0xDF8D4B, true),
        ("eat_south", 4, 37, 29, 0xDF8D4B, false),
        ("eat_south", 4, 38, 31, 0xB65932, false),
        ("eat_south", 4, 34, 29, 0xFFDC90, false),
        ("eat_south", 4, 38, 30, 0xF5F5F5, false),
        ("eat_south", 4, 36, 41, 0x212249, false),
        ("eat_south", 4, 37, 41, 0x353777, false),
        ("eat_south", 4, 37, 42, 0x4E51A2, false),
        ("eat_south", 4, 38, 45, 0x6A8BBA, false),
        ("eat_south", 4, 39, 45, 0x82BCD1, false),
        ("eat_south", 4, 39, 41, 0x653818, false),
        ("eat_south", 4, 38, 43, 0xA8571C, false),
        ("eat_south", 4, 39, 42, 0xD47D36, false),
        ("eat_south", 4, 39, 43, 0xF49B43, false),
        ("eat_south", 4, 37, 49, 0x351E1A, false),
        ("eat_south", 4, 37, 48, 0x58372A, false),
        ("eat_south", 4, 38, 48, 0x7F5C3F, false),
        ("eat_east", 2, 40, 35, 0x410808, false),
        ("eat_east", 2, 40, 36, 0x9E2626, false),
        ("eat_south", 2, 38, 35, 0x410808, false),
        ("eat_south", 2, 38, 36, 0x9E2626, false),
        ("drink_south", 0, 37, 36, 0xECF0E9, false),
        ("drink_south", 0, 37, 35, 0xC2B9BE, false),
        ("drink_east", 1, 40, 32, 0x762E21, true),
        ("drink_east", 1, 40, 33, 0x762E21, true),
        ("drink_east", 1, 41, 33, 0x762E21, true),
        ("drink_east", 1, 42, 33, 0x762E21, true),
        ("drink_east", 1, 42, 34, 0x762E21, true),
        ("eat_east", 1, 45, 33, 0x762E21, true),
        ("eat_east", 1, 44, 34, 0x762E21, true),
        ("eat_east", 1, 45, 34, 0x762E21, true),
        ("eat_east", 3, 44, 34, 0x762E21, true),
        ("eat_east", 3, 43, 35, 0x762E21, true),
        ("eat_east", 3, 44, 35, 0x762E21, true),
        ("eat_east", 2, 42, 31, 0x762E21, true),
        ("eat_east", 2, 42, 32, 0x762E21, true),
        ("eat_east", 2, 43, 32, 0x762E21, true),
        ("eat_east", 2, 44, 32, 0x762E21, true),
        ("eat_east", 2, 44, 33, 0x762E21, true),
        ("eat_south", 2, 36, 38, 0x762E21, true),
        ("eat_south", 2, 43, 38, 0x762E21, true),
        ("eat_east", 2, 42, 33, 0xDF8D4B, false),
        ("eat_east", 2, 43, 33, 0xDF8D4B, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("drink_east", &[32, 43, 32]),
        ("drink_north", &[7, 5, 7]),
        ("drink_south", &[41, 49, 41]),
        ("eat_east", &[30, 36, 29, 38, 30]),
        ("eat_north", &[7, 5, 7]),
        ("eat_south", &[39, 45, 39, 54, 40]),
    ];
    // Coordinates were reviewed against original grids, independently of seeds.
    let east_face = [
        (35, 40, 42),
        (36, 36, 45),
        (37, 36, 45),
        (38, 37, 44),
        (39, 38, 43),
        (40, 40, 42),
    ];
    let south_face = [
        (35, 39, 41),
        (36, 35, 45),
        (37, 35, 45),
        (38, 36, 44),
        (39, 37, 43),
        (40, 39, 41),
    ];
    let skin_at = |case: &str, x: u32, y: u32| {
        let frame = x / 80;
        let x = (x % 80) as i32;
        let y = y as i32;
        if case.ends_with("north") {
            return y >= 41;
        }
        let east = case.ends_with("east");
        let drink = case.starts_with("drink");
        if !east && !drink && frame == 2 && y == 38 && (x == 36 || x == 43) {
            return true;
        }
        let dx = if east && drink && frame == 1 {
            -2
        } else if east && !drink && frame == 1 {
            1
        } else {
            0
        };
        let dy = if (!east && frame == 1) || (!drink && frame == 3) {
            1
        } else if !drink && frame == 2 {
            -1
        } else {
            0
        };
        // Below the hair; plus the raised hands entering the head's vertical range.
        if y >= 41 + dy {
            return true;
        }
        let raised_hand = match (east, drink, frame) {
            (true, true, 1) => x >= 39 && y >= 40,
            (true, false, 1) => x >= 44 && y >= 40,
            (true, false, 2) => x >= 43 && y >= 39,
            (false, true, 1) => (34..37).contains(&x) && y >= 40,
            _ => false,
        };
        if raised_hand {
            return true;
        }
        let xx = x - dx;
        let yy = y - dy;
        let closed = if drink {
            frame == 1
        } else {
            (1..=3).contains(&frame)
        };
        if east && closed {
            let temple = if drink {
                [(40, 32), (40, 33), (41, 33), (42, 33), (42, 34)].contains(&(x, y))
            } else if frame == 2 {
                [(42, 31), (42, 32), (43, 32), (44, 32), (44, 33)].contains(&(x, y))
            } else {
                [(44, 33), (43, 34), (44, 34)].contains(&(xx, yy))
            };
            if temple {
                return true;
            }
        }
        if closed && yy == 35 {
            return if east {
                (37..45).contains(&xx)
            } else {
                (36..44).contains(&xx)
            };
        }
        if !east && !drink && frame == 2 && yy == 40 {
            return (39..43).contains(&xx);
        }
        let rows = if east { &east_face } else { &south_face };
        rows.iter()
            .any(|&(row, lo, hi)| row == yy && (lo..hi).contains(&xx))
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
            let prefix = "spr_npc_dell_winter";
            let asset = format!("assets/animations/NPCs/Dell/Sprites/Winter/{prefix}_{case}.png");
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
                    "Dell Winter material mismatch: {id} {case} [{x},{y}]"
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
