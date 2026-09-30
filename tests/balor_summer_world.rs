use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/balor-autumn-standard-study and the local accepted Balor world baseline"]
fn balor_summer_world_covers_skin_and_preserves_outfit() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/balor-autumn-standard-study");
    let baseline =
        root.join("generated/characters-balor-valen-eiland-spring-specials-trial/characters/balor");
    let set = std::env::var_os("FOM_BALOR_SUMMER_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/balor-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/balor-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..144];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..144], prior);
    assert_eq!(candidate["regions"].as_array().unwrap().len(), 195);
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
    // Literal source landmarks cover every Summer frame, rolled cuffs, bare
    // ankles, forearms, wrist accessories, shirt, hair and belt/shoe shadows.
    // The four world shades are exclusively skin in these six strips.
    let landmarks = [
        ("idle_north", 0, 34, 44, 0xFCD9B3, true),
        ("idle_north", 0, 37, 51, 0xF0B988, true),
        ("idle_north", 0, 34, 43, 0xD37A57, true),
        ("idle_north", 0, 33, 46, 0x672115, true),
        ("idle_north", 0, 39, 28, 0x686589, false),
        ("idle_north", 0, 42, 45, 0x612026, false),
        ("idle_north", 0, 37, 46, 0x494288, false),
        ("idle_north", 0, 38, 46, 0x6960AD, false),
        ("idle_north", 0, 36, 41, 0xB395D6, false),
        ("idle_north", 0, 37, 41, 0xDEDAE9, false),
        ("idle_north", 0, 33, 44, 0x374050, false),
        ("idle_south", 0, 40, 35, 0xFCD9B3, true),
        ("idle_south", 0, 40, 34, 0xF0B988, true),
        ("idle_south", 0, 35, 36, 0xD37A57, true),
        ("idle_south", 0, 36, 37, 0x672115, true),
        ("idle_south", 0, 39, 28, 0x686589, false),
        ("idle_south", 0, 37, 45, 0x612026, false),
        ("idle_south", 0, 42, 46, 0x494288, false),
        ("idle_south", 0, 38, 46, 0x6960AD, false),
        ("idle_south", 0, 37, 40, 0xB395D6, false),
        ("idle_south", 0, 37, 41, 0xDEDAE9, false),
        ("idle_south", 0, 33, 44, 0x374050, false),
        ("idle_east", 0, 41, 35, 0xFCD9B3, true),
        ("idle_east", 0, 41, 34, 0xF0B988, true),
        ("idle_east", 0, 36, 36, 0xD37A57, true),
        ("idle_east", 0, 37, 37, 0x672115, true),
        ("idle_east", 0, 39, 28, 0x686589, false),
        ("idle_east", 0, 38, 45, 0x612026, false),
        ("idle_east", 0, 41, 47, 0x494288, false),
        ("idle_east", 0, 39, 46, 0x6960AD, false),
        ("idle_east", 0, 38, 40, 0xB395D6, false),
        ("idle_east", 0, 38, 41, 0xDEDAE9, false),
        ("idle_east", 0, 34, 44, 0x374050, false),
        ("walk_north", 0, 34, 44, 0xFCD9B3, true),
        ("walk_north", 0, 37, 51, 0xF0B988, true),
        ("walk_north", 0, 34, 43, 0xD37A57, true),
        ("walk_north", 0, 33, 46, 0x672115, true),
        ("walk_north", 0, 39, 28, 0x686589, false),
        ("walk_north", 0, 42, 45, 0x612026, false),
        ("walk_north", 0, 37, 46, 0x494288, false),
        ("walk_north", 0, 38, 46, 0x6960AD, false),
        ("walk_north", 0, 36, 41, 0xB395D6, false),
        ("walk_north", 0, 37, 41, 0xDEDAE9, false),
        ("walk_north", 0, 33, 44, 0x374050, false),
        ("walk_north", 1, 35, 44, 0xFCD9B3, true),
        ("walk_north", 1, 38, 50, 0xF0B988, true),
        ("walk_north", 1, 34, 44, 0xD37A57, true),
        ("walk_north", 1, 46, 46, 0x672115, true),
        ("walk_north", 1, 39, 29, 0x686589, false),
        ("walk_north", 1, 42, 46, 0x612026, false),
        ("walk_north", 1, 37, 47, 0x494288, false),
        ("walk_north", 1, 38, 47, 0x6960AD, false),
        ("walk_north", 1, 36, 42, 0xB395D6, false),
        ("walk_north", 1, 37, 42, 0xDEDAE9, false),
        ("walk_north", 1, 45, 44, 0x374050, false),
        ("walk_north", 2, 34, 44, 0xFCD9B3, true),
        ("walk_north", 2, 37, 51, 0xF0B988, true),
        ("walk_north", 2, 34, 43, 0xD37A57, true),
        ("walk_north", 2, 33, 46, 0x672115, true),
        ("walk_north", 2, 39, 28, 0x686589, false),
        ("walk_north", 2, 42, 45, 0x612026, false),
        ("walk_north", 2, 37, 46, 0x494288, false),
        ("walk_north", 2, 38, 46, 0x6960AD, false),
        ("walk_north", 2, 36, 41, 0xB395D6, false),
        ("walk_north", 2, 37, 41, 0xDEDAE9, false),
        ("walk_north", 2, 33, 44, 0x374050, false),
        ("walk_north", 3, 44, 44, 0xFCD9B3, true),
        ("walk_north", 3, 41, 50, 0xF0B988, true),
        ("walk_north", 3, 35, 44, 0xD37A57, true),
        ("walk_north", 3, 33, 46, 0x672115, true),
        ("walk_north", 3, 39, 29, 0x686589, false),
        ("walk_north", 3, 42, 46, 0x612026, false),
        ("walk_north", 3, 37, 47, 0x494288, false),
        ("walk_north", 3, 38, 47, 0x6960AD, false),
        ("walk_north", 3, 39, 42, 0xB395D6, false),
        ("walk_north", 3, 37, 42, 0xDEDAE9, false),
        ("walk_north", 3, 34, 44, 0x374050, false),
        ("walk_south", 0, 40, 35, 0xFCD9B3, true),
        ("walk_south", 0, 40, 34, 0xF0B988, true),
        ("walk_south", 0, 35, 36, 0xD37A57, true),
        ("walk_south", 0, 36, 37, 0x672115, true),
        ("walk_south", 0, 39, 28, 0x686589, false),
        ("walk_south", 0, 37, 45, 0x612026, false),
        ("walk_south", 0, 42, 46, 0x494288, false),
        ("walk_south", 0, 38, 46, 0x6960AD, false),
        ("walk_south", 0, 37, 40, 0xB395D6, false),
        ("walk_south", 0, 37, 41, 0xDEDAE9, false),
        ("walk_south", 0, 33, 44, 0x374050, false),
        ("walk_south", 1, 40, 36, 0xFCD9B3, true),
        ("walk_south", 1, 40, 35, 0xF0B988, true),
        ("walk_south", 1, 35, 37, 0xD37A57, true),
        ("walk_south", 1, 36, 38, 0x672115, true),
        ("walk_south", 1, 39, 29, 0x686589, false),
        ("walk_south", 1, 37, 46, 0x612026, false),
        ("walk_south", 1, 42, 47, 0x494288, false),
        ("walk_south", 1, 38, 47, 0x6960AD, false),
        ("walk_south", 1, 37, 41, 0xB395D6, false),
        ("walk_south", 1, 37, 42, 0xDEDAE9, false),
        ("walk_south", 1, 45, 44, 0x374050, false),
        ("walk_south", 2, 40, 35, 0xFCD9B3, true),
        ("walk_south", 2, 40, 34, 0xF0B988, true),
        ("walk_south", 2, 35, 36, 0xD37A57, true),
        ("walk_south", 2, 36, 37, 0x672115, true),
        ("walk_south", 2, 39, 28, 0x686589, false),
        ("walk_south", 2, 37, 45, 0x612026, false),
        ("walk_south", 2, 42, 46, 0x494288, false),
        ("walk_south", 2, 38, 46, 0x6960AD, false),
        ("walk_south", 2, 37, 40, 0xB395D6, false),
        ("walk_south", 2, 37, 41, 0xDEDAE9, false),
        ("walk_south", 2, 33, 44, 0x374050, false),
        ("walk_south", 3, 39, 36, 0xFCD9B3, true),
        ("walk_south", 3, 40, 35, 0xF0B988, true),
        ("walk_south", 3, 35, 37, 0xD37A57, true),
        ("walk_south", 3, 36, 38, 0x672115, true),
        ("walk_south", 3, 39, 29, 0x686589, false),
        ("walk_south", 3, 37, 46, 0x612026, false),
        ("walk_south", 3, 42, 47, 0x494288, false),
        ("walk_south", 3, 38, 47, 0x6960AD, false),
        ("walk_south", 3, 37, 41, 0xB395D6, false),
        ("walk_south", 3, 37, 42, 0xDEDAE9, false),
        ("walk_south", 3, 34, 44, 0x374050, false),
        ("walk_east", 0, 41, 35, 0xFCD9B3, true),
        ("walk_east", 0, 41, 34, 0xF0B988, true),
        ("walk_east", 0, 36, 36, 0xD37A57, true),
        ("walk_east", 0, 37, 37, 0x672115, true),
        ("walk_east", 0, 39, 28, 0x686589, false),
        ("walk_east", 0, 38, 45, 0x612026, false),
        ("walk_east", 0, 41, 47, 0x494288, false),
        ("walk_east", 0, 39, 46, 0x6960AD, false),
        ("walk_east", 0, 38, 40, 0xB395D6, false),
        ("walk_east", 0, 38, 41, 0xDEDAE9, false),
        ("walk_east", 0, 34, 44, 0x374050, false),
        ("walk_east", 1, 41, 36, 0xFCD9B3, true),
        ("walk_east", 1, 41, 35, 0xF0B988, true),
        ("walk_east", 1, 36, 37, 0xD37A57, true),
        ("walk_east", 1, 38, 39, 0x672115, true),
        ("walk_east", 1, 39, 29, 0x686589, false),
        ("walk_east", 1, 38, 46, 0x612026, false),
        ("walk_east", 1, 42, 48, 0x494288, false),
        ("walk_east", 1, 39, 47, 0x6960AD, false),
        ("walk_east", 1, 38, 41, 0xB395D6, false),
        ("walk_east", 1, 38, 42, 0xDEDAE9, false),
        ("walk_east", 1, 34, 44, 0x374050, false),
        ("walk_east", 2, 41, 35, 0xFCD9B3, true),
        ("walk_east", 2, 41, 34, 0xF0B988, true),
        ("walk_east", 2, 36, 36, 0xD37A57, true),
        ("walk_east", 2, 37, 37, 0x672115, true),
        ("walk_east", 2, 39, 28, 0x686589, false),
        ("walk_east", 2, 38, 45, 0x612026, false),
        ("walk_east", 2, 41, 47, 0x494288, false),
        ("walk_east", 2, 39, 46, 0x6960AD, false),
        ("walk_east", 2, 38, 40, 0xB395D6, false),
        ("walk_east", 2, 38, 41, 0xDEDAE9, false),
        ("walk_east", 2, 34, 44, 0x374050, false),
        ("walk_east", 3, 41, 36, 0xFCD9B3, true),
        ("walk_east", 3, 41, 35, 0xF0B988, true),
        ("walk_east", 3, 36, 37, 0xD37A57, true),
        ("walk_east", 3, 37, 38, 0x672115, true),
        ("walk_east", 3, 39, 29, 0x686589, false),
        ("walk_east", 3, 38, 48, 0x612026, false),
        ("walk_east", 3, 40, 48, 0x494288, false),
        ("walk_east", 3, 39, 47, 0x6960AD, false),
        ("walk_east", 3, 38, 41, 0xB395D6, false),
        ("walk_east", 3, 38, 42, 0xDEDAE9, false),
        ("walk_east", 3, 35, 45, 0x374050, false),
        ("idle_south", 0, 37, 50, 0xB395D6, false),
        ("idle_south", 0, 37, 51, 0xF0B988, true),
        ("idle_south", 0, 38, 51, 0xFCD9B3, true),
        ("idle_south", 0, 37, 52, 0x612026, false),
        ("idle_south", 0, 33, 44, 0x374050, false),
        ("idle_south", 0, 34, 44, 0xFCD9B3, true),
    ];
    let cases: [(&str, &[usize]); 6] = [
        ("idle_north", &[20]),
        ("idle_south", &[49]),
        ("idle_east", &[41]),
        ("walk_north", &[20, 17, 20, 17]),
        ("walk_south", &[49, 46, 49, 46]),
        ("walk_east", &[41, 42, 41, 45]),
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
                    "idle_north" | "idle_south" | "idle_east" => 7,
                    "walk_north" | "walk_south" => 28,
                    "walk_east" => 22,
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
