use serde_json::{Value, json};
use std::{fs, process::Command};

fn run_controls(case: &str) {
    let interpreter = std::env::var_os("FOM_GML_INTERPRETER").expect("Set FOM_GML_INTERPRETER");
    let characters: Vec<Value> =
        serde_json::from_str(include_str!("../mod/config/characters.json")).unwrap();
    assert_eq!(characters.len(), 36);
    let definitions: Vec<Value> = characters
        .iter()
        .enumerate()
        .map(|(i, character)| {
            let n = 1000 * i;
            json!([
                character["id"],
                character["label"],
                character["hotkey"],
                ["Vanilla", "Candidate", "Other"],
                [
                    [
                        format!("p{}", n + 10),
                        format!("p{}", n + 20),
                        format!("p{}", n + 30)
                    ],
                    [
                        format!("spr_npc_{}", n + 300),
                        format!("spr_npc_{}", n + 301),
                        format!("spr_npc_{}", n + 302)
                    ]
                ],
                ["vanilla", "candidate", "other"]
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
fn native_palette_page_preserves_menu_and_animation_state() {
    run_controls("menu");
}

#[test]
#[ignore = "requires the pinned Fabricator interpreter in FOM_GML_INTERPRETER"]
fn palette_storage_uses_stable_ids_and_confirms_primary_writes() {
    run_controls("storage");
}
