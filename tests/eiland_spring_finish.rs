use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn asset(name: &str) -> String {
    let prefix = "specialanimation_spring";
    format!("assets/animations/NPCs/Eiland/Sprites/Spring/spr_npc_eiland_{prefix}_{name}.png")
}
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}
fn color(value: &Value) -> u32 {
    u32::from_str_radix(&value.as_str().unwrap()[1..7], 16).unwrap()
}

#[test]
#[ignore = "requires 162 local animations in extracted/eiland-summer-magnify-study and the accepted earlier output baseline"]
fn eiland_spring_finish_cover_moving_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/eiland-summer-magnify-study");
    let baseline = root.join(
        "generated/characters-balor-summer-valen-heal-eiland-magnify-trial/characters/eiland",
    );
    let profile_path = std::env::var_os("FOM_EILAND_SPRING_FINISH_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let set_path = std::env::var_os("FOM_EILAND_SPRING_FINISH_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let profile = read(&profile_path);
    assert_eq!(profile["regions"].as_array().unwrap().len(), 162);
    assert_eq!(profile["source_colors"].as_array().unwrap().len(), 12);
    assert_eq!(profile["color_groups"].as_array().unwrap().len(), 7);
    let set = read(&set_path);
    let temp = tempfile::tempdir().unwrap();
    // Literal source-art landmarks distinguish moving fingers and fine face shading
    // from gold trim, cape hem, rose, tool heads, shafts, bristles and trails.
    let landmarks = [
        ("axe_east", 0, 39, 33, 0x9C5241, true),
        ("axe_east", 0, 38, 34, 0xBA6A4C, true),
        ("axe_east", 0, 38, 35, 0xE9A980, true),
        ("axe_east", 0, 42, 36, 0xBA6A4C, true),
        ("axe_east", 0, 41, 25, 0xC4633D, false),
        ("axe_east", 0, 42, 25, 0xE9A482, false),
        ("axe_east", 0, 43, 24, 0x623B31, false),
        ("axe_east", 0, 38, 45, 0xBA6A4C, false),
        ("axe_east", 0, 43, 46, 0xBA6A4C, false),
        ("axe_east", 1, 53, 44, 0xDE8F5D, true),
        ("axe_east", 1, 54, 45, 0xDE8F5D, true),
        ("axe_east", 1, 50, 46, 0xE9A980, true),
        ("axe_east", 1, 48, 42, 0xDE8F5D, true),
        ("axe_east", 1, 57, 47, 0x000000, false),
        ("axe_east", 1, 60, 48, 0xC4633D, false),
        ("axe_east", 1, 42, 42, 0xBA6A4C, false),
        ("axe_east", 1, 43, 46, 0xBA6A4C, false),
        ("axe_east", 1, 72, 51, 0xFFFFFF, false),
        ("axe_east", 4, 46, 47, 0xBA6A4C, true),
        ("axe_east", 4, 47, 47, 0x7D3B14, true),
        ("axe_east", 4, 48, 47, 0xBA6A4C, true),
        ("pickaxe_east", 1, 53, 44, 0xDE8F5D, true),
        ("pickaxe_east", 1, 54, 45, 0xDE8F5D, true),
        ("pickaxe_east", 1, 50, 46, 0xE9A980, true),
        ("pickaxe_east", 1, 42, 42, 0xBA6A4C, false),
        ("pickaxe_east", 1, 43, 46, 0xBA6A4C, false),
        ("pickaxe_east", 1, 57, 47, 0xFFCF36, false),
        ("brush_east", 1, 48, 43, 0xE9A980, true),
        ("brush_east", 1, 49, 43, 0xDE8F5D, true),
        ("brush_east", 1, 47, 44, 0xE9A980, true),
        ("brush_east", 1, 47, 45, 0xDE8F5D, true),
        ("brush_east", 1, 48, 45, 0x7D3B14, true),
        ("brush_east", 1, 49, 45, 0xBA6A4C, true),
        ("brush_east", 1, 49, 47, 0x936244, false),
        ("brush_east", 1, 49, 48, 0x653D25, false),
        ("brush_east", 1, 50, 50, 0x653D25, false),
        ("brush_east", 1, 40, 45, 0xBA6A4C, false),
        ("brush_east", 1, 42, 46, 0xBA6A4C, false),
        ("brush_east", 1, 37, 47, 0xBA6A4C, false),
        ("brush_east", 1, 36, 49, 0xBA6A4C, false),
        ("princely_pose_start_south", 4, 48, 42, 0xE9A980, true),
        ("princely_pose_start_south", 4, 37, 45, 0xE9A980, true),
        ("princely_pose_start_south", 4, 38, 45, 0xE9A980, true),
        ("princely_pose_start_south", 4, 37, 46, 0xE9A980, true),
        ("princely_pose_start_south", 4, 38, 46, 0xBA6A4C, true),
        ("princely_pose_start_south", 4, 42, 35, 0xE9A980, true),
        ("princely_pose_start_south", 4, 33, 47, 0xBA6A4C, false),
        ("princely_pose_start_south", 4, 44, 45, 0xBA6A4C, false),
        ("princely_pose_start_south", 4, 41, 46, 0xBA6A4C, false),
        ("princely_pose_start_south", 4, 44, 48, 0xBA6A4C, false),
        ("princely_pose_start_south", 4, 50, 40, 0x711A1C, false),
        ("princely_pose_start_south", 4, 49, 38, 0xC1253C, false),
        ("princely_pose_start_south", 4, 38, 33, 0xFCDAE0, false),
        ("princely_pose_start_south", 5, 36, 47, 0xBA6A4C, false),
        ("princely_pose_start_south", 5, 33, 48, 0xBA6A4C, false),
        ("princely_pose_start_south", 6, 36, 48, 0xBA6A4C, false),
        ("princely_pose_start_south", 6, 34, 50, 0xBA6A4C, false),
        ("princely_pose_start_south", 7, 37, 48, 0xBA6A4C, false),
        ("princely_pose_loop_south", 0, 48, 42, 0xE9A980, true),
        ("princely_pose_loop_south", 0, 42, 35, 0xE9A980, true),
        ("princely_pose_loop_south", 0, 37, 48, 0xBA6A4C, false),
        ("princely_pose_loop_south", 0, 44, 45, 0xBA6A4C, false),
        ("princely_pose_end_south", 0, 36, 50, 0xBA6A4C, false),
        ("princely_pose_end_south", 0, 44, 46, 0xBA6A4C, false),
        ("princely_pose_end_south", 1, 43, 46, 0xBA6A4C, false),
        ("princely_pose_end_south", 1, 43, 49, 0xBA6A4C, false),
        ("princely_pose_end_south", 2, 37, 46, 0xBA6A4C, false),
        ("princely_pose_end_south", 2, 42, 49, 0xBA6A4C, false),
        ("trowel_east", 0, 40, 50, 0xBA6A4C, true),
        ("trowel_east", 0, 45, 50, 0xBA6A4C, true),
        ("trowel_east", 0, 46, 50, 0xDE8F5D, true),
        ("trowel_east", 0, 46, 51, 0xDE8F5D, true),
        ("trowel_east", 0, 38, 51, 0xBA6A4C, true),
        ("trowel_east", 0, 39, 52, 0xE9A980, true),
        ("trowel_east", 0, 40, 53, 0x7D3B14, true),
        ("trowel_east", 0, 41, 53, 0xE9A980, true),
        ("trowel_east", 0, 41, 38, 0x9C5241, true),
        ("trowel_east", 0, 44, 52, 0x70797D, false),
        ("trowel_east", 0, 44, 53, 0x9EB0BB, false),
        ("trowel_east", 0, 38, 49, 0xC1BDC8, false),
        ("trowel_east", 0, 39, 49, 0xC1BDC8, false),
        ("trowel_east", 0, 40, 48, 0x000000, false),
        ("trowel_east", 9, 40, 50, 0xBA6A4C, true),
        ("trowel_east", 9, 45, 50, 0xBA6A4C, true),
        ("trowel_east", 9, 46, 51, 0xDE8F5D, true),
    ];
    let skin = [0xE9A980, 0xDE8F5D, 0xBA6A4C, 0x9C5241, 0x7D3B14];
    let mut first_selection = None;
    let mut prior_files = 0;
    for r in profile["regions"].as_array().unwrap() {
        let a = r["asset"].as_str().unwrap();
        assert_eq!(
            r["source_sha256"],
            format!("{:x}", Sha256::digest(fs::read(original.join(a)).unwrap()))
        );
    }
    for r in profile["regions"].as_array().unwrap().iter().take(118) {
        let a = r["asset"].as_str().unwrap();
        for file in [a.to_owned(), a.replace(".png", ".meta.toml")] {
            assert_eq!(
                fs::read(original.join(&file)).unwrap(),
                fs::read(baseline.join("original").join(&file)).unwrap(),
                "previous source {file}"
            );
            prior_files += 1;
        }
    }
    for preset in set["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let recipe = temp.path().join(format!("{id}.json"));
        let mapping: serde_json::Map<_, _> = profile["source_colors"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, c)| (c.as_str().unwrap().to_owned(), preset["colors"][i].clone()))
            .collect();
        fs::write(
            &recipe,
            serde_json::to_vec(
                &json!({"profile":profile_path.canonicalize().unwrap(),"rgba_map":mapping}),
            )
            .unwrap(),
        )
        .unwrap();
        let output = temp.path().join(id);
        let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["apply", "--input"])
            .arg(&original)
            .arg("--palette")
            .arg(&recipe)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let targets = [0, 9, 10, 3, 11].map(|i| color(&preset["colors"][i]));
        let mut selection = vec![];
        let mut changed = 0;
        for (name, frames) in [
            ("axe_east", 6),
            ("brush_east", 7),
            ("pickaxe_east", 6),
            ("princely_pose_end_south", 3),
            ("princely_pose_loop_south", 1),
            ("princely_pose_start_south", 8),
            ("trowel_east", 10),
        ] {
            let a = asset(name);
            let before = image::open(original.join(&a)).unwrap().to_rgba8();
            let after = image::open(output.join(&a)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), after.dimensions());
            assert_eq!(before.dimensions(), (frames * 80, 80));
            let meta = a.replace(".png", ".meta.toml");
            let bytes = fs::read(original.join(&meta)).unwrap();
            assert_eq!(bytes, fs::read(output.join(&meta)).unwrap());
            let parsed: toml::Value = toml::from_str(std::str::from_utf8(&bytes).unwrap()).unwrap();
            assert_eq!(
                parsed["asset_properties"]["atlas"].as_str(),
                Some("Default")
            );
            assert_eq!(
                parsed["asset_properties"]["offset"]["horizontal"].as_str(),
                Some("Middle")
            );
            assert_eq!(
                parsed["asset_properties"]["offset"]["vertical"].as_float(),
                Some(54.0)
            );
            for &(case, f, x, y, c, selected) in &landmarks {
                if case != name {
                    continue;
                }
                assert_eq!(
                    before.get_pixel(f * 80 + x, y).0,
                    rgba(c),
                    "source {case} {f} [{x},{y}]"
                );
                let expected = if selected {
                    targets[skin.iter().position(|s| *s == c).unwrap()]
                } else {
                    c
                };
                assert_eq!(
                    after.get_pixel(f * 80 + x, y).0,
                    rgba(expected),
                    "material {id} {case} {f} [{x},{y}] skin={selected}"
                );
            }
            let mut counts = vec![0; before.width() as usize / 80];
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                // Observed skin colors have 166 explicit uniform/gold-trim pixels excluded.
                // This independent inventory catches omitted tiny components.
                let index = skin
                    .iter()
                    .position(|c| rgba(*c) == p.0)
                    .filter(|_| !is_trim(name, x / 80, x % 80, y));
                let expected = index.map_or(p.0, |i| rgba(targets[i]));
                assert_eq!(q.0, expected, "exact source/target {id} {name} [{x},{y}]");
                selection.push(index.is_some());
                if index.is_some() {
                    counts[x as usize / 80] += 1;
                    changed += 1;
                }
            }
            let expected_counts: &[usize] = match name {
                "axe_east" => &[21, 46, 44, 36, 34, 21],
                "brush_east" => &[31, 32, 31, 32, 31, 31, 32],
                "pickaxe_east" => &[22, 45, 43, 35, 34, 22],
                "princely_pose_end_south" => &[42, 37, 42],
                "princely_pose_loop_south" => &[42],
                "princely_pose_start_south" => &[42, 37, 42, 44, 42, 42, 42, 42],
                "trowel_east" => &[40, 35, 34, 35, 34, 35, 34, 35, 35, 40],
                _ => unreachable!(),
            };
            assert_eq!(counts, expected_counts, "skin inventory {name}");
        }
        assert_eq!(changed, 1476);
        if let Some(first) = &first_selection {
            assert_eq!(first, &selection);
        } else {
            first_selection = Some(selection);
        }
        for r in profile["regions"].as_array().unwrap().iter().take(118) {
            let a = r["asset"].as_str().unwrap();
            for file in [a.to_owned(), a.replace(".png", ".meta.toml")] {
                assert_eq!(
                    fs::read(output.join(&file)).unwrap(),
                    fs::read(baseline.join("variants").join(id).join(&file)).unwrap(),
                    "previous output changed {id} {file}"
                );
                prior_files += 1;
            }
        }
    }
    assert_eq!(prior_files, 1180);
}

