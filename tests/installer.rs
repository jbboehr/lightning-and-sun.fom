#![cfg(all(target_os = "linux", target_arch = "x86_64"))]
use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Cursor, Read, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use zip::{ZipWriter, write::SimpleFileOptions};

const SOURCE: &str =
    "assets/animations/NPCs/Adeline/Portraits/Spring/spr_portrait_adeline_spring_neutral";
const VARIANT: &str = "assets/animations/LightningAndSun/spr_lns_adeline_spring_neutral_blue";
const PALETTE_SCRIPTS: &str = "assets/gml/scripts/lightning_and_sun_adeline_palette_toggle_study";
const META: &str = "[meta_properties]\nid = '0000000000000001'\nasset_kind = 'Animation'\n[asset_properties]\nframe_size = [2,1]\nframe_len = 2\nduration = 0.2\natlas = 'PortraitsSpring'\n";
fn asset_script(expressions: &[&str]) -> Vec<u8> {
    let pairs: Vec<_> = expressions
        .iter()
        .map(|e| {
            [
                format!("spr_portrait_adeline_spring_{e}"),
                format!("spr_lns_adeline_spring_{e}_blue"),
            ]
        })
        .collect();
    format!("// Generated from the portraits included in this local package.\nfunction lns_palette_assets() {{ return {}; }}\nfunction lns_palette_names() {{ return [\"Vanilla\",\"Debug Blue\"]; }}\n",serde_json::to_string(&pairs).unwrap()).into_bytes()
}

fn png(image: RgbaImage) -> Vec<u8> {
    let mut bytes = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(image)
        .write_to(&mut bytes, ImageFormat::Png)
        .unwrap();
    bytes.into_inner()
}
fn zip(entries: Vec<(String, Vec<u8>)>) -> Vec<u8> {
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    for (name, bytes) in entries {
        zip.start_file(name, SimpleFileOptions::default()).unwrap();
        zip.write_all(&bytes).unwrap();
    }
    zip.finish().unwrap().into_inner()
}

fn with_zip_entry(bytes: &[u8], name: &str, contents: &[u8]) -> Vec<u8> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut entries = Vec::new();
    for index in 0..archive.len() {
        let mut file = archive.by_index(index).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        entries.push((file.name().to_owned(), data));
    }
    entries.push((name.to_owned(), contents.to_vec()));
    zip(entries)
}

fn replace_zip_entry(bytes: &[u8], name: &str, contents: &[u8]) -> Vec<u8> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut entries = Vec::new();
    let mut replaced = false;
    for index in 0..archive.len() {
        let mut file = archive.by_index(index).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        if file.name() == name {
            data = contents.to_vec();
            replaced = true;
        }
        entries.push((file.name().to_owned(), data));
    }
    assert!(replaced, "fixture archive did not contain {name}");
    zip(entries)
}

fn without_zip_entry(bytes: &[u8], name: &str) -> Vec<u8> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut entries = Vec::new();
    for index in 0..archive.len() {
        let mut file = archive.by_index(index).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        if file.name() != name {
            entries.push((file.name().to_owned(), data));
        }
    }
    zip(entries)
}

