use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{fs, process::Command};
#[test]
fn elsie_winter_seated_writing_package_preserve_native_metadata() {
    let elsie = [
        (
            "specialanimation_winter_write_sit_start_east",
            2,
            "0.125",
            "40.0",
        ),
        (
            "specialanimation_winter_write_sit_loop_east",
            4,
            "[0.1, 0.125, 0.1, 0.3]",
            "40.0",
        ),
        (
            "specialanimation_winter_write_sit_end_east",
            2,
            "[0.125, 0.1]",
            "40.0",
        ),
        (
            "specialanimation_winter_write_sit_start_south",
            2,
            "0.125",
            "40.0",
        ),
        (
            "specialanimation_winter_write_sit_loop_south",
            4,
            "[0.1, 0.125, 0.1, 0.3]",
            "40.0",
        ),
        (
            "specialanimation_winter_write_sit_end_south",
            2,
            "[0.125, 0.1]",
            "40.0",
        ),
    ];
    let (character, hotkey, expected_frames, cases) = ("elsie", "Z", 16, &elsie[..]);
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();
    for (name, frames, duration, horizontal) in cases {
        let stem = format!("spr_npc_{character}_{name}");
        let timing = if *frames == 1 {
            String::new()
        } else {
            format!("frame_len={frames}\nduration={duration}\n")
        };
        let meta = format!(
            "[meta_properties]\nid='synthetic'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[80,80]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal={horizontal}\nvertical=54.0\n"
        );
        for (directory, color) in [
            (&original, [10, 20, 30, 255]),
            (&modified, [40, 50, 60, 255]),
        ] {
            RgbaImage::from_pixel(frames * 80, 80, Rgba(color))
                .save(directory.join(format!("{stem}.png")))
                .unwrap();
            fs::write(directory.join(format!("{stem}.meta.toml")), &meta).unwrap();
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
        "{character}: {}",
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
    assert_eq!(table[0][0], character);
    assert_eq!(table[0][2], hotkey);
    assert_eq!(table[0][4].as_array().unwrap().len(), cases.len());
    let mut frames = 0;
    for row in table[0][4].as_array().unwrap() {
        let stem = row[0].as_str().unwrap();
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
    assert_eq!(frames, expected_frames);
}

#[test]
#[ignore = "requires the Elsie source corpus and retained finish-start comparison outputs"]
fn winter_seated_writing_recolors_faces_and_preserves_gloves_books_and_prior_art() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/elsie");
    let prior = root.join("local/authoring/finish-elsie");
    let set = std::env::var_os("FOM_ELSIE_FINISH_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/elsie-world-trial.json"));
    let profile: Value = serde_json::from_slice(
        &fs::read(root.join("palettes/profiles/elsie-world-trial.json")).unwrap(),
    )
    .unwrap();
    let old: Value =
        serde_json::from_slice(&fs::read(prior.join("prior-profile.json")).unwrap()).unwrap();
    assert_eq!(
        &profile["regions"].as_array().unwrap()[..158],
        old["regions"].as_array().unwrap()
    );
    assert_eq!(profile["source_colors"], old["source_colors"]);
    assert_eq!(profile["color_groups"], old["color_groups"]);

    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("bundle");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("build-presets")
        .arg("--original")
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
    let rgb = |n: u32| [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255];
    let source = [0xFBDCB9, 0xF3C193, 0xE08C6A, 0xA45838].map(rgb);
    let cases: [(&str, &[usize]); 6] = [
        ("end_east", &[33, 50]),
        ("end_south", &[40, 54]),
        ("loop_east", &[30, 37, 34, 38]),
        ("loop_south", &[37, 40, 40, 40]),
        ("start_east", &[50, 33]),
        ("start_south", &[54, 40]),
    ];
    let choices = [
        ("blue", [0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83]),
        ("npc_hayden", [0xE8B271, 0xCA9052, 0xB27146, 0x6E4922]),
        ("npc_ryis", [0xB06C57, 0x814A3A, 0x63342A, 0x491F1B]),
        ("npc_seridia", [0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48]),
    ];
    for (name, counts) in cases {
        let asset = format!(
            "assets/animations/NPCs/Elsie/Sprites/Winter/spr_npc_elsie_specialanimation_winter_write_sit_{name}.png"
        );
        let before = image::open(original.join(&asset)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), (80 * counts.len() as u32, 80));
        let mut observed = vec![0; counts.len()];
        for (x, y, p) in before.enumerate_pixels() {
            if source.contains(&p.0) {
                assert!(
                    (35..=44).contains(&(x % 80)) && (31..=40).contains(&y),
                    "skin shade outside the independently inspected face: {name} {x},{y}"
                );
                observed[x as usize / 80] += 1;
            }
        }
        assert_eq!(observed, counts);
        for (choice, target) in choices {
            let target = target.map(rgb);
            let after = image::open(output.join("variants").join(choice).join(&asset))
                .unwrap()
                .to_rgba8();
            assert_eq!(before.dimensions(), after.dimensions());
            for (x, y, p) in before.enumerate_pixels() {
                let expected = source
                    .iter()
                    .position(|c| c == &p.0)
                    .map_or(p.0, |i| target[i]);
                assert_eq!(after.get_pixel(x, y).0, expected, "{name} {choice} {x},{y}");
            }
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(output.join("variants").join(choice).join(&meta)).unwrap()
            );
        }
    }
    for region in old["regions"].as_array().unwrap() {
        let asset = region["asset"].as_str().unwrap();
        for (choice, _) in choices {
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(
                    fs::read(prior.join("prior-build/variants").join(choice).join(&path)).unwrap(),
                    fs::read(output.join("variants").join(choice).join(&path)).unwrap(),
                    "prior output changed: {choice} {path}"
                );
            }
        }
    }
}
