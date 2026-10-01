use serde_json::Value;
use std::{collections::BTreeMap, collections::BTreeSet, fs, path::Path};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn tracked_character_profiles_match_reviewed_coverage() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    // Growing profile totals belong here, not in historical pixel tests.
    // Fixed slice boundaries and accepted output baselines stay in those tests.
    let expected = BTreeMap::from([
        ("adeline", 282),
        ("hayden", 305),
        ("ryis", 273),
        ("reina", 278),
        ("juniper", 305),
        ("celine", 390),
        ("march", 398),
        ("balor", 262),
        ("valen", 257),
        ("eiland", 260),
        ("olric", 36),
        ("landen", 32),
        ("nora", 32),
        ("holt", 32),
        ("josephine", 32),
        ("darcy", 32),
        ("dell", 32),
        ("elsie", 36),
        ("errol", 36),
        ("hemlock", 32),
        ("louis", 32),
        ("luc", 32),
        ("maple", 32),
        ("merri", 32),
        ("terithia", 32),
        ("stillwell", 36),
        ("taliferro", 36),
        ("vera", 32),
        ("wheedle", 32),
        ("zorel", 32),
        ("darren", 8),
        ("linnet", 8),
        ("wiscar", 8),
        ("wynne", 8),
        ("caldarus", 98),
        ("seridia", 164),
    ]);
    let registry = read(root.join("mod/config/characters.json"));
    let registered: BTreeMap<_, _> = registry
        .as_array()
        .unwrap()
        .iter()
        .map(|c| (c["id"].as_str().unwrap(), c))
        .collect();
    assert_eq!(registered.len(), registry.as_array().unwrap().len());
    assert_eq!(
        registered.keys().collect::<Vec<_>>(),
        expected.keys().collect::<Vec<_>>()
    );

    let sets = root.join("palettes/sets");
    let collection = read(sets.join("characters-trial.json"));
    let mut actual = BTreeMap::new();
    for character in collection["characters"].as_array().unwrap() {
        let id = character["id"].as_str().unwrap();
        let set_path = sets.join(character["presets"].as_str().unwrap());
        let set = read(&set_path);
        let profile = read(
            set_path
                .parent()
                .unwrap()
                .join(set["profile"].as_str().unwrap()),
        );
        let regions = profile["regions"].as_array().unwrap();
        assert!(
            actual.insert(id, regions.len()).is_none(),
            "duplicate character {id}"
        );
        let assets: BTreeSet<_> = regions
            .iter()
            .map(|region| region["asset"].as_str().unwrap())
            .collect();
        assert_eq!(assets.len(), regions.len(), "duplicate asset in {id}");
        let registered_assets: BTreeSet<_> = registered[id]["animations"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            assets, registered_assets,
            "profile/registry coverage for {id}"
        );
    }
    assert_eq!(actual, expected, "reviewed source totals");
}
