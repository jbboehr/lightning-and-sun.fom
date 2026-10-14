use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/dell and the local accepted Dell world baseline"]
fn dell_autumn_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/dell");
    let baseline = root.join("local/fixtures/slice-163-build/characters/dell");
    let set = std::env::var_os("FOM_DELL_AUTUMN_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/dell-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/dell-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..89];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..89], prior);
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
    // Face and exposed hands change. Shared-color blonde hair, eyes,
    // ribbon, blue coat, red outfit and brown boots stay original.
    let landmarks = [
        ("idle_east", 0, 40, 39, 0xECC889, true),
        ("idle_east", 0, 35, 48, 0xECC889, true),
        ("idle_east", 0, 44, 48, 0x762E21, true),
        ("idle_east", 0, 37, 32, 0xDF8D4B, false),
        ("idle_east", 0, 38, 34, 0xB65932, false),
        ("idle_east", 0, 38, 35, 0x762E21, false),
        ("idle_east", 0, 34, 32, 0xFFDC90, false),
        ("idle_east", 0, 38, 33, 0xF5F5F5, false),
        ("idle_east", 0, 37, 45, 0x444D6E, false),
        ("idle_east", 0, 38, 44, 0x626A8C, false),
        ("idle_east", 0, 38, 45, 0x80AABA, false),
        ("idle_east", 0, 38, 51, 0x653127, false),
        ("idle_east", 0, 38, 51, 0x653127, false),
        ("idle_east", 0, 39, 52, 0x935436, false),
        ("idle_east", 0, 39, 51, 0xBC8A54, false),
        ("idle_east", 0, 39, 44, 0x561830, false),
        ("idle_east", 0, 40, 45, 0x7F2748, false),
        ("idle_east", 0, 40, 46, 0xDD4655, false),
        ("idle_east", 0, 34, 35, 0x491926, false),
        ("idle_north", 0, 34, 48, 0xECC889, true),
        ("idle_north", 0, 44, 48, 0x762E21, true),
        ("idle_north", 0, 37, 32, 0xDF8D4B, false),
        ("idle_north", 0, 33, 38, 0xB65932, false),
        ("idle_north", 0, 38, 32, 0xFFDC90, false),
        ("idle_north", 0, 36, 34, 0xF5F5F5, false),
        ("idle_north", 0, 37, 44, 0x444D6E, false),
        ("idle_north", 0, 38, 44, 0x626A8C, false),
        ("idle_north", 0, 39, 44, 0x80AABA, false),
        ("idle_north", 0, 37, 52, 0x935436, false),
        ("idle_north", 0, 37, 52, 0x935436, false),
        ("idle_north", 0, 38, 52, 0xBC8A54, false),
        ("idle_south", 0, 39, 39, 0xECC889, true),
        ("idle_south", 0, 34, 48, 0xECC889, true),
        ("idle_south", 0, 44, 48, 0x762E21, true),
        ("idle_south", 0, 37, 32, 0xDF8D4B, false),
        ("idle_south", 0, 38, 34, 0xB65932, false),
        ("idle_south", 0, 38, 35, 0x762E21, false),
        ("idle_south", 0, 34, 32, 0xFFDC90, false),
        ("idle_south", 0, 38, 33, 0xF5F5F5, false),
        ("idle_south", 0, 36, 45, 0x444D6E, false),
        ("idle_south", 0, 37, 44, 0x626A8C, false),
        ("idle_south", 0, 37, 45, 0x80AABA, false),
        ("idle_south", 0, 37, 51, 0x935436, false),
        ("idle_south", 0, 37, 52, 0x653127, false),
        ("idle_south", 0, 37, 51, 0x935436, false),
        ("idle_south", 0, 38, 51, 0xBC8A54, false),
        ("idle_south", 0, 38, 44, 0x561830, false),
        ("idle_south", 0, 39, 45, 0x7F2748, false),
        ("idle_south", 0, 39, 46, 0xDD4655, false),
        ("idle_south", 0, 34, 35, 0x491926, false),
        ("walk_east", 0, 40, 39, 0xECC889, true),
        ("walk_east", 0, 35, 48, 0xECC889, true),
        ("walk_east", 0, 44, 48, 0x762E21, true),
        ("walk_east", 0, 37, 32, 0xDF8D4B, false),
        ("walk_east", 0, 38, 34, 0xB65932, false),
        ("walk_east", 0, 38, 35, 0x762E21, false),
        ("walk_east", 0, 34, 32, 0xFFDC90, false),
        ("walk_east", 0, 38, 33, 0xF5F5F5, false),
        ("walk_east", 0, 37, 45, 0x444D6E, false),
        ("walk_east", 0, 38, 44, 0x626A8C, false),
        ("walk_east", 0, 38, 45, 0x80AABA, false),
        ("walk_east", 0, 38, 51, 0x653127, false),
        ("walk_east", 0, 38, 51, 0x653127, false),
        ("walk_east", 0, 39, 52, 0x935436, false),
        ("walk_east", 0, 39, 51, 0xBC8A54, false),
        ("walk_east", 0, 39, 44, 0x561830, false),
        ("walk_east", 0, 40, 45, 0x7F2748, false),
        ("walk_east", 0, 40, 46, 0xDD4655, false),
        ("walk_east", 0, 34, 35, 0x491926, false),
        ("walk_east", 1, 40, 40, 0xECC889, true),
        ("walk_east", 1, 34, 48, 0xB65932, true),
        ("walk_east", 1, 45, 48, 0x762E21, true),
        ("walk_east", 1, 37, 33, 0xDF8D4B, false),
        ("walk_east", 1, 38, 35, 0xB65932, false),
        ("walk_east", 1, 38, 36, 0x762E21, false),
        ("walk_east", 1, 34, 33, 0xFFDC90, false),
        ("walk_east", 1, 38, 34, 0xF5F5F5, false),
        ("walk_east", 1, 35, 46, 0x444D6E, false),
        ("walk_east", 1, 38, 45, 0x626A8C, false),
        ("walk_east", 1, 38, 46, 0x80AABA, false),
        ("walk_east", 1, 37, 51, 0x653127, false),
        ("walk_east", 1, 37, 51, 0x653127, false),
        ("walk_east", 1, 40, 52, 0x935436, false),
        ("walk_east", 1, 42, 52, 0xBC8A54, false),
        ("walk_east", 1, 39, 45, 0x561830, false),
        ("walk_east", 1, 40, 46, 0x7F2748, false),
        ("walk_east", 1, 40, 47, 0xDD4655, false),
        ("walk_east", 1, 34, 36, 0x491926, false),
        ("walk_east", 2, 40, 39, 0xECC889, true),
        ("walk_east", 2, 35, 48, 0xECC889, true),
        ("walk_east", 2, 44, 48, 0x762E21, true),
        ("walk_east", 2, 37, 32, 0xDF8D4B, false),
        ("walk_east", 2, 38, 34, 0xB65932, false),
        ("walk_east", 2, 38, 35, 0x762E21, false),
        ("walk_east", 2, 34, 32, 0xFFDC90, false),
        ("walk_east", 2, 38, 33, 0xF5F5F5, false),
        ("walk_east", 2, 37, 45, 0x444D6E, false),
        ("walk_east", 2, 38, 44, 0x626A8C, false),
        ("walk_east", 2, 38, 45, 0x80AABA, false),
        ("walk_east", 2, 38, 51, 0x653127, false),
        ("walk_east", 2, 38, 51, 0x653127, false),
        ("walk_east", 2, 39, 52, 0x935436, false),
        ("walk_east", 2, 39, 51, 0xBC8A54, false),
        ("walk_east", 2, 39, 44, 0x561830, false),
        ("walk_east", 2, 40, 45, 0x7F2748, false),
        ("walk_east", 2, 40, 46, 0xDD4655, false),
        ("walk_east", 2, 34, 35, 0x491926, false),
        ("walk_east", 3, 40, 40, 0xECC889, true),
        ("walk_east", 3, 36, 49, 0xECC889, true),
        ("walk_east", 3, 44, 49, 0xB65932, true),
        ("walk_east", 3, 37, 33, 0xDF8D4B, false),
        ("walk_east", 3, 38, 35, 0xB65932, false),
        ("walk_east", 3, 38, 36, 0x762E21, false),
        ("walk_east", 3, 34, 33, 0xFFDC90, false),
        ("walk_east", 3, 38, 34, 0xF5F5F5, false),
        ("walk_east", 3, 37, 46, 0x444D6E, false),
        ("walk_east", 3, 38, 45, 0x626A8C, false),
        ("walk_east", 3, 38, 46, 0x80AABA, false),
        ("walk_east", 3, 39, 51, 0x653127, false),
        ("walk_east", 3, 39, 51, 0x653127, false),
        ("walk_east", 3, 42, 51, 0x935436, false),
        ("walk_east", 3, 43, 52, 0xBC8A54, false),
        ("walk_east", 3, 39, 45, 0x561830, false),
        ("walk_east", 3, 40, 46, 0x7F2748, false),
        ("walk_east", 3, 40, 47, 0xDD4655, false),
        ("walk_east", 3, 34, 36, 0x491926, false),
        ("walk_north", 0, 34, 48, 0xECC889, true),
        ("walk_north", 0, 44, 48, 0x762E21, true),
        ("walk_north", 0, 37, 32, 0xDF8D4B, false),
        ("walk_north", 0, 33, 38, 0xB65932, false),
        ("walk_north", 0, 38, 32, 0xFFDC90, false),
        ("walk_north", 0, 36, 34, 0xF5F5F5, false),
        ("walk_north", 0, 37, 44, 0x444D6E, false),
        ("walk_north", 0, 38, 44, 0x626A8C, false),
        ("walk_north", 0, 39, 44, 0x80AABA, false),
        ("walk_north", 0, 37, 52, 0x935436, false),
        ("walk_north", 0, 37, 52, 0x935436, false),
        ("walk_north", 0, 38, 52, 0xBC8A54, false),
        ("walk_north", 1, 35, 50, 0xECC889, true),
        ("walk_north", 1, 44, 48, 0xDF8D4B, true),
        ("walk_north", 1, 37, 33, 0xDF8D4B, false),
        ("walk_north", 1, 33, 39, 0xB65932, false),
        ("walk_north", 1, 38, 33, 0xFFDC90, false),
        ("walk_north", 1, 36, 35, 0xF5F5F5, false),
        ("walk_north", 1, 37, 45, 0x444D6E, false),
        ("walk_north", 1, 38, 45, 0x626A8C, false),
        ("walk_north", 1, 39, 45, 0x80AABA, false),
        ("walk_north", 1, 37, 52, 0x653127, false),
        ("walk_north", 1, 37, 52, 0x653127, false),
        ("walk_north", 1, 38, 52, 0x935436, false),
        ("walk_north", 1, 41, 53, 0xBC8A54, false),
        ("walk_north", 2, 34, 48, 0xECC889, true),
        ("walk_north", 2, 44, 48, 0x762E21, true),
        ("walk_north", 2, 37, 32, 0xDF8D4B, false),
        ("walk_north", 2, 33, 38, 0xB65932, false),
        ("walk_north", 2, 38, 32, 0xFFDC90, false),
        ("walk_north", 2, 36, 34, 0xF5F5F5, false),
        ("walk_north", 2, 37, 44, 0x444D6E, false),
        ("walk_north", 2, 38, 44, 0x626A8C, false),
        ("walk_north", 2, 39, 44, 0x80AABA, false),
        ("walk_north", 2, 37, 52, 0x935436, false),
        ("walk_north", 2, 37, 52, 0x935436, false),
        ("walk_north", 2, 38, 52, 0xBC8A54, false),
        ("walk_north", 3, 35, 48, 0xDF8D4B, true),
        ("walk_north", 3, 43, 50, 0x762E21, true),
        ("walk_north", 3, 37, 33, 0xDF8D4B, false),
        ("walk_north", 3, 33, 39, 0xB65932, false),
        ("walk_north", 3, 38, 33, 0xFFDC90, false),
        ("walk_north", 3, 36, 35, 0xF5F5F5, false),
        ("walk_north", 3, 37, 45, 0x444D6E, false),
        ("walk_north", 3, 38, 45, 0x626A8C, false),
        ("walk_north", 3, 39, 45, 0x80AABA, false),
        ("walk_north", 3, 41, 52, 0x935436, false),
        ("walk_north", 3, 42, 52, 0x653127, false),
        ("walk_north", 3, 41, 52, 0x935436, false),
        ("walk_north", 3, 38, 53, 0xBC8A54, false),
        ("walk_south", 0, 39, 39, 0xECC889, true),
        ("walk_south", 0, 34, 48, 0xECC889, true),
        ("walk_south", 0, 44, 48, 0x762E21, true),
        ("walk_south", 0, 37, 32, 0xDF8D4B, false),
        ("walk_south", 0, 38, 34, 0xB65932, false),
        ("walk_south", 0, 38, 35, 0x762E21, false),
        ("walk_south", 0, 34, 32, 0xFFDC90, false),
        ("walk_south", 0, 38, 33, 0xF5F5F5, false),
        ("walk_south", 0, 36, 45, 0x444D6E, false),
        ("walk_south", 0, 37, 44, 0x626A8C, false),
        ("walk_south", 0, 37, 45, 0x80AABA, false),
        ("walk_south", 0, 37, 51, 0x935436, false),
        ("walk_south", 0, 37, 52, 0x653127, false),
        ("walk_south", 0, 37, 51, 0x935436, false),
        ("walk_south", 0, 38, 51, 0xBC8A54, false),
        ("walk_south", 0, 38, 44, 0x561830, false),
        ("walk_south", 0, 39, 45, 0x7F2748, false),
        ("walk_south", 0, 39, 46, 0xDD4655, false),
        ("walk_south", 0, 34, 35, 0x491926, false),
        ("walk_south", 1, 39, 40, 0xECC889, true),
        ("walk_south", 1, 35, 50, 0xECC889, true),
        ("walk_south", 1, 44, 48, 0xDF8D4B, true),
        ("walk_south", 1, 37, 33, 0xDF8D4B, false),
        ("walk_south", 1, 38, 35, 0xB65932, false),
        ("walk_south", 1, 38, 36, 0x762E21, false),
        ("walk_south", 1, 34, 33, 0xFFDC90, false),
        ("walk_south", 1, 38, 34, 0xF5F5F5, false),
        ("walk_south", 1, 36, 46, 0x444D6E, false),
        ("walk_south", 1, 37, 45, 0x626A8C, false),
        ("walk_south", 1, 37, 46, 0x80AABA, false),
        ("walk_south", 1, 38, 51, 0x653127, false),
        ("walk_south", 1, 38, 51, 0x653127, false),
        ("walk_south", 1, 37, 52, 0x935436, false),
        ("walk_south", 1, 41, 52, 0xBC8A54, false),
        ("walk_south", 1, 38, 44, 0x561830, false),
        ("walk_south", 1, 39, 46, 0x7F2748, false),
        ("walk_south", 1, 39, 47, 0xDD4655, false),
        ("walk_south", 1, 34, 36, 0x491926, false),
        ("walk_south", 2, 39, 39, 0xECC889, true),
        ("walk_south", 2, 34, 48, 0xECC889, true),
        ("walk_south", 2, 44, 48, 0x762E21, true),
        ("walk_south", 2, 37, 32, 0xDF8D4B, false),
        ("walk_south", 2, 38, 34, 0xB65932, false),
        ("walk_south", 2, 38, 35, 0x762E21, false),
        ("walk_south", 2, 34, 32, 0xFFDC90, false),
        ("walk_south", 2, 38, 33, 0xF5F5F5, false),
        ("walk_south", 2, 36, 45, 0x444D6E, false),
        ("walk_south", 2, 37, 44, 0x626A8C, false),
        ("walk_south", 2, 37, 45, 0x80AABA, false),
        ("walk_south", 2, 37, 51, 0x935436, false),
        ("walk_south", 2, 37, 52, 0x653127, false),
        ("walk_south", 2, 37, 51, 0x935436, false),
        ("walk_south", 2, 38, 51, 0xBC8A54, false),
        ("walk_south", 2, 38, 44, 0x561830, false),
        ("walk_south", 2, 39, 45, 0x7F2748, false),
        ("walk_south", 2, 39, 46, 0xDD4655, false),
        ("walk_south", 2, 34, 35, 0x491926, false),
        ("walk_south", 3, 39, 40, 0xECC889, true),
        ("walk_south", 3, 35, 48, 0xDF8D4B, true),
        ("walk_south", 3, 43, 50, 0x762E21, true),
        ("walk_south", 3, 37, 33, 0xDF8D4B, false),
        ("walk_south", 3, 38, 35, 0xB65932, false),
        ("walk_south", 3, 38, 36, 0x762E21, false),
        ("walk_south", 3, 34, 33, 0xFFDC90, false),
        ("walk_south", 3, 38, 34, 0xF5F5F5, false),
        ("walk_south", 3, 36, 46, 0x444D6E, false),
        ("walk_south", 3, 37, 45, 0x626A8C, false),
        ("walk_south", 3, 37, 46, 0x80AABA, false),
        ("walk_south", 3, 41, 51, 0x653127, false),
        ("walk_south", 3, 41, 51, 0x653127, false),
        ("walk_south", 3, 37, 52, 0x935436, false),
        ("walk_south", 3, 38, 52, 0xBC8A54, false),
        ("walk_south", 3, 38, 44, 0x561830, false),
        ("walk_south", 3, 39, 46, 0x7F2748, false),
        ("walk_south", 3, 39, 47, 0xDD4655, false),
        ("walk_south", 3, 34, 36, 0x491926, false),
        ("idle_south", 0, 39, 39, 0xECC889, true),
        ("idle_south", 0, 38, 41, 0xDF8D4B, true),
        ("idle_south", 0, 36, 39, 0xB65932, true),
        ("idle_south", 0, 36, 41, 0x762E21, true),
        ("idle_south", 0, 37, 39, 0xECF0E9, false),
        ("idle_south", 0, 37, 38, 0xC2B9BE, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[30]),
        ("idle_north", &[4]),
        ("idle_south", &[34]),
        ("walk_east", &[30, 30, 30, 29]),
        ("walk_north", &[4, 3, 4, 3]),
        ("walk_south", &[34, 33, 34, 33]),
    ];
    // Reviewed original face rows, independent of the component seeds.
    // Only the hands emerge from the Autumn coat; trousers cover the legs.
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
        let frame = (x / 80) as usize;
        let hands: &[(u32, u32)] = if case.starts_with("walk") && frame % 2 == 1 {
            if case.ends_with("east") {
                if frame == 1 {
                    &[(34, 48), (35, 48), (45, 48), (46, 48)]
                } else {
                    &[(36, 49), (37, 49), (44, 49)]
                }
            } else if frame == 1 {
                &[(44, 48), (35, 50), (36, 50)]
            } else {
                &[(35, 48), (43, 50), (44, 50)]
            }
        } else if case.ends_with("east") {
            &[(35, 48), (36, 48), (44, 48), (45, 48)]
        } else {
            &[(34, 48), (35, 48), (44, 48), (45, 48)]
        };
        if hands.contains(&(x % 80, y)) {
            return true;
        }
        if case.ends_with("north") {
            return false;
        }
        let bob = u32::from(case.starts_with("walk") && (x / 80) % 2 == 1);
        let rows = if case.ends_with("east") {
            &east_face
        } else {
            &south_face
        };
        rows.iter()
            .any(|&(yy, lo, hi)| y == yy + bob && (lo..hi).contains(&(x % 80)))
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
            let prefix = "spr_npc_dell_autumn";
            let asset = format!("assets/animations/NPCs/Dell/Sprites/Autumn/{prefix}_{case}.png");
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
                    "Dell Autumn material mismatch: {id} {case} [{x},{y}]"
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
