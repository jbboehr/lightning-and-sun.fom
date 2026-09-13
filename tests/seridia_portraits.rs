use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

const SOURCE: [u32; 20] = [
    0xC1AFA5, 0xA69084, 0x8E746D, 0x624A48, 0xB4A095, 0xB29E93, 0xAD7165, 0xFCDEBE, 0xF5BB90,
    0xEA9866, 0xB35A35, 0x9A4825, 0xF7C49D, 0xE28A55, 0xDA8A52, 0xC3AA9B, 0x95726A, 0xA5462C,
    0xF3AC87, 0xFFE1C5,
];
const TARGETS: [[u32; 20]; 4] = [
    [
        0x9DB9D4, 0x7F9FBD, 0x6687AD, 0x445F83, 0x90AAC4, 0x8EA8C2, 0x86809E, 0x9DB9D4, 0x7F9FBD,
        0x6687AD, 0x445F83, 0x445F83, 0x7F9FBD, 0x6687AD, 0x6687AD, 0x9DB9D4, 0x6687AD, 0x445F83,
        0x7D90B4, 0xA0BCDB,
    ],
    [
        0xE3A17B, 0xD48363, 0xC47054, 0x9F5544, 0xD6926B, 0xD49069, 0xDB6444, 0xE3A17B, 0xD48363,
        0xC47054, 0x9F5544, 0x9F5544, 0xD48363, 0xC47054, 0xC47054, 0xE3A17B, 0xC47054, 0x9F5544,
        0xD2745A, 0xE6A482,
    ],
    [
        0xE8B271, 0xCA9052, 0xB27146, 0x6E4922, 0xDBA361, 0xD9A15F, 0xD17133, 0xE8B271, 0xCA9052,
        0xB27146, 0x6E4922, 0x6E4922, 0xCA9052, 0xB27146, 0xB27146, 0xE8B271, 0xB27146, 0x6E4922,
        0xC88149, 0xEBB578,
    ],
    [
        0xB06C57, 0x814A3A, 0x63342A, 0x491F1B, 0xA35D47, 0xA15B45, 0x882B1B, 0xB06C57, 0x814A3A,
        0x63342A, 0x491F1B, 0x491F1B, 0x814A3A, 0x63342A, 0x63342A, 0xB06C57, 0x63342A, 0x491F1B,
        0x7F3B31, 0xB36F5E,
    ],
];
// Literal source-art landmarks, chosen independently of profile seeds.
type Landmark = (u32, u32, u32, bool);
const LANDMARKS: &[(&str, &[Landmark])] = &[
    (
        "spr_portrait_seridia_autumn_angry_brows.png",
        &[
            (172, 79, 0xC3AA9B, true),  // Autumn exposed hand light blend
            (172, 78, 0x95726A, true),  // Autumn hand skin beside glove
            (173, 76, 0x493F6B, false), // Autumn glove purple material
            (174, 76, 0x352D52, false), // Autumn glove dark edge
            (182, 71, 0xD0C0A2, false), // Autumn bone armor shadow
            (181, 71, 0xFFF9E8, false), // Autumn bone armor light
            (165, 85, 0x861A4C, false), // dark red nail
            (167, 90, 0xFFD565, false), // wrist gold ornament
        ],
    ),
    (
        "spr_portrait_seridia_autumn_blush.png",
        &[
            (155, 61, 0xC79A97, false), // original blush
        ],
    ),
    (
        "spr_portrait_seridia_autumn_neutral.png",
        &[
            (108, 156, 0x624A48, false), // Autumn white gauntlet dark line
            (126, 154, 0xC3AA9B, false), // Autumn white gauntlet pale line
            (109, 160, 0xC1AFA5, false), // Autumn gauntlet line blend
            (128, 168, 0xC1AFA5, true),  // Autumn exposed thigh beside gauntlet
            (126, 174, 0xC1AFA5, true),  // Autumn thigh lower edge beside gauntlet
        ],
    ),
    (
        "spr_portrait_seridia_autumn_ugh.png",
        &[
            (148, 62, 0x808DBA, false), // original expression marks
        ],
    ),
    (
        "spr_portrait_seridia_beach_bath_neutral.png",
        &[
            (158, 77, 0xA69084, true),   // horned hand medium
            (147, 77, 0x8E746D, true),   // horned neck shadow
            (151, 78, 0x624A48, true),   // horned hand deep crease
            (156, 142, 0xB29E93, true),  // bath torso subtle line
            (126, 92, 0xE7D5D5, false),  // white body marking blend
            (165, 93, 0xFFFFFF, false),  // white body marking
            (155, 159, 0xE7D5E1, false), // white body pink fringe
            (136, 79, 0xEF83BA, false),  // bath pink cloth
            (134, 83, 0xFFF9E8, false),  // bath white cloth stripe
        ],
    ),
    (
        "spr_portrait_seridia_dragon_neutral.png",
        &[
            (156, 79, 0x827888, false), // Dragon dark stone scale
            (452, 79, 0x827888, false), // Dragon dark stone scale
            (161, 89, 0xA0959D, false), // Dragon light stone scale
            (457, 89, 0xA0959D, false), // Dragon light stone scale
        ],
    ),
    (
        "spr_portrait_seridia_flashback_priestess_closed_eyes.png",
        &[
            (447, 80, 0xA5462C, true),  // Flashback speaking upper lip fringe
            (450, 82, 0xF3AC87, true),  // Flashback speaking outer lip blend
            (446, 83, 0xFFE1C5, true),  // Flashback speaking lower light blend
            (448, 80, 0x772E24, false), // Flashback mouth interior dark
            (450, 81, 0xA30A17, false), // Flashback mouth interior red
            (447, 81, 0xFC708D, false), // Flashback speaking pink mouth
            (447, 82, 0xD34864, false), // Flashback speaking dark pink mouth
        ],
    ),
    (
        "spr_portrait_seridia_flashback_priestess_eyes_shine.png",
        &[
            (152, 67, 0xFCDEBE, true),   // Flashback cheek light
            (153, 59, 0xF5BB90, true),   // Flashback forehead medium
            (152, 59, 0xEA9866, true),   // Flashback forehead shadow
            (142, 83, 0xB35A35, true),   // Flashback jaw deep
            (155, 77, 0x9A4825, true),   // Flashback nose contour
            (152, 73, 0xF7C49D, true),   // Flashback fine nose blend
            (140, 79, 0xE28A55, true),   // Flashback fine jaw blend
            (153, 91, 0xDA8A52, true),   // Flashback exposed neck below hair
            (135, 49, 0xF4CE9F, false),  // Flashback orange hair light
            (154, 49, 0xDA8A52, false),  // Flashback orange hair shade
            (138, 59, 0xDA8A52, false),  // Flashback diagonal bang shared shade
            (146, 59, 0xDA8A52, false),  // Flashback fine bang shared shade
            (145, 60, 0xDA8A52, false),  // Flashback bang fringe shared shade
            (138, 71, 0xDA8A52, false),  // Flashback long face adjacent hair shade
            (143, 72, 0x619278, false),  // Flashback green iris
            (142, 70, 0x38504D, false),  // Flashback deep green iris
            (148, 70, 0x6C6363, false),  // Flashback gray eye
            (147, 70, 0xC9C9CC, false),  // Flashback white eye edge
            (140, 50, 0xF5F5F5, false),  // Flashback hair shine
            (160, 58, 0xE3EFFF, false),  // Flashback hair shine blue fringe
            (151, 81, 0xE6687A, false),  // Flashback pink lip
            (151, 82, 0xF09DA3, false),  // Flashback light pink lip
            (157, 93, 0xF5F5F5, false),  // Flashback white glove at clasp
            (164, 106, 0x407096, false), // Flashback blue glove outline
            (153, 110, 0xADDADB, false), // Flashback blue glove sleeve
            (156, 93, 0xA3C0C4, false),  // Flashback glove shade
            (154, 105, 0xCDE1F9, false), // Flashback glove highlight
            (155, 99, 0xA1121D, false),  // Flashback red clasp
            (163, 92, 0xF4CE9F, false),  // Flashback orange braid
        ],
    ),
    (
        "spr_portrait_seridia_flashback_priestess_neutral.png",
        &[
            (152, 67, 0xFCDEBE, true),   // Flashback cheek light
            (153, 59, 0xF5BB90, true),   // Flashback forehead medium
            (152, 59, 0xEA9866, true),   // Flashback forehead shadow
            (142, 83, 0xB35A35, true),   // Flashback jaw deep
            (155, 77, 0x9A4825, true),   // Flashback nose contour
            (152, 73, 0xF7C49D, true),   // Flashback fine nose blend
            (140, 79, 0xE28A55, true),   // Flashback fine jaw blend
            (153, 91, 0xDA8A52, true),   // Flashback exposed neck below hair
            (135, 49, 0xF4CE9F, false),  // Flashback orange hair light
            (154, 49, 0xDA8A52, false),  // Flashback orange hair shade
            (138, 59, 0xDA8A52, false),  // Flashback diagonal bang shared shade
            (146, 59, 0xDA8A52, false),  // Flashback fine bang shared shade
            (145, 60, 0xDA8A52, false),  // Flashback bang fringe shared shade
            (138, 71, 0xDA8A52, false),  // Flashback long face adjacent hair shade
            (143, 72, 0x619278, false),  // Flashback green iris
            (142, 70, 0x38504D, false),  // Flashback deep green iris
            (148, 70, 0x6C6363, false),  // Flashback gray eye
            (147, 70, 0xC9C9CC, false),  // Flashback white eye edge
            (140, 50, 0xF5F5F5, false),  // Flashback hair shine
            (160, 58, 0xE3EFFF, false),  // Flashback hair shine blue fringe
            (151, 81, 0xE6687A, false),  // Flashback pink lip
            (151, 82, 0xF09DA3, false),  // Flashback light pink lip
            (157, 93, 0xF5F5F5, false),  // Flashback white glove at clasp
            (164, 106, 0x407096, false), // Flashback blue glove outline
            (153, 110, 0xADDADB, false), // Flashback blue glove sleeve
            (156, 93, 0xA3C0C4, false),  // Flashback glove shade
            (154, 105, 0xCDE1F9, false), // Flashback glove highlight
            (155, 99, 0xA1121D, false),  // Flashback red clasp
            (163, 92, 0xF4CE9F, false),  // Flashback orange braid
            (134, 117, 0xFCDEBE, true),  // Flashback exposed upper arm or thigh slit
            (134, 177, 0xFCDEBE, true),  // Flashback exposed upper arm or thigh slit
            (430, 117, 0xFCDEBE, true),  // Flashback exposed upper arm or thigh slit
            (430, 177, 0xFCDEBE, true),  // Flashback exposed upper arm or thigh slit
        ],
    ),
    (
        "spr_portrait_seridia_priestess_neutral.png",
        &[
            (149, 49, 0xB7C2CE, false), // spectral face light
            (140, 49, 0x8893CA, false), // spectral hair shade
            (145, 58, 0x6F5A9F, false), // spectral eye
            (148, 81, 0xA3C0C4, false), // spectral clothing
        ],
    ),
    (
        "spr_portrait_seridia_spring_neutral.png",
        &[
            (162, 43, 0xC1AFA5, true),   // forehead between circlet and hairline
            (162, 42, 0x8E746D, true),   // forehead hairline shadow
            (163, 41, 0x624A48, true),   // deepest forehead hairline edge
            (168, 44, 0x8E746D, true),   // forehead beside the horn
            (458, 43, 0xC1AFA5, true),   // same forehead in the speaking frame
            (458, 42, 0x8E746D, true),   // speaking-frame hairline shadow
            (459, 41, 0x624A48, true),   // speaking-frame hairline edge
            (464, 44, 0x8E746D, true),   // speaking-frame forehead beside horn
            (153, 64, 0xC1AFA5, true),   // horned cheek light
            (153, 68, 0xB4A095, true),   // upper lip skin blend
            (157, 71, 0xAD7165, true),   // outer lip skin fringe
            (153, 42, 0x655753, false),  // actual horn material
            (149, 42, 0x8E746D, false),  // shared shadow in horn
            (217, 139, 0xC1AFA5, false), // shared light in tail spine
            (216, 139, 0x8E746D, false), // shared shadow in tail spine
            (215, 137, 0x655753, false), // actual tail material
            (155, 51, 0x3D2E2D, false),  // actual brow core
            (153, 57, 0xA9A2A6, false),  // gray eye blend
            (150, 56, 0xE26BA2, false),  // pink iris
            (151, 54, 0x51122F, false),  // deep eye liner
            (152, 53, 0xC5768B, false),  // intentional pink makeup blend
            (148, 53, 0xC7496B, false),  // pink eye shadow
            (155, 70, 0xC7496B, false),  // pink upper lip
            (154, 70, 0xAD2C5C, false),  // actual lipstick dark pink
            (145, 57, 0xE7D5D5, false),  // white face marking blend
            (145, 58, 0xFFFFFF, false),  // white face marking
            (134, 42, 0x9BCDF3, false),  // actual blue hair
            (167, 42, 0x7087D1, false),  // actual blue hair shade
            (166, 42, 0x3E4F8E, false),  // actual dark blue hair contour
            (137, 42, 0x82B2ED, false),  // fine blue hair blend
            (133, 42, 0xD6EDFF, false),  // blue hair highlight
            (137, 64, 0xB36844, false),  // gold earring dark edge
            (139, 65, 0xE69935, false),  // gold earring
            (135, 65, 0xFFD565, false),  // gold earring highlight
            (158, 44, 0xE69935, false),  // forehead gold bead
            (164, 49, 0xFFFFFF, false),  // forehead white bead
            (157, 44, 0x624A48, false),  // shared brown forehead chain link
        ],
    ),
    (
        "spr_portrait_seridia_wedding_neutral.png",
        &[
            (162, 43, 0xC1AFA5, true),   // forehead above Wedding circlet
            (163, 47, 0xA69084, true),   // Wedding forehead shade above bead
            (458, 43, 0xC1AFA5, true),   // same forehead in the speaking frame
            (459, 47, 0xA69084, true),   // speaking-frame forehead shade
            (148, 84, 0x624A48, false),  // Wedding brown necklace bead backing
            (151, 78, 0x624A48, true),   // Wedding actual neck shadow above necklace
            (105, 131, 0x95726A, false), // Wedding gauntlet dark line
            (106, 135, 0xC3AA9B, false), // Wedding gauntlet pale line
            (121, 148, 0xC1AFA5, true),  // Wedding exposed thigh beside gauntlet
            (123, 148, 0xC1AFA5, true),  // Wedding exposed thigh interior
        ],
    ),
    (
        "spr_portrait_seridia_winter_neutral.png",
        &[
            (143, 152, 0x8E746D, false), // Winter lower white bone line
            (172, 160, 0xA69084, false), // Winter lower white bone medium line
        ],
    ),
];
fn rgba(color: u32) -> [u8; 4] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]
}