struct Lab {
    _temp: tempfile::TempDir,
    game: PathBuf,
    runner: PathBuf,
    result: PathBuf,
    before: Vec<u8>,
}
impl Lab {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let game = temp.path().join("game with spaces");
        fs::create_dir_all(game.join("mods/other")).unwrap();
        fs::write(game.join("Maybe.toml"), "synthetic game\n").unwrap();
        fs::write(game.join("mods/other/keep.txt"), "another mod's source").unwrap();
        fs::write(game.join("mods/other/manifest.toml"), "name = 'Other'\nversion = '1.0.0'\nauthor = 'Test'\nminInstallerVersion = '0.15.10'\nmanifestVersion = '2'\n").unwrap();
        let original = RgbaImage::from_pixel(4, 1, Rgba([227, 161, 123, 255]));
        let blue = RgbaImage::from_pixel(4, 1, Rgba([157, 185, 212, 255]));
        let mut atlas = RgbaImage::new(4, 2);
        image::imageops::replace(&mut atlas, &original, 0, 0);
        image::imageops::replace(&mut atlas, &blue, 0, 1);
        let mut entries = vec![
            (format!("{SOURCE}.png"), png(original.clone())),
            (format!("{SOURCE}.meta.toml"), META.as_bytes().to_vec()),
            (
                "other-mod.txt".into(),
                b"another mod's installed data".to_vec(),
            ),
        ];
        let before = zip(entries.clone());
        fs::write(game.join("assets.zip"), &before).unwrap();
        entries.extend([
            ("manifest.toml".into(), Vec::new()),
            (
                format!("{VARIANT}.meta.toml"),
                META.replace("0000000000000001", "0000000000000002")
                    .into_bytes(),
            ),
            (
                format!("{PALETTE_SCRIPTS}/palette_toggle.gml"),
                include_bytes!("../mod/toggle/gml/palette_toggle.gml").to_vec(),
            ),
            (
                format!("{PALETTE_SCRIPTS}/palette_assets.gml"),
                asset_script(&["neutral"]),
            ),
            ("assets/atlases/PortraitsSpringAtlas.png".into(), png(atlas)),
            (
                "assets/atlases/PortraitsSpringAtlas.meta.toml".into(),
                br#"[asset_properties]
animations = [
  {texture_ids = ["0000000000000001::0"], placement = [0,0,2,1,2,1,0,0]},
  {texture_ids = ["0000000000000001::1"], placement = [2,0,2,1,2,1,0,0]},
  {texture_ids = ["0000000000000002::0"], placement = [0,1,2,1,2,1,0,0]},
  {texture_ids = ["0000000000000002::1"], placement = [2,1,2,1,2,1,0,0]},
]
"#
                .to_vec(),
            ),
        ]);
        let result = temp.path().join("momi-result.zip");
        let source_hash = format!("{:x}", Sha256::digest(&entries[0].1));
        fs::write(result.with_extension("palette.json"), serde_json::to_vec(&serde_json::json!({
            "rgba_map": {"#E3A17B":"#9DB9D4"},
            "regions": [{"asset":format!("{SOURCE}.png"),"source_sha256":source_hash,"size":[4,1],"seeds":[[0,0]]}]
        })).unwrap()).unwrap();
        fs::write(&result, zip(entries)).unwrap();
        fs::write(result.with_extension("zip.mods"), br#"{"mods":[{"name":"Other","version":"1.0.0"},{"name":"Adeline Palette Toggle Study","version":"0.1.0"}]}"#).unwrap();
        let runner = temp.path().join("runner");
        let shell = Command::new("sh")
            .args(["-c", "command -v sh"])
            .output()
            .unwrap();
        fs::write(&runner, format!("#!{}\nset -eu\ntest -f \"$2/mods/other/keep.txt\"\ntest -f \"$2/mods/lns_palette/gml/palette_toggle.gml\"\ncp \"$1\" \"$2/assets.zip\"\nmkdir -p \"$2/config/mods\"\ncp \"$1.mods\" \"$2/config/mods/manifest.json\"\n", String::from_utf8(shell.stdout).unwrap().trim())).unwrap();
        fs::set_permissions(&runner, fs::Permissions::from_mode(0o755)).unwrap();
        Self {
            _temp: temp,
            game,
            runner,
            result,
            before,
        }
    }
    fn run(&self, action: &str) -> Output {
        self.run_with_installed_mods(action, None)
    }
    fn run_with_installed_mods(&self, action: &str, installed_mods: Option<&Path>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_mistria-palette"));
        command
            .arg(action)
            .arg("--game-dir")
            .arg(&self.game)
            .env("MISTRIA_MOMI_RUNNER", &self.runner);
        if action == "install" {
            command.arg("--momi").arg(&self.result);
            command
                .arg("--palette")
                .arg(self.result.with_extension("palette.json"));
            if let Some(path) = installed_mods {
                command.arg("--installed-mods").arg(path);
            }
        }
        command.output().unwrap()
    }
    fn write_installed_mods(&self, contents: &[u8]) -> PathBuf {
        let path = self.game.join("config/mods/manifest.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, contents).unwrap();
        path
    }
    fn install(&self) {
        let result = self.run("install");
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}

#[test]
fn seasonal_install_checks_each_atlas_and_restores_the_original_archive() {
    for (season, folder, atlas_name) in [
        ("summer", "Summer", "PortraitsSummer"),
        ("autumn", "Autumn", "PortraitsAutumn"),
        ("winter", "Winter", "PortraitsWinter"),
    ] {
        for case in [
            "mixed",
            "season_only",
            "wrong_pixels",
            "wrong_atlas",
            "variant_wrong_atlas",
            "wrong_folder",
        ] {
            let mut lab = Lab::new();
            let source = if case == "wrong_folder" {
                SOURCE.replace("_spring_", &format!("_{season}_"))
            } else {
                SOURCE
                    .replace("Spring", folder)
                    .replace("_spring_", &format!("_{season}_"))
            };
            let variant = VARIANT.replace("_spring_", &format!("_{season}_"));
            let meta = META
                .replace("PortraitsSpring", atlas_name)
                .replace("0000000000000001", "0000000000000003");
            let original = png(RgbaImage::from_pixel(4, 1, Rgba([227, 161, 123, 255])));
            lab.before = with_zip_entry(&lab.before, &format!("{source}.png"), &original);
            lab.before =
                with_zip_entry(&lab.before, &format!("{source}.meta.toml"), meta.as_bytes());
            fs::write(lab.game.join("assets.zip"), &lab.before).unwrap();
            let recipe_path = lab.result.with_extension("palette.json");
            let mut recipe: serde_json::Value =
                serde_json::from_slice(&fs::read(&recipe_path).unwrap()).unwrap();
            let mut region = recipe["regions"][0].clone();
            region["asset"] = serde_json::json!(format!("{source}.png"));
            if case == "season_only" {
                recipe["regions"] = serde_json::json!([region]);
            } else {
                recipe["regions"].as_array_mut().unwrap().push(region);
            }
            fs::write(recipe_path, serde_json::to_vec(&recipe).unwrap()).unwrap();

            let mut result = fs::read(&lab.result).unwrap();
            result = with_zip_entry(&result, &format!("{source}.png"), &original);
            result = with_zip_entry(&result, &format!("{source}.meta.toml"), meta.as_bytes());
            result = with_zip_entry(
                &result,
                &format!("{variant}.meta.toml"),
                meta.replace("0000000000000003", "0000000000000004")
                    .as_bytes(),
            );
            let mut atlas = RgbaImage::from_fn(4, 2, |_, y| {
                Rgba(if y == 0 {
                    [227, 161, 123, 255]
                } else {
                    [157, 185, 212, 255]
                })
            });
            if case == "wrong_pixels" {
                atlas.put_pixel(3, 1, Rgba([0, 0, 0, 255]));
            }
            let placements = br#"[asset_properties]
animations = [
 {texture_ids = ["0000000000000003::0"], placement = [0,0,2,1,2,1,0,0]},
 {texture_ids = ["0000000000000003::1"], placement = [2,0,2,1,2,1,0,0]},
 {texture_ids = ["0000000000000004::0"], placement = [0,1,2,1,2,1,0,0]},
 {texture_ids = ["0000000000000004::1"], placement = [2,1,2,1,2,1,0,0]},
]
"#;
            result = with_zip_entry(
                &result,
                &format!("assets/atlases/{atlas_name}Atlas_1.png"),
                &png(atlas.clone()),
            );
            result = with_zip_entry(
                &result,
                &format!("assets/atlases/{atlas_name}Atlas_1.meta.toml"),
                match case {
                    "wrong_atlas" => b"[asset_properties]\nanimations=[]\n",
                    "variant_wrong_atlas" => {
                        br#"[asset_properties]
animations = [
 {texture_ids = ["0000000000000003::0"], placement = [0,0,2,1,2,1,0,0]},
 {texture_ids = ["0000000000000003::1"], placement = [2,0,2,1,2,1,0,0]},
]
"#
                    }
                    _ => placements,
                },
            );
            if ["wrong_atlas", "variant_wrong_atlas"].contains(&case) {
                result = with_zip_entry(
                    &result,
                    "assets/atlases/PortraitsSpringAtlas_1.png",
                    &png(atlas),
                );
                result = with_zip_entry(
                    &result,
                    "assets/atlases/PortraitsSpringAtlas_1.meta.toml",
                    if case == "variant_wrong_atlas" {
                        br#"[asset_properties]
animations = [
 {texture_ids = ["0000000000000004::0"], placement = [0,1,2,1,2,1,0,0]},
 {texture_ids = ["0000000000000004::1"], placement = [2,1,2,1,2,1,0,0]},
]
"#
                    } else {
                        placements
                    },
                );
            }
            let spring = [
                "spr_portrait_adeline_spring_neutral".to_owned(),
                "spr_lns_adeline_spring_neutral_blue".to_owned(),
            ];
            let seasonal = [
                format!("spr_portrait_adeline_{season}_neutral"),
                format!("spr_lns_adeline_{season}_neutral_blue"),
            ];
            let groups = if case == "season_only" {
                vec![seasonal]
            } else {
                // Packaging sorts the source paths before emitting the table.
                if season == "autumn" {
                    vec![seasonal, spring]
                } else {
                    vec![spring, seasonal]
                }
            };
            let script = format!(
                "// Generated from the portraits included in this local package.\nfunction lns_palette_assets() {{ return {}; }}\nfunction lns_palette_names() {{ return [\"Vanilla\",\"Debug Blue\"]; }}\n",
                serde_json::to_string(&groups).unwrap()
            );
            result = replace_zip_entry(
                &result,
                &format!("{PALETTE_SCRIPTS}/palette_assets.gml"),
                script.as_bytes(),
            );
            fs::write(&lab.result, result).unwrap();
            let outcome = lab.run("install");
            if ["mixed", "season_only"].contains(&case) {
                assert!(
                    outcome.status.success(),
                    "{season} {case}: {}",
                    String::from_utf8_lossy(&outcome.stderr)
                );
                assert!(lab.run("uninstall").status.success());
            } else {
                assert!(!outcome.status.success(), "accepted {case}");
                if case == "variant_wrong_atlas" {
                    assert!(
                        String::from_utf8_lossy(&outcome.stderr)
                            .contains("Missing or duplicate atlas frame: 0000000000000004::0"),
                        "variant case failed before testing atlas-family isolation: {}",
                        String::from_utf8_lossy(&outcome.stderr)
                    );
                }
                assert!(!lab.game.join(".mistria-palette").exists());
            }
            assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
        }
    }
}

#[test]
fn every_selected_portrait_and_its_runtime_mapping_are_verified() {
    for corruption in [
        "none",
        "second_frame",
        "second_metadata",
        "overlapping_id",
        "mapping",
    ] {
        let mut lab = Lab::new();
        let source = SOURCE.replace("neutral", "happy");
        let variant = VARIANT.replace("neutral", "happy");
        let original = png(RgbaImage::from_pixel(4, 1, Rgba([227, 161, 123, 255])));
        let meta = META.replace("0000000000000001", "0000000000000003");
        lab.before = with_zip_entry(&lab.before, &format!("{source}.png"), &original);
        lab.before = with_zip_entry(&lab.before, &format!("{source}.meta.toml"), meta.as_bytes());
        fs::write(lab.game.join("assets.zip"), &lab.before).unwrap();
        let palette_path = lab.result.with_extension("palette.json");
        let mut recipe: serde_json::Value =
            serde_json::from_slice(&fs::read(&palette_path).unwrap()).unwrap();
        let mut second = recipe["regions"][0].clone();
        second["asset"] = serde_json::json!(format!("{source}.png"));
        recipe["regions"].as_array_mut().unwrap().push(second);
        fs::write(palette_path, serde_json::to_vec(&recipe).unwrap()).unwrap();
        let mut result = fs::read(&lab.result).unwrap();
        result = with_zip_entry(&result, &format!("{source}.png"), &original);
        result = with_zip_entry(&result, &format!("{source}.meta.toml"), meta.as_bytes());
        let variant_meta = meta.replace("0000000000000003", "0000000000000004");
        let variant_meta = match corruption {
            "second_metadata" => variant_meta.replace("duration = 0.2", "duration = 0.3"),
            "overlapping_id" => variant_meta.replace("0000000000000004", "0000000000000002"),
            _ => variant_meta,
        };
        result = with_zip_entry(
            &result,
            &format!("{variant}.meta.toml"),
            variant_meta.as_bytes(),
        );
        let mut atlas = RgbaImage::from_fn(4, 4, |_, y| {
            Rgba(if y % 2 == 0 {
                [227, 161, 123, 255]
            } else {
                [157, 185, 212, 255]
            })
        });
        if corruption == "second_frame" {
            atlas.put_pixel(3, 3, Rgba([0, 0, 0, 255]));
        }
        result = replace_zip_entry(
            &result,
            "assets/atlases/PortraitsSpringAtlas.png",
            &png(atlas),
        );
        let mut placements = String::from("[asset_properties]\nanimations = [\n");
        for row in 0..4 {
            for frame in 0..2 {
                placements.push_str(&format!("{{texture_ids = [\"{:016}::{frame}\"], placement = [{},{row},2,1,2,1,0,0]}},\n",row+1,frame*2));
            }
        }
        placements.push_str("]\n");
        result = replace_zip_entry(
            &result,
            "assets/atlases/PortraitsSpringAtlas.meta.toml",
            placements.as_bytes(),
        );
        result = replace_zip_entry(
            &result,
            &format!("{PALETTE_SCRIPTS}/palette_assets.gml"),
            &asset_script(if corruption == "mapping" {
                &["neutral"]
            } else {
                &["happy", "neutral"]
            }),
        );
        fs::write(&lab.result, result).unwrap();
        let outcome = lab.run("install");
        if corruption == "none" {
            assert!(
                outcome.status.success(),
                "{}",
                String::from_utf8_lossy(&outcome.stderr)
            );
            assert!(lab.run("uninstall").status.success());
        } else {
            assert!(!outcome.status.success(), "accepted {corruption}");
            assert!(!lab.game.join(".mistria-palette").exists());
        }
        assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
    }
}

#[test]
fn palette_without_regions_uses_the_neutral_fallback() {
    let lab = Lab::new();
    let palette = lab.result.with_extension("palette.json");
    let mut recipe: serde_json::Value =
        serde_json::from_slice(&fs::read(&palette).unwrap()).unwrap();
    recipe.as_object_mut().unwrap().remove("regions");
    fs::write(&palette, serde_json::to_vec(&recipe).unwrap()).unwrap();

    let installed = lab.run("install");
    assert!(
        installed.status.success(),
        "region-free recipe did not use neutral fallback: {}",
        String::from_utf8_lossy(&installed.stderr)
    );
    assert_eq!(
        fs::read(lab.game.join("assets.zip")).unwrap(),
        fs::read(&lab.result).unwrap()
    );
    assert!(lab.run("uninstall").status.success());
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
}

#[test]
fn verification_follows_frames_across_numbered_atlas_pages() {
    for case in ["split", "duplicate", "wrong_pixels"] {
        let lab = Lab::new();
        let mut result = fs::read(&lab.result).unwrap();
        if case != "duplicate" {
            result = replace_zip_entry(
                &result,
                "assets/atlases/PortraitsSpringAtlas.meta.toml",
                br#"[asset_properties]
animations = [
 {texture_ids = ["0000000000000001::0"], placement = [0,0,2,1,2,1,0,0]},
 {texture_ids = ["0000000000000001::1"], placement = [2,0,2,1,2,1,0,0]},
 {texture_ids = ["0000000000000002::0"], placement = [0,1,2,1,2,1,0,0]},
]
"#,
            );
        }
        result = with_zip_entry(
            &result,
            "assets/atlases/PortraitsSpringAtlas_1.meta.toml",
            br#"[asset_properties]
animations = [{texture_ids = ["0000000000000002::1"], placement = [0,0,2,1,2,1,0,0]}]
"#,
        );
        result = with_zip_entry(
            &result,
            "assets/atlases/PortraitsSpringAtlas_1.png",
            &png(RgbaImage::from_pixel(
                2,
                1,
                Rgba(if case == "wrong_pixels" {
                    [0, 0, 0, 255]
                } else {
                    [157, 185, 212, 255]
                }),
            )),
        );
        fs::write(&lab.result, result).unwrap();
        let outcome = lab.run("install");
        if case == "split" {
            assert!(
                outcome.status.success(),
                "{}",
                String::from_utf8_lossy(&outcome.stderr)
            );
            assert!(lab.run("uninstall").status.success());
        } else {
            assert!(!outcome.status.success(), "accepted {case}");
            assert!(!lab.game.join(".mistria-palette").exists());
        }
        assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
    }
}

#[test]
fn default_palette_refuses_an_unreviewed_portrait() {
    let lab = Lab::new();
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .arg("install")
        .arg("--game-dir")
        .arg(&lab.game)
        .arg("--momi")
        .arg(&lab.result)
        .env("MISTRIA_MOMI_RUNNER", &lab.runner)
        .output()
        .unwrap();
    assert!(
        !result.status.success(),
        "default palette accepted an unreviewed portrait"
    );
    assert!(String::from_utf8_lossy(&result.stderr).contains("Region source checksum mismatch"));
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
    assert!(!lab.game.join(".mistria-palette").exists());
}

#[test]
fn install_and_remove_preserve_other_mods_and_restore_exact_archive() {
    let lab = Lab::new();
    lab.install();
    assert_eq!(
        fs::read(lab.game.join("assets.zip")).unwrap(),
        fs::read(&lab.result).unwrap()
    );
    assert_eq!(
        fs::read_to_string(lab.game.join("mods/other/keep.txt")).unwrap(),
        "another mod's source"
    );
    let removed = lab.run("uninstall");
    assert!(
        removed.status.success(),
        "{}",
        String::from_utf8_lossy(&removed.stderr)
    );
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
    assert!(!lab.game.join(".mistria-palette").exists());
    assert!(lab.run("uninstall").status.success());
}

#[test]
fn an_existing_momi_install_is_rebuilt_from_backup_then_restored_exactly() {
    let lab = Lab::new();
    let pristine = without_zip_entry(&lab.before, "other-mod.txt");
    let prior = with_zip_entry(&lab.before, "manifest.toml", b"existing MOMI marker");
    let prior = with_zip_entry(
        &prior,
        "existing-install-only.txt",
        b"must return after palette removal",
    );
    fs::write(lab.game.join("assets.bak.zip"), &pristine).unwrap();
    fs::write(lab.game.join("assets.zip"), &prior).unwrap();
    let selection = lab.write_installed_mods(br#"{"mods":[{"name":"Other","version":"1.0.0"}]}"#);

    let installed = lab.run_with_installed_mods("install", Some(&selection));
    assert!(
        installed.status.success(),
        "{}",
        String::from_utf8_lossy(&installed.stderr)
    );
    assert_eq!(
        fs::read(lab.game.join("assets.zip")).unwrap(),
        fs::read(&lab.result).unwrap()
    );
    assert_eq!(
        fs::read(lab.game.join("assets.bak.zip")).unwrap(),
        pristine,
        "the existing MOMI backup was changed"
    );
    assert_eq!(
        fs::read(lab.game.join(".mistria-palette/previous.zip")).unwrap(),
        prior,
        "the recovery snapshot was not the exact pre-install archive"
    );

    let removed = lab.run("uninstall");
    assert!(
        removed.status.success(),
        "{}",
        String::from_utf8_lossy(&removed.stderr)
    );
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), prior);
    assert_eq!(
        fs::read_to_string(lab.game.join("mods/other/keep.txt")).unwrap(),
        "another mod's source"
    );
}

#[test]
fn incomplete_sources_for_a_marked_momi_archive_prevent_publication() {
    let lab = Lab::new();
    let pristine = without_zip_entry(&lab.before, "other-mod.txt");
    let prior = with_zip_entry(&pristine, "manifest.toml", b"existing MOMI marker");
    let prior = with_zip_entry(&prior, "other-mod.txt", b"installed from retained source A");
    let prior = with_zip_entry(
        &prior,
        "missing-source-mod.txt",
        b"installed from missing source B",
    );
    fs::write(lab.game.join("assets.bak.zip"), pristine).unwrap();
    fs::write(lab.game.join("assets.zip"), &prior).unwrap();
    let prior_selection = br#"{"mods":[{"name":"Other","version":"1.0.0"},{"name":"Missing Source","version":"1.0.0"}]}"#;
    let selection = lab.write_installed_mods(prior_selection);

    let installed = lab.run_with_installed_mods("install", Some(&selection));
    assert!(
        !installed.status.success(),
        "published an A-only rebuild over a marked archive containing A+B"
    );
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), prior);
    assert_eq!(
        fs::read(lab.game.join("config/mods/manifest.json")).unwrap(),
        prior_selection
    );
    assert!(!lab.game.join(".mistria-palette").exists());
}

