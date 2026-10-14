use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/dell and the local accepted Dell world baseline"]
fn dell_spring_actions_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/dell");
    let baseline = root.join("local/fixtures/slice-153-build/characters/dell");
    let set = std::env::var_os("FOM_DELL_SPRING_ACTIONS_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/dell-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/dell-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..38];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..38], prior);
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
    // Face, eyelids, hands and exposed ankles follow the palette. Three dark
    // temple/fringe pixels connected to the closed East eye follow its edge.
    // Blonde hair, lashes, eye details, ribbon, overalls and boots stay original
    // elsewhere. These landmarks and the pixel oracle do not use profile seeds.
    let landmarks = [
        ("blink_east", 0, 40, 39, 0xECC889, true),
        ("blink_east", 0, 40, 44, 0xDF8D4B, true),
        ("blink_east", 0, 35, 46, 0x762E21, true),
        ("blink_east", 0, 45, 46, 0x762E21, true),
        ("blink_east", 0, 38, 51, 0xB65932, true),
        ("blink_east", 0, 37, 32, 0xDF8D4B, false),
        ("blink_east", 0, 38, 34, 0xB65932, false),
        ("blink_east", 0, 38, 35, 0x762E21, false),
        ("blink_east", 0, 34, 32, 0xFFDC90, false),
        ("blink_east", 0, 38, 33, 0xF5F5F5, false),
        ("blink_east", 0, 38, 45, 0x382A61, false),
        ("blink_east", 0, 38, 44, 0x616BBE, false),
        ("blink_east", 0, 38, 49, 0x7FB0E4, false),
        ("blink_east", 0, 39, 44, 0xF96F8C, false),
        ("blink_east", 0, 38, 52, 0xA1121D, false),
        ("blink_east", 0, 38, 50, 0x391E22, false),
        ("blink_east", 1, 39, 39, 0xECC889, true),
        ("blink_east", 1, 40, 44, 0xDF8D4B, true),
        ("blink_east", 1, 35, 46, 0x762E21, true),
        ("blink_east", 1, 45, 46, 0x762E21, true),
        ("blink_east", 1, 38, 51, 0xB65932, true),
        ("blink_east", 1, 37, 32, 0xDF8D4B, false),
        ("blink_east", 1, 38, 34, 0xB65932, false),
        ("blink_east", 1, 38, 35, 0x762E21, false),
        ("blink_east", 1, 34, 32, 0xFFDC90, false),
        ("blink_east", 1, 38, 33, 0xF5F5F5, false),
        ("blink_east", 1, 38, 45, 0x382A61, false),
        ("blink_east", 1, 38, 44, 0x616BBE, false),
        ("blink_east", 1, 38, 49, 0x7FB0E4, false),
        ("blink_east", 1, 39, 44, 0xF96F8C, false),
        ("blink_east", 1, 38, 52, 0xA1121D, false),
        ("blink_east", 1, 38, 50, 0x391E22, false),
        ("blink_east", 2, 40, 39, 0xECC889, true),
        ("blink_east", 2, 40, 44, 0xDF8D4B, true),
        ("blink_east", 2, 35, 46, 0x762E21, true),
        ("blink_east", 2, 45, 46, 0x762E21, true),
        ("blink_east", 2, 38, 51, 0xB65932, true),
        ("blink_east", 2, 37, 32, 0xDF8D4B, false),
        ("blink_east", 2, 38, 34, 0xB65932, false),
        ("blink_east", 2, 38, 35, 0x762E21, false),
        ("blink_east", 2, 34, 32, 0xFFDC90, false),
        ("blink_east", 2, 38, 33, 0xF5F5F5, false),
        ("blink_east", 2, 38, 45, 0x382A61, false),
        ("blink_east", 2, 38, 44, 0x616BBE, false),
        ("blink_east", 2, 38, 49, 0x7FB0E4, false),
        ("blink_east", 2, 39, 44, 0xF96F8C, false),
        ("blink_east", 2, 38, 52, 0xA1121D, false),
        ("blink_east", 2, 38, 50, 0x391E22, false),
        ("blink_south", 0, 39, 39, 0xECC889, true),
        ("blink_south", 0, 39, 44, 0xDF8D4B, true),
        ("blink_south", 0, 34, 46, 0x762E21, true),
        ("blink_south", 0, 45, 46, 0x762E21, true),
        ("blink_south", 0, 37, 51, 0xDF8D4B, true),
        ("blink_south", 0, 37, 32, 0xDF8D4B, false),
        ("blink_south", 0, 38, 34, 0xB65932, false),
        ("blink_south", 0, 38, 35, 0x762E21, false),
        ("blink_south", 0, 34, 32, 0xFFDC90, false),
        ("blink_south", 0, 38, 33, 0xF5F5F5, false),
        ("blink_south", 0, 36, 44, 0x382A61, false),
        ("blink_south", 0, 37, 44, 0x616BBE, false),
        ("blink_south", 0, 37, 49, 0x7FB0E4, false),
        ("blink_south", 0, 38, 44, 0xF96F8C, false),
        ("blink_south", 0, 37, 52, 0xA1121D, false),
        ("blink_south", 0, 37, 50, 0x391E22, false),
        ("blink_south", 1, 39, 39, 0xECC889, true),
        ("blink_south", 1, 39, 44, 0xDF8D4B, true),
        ("blink_south", 1, 34, 46, 0x762E21, true),
        ("blink_south", 1, 45, 46, 0x762E21, true),
        ("blink_south", 1, 37, 51, 0xDF8D4B, true),
        ("blink_south", 1, 37, 32, 0xDF8D4B, false),
        ("blink_south", 1, 38, 34, 0xB65932, false),
        ("blink_south", 1, 38, 35, 0x762E21, false),
        ("blink_south", 1, 34, 32, 0xFFDC90, false),
        ("blink_south", 1, 38, 33, 0xF5F5F5, false),
        ("blink_south", 1, 36, 44, 0x382A61, false),
        ("blink_south", 1, 37, 44, 0x616BBE, false),
        ("blink_south", 1, 37, 49, 0x7FB0E4, false),
        ("blink_south", 1, 38, 44, 0xF96F8C, false),
        ("blink_south", 1, 37, 52, 0xA1121D, false),
        ("blink_south", 1, 37, 50, 0x391E22, false),
        ("blink_south", 2, 39, 39, 0xECC889, true),
        ("blink_south", 2, 39, 44, 0xDF8D4B, true),
        ("blink_south", 2, 34, 46, 0x762E21, true),
        ("blink_south", 2, 45, 46, 0x762E21, true),
        ("blink_south", 2, 37, 51, 0xDF8D4B, true),
        ("blink_south", 2, 37, 32, 0xDF8D4B, false),
        ("blink_south", 2, 38, 34, 0xB65932, false),
        ("blink_south", 2, 38, 35, 0x762E21, false),
        ("blink_south", 2, 34, 32, 0xFFDC90, false),
        ("blink_south", 2, 38, 33, 0xF5F5F5, false),
        ("blink_south", 2, 36, 44, 0x382A61, false),
        ("blink_south", 2, 37, 44, 0x616BBE, false),
        ("blink_south", 2, 37, 49, 0x7FB0E4, false),
        ("blink_south", 2, 38, 44, 0xF96F8C, false),
        ("blink_south", 2, 37, 52, 0xA1121D, false),
        ("blink_south", 2, 37, 50, 0x391E22, false),
        ("sit_east", 0, 40, 36, 0xECC889, true),
        ("sit_east", 0, 40, 41, 0xDF8D4B, true),
        ("sit_east", 0, 36, 43, 0xDF8D4B, true),
        ("sit_east", 0, 44, 45, 0xECC889, true),
        ("sit_east", 0, 35, 45, 0xECC889, true),
        ("sit_east", 0, 37, 29, 0xDF8D4B, false),
        ("sit_east", 0, 38, 31, 0xB65932, false),
        ("sit_east", 0, 38, 32, 0x762E21, false),
        ("sit_east", 0, 34, 29, 0xFFDC90, false),
        ("sit_east", 0, 38, 30, 0xF5F5F5, false),
        ("sit_east", 0, 38, 42, 0x382A61, false),
        ("sit_east", 0, 38, 41, 0x616BBE, false),
        ("sit_east", 0, 43, 44, 0x7FB0E4, false),
        ("sit_east", 0, 39, 41, 0xF96F8C, false),
        ("sit_east", 0, 41, 47, 0xA1121D, false),
        ("sit_east", 0, 44, 44, 0x391E22, false),
        ("sit_north", 0, 35, 43, 0xB65932, true),
        ("sit_north", 0, 44, 43, 0xB65932, true),
        ("sit_north", 0, 37, 29, 0xDF8D4B, false),
        ("sit_north", 0, 33, 35, 0xB65932, false),
        ("sit_north", 0, 38, 29, 0xFFDC90, false),
        ("sit_north", 0, 36, 31, 0xF5F5F5, false),
        ("sit_north", 0, 37, 42, 0x382A61, false),
        ("sit_north", 0, 37, 41, 0x616BBE, false),
        ("sit_north", 0, 38, 41, 0xF96F8C, false),
        ("sit_south", 0, 39, 36, 0xECC889, true),
        ("sit_south", 0, 39, 41, 0xDF8D4B, true),
        ("sit_south", 0, 34, 44, 0xDF8D4B, true),
        ("sit_south", 0, 44, 44, 0xDF8D4B, true),
        ("sit_south", 0, 35, 45, 0xDF8D4B, true),
        ("sit_south", 0, 37, 29, 0xDF8D4B, false),
        ("sit_south", 0, 38, 31, 0xB65932, false),
        ("sit_south", 0, 38, 32, 0x762E21, false),
        ("sit_south", 0, 34, 29, 0xFFDC90, false),
        ("sit_south", 0, 38, 30, 0xF5F5F5, false),
        ("sit_south", 0, 37, 42, 0x382A61, false),
        ("sit_south", 0, 37, 41, 0x616BBE, false),
        ("sit_south", 0, 37, 46, 0x7FB0E4, false),
        ("sit_south", 0, 38, 41, 0xF96F8C, false),
        ("sit_south", 0, 37, 49, 0xA1121D, false),
        ("sit_south", 0, 37, 47, 0x391E22, false),
        ("blink_east", 1, 38, 38, 0xECC889, true),
        ("blink_south", 1, 37, 38, 0xECC889, true),
        ("blink_east", 1, 38, 40, 0x000000, false),
        ("blink_south", 1, 37, 40, 0x000000, false),
        ("blink_east", 1, 44, 36, 0x762E21, true),
        ("blink_east", 1, 43, 37, 0x762E21, true),
        ("blink_east", 1, 44, 37, 0x762E21, true),
        ("blink_east", 1, 44, 38, 0x762E21, true),
        ("blink_east", 1, 44, 39, 0x762E21, true),
        ("blink_east", 1, 44, 40, 0x762E21, true),
        ("blink_east", 1, 45, 38, 0xDF8D4B, false),
        ("blink_east", 1, 46, 38, 0xFFDC90, false),
        ("blink_south", 0, 37, 40, 0xECF0E9, false),
        ("blink_south", 0, 37, 39, 0xC2B9BE, false),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("blink_east", &[42, 55, 42]),
        ("blink_south", &[46, 56, 46]),
        ("sit_east", &[39]),
        ("sit_north", &[16]),
        ("sit_south", &[50]),
    ];
    // Coordinates come from the reviewed original grids, independently of seeds.
    // Face rows give [y, x_start, x_end_exclusive]; sitting shifts the face up three.
    let east_face = [
        (38, 40, 42),
        (39, 36, 45),
        (40, 36, 45),
        (41, 37, 44),
        (42, 38, 43),
        (43, 40, 42),
        (44, 40, 42),
    ];
    let south_face = [
        (38, 39, 41),
        (39, 35, 45),
        (40, 35, 45),
        (41, 36, 44),
        (42, 37, 43),
        (43, 39, 41),
        (44, 39, 41),
    ];
    let skin_at = |case: &str, x: u32, y: u32| {
        let frame = x / 80;
        let local_x = x % 80;
        let shifted_y = y + if case.starts_with("sit") { 3 } else { 0 };
        if shifted_y >= 46 {
            return true;
        }
        if case.ends_with("north") {
            return false;
        }
        if case.starts_with("blink") && frame == 1 {
            if case.ends_with("east")
                && ((y == 36 && local_x == 44) || (y == 37 && (43..45).contains(&local_x)))
            {
                return true;
            }
            if y == 38 {
                return if case.ends_with("east") {
                    (37..45).contains(&local_x)
                } else {
                    (36..44).contains(&local_x)
                };
            }
        }
        let rows = if case.ends_with("east") {
            &east_face
        } else {
            &south_face
        };
        rows.iter()
            .any(|&(yy, lo, hi)| shifted_y == yy && (lo..hi).contains(&(x % 80)))
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