#[test]
#[ignore = "requires the 164 local strips in extracted/seridia-portraits-study"]
fn seridia_covers_human_skin_without_recoloring_hair_jewelry_armor_or_spectral_forms() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/seridia-portraits-study");
    let profile_path = root.join("palettes/profiles/seridia-portraits.json");
    let profile: Value = serde_json::from_slice(&fs::read(&profile_path).unwrap()).unwrap();
    let dragon_regions: Vec<_> = profile["regions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| {
            let name = r["asset"].as_str().unwrap();
            name.contains("/Dragon/") || name.contains("/Priestess/")
        })
        .collect();
    assert_eq!(dragon_regions.len(), 18);
    assert!(
        dragon_regions
            .iter()
            .all(|r| r["seeds"].as_array().unwrap().is_empty())
    );
    let temp = tempfile::tempdir().unwrap();
    let mut blue_selection = Vec::new();
    for (target_index, target) in TARGETS.iter().enumerate() {
        let map: serde_json::Map<String, Value> = SOURCE
            .iter()
            .zip(target)
            .map(|(source, target)| (format!("#{source:06X}"), json!(format!("#{target:06X}"))))
            .collect();
        let recipe = temp.path().join(format!("recipe-{target_index}.json"));
        fs::write(
            &recipe,
            serde_json::to_vec(&json!({"profile":profile_path,"rgba_map":map})).unwrap(),
        )
        .unwrap();
        let recipe = if target_index == 0 {
            std::env::var_os("FOM_SERIDIA_RECIPE")
                .map(std::path::PathBuf::from)
                .unwrap_or(recipe)
        } else {
            recipe
        };
        let modified = temp.path().join(format!("modified-{target_index}"));
        let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["apply", "--input"])
            .arg(&original)
            .arg("--palette")
            .arg(recipe)
            .arg("--output")
            .arg(&modified)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let report: Value = serde_json::from_slice(&result.stdout).unwrap();
        let rows = report["files"].as_array().unwrap();
        assert_eq!(rows.len(), 164);
        let mut selection = Vec::new();
        let mut changed = [0; 20];
        let mut skin_landmarks = 0;
        let mut protected_landmarks = 0;
        let mut unchanged_dragons = 0;
        let mut pixel_index = 0;
        for row in rows {
            let name = row["path"].as_str().unwrap();
            let before = image::open(original.join(name)).unwrap().to_rgba8();
            let after = image::open(modified.join(name)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), (592, 180));
            assert_eq!(after.dimensions(), before.dimensions());
            let meta = name.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(modified.join(&meta)).unwrap()
            );
            if name.contains("/Dragon/") || name.contains("/Priestess/") {
                assert_eq!(
                    fs::read(original.join(name)).unwrap(),
                    fs::read(modified.join(name)).unwrap()
                );
                assert_eq!(row["changed_pixels"], 0);
                unchanged_dragons += 1;
            }
            for (suffix, marks) in LANDMARKS {
                if !name.ends_with(suffix) {
                    continue;
                }
                for &(x, y, color, selected) in *marks {
                    assert_eq!(
                        before.get_pixel(x, y).0,
                        rgba(color),
                        "source art {name} [{x},{y}]"
                    );
                    let expected = if selected {
                        let shade = SOURCE.iter().position(|c| *c == color).unwrap();
                        skin_landmarks += 1;
                        rgba(target[shade])
                    } else {
                        protected_landmarks += 1;
                        rgba(color)
                    };
                    assert_eq!(
                        after.get_pixel(x, y).0,
                        expected,
                        "{} {name} [{x},{y}]",
                        if selected {
                            "missed skin"
                        } else {
                            "changed protected material"
                        }
                    );
                }
            }
            for (x, y, pixel) in before.enumerate_pixels() {
                let new = after.get_pixel(x, y);
                assert_eq!(pixel[3], new[3], "alpha {name} [{x},{y}]");
                if let Some(shade) = SOURCE.iter().position(|c| rgba(*c) == pixel.0) {
                    if pixel != new {
                        assert_eq!(new.0, rgba(target[shade]), "wrong color {name} [{x},{y}]");
                        changed[shade] += 1;
                    }
                    if target_index != 0 {
                        let expected = if blue_selection[pixel_index] {
                            rgba(target[shade])
                        } else {
                            pixel.0
                        };
                        assert_eq!(new.0, expected, "preset selection differs {name} [{x},{y}]");
                    }
                } else {
                    assert_eq!(pixel, new, "unrelated color {name} [{x},{y}]");
                }
                selection.push(pixel != new);
                pixel_index += 1;
            }
        }
        assert_eq!(skin_landmarks, 49);
        assert_eq!(protected_landmarks, 101);
        assert_eq!(unchanged_dragons, 18);
        assert!(changed.iter().all(|n| *n > 0));
        if target_index == 0 {
            blue_selection = selection;
        }
    }
}
