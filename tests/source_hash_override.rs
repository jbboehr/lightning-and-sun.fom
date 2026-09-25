use image::{Rgba, RgbaImage};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use zip::{ZipWriter, write::SimpleFileOptions};

const ASSET: &str =
    "assets/animations/NPCs/Adeline/Portraits/Spring/spr_portrait_adeline_spring_neutral.png";
const FLAG: &str = "--allow-source-hash-mismatch";
const SKIN: [u8; 4] = [10, 20, 30, 255];
const BLUE: [u8; 4] = [40, 50, 60, 255];
const WARM: [u8; 4] = [100, 80, 60, 255];
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn write(path: impl AsRef<Path>, value: &Value) {
    fs::write(path, serde_json::to_vec(value).unwrap()).unwrap();
}
fn success(result: Output) -> Value {
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&result.stdout).unwrap()
}
fn failure(result: Output, message: &str) {
    assert!(!result.status.success());
    assert!(
        String::from_utf8_lossy(&result.stderr).contains(message),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
struct Fixture {
    temp: tempfile::TempDir,
    original: PathBuf,
    expected: String,
    actual: String,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let original = root.join("original");
        let p = original.join(ASSET);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        let mut image = RgbaImage::from_fn(8, 1, |x, _| {
            Rgba(if x % 2 == 0 { SKIN } else { [0, 0, 0, 255] })
        });
        image.save(&p).unwrap();
        let expected = hash(&fs::read(&p).unwrap());
        write(
            root.join("profile.json"),
            &json!({"source_colors":["#0A141E"],"regions":[{"asset":ASSET,"source_sha256":expected,"size":[8,1],"seeds":[[0,0],[4,0]]}]}),
        );
        // An actual PNG change away from the seeds; the reviewed hash stays pinned.
        image.put_pixel(7, 0, Rgba([5, 6, 7, 255]));
        image.save(&p).unwrap();
        let actual = hash(&fs::read(&p).unwrap());
        assert_ne!(expected, actual);
        fs::write(original.join(ASSET.replace(".png",".meta.toml")),"[meta_properties]\nid='synthetic'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[4,1]\nframe_len=2\natlas='PortraitsSpring'\nduration=0.2\n").unwrap();
        write(
            root.join("palette.json"),
            &json!({"profile":"profile.json","rgba_map":{"#0A141E":"#28323C"}}),
        );
        write(
            root.join("presets.json"),
            &json!({"profile":"profile.json","presets":[{"id":"blue","label":"Blue","colors":["#28323C"]},{"id":"warm","label":"Warm","colors":["#64503C"]}]}),
        );
        write(
            root.join("characters.json"),
            &json!({"characters":[{"id":"adeline","presets":"presets.json"}]}),
        );
        let mut archive = ZipWriter::new(fs::File::create(root.join("assets.zip")).unwrap());
        for name in [ASSET.to_string(), ASSET.replace(".png", ".meta.toml")] {
            archive
                .start_file(&name, SimpleFileOptions::default())
                .unwrap();
            archive
                .write_all(&fs::read(original.join(name)).unwrap())
                .unwrap();
        }
        archive.finish().unwrap();
        Self {
            temp,
            original,
            expected,
            actual,
        }
    }
    fn generate(&self, command: &str, output: &Path, allow: bool) -> Output {
        let root = self.temp.path();
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_mistria-palette"));
        cmd.arg(command);
        match command {
            "apply" => {
                cmd.arg("--input")
                    .arg(&self.original)
                    .arg("--palette")
                    .arg(root.join("palette.json"));
            }
            "build-presets" => {
                cmd.arg("--original")
                    .arg(&self.original)
                    .arg("--presets")
                    .arg(root.join("presets.json"));
            }
            "build-characters" => {
                cmd.arg("--archive")
                    .arg(root.join("assets.zip"))
                    .arg("--characters")
                    .arg(root.join("characters.json"));
            }
            _ => unreachable!(),
        }
        cmd.arg("--output").arg(output);
        if allow {
            cmd.arg(FLAG);
        }
        cmd.output().unwrap()
    }
    fn validate(&self, output: &Path, allow: bool) -> Output {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_mistria-palette"));
        cmd.arg("validate")
            .arg("--original")
            .arg(&self.original)
            .arg("--modified")
            .arg(output)
            .arg("--palette")
            .arg(self.temp.path().join("palette.json"));
        if allow {
            cmd.arg(FLAG);
        }
        cmd.output().unwrap()
    }
    fn check_pixels(&self, output: &Path, target: [u8; 4]) {
        let before = image::open(self.original.join(ASSET)).unwrap().to_rgba8();
        let after = image::open(output.join(ASSET)).unwrap().to_rgba8();
        assert_eq!(before.dimensions(), after.dimensions());
        for x in 0..8 {
            assert_eq!(
                after.get_pixel(x, 0).0,
                if x == 0 || x == 4 {
                    target
                } else {
                    before.get_pixel(x, 0).0
                }
            );
        }
        let meta = ASSET.replace(".png", ".meta.toml");
        assert_eq!(
            fs::read(self.original.join(&meta)).unwrap(),
            fs::read(output.join(meta)).unwrap()
        );
    }
}
#[test]
fn generation_only_bypasses_source_hash_when_explicitly_requested() {
    for command in ["apply", "build-presets", "build-characters"] {
        let f = Fixture::new();
        let root = f.temp.path();
        let output = root.join("output");
        let profile = fs::read(root.join("profile.json")).unwrap();
        failure(
            f.generate(command, &output, false),
            "Region source checksum mismatch",
        );
        assert!(!output.exists());
        let result = f.generate(command, &output, true);
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert!(result.status.success(), "{stderr}");
        assert!(
            stderr.contains(ASSET) && stderr.contains(&f.expected) && stderr.contains(&f.actual),
            "{stderr}"
        );
        let report = success(result);
        let prefix = if command == "build-characters" {
            output.join("characters/adeline")
        } else {
            output.clone()
        };
        let reports = match command {
            "apply" => vec![report],
            "build-presets" => report["presets"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| p["report"].clone())
                .collect(),
            _ => report["characters"][0]["presets"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| p["report"].clone())
                .collect(),
        };
        for r in reports {
            assert_eq!(r["files"][0]["original_sha256"], f.actual);
        }
        if command == "apply" {
            f.check_pixels(&prefix, BLUE);
        } else {
            f.check_pixels(&prefix.join("variants/blue"), BLUE);
            f.check_pixels(&prefix.join("variants/warm"), WARM);
        }
        assert_eq!(fs::read(root.join("profile.json")).unwrap(), profile);
        assert_eq!(hash(&fs::read(f.original.join(ASSET)).unwrap()), f.actual);
        failure(
            f.generate(command, &root.join("still-strict"), false),
            "Region source checksum mismatch",
        );
    }
}
#[test]
fn override_validation_still_checks_exact_pixels_and_alpha() {
    let f = Fixture::new();
    let output = f.temp.path().join("output");
    success(f.generate("apply", &output, true));
    failure(
        f.validate(&output, false),
        "Region source checksum mismatch",
    );
    assert_eq!(success(f.validate(&output, true))["palette_verified"], true);
    let p = output.join(ASSET);
    let original = fs::read(&p).unwrap();
    for (pixel, message) in [
        (Rgba([1, 2, 3, 255]), "Palette mismatch"),
        (Rgba([40, 50, 60, 128]), "Alpha"),
    ] {
        let mut image = image::load_from_memory(&original).unwrap().to_rgba8();
        image.put_pixel(0, 0, pixel);
        image.save(&p).unwrap();
        let result = f.validate(&output, true);
        assert!(!result.status.success());
        assert!(
            String::from_utf8_lossy(&result.stderr)
                .to_lowercase()
                .contains(&message.to_lowercase()),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}
#[test]
fn override_preserves_geometry_inventory_and_seed_checks_without_partial_output() {
    for (case, message) in [
        ("size", "Region dimensions differ"),
        ("seed_outside", "Region seed outside image"),
        ("seed_color", "Region seed must match"),
        ("inventory", "Region definitions must match"),
        ("invalid_hash", "Expected a SHA-256"),
    ] {
        let f = Fixture::new();
        let root = f.temp.path();
        let mut profile = read(root.join("profile.json"));
        match case {
            "size" => profile["regions"][0]["size"] = json!([7, 1]),
            "seed_outside" => profile["regions"][0]["seeds"] = json!([[8, 0]]),
            "seed_color" => profile["regions"][0]["seeds"] = json!([[1, 0]]),
            "inventory" => profile["regions"][0]["asset"] = json!("missing.png"),
            "invalid_hash" => profile["regions"][0]["source_sha256"] = json!("not-a-hash"),
            _ => unreachable!(),
        }
        write(root.join("profile.json"), &profile);
        for command in ["apply", "build-presets"] {
            let out = root.join(command);
            failure(f.generate(command, &out, true), message);
            assert!(!out.exists());
        }
    }
}
#[test]
fn validation_override_requires_a_palette() {
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args([
            "validate",
            "--original",
            "missing",
            "--modified",
            "missing",
            FLAG,
        ])
        .output()
        .unwrap();
    failure(result, "--palette");
}
