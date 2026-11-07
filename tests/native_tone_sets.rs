use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};

const ADDITIONS: [&str; 7] = [
    "player_04",
    "npc_march",
    "npc_wiscar",
    "player_16",
    "player_22",
    "player_35",
    "player_27",
];

const HISTORICAL: [(&str, &str); 36] = [
    ("adeline", "adeline-world-finish-trial.json"),
    ("hayden", "hayden-world-trial.json"),
    ("ryis", "ryis-world-trial.json"),
    ("reina", "reina-mouth-corrected-trial.json"),
    ("juniper", "juniper-world-finish-trial.json"),
    ("celine", "celine-world-trial.json"),
    ("march", "march-world-trial.json"),
    ("balor", "balor-world-trial.json"),
    ("valen", "valen-world-trial.json"),
    ("eiland", "eiland-world-trial.json"),
    ("olric", "olric-world-trial.json"),
    ("landen", "landen-world-trial.json"),
    ("nora", "nora-world-trial.json"),
    ("holt", "holt-world-trial.json"),
    ("josephine", "josephine-world-trial.json"),
    ("darcy", "darcy-world-trial.json"),
    ("dell", "dell-world-trial.json"),
    ("elsie", "elsie-world-trial.json"),
    ("errol", "errol-world-trial.json"),
    ("hemlock", "hemlock-world-trial.json"),
    ("louis", "louis-world-trial.json"),
    ("luc", "luc-world-trial.json"),
    ("maple", "maple-world-trial.json"),
    ("merri", "merri-world-trial.json"),
    ("terithia", "terithia-world-trial.json"),
    ("stillwell", "stillwell-world-trial.json"),
    ("taliferro", "taliferro-world-trial.json"),
    ("vera", "vera-world-trial.json"),
    ("wheedle", "wheedle-world-trial.json"),
    ("zorel", "zorel-world-trial.json"),
    ("darren", "darren-world-trial.json"),
    ("linnet", "linnet-world-trial.json"),
    ("wiscar", "wiscar-world-trial.json"),
    ("wynne", "wynne-world-trial.json"),
    ("caldarus", "caldarus-world-trial.json"),
    ("seridia", "seridia-world-trial.json"),
];

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap())
        .unwrap()
}

fn rgba(value: &Value) -> [i32; 4] {
    let value = value.as_str().unwrap().strip_prefix('#').unwrap();
    assert!(value.len() == 6 || value.len() == 8, "invalid RGBA {value}");
    let mut result = [0, 0, 0, 255];
    for (index, channel) in result.iter_mut().enumerate().take(value.len() / 2) {
        *channel = i32::from_str_radix(&value[index * 2..index * 2 + 2], 16).unwrap();
    }
    assert_eq!(result[3], 255, "nonopaque target {value}");
    result
}

fn native_set(character: &str) -> Value {
    read(format!("palettes/sets/{character}-native-tones.json"))
}

#[test]
fn native_donors_bind_to_the_original_four_role_catalogs() {
    let catalog = read("palettes/catalog/native-tones.json");
    let presets = catalog["presets"].as_array().unwrap();
    assert_eq!(presets.len(), ADDITIONS.len());
    for (preset, id) in presets.iter().zip(ADDITIONS) {
        assert_eq!(preset["id"], id);
        assert_eq!(preset["source_id"], id);
        let source_name = preset["source_catalog"].as_str().unwrap();
        assert_eq!(
            source_name,
            if id.starts_with("npc_") {
                "npc-spring.json"
            } else {
                "player.json"
            }
        );
        let source = read(format!("palettes/catalog/{source_name}"));
        let ramp = source[if id.starts_with("npc_") {
            "ramps"
        } else {
            "presets"
        }]
        .as_array()
        .unwrap()
        .iter()
        .find(|ramp| ramp["id"] == id)
        .unwrap();
        let colors = preset["colors"].as_array().unwrap();
        assert_eq!(colors.len(), 4);
        assert_eq!(colors, ramp["colors"].as_array().unwrap());
        colors.iter().for_each(|color| {
            rgba(color);
        });
    }
}

