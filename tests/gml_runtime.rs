use std::{fs, process::Command};

#[test]
#[ignore = "requires the pinned Fabricator interpreter in FOM_GML_INTERPRETER"]
fn toggle_preserves_frames_and_handles_menu_lifecycle() {
    let interpreter = std::env::var_os("FOM_GML_INTERPRETER")
        .expect("Set FOM_GML_INTERPRETER to the pinned Fabricator interpreter");
    let temp = tempfile::tempdir().unwrap();
    let script = temp.path().join("toggle_test.gml");
    fs::write(
        &script,
        [
            include_str!("gml/engine_stub.gml"),
            include_str!("../mod/toggle/gml/palette_toggle.gml"),
            include_str!("gml/toggle_cases.gml"),
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
fn world_palette_follows_selection_without_changing_animation_or_movement() {
    let interpreter = std::env::var_os("FOM_GML_INTERPRETER").expect("Set FOM_GML_INTERPRETER");
    let temp = tempfile::tempdir().unwrap();
    let script = temp.path().join("world_test.gml");
    fs::write(
        &script,
        [
            include_str!("gml/engine_stub.gml"),
            include_str!("../mod/toggle/gml/palette_toggle.gml"),
            include_str!("gml/world_cases.gml"),
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