#[test]
fn a_marked_archive_requires_an_explicit_installed_mod_list() {
    let lab = Lab::new();
    let pristine = without_zip_entry(&lab.before, "other-mod.txt");
    let prior = with_zip_entry(&lab.before, "manifest.toml", b"existing MOMI marker");
    fs::write(lab.game.join("assets.bak.zip"), &pristine).unwrap();
    fs::write(lab.game.join("assets.zip"), &prior).unwrap();
    lab.write_installed_mods(br#"{"mods":[{"name":"Other","version":"1.0.0"}]}"#);

    let installed = lab.run("install");
    assert!(!installed.status.success());
    assert!(String::from_utf8_lossy(&installed.stderr).contains("--installed-mods"));
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), prior);
    assert_eq!(fs::read(lab.game.join("assets.bak.zip")).unwrap(), pristine);
    assert!(!lab.game.join(".mistria-palette").exists());
}

#[test]
fn supplied_mod_list_must_match_existing_source_names_and_versions() {
    for marked in [false, true] {
        for selection in [
            r#"{"mods":[]}"#,
            r#"{"mods":[{"name":"Different","version":"1.0.0"}]}"#,
            r#"{"mods":[{"name":"Other","version":"1.0.0"},{"name":"Different","version":"1.0.0"}]}"#,
            r#"{"mods":[{"name":"Other","version":"1.0.0"},{"name":"Other","version":"1.0.0"}]}"#,
            r#"{"mods":[{"name":"Other","version":"2.0.0"}]}"#,
            r#"{"mods":[{"name":"Other","version":"1.0.0"},{"name":"Adeline Palette Toggle Study","version":"0.1.0"}]}"#,
            "not a mod list",
        ] {
            let lab = Lab::new();
            let prior = if marked {
                let pristine = without_zip_entry(&lab.before, "other-mod.txt");
                fs::write(lab.game.join("assets.bak.zip"), pristine).unwrap();
                with_zip_entry(&lab.before, "manifest.toml", b"existing MOMI marker")
            } else {
                lab.before.clone()
            };
            fs::write(lab.game.join("assets.zip"), &prior).unwrap();
            let path = lab.write_installed_mods(selection.as_bytes());

            let installed = lab.run_with_installed_mods("install", Some(&path));
            assert!(!installed.status.success(), "accepted {selection}");
            assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), prior);
            assert!(!lab.game.join(".mistria-palette").exists());
        }
    }
}