#[test]
fn expanded_sets_preserve_all_historical_presets_and_source_bound_profiles() {
    let catalog = read("palettes/catalog/native-tones.json");
    let donors = catalog["presets"].as_array().unwrap();
    let mut total_shared_slots = 0;
    for (character, historical_file) in HISTORICAL {
        let historical = read(format!("palettes/sets/{historical_file}"));
        let expanded = native_set(character);
        let mut expected_historical = historical["presets"].as_array().unwrap().clone();
        if character == "celine" {
            assert_eq!(
                expanded["profile"],
                "../profiles/celine-native-tone-blush.json"
            );
            for preset in &mut expected_historical {
                preset["colors"]
                    .as_array_mut()
                    .unwrap()
                    .extend(["#F4AEAE", "#FCB0B6", "#E6687A"].map(Value::from));
            }
        } else if character == "holt" {
            assert_eq!(
                expanded["profile"],
                "../profiles/holt-native-tone-base.json"
            );
        } else {
            assert_eq!(expanded["profile"], historical["profile"], "{character}");
        }
        let profile = read(Path::new("palettes/sets").join(expanded["profile"].as_str().unwrap()));
        let source_colors = profile["source_colors"].as_array().unwrap();
        assert!(!source_colors.is_empty(), "{character}");
        let unique_sources: BTreeSet<_> = source_colors.iter().map(rgba).collect();
        assert_eq!(unique_sources.len(), source_colors.len(), "{character}");
        total_shared_slots += source_colors.len();
        let presets = expanded["presets"].as_array().unwrap();
        let old_presets = &expected_historical;
        assert_eq!(old_presets.len(), 4, "historical contract: {character}");
        assert_eq!(presets.len(), 11, "{character}");
        assert_eq!(
            &presets[..4],
            old_presets,
            "historical choices: {character}"
        );
        for (preset, donor) in presets[4..].iter().zip(donors) {
            assert_eq!(preset["id"], donor["id"], "{character}");
            assert_eq!(preset["label"], donor["label"], "{character}");
        }
        let mut ids = BTreeSet::new();
        for (index, preset) in presets.iter().enumerate() {
            assert!(ids.insert(preset["id"].as_str().unwrap()), "{character}");
            assert!(!preset["label"].as_str().unwrap().is_empty());
            if character == "celine" && index >= 4 {
                assert_eq!(
                    preset["profile"],
                    "../profiles/celine-native-tone-face.json"
                );
            }
            let target_slots = if let Some(path) = preset.get("profile") {
                assert!(index >= 4, "historical override: {character}");
                assert_eq!(
                    path,
                    &Value::from(format!("../profiles/{character}-native-tone-face.json"))
                );
                let facial_profile = read(Path::new("palettes/sets").join(path.as_str().unwrap()));
                let facial_sources = facial_profile["source_colors"].as_array().unwrap();
                assert!(facial_sources.len() <= 64, "{character}");
                assert_eq!(&facial_sources[..source_colors.len()], source_colors);
                if let Some(groups) = profile["color_groups"].as_array() {
                    assert_eq!(
                        &facial_profile["color_groups"].as_array().unwrap()[..groups.len()],
                        groups
                    );
                }
                let old_regions = profile["regions"].as_array().unwrap();
                let regions = facial_profile["regions"].as_array().unwrap();
                assert_eq!(regions.len(), old_regions.len(), "{character}");
                for (region, old) in regions.iter().zip(old_regions) {
                    let old_seeds = old["seeds"].as_array().unwrap();
                    assert_eq!(
                        &region["seeds"].as_array().unwrap()[..old_seeds.len()],
                        old_seeds
                    );
                    let mut inherited = region.clone();
                    inherited["seeds"] = old["seeds"].clone();
                    assert_eq!(&inherited, old, "inherited binding: {character}");
                    if !old["asset"].as_str().unwrap().contains("/Portraits/") {
                        assert_eq!(region, old, "world mask: {character}");
                    }
                }
                facial_sources.len()
            } else {
                source_colors.len()
            };
            let colors = preset["colors"].as_array().unwrap();
            assert_eq!(colors.len(), target_slots, "{character}");
            colors.iter().for_each(|color| {
                rgba(color);
            });
        }
    }
    assert_eq!(total_shared_slots, 459);
}

#[test]
fn facial_overrides_retain_pre_correction_skin_targets() {
    let baseline = read("tests/fixtures/native-tone-face-baseline.json");
    assert_eq!(baseline["characters"].as_object().unwrap().len(), 36);
    for (character, old) in baseline["characters"].as_object().unwrap() {
        let current = native_set(character);
        if character == "holt" {
            assert_eq!(current["profile"], "../profiles/holt-native-tone-base.json");
        } else {
            assert_eq!(current["profile"], old["profile"]);
        }
        assert_eq!(old["presets"].as_array().unwrap().len(), 6);
        for (preset, before) in current["presets"].as_array().unwrap()[4..10]
            .iter()
            .zip(old["presets"].as_array().unwrap())
        {
            assert_eq!(preset["id"], before["id"]);
            let inherited = before["colors"].as_array().unwrap();
            assert_eq!(
                &preset["colors"].as_array().unwrap()[..inherited.len()],
                inherited,
                "existing targets: {character} {}",
                preset["id"]
            );
        }
    }
}

