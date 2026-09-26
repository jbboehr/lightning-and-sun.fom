use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 334 local animations in extracted/celine-beach-actions-study"]
fn celine_winter_idle_walk_cover_skin_and_preserve_coat_hair_and_boots() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/celine-beach-actions-study");
    let profile = std::env::var_os("FOM_CELINE_WINTER_STANDARD_PROFILE")
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
    // from identical dark hair, belt and footwear colors in the Winter outfit.
    let landmarks = [
        ("idle_south", 0, 36, 36, 0x672115, true),
        ("idle_south", 0, 35, 36, 0xB65932, false),
        ("idle_south", 0, 33, 47, 0x672115, true),
        ("idle_south", 0, 37, 52, 0x672115, false),
        ("idle_south", 0, 40, 43, 0xA3D8DD, false),
        ("idle_east", 0, 44, 36, 0x672115, true),
        ("idle_east", 0, 35, 31, 0x672115, false),
        ("idle_east", 0, 38, 45, 0x672115, false),
        ("idle_east", 0, 35, 47, 0x672115, true),
        ("idle_east", 0, 40, 49, 0xFFFFFF, false),
        ("idle_north", 0, 38, 36, 0x672115, false),
        ("idle_north", 0, 35, 45, 0x672115, false),
        ("idle_north", 0, 33, 47, 0x672115, true),
        ("idle_north", 0, 40, 46, 0xDF8D4B, false),
        ("walk_east", 1, 46, 47, 0x672115, true),
        ("walk_east", 1, 32, 46, 0xFCD9B3, true),
        ("walk_east", 1, 42, 46, 0xB65932, false),
        ("walk_east", 1, 39, 51, 0x71A5B8, false),
        ("walk_east", 3, 36, 48, 0x672115, true),
        ("walk_east", 3, 41, 44, 0xA3D8DD, false),
        ("walk_south", 1, 34, 48, 0x672115, true),
        ("walk_south", 1, 45, 46, 0xD37A57, true),
        ("walk_south", 1, 35, 44, 0xFFFFFF, false),
        ("walk_south", 3, 45, 48, 0x672115, true),
        ("walk_south", 3, 40, 50, 0x4F748F, false),
        ("walk_north", 1, 34, 48, 0x672115, true),
        ("walk_north", 1, 45, 46, 0xD37A57, true),
        ("walk_north", 1, 39, 41, 0xFCD69E, false),
        ("walk_north", 3, 45, 48, 0x672115, true),
        ("walk_north", 3, 37, 52, 0xC8E1EE, false),
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
            ("idle_east", 37),
            ("idle_north", 12),
            ("idle_south", 40),
            ("walk_east", 154),
            ("walk_north", 40),
            ("walk_south", 158),
        ] {
            let prefix = "winter";
            let asset = format!(
                "assets/animations/NPCs/Celine/Sprites/Winter/spr_npc_celine_{prefix}_{name}.png"
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
        assert_eq!((changed, protected, frames), (441, 156, 15));
        if first_selection.is_empty() {
            first_selection = selection;
        } else {
            assert_eq!(first_selection, selection);
        }
    }
}
