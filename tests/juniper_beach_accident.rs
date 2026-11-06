use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, process::Command};

const ACTIONS: [(&str, u32, &str); 6] = [
    ("blink_east", 3, "[0.075,0.125,0.075]"),
    ("blink_south", 3, "[0.075,0.125,0.075]"),
    ("idle_east", 1, ""),
    ("idle_north", 1, ""),
    ("idle_south", 1, ""),
    (
        "transform_south",
        22,
        "[0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,1.0]",
    ),
];

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn finish_profile_retains_every_prior_region_and_palette_role() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let before = read_json(root.join("palettes/profiles/juniper-world-trial.json"));
    let after = read_json(root.join("palettes/profiles/juniper-world-finish.json"));
    assert_eq!(after["source_colors"], before["source_colors"]);
    assert_eq!(after["color_groups"], before["color_groups"]);
    let regions = after["regions"].as_array().unwrap();
    assert_eq!(&regions[..305], before["regions"].as_array().unwrap());
    assert_eq!(regions.len(), 311);
    let expected: BTreeSet<_> = ACTIONS.iter().map(|(action,_,_)|format!("assets/animations/NPCs/Juniper/Sprites/Beach Accident/spr_npc_juniper_specialanimation_beach_accident_{action}.png")).collect();
    let actual: BTreeSet<_> = regions[305..]
        .iter()
        .map(|r| r["asset"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(actual, expected);
    let old_set = read_json(root.join("palettes/sets/juniper-world-trial.json"));
    let new_set = read_json(root.join("palettes/sets/juniper-world-finish-trial.json"));
    assert_eq!(new_set["presets"], old_set["presets"]);
    assert!(
        regions[305..]
            .iter()
            .all(|r| !r["seeds"].as_array().unwrap().is_empty())
    );
}

#[test]
fn beach_accident_packages_preserve_native_frames_timing_offsets_and_atlas() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();
    for (action, frames, duration) in ACTIONS {
        let stem = format!("spr_npc_juniper_specialanimation_beach_accident_{action}");
        let timing = if frames == 1 {
            String::new()
        } else {
            format!("frame_len={frames}\nduration={duration}\n")
        };
        let metadata = format!(
            "[meta_properties]\nid='synthetic'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[80,80]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal=40.0\nvertical=54.0\n"
        );
        for (directory, color) in [
            (&original, [10, 20, 30, 255]),
            (&modified, [40, 50, 60, 255]),
        ] {
            RgbaImage::from_pixel(frames * 80, 80, Rgba(color))
                .save(directory.join(format!("{stem}.png")))
                .unwrap();
            fs::write(directory.join(format!("{stem}.meta.toml")), &metadata).unwrap();
        }
    }
    let output = temp.path().join("package");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["package-toggle", "--original"])
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
    assert_eq!(table[0][0], "juniper");
    assert_eq!(table[0][4].as_array().unwrap().len(), 6);
    let mut frames = 0;
    for row in table[0][4].as_array().unwrap() {
        let stem = row[0].as_str().unwrap();
        assert!(!stem.contains("west"));
        let target = row[1].as_str().unwrap();
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
        assert_eq!(
            before["asset_properties"], after["asset_properties"],
            "{stem}"
        );
        frames += before["asset_properties"]
            .get("frame_len")
            .and_then(toml::Value::as_integer)
            .unwrap_or(1);
        assert_eq!(
            fs::read(modified.join(format!("{stem}.png"))).unwrap(),
            fs::read(output.join(format!("animations/LightningAndSun/{target}.png"))).unwrap()
        );
    }
    assert_eq!(frames, 31);
}

fn bangle(name: &str, frame: u32, x: u32, y: u32) -> bool {
    matches!(
        (name, frame, x, y),
        ("blink_east" | "idle_east", _, 34, 45)
            | ("blink_south" | "idle_south", _, 33, 45)
            | ("idle_north", _, 46, 45)
            | ("transform_south", 0 | 1, 33, 46)
            | ("transform_south", 17..=21, 33 | 35 | 44 | 46, 45)
    )
}