#[test]
fn holt_current_base_only_removes_reviewed_moustache_seeds() {
    let contract = read("tests/fixtures/holt-moustache-seeds.json");
    let mut expected = read("palettes/profiles/holt-world-trial.json");
    let changes = contract["regions"].as_array().unwrap();
    assert!(!changes.is_empty());
    let mut applied = BTreeSet::new();
    for region in expected["regions"].as_array_mut().unwrap() {
        let asset = region["asset"].as_str().unwrap().to_owned();
        if let Some(change) = changes.iter().find(|change| change["asset"] == asset) {
            assert!(asset.contains("/Holt/Portraits/"));
            assert!(applied.insert(asset));
            assert_eq!(region["source_sha256"], change["source_sha256"]);
            let removed = change["removed_seeds"].as_array().unwrap();
            assert!(!removed.is_empty());
            let seeds = region["seeds"].as_array_mut().unwrap();
            for seed in removed {
                let matches = seeds.iter().filter(|candidate| *candidate == seed).count();
                assert_eq!(matches, 1);
            }
            seeds.retain(|seed| !removed.contains(seed));
        }
    }
    assert_eq!(applied.len(), changes.len());
    assert_eq!(
        expected,
        read("palettes/profiles/holt-native-tone-base.json")
    );
}

#[test]
fn selected_collection_uses_all_expanded_sets() {
    let collection = read("palettes/sets/characters-trial.json");
    let characters = collection["characters"].as_array().unwrap();
    assert_eq!(characters.len(), HISTORICAL.len());
    let mut selected = BTreeSet::new();
    for character in characters {
        let id = character["id"].as_str().unwrap();
        assert!(selected.insert(id));
        assert!(HISTORICAL.iter().any(|(historical, _)| *historical == id));
        assert_eq!(character["presets"], format!("{id}-native-tones.json"));
    }
}

#[test]
fn reina_new_choices_retain_the_corrected_outer_mouth_skin_role() {
    let set = native_set("reina");
    assert_eq!(set["profile"], "../profiles/reina-mouth-corrected.json");
    let profile = read("palettes/profiles/reina-mouth-corrected.json");
    assert_eq!(profile["source_colors"][11], "#B2684A");
    for preset in set["presets"].as_array().unwrap() {
        assert_eq!(
            rgba(&preset["colors"][11]),
            rgba(&preset["colors"][0]),
            "{}",
            preset["id"]
        );
    }
}

#[test]
fn seridia_new_choices_preserve_literal_glow_offsets_with_channel_clamping() {
    let set = native_set("seridia");
    assert_eq!(set["profile"], "../profiles/seridia-world-trial.json");
    let profile = read("palettes/profiles/seridia-world-trial.json");
    assert_eq!(profile["source_colors"].as_array().unwrap().len(), 50);
    let glow = [
        (21, 3, [11, 6, 14]),
        (22, 3, [25, 13, 34]),
        (23, 3, [35, 19, 47]),
        (24, 3, [38, 20, 51]),
        (25, 3, [42, 23, 57]),
        (27, 2, [7, 2, 11]),
        (28, 2, [6, 2, 11]),
        (29, 2, [16, 5, 26]),
        (30, 2, [15, 4, 26]),
        (31, 2, [23, 7, 37]),
        (32, 2, [24, 8, 40]),
        (33, 2, [22, 6, 36]),
        (34, 2, [27, 9, 44]),
        (35, 2, [23, 7, 39]),
        (36, 2, [26, 8, 44]),
        (37, 1, [5, 0, 9]),
        (38, 1, [11, 0, 22]),
        (39, 1, [16, 0, 30]),
        (40, 1, [17, 0, 33]),
        (41, 1, [19, 0, 37]),
        (42, 0, [2, -2, 7]),
        (43, 0, [6, -5, 16]),
        (44, 0, [9, -8, 22]),
        (45, 0, [10, -8, 24]),
        (46, 0, [11, -9, 27]),
    ];
    for preset in set["presets"].as_array().unwrap() {
        for (slot, role, offsets) in glow {
            let base = rgba(&preset["colors"][role]);
            let mut expected = [0, 0, 0, 255];
            for channel in 0..3 {
                expected[channel] = (base[channel] + offsets[channel]).clamp(0, 255);
            }
            assert_eq!(
                rgba(&preset["colors"][slot]),
                expected,
                "{} slot {slot}",
                preset["id"]
            );
        }
    }
    assert_eq!(rgba(&set["presets"][4]["colors"][46])[0], 255);
}

