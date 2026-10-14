use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/elsie and the local accepted Elsie world baseline"]
fn elsie_autumn_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/elsie");
    let baseline = root.join("local/fixtures/slice-188-build/characters/elsie");
    let set = std::env::var_os("FOM_ELSIE_AUTUMN_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/elsie-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/elsie-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..103];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..103], prior);
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
    // The four world shades occur only in skin in these six pinned sources.
    // Forehead, face, neckline and hands follow the palette.
    // Check every pixel independently of component seeds; hair, eye details,
    // gold earrings, blue collar, burgundy gown, pink trim and shoes stay original.
    // In particular, preserve clothing shade #4C0F1E despite its portrait role.
    let landmarks = [
        ("idle_north", 0, 39, 42, 0xFBDCB9, true),
        ("idle_north", 0, 33, 45, 0xE08C6A, true),
        ("idle_north", 0, 46, 45, 0xE08C6A, true),
        ("idle_north", 0, 36, 28, 0x5B566E, false),
        ("idle_north", 0, 38, 27, 0x91879B, false),
        ("idle_north", 0, 39, 27, 0xD7C8CE, false),
        ("idle_north", 0, 37, 50, 0x4C0F1E, false),
        ("idle_north", 0, 36, 43, 0x6C1C2F, false),
        ("idle_north", 0, 36, 42, 0x8F4457, false),
        ("idle_north", 0, 38, 48, 0xA43259, false),
        ("idle_north", 0, 37, 51, 0xCE6470, false),
        ("idle_north", 0, 37, 43, 0xDD7192, false),
        ("idle_north", 0, 38, 41, 0x3C347B, false),
        ("idle_north", 0, 39, 41, 0x6062B6, false),
        ("idle_south", 0, 40, 33, 0xFBDCB9, true),
        ("idle_south", 0, 40, 32, 0xF3C193, true),
        ("idle_south", 0, 40, 31, 0xE08C6A, true),
        ("idle_south", 0, 35, 35, 0xA45838, true),
        ("idle_south", 0, 38, 42, 0xF3C193, true),
        ("idle_south", 0, 33, 45, 0xE08C6A, true),
        ("idle_south", 0, 46, 45, 0xE08C6A, true),
        ("idle_south", 0, 35, 34, 0x000000, false),
        ("idle_south", 0, 37, 35, 0xC2B9BE, false),
        ("idle_south", 0, 37, 36, 0xECF0E9, false),
        ("idle_south", 0, 36, 28, 0x5B566E, false),
        ("idle_south", 0, 37, 27, 0x91879B, false),
        ("idle_south", 0, 38, 27, 0xD7C8CE, false),
        ("idle_south", 0, 39, 45, 0x4C0F1E, false),
        ("idle_south", 0, 37, 41, 0x6C1C2F, false),
        ("idle_south", 0, 36, 42, 0x8F4457, false),
        ("idle_south", 0, 37, 43, 0xA43259, false),
        ("idle_south", 0, 37, 51, 0xCE6470, false),
        ("idle_south", 0, 35, 44, 0xDD7192, false),
        ("idle_south", 0, 38, 41, 0x3C347B, false),
        ("idle_south", 0, 39, 41, 0x6062B6, false),
        ("idle_south", 0, 37, 53, 0xFBC9BB, false),
        ("idle_south", 0, 35, 39, 0xF4AB50, false),
        ("idle_east", 0, 41, 33, 0xFBDCB9, true),
        ("idle_east", 0, 41, 32, 0xF3C193, true),
        ("idle_east", 0, 41, 31, 0xE08C6A, true),
        ("idle_east", 0, 36, 35, 0xA45838, true),
        ("idle_east", 0, 39, 42, 0xE08C6A, true),
        ("idle_east", 0, 34, 45, 0xE08C6A, true),
        ("idle_east", 0, 45, 46, 0xF3C193, true),
        ("idle_east", 0, 36, 34, 0x000000, false),
        ("idle_east", 0, 38, 35, 0xC2B9BE, false),
        ("idle_east", 0, 38, 36, 0xECF0E9, false),
        ("idle_east", 0, 37, 27, 0x5B566E, false),
        ("idle_east", 0, 38, 27, 0x91879B, false),
        ("idle_east", 0, 39, 27, 0xD7C8CE, false),
        ("idle_east", 0, 41, 45, 0x4C0F1E, false),
        ("idle_east", 0, 39, 41, 0x6C1C2F, false),
        ("idle_east", 0, 38, 41, 0x8F4457, false),
        ("idle_east", 0, 39, 43, 0xA43259, false),
        ("idle_east", 0, 38, 51, 0xCE6470, false),
        ("idle_east", 0, 37, 42, 0xDD7192, false),
        ("idle_east", 0, 42, 41, 0x3C347B, false),
        ("idle_east", 0, 40, 41, 0x6062B6, false),
        ("idle_east", 0, 39, 53, 0xFBC9BB, false),
        ("idle_east", 0, 36, 39, 0xF4AB50, false),
        ("walk_north", 0, 39, 42, 0xFBDCB9, true),
        ("walk_north", 0, 33, 45, 0xE08C6A, true),
        ("walk_north", 0, 46, 45, 0xE08C6A, true),
        ("walk_north", 0, 36, 28, 0x5B566E, false),
        ("walk_north", 0, 38, 27, 0x91879B, false),
        ("walk_north", 0, 39, 27, 0xD7C8CE, false),
        ("walk_north", 0, 37, 50, 0x4C0F1E, false),
        ("walk_north", 0, 36, 43, 0x6C1C2F, false),
        ("walk_north", 0, 36, 42, 0x8F4457, false),
        ("walk_north", 0, 38, 48, 0xA43259, false),
        ("walk_north", 0, 37, 51, 0xCE6470, false),
        ("walk_north", 0, 37, 43, 0xDD7192, false),
        ("walk_north", 0, 38, 41, 0x3C347B, false),
        ("walk_north", 0, 39, 41, 0x6062B6, false),
        ("walk_north", 1, 39, 41, 0xE08C6A, true),
        ("walk_north", 1, 33, 46, 0xE08C6A, true),
        ("walk_north", 1, 46, 46, 0xE08C6A, true),
        ("walk_north", 1, 36, 29, 0x5B566E, false),
        ("walk_north", 1, 38, 28, 0x91879B, false),
        ("walk_north", 1, 39, 28, 0xD7C8CE, false),
        ("walk_north", 1, 38, 51, 0x4C0F1E, false),
        ("walk_north", 1, 36, 44, 0x6C1C2F, false),
        ("walk_north", 1, 36, 43, 0x8F4457, false),
        ("walk_north", 1, 41, 49, 0xA43259, false),
        ("walk_north", 1, 38, 52, 0xCE6470, false),
        ("walk_north", 1, 37, 44, 0xDD7192, false),
        ("walk_north", 1, 38, 42, 0x3C347B, false),
        ("walk_north", 1, 39, 42, 0x6062B6, false),
        ("walk_north", 2, 39, 42, 0xFBDCB9, true),
        ("walk_north", 2, 33, 45, 0xE08C6A, true),
        ("walk_north", 2, 46, 45, 0xE08C6A, true),
        ("walk_north", 2, 36, 28, 0x5B566E, false),
        ("walk_north", 2, 38, 27, 0x91879B, false),
        ("walk_north", 2, 39, 27, 0xD7C8CE, false),
        ("walk_north", 2, 37, 50, 0x4C0F1E, false),
        ("walk_north", 2, 36, 43, 0x6C1C2F, false),
        ("walk_north", 2, 36, 42, 0x8F4457, false),
        ("walk_north", 2, 38, 48, 0xA43259, false),
        ("walk_north", 2, 37, 51, 0xCE6470, false),
        ("walk_north", 2, 37, 43, 0xDD7192, false),
        ("walk_north", 2, 38, 41, 0x3C347B, false),
        ("walk_north", 2, 39, 41, 0x6062B6, false),
        ("walk_north", 3, 39, 41, 0xE08C6A, true),
        ("walk_north", 3, 33, 46, 0xE08C6A, true),
        ("walk_north", 3, 45, 46, 0xE08C6A, true),
        ("walk_north", 3, 36, 29, 0x5B566E, false),
        ("walk_north", 3, 38, 28, 0x91879B, false),
        ("walk_north", 3, 39, 28, 0xD7C8CE, false),
        ("walk_north", 3, 37, 51, 0x4C0F1E, false),
        ("walk_north", 3, 36, 44, 0x6C1C2F, false),
        ("walk_north", 3, 36, 43, 0x8F4457, false),
        ("walk_north", 3, 38, 49, 0xA43259, false),
        ("walk_north", 3, 37, 52, 0xCE6470, false),
        ("walk_north", 3, 37, 44, 0xDD7192, false),
        ("walk_north", 3, 38, 42, 0x3C347B, false),
        ("walk_north", 3, 39, 42, 0x6062B6, false),
        ("walk_south", 0, 40, 33, 0xFBDCB9, true),
        ("walk_south", 0, 40, 32, 0xF3C193, true),
        ("walk_south", 0, 40, 31, 0xE08C6A, true),
        ("walk_south", 0, 35, 35, 0xA45838, true),
        ("walk_south", 0, 38, 42, 0xF3C193, true),
        ("walk_south", 0, 33, 45, 0xE08C6A, true),
        ("walk_south", 0, 46, 45, 0xE08C6A, true),
        ("walk_south", 0, 35, 34, 0x000000, false),
        ("walk_south", 0, 37, 35, 0xC2B9BE, false),
        ("walk_south", 0, 37, 36, 0xECF0E9, false),
        ("walk_south", 0, 36, 28, 0x5B566E, false),
        ("walk_south", 0, 37, 27, 0x91879B, false),
        ("walk_south", 0, 38, 27, 0xD7C8CE, false),
        ("walk_south", 0, 39, 45, 0x4C0F1E, false),
        ("walk_south", 0, 37, 41, 0x6C1C2F, false),
        ("walk_south", 0, 36, 42, 0x8F4457, false),
        ("walk_south", 0, 37, 43, 0xA43259, false),
        ("walk_south", 0, 37, 51, 0xCE6470, false),
        ("walk_south", 0, 35, 44, 0xDD7192, false),
        ("walk_south", 0, 38, 41, 0x3C347B, false),
        ("walk_south", 0, 39, 41, 0x6062B6, false),
        ("walk_south", 0, 37, 53, 0xFBC9BB, false),
        ("walk_south", 0, 35, 39, 0xF4AB50, false),
        ("walk_south", 1, 40, 34, 0xFBDCB9, true),
        ("walk_south", 1, 40, 33, 0xF3C193, true),
        ("walk_south", 1, 40, 32, 0xE08C6A, true),
        ("walk_south", 1, 35, 36, 0xA45838, true),
        ("walk_south", 1, 39, 41, 0xE08C6A, true),
        ("walk_south", 1, 33, 46, 0xE08C6A, true),
        ("walk_south", 1, 46, 46, 0xE08C6A, true),
        ("walk_south", 1, 37, 34, 0x000000, false),
        ("walk_south", 1, 37, 36, 0xC2B9BE, false),
        ("walk_south", 1, 37, 37, 0xECF0E9, false),
        ("walk_south", 1, 36, 29, 0x5B566E, false),
        ("walk_south", 1, 37, 28, 0x91879B, false),
        ("walk_south", 1, 38, 28, 0xD7C8CE, false),
        ("walk_south", 1, 39, 46, 0x4C0F1E, false),
        ("walk_south", 1, 37, 42, 0x6C1C2F, false),
        ("walk_south", 1, 36, 43, 0x8F4457, false),
        ("walk_south", 1, 37, 44, 0xA43259, false),
        ("walk_south", 1, 42, 52, 0xCE6470, false),
        ("walk_south", 1, 35, 45, 0xDD7192, false),
        ("walk_south", 1, 38, 42, 0x3C347B, false),
        ("walk_south", 1, 39, 42, 0x6062B6, false),
        ("walk_south", 1, 41, 54, 0xFBC9BB, false),
        ("walk_south", 1, 35, 40, 0xF4AB50, false),
        ("walk_south", 2, 40, 33, 0xFBDCB9, true),
        ("walk_south", 2, 40, 32, 0xF3C193, true),
        ("walk_south", 2, 40, 31, 0xE08C6A, true),
        ("walk_south", 2, 35, 35, 0xA45838, true),
        ("walk_south", 2, 38, 42, 0xF3C193, true),
        ("walk_south", 2, 33, 45, 0xE08C6A, true),
        ("walk_south", 2, 46, 45, 0xE08C6A, true),
        ("walk_south", 2, 35, 34, 0x000000, false),
        ("walk_south", 2, 37, 35, 0xC2B9BE, false),
        ("walk_south", 2, 37, 36, 0xECF0E9, false),
        ("walk_south", 2, 36, 28, 0x5B566E, false),
        ("walk_south", 2, 37, 27, 0x91879B, false),
        ("walk_south", 2, 38, 27, 0xD7C8CE, false),
        ("walk_south", 2, 39, 45, 0x4C0F1E, false),
        ("walk_south", 2, 37, 41, 0x6C1C2F, false),
        ("walk_south", 2, 36, 42, 0x8F4457, false),
        ("walk_south", 2, 37, 43, 0xA43259, false),
        ("walk_south", 2, 37, 51, 0xCE6470, false),
        ("walk_south", 2, 35, 44, 0xDD7192, false),
        ("walk_south", 2, 38, 41, 0x3C347B, false),
        ("walk_south", 2, 39, 41, 0x6062B6, false),
        ("walk_south", 2, 37, 53, 0xFBC9BB, false),
        ("walk_south", 2, 35, 39, 0xF4AB50, false),
        ("walk_south", 3, 40, 34, 0xFBDCB9, true),
        ("walk_south", 3, 40, 33, 0xF3C193, true),
        ("walk_south", 3, 40, 32, 0xE08C6A, true),
        ("walk_south", 3, 35, 36, 0xA45838, true),
        ("walk_south", 3, 39, 41, 0xE08C6A, true),
        ("walk_south", 3, 33, 46, 0xE08C6A, true),
        ("walk_south", 3, 45, 46, 0xE08C6A, true),
        ("walk_south", 3, 37, 34, 0x000000, false),
        ("walk_south", 3, 37, 36, 0xC2B9BE, false),
        ("walk_south", 3, 37, 37, 0xECF0E9, false),
        ("walk_south", 3, 36, 29, 0x5B566E, false),
        ("walk_south", 3, 37, 28, 0x91879B, false),
        ("walk_south", 3, 38, 28, 0xD7C8CE, false),
        ("walk_south", 3, 39, 46, 0x4C0F1E, false),
        ("walk_south", 3, 34, 45, 0x6C1C2F, false),
        ("walk_south", 3, 36, 43, 0x8F4457, false),
        ("walk_south", 3, 37, 44, 0xA43259, false),
        ("walk_south", 3, 37, 52, 0xCE6470, false),
        ("walk_south", 3, 35, 45, 0xDD7192, false),
        ("walk_south", 3, 38, 42, 0x3C347B, false),
        ("walk_south", 3, 39, 42, 0x6062B6, false),
        ("walk_south", 3, 37, 54, 0xFBC9BB, false),
        ("walk_south", 3, 35, 40, 0xF4AB50, false),
        ("walk_east", 0, 41, 33, 0xFBDCB9, true),
        ("walk_east", 0, 41, 32, 0xF3C193, true),
        ("walk_east", 0, 41, 31, 0xE08C6A, true),
        ("walk_east", 0, 36, 35, 0xA45838, true),
        ("walk_east", 0, 39, 42, 0xE08C6A, true),
        ("walk_east", 0, 34, 45, 0xE08C6A, true),
        ("walk_east", 0, 45, 46, 0xF3C193, true),
        ("walk_east", 0, 36, 34, 0x000000, false),
        ("walk_east", 0, 38, 35, 0xC2B9BE, false),
        ("walk_east", 0, 38, 36, 0xECF0E9, false),
        ("walk_east", 0, 37, 27, 0x5B566E, false),
        ("walk_east", 0, 38, 27, 0x91879B, false),
        ("walk_east", 0, 39, 27, 0xD7C8CE, false),
        ("walk_east", 0, 41, 45, 0x4C0F1E, false),
        ("walk_east", 0, 39, 41, 0x6C1C2F, false),
        ("walk_east", 0, 38, 41, 0x8F4457, false),
        ("walk_east", 0, 39, 43, 0xA43259, false),
        ("walk_east", 0, 38, 51, 0xCE6470, false),
        ("walk_east", 0, 37, 42, 0xDD7192, false),
        ("walk_east", 0, 42, 41, 0x3C347B, false),
        ("walk_east", 0, 40, 41, 0x6062B6, false),
        ("walk_east", 0, 39, 53, 0xFBC9BB, false),
        ("walk_east", 0, 36, 39, 0xF4AB50, false),
        ("walk_east", 1, 41, 34, 0xFBDCB9, true),
        ("walk_east", 1, 41, 33, 0xF3C193, true),
        ("walk_east", 1, 41, 32, 0xE08C6A, true),
        ("walk_east", 1, 36, 36, 0xA45838, true),
        ("walk_east", 1, 40, 41, 0xE08C6A, true),
        ("walk_east", 1, 33, 45, 0xE08C6A, true),
        ("walk_east", 1, 46, 45, 0xE08C6A, true),
        ("walk_east", 1, 38, 34, 0x000000, false),
        ("walk_east", 1, 38, 36, 0xC2B9BE, false),
        ("walk_east", 1, 38, 37, 0xECF0E9, false),
        ("walk_east", 1, 37, 28, 0x5B566E, false),
        ("walk_east", 1, 38, 28, 0x91879B, false),
        ("walk_east", 1, 39, 28, 0xD7C8CE, false),
        ("walk_east", 1, 41, 46, 0x4C0F1E, false),
        ("walk_east", 1, 37, 42, 0x6C1C2F, false),
        ("walk_east", 1, 38, 42, 0x8F4457, false),
        ("walk_east", 1, 39, 44, 0xA43259, false),
        ("walk_east", 1, 41, 51, 0xCE6470, false),
        ("walk_east", 1, 37, 43, 0xDD7192, false),
        ("walk_east", 1, 42, 42, 0x3C347B, false),
        ("walk_east", 1, 40, 42, 0x6062B6, false),
        ("walk_east", 1, 43, 52, 0xFBC9BB, false),
        ("walk_east", 1, 36, 40, 0xF4AB50, false),
        ("walk_east", 2, 41, 33, 0xFBDCB9, true),
        ("walk_east", 2, 41, 32, 0xF3C193, true),
        ("walk_east", 2, 41, 31, 0xE08C6A, true),
        ("walk_east", 2, 36, 35, 0xA45838, true),
        ("walk_east", 2, 39, 42, 0xE08C6A, true),
        ("walk_east", 2, 34, 45, 0xE08C6A, true),
        ("walk_east", 2, 45, 46, 0xF3C193, true),
        ("walk_east", 2, 36, 34, 0x000000, false),
        ("walk_east", 2, 38, 35, 0xC2B9BE, false),
        ("walk_east", 2, 38, 36, 0xECF0E9, false),
        ("walk_east", 2, 37, 27, 0x5B566E, false),
        ("walk_east", 2, 38, 27, 0x91879B, false),
        ("walk_east", 2, 39, 27, 0xD7C8CE, false),
        ("walk_east", 2, 41, 45, 0x4C0F1E, false),
        ("walk_east", 2, 39, 41, 0x6C1C2F, false),
        ("walk_east", 2, 38, 41, 0x8F4457, false),
        ("walk_east", 2, 39, 43, 0xA43259, false),
        ("walk_east", 2, 38, 51, 0xCE6470, false),
        ("walk_east", 2, 37, 42, 0xDD7192, false),
        ("walk_east", 2, 42, 41, 0x3C347B, false),
        ("walk_east", 2, 40, 41, 0x6062B6, false),
        ("walk_east", 2, 39, 53, 0xFBC9BB, false),
        ("walk_east", 2, 36, 39, 0xF4AB50, false),
        ("walk_east", 3, 41, 34, 0xFBDCB9, true),
        ("walk_east", 3, 41, 33, 0xF3C193, true),
        ("walk_east", 3, 41, 32, 0xE08C6A, true),
        ("walk_east", 3, 36, 36, 0xA45838, true),
        ("walk_east", 3, 40, 41, 0xE08C6A, true),
        ("walk_east", 3, 35, 46, 0xE08C6A, true),
        ("walk_east", 3, 44, 47, 0xA45838, true),
        ("walk_east", 3, 38, 34, 0x000000, false),
        ("walk_east", 3, 38, 36, 0xC2B9BE, false),
        ("walk_east", 3, 38, 37, 0xECF0E9, false),
        ("walk_east", 3, 37, 28, 0x5B566E, false),
        ("walk_east", 3, 38, 28, 0x91879B, false),
        ("walk_east", 3, 39, 28, 0xD7C8CE, false),
        ("walk_east", 3, 41, 46, 0x4C0F1E, false),
        ("walk_east", 3, 39, 42, 0x6C1C2F, false),
        ("walk_east", 3, 38, 42, 0x8F4457, false),
        ("walk_east", 3, 39, 44, 0xA43259, false),
        ("walk_east", 3, 37, 43, 0xDD7192, false),
        ("walk_east", 3, 42, 42, 0x3C347B, false),
        ("walk_east", 3, 40, 42, 0x6062B6, false),
        ("walk_east", 3, 36, 52, 0xFBC9BB, false),
        ("walk_east", 3, 36, 40, 0xF4AB50, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_north", &[13]),
        ("idle_south", &[59]),
        ("idle_east", &[55]),
        ("walk_north", &[13, 11, 13, 11]),
        ("walk_south", &[59, 57, 59, 57]),
        ("walk_east", &[55, 60, 55, 54]),
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
            let prefix = "spr_npc_elsie_autumn";
            let asset = format!("assets/animations/NPCs/Elsie/Sprites/Autumn/{prefix}_{case}.png");
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
                    "Elsie Autumn material mismatch: {id} {case} [{x},{y}]"
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
