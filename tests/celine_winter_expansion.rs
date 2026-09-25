use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires the 314 local animations in extracted/celine-winter-standard-study"]
fn celine_autumn_specials_cover_fingers_and_preserve_books_tools_and_clothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/celine-winter-standard-study");
    let profile = std::env::var_os("FOM_CELINE_WINTER_EXPANSION_PROFILE")
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
    // Source-art points distinguish fine Autumn face contours and moving fingers
    // from shared dark hair, belt, footwear and garment colors beside the tools.
    let landmarks = [
        ("book_stand_start_south", 0, 37, 45, 0x672115, false),
        ("book_stand_start_south", 1, 35, 47, 0x672115, true),
        ("book_stand_start_south", 1, 39, 41, 0xF6E4D7, false),
        ("book_stand_loop_south", 0, 44, 36, 0x672115, true),
        ("book_stand_loop_south", 0, 35, 32, 0xDF8D4B, false),
        ("book_stand_loop_south", 0, 32, 42, 0xF6E4D7, false),
        ("book_stand_loop_south", 2, 38, 40, 0x672115, true),
        ("book_stand_end_south", 0, 44, 47, 0x672115, true),
        ("book_stand_end_south", 2, 37, 45, 0x672115, false),
        ("book_sit_start_south", 0, 34, 47, 0x672115, true),
        ("book_sit_start_south", 0, 37, 45, 0x535B8B, false),
        ("book_sit_start_south", 1, 44, 47, 0x672115, true),
        ("book_sit_start_south", 1, 39, 41, 0xF6E4D7, false),
        ("book_sit_loop_south", 0, 41, 40, 0x672115, true),
        ("book_sit_loop_south", 0, 32, 42, 0xF6E4D7, false),
        ("book_sit_loop_south", 2, 35, 36, 0x672115, true),
        ("book_sit_end_south", 0, 35, 47, 0x672115, true),
        ("book_sit_end_south", 2, 42, 45, 0x535B8B, false),
        ("sweep_start_south", 0, 33, 48, 0x672115, true),
        ("sweep_start_south", 0, 37, 46, 0x672115, false),
        ("sweep_end_south", 0, 46, 48, 0x672115, true),
        ("sweep_end_south", 0, 42, 46, 0x672115, false),
        ("sweep_loop_south", 0, 39, 48, 0x672115, true),
        ("sweep_loop_south", 0, 44, 47, 0xBB8151, false),
        ("sweep_loop_south", 1, 39, 46, 0x672115, false),
        ("sweep_loop_south", 1, 44, 47, 0xFCD9B3, true),
        ("sweep_loop_south", 3, 38, 35, 0x672115, true),
        ("sweep_loop_south", 12, 40, 47, 0x672115, true),
        ("sweep_loop_south", 13, 36, 47, 0x672115, true),
        ("sweep_loop_south", 14, 37, 45, 0x672115, false),
        ("water_east", 0, 34, 46, 0x672115, false),
        ("water_east", 0, 46, 44, 0xFFCF36, false),
        ("water_east", 1, 38, 41, 0x672115, false),
        ("water_east", 1, 47, 39, 0x672115, true),
        ("water_east", 1, 45, 43, 0x672115, false),
        ("water_east", 1, 43, 52, 0x672115, false),
        ("water_east", 3, 43, 43, 0xDD9D3E, false),
        ("water_east", 0, 44, 37, 0x672115, true),
        ("water_east", 1, 47, 36, 0x672115, true),
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
            ("book_sit_end_south", 95),
            ("book_sit_loop_south", 136),
            ("book_sit_start_south", 95),
            ("book_stand_end_south", 98),
            ("book_stand_loop_south", 136),
            ("book_stand_start_south", 98),
            ("sweep_end_south", 50),
            ("sweep_loop_south", 738),
            ("sweep_start_south", 40),
            ("water_east", 137),
        ] {
            let prefix = "specialanimation_autumn";
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
        assert_eq!((changed, protected, frames), (1623, 141, 41));
        if first_selection.is_empty() {
            first_selection = selection;
        } else {
            assert_eq!(first_selection, selection);
        }
    }
}