#[test]
fn new_lip_treatments_keep_muted_donor_specific_contrast() {
    let nora = native_set("nora");
    let maple = native_set("maple");
    let expected = [
        ["#FBC7B8", "#D68C86", "#D0A397"],
        ["#EEC5A0", "#CC8B77", "#C9A586"],
        ["#DD9181", "#BD6966", "#BE7C73"],
        ["#D27A73", "#A6535A", "#975C59"],
        ["#8E4D4D", "#6D303C", "#673A40"],
        ["#4A202A", "#35141F", "#331D27"],
        ["#814538", "#582C2E", "#523632"],
    ];
    let brightness = |color: [i32; 4]| color[0] * 213 + color[1] * 715 + color[2] * 72;
    for (index, [highlight, middle, maple_lip]) in expected.into_iter().enumerate() {
        let nora_colors = &nora["presets"][index + 4]["colors"];
        let maple_colors = &maple["presets"][index + 4]["colors"];
        assert_eq!(rgba(&nora_colors[8]), rgba(&Value::from(highlight)));
        assert_eq!(rgba(&nora_colors[9]), rgba(&Value::from(middle)));
        assert_eq!(rgba(&maple_colors[9]), rgba(&Value::from(maple_lip)));
        assert!(brightness(rgba(&nora_colors[8])) > brightness(rgba(&nora_colors[9])));
        assert!(brightness(rgba(&nora_colors[8])) <= brightness(rgba(&nora_colors[0])) + 15_000);
        assert!(
            (brightness(rgba(&maple_colors[9])) - brightness(rgba(&maple_colors[1]))).abs()
                <= 30_000
        );
    }
}

#[test]
fn player27_append_preserves_the_complete_current_prefix_and_effective_profiles() {
    let frozen_catalog_path = "tests/fixtures/player27-six-family-catalog.json";
    let frozen_catalog = read(frozen_catalog_path);
    let bytes = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(frozen_catalog_path)).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(bytes)),
        "1cb71c3f49b88acb9540d0eb4a6658efb03aadddd555069a341cbc6e313b12be"
    );
    let catalog = read("palettes/catalog/native-tones.json");
    assert_eq!(
        &catalog["presets"].as_array().unwrap()[..6],
        frozen_catalog["presets"].as_array().unwrap()
    );
    assert_eq!(catalog["presets"][6]["id"], "player_27");
    assert_eq!(catalog["presets"][6]["label"], "Deep umber (Player 27)");
    let ramp = ["#813723FF", "#582718FF", "#431D11FF", "#2D130AFF"].map(Value::from);
    assert_eq!(catalog["presets"][6]["colors"].as_array().unwrap(), &ramp);

    let baseline = read("tests/fixtures/player27-ten-choice-baseline.json");
    let expected_targets = read("tests/fixtures/player27-expected-targets.json");
    let characters = baseline["characters"].as_object().unwrap();
    assert_eq!(characters.len(), 36);
    assert_eq!(expected_targets.as_object().unwrap().len(), 36);
    let mut effective_profiles = BTreeSet::new();
    let mut slots = 0;
    for (character, before) in characters {
        let mut current = native_set(character);
        assert_eq!(current["presets"].as_array().unwrap().len(), 11);
        let addition = current["presets"].as_array_mut().unwrap().pop().unwrap();
        assert_eq!(current, *before, "complete prior recipe: {character}");
        assert_eq!(addition["id"], "player_27");
        assert_eq!(addition["label"], "Deep umber (Player 27)");
        assert_eq!(
            addition["colors"], expected_targets[character],
            "{character}"
        );
        slots += addition["colors"].as_array().unwrap().len();
        let reference = before["presets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|preset| preset["id"] == "player_22")
            .unwrap();
        assert_eq!(
            addition.get("profile"),
            reference.get("profile"),
            "{character}"
        );
        for preset in before["presets"].as_array().unwrap() {
            let selected = preset.get("profile").unwrap_or(&before["profile"]);
            let path = format!(
                "palettes/{}",
                selected.as_str().unwrap().strip_prefix("../").unwrap()
            );
            if effective_profiles.insert(path.clone()) {
                let bytes = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(&path)).unwrap();
                assert_eq!(
                    format!("{:x}", Sha256::digest(bytes)),
                    baseline["profiles_sha256"][&path].as_str().unwrap(),
                    "effective profile: {character} {}",
                    preset["id"]
                );
            }
        }
    }
    assert_eq!(slots, 524);
    assert_eq!(
        effective_profiles,
        baseline["profiles_sha256"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect()
    );
    for (character, slot, expected) in [
        ("nora", 8, "#814538FF"),
        ("nora", 9, "#582C2EFF"),
        ("nora", 15, "#582C2EFF"),
        ("maple", 9, "#523632FF"),
        ("terithia", 12, "#843E33FF"),
        ("caldarus", 11, "#843E33FF"),
    ] {
        assert_eq!(
            native_set(character)["presets"][10]["colors"][slot],
            expected
        );
    }
}