// Independent source-grid material inventory: gold belt and shifting cape hem.
// Tool warm bands and the rose have distinct source colors and stay unchanged.
fn is_trim(name: &str, frame: u32, x: u32, y: u32) -> bool {
    let points: &[(u32, u32)] = match (name, frame) {
        ("axe_east", 0) => &[(38, 45), (40, 46), (41, 46), (43, 46)],
        ("axe_east", 1) => &[(42, 42), (42, 46), (43, 46), (44, 46), (46, 46), (47, 47)],
        ("axe_east", 2) => &[(41, 43), (42, 46), (43, 46), (44, 46), (46, 46), (47, 47)],
        ("axe_east", 3) => &[(41, 43), (42, 46), (43, 46), (44, 46), (46, 46), (47, 47)],
        ("axe_east", 4) => &[(39, 45), (42, 46), (43, 46)],
        ("axe_east", 5) => &[(38, 45), (40, 46), (41, 46), (43, 46)],
        ("brush_east", 0) => &[(35, 47), (44, 47)],
        ("brush_east", 1) => &[(40, 45), (42, 46), (43, 46), (45, 46), (37, 47), (36, 49)],
        ("brush_east", 2) => &[(40, 45), (42, 46), (37, 47), (36, 49)],
        ("brush_east", 3) => &[(40, 45), (42, 46), (43, 46), (45, 46), (37, 47), (36, 49)],
        ("brush_east", 4) => &[(40, 45), (42, 46), (37, 47), (36, 49)],
        ("brush_east", 5) => &[(35, 47), (44, 47)],
        ("brush_east", 6) => &[(38, 45), (40, 46), (41, 46), (43, 46)],
        ("pickaxe_east", 0) => &[(38, 45), (40, 46), (41, 46), (43, 46)],
        ("pickaxe_east", 1) => &[(42, 42), (42, 46), (43, 46), (44, 46), (46, 46), (47, 47)],
        ("pickaxe_east", 2) => &[(41, 43), (42, 46), (43, 46), (44, 46), (46, 46), (47, 47)],
        ("pickaxe_east", 3) => &[(41, 43), (42, 46), (43, 46), (44, 46), (46, 46), (47, 47)],
        ("pickaxe_east", 4) => &[(39, 45), (42, 46), (43, 46)],
        ("pickaxe_east", 5) => &[(38, 45), (40, 46), (41, 46), (43, 46)],
        ("princely_pose_end_south", 0) => &[
            (44, 46),
            (41, 47),
            (42, 47),
            (45, 47),
            (44, 48),
            (44, 49),
            (36, 50),
        ],
        ("princely_pose_end_south", 1) => {
            &[(43, 46), (40, 47), (41, 47), (44, 47), (43, 48), (43, 49)]
        }
        ("princely_pose_end_south", 2) => {
            &[(37, 46), (42, 46), (39, 47), (40, 47), (42, 48), (42, 49)]
        }
        ("princely_pose_loop_south", 0) => &[
            (44, 45),
            (41, 46),
            (42, 46),
            (45, 46),
            (44, 47),
            (44, 48),
            (37, 48),
        ],
        ("princely_pose_start_south", 0) => {
            &[(37, 46), (42, 46), (39, 47), (40, 47), (42, 48), (42, 49)]
        }
        ("princely_pose_start_south", 1) => {
            &[(43, 46), (40, 47), (41, 47), (44, 47), (43, 48), (43, 49)]
        }
        ("princely_pose_start_south", 2) => {
            &[(44, 46), (41, 47), (42, 47), (45, 47), (44, 48), (44, 49)]
        }
        ("princely_pose_start_south", 3) => {
            &[(44, 45), (41, 46), (42, 46), (45, 46), (44, 47), (44, 48)]
        }
        ("princely_pose_start_south", 4) => &[
            (44, 45),
            (41, 46),
            (42, 46),
            (45, 46),
            (33, 47),
            (44, 47),
            (44, 48),
        ],
        ("princely_pose_start_south", 5) => &[
            (44, 45),
            (41, 46),
            (42, 46),
            (45, 46),
            (36, 47),
            (44, 47),
            (44, 48),
            (33, 48),
        ],
        ("princely_pose_start_south", 6) => &[
            (44, 45),
            (41, 46),
            (42, 46),
            (45, 46),
            (44, 47),
            (44, 48),
            (36, 48),
            (34, 50),
        ],
        ("princely_pose_start_south", 7) => &[
            (44, 45),
            (41, 46),
            (42, 46),
            (45, 46),
            (44, 47),
            (44, 48),
            (37, 48),
        ],
        _ => &[],
    };
    points.contains(&(x, y))
}
