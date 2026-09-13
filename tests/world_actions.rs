use image::{Rgba, RgbaImage};
use std::{fs, process::Command};

#[test]
fn spring_actions_preserve_frame_timing_and_offsets() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();
    let cases = [
        ("spring_blink_south", 3, "[0.075,0.125,0.075]"),
        ("spring_blink_east", 3, "[0.075,0.125,0.075]"),
        ("spring_drink_north", 3, "1.0"),
        ("spring_drink_south", 3, "1.0"),
        ("spring_drink_east", 3, "1.0"),
        ("spring_eat_north", 3, "1.0"),
        ("spring_eat_south", 5, "[0.125,0.15,0.175,0.125,0.6]"),
        ("spring_eat_east", 5, "[0.125,0.15,0.175,0.125,0.6]"),
        ("spring_sit_north", 1, "1.0"),
        ("spring_sit_south", 1, "1.0"),
        ("spring_sit_east", 1, "1.0"),
        (
            "spring_action_north",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "spring_action_south",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        ("spring_action_east", 7, "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]"),
        ("spring_shocked_start_south", 1, "1.0"),
        ("spring_shocked_loop_south", 1, "1.0"),
        ("spring_shocked_end_south", 1, "1.0"),
        ("spring_sleep_east", 1, "1.0"),
        ("spring_kiss_east", 4, "[0.15,0.15,0.8,0.15]"),
        ("specialanimation_spring_write_start_south", 2, "0.125"),
        (
            "specialanimation_spring_write_loop_south",
            4,
            "[0.1,0.125,0.1,0.3]",
        ),
        ("specialanimation_spring_write_end_south", 2, "[0.125,0.1]"),
        ("specialanimation_spring_write_sit_start_south", 2, "0.125"),
        (
            "specialanimation_spring_write_sit_loop_south",
            4,
            "[0.1,0.125,0.1,0.3]",
        ),
        (
            "specialanimation_spring_write_sit_end_south",
            2,
            "[0.125,0.1]",
        ),
        ("specialanimation_spring_read_sit_start_south", 3, "0.1"),
        (
            "specialanimation_spring_read_sit_loop_south",
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        ("specialanimation_spring_read_sit_end_south", 3, "0.1"),
        ("specialanimation_spring_work_sit_start_south", 1, "1.0"),
        (
            "specialanimation_spring_work_sit_loop_south",
            7,
            "[0.5,0.25,0.25,0.25,0.25,0.25,0.25]",
        ),
        (
            "specialanimation_spring_work_sit_end_south",
            2,
            "[0.25,0.175]",
        ),
        ("specialanimation_spring_think_start_south", 2, "0.1"),
        ("specialanimation_spring_think_loop_south", 1, "1.0"),
        ("specialanimation_spring_think_end_south", 1, "1.0"),
        (
            "specialanimation_spring_finger_snap_south",
            5,
            "[0.1,0.25,0.1,0.75,0.1]",
        ),
        (
            "specialanimation_spring_faint_south",
            7,
            "[0.2,0.1,0.15,0.8,0.1,0.1,1.6]",
        ),
    ];
    for (cycle, count, duration) in cases {
        let name = format!("spr_npc_adeline_{cycle}");
        let timing = if count == 1 {
            String::new()
        } else {
            format!("frame_len={count}\nduration={duration}\n")
        };
        let metadata = format!(
            "[meta_properties]\nid='original-id'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[2,3]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal='Middle'\nvertical=54.0\n"
        );
        for (root, color) in [
            (&original, [10, 20, 30, 255]),
            (&modified, [40, 50, 60, 255]),
        ] {
            RgbaImage::from_pixel(2 * count, 3, Rgba(color))
                .save(root.join(format!("{name}.png")))
                .unwrap();
            fs::write(root.join(format!("{name}.meta.toml")), &metadata).unwrap();
        }
    }
    let output = temp.path().join("package");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("package-toggle")
        .arg("--original")
        .arg(&original)
        .arg("--modified")
        .arg(&modified)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let mut expected = Vec::new();
    for (cycle, _, _) in cases {
        let name = format!("spr_npc_adeline_{cycle}");
        let target = format!("spr_lns_npc_adeline_{cycle}_blue");
        let before: toml::Value = toml::from_str(
            &fs::read_to_string(original.join(format!("{name}.meta.toml"))).unwrap(),
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
        assert!(after["meta_properties"].get("id").is_none());
        assert_eq!(
            fs::read(modified.join(format!("{name}.png"))).unwrap(),
            fs::read(output.join(format!("animations/LightningAndSun/{target}.png"))).unwrap()
        );
        expected.push(vec![name, target]);
    }
    expected.sort();
    let script = fs::read_to_string(output.join("gml/palette_assets.gml")).unwrap();
    let table: serde_json::Value = serde_json::from_str(
        script
            .split_once("return ")
            .unwrap()
            .1
            .split_once("; }")
            .unwrap()
            .0,
    )
    .unwrap();
    assert_eq!(table[0][4], serde_json::json!(expected));

    // These directional strips do not exist; special-animation names still
    // require an exact reviewed registry entry.
    for cycle in [
        "spring_blink_north",
        "spring_sit_west",
        "spring_action_west",
        "spring_shocked_start_east",
        "spring_sleep_north",
        "spring_kiss_south",
        "specialanimation_spring_write_start_east",
        "specialanimation_spring_faint_west",
    ] {
        let source = temp.path().join(format!("unsupported-{cycle}"));
        let recolor = temp.path().join(format!("recolor-{cycle}"));
        fs::create_dir(&source).unwrap();
        fs::create_dir(&recolor).unwrap();
        for (from, to) in [(&original, &source), (&modified, &recolor)] {
            for extension in ["png", "meta.toml"] {
                fs::copy(
                    from.join(format!("spr_npc_adeline_spring_sit_south.{extension}")),
                    to.join(format!("spr_npc_adeline_{cycle}.{extension}")),
                )
                .unwrap();
            }
        }
        let rejected = temp.path().join(format!("rejected-{cycle}"));
        let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .arg("package-toggle")
            .arg("--original")
            .arg(source)
            .arg("--modified")
            .arg(recolor)
            .arg("--output")
            .arg(&rejected)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("Unsupported animation"));
        assert!(!rejected.exists());
    }
}

#[test]
#[ignore = "requires the 168 local animations in extracted/adeline-world-actions-study"]
fn spring_action_masks_cover_hands_and_preserve_clothing_and_mouth_colors() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/adeline-world-actions-study");
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("blue");
    let recipe = std::env::var_os("FOM_ACTION_RECIPE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/stylized/adeline-world-actions.json"));
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("apply")
        .arg("--input")
        .arg(&original)
        .arg("--palette")
        .arg(recipe)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["files"].as_array().unwrap().len(), 168);
    let sources = [
        [233, 169, 128, 255],
        [222, 143, 93, 255],
        [186, 106, 76, 255],
        [125, 59, 20, 255],
    ];
    let blue = [
        [157, 185, 212, 255],
        [127, 159, 189, 255],
        [102, 135, 173, 255],
        [68, 95, 131, 255],
    ];
    let (mut strips, mut frames, mut changed) = (0, 0, 0);
    for row in report["files"].as_array().unwrap() {
        let name = row["path"].as_str().unwrap();
        if !name.contains("/spr_npc_adeline_spring_")
            || !["_sit_", "_eat_", "_drink_", "_blink_"]
                .iter()
                .any(|cycle| name.contains(cycle))
        {
            continue;
        }
        let before = image::open(original.join(name)).unwrap().to_rgba8();
        let after = image::open(output.join(name)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), after.dimensions());
        let meta = name.replace(".png", ".meta.toml");
        assert_eq!(
            fs::read(original.join(&meta)).unwrap(),
            fs::read(output.join(meta)).unwrap()
        );
        strips += 1;
        frames += before.width() / 80;
        for (x, y, p) in before.enumerate_pixels() {
            let q = after.get_pixel(x, y);
            assert_eq!(p[3], q[3]);
            let shade = sources.iter().position(|color| color == &p.0);
            // The lowered hand in south eating frame 1 reaches y=49.
            let low_hand = name.ends_with("eat_south.png")
                && x / 80 == 1
                && (36..=38).contains(&(x % 80))
                && y == 49;
            if shade.is_none() || (y >= 49 && !low_hand) || (name.ends_with("north.png") && y >= 47)
            {
                assert_eq!(p, q, "changed clothing/other color: {name} [{x},{y}]");
            }
            if y < 44
                && let Some(shade) = shade
            {
                assert_eq!(
                    q.0, blue[shade],
                    "missed head/raised hand: {name} [{x},{y}]"
                );
            }
            if p != q {
                changed += 1;
            }
        }
        for frame in 0..before.width() / 80 {
            let landmarks: Vec<(u32, u32)> = if name.ends_with("north.png") {
                if frame == 1 {
                    vec![(33, 46)]
                } else {
                    vec![(34, 45), (45, 45)]
                }
            } else if name.ends_with("south.png") {
                vec![(
                    44,
                    if name.contains("eat_") && (frame == 1 || frame == 3) {
                        45
                    } else {
                        44
                    },
                )]
            } else if name.contains("blink_") {
                vec![(35, 44), (45, 46)]
            } else if name.contains("sit_") {
                vec![(36, 44)]
            } else {
                Vec::new()
            };
            for (x, y) in landmarks {
                let x = x + frame * 80;
                let shade = sources
                    .iter()
                    .position(|color| color == &before.get_pixel(x, y).0)
                    .unwrap();
                assert_eq!(
                    after.get_pixel(x, y).0,
                    blue[shade],
                    "missed hand: {name} [{x},{y}]"
                );
            }
        }
    }
    assert_eq!((strips, frames), (11, 31));
    eprintln!("Verified {strips} action strips / {frames} frames; {changed} skin pixels changed");
}

