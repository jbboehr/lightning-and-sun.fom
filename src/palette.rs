use anyhow::{Result, ensure};
use serde::{
    Deserialize, Deserializer,
    de::{self, MapAccess, Visitor},
};
use std::{collections::BTreeMap, fmt, fs, path::Path};

#[derive(Deserialize)]
struct Palette {
    #[serde(deserialize_with = "unique_map")]
    rgba_map: BTreeMap<String, String>,
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

pub fn load(path: &Path) -> Result<BTreeMap<[u8; 4], [u8; 4]>> {
    let palette: Palette = serde_json::from_slice(&fs::read(path)?)?;
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
    Ok(mapping)
}
