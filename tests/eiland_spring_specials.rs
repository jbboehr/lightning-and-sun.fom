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
#[ignore = "requires 188 local animations in extracted/eiland-autumn-standard-study and the accepted earlier output baseline"]
fn eiland_spring_specials_cover_moving_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/eiland-autumn-standard-study");
    let baseline = root
        .join("generated/characters-balor-valen-eiland-spring-reactions-trial/characters/eiland");
    let profile_path = std::env::var_os("FOM_EILAND_SPRING_SPECIALS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let set_path = std::env::var_os("FOM_EILAND_SPRING_SPECIALS_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let profile = read(&profile_path);
    assert_eq!(profile["regions"].as_array().unwrap().len(), 188);
    assert_eq!(profile["source_colors"].as_array().unwrap().len(), 12);
    assert_eq!(profile["color_groups"].as_array().unwrap().len(), 7);
    let set = read(&set_path);
    let temp = tempfile::tempdir().unwrap();
    // Literal source-art landmarks distinguish moving fingers and fine face shading
    // from gold trim, uniform, eye details, orange quill and brown writing surface.
    let landmarks = [
        ("write_start_south", 0, 40, 34, 0x9C5241, true),
        ("write_start_south", 0, 39, 35, 0xBA6A4C, true),
        ("write_start_south", 0, 38, 36, 0xE9A980, true),
        ("write_start_south", 0, 37, 40, 0x7D3B14, true),
        ("write_start_south", 0, 39, 41, 0xBA6A4C, true),
        ("write_start_south", 0, 37, 46, 0xBA6A4C, false),
        ("write_start_south", 0, 39, 47, 0xBA6A4C, false),
        ("write_start_south", 0, 33, 46, 0xD36A0E, false),
        ("write_start_south", 0, 33, 48, 0xFAB680, false),
        ("write_start_south", 0, 34, 48, 0xFFD8D1, false),
        ("write_start_south", 0, 34, 49, 0x663409, false),
        ("write_start_south", 0, 46, 47, 0xB28159, false),
        ("write_start_south", 0, 37, 30, 0xE797AC, false),
        ("write_start_south", 1, 42, 48, 0xE9A980, true),
        ("write_start_south", 1, 44, 48, 0xBA6A4C, true),
        ("write_start_south", 1, 43, 49, 0xDE8F5D, true),
        ("write_start_south", 1, 38, 39, 0x7D3B14, true),
        ("write_start_south", 1, 35, 47, 0xBA6A4C, false),
        ("write_start_south", 1, 34, 48, 0xBA6A4C, false),
        ("write_start_south", 1, 37, 45, 0xBA6A4C, false),
        ("write_start_south", 1, 42, 47, 0x57342B, false),
        ("write_start_south", 1, 46, 45, 0xB28159, false),
        ("write_start_south", 1, 33, 43, 0xF9AB6C, false),
        ("write_loop_south", 0, 36, 45, 0xDE8F5D, true),
        ("write_loop_south", 0, 37, 45, 0xE9A980, true),
        ("write_loop_south", 0, 36, 46, 0xBA6A4C, true),
        ("write_loop_south", 0, 41, 47, 0xDE8F5D, true),
        ("write_loop_south", 0, 42, 48, 0xE9A980, true),
        ("write_loop_south", 0, 35, 47, 0xBA6A4C, false),
        ("write_loop_south", 0, 34, 48, 0xBA6A4C, false),
        ("write_loop_south", 0, 45, 48, 0xBA6A4C, false),
        ("write_loop_south", 0, 39, 44, 0xFFD8D1, false),
        ("write_loop_south", 0, 42, 46, 0x57342B, false),
        ("write_loop_south", 0, 37, 36, 0x6C2859, false),
        ("write_loop_south", 0, 38, 37, 0xECF0E9, false),
        ("write_loop_south", 1, 36, 46, 0xE9A980, true),
        ("write_loop_south", 1, 35, 47, 0xBA6A4C, true),
        ("write_loop_south", 1, 42, 48, 0xE9A980, true),
        ("write_loop_south", 1, 34, 48, 0xBA6A4C, false),
        ("write_loop_south", 1, 45, 48, 0xBA6A4C, false),
        ("write_loop_south", 2, 34, 45, 0xDE8F5D, true),
        ("write_loop_south", 2, 34, 46, 0xBA6A4C, true),
        ("write_loop_south", 2, 41, 48, 0xDE8F5D, true),
        ("write_loop_south", 2, 34, 48, 0xBA6A4C, false),
        ("write_loop_south", 2, 38, 45, 0x663409, false),
        ("write_loop_south", 3, 34, 46, 0xE9A980, true),
        ("write_loop_south", 3, 34, 47, 0xDE8F5D, true),
        ("write_loop_south", 3, 43, 48, 0xDE8F5D, true),
        ("write_loop_south", 3, 45, 48, 0xBA6A4C, false),
        ("write_loop_south", 3, 38, 46, 0x663409, false),
        ("write_end_south", 0, 44, 48, 0xBA6A4C, true),
        ("write_end_south", 1, 42, 46, 0xBA6A4C, false),
        ("write_sit_start_south", 0, 39, 41, 0xBA6A4C, true),
        ("write_sit_start_south", 0, 37, 44, 0xBA6A4C, false),
        ("write_sit_start_south", 0, 42, 44, 0xBA6A4C, false),
        ("write_sit_start_south", 0, 37, 45, 0x6A3126, false),
        ("write_sit_start_south", 1, 44, 48, 0xBA6A4C, true),
        ("write_sit_start_south", 1, 37, 44, 0xBA6A4C, false),
        ("write_sit_start_south", 1, 42, 47, 0x57342B, false),
        ("write_sit_loop_south", 0, 42, 48, 0xE9A980, true),
        ("write_sit_loop_south", 0, 46, 45, 0xB28159, false),
        ("write_sit_loop_south", 1, 42, 48, 0xE9A980, true),
        ("write_sit_loop_south", 1, 46, 45, 0xB28159, false),
        ("write_sit_loop_south", 2, 42, 48, 0xE9A980, true),
        ("write_sit_loop_south", 2, 46, 45, 0xB28159, false),
        ("write_sit_loop_south", 3, 42, 48, 0xE9A980, true),
        ("write_sit_loop_south", 3, 46, 45, 0xB28159, false),
        ("write_sit_end_south", 0, 43, 49, 0xDE8F5D, true),
        ("write_sit_end_south", 1, 37, 44, 0xBA6A4C, false),
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
    for r in profile["regions"].as_array().unwrap().iter().take(106) {
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
            ("write_end_south", 2),
            ("write_loop_south", 4),
            ("write_sit_end_south", 2),
            ("write_sit_loop_south", 4),
            ("write_sit_start_south", 2),
            ("write_start_south", 2),
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
                // Observed skin colors have 31 explicit uniform/gold-trim pixels excluded.
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
                "write_end_south" => &[30, 34],
                "write_loop_south" => &[36, 35, 36, 36],
                "write_sit_end_south" => &[30, 34],
                "write_sit_loop_south" => &[35, 34, 35, 35],
                "write_sit_start_south" => &[34, 30],
                "write_start_south" => &[34, 30],
                _ => unreachable!(),
            };
            assert_eq!(counts, expected_counts, "skin inventory {name}");
        }
        assert_eq!(changed, 538);
        if let Some(first) = &first_selection {
            assert_eq!(first, &selection);
        } else {
            first_selection = Some(selection);
        }
        for r in profile["regions"].as_array().unwrap().iter().take(106) {
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
    assert_eq!(prior_files, 1060);
}

// Independent source-grid inventory: shared gold-trim and uniform pixels.
fn is_trim(name: &str, frame: u32, x: u32, y: u32) -> bool {
    let points: &[(u32, u32)] = match (name, frame) {
        ("write_start_south", 0) => &[(37, 46), (42, 46), (39, 47), (40, 47)],
        ("write_start_south", 1) => &[(37, 45), (39, 46), (40, 46), (35, 47), (34, 48)],
        ("write_end_south", 0) => &[(37, 45), (39, 46), (40, 46), (35, 47), (34, 48)],
        ("write_end_south", 1) => &[(37, 46), (42, 46), (39, 47), (40, 47)],
        ("write_loop_south", 0) => &[(35, 47), (34, 48), (45, 48)],
        ("write_loop_south", 1) => &[(34, 48), (45, 48)],
        ("write_loop_south", 2) => &[(34, 48)],
        ("write_loop_south", 3) => &[(45, 48)],
        ("write_sit_start_south", 0) => &[(37, 44), (42, 44)],
        ("write_sit_start_south", 1) => &[(37, 44)],
        ("write_sit_end_south", 0) => &[(37, 44)],
        ("write_sit_end_south", 1) => &[(37, 44), (42, 44)],
        _ => &[],
    };
    points.contains(&(x, y))
}