#[test]
fn an_unmarked_archive_accepts_a_matching_unordered_installed_mod_list() {
    let lab = Lab::new();
    fs::create_dir(lab.game.join("mods/additional")).unwrap();
    fs::write(
        lab.game.join("mods/additional/manifest.json"),
        br#"{"name":"Additional","version":"2.0.0"}"#,
    )
    .unwrap();
    let selection = lab.write_installed_mods(
        br#"{"mods":[{"name":"Other","version":"1.0.0"},{"name":"Additional","version":"2.0.0"}]}"#,
    );
    fs::write(
        lab.result.with_extension("zip.mods"),
        br#"{"mods":[{"name":"Other","version":"1.0.0"},{"name":"Adeline Palette Toggle Study","version":"0.1.0"},{"name":"Additional","version":"2.0.0"}]}"#,
    )
    .unwrap();

    let installed = lab.run_with_installed_mods("install", Some(&selection));
    assert!(
        installed.status.success(),
        "{}",
        String::from_utf8_lossy(&installed.stderr)
    );
    assert_eq!(
        fs::read(lab.game.join("assets.zip")).unwrap(),
        fs::read(&lab.result).unwrap()
    );
}

#[test]
fn rebuilding_must_preserve_existing_mod_order() {
    for (prior_names, rebuilt_names, palette_index, succeeds) in [
        (["Other", "Additional"], ["Additional", "Other"], 1, false),
        (["Additional", "Other"], ["Other", "Additional"], 1, false),
        (["Other", "Additional"], ["Other", "Additional"], 0, true),
        (["Other", "Additional"], ["Other", "Additional"], 1, true),
        (["Other", "Additional"], ["Other", "Additional"], 2, true),
        (["Additional", "Other"], ["Additional", "Other"], 0, true),
        (["Additional", "Other"], ["Additional", "Other"], 1, true),
        (["Additional", "Other"], ["Additional", "Other"], 2, true),
    ] {
        let lab = Lab::new();
        fs::create_dir(lab.game.join("mods/additional")).unwrap();
        fs::write(
            lab.game.join("mods/additional/manifest.json"),
            br#"{"name":"Additional","version":"1.0.0"}"#,
        )
        .unwrap();
        let prior = with_zip_entry(&lab.before, "manifest.toml", b"");
        fs::write(lab.game.join("assets.bak.zip"), &lab.before).unwrap();
        fs::write(lab.game.join("assets.zip"), &prior).unwrap();
        let entries = |names: [&str; 2]| {
            names.map(|name| serde_json::json!({"name": name, "version": "1.0.0"}))
        };
        let selection_bytes =
            serde_json::to_vec(&serde_json::json!({"mods": entries(prior_names)})).unwrap();
        let selection = lab.write_installed_mods(&selection_bytes);
        let mut rebuilt = entries(rebuilt_names).to_vec();
        rebuilt.insert(
            palette_index,
            serde_json::json!({"name": "Adeline Palette Toggle Study", "version": "0.1.0"}),
        );
        fs::write(
            lab.result.with_extension("zip.mods"),
            serde_json::to_vec(&serde_json::json!({"mods": rebuilt})).unwrap(),
        )
        .unwrap();

        let result = lab.run_with_installed_mods("install", Some(&selection));
        assert_eq!(
            result.status.success(),
            succeeds,
            "prior {prior_names:?}, rebuilt {rebuilt_names:?}, palette index {palette_index}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        if succeeds {
            assert_eq!(
                fs::read(lab.game.join("assets.zip")).unwrap(),
                fs::read(&lab.result).unwrap()
            );
            assert!(lab.run("uninstall").status.success());
        } else {
            assert!(String::from_utf8_lossy(&result.stderr).contains("load order"));
        }
        assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), prior);
        assert_eq!(fs::read(selection).unwrap(), selection_bytes);
        assert_eq!(
            fs::read(lab.game.join("assets.bak.zip")).unwrap(),
            lab.before
        );
        assert!(!lab.game.join(".mistria-palette").exists());
    }
}

