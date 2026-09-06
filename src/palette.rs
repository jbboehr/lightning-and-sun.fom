use crate::assets::{Files, digest};
use anyhow::{Context, Result, ensure};
use image::RgbaImage;
use serde::{
    Deserialize, Deserializer,
    de::{self, MapAccess, Visitor},
};
use std::{
    collections::BTreeMap,
    fmt, fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Recipe {
    #[serde(default, rename = "description")]
    _description: Option<String>,
    #[serde(deserialize_with = "unique_map")]
    rgba_map: BTreeMap<String, String>,
    #[serde(default, deserialize_with = "regions")]
    regions: Option<Vec<Region>>,
    #[serde(default, deserialize_with = "profile")]
    profile: Option<PathBuf>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub source_colors: Vec<String>,
    pub regions: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Region {
    asset: String,
    source_sha256: String,
    size: [u32; 2],
    seeds: Vec<[u32; 2]>,
}

// Missing means unrestricted; an explicit null must not silently disable masks.
fn regions<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Vec<Region>>, D::Error> {
    Vec::deserialize(de).map(Some)
}

fn profile<'de, D: Deserializer<'de>>(de: D) -> Result<Option<PathBuf>, D::Error> {
    PathBuf::deserialize(de).map(Some)
}

pub struct Palette {
    pub mapping: BTreeMap<[u8; 4], [u8; 4]>,
    regions: Option<BTreeMap<String, Region>>,
}

impl Palette {
    pub fn assets(&self) -> Option<Vec<String>> {
        self.regions
            .as_ref()
            .map(|regions| regions.keys().cloned().collect())
    }

    pub fn check_inventory(&self, images: &Files) -> Result<()> {
        if let Some(regions) = &self.regions {
            let names: std::collections::BTreeSet<_> =
                images.keys().map(|p| p.replace('\\', "/")).collect();
            ensure!(
                names.iter().eq(regions.keys()),
                "Region definitions must match the exact input PNG set"
            );
        }
        Ok(())
    }

    pub fn mask(&self, name: &str, bytes: &[u8], image: &RgbaImage) -> Result<Option<Vec<bool>>> {
        let Some(regions) = &self.regions else {
            return Ok(None);
        };
        let region = regions
            .get(&name.replace('\\', "/"))
            .context("Missing region definition")?;
        ensure!(
            digest(bytes) == region.source_sha256,
            "Region source checksum mismatch: {name}; review the mask against this image"
        );
        ensure!(
            region.size == [image.width(), image.height()],
            "Region dimensions differ: {name}"
        );
        let eligible = |x, y| {
            let pixel = image.get_pixel(x, y);
            pixel[3] != 0 && self.mapping.contains_key(&pixel.0)
        };
        let index = |x: u32, y: u32| y as usize * image.width() as usize + x as usize;
        let mut selected = vec![false; image.as_raw().len() / 4];
        let mut stack = Vec::new();
        for &[x, y] in &region.seeds {
            ensure!(
                x < image.width() && y < image.height(),
                "Region seed outside image: {name} [{x},{y}]"
            );
            ensure!(
                eligible(x, y),
                "Region seed must match a nontransparent source color: {name} [{x},{y}]"
            );
            if !selected[index(x, y)] {
                selected[index(x, y)] = true;
                stack.push((x, y));
            }
        }
        // Flood only original source colors, with four-way connectivity. Seed overlap
        // is harmless, and transparent pixels cannot connect separate regions.
        while let Some((x, y)) = stack.pop() {
            for (nx, ny) in [
                x.checked_sub(1).map(|nx| (nx, y)),
                y.checked_sub(1).map(|ny| (x, ny)),
                Some((x + 1, y)),
                Some((x, y + 1)),
            ]
            .into_iter()
            .flatten()
            {
                if nx < image.width()
                    && ny < image.height()
                    && !selected[index(nx, ny)]
                    && eligible(nx, ny)
                {
                    selected[index(nx, ny)] = true;
                    stack.push((nx, ny));
                }
            }
        }
        Ok(Some(selected))
    }
}

fn unique_map<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<BTreeMap<String, String>, D::Error> {
    struct UniqueMap;
    impl<'de> Visitor<'de> for UniqueMap {
        type Value = BTreeMap<String, String>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an rgba_map object with distinct source colors")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut result = BTreeMap::new();
            while let Some((key, value)) = map.next_entry::<String, String>()? {
                if result.insert(key.clone(), value).is_some() {
                    return Err(de::Error::custom(format!("Duplicate JSON key: {key}")));
                }
            }
            Ok(result)
        }
    }
    deserializer.deserialize_map(UniqueMap)
}

fn color(value: &str) -> Result<[u8; 4]> {
    ensure!(
        (value.len() == 7 || value.len() == 9)
            && value.starts_with('#')
            && value[1..].bytes().all(|b| b.is_ascii_hexdigit()),
        "Expected #RRGGBB or #RRGGBBAA: {value}"
    );
    let mut result = [255; 4];
    for (i, channel) in result.iter_mut().take((value.len() - 1) / 2).enumerate() {
        *channel = u8::from_str_radix(&value[1 + 2 * i..3 + 2 * i], 16)?;
    }
    Ok(result)
}

pub fn load(path: &Path) -> Result<Palette> {
    let mut recipe: Recipe = serde_json::from_slice(&fs::read(path)?)?;
    if let Some(profile) = recipe.profile.take() {
        ensure!(
            recipe.regions.is_none(),
            "Use a profile or inline regions, not both"
        );
        let profile: Profile = serde_json::from_slice(&fs::read(
            path.parent().unwrap_or(Path::new(".")).join(profile),
        )?)?;
        let source_colors = profile
            .source_colors
            .iter()
            .map(|s| color(s))
            .collect::<Result<std::collections::BTreeSet<_>>>()?;
        ensure!(
            !source_colors.is_empty()
                && source_colors.len() == profile.source_colors.len()
                && !profile.regions.is_empty(),
            "Profile needs distinct source colors and reviewed regions"
        );
        let mapping_colors = recipe
            .rgba_map
            .keys()
            .map(|s| color(s))
            .collect::<Result<std::collections::BTreeSet<_>>>()?;
        ensure!(
            source_colors == mapping_colors,
            "Palette source colors differ from profile"
        );
        recipe.regions = Some(serde_json::from_value(serde_json::Value::Array(
            profile.regions,
        ))?);
    }
    decode(recipe)
}

pub fn parse(bytes: &[u8]) -> Result<Palette> {
    let palette: Recipe = serde_json::from_slice(bytes)?;
    ensure!(
        palette.profile.is_none(),
        "Profile references require a palette file path"
    );
    decode(palette)
}

fn decode(palette: Recipe) -> Result<Palette> {
    let mut mapping = BTreeMap::new();
    for (source, target) in palette.rgba_map {
        let (source, target) = (color(&source)?, color(&target)?);
        ensure!(
            !mapping.contains_key(&source),
            "Duplicate normalized source color"
        );
        ensure!(
            source[3] == target[3],
            "Palette mappings must preserve alpha"
        );
        ensure!(
            source[3] != 0 || source == target,
            "Fully transparent pixels must remain unchanged"
        );
        mapping.insert(source, target);
    }
    let regions = palette
        .regions
        .map(|entries| -> Result<_> {
            let mut regions = BTreeMap::new();
            for mut region in entries {
                ensure!(
                    !region.asset.contains('\\')
                        && region
                            .asset
                            .split('/')
                            .all(|p| !p.is_empty() && p != "." && p != ".."),
                    "Region asset must be an exact relative PNG path"
                );
                ensure!(
                    region.source_sha256.len() == 64
                        && region.source_sha256.bytes().all(|b| b.is_ascii_hexdigit()),
                    "Expected a SHA-256 source checksum"
                );
                region.source_sha256.make_ascii_lowercase();
                ensure!(
                    regions.insert(region.asset.clone(), region).is_none(),
                    "Duplicate region asset"
                );
            }
            Ok(regions)
        })
        .transpose()?;
    Ok(Palette { mapping, regions })
}
