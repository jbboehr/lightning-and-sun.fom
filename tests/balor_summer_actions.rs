use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/balor-autumn-finish-study and the local accepted Balor world baseline"]
fn balor_summer_actions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-autumn-finish-study");
    let baseline = root
        .join("generated/characters-balor-summer-valen-heal-eiland-magnify-trial/characters/balor");
    let set = std::env::var_os("FOM_BALOR_SUMMER_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/balor-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..150];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..150], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 201);
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
    // Literal source landmarks cover every Summer frame and the moving hand,
    // wrist accessories, shirt, hair, belt/shoes and open-mouth interiors.
    // All four reviewed world shades are skin in these eleven strips.
    let landmarks = [
        ("blink_east", 0, 41, 35, 0xFCD9B3, true),
        ("blink_east", 0, 36, 36, 0xD37A57, true),
        ("blink_east", 0, 37, 37, 0x672115, true),
        ("blink_east", 0, 39, 28, 0x686589, false),
        ("blink_east", 0, 38, 45, 0x612026, false),
        ("blink_east", 0, 38, 40, 0xB395D6, false),
        ("blink_east", 0, 38, 41, 0xDEDAE9, false),
        ("blink_east", 0, 34, 44, 0x374050, false),
        ("blink_east", 1, 39, 35, 0xFCD9B3, true),
        ("blink_east", 1, 36, 36, 0xD37A57, true),
        ("blink_east", 1, 37, 37, 0x672115, true),
        ("blink_east", 1, 39, 28, 0x686589, false),
        ("blink_east", 1, 38, 45, 0x612026, false),
        ("blink_east", 1, 38, 40, 0xB395D6, false),
        ("blink_east", 1, 38, 41, 0xDEDAE9, false),
        ("blink_east", 1, 34, 44, 0x374050, false),
        ("blink_east", 2, 41, 35, 0xFCD9B3, true),
        ("blink_east", 2, 36, 36, 0xD37A57, true),
        ("blink_east", 2, 37, 37, 0x672115, true),
        ("blink_east", 2, 39, 28, 0x686589, false),
        ("blink_east", 2, 38, 45, 0x612026, false),
        ("blink_east", 2, 38, 40, 0xB395D6, false),
        ("blink_east", 2, 38, 41, 0xDEDAE9, false),
        ("blink_east", 2, 34, 44, 0x374050, false),
        ("blink_south", 0, 40, 35, 0xFCD9B3, true),
        ("blink_south", 0, 35, 36, 0xD37A57, true),
        ("blink_south", 0, 36, 37, 0x672115, true),
        ("blink_south", 0, 39, 28, 0x686589, false),
        ("blink_south", 0, 37, 45, 0x612026, false),
        ("blink_south", 0, 37, 40, 0xB395D6, false),
        ("blink_south", 0, 37, 41, 0xDEDAE9, false),
        ("blink_south", 0, 33, 44, 0x374050, false),
        ("blink_south", 1, 38, 35, 0xFCD9B3, true),
        ("blink_south", 1, 35, 36, 0xD37A57, true),
        ("blink_south", 1, 36, 37, 0x672115, true),
        ("blink_south", 1, 39, 28, 0x686589, false),
        ("blink_south", 1, 37, 45, 0x612026, false),
        ("blink_south", 1, 37, 40, 0xB395D6, false),
        ("blink_south", 1, 37, 41, 0xDEDAE9, false),
        ("blink_south", 1, 33, 44, 0x374050, false),
        ("blink_south", 2, 40, 35, 0xFCD9B3, true),
        ("blink_south", 2, 35, 36, 0xD37A57, true),
        ("blink_south", 2, 36, 37, 0x672115, true),
        ("blink_south", 2, 39, 28, 0x686589, false),
        ("blink_south", 2, 37, 45, 0x612026, false),
        ("blink_south", 2, 37, 40, 0xB395D6, false),
        ("blink_south", 2, 37, 41, 0xDEDAE9, false),
        ("blink_south", 2, 33, 44, 0x374050, false),
        ("sit_north", 0, 34, 47, 0x672115, true),
        ("sit_north", 0, 39, 29, 0x686589, false),
        ("sit_north", 0, 42, 46, 0x612026, false),
        ("sit_north", 0, 36, 42, 0xB395D6, false),
        ("sit_north", 0, 37, 42, 0xDEDAE9, false),
        ("sit_north", 0, 34, 45, 0x374050, false),
        ("sit_south", 0, 40, 36, 0xFCD9B3, true),
        ("sit_south", 0, 35, 37, 0xD37A57, true),
        ("sit_south", 0, 36, 38, 0x672115, true),
        ("sit_south", 0, 39, 29, 0x686589, false),
        ("sit_south", 0, 37, 46, 0x612026, false),
        ("sit_south", 0, 37, 41, 0xB395D6, false),
        ("sit_south", 0, 37, 42, 0xDEDAE9, false),
        ("sit_south", 0, 34, 45, 0x374050, false),
        ("sit_east", 0, 41, 36, 0xFCD9B3, true),
        ("sit_east", 0, 36, 37, 0xD37A57, true),
        ("sit_east", 0, 37, 38, 0x672115, true),
        ("sit_east", 0, 39, 29, 0x686589, false),
        ("sit_east", 0, 39, 46, 0x612026, false),
        ("sit_east", 0, 38, 41, 0xB395D6, false),
        ("sit_east", 0, 38, 42, 0xDEDAE9, false),
        ("sit_east", 0, 35, 45, 0x374050, false),
        ("eat_north", 0, 44, 44, 0xFCD9B3, true),
        ("eat_north", 0, 46, 42, 0xD37A57, true),
        ("eat_north", 0, 45, 42, 0x672115, true),
        ("eat_north", 0, 39, 29, 0x686589, false),
        ("eat_north", 0, 42, 46, 0x612026, false),
        ("eat_north", 0, 36, 42, 0xB395D6, false),
        ("eat_north", 0, 37, 42, 0xDEDAE9, false),
        ("eat_north", 0, 46, 43, 0x374050, false),
        ("eat_north", 1, 45, 41, 0xD37A57, true),
        ("eat_north", 1, 45, 43, 0x672115, true),
        ("eat_north", 1, 39, 30, 0x686589, false),
        ("eat_north", 1, 42, 46, 0x612026, false),
        ("eat_north", 1, 36, 43, 0xB395D6, false),
        ("eat_north", 1, 43, 42, 0xDEDAE9, false),
        ("eat_north", 1, 45, 42, 0x374050, false),
        ("eat_north", 2, 44, 44, 0xFCD9B3, true),
        ("eat_north", 2, 46, 42, 0xD37A57, true),
        ("eat_north", 2, 45, 42, 0x672115, true),
        ("eat_north", 2, 39, 29, 0x686589, false),
        ("eat_north", 2, 42, 46, 0x612026, false),
        ("eat_north", 2, 36, 42, 0xB395D6, false),
        ("eat_north", 2, 37, 42, 0xDEDAE9, false),
        ("eat_north", 2, 46, 43, 0x374050, false),
        ("eat_south", 0, 40, 36, 0xFCD9B3, true),
        ("eat_south", 0, 35, 37, 0xD37A57, true),
        ("eat_south", 0, 36, 38, 0x672115, true),
        ("eat_south", 0, 39, 29, 0x686589, false),
        ("eat_south", 0, 37, 50, 0x612026, false),
        ("eat_south", 0, 37, 41, 0xB395D6, false),
        ("eat_south", 0, 37, 42, 0xDEDAE9, false),
        ("eat_south", 0, 45, 45, 0x374050, false),
        ("eat_south", 1, 40, 37, 0xFCD9B3, true),
        ("eat_south", 1, 35, 38, 0xD37A57, true),
        ("eat_south", 1, 36, 39, 0x672115, true),
        ("eat_south", 1, 39, 30, 0x686589, false),
        ("eat_south", 1, 41, 50, 0x612026, false),
        ("eat_south", 1, 42, 42, 0xB395D6, false),
        ("eat_south", 1, 42, 43, 0xDEDAE9, false),
        ("eat_south", 1, 45, 45, 0x374050, false),
        ("eat_south", 1, 39, 40, 0x9E2626, false),
        ("eat_south", 2, 40, 34, 0xFCD9B3, true),
        ("eat_south", 2, 35, 36, 0xD37A57, true),
        ("eat_south", 2, 36, 38, 0x672115, true),
        ("eat_south", 2, 39, 28, 0x686589, false),
        ("eat_south", 2, 37, 46, 0x612026, false),
        ("eat_south", 2, 42, 41, 0xB395D6, false),
        ("eat_south", 2, 42, 42, 0xDEDAE9, false),
        ("eat_south", 2, 36, 40, 0x374050, false),
        ("eat_south", 2, 38, 35, 0x410808, false),
        ("eat_south", 2, 38, 36, 0x9E2626, false),
        ("eat_south", 3, 40, 37, 0xFCD9B3, true),
        ("eat_south", 3, 35, 38, 0xD37A57, true),
        ("eat_south", 3, 36, 39, 0x672115, true),
        ("eat_south", 3, 39, 30, 0x686589, false),
        ("eat_south", 3, 37, 47, 0x612026, false),
        ("eat_south", 3, 42, 42, 0xB395D6, false),
        ("eat_south", 3, 42, 43, 0xDEDAE9, false),
        ("eat_south", 3, 35, 43, 0x374050, false),
        ("eat_south", 4, 40, 36, 0xFCD9B3, true),
        ("eat_south", 4, 35, 37, 0xD37A57, true),
        ("eat_south", 4, 36, 38, 0x672115, true),
        ("eat_south", 4, 39, 29, 0x686589, false),
        ("eat_south", 4, 37, 46, 0x612026, false),
        ("eat_south", 4, 37, 41, 0xB395D6, false),
        ("eat_south", 4, 37, 42, 0xDEDAE9, false),
        ("eat_south", 4, 34, 45, 0x374050, false),
        ("eat_east", 0, 41, 36, 0xFCD9B3, true),
        ("eat_east", 0, 36, 37, 0xD37A57, true),
        ("eat_east", 0, 37, 38, 0x672115, true),
        ("eat_east", 0, 39, 29, 0x686589, false),
        ("eat_east", 0, 39, 46, 0x612026, false),
        ("eat_east", 0, 38, 41, 0xB395D6, false),
        ("eat_east", 0, 38, 42, 0xDEDAE9, false),
        ("eat_east", 1, 42, 36, 0xFCD9B3, true),
        ("eat_east", 1, 37, 37, 0xD37A57, true),
        ("eat_east", 1, 38, 38, 0x672115, true),
        ("eat_east", 1, 40, 29, 0x686589, false),
        ("eat_east", 1, 39, 46, 0x612026, false),
        ("eat_east", 1, 40, 41, 0xB395D6, false),
        ("eat_east", 1, 41, 41, 0xDEDAE9, false),
        ("eat_east", 1, 45, 42, 0x374050, false),
        ("eat_east", 1, 41, 39, 0x9E2626, false),
        ("eat_east", 2, 41, 34, 0xFCD9B3, true),
        ("eat_east", 2, 36, 36, 0xD37A57, true),
        ("eat_east", 2, 39, 39, 0x672115, true),
        ("eat_east", 2, 38, 28, 0x686589, false),
        ("eat_east", 2, 39, 46, 0x612026, false),
        ("eat_east", 2, 39, 41, 0xB395D6, false),
        ("eat_east", 2, 40, 41, 0xDEDAE9, false),
        ("eat_east", 2, 43, 41, 0x374050, false),
        ("eat_east", 2, 40, 35, 0x410808, false),
        ("eat_east", 2, 40, 36, 0x9E2626, false),
        ("eat_east", 3, 41, 37, 0xFCD9B3, true),
        ("eat_east", 3, 36, 38, 0xD37A57, true),
        ("eat_east", 3, 37, 39, 0x672115, true),
        ("eat_east", 3, 39, 30, 0x686589, false),
        ("eat_east", 3, 39, 46, 0x612026, false),
        ("eat_east", 3, 39, 42, 0xB395D6, false),
        ("eat_east", 3, 39, 43, 0xDEDAE9, false),
        ("eat_east", 4, 41, 36, 0xFCD9B3, true),
        ("eat_east", 4, 36, 37, 0xD37A57, true),
        ("eat_east", 4, 37, 38, 0x672115, true),
        ("eat_east", 4, 39, 29, 0x686589, false),
        ("eat_east", 4, 39, 46, 0x612026, false),
        ("eat_east", 4, 38, 41, 0xB395D6, false),
        ("eat_east", 4, 39, 42, 0xDEDAE9, false),
        ("drink_north", 0, 44, 44, 0xFCD9B3, true),
        ("drink_north", 0, 46, 42, 0xD37A57, true),
        ("drink_north", 0, 45, 42, 0x672115, true),
        ("drink_north", 0, 39, 29, 0x686589, false),
        ("drink_north", 0, 42, 46, 0x612026, false),
        ("drink_north", 0, 36, 42, 0xB395D6, false),
        ("drink_north", 0, 37, 42, 0xDEDAE9, false),
        ("drink_north", 0, 46, 43, 0x374050, false),
        ("drink_north", 1, 45, 41, 0xD37A57, true),
        ("drink_north", 1, 45, 43, 0x672115, true),
        ("drink_north", 1, 39, 30, 0x686589, false),
        ("drink_north", 1, 42, 46, 0x612026, false),
        ("drink_north", 1, 36, 43, 0xB395D6, false),
        ("drink_north", 1, 43, 42, 0xDEDAE9, false),
        ("drink_north", 1, 45, 42, 0x374050, false),
        ("drink_north", 2, 44, 44, 0xFCD9B3, true),
        ("drink_north", 2, 46, 42, 0xD37A57, true),
        ("drink_north", 2, 45, 42, 0x672115, true),
        ("drink_north", 2, 39, 29, 0x686589, false),
        ("drink_north", 2, 42, 46, 0x612026, false),
        ("drink_north", 2, 36, 42, 0xB395D6, false),
        ("drink_north", 2, 37, 42, 0xDEDAE9, false),
        ("drink_north", 2, 46, 43, 0x374050, false),
        ("drink_south", 0, 40, 36, 0xFCD9B3, true),
        ("drink_south", 0, 35, 37, 0xD37A57, true),
        ("drink_south", 0, 36, 38, 0x672115, true),
        ("drink_south", 0, 39, 29, 0x686589, false),
        ("drink_south", 0, 37, 46, 0x612026, false),
        ("drink_south", 0, 37, 41, 0xB395D6, false),
        ("drink_south", 0, 37, 42, 0xDEDAE9, false),
        ("drink_south", 0, 45, 45, 0x374050, false),
        ("drink_south", 1, 40, 37, 0xFCD9B3, true),
        ("drink_south", 1, 35, 38, 0xD37A57, true),
        ("drink_south", 1, 43, 39, 0x672115, true),
        ("drink_south", 1, 39, 30, 0x686589, false),
        ("drink_south", 1, 37, 46, 0x612026, false),
        ("drink_south", 1, 42, 42, 0xB395D6, false),
        ("drink_south", 1, 37, 43, 0xDEDAE9, false),
        ("drink_south", 1, 35, 43, 0x374050, false),
        ("drink_south", 2, 40, 36, 0xFCD9B3, true),
        ("drink_south", 2, 35, 37, 0xD37A57, true),
        ("drink_south", 2, 36, 38, 0x672115, true),
        ("drink_south", 2, 39, 29, 0x686589, false),
        ("drink_south", 2, 37, 46, 0x612026, false),
        ("drink_south", 2, 37, 41, 0xB395D6, false),
        ("drink_south", 2, 37, 42, 0xDEDAE9, false),
        ("drink_south", 2, 45, 45, 0x374050, false),
        ("drink_east", 0, 41, 36, 0xFCD9B3, true),
        ("drink_east", 0, 36, 37, 0xD37A57, true),
        ("drink_east", 0, 37, 38, 0x672115, true),
        ("drink_east", 0, 39, 29, 0x686589, false),
        ("drink_east", 0, 39, 46, 0x612026, false),
        ("drink_east", 0, 38, 41, 0xB395D6, false),
        ("drink_east", 0, 38, 42, 0xDEDAE9, false),
        ("drink_east", 0, 41, 44, 0x374050, false),
        ("drink_east", 0, 36, 44, 0x010101, false),
        ("drink_east", 1, 37, 35, 0xFCD9B3, true),
        ("drink_east", 1, 34, 37, 0xD37A57, true),
        ("drink_east", 1, 35, 38, 0x672115, true),
        ("drink_east", 1, 36, 29, 0x686589, false),
        ("drink_east", 1, 39, 46, 0x612026, false),
        ("drink_east", 1, 37, 41, 0xB395D6, false),
        ("drink_east", 1, 36, 43, 0xDEDAE9, false),
        ("drink_east", 1, 40, 42, 0x374050, false),
        ("drink_east", 1, 36, 40, 0x010101, false),
        ("drink_east", 2, 41, 36, 0xFCD9B3, true),
        ("drink_east", 2, 36, 37, 0xD37A57, true),
        ("drink_east", 2, 37, 38, 0x672115, true),
        ("drink_east", 2, 39, 29, 0x686589, false),
        ("drink_east", 2, 39, 46, 0x612026, false),
        ("drink_east", 2, 38, 41, 0xB395D6, false),
        ("drink_east", 2, 38, 42, 0xDEDAE9, false),
        ("drink_east", 2, 41, 44, 0x374050, false),
        ("drink_east", 2, 36, 44, 0x010101, false),
        ("drink_east", 1, 39, 40, 0xFCD9B3, true),
        ("drink_east", 1, 40, 40, 0xF0B988, true),
        ("drink_east", 1, 40, 42, 0x374050, false),
        ("drink_east", 1, 39, 46, 0x612026, false),
    ];
    let cases: [(&str, &[usize]); 11] = [
        ("blink_east", &[43, 47, 43]),
        ("blink_south", &[51, 55, 51]),
        ("sit_north", &[9]),
        ("sit_south", &[38]),
        ("sit_east", &[34]),
        ("eat_north", &[12, 7, 12]),
        ("eat_south", &[40, 42, 35, 46, 38]),
        ("eat_east", &[30, 32, 29, 32, 35]),
        ("drink_north", &[12, 7, 12]),
        ("drink_south", &[40, 45, 40]),
        ("drink_east", &[31, 39, 31]),
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
            let prefix = "spr_npc_balor_summer";
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
            let mut belt_and_shoe_shadows = 0;
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                if p.0 == rgba(0x612026) {
                    assert_eq!(p, q, "belt or shoe shadow changed: {id} {case} [{x},{y}]");
                    belt_and_shoe_shadows += 1;
                }
                // All four reviewed world shades are skin in these Summer strips.
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
                    let index = source.iter().position(|c| rgba(*c) == p.0).expect(
                        "hair, shirt, belt, shoes, cuffs, eyes or another non-skin color changed",
                    );
                    assert_eq!(q.0, rgba(target[index]));
                    per_frame[x as usize / 80] += 1;
                }
            }
            assert_eq!(
                belt_and_shoe_shadows,
                match case {
                    "blink_east" => 21,
                    "blink_south" => 21,
                    "drink_east" => 18,
                    "drink_north" => 9,
                    "drink_south" => 21,
                    "eat_east" => 30,
                    "eat_north" => 9,
                    "eat_south" => 26,
                    "sit_east" => 6,
                    "sit_north" => 3,
                    "sit_south" => 7,
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
