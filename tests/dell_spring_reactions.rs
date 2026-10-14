use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/dell and the local accepted Dell world baseline"]
fn dell_spring_reactions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/dell");
    let baseline = root.join("local/fixtures/slice-155-build/characters/dell");
    let set = std::env::var_os("FOM_DELL_SPRING_REACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/dell-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/dell-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..49];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..49], prior);
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
    // Faces, closed eyelids, necks, hands and ankles change. Matching blonde
    // hair, eyebrow edges, eye details, mouth interiors and clothing stay original.
    let landmarks = [
        ("action_east", 0, 37, 40, 0xECC889, true),
        ("action_east", 0, 38, 48, 0xECC889, true),
        ("action_east", 0, 38, 33, 0xDF8D4B, false),
        ("action_east", 0, 39, 35, 0xB65932, false),
        ("action_east", 0, 35, 33, 0xFFDC90, false),
        ("action_east", 0, 39, 34, 0xF5F5F5, false),
        ("action_east", 0, 40, 45, 0xF96F8C, false),
        ("action_east", 0, 39, 45, 0x616BBE, false),
        ("action_east", 0, 38, 52, 0xA1121D, false),
        ("action_east", 0, 39, 40, 0xECF0E9, false),
        ("action_east", 0, 39, 39, 0xC2B9BE, false),
        ("action_east", 1, 37, 39, 0xECC889, true),
        ("action_east", 1, 40, 46, 0xDF8D4B, true),
        ("action_east", 1, 38, 51, 0xB65932, true),
        ("action_east", 1, 38, 32, 0xDF8D4B, false),
        ("action_east", 1, 39, 34, 0xB65932, false),
        ("action_east", 1, 35, 32, 0xFFDC90, false),
        ("action_east", 1, 39, 33, 0xF5F5F5, false),
        ("action_east", 1, 40, 44, 0xF96F8C, false),
        ("action_east", 1, 39, 47, 0x616BBE, false),
        ("action_east", 1, 38, 52, 0xA1121D, false),
        ("action_east", 1, 39, 39, 0xECF0E9, false),
        ("action_east", 1, 39, 38, 0xC2B9BE, false),
        ("action_east", 2, 37, 39, 0xECC889, true),
        ("action_east", 2, 39, 46, 0xECC889, true),
        ("action_east", 2, 38, 51, 0xB65932, true),
        ("action_east", 2, 38, 32, 0xDF8D4B, false),
        ("action_east", 2, 39, 34, 0xB65932, false),
        ("action_east", 2, 35, 32, 0xFFDC90, false),
        ("action_east", 2, 39, 33, 0xF5F5F5, false),
        ("action_east", 2, 40, 44, 0xF96F8C, false),
        ("action_east", 2, 41, 45, 0x616BBE, false),
        ("action_east", 2, 38, 52, 0xA1121D, false),
        ("action_east", 2, 39, 39, 0xECF0E9, false),
        ("action_east", 2, 39, 38, 0xC2B9BE, false),
        ("action_east", 3, 37, 39, 0xECC889, true),
        ("action_east", 3, 40, 46, 0xDF8D4B, true),
        ("action_east", 3, 38, 51, 0xB65932, true),
        ("action_east", 3, 38, 32, 0xDF8D4B, false),
        ("action_east", 3, 39, 34, 0xB65932, false),
        ("action_east", 3, 35, 32, 0xFFDC90, false),
        ("action_east", 3, 39, 33, 0xF5F5F5, false),
        ("action_east", 3, 40, 44, 0xF96F8C, false),
        ("action_east", 3, 39, 47, 0x616BBE, false),
        ("action_east", 3, 38, 52, 0xA1121D, false),
        ("action_east", 3, 39, 39, 0xECF0E9, false),
        ("action_east", 3, 39, 38, 0xC2B9BE, false),
        ("action_east", 4, 37, 39, 0xECC889, true),
        ("action_east", 4, 39, 46, 0xECC889, true),
        ("action_east", 4, 38, 51, 0xB65932, true),
        ("action_east", 4, 38, 32, 0xDF8D4B, false),
        ("action_east", 4, 39, 34, 0xB65932, false),
        ("action_east", 4, 35, 32, 0xFFDC90, false),
        ("action_east", 4, 39, 33, 0xF5F5F5, false),
        ("action_east", 4, 40, 44, 0xF96F8C, false),
        ("action_east", 4, 41, 45, 0x616BBE, false),
        ("action_east", 4, 38, 52, 0xA1121D, false),
        ("action_east", 4, 39, 39, 0xECF0E9, false),
        ("action_east", 4, 39, 38, 0xC2B9BE, false),
        ("action_east", 5, 37, 40, 0xECC889, true),
        ("action_east", 5, 38, 48, 0xECC889, true),
        ("action_east", 5, 38, 33, 0xDF8D4B, false),
        ("action_east", 5, 39, 35, 0xB65932, false),
        ("action_east", 5, 35, 33, 0xFFDC90, false),
        ("action_east", 5, 39, 34, 0xF5F5F5, false),
        ("action_east", 5, 40, 45, 0xF96F8C, false),
        ("action_east", 5, 39, 45, 0x616BBE, false),
        ("action_east", 5, 38, 52, 0xA1121D, false),
        ("action_east", 5, 39, 40, 0xECF0E9, false),
        ("action_east", 5, 39, 39, 0xC2B9BE, false),
        ("action_east", 6, 36, 39, 0xECC889, true),
        ("action_east", 6, 35, 46, 0x762E21, true),
        ("action_east", 6, 38, 51, 0xB65932, true),
        ("action_east", 6, 37, 32, 0xDF8D4B, false),
        ("action_east", 6, 38, 34, 0xB65932, false),
        ("action_east", 6, 34, 32, 0xFFDC90, false),
        ("action_east", 6, 38, 33, 0xF5F5F5, false),
        ("action_east", 6, 39, 44, 0xF96F8C, false),
        ("action_east", 6, 38, 44, 0x616BBE, false),
        ("action_east", 6, 38, 52, 0xA1121D, false),
        ("action_east", 6, 38, 39, 0xECF0E9, false),
        ("action_east", 6, 38, 38, 0xC2B9BE, false),
        ("action_north", 0, 34, 46, 0x762E21, true),
        ("action_north", 0, 45, 44, 0xDF8D4B, true),
        ("action_north", 0, 37, 51, 0xDF8D4B, true),
        ("action_north", 0, 37, 32, 0xDF8D4B, false),
        ("action_north", 0, 33, 38, 0xB65932, false),
        ("action_north", 0, 38, 32, 0xFFDC90, false),
        ("action_north", 0, 36, 34, 0xF5F5F5, false),
        ("action_north", 0, 38, 44, 0xF96F8C, false),
        ("action_north", 0, 36, 44, 0x382A61, false),
        ("action_north", 0, 37, 52, 0xA1121D, false),
        ("action_north", 1, 35, 46, 0xDF8D4B, true),
        ("action_north", 1, 44, 42, 0xDF8D4B, true),
        ("action_north", 1, 37, 50, 0xDF8D4B, true),
        ("action_north", 1, 37, 31, 0xDF8D4B, false),
        ("action_north", 1, 33, 37, 0xB65932, false),
        ("action_north", 1, 38, 31, 0xFFDC90, false),
        ("action_north", 1, 36, 33, 0xF5F5F5, false),
        ("action_north", 1, 38, 43, 0xF96F8C, false),
        ("action_north", 1, 37, 43, 0x616BBE, false),
        ("action_north", 1, 37, 51, 0xA1121D, false),
        ("action_north", 2, 35, 46, 0xDF8D4B, true),
        ("action_north", 2, 44, 42, 0xB65932, true),
        ("action_north", 2, 37, 50, 0xDF8D4B, true),
        ("action_north", 2, 37, 31, 0xDF8D4B, false),
        ("action_north", 2, 33, 37, 0xB65932, false),
        ("action_north", 2, 38, 31, 0xFFDC90, false),
        ("action_north", 2, 36, 33, 0xF5F5F5, false),
        ("action_north", 2, 38, 43, 0xF96F8C, false),
        ("action_north", 2, 37, 43, 0x616BBE, false),
        ("action_north", 2, 37, 51, 0xA1121D, false),
        ("action_north", 3, 35, 46, 0xDF8D4B, true),
        ("action_north", 3, 44, 42, 0xDF8D4B, true),
        ("action_north", 3, 37, 50, 0xDF8D4B, true),
        ("action_north", 3, 37, 31, 0xDF8D4B, false),
        ("action_north", 3, 33, 37, 0xB65932, false),
        ("action_north", 3, 38, 31, 0xFFDC90, false),
        ("action_north", 3, 36, 33, 0xF5F5F5, false),
        ("action_north", 3, 38, 43, 0xF96F8C, false),
        ("action_north", 3, 37, 43, 0x616BBE, false),
        ("action_north", 3, 37, 51, 0xA1121D, false),
        ("action_north", 4, 35, 46, 0xDF8D4B, true),
        ("action_north", 4, 44, 42, 0xB65932, true),
        ("action_north", 4, 37, 50, 0xDF8D4B, true),
        ("action_north", 4, 37, 31, 0xDF8D4B, false),
        ("action_north", 4, 33, 37, 0xB65932, false),
        ("action_north", 4, 38, 31, 0xFFDC90, false),
        ("action_north", 4, 36, 33, 0xF5F5F5, false),
        ("action_north", 4, 38, 43, 0xF96F8C, false),
        ("action_north", 4, 37, 43, 0x616BBE, false),
        ("action_north", 4, 37, 51, 0xA1121D, false),
        ("action_north", 5, 34, 46, 0x762E21, true),
        ("action_north", 5, 45, 44, 0xDF8D4B, true),
        ("action_north", 5, 37, 51, 0xDF8D4B, true),
        ("action_north", 5, 37, 32, 0xDF8D4B, false),
        ("action_north", 5, 33, 38, 0xB65932, false),
        ("action_north", 5, 38, 32, 0xFFDC90, false),
        ("action_north", 5, 36, 34, 0xF5F5F5, false),
        ("action_north", 5, 38, 44, 0xF96F8C, false),
        ("action_north", 5, 36, 44, 0x382A61, false),
        ("action_north", 5, 37, 52, 0xA1121D, false),
        ("action_north", 6, 34, 46, 0x762E21, true),
        ("action_north", 6, 45, 46, 0x762E21, true),
        ("action_north", 6, 37, 51, 0xDF8D4B, true),
        ("action_north", 6, 37, 32, 0xDF8D4B, false),
        ("action_north", 6, 33, 38, 0xB65932, false),
        ("action_north", 6, 38, 32, 0xFFDC90, false),
        ("action_north", 6, 36, 34, 0xF5F5F5, false),
        ("action_north", 6, 38, 44, 0xF96F8C, false),
        ("action_north", 6, 36, 44, 0x382A61, false),
        ("action_north", 6, 37, 52, 0xA1121D, false),
        ("action_south", 0, 35, 40, 0xECC889, true),
        ("action_south", 0, 35, 48, 0xDF8D4B, true),
        ("action_south", 0, 44, 47, 0xB65932, true),
        ("action_south", 0, 36, 50, 0x762E21, true),
        ("action_south", 0, 37, 33, 0xDF8D4B, false),
        ("action_south", 0, 38, 35, 0xB65932, false),
        ("action_south", 0, 34, 33, 0xFFDC90, false),
        ("action_south", 0, 38, 34, 0xF5F5F5, false),
        ("action_south", 0, 38, 45, 0xF96F8C, false),
        ("action_south", 0, 37, 45, 0x616BBE, false),
        ("action_south", 0, 37, 52, 0xA1121D, false),
        ("action_south", 0, 37, 40, 0xECF0E9, false),
        ("action_south", 0, 37, 39, 0xC2B9BE, false),
        ("action_south", 1, 35, 39, 0xECC889, true),
        ("action_south", 1, 35, 47, 0xDF8D4B, true),
        ("action_south", 1, 44, 46, 0xB65932, true),
        ("action_south", 1, 37, 51, 0xDF8D4B, true),
        ("action_south", 1, 37, 32, 0xDF8D4B, false),
        ("action_south", 1, 38, 34, 0xB65932, false),
        ("action_south", 1, 34, 32, 0xFFDC90, false),
        ("action_south", 1, 38, 33, 0xF5F5F5, false),
        ("action_south", 1, 38, 44, 0xF96F8C, false),
        ("action_south", 1, 37, 44, 0x616BBE, false),
        ("action_south", 1, 37, 52, 0xA1121D, false),
        ("action_south", 1, 37, 39, 0xECF0E9, false),
        ("action_south", 1, 37, 38, 0xC2B9BE, false),
        ("action_south", 2, 35, 39, 0xECC889, true),
        ("action_south", 2, 36, 46, 0xB65932, true),
        ("action_south", 2, 44, 46, 0xB65932, true),
        ("action_south", 2, 37, 51, 0xDF8D4B, true),
        ("action_south", 2, 37, 32, 0xDF8D4B, false),
        ("action_south", 2, 38, 34, 0xB65932, false),
        ("action_south", 2, 34, 32, 0xFFDC90, false),
        ("action_south", 2, 38, 33, 0xF5F5F5, false),
        ("action_south", 2, 41, 44, 0xF96F8C, false),
        ("action_south", 2, 38, 44, 0x616BBE, false),
        ("action_south", 2, 37, 52, 0xA1121D, false),
        ("action_south", 2, 37, 39, 0xECF0E9, false),
        ("action_south", 2, 37, 38, 0xC2B9BE, false),
        ("action_south", 3, 35, 39, 0xECC889, true),
        ("action_south", 3, 35, 47, 0xDF8D4B, true),
        ("action_south", 3, 44, 46, 0xB65932, true),
        ("action_south", 3, 37, 51, 0xDF8D4B, true),
        ("action_south", 3, 37, 32, 0xDF8D4B, false),
        ("action_south", 3, 38, 34, 0xB65932, false),
        ("action_south", 3, 34, 32, 0xFFDC90, false),
        ("action_south", 3, 38, 33, 0xF5F5F5, false),
        ("action_south", 3, 38, 44, 0xF96F8C, false),
        ("action_south", 3, 37, 44, 0x616BBE, false),
        ("action_south", 3, 37, 52, 0xA1121D, false),
        ("action_south", 3, 37, 39, 0xECF0E9, false),
        ("action_south", 3, 37, 38, 0xC2B9BE, false),
        ("action_south", 4, 35, 39, 0xECC889, true),
        ("action_south", 4, 36, 46, 0xB65932, true),
        ("action_south", 4, 44, 46, 0xB65932, true),
        ("action_south", 4, 37, 51, 0xDF8D4B, true),
        ("action_south", 4, 37, 32, 0xDF8D4B, false),
        ("action_south", 4, 38, 34, 0xB65932, false),
        ("action_south", 4, 34, 32, 0xFFDC90, false),
        ("action_south", 4, 38, 33, 0xF5F5F5, false),
        ("action_south", 4, 41, 44, 0xF96F8C, false),
        ("action_south", 4, 38, 44, 0x616BBE, false),
        ("action_south", 4, 37, 52, 0xA1121D, false),
        ("action_south", 4, 37, 39, 0xECF0E9, false),
        ("action_south", 4, 37, 38, 0xC2B9BE, false),
        ("action_south", 5, 35, 40, 0xECC889, true),
        ("action_south", 5, 35, 48, 0xDF8D4B, true),
        ("action_south", 5, 44, 47, 0xB65932, true),
        ("action_south", 5, 36, 50, 0x762E21, true),
        ("action_south", 5, 37, 33, 0xDF8D4B, false),
        ("action_south", 5, 38, 35, 0xB65932, false),
        ("action_south", 5, 34, 33, 0xFFDC90, false),
        ("action_south", 5, 38, 34, 0xF5F5F5, false),
        ("action_south", 5, 38, 45, 0xF96F8C, false),
        ("action_south", 5, 37, 45, 0x616BBE, false),
        ("action_south", 5, 37, 52, 0xA1121D, false),
        ("action_south", 5, 37, 40, 0xECF0E9, false),
        ("action_south", 5, 37, 39, 0xC2B9BE, false),
        ("action_south", 6, 35, 39, 0xECC889, true),
        ("action_south", 6, 34, 46, 0x762E21, true),
        ("action_south", 6, 45, 46, 0x762E21, true),
        ("action_south", 6, 37, 51, 0xDF8D4B, true),
        ("action_south", 6, 37, 32, 0xDF8D4B, false),
        ("action_south", 6, 38, 34, 0xB65932, false),
        ("action_south", 6, 34, 32, 0xFFDC90, false),
        ("action_south", 6, 38, 33, 0xF5F5F5, false),
        ("action_south", 6, 38, 44, 0xF96F8C, false),
        ("action_south", 6, 36, 44, 0x382A61, false),
        ("action_south", 6, 37, 52, 0xA1121D, false),
        ("action_south", 6, 37, 39, 0xECF0E9, false),
        ("action_south", 6, 37, 38, 0xC2B9BE, false),
        ("shocked_start_south", 0, 35, 40, 0xECC889, true),
        ("shocked_start_south", 0, 34, 47, 0x762E21, true),
        ("shocked_start_south", 0, 45, 47, 0x762E21, true),
        ("shocked_start_south", 0, 37, 33, 0xDF8D4B, false),
        ("shocked_start_south", 0, 38, 35, 0xB65932, false),
        ("shocked_start_south", 0, 34, 33, 0xFFDC90, false),
        ("shocked_start_south", 0, 38, 34, 0xF5F5F5, false),
        ("shocked_start_south", 0, 38, 45, 0xF96F8C, false),
        ("shocked_start_south", 0, 36, 45, 0x382A61, false),
        ("shocked_start_south", 0, 37, 52, 0xA1121D, false),
        ("shocked_loop_south", 0, 35, 36, 0xECC889, true),
        ("shocked_loop_south", 0, 33, 38, 0xECC889, true),
        ("shocked_loop_south", 0, 44, 38, 0xECC889, true),
        ("shocked_loop_south", 0, 35, 48, 0xB65932, true),
        ("shocked_loop_south", 0, 37, 29, 0xDF8D4B, false),
        ("shocked_loop_south", 0, 38, 31, 0xB65932, false),
        ("shocked_loop_south", 0, 35, 29, 0xFFDC90, false),
        ("shocked_loop_south", 0, 38, 30, 0xF5F5F5, false),
        ("shocked_loop_south", 0, 38, 41, 0xF96F8C, false),
        ("shocked_loop_south", 0, 37, 43, 0x382A61, false),
        ("shocked_loop_south", 0, 34, 49, 0xCF3A46, false),
        ("shocked_end_south", 0, 35, 40, 0xECC889, true),
        ("shocked_end_south", 0, 34, 47, 0x762E21, true),
        ("shocked_end_south", 0, 45, 47, 0x762E21, true),
        ("shocked_end_south", 0, 37, 33, 0xDF8D4B, false),
        ("shocked_end_south", 0, 38, 35, 0xB65932, false),
        ("shocked_end_south", 0, 34, 33, 0xFFDC90, false),
        ("shocked_end_south", 0, 38, 34, 0xF5F5F5, false),
        ("shocked_end_south", 0, 38, 45, 0xF96F8C, false),
        ("shocked_end_south", 0, 36, 45, 0x382A61, false),
        ("shocked_end_south", 0, 37, 52, 0xA1121D, false),
        ("shocked_loop_south", 0, 39, 38, 0x410808, false),
        ("shocked_loop_south", 0, 39, 39, 0x9E2626, false),
        ("shocked_start_south", 0, 37, 39, 0xECC889, true),
        ("shocked_start_south", 0, 37, 41, 0x000000, false),
        ("shocked_start_south", 0, 37, 38, 0xDF8D4B, false),
        ("shocked_end_south", 0, 37, 39, 0xECC889, true),
        ("shocked_end_south", 0, 37, 41, 0x000000, false),
        ("shocked_end_south", 0, 37, 38, 0xDF8D4B, false),
        ("action_east", 1, 44, 52, 0x5E0721, false),
        ("action_north", 2, 37, 53, 0x5E0721, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[32, 39, 41, 39, 41, 32, 42]),
        ("action_north", &[11, 11, 14, 11, 14, 11, 14]),
        ("action_south", &[40, 42, 46, 42, 46, 40, 46]),
        ("shocked_start_south", &[52]),
        ("shocked_loop_south", &[54]),
        ("shocked_end_south", &[52]),
    ];
    // Reviewed original face rows; the seed flood does not define this oracle.
    let east_face = [
        (38, 40, 42),
        (39, 36, 45),
        (40, 36, 45),
        (41, 37, 44),
        (42, 38, 43),
        (43, 40, 42),
    ];
    let south_face = [
        (38, 39, 41),
        (39, 35, 45),
        (40, 35, 45),
        (41, 36, 44),
        (42, 37, 43),
        (43, 39, 41),
    ];
    let skin_at = |case: &str, x: u32, y: u32| {
        let frame = x / 80;
        let x = (x % 80) as i32;
        let y = y as i32;
        if case == "shocked_loop_south" {
            // Raised hands are below the splayed hair; the open mouth is unmapped.
            return y >= 38
                || [(35, 39, 41), (36, 35, 45), (37, 35, 45)]
                    .iter()
                    .any(|&(row, lo, hi)| y == row && (lo..hi).contains(&x));
        }
        if case == "action_north" {
            // Rear hair covers the neck and ends above the raised right hand.
            return y >= [43, 42, 42, 42, 42, 43, 43][frame as usize];
        }
        let shock = case.starts_with("shocked");
        let dy = if shock {
            1
        } else {
            [1, 0, 0, 0, 0, 1, 0][frame as usize]
        };
        if y >= 44 + dy {
            return true;
        }
        let east = case.ends_with("east");
        let xx = x - if east && frame < 6 { 1 } else { 0 };
        let yy = y - dy;
        if shock && yy == 38 {
            return (36..44).contains(&xx);
        }
        let rows = if east { &east_face } else { &south_face };
        rows.iter()
            .any(|&(row, lo, hi)| yy == row && (lo..hi).contains(&xx))
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
            let prefix = "spr_npc_dell_spring";
            let asset = format!("assets/animations/NPCs/Dell/Sprites/Spring/{prefix}_{case}.png");
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
                    "Dell Spring material mismatch: {id} {case} [{x},{y}]"
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