#[test]
fn prepared_but_unpublished_install_can_be_cleared_without_touching_live_archive() {
    let lab = Lab::new();
    lab.install();
    let state = lab.game.join(".mistria-palette");
    fs::copy(lab.game.join("assets.zip"), state.join("ready.zip")).unwrap();
    fs::write(lab.game.join("assets.zip"), &lab.before).unwrap();

    let removed = lab.run("uninstall");
    assert!(
        removed.status.success(),
        "{}",
        String::from_utf8_lossy(&removed.stderr)
    );
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
    assert!(!state.exists());
}

#[test]
fn empty_recovery_directory_from_interrupted_cleanup_is_retryable() {
    let lab = Lab::new();
    lab.install();
    let state = lab.game.join(".mistria-palette");
    fs::write(lab.game.join("assets.zip"), &lab.before).unwrap();
    fs::remove_file(state.join("previous.zip")).unwrap();
    fs::remove_file(state.join("receipt.json")).unwrap();
    assert_eq!(fs::read_dir(&state).unwrap().count(), 0);

    let removed = lab.run("uninstall");
    assert!(
        removed.status.success(),
        "empty cleanup state was not retryable: {}",
        String::from_utf8_lossy(&removed.stderr)
    );
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
    assert!(!state.exists());
}

#[test]
fn nonempty_unrecognized_recovery_state_is_retained() {
    let unknown = Lab::new();
    unknown.install();
    let unknown_state = unknown.game.join(".mistria-palette");
    fs::write(unknown_state.join("unexpected"), b"do not delete").unwrap();
    let removed = unknown.run("uninstall");
    assert!(!removed.status.success());
    assert!(unknown_state.join("unexpected").is_file());
    assert!(unknown_state.join("receipt.json").is_file());

    let missing_receipt = Lab::new();
    missing_receipt.install();
    let missing_state = missing_receipt.game.join(".mistria-palette");
    fs::remove_file(missing_state.join("receipt.json")).unwrap();
    let removed = missing_receipt.run("uninstall");
    assert!(!removed.status.success());
    assert!(missing_state.join("previous.zip").is_file());
    assert!(!missing_state.join("receipt.json").exists());
}

