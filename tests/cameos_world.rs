use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const CHARACTERS: [&str; 4] = ["darren", "linnet", "wiscar", "wynne"];
const ACTIONS: [(&str, u32, &str); 12] = [
    ("action_east", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
    ("blink_east", 3, "[0.075,0.125,0.075]"),
    ("blink_south", 3, "[0.075,0.125,0.075]"),
    ("idle_east", 1, ""),
    ("idle_north", 1, ""),
    ("idle_south", 1, ""),
    ("sit_east", 1, ""),
    ("sit_north", 1, ""),
    ("sit_south", 1, ""),
    ("walk_east", 4, "0.15"),
    ("walk_north", 4, "0.15"),
    ("walk_south", 4, "0.15"),
];
fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn title(id: &str) -> String {
    let mut c = id.chars();
    c.next().unwrap().to_uppercase().to_string() + c.as_str()
}
fn assets(id: &str) -> Vec<(String, u32, &str)> {
    ACTIONS
        .iter()
        .filter(|(a, _, _)| *a != "action_east" || id == "linnet")
        .map(|(a, n, d)| {
            (
                format!(
                    "assets/animations/NPCs/{}/Sprites/spr_npc_{id}_spring_{a}.png",
                    title(id)
                ),
                *n,
                *d,
            )
        })
        .collect()
}
#[test]
fn cameo_profiles_preserve_portraits_roles_and_complete_root_world_inventory() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for id in CHARACTERS {
        let old = read(root.join(format!("palettes/profiles/{id}-portraits.json")));
        let new = read(root.join(format!("palettes/profiles/{id}-world-trial.json")));
        let n = old["source_colors"].as_array().unwrap().len();
        assert_eq!(
            &new["source_colors"].as_array().unwrap()[..n],
            old["source_colors"].as_array().unwrap()
        );
        assert_eq!(
            &new["color_groups"].as_array().unwrap()[..n],
            old["color_groups"].as_array().unwrap()
        );
        assert_eq!(
            &new["regions"].as_array().unwrap()[..8],
            old["regions"].as_array().unwrap()
        );
        let extras: &[(&str, usize)] = match id {
            "darren" => &[],
            "linnet" => &[("#A85E44", 1), ("#7F3E30", 2), ("#B97551", 8)],
            "wiscar" => &[("#CC795D", 1)],
            "wynne" => &[("#68332C", 2)],
            _ => unreachable!(),
        };
        assert_eq!(
            new["source_colors"].as_array().unwrap().len(),
            n + extras.len()
        );
        assert_eq!(
            new["color_groups"].as_array().unwrap().len(),
            n + extras.len()
        );
        let oldset = read(root.join(format!("palettes/sets/{id}-portraits-trial.json")));
        let newset = read(root.join(format!("palettes/sets/{id}-world-trial.json")));
        assert_eq!(newset["presets"].as_array().unwrap().len(), 4);
        for (oldp, newp) in oldset["presets"]
            .as_array()
            .unwrap()
            .iter()
            .zip(newset["presets"].as_array().unwrap())
        {
            assert_eq!(oldp["id"], newp["id"]);
            assert_eq!(oldp["label"], newp["label"]);
            assert_eq!(
                &newp["colors"].as_array().unwrap()[..n],
                oldp["colors"].as_array().unwrap()
            );
            assert_eq!(newp["colors"].as_array().unwrap().len(), n + extras.len());
            for (i, (color, role)) in extras.iter().enumerate() {
                assert_eq!(new["source_colors"][n + i], *color);
                assert_eq!(new["color_groups"][n + i], serde_json::json!([color]));
                assert_eq!(newp["colors"][n + i], oldp["colors"][role]);
            }
        }
        let expected: BTreeSet<_> = assets(id)
            .into_iter()
            .map(|(a, frames, _)| {
                let r = new["regions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|r| r["asset"] == a)
                    .unwrap();
                assert_eq!(r["size"], serde_json::json!([80 * frames, 80]));
                assert_eq!(
                    r["seeds"].as_array().unwrap().is_empty(),
                    id == "linnet" && a.ends_with("_north.png")
                );
                a
            })
            .collect();
        let actual: Vec<_> = new["regions"].as_array().unwrap()[8..]
            .iter()
            .map(|r| r["asset"].as_str().unwrap().to_owned())
            .collect();
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual.into_iter().collect::<BTreeSet<_>>(), expected);
    }
}
#[test]
fn packages_preserve_all_cameo_native_frames_timing_offsets_and_pixels() {
    let temp = tempfile::tempdir().unwrap();
    let mut frames = 0;
    let mut strips = 0;
    for id in CHARACTERS {
        let original = temp.path().join(format!("original-{id}"));
        let modified = temp.path().join(format!("modified-{id}"));
        fs::create_dir(&original).unwrap();
        fs::create_dir(&modified).unwrap();
        for (a, count, duration) in assets(id) {
            let stem = Path::new(&a).file_stem().unwrap().to_str().unwrap();
            let timing = if count == 1 {
                String::new()
            } else {
                format!("frame_len={count}\nduration={duration}\n")
            };
            let atlas =
                if id == "wiscar" && (stem.contains("_idle_") || stem.ends_with("walk_north")) {
                    "PortraitsMisc"
                } else {
                    "Default"
                };
            let meta = format!(
                "[meta_properties]\nid='synthetic'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[80,80]\n{timing}atlas='{atlas}'\n[asset_properties.offset]\nhorizontal=40.0\nvertical=54.0\n"
            );
            for (dir, color) in [
                (&original, [10, 20, 30, 255]),
                (&modified, [40, 50, 60, 255]),
            ] {
                RgbaImage::from_pixel(count * 80, 80, Rgba(color))
                    .save(dir.join(format!("{stem}.png")))
                    .unwrap();
                fs::write(dir.join(format!("{stem}.meta.toml")), &meta).unwrap();
            }
        }
        let output = temp.path().join(format!("package-{id}"));
        let r = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["package-toggle", "--original"])
            .arg(&original)
            .arg("--modified")
            .arg(&modified)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap();
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        let script = fs::read_to_string(output.join("gml/palette_assets.gml")).unwrap();
        let table: Value = serde_json::from_str(
            script
                .split_once("return ")
                .unwrap()
                .1
                .split_once("; }")
                .unwrap()
                .0,
        )
        .unwrap();
        assert_eq!(table.as_array().unwrap().len(), 1);
        assert_eq!(table[0][0], id);
        let row = &table[0][4];
        assert_eq!(
            row.as_array().unwrap().len(),
            if id == "linnet" { 12 } else { 11 }
        );
        for a in row.as_array().unwrap() {
            let stem = a[0].as_str().unwrap();
            assert!(!stem.contains("west"));
            let target = a[1].as_str().unwrap();
            let before: toml::Value = toml::from_str(
                &fs::read_to_string(original.join(format!("{stem}.meta.toml"))).unwrap(),
            )
            .unwrap();
            let after: toml::Value = toml::from_str(
                &fs::read_to_string(
                    output.join(format!("animations/LightningAndSun/{target}.meta.toml")),
                )
                .unwrap(),
            )
            .unwrap();
            assert_eq!(before["asset_properties"], after["asset_properties"]);
            assert_eq!(
                fs::read(modified.join(format!("{stem}.png"))).unwrap(),
                fs::read(output.join(format!("animations/LightningAndSun/{target}.png"))).unwrap()
            );
            frames += before["asset_properties"]
                .get("frame_len")
                .and_then(toml::Value::as_integer)
                .unwrap_or(1);
            strips += 1;
        }
    }
    assert_eq!(strips, 45);
    assert_eq!(frames, 103);
}

