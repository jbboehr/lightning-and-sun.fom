use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 299 local animations in extracted/celine-winter-general-study"]
fn celine_summer_actions_cover_bare_arms_and_preserve_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/celine-winter-general-study");
    let profile = std::env::var_os("FOM_CELINE_SEASONAL_ACTIONS_PROFILE")
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
    // Independent source-art points distinguish exposed forearms/armpits,
    // fingertips and toes from shared hair, belt, scarf and sandal colors.
    let landmarks = [
        ("blink_east", 0, 34, 32, 0x672115, false),
        ("blink_east", 1, 44, 35, 0x672115, true),
        ("blink_east", 0, 35, 47, 0x672115, true),
        ("blink_east", 1, 38, 45, 0x672115, false),
        ("blink_south", 0, 36, 44, 0x672115, true),
        ("blink_south", 1, 36, 35, 0x672115, true),
        ("blink_south", 0, 37, 45, 0x672115, false),
        ("sit_east", 0, 35, 47, 0x672115, true),
        ("sit_east", 0, 43, 44, 0x672115, false),
        ("sit_east", 0, 41, 49, 0xD37A57, true),
        ("sit_east", 0, 42, 48, 0xB65932, false),
        ("sit_north", 0, 34, 47, 0x672115, true),
        ("sit_north", 0, 45, 47, 0x672115, true),
        ("sit_north", 0, 38, 46, 0x672115, false),
        ("sit_south", 0, 34, 47, 0x672115, true),
        ("sit_south", 0, 36, 44, 0x672115, true),
        ("sit_south", 0, 37, 50, 0xFCD9B3, true),
        ("sit_south", 0, 37, 49, 0xB65932, false),
        ("drink_east", 1, 42, 34, 0x672115, true),
        ("drink_east", 1, 32, 32, 0x672115, false),
        ("drink_east", 1, 36, 44, 0xF0B988, true),
        ("drink_east", 1, 43, 44, 0x672115, false),
        ("drink_north", 1, 34, 47, 0x672115, true),
        ("drink_north", 1, 35, 46, 0x672115, false),
        ("drink_south", 1, 35, 43, 0x672115, true),
        ("drink_south", 1, 38, 42, 0x672115, false),
        ("drink_south", 1, 43, 44, 0x672115, true),
        ("eat_east", 0, 39, 43, 0x672115, true),
        ("eat_east", 1, 40, 43, 0x672115, true),
        ("eat_east", 1, 46, 42, 0x672115, true),
        ("eat_east", 2, 39, 42, 0x672115, true),
        ("eat_east", 2, 43, 44, 0x672115, false),
        ("eat_east", 2, 40, 36, 0x9E2626, false),
        ("eat_east", 3, 39, 44, 0x672115, true),
        ("eat_east", 4, 40, 42, 0x672115, true),
        ("eat_east", 4, 38, 45, 0x672115, false),
        ("eat_north", 1, 34, 47, 0x672115, true),
        ("eat_north", 1, 38, 47, 0x672115, false),
        ("eat_south", 0, 36, 43, 0x672115, true),
        ("eat_south", 1, 38, 44, 0x672115, true),
        ("eat_south", 1, 38, 42, 0x672115, false),
        ("eat_south", 2, 36, 33, 0x672115, true),
        ("eat_south", 2, 39, 35, 0x410808, false),
        ("eat_south", 2, 39, 37, 0x9E2626, false),
        ("eat_south", 4, 34, 47, 0x672115, true),
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
            ("blink_east", 193),
            ("blink_south", 220),
            ("sit_east", 49),
            ("sit_north", 8),
            ("sit_south", 54),
            ("drink_east", 165),
            ("drink_north", 12),
            ("drink_south", 163),
            ("eat_east", 265),
            ("eat_north", 12),
            ("eat_south", 292),
        ] {
            let asset = format!(
                "assets/animations/NPCs/Celine/Sprites/Summer/spr_npc_celine_summer_{name}.png"
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
        assert_eq!((changed, protected, frames), (1433, 196, 31));
        if first_selection.is_empty() {
            first_selection = selection;
        } else {
            assert_eq!(first_selection, selection);
        }
    }
}
