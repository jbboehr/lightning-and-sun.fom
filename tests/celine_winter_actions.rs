use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 328 local animations in extracted/celine-wedding-pilot-study"]
fn celine_autumn_garden_cover_hands_and_preserve_braids_tools_and_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/celine-wedding-pilot-study");
    let profile = std::env::var_os("FOM_CELINE_WINTER_ACTIONS_PROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/profiles/celine-world-trial.json"));
    let profile: Value = serde_json::from_slice(&fs::read(profile).unwrap()).unwrap();
    let set: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/sets/celine-world-trial.json")).unwrap(),
    )
    .unwrap();
    let temp = tempfile::tempdir().unwrap();
    let profile_path = temp.path().join("profile.json");
    fs::write(&profile_path, serde_json::to_vec(&profile).unwrap()).unwrap();
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let source = [0xFCD9B3, 0xF0B988, 0xD37A57, 0x672115];
    // Source-art landmarks distinguish moving wrists, fingers and facial contours
    // from identical dark braid, belt and footwear colors in the Autumn garden outfit.
    let landmarks = [
        ("idle_south", 0, 43, 44, 0x8686A6, false),
        ("idle_south", 0, 42, 45, 0x672115, false),
        ("idle_south", 0, 37, 52, 0x672115, false),
        ("idle_east", 0, 44, 36, 0x672115, true),
        ("idle_east", 0, 35, 40, 0x672115, false),
        ("idle_east", 0, 35, 47, 0x672115, true),
        ("idle_north", 0, 36, 44, 0x8686A6, false),
        ("idle_north", 0, 35, 36, 0x672115, false),
        ("walk_north", 1, 34, 48, 0x672115, true),
        ("walk_north", 1, 37, 46, 0x000000, false),
        ("walk_north", 3, 45, 48, 0x672115, true),
        ("walk_south", 1, 34, 48, 0x672115, true),
        ("walk_south", 1, 34, 41, 0x672115, false),
        ("walk_south", 3, 45, 48, 0x672115, true),
        ("walk_east", 1, 44, 46, 0xB1B1C7, false),
        ("walk_east", 1, 41, 50, 0xB65932, false),
        ("walk_east", 1, 39, 51, 0x672115, false),
        ("walk_east", 1, 42, 46, 0x672115, false),
        ("walk_east", 3, 36, 48, 0x672115, true),
        ("blink_east", 1, 44, 35, 0x672115, true),
        ("blink_east", 1, 35, 40, 0x672115, false),
        ("blink_east", 1, 35, 47, 0x672115, true),
        ("blink_south", 1, 36, 35, 0x672115, true),
        ("blink_south", 1, 43, 44, 0x8686A6, false),
        ("blink_south", 1, 36, 40, 0x672115, false),
        ("sit_east", 0, 35, 47, 0x672115, true),
        ("sit_east", 0, 43, 44, 0x000000, false),
        ("sit_east", 0, 41, 48, 0x672115, false),
        ("sit_north", 0, 34, 47, 0x672115, true),
        ("sit_north", 0, 35, 36, 0x672115, false),
        ("sit_south", 0, 45, 47, 0x672115, true),
        ("sit_south", 0, 42, 45, 0x535B8B, false),
        ("kiss_east", 2, 47, 35, 0x672115, true),
        ("kiss_east", 2, 36, 46, 0x672115, true),
        ("kiss_east", 2, 38, 41, 0x672115, false),
        ("kiss_east", 2, 44, 45, 0x000000, false),
        ("kiss_east", 1, 36, 48, 0x672115, true),
        ("harvest_east", 0, 40, 53, 0x672115, true),
        ("harvest_east", 0, 38, 45, 0x672115, false),
        ("harvest_east", 0, 43, 50, 0xB65932, false),
        ("harvest_east", 1, 48, 51, 0x672115, true),
        ("harvest_east", 1, 42, 52, 0x672115, true),
        ("harvest_east", 1, 44, 50, 0xB65932, false),
        ("harvest_east", 3, 51, 52, 0x672115, true),
        ("harvest_east", 3, 47, 53, 0x672115, true),
        ("harvest_east", 3, 40, 47, 0x672115, false),
        ("harvest_east", 3, 41, 50, 0x535B8B, false),
        ("water_east", 0, 38, 44, 0x33335C, false),
        ("water_east", 0, 36, 41, 0x000000, false),
        ("water_east", 0, 46, 44, 0xFFCF36, false),
        ("water_east", 1, 47, 39, 0x672115, true),
        ("water_east", 1, 37, 39, 0x672115, false),
        ("water_east", 1, 43, 52, 0x672115, false),
        ("water_east", 3, 36, 44, 0x33335C, false),
        ("water_east", 3, 43, 43, 0xDD9D3E, false),
        ("water_east", 0, 44, 37, 0x672115, true),
        ("water_east", 1, 47, 36, 0x672115, true),
        ("kiss_east", 2, 40, 40, 0x672115, false),
        ("kiss_east", 2, 42, 40, 0x672115, false),
    ];
    let mut first_selection = Vec::new();
    for (id, targets) in [
        ("blue", [0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83]),
        ("npc_hayden", [0xE8B271, 0xCA9052, 0xB27146, 0x6E4922]),
        ("npc_ryis", [0xB06C57, 0x814A3A, 0x63342A, 0x491F1B]),
        ("npc_seridia", [0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48]),
    ] {
        let preset = set["presets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["id"] == id)
            .unwrap();
        for (i, shade) in [11, 12, 13, 9].into_iter().enumerate() {
            assert_eq!(
                u32::from_str_radix(&preset["colors"][shade].as_str().unwrap()[1..7], 16).unwrap(),
                targets[i]
            );
        }
        let map = profile["source_colors"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, color)| {
                (
                    color.as_str().unwrap().to_owned(),
                    preset["colors"][i].clone(),
                )
            })
            .collect::<serde_json::Map<_, _>>();
        let recipe = temp.path().join(format!("{id}.json"));
        fs::write(
            &recipe,
            serde_json::to_vec(&json!({"profile":profile_path,"rgba_map":map})).unwrap(),
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
        let mut selection = Vec::new();
        let (mut changed, mut protected, mut frames) = (0, 0, 0);
        for (name, count) in [
            ("water_east", 135),
            ("kiss_east", 163),
            ("blink_east", 118),
            ("blink_south", 123),
            ("idle_east", 36),
            ("idle_north", 12),
            ("idle_south", 38),
            ("sit_east", 34),
            ("sit_north", 6),
            ("sit_south", 32),
            ("walk_east", 146),
            ("walk_north", 44),
            ("walk_south", 148),
            ("harvest_east", 426),
        ] {
            let prefix = if name.starts_with("harvest") || name.starts_with("water") {
                "specialanimation_autumn_garden"
            } else {
                "autumn_garden"
            };
            let asset = format!(
                "assets/animations/NPCs/Celine/Sprites/Autumn/spr_npc_celine_{prefix}_{name}.png"
            );
            let before = image::open(original.join(&asset)).unwrap().to_rgba8();
            let after = image::open(output.join(&asset)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), after.dimensions());
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(output.join(meta)).unwrap()
            );
            for &(case, frame, x, y, color, skin) in &landmarks {
                if case == name {
                    let x = x + frame * 80;
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        rgba(color),
                        "source {case} [{x},{y}]"
                    );
                    assert_eq!(
                        before.get_pixel(x, y) != after.get_pixel(x, y),
                        skin,
                        "material boundary {id} {case} [{x},{y}]"
                    );
                }
            }
            let mut per_frame = vec![0; before.width() as usize / 80];
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3]);
                selection.push(p != q);
                let shade = source.iter().position(|c| rgba(*c) == p.0);
                if p != q {
                    assert_eq!(
                        q.0,
                        rgba(targets[shade.expect("non-skin material changed")])
                    );
                    per_frame[x as usize / 80] += 1;
                } else if let Some(i) = shade {
                    assert_eq!(i, 3, "missing skin {id} {name} [{x},{y}]");
                    protected += 1;
                }
            }
            assert!(per_frame.iter().all(|n| *n > 0));
            assert_eq!(per_frame.iter().sum::<usize>(), count, "{id} {name}");
            frames += per_frame.len();
            changed += count;
        }
        assert_eq!((changed, protected, frames), (1461, 262, 42));
        if first_selection.is_empty() {
            first_selection = selection;
        } else {
            assert_eq!(first_selection, selection);
        }
    }
}