fn rgba(n: u32) -> [u8; 4] {
    [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255]
}
fn material_exclusion(id: &str, action: &str, f: u32, x: u32, y: u32, color: u32) -> bool {
    if id == "darren" && color == 0x83453A {
        return action.ends_with("_east")
            && x == 40
            && y == if action == "walk_east" && f % 2 == 1 {
                43
            } else {
                42
            }
            || action.ends_with("_south")
                && x == 39
                && y == if action == "walk_south" && f % 2 == 1 {
                    43
                } else {
                    42
                };
    }
    if id != "wiscar" || color != 0x6D3328 {
        return false;
    }
    let cloth: &[(u32, u32)] = match (action, f) {
        ("blink_east" | "idle_east", _) | ("walk_east", 0 | 2) => {
            &[(45, 43), (43, 46), (45, 48), (38, 50)]
        }
        ("blink_south" | "idle_south", _) | ("walk_south", 0 | 2) => {
            &[(36, 46), (43, 46), (37, 50), (42, 50)]
        }
        ("sit_east", _) => &[(43, 44), (42, 46)],
        ("walk_east", 1) => &[(44, 45), (43, 47), (45, 49), (38, 51)],
        ("walk_east", 3) => &[(43, 47), (37, 51), (44, 51)],
        ("idle_north", _) | ("walk_north", 0 | 2) => &[(40, 50)],
        ("walk_north", 3) => &[(40, 51), (43, 51)],
        ("walk_south", 1) => &[(43, 47), (37, 51), (42, 51)],
        ("walk_south", 3) => &[(37, 51), (42, 51)],
        _ => &[],
    };
    cloth.contains(&(x, y))
}