#[test]
fn damaged_recovery_archive_blocks_removal_and_retains_recovery_state() {
    let lab = Lab::new();
    lab.install();
    let installed = fs::read(lab.game.join("assets.zip")).unwrap();
    let state = lab.game.join(".mistria-palette");
    fs::write(state.join("previous.zip"), b"damaged recovery data").unwrap();

    let removed = lab.run("uninstall");
    assert!(!removed.status.success());
    assert!(String::from_utf8_lossy(&removed.stderr).contains("checksum mismatch"));
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), installed);
    assert_eq!(
        fs::read(state.join("previous.zip")).unwrap(),
        b"damaged recovery data"
    );
    assert!(state.join("receipt.json").is_file());
}

#[test]
fn unsuccessful_or_incomplete_installer_output_leaves_live_files_unchanged() {
    let lab = Lab::new();
    // A process can succeed without actually installing the palette.
    fs::write(&lab.result, &lab.before).unwrap();
    let result = lab.run("install");
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("Missing archive member"));
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
    assert!(!lab.game.join(".mistria-palette").exists());
}

#[test]
fn palette_scripts_can_share_filenames_with_another_mod() {
    for name in ["palette_assets.gml", "palette_toggle.gml"] {
        let lab = Lab::new();
        let result = with_zip_entry(
            &fs::read(&lab.result).unwrap(),
            &format!("assets/gml/scripts/test_other/{name}"),
            b"// Another mod's script with its own contents.\n",
        );
        fs::write(&lab.result, &result).unwrap();

        let installed = lab.run("install");
        assert!(
            installed.status.success(),
            "another mod's {name} prevented installation: {}",
            String::from_utf8_lossy(&installed.stderr)
        );
        assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), result);
        assert!(lab.run("uninstall").status.success());
        assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
    }
}

