use serde_json::{Value, json};
use std::{fs, process::Command};

fn run_controls(case: &str) {
    let interpreter = std::env::var_os("FOM_GML_INTERPRETER").expect("Set FOM_GML_INTERPRETER");
    let characters: Vec<Value> =
        serde_json::from_str(include_str!("../mod/config/characters.json")).unwrap();
    assert_eq!(characters.len(), 36);
    let presets: &[(&str, &str)] = if matches!(case, "max_presets" | "presets") {
        &[
            ("vanilla", "Vanilla"),
            ("blue", "Debug Blue"),
            ("npc_hayden", "Hayden palette"),
            ("npc_ryis", "Ryis palette"),
            ("npc_seridia", "Seridia palette"),
            ("player_04", "Light peach (Player 04)"),
            ("npc_march", "Light golden (March)"),
            ("npc_wiscar", "Warm peach (Wiscar)"),
            ("player_16", "Warm brown (Player 16)"),
            ("player_22", "Deep brown (Player 22)"),
            ("player_35", "Very deep (Player 35)"),
            ("player_27", "Deep umber (Player 27)"),
            ("npc_stillwell", "Muted light (Stillwell)"),
            ("npc_terithia", "Muted medium (Terithia)"),
        ]
    } else {
        &[
            ("vanilla", "Vanilla"),
            ("candidate", "Candidate"),
            ("other", "Other"),
        ]
    };
    let definitions: Vec<Value> = characters
        .iter()
        .enumerate()
        .map(|(i, character)| {
            let n = 1000 * i;
            json!([
                character["id"],
                character["label"],
                character["hotkey"],
                presets.iter().map(|(_, label)| label).collect::<Vec<_>>(),
                [
                    (0..presets.len())
                        .map(|index| format!("p{}", n + 10 * (index + 1)))
                        .collect::<Vec<_>>(),
                    (0..presets.len())
                        .map(|index| format!("spr_npc_{}", n + 300 + index))
                        .collect::<Vec<_>>()
                ],
                presets.iter().map(|(id, _)| id).collect::<Vec<_>>()
            ])
        })
        .collect();
    let temp = tempfile::tempdir().unwrap();
    let script = temp.path().join("palette_controls.gml");
    fs::write(
        &script,
        [
            include_str!("gml/palette_controls_stubs.gml").to_owned(),
            format!(
                "function lns_palette_definitions() {{ var definitions = {};
                 if (global.definitions_reordered) {{
                     for (var i = 0; i < array_length(definitions); i++) {{
                         definitions[i][3] = [\"Vanilla\",\"Other\",\"Candidate\"];
                         definitions[i][5] = [\"vanilla\",\"other\",\"candidate\"];
                         for (var j = 0; j < 2; j++) {{
                             var pair = definitions[i][4][j];
                             definitions[i][4][j] = [pair[0], pair[2], pair[1]];
                         }}
                     }}
                 }} return definitions; }}",
                json!(definitions)
            ),
            include_str!("../mod/toggle/gml/palette_toggle.gml").to_owned(),
            include_str!("../mod/toggle/gml/palette_controls.gml").to_owned(),
            format!("global.controls_case = {};", json!(case)),
            include_str!("gml/palette_controls_cases.gml").to_owned(),
        ]
        .join("\n"),
    )
    .unwrap();
    let result = Command::new(interpreter)
        .arg("run")
        .arg(script)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
#[ignore = "requires the pinned Fabricator interpreter in FOM_GML_INTERPRETER"]
fn title_settings_initializes_palettes_before_game_begin_step() {
    run_controls("title");
}

#[test]
#[ignore = "requires the pinned Fabricator interpreter in FOM_GML_INTERPRETER"]
fn title_save_failure_shows_a_native_notice_without_stealing_focus() {
    run_controls("title_notice");
}

#[test]
#[ignore = "requires the pinned Fabricator interpreter in FOM_GML_INTERPRETER"]
fn native_palette_page_preserves_menu_and_animation_state() {
    run_controls("menu");
}

#[test]
#[ignore = "requires the pinned Fabricator interpreter in FOM_GML_INTERPRETER"]
fn palette_storage_uses_stable_ids_and_confirms_primary_writes() {
    run_controls("storage");
}

#[test]
#[ignore = "requires the pinned Fabricator interpreter in FOM_GML_INTERPRETER"]
fn fourteen_choices_preserve_navigation_animation_and_version_one_storage() {
    run_controls("max_presets");
}

#[test]
#[ignore = "requires the pinned Fabricator interpreter in FOM_GML_INTERPRETER"]
fn presets_set_every_character_and_save_once() {
    run_controls("presets");
}

#[test]
#[ignore = "requires the pinned Fabricator interpreter in FOM_GML_INTERPRETER"]
fn presets_fall_back_to_vanilla_without_the_shared_tones() {
    run_controls("presets_unavailable");
}
