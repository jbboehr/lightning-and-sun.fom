use anyhow::{Context, Result, ensure};
use image::{DynamicImage, ImageFormat, RgbaImage, codecs::png::PngDecoder};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Cursor, Write},
    path::{Component, Path, PathBuf},
};

pub type Files = BTreeMap<String, PathBuf>;
pub type Outputs = BTreeMap<String, Vec<u8>>;

pub fn digest(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}

pub fn file_digest(path: &Path) -> Result<String> {
    let mut hash = Sha256::new();
    std::io::copy(&mut fs::File::open(path)?, &mut hash)?;
    Ok(format!("{:x}", hash.finalize()))
}

pub fn json_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut data = serde_json::to_vec_pretty(value)?;
    data.push(b'\n');
    Ok(data)
}

// Resolve existing ancestors before handling '..', including symlink aliases.
fn resolve(path: &Path) -> Result<PathBuf> {
    let absolute = std::env::current_dir()?.join(path);
    let mut resolved = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                resolved.pop();
            }
            component => {
                resolved.push(component);
                match fs::symlink_metadata(&resolved) {
                    Ok(_) => resolved = fs::canonicalize(&resolved)?,
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                    Err(error) => return Err(error.into()),
                }
            }
        }
    }
    Ok(resolved)
}

pub fn fresh_output(output: &Path, inputs: &[&Path]) -> Result<PathBuf> {
    ensure!(
        fs::symlink_metadata(output).is_err(),
        "Output already exists; choose a fresh path: {}",
        output.display()
    );
    let resolved = resolve(output)?;
    for input in inputs {
        let input = resolve(input)?;
        ensure!(
            !resolved.starts_with(&input) && !input.starts_with(&resolved),
            "Output overlaps an input: {}",
            output.display()
        );
    }
    ensure!(
        !resolved.exists(),
        "Output already exists: {}",
        resolved.display()
    );
    Ok(resolved)
}

pub fn inventory(root: &Path) -> Result<(Files, Files)> {
    ensure!(
        root.is_dir(),
        "Input directory does not exist: {}",
        root.display()
    );
    let root = fs::canonicalize(root)?;
    let (mut images, mut metadata) = (Files::new(), Files::new());
    for entry in walkdir::WalkDir::new(&root) {
        let entry = entry?;
        ensure!(
            !entry.file_type().is_symlink(),
            "Input symlinks are not supported: {}",
            entry.path().display()
        );
        if entry.file_type().is_file() {
            let relative = entry
                .path()
                .strip_prefix(&root)?
                .to_str()
                .context("Asset paths must be UTF-8")?
                .to_owned();
            if entry
                .path()
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
            {
                images.insert(relative, entry.path().to_owned());
            } else if relative.ends_with(".meta.toml") {
                metadata.insert(relative, entry.path().to_owned());
            }
        }
    }
    ensure!(!images.is_empty(), "No PNG files found: {}", root.display());
    Ok((images, metadata))
}

pub fn rgba(data: &[u8]) -> Result<RgbaImage> {
    let decoder = PngDecoder::new(Cursor::new(data)).context("Expected a static PNG")?;
    ensure!(
        !decoder.is_apng()?,
        "APNG is not supported; use a static animation strip"
    );
    Ok(DynamicImage::from_decoder(decoder)?.to_rgba8())
}

pub fn png(image: &DynamicImage) -> Result<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, ImageFormat::Png)?;
    Ok(buffer.into_inner())
}

pub fn write_new(path: &Path, data: &[u8]) -> Result<()> {
    fs::create_dir_all(path.parent().context("Output needs a parent directory")?)?;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(data)?;
    Ok(())
}

pub fn write_tree(root: &Path, outputs: Outputs) -> Result<()> {
    for name in outputs.keys() {
        for ancestor in Path::new(name).ancestors().skip(1) {
            ensure!(
                !outputs.contains_key(ancestor.to_str().unwrap()),
                "Conflicting output paths: {} and {name}",
                ancestor.display()
            );
        }
    }
    fs::create_dir_all(root.parent().context("Output needs a parent directory")?)?;
    fs::create_dir(root)?;
    for (name, data) in outputs {
        write_new(&root.join(name), &data)?;
    }
    Ok(())
}

pub fn compare(original: &Path, modified: &Path) -> Result<Value> {
    let (originals, original_meta) = inventory(original)?;
    let (modifieds, modified_meta) = inventory(modified)?;
    ensure!(
        originals.keys().eq(modifieds.keys()),
        "PNG file sets differ"
    );
    ensure!(
        original_meta.keys().eq(modified_meta.keys()),
        "Metadata file sets differ"
    );
    for (name, path) in &original_meta {
        ensure!(
            fs::read(path)? == fs::read(&modified_meta[name])?,
            "Metadata changed: {name}"
        );
    }
    let mut rows = Vec::new();
    let mut total = 0u64;
    for (name, path) in originals {
        let before = fs::read(path)?;
        let after = fs::read(&modifieds[&name])?;
        let left = rgba(&before).with_context(|| name.clone())?;
        let right = rgba(&after).with_context(|| name.clone())?;
        ensure!(
            left.dimensions() == right.dimensions(),
            "Dimensions changed: {name}"
        );
        ensure!(
            left.pixels().zip(right.pixels()).all(|(a, b)| a[3] == b[3]),
            "Alpha changed: {name}"
        );
        let changed = left
            .pixels()
            .zip(right.pixels())
            .filter(|(a, b)| a != b)
            .count() as u64;
        total += changed;
        rows.push(json!({"path":name,"size":[left.width(),left.height()],"alpha_preserved":true,
            "changed_pixels":changed,"original_sha256":digest(&before),"modified_sha256":digest(&after)}));
    }
    Ok(
        json!({"files":rows,"changed_pixels":total,"metadata_files":original_meta.keys().collect::<Vec<_>>()}),
    )
}