#[test]
fn palette_scripts_must_be_present_in_the_package_namespace() {
    for (name, expected) in [
        ("palette_assets.gml", asset_script(&["neutral"])),
        (
            "palette_toggle.gml",
            include_bytes!("../mod/toggle/gml/palette_toggle.gml").to_vec(),
        ),
    ] {
        let lab = Lab::new();
        let result = without_zip_entry(
            &fs::read(&lab.result).unwrap(),
            &format!("{PALETTE_SCRIPTS}/{name}"),
        );
        let result = with_zip_entry(
            &result,
            &format!("assets/gml/scripts/test_other/{name}"),
            &expected,
        );
        fs::write(&lab.result, result).unwrap();

        let installed = lab.run("install");
        assert!(
            !installed.status.success(),
            "accepted {name} from another mod when the package's script was missing"
        );
        assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
        assert!(!lab.game.join(".mistria-palette").exists());
    }
}

#[test]
fn corrupted_pixels_metadata_or_script_prevent_publication() {
    let corruptions = [
        (
            "assets/atlases/PortraitsSpringAtlas.png",
            png(RgbaImage::from_pixel(4, 2, Rgba([0, 0, 0, 0]))),
            "Installed pixels differ",
        ),
        (
            "assets/animations/LightningAndSun/spr_lns_adeline_spring_neutral_blue.meta.toml",
            META.replace("0000000000000001", "0000000000000002")
                .replace("duration = 0.2", "duration = 0.3")
                .into_bytes(),
            "Variant properties differ",
        ),
        (
            &format!("{PALETTE_SCRIPTS}/palette_toggle.gml"),
            b"// wrong script\n".to_vec(),
            "Installed palette script differs",
        ),
    ];

    for (entry, contents, expected_error) in corruptions {
        let lab = Lab::new();
        let result = replace_zip_entry(&fs::read(&lab.result).unwrap(), entry, &contents);
        fs::write(&lab.result, result).unwrap();

        let installed = lab.run("install");
        assert!(
            !installed.status.success(),
            "published archive with corrupted {entry}"
        );
        assert!(
            String::from_utf8_lossy(&installed.stderr).contains(expected_error),
            "unexpected error for {entry}: {}",
            String::from_utf8_lossy(&installed.stderr)
        );
        assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
        assert!(!lab.game.join(".mistria-palette").exists());
    }
}

