use serde_json::Value;
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/test-corpus/dell and the local accepted Dell world baseline"]
fn dell_summer_world_covers_skin_and_preserves_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/dell");
    let baseline = root.join("local/fixtures/slice-158-build/characters/dell");
    let set = std::env::var_os("FOM_DELL_SUMMER_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/dell-world-trial.json"));
    let presets: Value = serde_json::from_slice(&fs::read(&set).unwrap()).unwrap();
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/dell-world-trial.json")).unwrap(),
    )
    .unwrap();
    let prior = &profile["regions"].as_array().unwrap()[..65];
    let candidate_path = set
        .parent()
        .unwrap()
        .join(presets["profile"].as_str().unwrap());
    let candidate: Value = serde_json::from_slice(&fs::read(candidate_path).unwrap()).unwrap();
    assert_eq!(&candidate["regions"].as_array().unwrap()[..65], prior);
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
    // Face, arms and exposed legs change. Shared-color blonde hair, eyes,
    // ribbon, blue shirt, brown overalls, pale rolled cuffs and shoes stay original.
    let landmarks = [
        ("idle_east", 0, 40, 39, 0xECC889, true),
        ("idle_east", 0, 36, 45, 0x762E21, true),
        ("idle_east", 0, 44, 46, 0xB65932, true),
        ("idle_east", 0, 38, 50, 0xB65932, true),
        ("idle_east", 0, 37, 32, 0xDF8D4B, false),
        ("idle_east", 0, 39, 34, 0xB65932, false),
        ("idle_east", 0, 39, 36, 0x762E21, false),
        ("idle_east", 0, 34, 32, 0xFFDC90, false),
        ("idle_east", 0, 41, 34, 0xF5F5F5, false),
        ("idle_east", 0, 38, 44, 0x804637, false),
        ("idle_east", 0, 39, 44, 0xB17557, false),
        ("idle_east", 0, 38, 49, 0xD8A474, false),
        ("idle_east", 0, 37, 44, 0x6D91AF, false),
        ("idle_east", 0, 40, 44, 0xB5DEF2, false),
        ("idle_east", 0, 39, 53, 0xE16667, false),
        ("idle_north", 0, 35, 45, 0x762E21, true),
        ("idle_north", 0, 44, 45, 0x762E21, true),
        ("idle_north", 0, 37, 50, 0xDF8D4B, true),
        ("idle_north", 0, 37, 32, 0xDF8D4B, false),
        ("idle_north", 0, 33, 38, 0xB65932, false),
        ("idle_north", 0, 38, 32, 0xFFDC90, false),
        ("idle_north", 0, 36, 34, 0xF5F5F5, false),
        ("idle_north", 0, 37, 44, 0x804637, false),
        ("idle_north", 0, 38, 47, 0xB17557, false),
        ("idle_north", 0, 37, 49, 0xD8A474, false),
        ("idle_north", 0, 36, 44, 0x6D91AF, false),
        ("idle_north", 0, 38, 44, 0xB5DEF2, false),
        ("idle_north", 0, 38, 52, 0xE16667, false),
        ("idle_south", 0, 39, 39, 0xECC889, true),
        ("idle_south", 0, 35, 45, 0x762E21, true),
        ("idle_south", 0, 44, 45, 0x762E21, true),
        ("idle_south", 0, 37, 50, 0xDF8D4B, true),
        ("idle_south", 0, 35, 33, 0xDF8D4B, false),
        ("idle_south", 0, 38, 34, 0xB65932, false),
        ("idle_south", 0, 38, 36, 0x762E21, false),
        ("idle_south", 0, 34, 32, 0xFFDC90, false),
        ("idle_south", 0, 40, 34, 0xF5F5F5, false),
        ("idle_south", 0, 37, 44, 0x804637, false),
        ("idle_south", 0, 38, 44, 0xB17557, false),
        ("idle_south", 0, 37, 49, 0xD8A474, false),
        ("idle_south", 0, 36, 44, 0x6D91AF, false),
        ("idle_south", 0, 39, 44, 0xB5DEF2, false),
        ("idle_south", 0, 37, 52, 0xC44748, false),
        ("walk_east", 0, 40, 39, 0xECC889, true),
        ("walk_east", 0, 36, 45, 0x762E21, true),
        ("walk_east", 0, 44, 46, 0xB65932, true),
        ("walk_east", 0, 38, 50, 0xB65932, true),
        ("walk_east", 0, 37, 32, 0xDF8D4B, false),
        ("walk_east", 0, 39, 34, 0xB65932, false),
        ("walk_east", 0, 39, 36, 0x762E21, false),
        ("walk_east", 0, 34, 32, 0xFFDC90, false),
        ("walk_east", 0, 41, 34, 0xF5F5F5, false),
        ("walk_east", 0, 38, 44, 0x804637, false),
        ("walk_east", 0, 39, 44, 0xB17557, false),
        ("walk_east", 0, 38, 49, 0xD8A474, false),
        ("walk_east", 0, 37, 44, 0x6D91AF, false),
        ("walk_east", 0, 40, 44, 0xB5DEF2, false),
        ("walk_east", 0, 39, 53, 0xE16667, false),
        ("walk_east", 1, 40, 40, 0xECC889, true),
        ("walk_east", 1, 35, 46, 0xB65932, true),
        ("walk_east", 1, 44, 47, 0xDF8D4B, true),
        ("walk_east", 1, 39, 51, 0xDF8D4B, true),
        ("walk_east", 1, 37, 33, 0xDF8D4B, false),
        ("walk_east", 1, 39, 35, 0xB65932, false),
        ("walk_east", 1, 39, 37, 0x762E21, false),
        ("walk_east", 1, 34, 33, 0xFFDC90, false),
        ("walk_east", 1, 41, 35, 0xF5F5F5, false),
        ("walk_east", 1, 38, 45, 0x804637, false),
        ("walk_east", 1, 39, 45, 0xB17557, false),
        ("walk_east", 1, 38, 50, 0xD8A474, false),
        ("walk_east", 1, 37, 45, 0x6D91AF, false),
        ("walk_east", 1, 40, 45, 0xB5DEF2, false),
        ("walk_east", 1, 42, 52, 0xE16667, false),
        ("walk_east", 2, 40, 39, 0xECC889, true),
        ("walk_east", 2, 36, 45, 0x762E21, true),
        ("walk_east", 2, 44, 46, 0xB65932, true),
        ("walk_east", 2, 38, 50, 0xB65932, true),
        ("walk_east", 2, 37, 32, 0xDF8D4B, false),
        ("walk_east", 2, 39, 34, 0xB65932, false),
        ("walk_east", 2, 39, 36, 0x762E21, false),
        ("walk_east", 2, 34, 32, 0xFFDC90, false),
        ("walk_east", 2, 41, 34, 0xF5F5F5, false),
        ("walk_east", 2, 38, 44, 0x804637, false),
        ("walk_east", 2, 39, 44, 0xB17557, false),
        ("walk_east", 2, 38, 49, 0xD8A474, false),
        ("walk_east", 2, 37, 44, 0x6D91AF, false),
        ("walk_east", 2, 40, 44, 0xB5DEF2, false),
        ("walk_east", 2, 39, 53, 0xE16667, false),
        ("walk_east", 3, 40, 40, 0xECC889, true),
        ("walk_east", 3, 36, 47, 0xB65932, true),
        ("walk_east", 3, 44, 48, 0xDF8D4B, true),
        ("walk_east", 3, 38, 51, 0xECC889, true),
        ("walk_east", 3, 37, 33, 0xDF8D4B, false),
        ("walk_east", 3, 39, 35, 0xB65932, false),
        ("walk_east", 3, 39, 37, 0x762E21, false),
        ("walk_east", 3, 34, 33, 0xFFDC90, false),
        ("walk_east", 3, 41, 35, 0xF5F5F5, false),
        ("walk_east", 3, 38, 45, 0x804637, false),
        ("walk_east", 3, 39, 45, 0xB17557, false),
        ("walk_east", 3, 38, 50, 0xD8A474, false),
        ("walk_east", 3, 37, 45, 0x6D91AF, false),
        ("walk_east", 3, 40, 45, 0xB5DEF2, false),
        ("walk_east", 3, 38, 52, 0xE16667, false),
        ("walk_north", 0, 35, 45, 0x762E21, true),
        ("walk_north", 0, 44, 45, 0x762E21, true),
        ("walk_north", 0, 37, 50, 0xDF8D4B, true),
        ("walk_north", 0, 37, 32, 0xDF8D4B, false),
        ("walk_north", 0, 33, 38, 0xB65932, false),
        ("walk_north", 0, 38, 32, 0xFFDC90, false),
        ("walk_north", 0, 36, 34, 0xF5F5F5, false),
        ("walk_north", 0, 37, 44, 0x804637, false),
        ("walk_north", 0, 38, 47, 0xB17557, false),
        ("walk_north", 0, 37, 49, 0xD8A474, false),
        ("walk_north", 0, 36, 44, 0x6D91AF, false),
        ("walk_north", 0, 38, 44, 0xB5DEF2, false),
        ("walk_north", 0, 38, 52, 0xE16667, false),
        ("walk_north", 1, 36, 46, 0xB65932, true),
        ("walk_north", 1, 44, 47, 0xB65932, true),
        ("walk_north", 1, 41, 51, 0xECC889, true),
        ("walk_north", 1, 37, 33, 0xDF8D4B, false),
        ("walk_north", 1, 33, 39, 0xB65932, false),
        ("walk_north", 1, 38, 33, 0xFFDC90, false),
        ("walk_north", 1, 36, 35, 0xF5F5F5, false),
        ("walk_north", 1, 37, 45, 0x804637, false),
        ("walk_north", 1, 38, 48, 0xB17557, false),
        ("walk_north", 1, 38, 50, 0xD8A474, false),
        ("walk_north", 1, 38, 45, 0x6D91AF, false),
        ("walk_north", 1, 39, 45, 0xB5DEF2, false),
        ("walk_north", 1, 41, 53, 0xE16667, false),
        ("walk_north", 2, 35, 45, 0x762E21, true),
        ("walk_north", 2, 44, 45, 0x762E21, true),
        ("walk_north", 2, 37, 50, 0xDF8D4B, true),
        ("walk_north", 2, 37, 32, 0xDF8D4B, false),
        ("walk_north", 2, 33, 38, 0xB65932, false),
        ("walk_north", 2, 38, 32, 0xFFDC90, false),
        ("walk_north", 2, 36, 34, 0xF5F5F5, false),
        ("walk_north", 2, 37, 44, 0x804637, false),
        ("walk_north", 2, 38, 47, 0xB17557, false),
        ("walk_north", 2, 37, 49, 0xD8A474, false),
        ("walk_north", 2, 36, 44, 0x6D91AF, false),
        ("walk_north", 2, 38, 44, 0xB5DEF2, false),
        ("walk_north", 2, 38, 52, 0xE16667, false),
        ("walk_north", 3, 35, 47, 0xB65932, true),
        ("walk_north", 3, 43, 46, 0xB65932, true),
        ("walk_north", 3, 37, 51, 0xDF8D4B, true),
        ("walk_north", 3, 37, 33, 0xDF8D4B, false),
        ("walk_north", 3, 33, 39, 0xB65932, false),
        ("walk_north", 3, 38, 33, 0xFFDC90, false),
        ("walk_north", 3, 36, 35, 0xF5F5F5, false),
        ("walk_north", 3, 37, 45, 0x804637, false),
        ("walk_north", 3, 38, 48, 0xB17557, false),
        ("walk_north", 3, 37, 50, 0xD8A474, false),
        ("walk_north", 3, 41, 45, 0x6D91AF, false),
        ("walk_north", 3, 38, 45, 0xB5DEF2, false),
        ("walk_north", 3, 38, 53, 0xE16667, false),
        ("walk_south", 0, 39, 39, 0xECC889, true),
        ("walk_south", 0, 35, 45, 0x762E21, true),
        ("walk_south", 0, 44, 45, 0x762E21, true),
        ("walk_south", 0, 37, 50, 0xDF8D4B, true),
        ("walk_south", 0, 35, 33, 0xDF8D4B, false),
        ("walk_south", 0, 38, 34, 0xB65932, false),
        ("walk_south", 0, 38, 36, 0x762E21, false),
        ("walk_south", 0, 34, 32, 0xFFDC90, false),
        ("walk_south", 0, 40, 34, 0xF5F5F5, false),
        ("walk_south", 0, 37, 44, 0x804637, false),
        ("walk_south", 0, 38, 44, 0xB17557, false),
        ("walk_south", 0, 37, 49, 0xD8A474, false),
        ("walk_south", 0, 36, 44, 0x6D91AF, false),
        ("walk_south", 0, 39, 44, 0xB5DEF2, false),
        ("walk_south", 0, 37, 52, 0xC44748, false),
        ("walk_south", 1, 39, 40, 0xECC889, true),
        ("walk_south", 1, 36, 46, 0xB65932, true),
        ("walk_south", 1, 44, 47, 0xB65932, true),
        ("walk_south", 1, 41, 51, 0xECC889, true),
        ("walk_south", 1, 35, 34, 0xDF8D4B, false),
        ("walk_south", 1, 38, 35, 0xB65932, false),
        ("walk_south", 1, 38, 37, 0x762E21, false),
        ("walk_south", 1, 34, 33, 0xFFDC90, false),
        ("walk_south", 1, 40, 35, 0xF5F5F5, false),
        ("walk_south", 1, 42, 45, 0x804637, false),
        ("walk_south", 1, 38, 45, 0xB17557, false),
        ("walk_south", 1, 38, 50, 0xD8A474, false),
        ("walk_south", 1, 37, 45, 0x6D91AF, false),
        ("walk_south", 1, 39, 45, 0xB5DEF2, false),
        ("walk_south", 1, 37, 52, 0xC44748, false),
        ("walk_south", 2, 39, 39, 0xECC889, true),
        ("walk_south", 2, 35, 45, 0x762E21, true),
        ("walk_south", 2, 44, 45, 0x762E21, true),
        ("walk_south", 2, 37, 50, 0xDF8D4B, true),
        ("walk_south", 2, 35, 33, 0xDF8D4B, false),
        ("walk_south", 2, 38, 34, 0xB65932, false),
        ("walk_south", 2, 38, 36, 0x762E21, false),
        ("walk_south", 2, 34, 32, 0xFFDC90, false),
        ("walk_south", 2, 40, 34, 0xF5F5F5, false),
        ("walk_south", 2, 37, 44, 0x804637, false),
        ("walk_south", 2, 38, 44, 0xB17557, false),
        ("walk_south", 2, 37, 49, 0xD8A474, false),
        ("walk_south", 2, 36, 44, 0x6D91AF, false),
        ("walk_south", 2, 39, 44, 0xB5DEF2, false),
        ("walk_south", 2, 37, 52, 0xC44748, false),
        ("walk_south", 3, 39, 40, 0xECC889, true),
        ("walk_south", 3, 35, 47, 0xB65932, true),
        ("walk_south", 3, 43, 46, 0xB65932, true),
        ("walk_south", 3, 37, 51, 0xDF8D4B, true),
        ("walk_south", 3, 35, 34, 0xDF8D4B, false),
        ("walk_south", 3, 38, 35, 0xB65932, false),
        ("walk_south", 3, 38, 37, 0x762E21, false),
        ("walk_south", 3, 34, 33, 0xFFDC90, false),
        ("walk_south", 3, 40, 35, 0xF5F5F5, false),
        ("walk_south", 3, 37, 45, 0x804637, false),
        ("walk_south", 3, 38, 45, 0xB17557, false),
        ("walk_south", 3, 37, 50, 0xD8A474, false),
        ("walk_south", 3, 42, 45, 0x6D91AF, false),
        ("walk_south", 3, 39, 45, 0xB5DEF2, false),
        ("walk_south", 3, 41, 52, 0xC44748, false),
        ("idle_south", 0, 39, 39, 0xECC889, true),
        ("idle_south", 0, 38, 41, 0xDF8D4B, true),
        ("idle_south", 0, 36, 39, 0xB65932, true),
        ("idle_south", 0, 36, 41, 0x762E21, true),
        ("idle_south", 0, 37, 39, 0xECF0E9, false),
        ("idle_south", 0, 37, 38, 0xC2B9BE, false),
        ("idle_east", 0, 36, 45, 0x762E21, true),
        ("idle_north", 0, 35, 45, 0x762E21, true),
        ("idle_south", 0, 35, 45, 0x762E21, true),
        ("walk_east", 0, 36, 45, 0x762E21, true),
        ("walk_east", 2, 36, 45, 0x762E21, true),
        ("walk_north", 0, 35, 45, 0x762E21, true),
        ("walk_north", 2, 35, 45, 0x762E21, true),
        ("walk_south", 0, 35, 45, 0x762E21, true),
        ("walk_south", 2, 35, 45, 0x762E21, true),
    ];
    let cases: &[(&str, &[usize])] = &[
        ("idle_east", &[47]),
        ("idle_north", &[22]),
        ("idle_south", &[52]),
        ("walk_east", &[47, 42, 47, 38]),
        ("walk_north", &[22, 15, 22, 15]),
        ("walk_south", &[52, 45, 52, 45]),
    ];
    // Reviewed original face rows, independent of the component seeds.
    // The Summer sleeves expose the dark upper-arm edge on row 45.
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
        if y >= 45 {
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
            let prefix = "spr_npc_dell_summer";
            let asset = format!("assets/animations/NPCs/Dell/Sprites/Summer/{prefix}_{case}.png");
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
                    "Dell Summer material mismatch: {id} {case} [{x},{y}]"
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
