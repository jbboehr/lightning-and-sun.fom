use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/nora and the local accepted Nora world baseline"]
fn nora_spring_reactions_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/nora");
    let baseline = root.join("local/fixtures/slice-062-build/characters/nora");
    let set = std::env::var_os("FOM_NORA_SPRING_REACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/nora-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/nora-world-trial.json")).unwrap(),
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
    let source = [0xFCD9B3, 0xF0B988, 0xD37A57, 0x762E21];
    // Source grids separate skin from hair, blouse, overalls, shoes and mouth interiors.
    // In these pinned strips, the deepest skin shade belongs to hands from row 41.
    // The same shade above that row is hair; the other world shades are skin.
    let landmarks = [
        ("action_east", 0, 44, 48, 0x000000, false),
        ("action_east", 0, 42, 50, 0x163253, false),
        ("action_east", 0, 38, 51, 0x254397, false),
        ("action_east", 0, 39, 53, 0x3B5370, false),
        ("action_east", 0, 43, 46, 0x5078E7, false),
        ("action_east", 0, 44, 40, 0x762E21, false),
        ("action_east", 0, 40, 48, 0x762E21, true),
        ("action_east", 0, 43, 43, 0x91AEFF, false),
        ("action_east", 0, 44, 36, 0xA59DA2, false),
        ("action_east", 0, 43, 33, 0xB65932, false),
        ("action_east", 0, 39, 44, 0xBEBC91, false),
        ("action_east", 0, 40, 36, 0xC2B9BE, false),
        ("action_east", 0, 45, 40, 0xCC9155, false),
        ("action_east", 0, 38, 47, 0xD37A57, true),
        ("action_east", 0, 39, 38, 0xECF0E9, false),
        ("action_east", 0, 38, 40, 0xEFCC86, false),
        ("action_east", 0, 41, 42, 0xF0B988, true),
        ("action_east", 0, 39, 33, 0xFAECB2, false),
        ("action_east", 0, 41, 47, 0xFCD9B3, true),
        ("action_east", 0, 42, 43, 0xFFFDD3, false),
        ("action_east", 1, 37, 31, 0x762E21, false),
        ("action_east", 1, 48, 45, 0x762E21, true),
        ("action_east", 1, 49, 45, 0xD37A57, true),
        ("action_east", 1, 38, 51, 0xF0B988, true),
        ("action_east", 1, 47, 44, 0xFCD9B3, true),
        ("action_east", 3, 38, 51, 0xF0B988, true),
        ("action_north", 0, 35, 46, 0x000000, false),
        ("action_north", 0, 42, 48, 0x163253, false),
        ("action_north", 0, 37, 45, 0x254397, false),
        ("action_north", 0, 39, 48, 0x3B5370, false),
        ("action_north", 0, 38, 44, 0x5078E7, false),
        ("action_north", 0, 40, 34, 0x762E21, false),
        ("action_north", 0, 33, 45, 0x762E21, true),
        ("action_north", 0, 40, 41, 0x91AEFF, false),
        ("action_north", 0, 45, 32, 0xB65932, false),
        ("action_north", 0, 34, 43, 0xBEBC91, false),
        ("action_north", 0, 43, 34, 0xCC9155, false),
        ("action_north", 0, 33, 43, 0xD37A57, true),
        ("action_north", 0, 42, 32, 0xEFCC86, false),
        ("action_north", 0, 37, 52, 0xF0B988, true),
        ("action_north", 0, 40, 31, 0xFAECB2, false),
        ("action_north", 0, 32, 45, 0xFCD9B3, true),
        ("action_north", 0, 36, 42, 0xFFFDD3, false),
        ("action_north", 1, 40, 33, 0x762E21, false),
        ("action_north", 1, 35, 46, 0x762E21, true),
        ("action_north", 1, 34, 44, 0xD37A57, true),
        ("action_north", 1, 42, 51, 0xF0B988, true),
        ("action_north", 1, 34, 46, 0xFCD9B3, true),
        ("action_north", 2, 42, 51, 0xF0B988, true),
        ("action_north", 3, 42, 51, 0xF0B988, true),
        ("action_north", 4, 42, 51, 0xF0B988, true),
        ("action_north", 5, 37, 52, 0xF0B988, true),
        ("action_north", 6, 42, 52, 0xF0B988, true),
        ("action_south", 0, 44, 48, 0x000000, false),
        ("action_south", 0, 37, 49, 0x163253, false),
        ("action_south", 0, 40, 46, 0x254397, false),
        ("action_south", 0, 39, 49, 0x3B5370, false),
        ("action_south", 0, 38, 46, 0x5078E7, false),
        ("action_south", 0, 42, 40, 0x762E21, false),
        ("action_south", 0, 34, 48, 0x762E21, true),
        ("action_south", 0, 41, 43, 0x91AEFF, false),
        ("action_south", 0, 40, 33, 0xB65932, false),
        ("action_south", 0, 36, 45, 0xBEBC91, false),
        ("action_south", 0, 41, 36, 0xC2B9BE, false),
        ("action_south", 0, 35, 32, 0xCC9155, false),
        ("action_south", 0, 45, 46, 0xD37A57, true),
        ("action_south", 0, 37, 38, 0xECF0E9, false),
        ("action_south", 0, 43, 40, 0xEFCC86, false),
        ("action_south", 0, 40, 42, 0xF0B988, true),
        ("action_south", 0, 42, 33, 0xFAECB2, false),
        ("action_south", 0, 34, 47, 0xFCD9B3, true),
        ("action_south", 0, 40, 43, 0xFFFDD3, false),
        ("action_south", 1, 40, 29, 0x762E21, false),
        ("action_south", 1, 35, 47, 0x762E21, true),
        ("action_south", 1, 45, 45, 0xD37A57, true),
        ("action_south", 1, 38, 52, 0xF0B988, true),
        ("action_south", 1, 36, 46, 0xFCD9B3, true),
        ("action_south", 3, 38, 52, 0xF0B988, true),
        ("action_south", 6, 38, 52, 0xF0B988, true),
        ("shocked_end_south", 0, 48, 48, 0x000000, false),
        ("shocked_end_south", 0, 37, 49, 0x163253, false),
        ("shocked_end_south", 0, 40, 46, 0x254397, false),
        ("shocked_end_south", 0, 39, 49, 0x3B5370, false),
        ("shocked_end_south", 0, 38, 46, 0x5078E7, false),
        ("shocked_end_south", 0, 42, 40, 0x762E21, false),
        ("shocked_end_south", 0, 46, 48, 0x762E21, true),
        ("shocked_end_south", 0, 41, 43, 0x91AEFF, false),
        ("shocked_end_south", 0, 45, 33, 0xB65932, false),
        ("shocked_end_south", 0, 36, 45, 0xBEBC91, false),
        ("shocked_end_south", 0, 35, 32, 0xCC9155, false),
        ("shocked_end_south", 0, 33, 46, 0xD37A57, true),
        ("shocked_end_south", 0, 43, 40, 0xEFCC86, false),
        ("shocked_end_south", 0, 40, 42, 0xF0B988, true),
        ("shocked_end_south", 0, 42, 33, 0xFAECB2, false),
        ("shocked_end_south", 0, 46, 47, 0xFCD9B3, true),
        ("shocked_end_south", 0, 40, 43, 0xFFFDD3, false),
        ("shocked_loop_south", 0, 39, 48, 0x000000, false),
        ("shocked_loop_south", 0, 43, 46, 0x163253, false),
        ("shocked_loop_south", 0, 36, 47, 0x254397, false),
        ("shocked_loop_south", 0, 39, 46, 0x3B5370, false),
        ("shocked_loop_south", 0, 40, 36, 0x410808, false),
        ("shocked_loop_south", 0, 39, 43, 0x5078E7, false),
        ("shocked_loop_south", 0, 36, 27, 0x762E21, false),
        ("shocked_loop_south", 0, 42, 40, 0x91AEFF, false),
        ("shocked_loop_south", 0, 40, 37, 0x9E2626, false),
        ("shocked_loop_south", 0, 41, 30, 0xB65932, false),
        ("shocked_loop_south", 0, 43, 40, 0xBEBC91, false),
        ("shocked_loop_south", 0, 41, 33, 0xC2B9BE, false),
        ("shocked_loop_south", 0, 40, 29, 0xCC9155, false),
        ("shocked_loop_south", 0, 46, 49, 0xD37A57, true),
        ("shocked_loop_south", 0, 38, 35, 0xECF0E9, false),
        ("shocked_loop_south", 0, 39, 29, 0xEFCC86, false),
        ("shocked_loop_south", 0, 45, 50, 0xF0B988, true),
        ("shocked_loop_south", 0, 42, 30, 0xFAECB2, false),
        ("shocked_loop_south", 0, 45, 49, 0xFCD9B3, true),
        ("shocked_loop_south", 0, 40, 40, 0xFFFDD3, false),
        ("shocked_start_south", 0, 48, 48, 0x000000, false),
        ("shocked_start_south", 0, 37, 49, 0x163253, false),
        ("shocked_start_south", 0, 40, 46, 0x254397, false),
        ("shocked_start_south", 0, 39, 49, 0x3B5370, false),
        ("shocked_start_south", 0, 38, 46, 0x5078E7, false),
        ("shocked_start_south", 0, 42, 40, 0x762E21, false),
        ("shocked_start_south", 0, 46, 48, 0x762E21, true),
        ("shocked_start_south", 0, 41, 43, 0x91AEFF, false),
        ("shocked_start_south", 0, 45, 33, 0xB65932, false),
        ("shocked_start_south", 0, 36, 45, 0xBEBC91, false),
        ("shocked_start_south", 0, 35, 32, 0xCC9155, false),
        ("shocked_start_south", 0, 33, 46, 0xD37A57, true),
        ("shocked_start_south", 0, 43, 40, 0xEFCC86, false),
        ("shocked_start_south", 0, 40, 42, 0xF0B988, true),
        ("shocked_start_south", 0, 42, 33, 0xFAECB2, false),
        ("shocked_start_south", 0, 46, 47, 0xFCD9B3, true),
        ("shocked_start_south", 0, 40, 43, 0xFFFDD3, false),
        ("action_north", 0, 45, 41, 0x762E21, true),
        ("action_north", 5, 45, 41, 0x762E21, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("action_east", &[35, 37, 35, 37, 35, 35, 41]),
        ("action_north", &[20, 14, 14, 14, 14, 22, 22]),
        ("action_south", &[42, 42, 43, 42, 43, 42, 46]),
        ("shocked_end_south", &[55]),
        ("shocked_loop_south", &[54]),
        ("shocked_start_south", &[55]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [10, 11, 12, 3]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_nora_spring";
            let asset = format!("assets/animations/NPCs/Nora/Sprites/Spring/{prefix}_{case}.png");
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
                // Include raised and lowered hands while preserving the dark hair tips.
                // Exposed ankles change; blue shoes and mouth interiors stay original.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 41))
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Nora Spring material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin separately from changed pixels.
                if source
                    .iter()
                    .any(|c| rgba(*c) == p.0 && (*c != 0x762E21 || y >= 41))
                {
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