fn counts(id: &str) -> Vec<(&'static str, &'static [usize])> {
    match id {
        "darren" => vec![
            ("blink_east", &[36, 41, 36]),
            ("blink_south", &[38, 42, 38]),
            ("idle_east", &[32]),
            ("idle_north", &[22]),
            ("idle_south", &[34]),
            ("sit_east", &[30]),
            ("sit_north", &[26]),
            ("sit_south", &[38]),
            ("walk_east", &[32, 38, 32, 28]),
            ("walk_north", &[22, 19, 22, 19]),
            ("walk_south", &[34, 31, 34, 31]),
        ],
        "linnet" => vec![
            ("action_east", &[31, 29, 31, 29, 31, 31, 31]),
            ("blink_east", &[35, 39, 35]),
            ("blink_south", &[38, 42, 38]),
            ("idle_east", &[31]),
            ("idle_north", &[0]),
            ("idle_south", &[34]),
            ("sit_east", &[31]),
            ("sit_north", &[0]),
            ("sit_south", &[34]),
            ("walk_east", &[31, 31, 31, 31]),
            ("walk_north", &[0, 0, 0, 0]),
            ("walk_south", &[34, 34, 34, 34]),
        ],
        "wiscar" => vec![
            ("blink_east", &[33, 38, 33]),
            ("blink_south", &[40, 44, 40]),
            ("idle_east", &[30]),
            ("idle_north", &[8]),
            ("idle_south", &[36]),
            ("sit_east", &[28]),
            ("sit_north", &[5]),
            ("sit_south", &[34]),
            ("walk_east", &[30, 34, 30, 29]),
            ("walk_north", &[8, 7, 8, 6]),
            ("walk_south", &[36, 34, 36, 34]),
        ],
        "wynne" => vec![
            ("blink_east", &[45, 49, 45]),
            ("blink_south", &[54, 58, 54]),
            ("idle_east", &[45]),
            ("idle_north", &[22]),
            ("idle_south", &[54]),
            ("sit_east", &[43]),
            ("sit_north", &[20]),
            ("sit_south", &[48]),
            ("walk_east", &[45, 51, 45, 45]),
            ("walk_north", &[22, 20, 22, 20]),
            ("walk_south", &[54, 52, 54, 52]),
        ],
        _ => unreachable!(),
    }
}
type Landmark = (u32, u32, u32);
fn landmarks(id: &str) -> (Vec<Landmark>, Vec<Landmark>) {
    match id {
        "darren" => (
            vec![
                (38, 33, 0xA3614E),
                (39, 32, 0x83453A),
                (38, 32, 0x63342A),
                (32, 46, 0xA3614E),
                (34, 47, 0x63342A),
            ],
            vec![
                (39, 42, 0x83453A),
                (39, 37, 0x6A4A42),
                (40, 30, 0x5E423B),
                (37, 35, 0xC2B9BE),
            ],
        ),
        "linnet" => (
            vec![
                (39, 33, 0xC7875C),
                (39, 32, 0xA85E44),
                (38, 32, 0x7F3E30),
                (37, 38, 0x4A1B16),
                (40, 37, 0xB97551),
            ],
            vec![
                (39, 31, 0xC9785A),
                (40, 36, 0xFFA793),
                (41, 37, 0xEF977D),
                (37, 28, 0xBE9FD4),
                (37, 35, 0xC2B9BE),
                (39, 43, 0xF7F1F8),
            ],
        ),
        "wiscar" => (
            vec![
                (39, 34, 0xE3A17B),
                (38, 34, 0xCC795D),
                (38, 33, 0x9F5544),
                (42, 33, 0x6D3328),
                (33, 47, 0x6D3328),
            ],
            vec![
                (36, 46, 0x6D3328),
                (39, 46, 0xD98364),
                (39, 38, 0xDC8C9F),
                (39, 32, 0xC9785A),
                (37, 36, 0xC2B9BE),
            ],
        ),
        "wynne" => (
            vec![
                (38, 33, 0xA3614E),
                (38, 32, 0x83453A),
                (37, 33, 0x68332C),
                (33, 47, 0x491F1B),
            ],
            vec![
                (39, 30, 0x3C2923),
                (38, 52, 0x9C5F25),
                (37, 36, 0xC2B9BE),
                (39, 44, 0x15224C),
            ],
        ),
        _ => unreachable!(),
    }
}
fn check_world(id: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join(format!("extracted/test-corpus/{id}"));
    let set = std::env::var_os("FOM_CAMEO_WORLD_PRESETS")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join(format!("palettes/sets/{id}-world-trial.json")));
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
    let colors: &[u32] = match id {
        "darren" => &[0xA3614E, 0x83453A, 0x63342A, 0x491F1B],
        "linnet" => &[0xC7875C, 0xA85E44, 0x7F3E30, 0x4A1B16, 0xB97551],
        "wiscar" => &[0xE3A17B, 0xCC795D, 0x9F5544, 0x6D3328],
        "wynne" => &[0xA3614E, 0x83453A, 0x68332C, 0x491F1B],
        _ => unreachable!(),
    };
    let targets: Vec<(&str, Vec<u32>)> = if id == "wiscar" {
        vec![
            ("blue", vec![0x9DB9D4, 0x7F9FBD, 0x445F83, 0x2D3F57]),
            ("npc_hayden", vec![0xE8B271, 0xCA9052, 0x6E4922, 0x493117]),
            ("npc_ryis", vec![0xB06C57, 0x814A3A, 0x491F1B, 0x311512]),
            ("npc_seridia", vec![0xC1AFA5, 0xA69084, 0x624A48, 0x413130]),
        ]
    } else {
        let mut choices = vec![
            ("blue", vec![0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83]),
            ("npc_hayden", vec![0xE8B271, 0xCA9052, 0xB27146, 0x6E4922]),
            ("npc_ryis", vec![0xB06C57, 0x814A3A, 0x63342A, 0x491F1B]),
            ("npc_seridia", vec![0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48]),
        ];
        if id == "linnet" {
            for (_, v) in &mut choices {
                v.push(v[1]);
            }
        }
        choices
    };
    let mut selected = 0;
    let mut protected = 0;
    for (action, expected_counts) in counts(id) {
        let asset = format!(
            "assets/animations/NPCs/{}/Sprites/spr_npc_{id}_spring_{action}.png",
            title(id)
        );
        let before = image::open(original.join(&asset)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (80 * expected_counts.len() as u32, 80));
        let mut actual = vec![0; expected_counts.len()];
        for (x, y, p) in before.enumerate_pixels() {
            let color = u32::from_be_bytes([0, p[0], p[1], p[2]]);
            if p[3] == 255 && colors.contains(&color) {
                if material_exclusion(id, action, x / 80, x % 80, y, color) {
                    protected += 1;
                } else {
                    assert!(
                        (30..=50).contains(&(x % 80)) && (30..=49).contains(&y),
                        "unexpected skin {id} {action} {x},{y}"
                    );
                    actual[x as usize / 80] += 1;
                }
            }
        }
        assert_eq!(actual, expected_counts, "source counts {id} {action}");
        selected += actual.iter().sum::<usize>();
        for (choice, target) in &targets {
            let after = image::open(output.join("variants").join(choice).join(&asset))
                .unwrap()
                .to_rgba8();
            assert_eq!(before.dimensions(), after.dimensions());
            for (x, y, p) in before.enumerate_pixels() {
                let color = u32::from_be_bytes([0, p[0], p[1], p[2]]);
                let expected =
                    if p[3] != 255 || material_exclusion(id, action, x / 80, x % 80, y, color) {
                        p.0
                    } else {
                        colors
                            .iter()
                            .position(|c| *c == color)
                            .map_or(p.0, |i| rgba(target[i]))
                    };
                assert_eq!(
                    after.get_pixel(x, y).0,
                    expected,
                    "{id} {action} {choice} {x},{y}"
                );
            }
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(output.join("variants").join(choice).join(meta)).unwrap()
            );
            if action == "idle_south" {
                let (skin, keep) = landmarks(id);
                for (x, y, c) in skin {
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        rgba(c),
                        "source landmark {id} {x},{y}"
                    );
                    let i = colors.iter().position(|v| *v == c).unwrap();
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        rgba(target[i]),
                        "missed landmark {id} {x},{y}"
                    );
                }
                for (x, y, c) in keep {
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        rgba(c),
                        "source protection {id} {x},{y}"
                    );
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        rgba(c),
                        "material leak {id} {x},{y}"
                    );
                }
            }
        }
    }
    assert_eq!(
        selected,
        match id {
            "darren" => 755,
            "linnet" => 830,
            "wiscar" => 661,
            "wynne" => 1019,
            _ => unreachable!(),
        }
    );
    assert_eq!(
        protected,
        match id {
            "darren" => 18,
            "wiscar" => 67,
            _ => 0,
        }
    );
    let old = read(root.join(format!("palettes/profiles/{id}-portraits.json")));
    let prior = temp.path().join("prior");
    fs::create_dir(&prior).unwrap();
    for r in old["regions"].as_array().unwrap() {
        let a = r["asset"].as_str().unwrap();
        for a in [a.to_string(), a.replace(".png", ".meta.toml")] {
            let to = prior.join(&a);
            fs::create_dir_all(to.parent().unwrap()).unwrap();
            fs::copy(original.join(a), to).unwrap();
        }
    }
    let old_output = temp.path().join("old-bundle");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["build-presets", "--original"])
        .arg(prior)
        .arg("--presets")
        .arg(root.join(format!("palettes/sets/{id}-portraits-trial.json")))
        .arg("--output")
        .arg(&old_output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for (choice, _) in &targets {
        for r in old["regions"].as_array().unwrap() {
            let a = r["asset"].as_str().unwrap();
            for a in [a.to_string(), a.replace(".png", ".meta.toml")] {
                let path = Path::new("variants").join(choice).join(a);
                assert_eq!(
                    fs::read(output.join(&path)).unwrap(),
                    fs::read(old_output.join(path)).unwrap()
                );
            }
        }
    }
}
#[test]
#[ignore = "requires complete source-pinned Darren corpus"]
fn darren_world_materials_and_retained_portraits() {
    check_world("darren")
}
#[test]
#[ignore = "requires complete source-pinned Linnet corpus"]
fn linnet_world_materials_and_retained_portraits() {
    check_world("linnet")
}
#[test]
#[ignore = "requires complete source-pinned Wiscar corpus"]
fn wiscar_world_materials_and_retained_portraits() {
    check_world("wiscar")
}
#[test]
#[ignore = "requires complete source-pinned Wynne corpus"]
fn wynne_world_materials_and_retained_portraits() {
    check_world("wynne")
}
