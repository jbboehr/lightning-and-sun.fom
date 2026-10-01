use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn eiland_world_adds_six_pinned_regions_with_isolated_world_aliases() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let p = read(root.join("palettes/profiles/eiland-world-trial.json"));
    assert_eq!(p["regions"].as_array().unwrap().len(), 188);
    assert_eq!(p["source_colors"].as_array().unwrap().len(), 12);
    assert_eq!(p["color_groups"].as_array().unwrap().len(), 7);
    let expected: Vec<_> = [
        "idle_east",
        "idle_north",
        "idle_south",
        "walk_east",
        "walk_north",
        "walk_south",
    ]
    .iter()
    .map(|n| json!(asset(n)))
    .collect();
    let actual: Vec<_> = p["regions"].as_array().unwrap()[78..84]
        .iter()
        .map(|r| r["asset"].clone())
        .collect();
    assert_eq!(actual, expected);
    let portrait = read(root.join("palettes/profiles/eiland-portraits.json"));
    assert_eq!(
        &p["regions"].as_array().unwrap()[..78],
        portrait["regions"].as_array().unwrap()
    );
    assert_eq!(
        &p["source_colors"].as_array().unwrap()[..9],
        portrait["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &p["color_groups"].as_array().unwrap()[..4],
        portrait["color_groups"].as_array().unwrap()
    );
    assert_eq!(
        &p["source_colors"].as_array().unwrap()[9..],
        &[json!("#DE8F5DFF"), json!("#BA6A4CFF"), json!("#7D3B14FF")]
    );
    assert_eq!(
        &p["color_groups"].as_array().unwrap()[4..],
        &[
            json!(["#DE8F5DFF"]),
            json!(["#BA6A4CFF"]),
            json!(["#7D3B14FF"])
        ]
    );
    let previous = read(root.join("palettes/sets/eiland-portraits-trial.json"));
    let world = read(root.join("palettes/sets/eiland-world-trial.json"));
    let style = read(root.join("palettes/stylized/eiland-world-trial.json"));
    assert_eq!(world["profile"], "../profiles/eiland-world-trial.json");
    assert_eq!(style["profile"], "../profiles/eiland-world-trial.json");
    for (before, after) in previous["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(world["presets"].as_array().unwrap())
    {
        assert_eq!(before["id"], after["id"]);
        assert_eq!(before["label"], after["label"]);
        assert_eq!(
            &after["colors"].as_array().unwrap()[..9],
            before["colors"].as_array().unwrap()
        );
        assert_eq!(
            &after["colors"].as_array().unwrap()[9..],
            &[
                before["colors"][1].clone(),
                before["colors"][2].clone(),
                before["colors"][3].clone()
            ]
        );
    }
    for (i, c) in p["source_colors"].as_array().unwrap().iter().enumerate() {
        assert_eq!(
            style["rgba_map"][c.as_str().unwrap()],
            world["presets"][0]["colors"][i]
        );
    }
}

fn asset(name: &str) -> String {
    let prefix = "spring";
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
fn eiland_world_cover_moving_skin_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/eiland-autumn-standard-study");
    let baseline = root
        .join("generated/characters-reina-juniper-march-wedding-finish-trial/characters/eiland");
    let profile_path = std::env::var_os("FOM_EILAND_WORLD_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/eiland-world-trial.json"));
    let set_path = std::env::var_os("FOM_EILAND_WORLD_SET")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/eiland-world-trial.json"));
    let profile = read(&profile_path);
    let set = read(&set_path);
    let temp = tempfile::tempdir().unwrap();
    // Literal source-art landmarks distinguish forehead/face shading and moving
    // fingers from pink hair, gold shoulder and belt trim, cape and boots.
    let landmarks = [
        ("idle_east", 0, 40, 35, 0xE9A980, true),
        ("idle_east", 0, 39, 38, 0xDE8F5D, true),
        ("idle_east", 0, 38, 38, 0xBA6A4C, true),
        ("idle_east", 0, 40, 33, 0x9C5241, true),
        ("idle_east", 0, 38, 39, 0x7D3B14, true),
        ("idle_east", 0, 35, 47, 0x7D3B14, true),
        ("idle_east", 0, 45, 46, 0xDE8F5D, true),
        ("idle_east", 0, 40, 46, 0xBA6A4C, false),
        ("idle_east", 0, 38, 46, 0x6A3126, false),
        ("idle_east", 0, 43, 43, 0xBA6A4C, false),
        ("idle_east", 0, 35, 45, 0xEDE0EF, false),
        ("idle_east", 0, 40, 49, 0x000000, false),
        ("idle_east", 0, 37, 36, 0x6C2859, false),
        ("idle_north", 0, 32, 46, 0xE9A980, true),
        ("idle_north", 0, 47, 47, 0xE9A980, true),
        ("idle_north", 0, 39, 46, 0x6A3126, false),
        ("idle_north", 0, 39, 45, 0xF0BC70, false),
        ("idle_north", 0, 39, 40, 0x927D96, false),
        ("idle_south", 0, 37, 39, 0x7D3B14, true),
        ("idle_south", 0, 42, 39, 0x7D3B14, true),
        ("idle_south", 0, 40, 33, 0x9C5241, true),
        ("idle_south", 0, 33, 47, 0x7D3B14, true),
        ("idle_south", 0, 46, 47, 0x7D3B14, true),
        ("idle_south", 0, 37, 45, 0xBA6A4C, false),
        ("idle_south", 0, 42, 48, 0xBA6A4C, false),
        ("idle_south", 0, 39, 46, 0xBA6A4C, false),
        ("walk_east", 1, 33, 47, 0x7D3B14, true),
        ("walk_east", 1, 46, 47, 0x7D3B14, true),
        ("walk_east", 1, 47, 47, 0xBA6A4C, true),
        ("walk_east", 1, 45, 44, 0xBA6A4C, false),
        ("walk_east", 1, 43, 47, 0xBA6A4C, false),
        ("walk_east", 3, 36, 48, 0x7D3B14, true),
        ("walk_east", 3, 35, 48, 0xBA6A4C, true),
        ("walk_east", 3, 38, 44, 0xBA6A4C, false),
        ("walk_east", 3, 44, 48, 0xBA6A4C, false),
        ("walk_north", 1, 33, 46, 0xBA6A4C, true),
        ("walk_north", 1, 33, 48, 0xBA6A4C, true),
        ("walk_north", 1, 40, 47, 0x6A3126, false),
        ("walk_north", 3, 46, 46, 0xBA6A4C, true),
        ("walk_north", 3, 46, 48, 0xBA6A4C, true),
        ("walk_north", 3, 39, 47, 0x6A3126, false),
        ("walk_south", 1, 34, 48, 0x7D3B14, true),
        ("walk_south", 1, 45, 46, 0xBA6A4C, true),
        ("walk_south", 1, 43, 47, 0xBA6A4C, false),
        ("walk_south", 1, 42, 49, 0xBA6A4C, false),
        ("walk_south", 3, 33, 47, 0x7D3B14, true),
        ("walk_south", 3, 46, 48, 0xBA6A4C, true),
        ("walk_south", 3, 42, 49, 0xBA6A4C, false),
        ("walk_south", 3, 42, 46, 0xBA6A4C, false),
        ("idle_east", 0, 38, 36, 0xECF0E9, false),
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
    for r in profile["regions"].as_array().unwrap().iter().take(78) {
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
            ("idle_east", 1),
            ("idle_north", 1),
            ("idle_south", 1),
            ("walk_east", 4),
            ("walk_north", 4),
            ("walk_south", 4),
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
                // Observed skin colors have sixty explicit clothing pixels excluded.
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
                "idle_east" => &[32],
                "idle_north" => &[4],
                "idle_south" => &[38],
                "walk_east" => &[32, 37, 32, 31],
                "walk_north" => &[4, 3, 4, 3],
                "walk_south" => &[38, 36, 38, 34],
                _ => unreachable!(),
            };
            assert_eq!(counts, expected_counts, "skin inventory {name}");
        }
        assert_eq!(changed, 366);
        if let Some(first) = &first_selection {
            assert_eq!(first, &selection);
        } else {
            first_selection = Some(selection);
        }
        for r in profile["regions"].as_array().unwrap().iter().take(78) {
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
    assert_eq!(prior_files, 780);
}

// These literal pixels are gold shoulder/belt shading, independently read from
// the source. The same BA6A4C shade also occurs on actual face and finger skin.
fn is_trim(name: &str, frame: u32, x: u32, y: u32) -> bool {
    let points: &[(u32, u32)] = match (name, frame) {
        ("idle_east", 0) | ("walk_east", 0 | 2) => {
            &[(43, 43), (38, 45), (40, 46), (41, 46), (43, 46)]
        }
        ("walk_east", 1) => &[(45, 44), (38, 46), (40, 47), (41, 47), (43, 47)],
        ("walk_east", 3) => &[(38, 44), (40, 47), (41, 47), (43, 47), (44, 48)],
        ("idle_south", 0) | ("walk_south", 0 | 2) => &[
            (37, 45),
            (42, 45),
            (39, 46),
            (40, 46),
            (43, 46),
            (42, 47),
            (42, 48),
        ],
        ("walk_south", 1 | 3) => &[
            (37, 46),
            (42, 46),
            (39, 47),
            (40, 47),
            (43, 47),
            (42, 48),
            (42, 49),
        ],
        _ => &[],
    };
    points.contains(&(x, y))
}