#[test]
fn corrupted_variant_asset_kind_prevents_publication() {
    let lab = Lab::new();
    let entry = "assets/animations/LightningAndSun/spr_lns_adeline_spring_neutral_blue.meta.toml";
    let metadata = META
        .replace("0000000000000001", "0000000000000002")
        .replace("asset_kind = 'Animation'", "asset_kind = 'Script'");
    let result = replace_zip_entry(&fs::read(&lab.result).unwrap(), entry, metadata.as_bytes());
    fs::write(&lab.result, result).unwrap();

    let installed = lab.run("install");
    assert!(
        !installed.status.success(),
        "published archive with a corrupted variant asset kind"
    );
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
    assert!(!lab.game.join(".mistria-palette").exists());
}

#[test]
fn removal_refuses_to_overwrite_a_game_update() {
    let lab = Lab::new();
    lab.install();
    fs::write(
        lab.game.join("assets.zip"),
        b"a newly downloaded game archive",
    )
    .unwrap();
    assert!(!lab.run("uninstall").status.success());
    assert_eq!(
        fs::read(lab.game.join("assets.zip")).unwrap(),
        b"a newly downloaded game archive"
    );
    assert!(lab.game.join(".mistria-palette/previous.zip").is_file());
}

#[test]
fn a_silently_skipped_existing_mod_prevents_publication() {
    let lab = Lab::new();
    fs::write(
        lab.result.with_extension("zip.mods"),
        br#"{"mods":[{"name":"Adeline Palette Toggle Study","version":"0.1.0"}]}"#,
    )
    .unwrap();
    let result = lab.run("install");
    assert!(
        !result.status.success(),
        "published despite a skipped existing mod"
    );
    assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
    assert!(!lab.game.join(".mistria-palette").exists());
}

#[test]
fn preset_install_verifies_the_last_variant_and_restores_the_prior_archive() {
    for corrupt in [false, true] {
        let lab = Lab::new();
        let old: serde_json::Value =
            serde_json::from_slice(&fs::read(lab.result.with_extension("palette.json")).unwrap())
                .unwrap();
        let profile = lab.result.with_extension("profile.json");
        fs::write(
            &profile,
            serde_json::to_vec(
                &serde_json::json!({"source_colors":["#E3A17B"],"regions":old["regions"]}),
            )
            .unwrap(),
        )
        .unwrap();
        let presets = lab.result.with_extension("presets.json");
        fs::write(&presets,serde_json::to_vec(&serde_json::json!({"profile":profile,"presets":[{"id":"blue","label":"Debug Blue","colors":["#9DB9D4"]},{"id":"warm","label":"Warm trial","colors":["#806040"]}]})).unwrap()).unwrap();
        let mut result = with_zip_entry(
            &fs::read(&lab.result).unwrap(),
            &format!("{}.meta.toml", VARIANT.replace("_blue", "_warm")),
            META.replace("0000000000000001", "0000000000000003")
                .as_bytes(),
        );
        let mut atlas = RgbaImage::from_fn(4, 3, |_, y| {
            Rgba(match y {
                0 => [227, 161, 123, 255],
                1 => [157, 185, 212, 255],
                _ => [128, 96, 64, 255],
            })
        });
        if corrupt {
            atlas.put_pixel(3, 2, Rgba([0, 0, 0, 255]));
        }
        result = replace_zip_entry(
            &result,
            "assets/atlases/PortraitsSpringAtlas.png",
            &png(atlas),
        );
        let mut placements = String::from("[asset_properties]\nanimations=[\n");
        for row in 0..3 {
            for frame in 0..2 {
                placements.push_str(&format!(
                    "{{texture_ids=['{:016}::{frame}'],placement=[{},{row},2,1,2,1,0,0]}},\n",
                    row + 1,
                    frame * 2
                ));
            }
        }
        placements.push_str("]\n");
        result = replace_zip_entry(
            &result,
            "assets/atlases/PortraitsSpringAtlas.meta.toml",
            placements.as_bytes(),
        );
        result=replace_zip_entry(&result,&format!("{PALETTE_SCRIPTS}/palette_assets.gml"),b"// Generated from the portraits included in this local package.\nfunction lns_palette_assets() { return [[\"spr_portrait_adeline_spring_neutral\",\"spr_lns_adeline_spring_neutral_blue\",\"spr_lns_adeline_spring_neutral_warm\"]]; }\nfunction lns_palette_names() { return [\"Vanilla\",\"Debug Blue\",\"Warm trial\"]; }\n");
        fs::write(&lab.result, result).unwrap();
        let installed = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .arg("install")
            .arg("--game-dir")
            .arg(&lab.game)
            .arg("--momi")
            .arg(&lab.result)
            .arg("--presets")
            .arg(&presets)
            .env("MISTRIA_MOMI_RUNNER", &lab.runner)
            .output()
            .unwrap();
        if corrupt {
            assert!(!installed.status.success());
            assert!(String::from_utf8_lossy(&installed.stderr).contains("Installed pixels differ"));
            assert!(!lab.game.join(".mistria-palette").exists());
        } else {
            assert!(
                installed.status.success(),
                "{}",
                String::from_utf8_lossy(&installed.stderr)
            );
            assert!(lab.run("uninstall").status.success());
        }
        assert_eq!(fs::read(lab.game.join("assets.zip")).unwrap(), lab.before);
    }
}