#[test]
#[ignore = "requires the 168 local animations in extracted/adeline-world-actions-study"]
fn spring_standard_and_special_masks_preserve_material_boundaries() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/adeline-world-actions-study");
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("presets");
    let presets = std::env::var_os("FOM_STANDARD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/adeline-world-actions-trial.json"));
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["build-presets", "--original"])
        .arg(&original)
        .arg("--presets")
        .arg(presets)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let cases = [
        ("spring_action_north", 7),
        ("spring_action_south", 7),
        ("spring_action_east", 7),
        ("spring_shocked_start_south", 1),
        ("spring_shocked_loop_south", 1),
        ("spring_shocked_end_south", 1),
        ("spring_sleep_east", 1),
        ("spring_kiss_east", 4),
        ("specialanimation_spring_write_start_south", 2),
        ("specialanimation_spring_write_loop_south", 4),
        ("specialanimation_spring_write_end_south", 2),
        ("specialanimation_spring_write_sit_start_south", 2),
        ("specialanimation_spring_write_sit_loop_south", 4),
        ("specialanimation_spring_write_sit_end_south", 2),
        ("specialanimation_spring_read_sit_start_south", 3),
        ("specialanimation_spring_read_sit_loop_south", 4),
        ("specialanimation_spring_read_sit_end_south", 3),
        ("specialanimation_spring_work_sit_start_south", 1),
        ("specialanimation_spring_work_sit_loop_south", 7),
        ("specialanimation_spring_work_sit_end_south", 2),
        ("specialanimation_spring_think_start_south", 2),
        ("specialanimation_spring_think_loop_south", 1),
        ("specialanimation_spring_think_end_south", 1),
        ("specialanimation_spring_finger_snap_south", 5),
        ("specialanimation_spring_faint_south", 7),
    ];
    let sources = [0xE9A980, 0xDE8F5D, 0xBA6A4C, 0x7D3B14];
    let targets = [
        ("blue", [0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83]),
        ("npc_hayden", [0xE8B271, 0xCA9052, 0xB27146, 0x6E4922]),
        ("npc_ryis", [0xB06C57, 0x814A3A, 0x63342A, 0x491F1B]),
        ("npc_seridia", [0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    // Source-art landmarks, independent of the connected-component seeds.
    let landmarks = [
        ("spring_action_north", 32, 44, 0xE9A980, true),
        ("spring_action_north", 34, 45, 0xE9A980, true),
        ("spring_action_north", 33, 45, 0x7D3B14, true),
        ("spring_action_north", 39, 47, 0xBA6A4C, false),
        ("spring_action_north", 37, 52, 0xBA6A4C, false),
        ("spring_shocked_loop_south", 31, 33, 0xE9A980, true),
        ("spring_shocked_loop_south", 48, 33, 0xE9A980, true),
        ("spring_shocked_loop_south", 39, 36, 0x410808, false),
        ("spring_shocked_loop_south", 39, 37, 0x9E2626, false),
        ("spring_action_south", 39, 36, 0xE9A980, true),
        ("spring_action_south", 115, 45, 0xE9A980, true), // frame 1 hand
        ("spring_action_south", 198, 47, 0x7D3B14, true), // frame 2 inward hand
        ("spring_action_south", 526, 47, 0x7D3B14, true), // frame 6 far hand
        ("spring_action_east", 41, 36, 0xE9A980, true),
        ("spring_action_east", 44, 46, 0xBA6A4C, false), // gold trim
        ("spring_action_east", 44, 36, 0xA59DA2, false), // eye fringe
        ("spring_action_east", 128, 43, 0xE9A980, true), // frame 1 extended finger
        ("spring_action_east", 205, 46, 0x7D3B14, true), // frame 2 hand crease
        ("spring_action_east", 525, 46, 0xDE8F5D, true), // frame 6 isolated far hand
        ("spring_shocked_start_south", 33, 48, 0x7D3B14, true),
        ("spring_shocked_end_south", 46, 48, 0x7D3B14, true),
        ("spring_sleep_east", 43, 40, 0xE9A980, true), // hand below chin
        ("spring_sleep_east", 40, 45, 0xBA6A4C, false), // cape trim
        ("spring_kiss_east", 35, 48, 0x7D3B14, true),  // frame 0 hand
        ("spring_kiss_east", 125, 38, 0x7D3B14, true), // frame 1 face edge
        ("spring_kiss_east", 196, 46, 0x7D3B14, true), // frame 2 hand crease
        ("spring_kiss_east", 203, 35, 0xE9A980, true), // frame 2 cheek
        ("spring_kiss_east", 276, 48, 0x7D3B14, true), // frame 3 hand
        (
            "specialanimation_spring_write_start_south",
            46,
            49,
            0xDE8F5D,
            true,
        ), // standing writing supporting finger
        (
            "specialanimation_spring_write_loop_south",
            43,
            43,
            0xC5895C,
            false,
        ), // writing board light wood
        (
            "specialanimation_spring_write_loop_south",
            274,
            44,
            0xBA6A4C,
            true,
        ), // last standing writing frame forearm edge
        (
            "specialanimation_spring_write_sit_start_south",
            123,
            49,
            0xDE8F5D,
            true,
        ), // seated writing supporting fingers
        (
            "specialanimation_spring_write_sit_loop_south",
            274,
            47,
            0xDE8F5D,
            true,
        ), // last seated writing frame fingers
        (
            "specialanimation_spring_read_sit_start_south",
            195,
            47,
            0x7D3B14,
            true,
        ), // opening book left finger edge
        (
            "specialanimation_spring_read_sit_start_south",
            196,
            41,
            0xC9AF9C,
            false,
        ), // paper shadow
        (
            "specialanimation_spring_read_sit_end_south",
            114,
            45,
            0xBA6A4C,
            true,
        ), // closing book hand shade
        (
            "specialanimation_spring_work_sit_loop_south",
            515,
            48,
            0x7D3B14,
            true,
        ), // last working frame left hand tip
        (
            "specialanimation_spring_work_sit_loop_south",
            521,
            48,
            0x7D3B14,
            true,
        ), // last working frame right hand tip
        (
            "specialanimation_spring_think_start_south",
            124,
            41,
            0x7D3B14,
            true,
        ), // thinking raised finger crease
        (
            "specialanimation_spring_think_end_south",
            35,
            47,
            0x7D3B14,
            true,
        ), // thinking end lowered hand edge
        (
            "specialanimation_spring_finger_snap_south",
            117,
            47,
            0xDE8F5D,
            true,
        ), // finger snap lowered fingertip light
        (
            "specialanimation_spring_finger_snap_south",
            117,
            48,
            0xBA6A4C,
            true,
        ), // finger snap lowered fingertip shade
        (
            "specialanimation_spring_finger_snap_south",
            118,
            45,
            0xBA6A4C,
            false,
        ), // finger snap adjacent chest trim
        (
            "specialanimation_spring_finger_snap_south",
            288,
            40,
            0x7D3B14,
            true,
        ), // finger snap release fingertip
        (
            "specialanimation_spring_faint_south",
            32,
            46,
            0x7D3B14,
            true,
        ), // faint first frame left hand contour
        (
            "specialanimation_spring_faint_south",
            514,
            51,
            0x7D3B14,
            true,
        ), // faint final frame left fingers
        (
            "specialanimation_spring_faint_south",
            525,
            51,
            0x7D3B14,
            true,
        ), // faint final frame right fingers
        (
            "specialanimation_spring_faint_south",
            518,
            49,
            0xBA6A4C,
            false,
        ), // faint final frame chest trim
    ];
    let mut blue_selection = Vec::new();
    for (target_index, (id, target)) in targets.iter().enumerate() {
        let mut selection = Vec::new();
        let mut frames = 0;
        for (cycle, count) in cases {
            let name = format!(
                "assets/animations/NPCs/Adeline/Sprites/Spring/spr_npc_adeline_{cycle}.png"
            );
            let before = image::open(original.join(&name)).unwrap().to_rgba8();
            let modified = output.join("variants").join(id);
            let after = image::open(modified.join(&name)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), (80 * count, 80));
            assert_eq!(after.dimensions(), before.dimensions());
            let meta = name.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(modified.join(&meta)).unwrap()
            );
            let mut changed_per_frame = vec![0; count as usize];
            for (x, y, p) in before.enumerate_pixels() {
                let q = after.get_pixel(x, y);
                assert_eq!(p[3], q[3], "alpha: {id} {cycle} [{x},{y}]");
                if p != q {
                    let shade = sources
                        .iter()
                        .position(|c| rgba(*c) == p.0)
                        .expect("changed a color outside the reviewed skin ramp");
                    assert_eq!(q.0, rgba(target[shade]), "{id} {cycle} [{x},{y}]");
                    changed_per_frame[(x / 80) as usize] += 1;
                }
                selection.push(p != q);
            }
            assert!(changed_per_frame.iter().all(|n| *n > 0), "{id} {cycle}");
            for &(landmark_cycle, x, y, color, skin) in &landmarks {
                if landmark_cycle != cycle {
                    continue;
                }
                assert_eq!(before.get_pixel(x, y).0, rgba(color), "source landmark");
                let expected = if skin {
                    target[sources.iter().position(|c| *c == color).unwrap()]
                } else {
                    color
                };
                assert_eq!(
                    after.get_pixel(x, y).0,
                    rgba(expected),
                    "material boundary: {id} {cycle} [{x},{y}] skin={skin}"
                );
            }
            frames += count;
        }
        assert_eq!(frames, 81);
        if target_index == 0 {
            blue_selection = selection;
        } else {
            assert_eq!(selection, blue_selection, "{id} changed the skin selection");
        }
    }
}
