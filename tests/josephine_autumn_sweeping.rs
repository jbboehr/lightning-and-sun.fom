use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/josephine and the local accepted Josephine world baseline"]
fn josephine_autumn_sweeping_cover_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/josephine");
    let baseline = root.join("local/fixtures/slice-131-build/characters/josephine");
    let set = std::env::var_os("FOM_JOSEPHINE_AUTUMN_SWEEPING_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/josephine-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/josephine-world-trial.json")).unwrap(),
    )
    .unwrap();
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
    let source = [0x9B643A, 0x7E442F, 0x713222, 0x57271D];
    // These pinned sources use the four world shades exclusively for skin.
    // Face, hands and visible ankles change; hair, eyes, clothing, broom and
    // dust stay original. This oracle does not use the component seeds.
    let landmarks = [
        ("sweep_end_south", 0, 38, 33, 0x713222, true),
        ("sweep_end_south", 0, 39, 40, 0x713222, true),
        ("sweep_end_south", 0, 41, 44, 0x7E442F, true),
        ("sweep_end_south", 0, 37, 51, 0x7E442F, true),
        ("sweep_end_south", 1, 38, 33, 0x713222, true),
        ("sweep_end_south", 1, 39, 40, 0x713222, true),
        ("sweep_end_south", 1, 41, 41, 0x713222, true),
        ("sweep_end_south", 1, 37, 51, 0x7E442F, true),
        ("sweep_end_south", 2, 38, 34, 0x713222, true),
        ("sweep_end_south", 2, 37, 40, 0x57271D, true),
        ("sweep_end_south", 2, 41, 40, 0x713222, true),
        ("sweep_loop_south", 0, 38, 33, 0x713222, true),
        ("sweep_loop_south", 0, 39, 40, 0x7E442F, true),
        ("sweep_loop_south", 1, 38, 33, 0x713222, true),
        ("sweep_loop_south", 1, 39, 41, 0x7E442F, true),
        ("sweep_loop_south", 1, 41, 45, 0x9B643A, true),
        ("sweep_loop_south", 1, 37, 51, 0x7E442F, true),
        ("sweep_loop_south", 2, 38, 34, 0x713222, true),
        ("sweep_loop_south", 2, 39, 41, 0x7E442F, true),
        ("sweep_loop_south", 2, 41, 40, 0x713222, true),
        ("sweep_loop_south", 3, 38, 33, 0x713222, true),
        ("sweep_loop_south", 3, 39, 41, 0x7E442F, true),
        ("sweep_loop_south", 3, 41, 45, 0x9B643A, true),
        ("sweep_loop_south", 3, 37, 51, 0x7E442F, true),
        ("sweep_loop_south", 4, 38, 33, 0x713222, true),
        ("sweep_loop_south", 4, 39, 40, 0x7E442F, true),
        ("sweep_loop_south", 5, 38, 33, 0x713222, true),
        ("sweep_loop_south", 5, 39, 41, 0x7E442F, true),
        ("sweep_loop_south", 5, 41, 51, 0x9B643A, true),
        ("sweep_loop_south", 6, 38, 34, 0x713222, true),
        ("sweep_loop_south", 6, 37, 40, 0x57271D, true),
        ("sweep_loop_south", 7, 38, 33, 0x713222, true),
        ("sweep_loop_south", 7, 39, 41, 0x7E442F, true),
        ("sweep_loop_south", 7, 41, 51, 0x9B643A, true),
        ("sweep_loop_south", 8, 38, 33, 0x713222, true),
        ("sweep_loop_south", 8, 39, 40, 0x7E442F, true),
        ("sweep_start_south", 0, 38, 34, 0x713222, true),
        ("sweep_start_south", 0, 37, 40, 0x57271D, true),
        ("sweep_start_south", 0, 41, 40, 0x713222, true),
        ("sweep_start_south", 1, 38, 33, 0x713222, true),
        ("sweep_start_south", 1, 39, 40, 0x713222, true),
        ("sweep_start_south", 1, 41, 41, 0x713222, true),
        ("sweep_start_south", 1, 37, 51, 0x7E442F, true),
        ("sweep_start_south", 2, 38, 33, 0x713222, true),
        ("sweep_start_south", 2, 39, 40, 0x713222, true),
        ("sweep_start_south", 2, 41, 44, 0x7E442F, true),
        ("sweep_start_south", 2, 37, 51, 0x7E442F, true),
        ("sweep_loop_south", 2, 36, 29, 0x5F3737, false),
        ("sweep_loop_south", 2, 44, 42, 0xF5A95B, false),
        ("sweep_loop_south", 2, 35, 39, 0xFCD288, false),
        ("sweep_loop_south", 2, 37, 42, 0xD8753A, false),
        ("sweep_loop_south", 2, 44, 50, 0x653D25, false),
        ("sweep_loop_south", 2, 38, 40, 0x936244, false),
        ("sweep_loop_south", 2, 38, 39, 0xBB8151, false),
        ("sweep_loop_south", 2, 48, 51, 0xE9B03A, false),
        ("sweep_loop_south", 2, 49, 50, 0xFFE36F, false),
        ("sweep_loop_south", 2, 53, 49, 0xA0A0A0, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("sweep_end_south", &[58, 53, 48]),
        ("sweep_loop_south", &[51, 57, 53, 57, 51, 57, 53, 57, 51]),
        ("sweep_start_south", &[48, 53, 58]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    for preset in presets["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let target: Vec<_> = [0, 9, 10, 11]
            .iter()
            .map(|i| &preset["colors"][*i])
            .map(|c| u32::from_str_radix(&c.as_str().unwrap()[1..7], 16).unwrap())
            .collect();
        let variant = output.join("variants").join(id);
        for &(case, expected_per_frame) in cases {
            let frames = expected_per_frame.len() as u32;
            let prefix = "spr_npc_josephine_specialanimation_autumn";
            let asset =
                format!("assets/animations/NPCs/Josephine/Sprites/Autumn/{prefix}_{case}.png");
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
                // Exposed ankles below the hem are skin too; shoes stay original.
                let expected = source
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .map_or(p.0, |i| rgba(target[i]));
                assert_eq!(
                    q.0, expected,
                    "Josephine Autumn material mismatch: {id} {case} [{x},{y}]"
                );
                // Count selected skin independently of the target colors.
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