#[test]
#[ignore = "requires the complete Juniper source corpus; checks all world pixels and retained prior_assets"]
fn every_world_pixel_matches_reviewed_materials_and_prior_assets_remain_identical() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/juniper-beach-accident-finish-study");
    let set = std::env::var_os("FOM_JUNIPER_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/juniper-world-finish-trial.json"));
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("bundle");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["build-presets", "--original"])
        .arg(&original)
        .arg("--presets")
        .arg(set)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let cases: [(&str, &[usize]); 6] = [
        ("blink_east", &[39, 41, 39]),
        ("blink_south", &[45, 47, 45]),
        ("idle_east", &[39]),
        ("idle_north", &[36]),
        ("idle_south", &[45]),
        (
            "transform_south",
            &[
                47, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 72, 80, 82, 82,
            ],
        ),
    ];
    let rgb = |n: u32| [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255];
    let skin = [0xEFD89A, 0xE3BF7F, 0xBC8B43, 0x763F21].map(rgb);
    let choices = [
        ("blue", [0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83]),
        ("npc_hayden", [0xE8B271, 0xCA9052, 0xB27146, 0x6E4922]),
        ("npc_ryis", [0xB06C57, 0x814A3A, 0x63342A, 0x491F1B]),
        ("npc_seridia", [0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48]),
    ];
    let mut selected = 0;
    let mut protected = 0;
    for (name, counts) in cases {
        let asset = format!(
            "assets/animations/NPCs/Juniper/Sprites/Beach Accident/spr_npc_juniper_specialanimation_beach_accident_{name}.png"
        );
        let before = image::open(original.join(&asset)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (80 * counts.len() as u32, 80));
        let mut observed = vec![0; counts.len()];
        for (x, y, p) in before.enumerate_pixels() {
            if skin.contains(&p.0) {
                if bangle(name, x / 80, x % 80, y) {
                    assert_eq!(p.0, rgb(0xBC8B43));
                    protected += 1;
                } else {
                    assert!(
                        (32..=47).contains(&(x % 80)) && (33..=54).contains(&y),
                        "unexpected skin material {name} {x},{y}"
                    );
                    observed[x as usize / 80] += 1;
                }
            }
        }
        assert_eq!(observed, counts, "{name}");
        selected += counts.iter().sum::<usize>();
        for (choice, target) in choices {
            let target = target.map(rgb);
            let after = image::open(output.join("variants").join(choice).join(&asset))
                .unwrap()
                .to_rgba8();
            assert_eq!(before.dimensions(), after.dimensions());
            for (x, y, p) in before.enumerate_pixels() {
                let expected = if bangle(name, x / 80, x % 80, y) {
                    p.0
                } else {
                    skin.iter()
                        .position(|c| c == &p.0)
                        .map_or(p.0, |i| target[i])
                };
                assert_eq!(after.get_pixel(x, y).0, expected, "{name} {choice} {x},{y}");
            }
            let landmarks = [
                ("idle_south", 0, 40, 34, 0xEFD89A, true),
                ("idle_south", 0, 38, 34, 0x93A58C, false),
                ("idle_south", 0, 33, 45, 0xBC8B43, false),
                ("idle_south", 0, 34, 45, 0xFFF45D, false),
                ("idle_south", 0, 35, 27, 0xDF8D4B, false),
                ("idle_east", 0, 36, 36, 0xEFD89A, true),
                ("idle_east", 0, 45, 46, 0x93A58C, false),
                ("idle_east", 0, 40, 45, 0xEFD89A, true),
                ("transform_south", 17, 39, 34, 0xEFD89A, true),
                ("transform_south", 17, 36, 38, 0xBE8BFF, false),
                ("transform_south", 15, 39, 34, 0xFBF4D3, false),
                ("transform_south", 2, 39, 35, 0xB3D5B9, false),
                ("transform_south", 9, 39, 35, 0xFAFBFA, false),
            ];
            for (case, frame, x, y, color, changes) in landmarks {
                if case != name {
                    continue;
                }
                let source = rgb(color);
                assert_eq!(
                    before.get_pixel(frame * 80 + x, y).0,
                    source,
                    "source landmark {name} {frame} {x},{y}"
                );
                let expected = if changes {
                    target[skin.iter().position(|p| p == &source).unwrap()]
                } else {
                    source
                };
                assert_eq!(
                    after.get_pixel(frame * 80 + x, y).0,
                    expected,
                    "material landmark {name} {choice} {frame} {x},{y}"
                );
            }
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(output.join("variants").join(choice).join(&meta)).unwrap()
            );
        }
    }
    assert_eq!(selected, 847);
    assert_eq!(protected, 31);
    let prior_assets = read_json(root.join("palettes/profiles/juniper-world-trial.json"));
    let prior = temp.path().join("prior");
    fs::create_dir(&prior).unwrap();
    for region in prior_assets["regions"].as_array().unwrap() {
        let asset = region["asset"].as_str().unwrap();
        for asset in [asset.to_string(), asset.replace(".png", ".meta.toml")] {
            let to = prior.join(&asset);
            fs::create_dir_all(to.parent().unwrap()).unwrap();
            fs::copy(original.join(&asset), to).unwrap();
        }
    }
    let prior_output = temp.path().join("prior-bundle");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["build-presets", "--original"])
        .arg(prior)
        .arg("--presets")
        .arg(root.join("palettes/sets/juniper-world-trial.json"))
        .arg("--output")
        .arg(&prior_output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for (choice, _) in choices {
        for region in prior_assets["regions"].as_array().unwrap() {
            let asset = region["asset"].as_str().unwrap();
            for asset in [asset.to_string(), asset.replace(".png", ".meta.toml")] {
                let path = Path::new("variants").join(choice).join(asset);
                assert_eq!(
                    fs::read(output.join(&path)).unwrap(),
                    fs::read(prior_output.join(path)).unwrap()
                );
            }
        }
    }
}
