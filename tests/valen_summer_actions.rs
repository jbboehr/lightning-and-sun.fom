use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

#[test]
#[ignore = "requires extracted/valen-summer-writing-study and the retained Summer-pilot Valen bundle"]
fn valen_summer_actions_cover_skin_and_preserve_reviewed_materials() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/valen-summer-writing-study");
    let profile_path = root.join("palettes/profiles/valen-world-trial.json");
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let profile = read(&profile_path);
    let set = read(&root.join("palettes/sets/valen-world-trial.json"));
    let cases: [(&str, &[usize]); 11] = [
        ("blink_east", &[60, 70, 60]),
        ("blink_south", &[71, 80, 71]),
        ("sit_north", &[14]),
        ("sit_south", &[59]),
        ("sit_east", &[47]),
        ("eat_north", &[15, 12, 15]),
        ("eat_south", &[60, 64, 52, 73, 59]),
        ("eat_east", &[42, 50, 51, 50, 45]),
        ("drink_north", &[15, 12, 15]),
        ("drink_south", &[59, 70, 59]),
        ("drink_east", &[46, 53, 46]),
    ];
    let rgba = |c: u32| [(c >> 16) as u8, (c >> 8) as u8, c as u8, 255];
    let skin = [0xFBD3A7, 0xEFA67A, 0xC37555, 0x762E21];
    // Source-grid landmarks cover every frame: exposed skin versus shirt,
    // bracelets, sandal straps, trousers, goggles and mouth interiors.
    let landmarks = [
        ("blink_east", 0, 39, 41, 0xFBD3A7, true),
        ("blink_east", 0, 40, 41, 0xEFA67A, true),
        ("blink_east", 0, 40, 40, 0xC37555, true),
        ("blink_east", 0, 35, 47, 0x762E21, true),
        ("blink_east", 0, 39, 42, 0xCD8D8B, false),
        ("blink_east", 0, 37, 42, 0xAB615F, false),
        ("blink_east", 0, 38, 41, 0x542F3A, false),
        ("blink_east", 0, 34, 45, 0x6D70AF, false),
        ("blink_east", 0, 35, 45, 0xAEB0DF, false),
        ("blink_east", 0, 38, 52, 0x6264A0, false),
        ("blink_east", 0, 39, 46, 0xE3DACA, false),
        ("blink_east", 0, 36, 28, 0xBD8E19, false),
        ("blink_east", 1, 39, 41, 0xFBD3A7, true),
        ("blink_east", 1, 40, 41, 0xEFA67A, true),
        ("blink_east", 1, 40, 40, 0xC37555, true),
        ("blink_east", 1, 35, 47, 0x762E21, true),
        ("blink_east", 1, 39, 42, 0xCD8D8B, false),
        ("blink_east", 1, 37, 42, 0xAB615F, false),
        ("blink_east", 1, 38, 41, 0x542F3A, false),
        ("blink_east", 1, 34, 45, 0x6D70AF, false),
        ("blink_east", 1, 35, 45, 0xAEB0DF, false),
        ("blink_east", 1, 38, 52, 0x6264A0, false),
        ("blink_east", 1, 39, 46, 0xE3DACA, false),
        ("blink_east", 1, 36, 28, 0xBD8E19, false),
        ("blink_east", 2, 39, 41, 0xFBD3A7, true),
        ("blink_east", 2, 40, 41, 0xEFA67A, true),
        ("blink_east", 2, 40, 40, 0xC37555, true),
        ("blink_east", 2, 35, 47, 0x762E21, true),
        ("blink_east", 2, 39, 42, 0xCD8D8B, false),
        ("blink_east", 2, 37, 42, 0xAB615F, false),
        ("blink_east", 2, 38, 41, 0x542F3A, false),
        ("blink_east", 2, 34, 45, 0x6D70AF, false),
        ("blink_east", 2, 35, 45, 0xAEB0DF, false),
        ("blink_east", 2, 38, 52, 0x6264A0, false),
        ("blink_east", 2, 39, 46, 0xE3DACA, false),
        ("blink_east", 2, 36, 28, 0xBD8E19, false),
        ("blink_south", 0, 38, 41, 0xFBD3A7, true),
        ("blink_south", 0, 39, 41, 0xEFA67A, true),
        ("blink_south", 0, 39, 40, 0xC37555, true),
        ("blink_south", 0, 33, 47, 0x762E21, true),
        ("blink_south", 0, 38, 42, 0xCD8D8B, false),
        ("blink_south", 0, 36, 42, 0xAB615F, false),
        ("blink_south", 0, 37, 41, 0x542F3A, false),
        ("blink_south", 0, 33, 45, 0x6D70AF, false),
        ("blink_south", 0, 34, 45, 0xAEB0DF, false),
        ("blink_south", 0, 37, 52, 0x6264A0, false),
        ("blink_south", 0, 39, 46, 0xE3DACA, false),
        ("blink_south", 0, 35, 28, 0xBD8E19, false),
        ("blink_south", 1, 38, 41, 0xFBD3A7, true),
        ("blink_south", 1, 39, 41, 0xEFA67A, true),
        ("blink_south", 1, 39, 40, 0xC37555, true),
        ("blink_south", 1, 33, 47, 0x762E21, true),
        ("blink_south", 1, 38, 42, 0xCD8D8B, false),
        ("blink_south", 1, 36, 42, 0xAB615F, false),
        ("blink_south", 1, 37, 41, 0x542F3A, false),
        ("blink_south", 1, 33, 45, 0x6D70AF, false),
        ("blink_south", 1, 34, 45, 0xAEB0DF, false),
        ("blink_south", 1, 37, 52, 0x6264A0, false),
        ("blink_south", 1, 39, 46, 0xE3DACA, false),
        ("blink_south", 1, 35, 28, 0xBD8E19, false),
        ("blink_south", 2, 38, 41, 0xFBD3A7, true),
        ("blink_south", 2, 39, 41, 0xEFA67A, true),
        ("blink_south", 2, 39, 40, 0xC37555, true),
        ("blink_south", 2, 33, 47, 0x762E21, true),
        ("blink_south", 2, 38, 42, 0xCD8D8B, false),
        ("blink_south", 2, 36, 42, 0xAB615F, false),
        ("blink_south", 2, 37, 41, 0x542F3A, false),
        ("blink_south", 2, 33, 45, 0x6D70AF, false),
        ("blink_south", 2, 34, 45, 0xAEB0DF, false),
        ("blink_south", 2, 37, 52, 0x6264A0, false),
        ("blink_south", 2, 39, 46, 0xE3DACA, false),
        ("blink_south", 2, 35, 28, 0xBD8E19, false),
        ("sit_north", 0, 34, 45, 0xEFA67A, true),
        ("sit_north", 0, 35, 44, 0xC37555, true),
        ("sit_north", 0, 34, 47, 0x762E21, true),
        ("sit_north", 0, 38, 42, 0xCD8D8B, false),
        ("sit_north", 0, 38, 41, 0xAB615F, false),
        ("sit_north", 0, 37, 41, 0x542F3A, false),
        ("sit_north", 0, 44, 45, 0x6D70AF, false),
        ("sit_north", 0, 45, 45, 0xAEB0DF, false),
        ("sit_north", 0, 38, 46, 0xE3DACA, false),
        ("sit_north", 0, 44, 28, 0xBD8E19, false),
        ("sit_south", 0, 38, 41, 0xFBD3A7, true),
        ("sit_south", 0, 39, 41, 0xEFA67A, true),
        ("sit_south", 0, 39, 40, 0xC37555, true),
        ("sit_south", 0, 34, 47, 0x762E21, true),
        ("sit_south", 0, 38, 42, 0xCD8D8B, false),
        ("sit_south", 0, 36, 42, 0xAB615F, false),
        ("sit_south", 0, 37, 41, 0x542F3A, false),
        ("sit_south", 0, 35, 45, 0x6D70AF, false),
        ("sit_south", 0, 34, 45, 0xAEB0DF, false),
        ("sit_south", 0, 37, 49, 0x6264A0, false),
        ("sit_south", 0, 39, 45, 0xE3DACA, false),
        ("sit_south", 0, 35, 28, 0xBD8E19, false),
        ("sit_east", 0, 39, 41, 0xFBD3A7, true),
        ("sit_east", 0, 40, 41, 0xEFA67A, true),
        ("sit_east", 0, 40, 40, 0xC37555, true),
        ("sit_east", 0, 35, 47, 0x762E21, true),
        ("sit_east", 0, 39, 42, 0xCD8D8B, false),
        ("sit_east", 0, 37, 42, 0xAB615F, false),
        ("sit_east", 0, 38, 41, 0x542F3A, false),
        ("sit_east", 0, 36, 45, 0x6D70AF, false),
        ("sit_east", 0, 35, 45, 0xAEB0DF, false),
        ("sit_east", 0, 44, 47, 0x6264A0, false),
        ("sit_east", 0, 40, 45, 0xE3DACA, false),
        ("sit_east", 0, 36, 28, 0xBD8E19, false),
        ("eat_north", 0, 44, 44, 0xFBD3A7, true),
        ("eat_north", 0, 34, 45, 0xEFA67A, true),
        ("eat_north", 0, 46, 42, 0xC37555, true),
        ("eat_north", 0, 45, 42, 0x762E21, true),
        ("eat_north", 0, 38, 42, 0xCD8D8B, false),
        ("eat_north", 0, 38, 41, 0xAB615F, false),
        ("eat_north", 0, 37, 41, 0x542F3A, false),
        ("eat_north", 0, 46, 45, 0x6D70AF, false),
        ("eat_north", 0, 46, 44, 0xAEB0DF, false),
        ("eat_north", 0, 38, 46, 0xE3DACA, false),
        ("eat_north", 0, 44, 28, 0xBD8E19, false),
        ("eat_north", 1, 44, 42, 0xEFA67A, true),
        ("eat_north", 1, 45, 41, 0xC37555, true),
        ("eat_north", 1, 45, 43, 0x762E21, true),
        ("eat_north", 1, 43, 42, 0xCD8D8B, false),
        ("eat_north", 1, 38, 42, 0xAB615F, false),
        ("eat_north", 1, 37, 42, 0x542F3A, false),
        ("eat_north", 1, 45, 42, 0x6D70AF, false),
        ("eat_north", 1, 44, 41, 0xAEB0DF, false),
        ("eat_north", 1, 38, 46, 0xE3DACA, false),
        ("eat_north", 1, 44, 29, 0xBD8E19, false),
        ("eat_north", 2, 44, 44, 0xFBD3A7, true),
        ("eat_north", 2, 34, 45, 0xEFA67A, true),
        ("eat_north", 2, 46, 42, 0xC37555, true),
        ("eat_north", 2, 45, 42, 0x762E21, true),
        ("eat_north", 2, 38, 42, 0xCD8D8B, false),
        ("eat_north", 2, 38, 41, 0xAB615F, false),
        ("eat_north", 2, 37, 41, 0x542F3A, false),
        ("eat_north", 2, 46, 45, 0x6D70AF, false),
        ("eat_north", 2, 46, 44, 0xAEB0DF, false),
        ("eat_north", 2, 38, 46, 0xE3DACA, false),
        ("eat_north", 2, 44, 28, 0xBD8E19, false),
        ("eat_south", 0, 38, 41, 0xFBD3A7, true),
        ("eat_south", 0, 39, 41, 0xEFA67A, true),
        ("eat_south", 0, 39, 40, 0xC37555, true),
        ("eat_south", 0, 45, 47, 0x762E21, true),
        ("eat_south", 0, 38, 42, 0xCD8D8B, false),
        ("eat_south", 0, 36, 42, 0xAB615F, false),
        ("eat_south", 0, 37, 41, 0x542F3A, false),
        ("eat_south", 0, 36, 45, 0x6D70AF, false),
        ("eat_south", 0, 35, 45, 0xAEB0DF, false),
        ("eat_south", 0, 37, 49, 0x6264A0, false),
        ("eat_south", 0, 39, 45, 0xE3DACA, false),
        ("eat_south", 0, 35, 28, 0xBD8E19, false),
        ("eat_south", 1, 38, 42, 0xFBD3A7, true),
        ("eat_south", 1, 39, 42, 0xEFA67A, true),
        ("eat_south", 1, 38, 40, 0xC37555, true),
        ("eat_south", 1, 37, 40, 0x762E21, true),
        ("eat_south", 1, 38, 43, 0xCD8D8B, false),
        ("eat_south", 1, 36, 43, 0xAB615F, false),
        ("eat_south", 1, 37, 42, 0x542F3A, false),
        ("eat_south", 1, 37, 46, 0x6D70AF, false),
        ("eat_south", 1, 36, 46, 0xAEB0DF, false),
        ("eat_south", 1, 41, 49, 0x6264A0, false),
        ("eat_south", 1, 41, 47, 0xE3DACA, false),
        ("eat_south", 1, 35, 29, 0xBD8E19, false),
        ("eat_south", 1, 39, 40, 0x9E2626, false),
        ("eat_south", 2, 37, 40, 0xFBD3A7, true),
        ("eat_south", 2, 38, 41, 0xEFA67A, true),
        ("eat_south", 2, 38, 40, 0xC37555, true),
        ("eat_south", 2, 36, 40, 0x762E21, true),
        ("eat_south", 2, 41, 42, 0xCD8D8B, false),
        ("eat_south", 2, 42, 42, 0xAB615F, false),
        ("eat_south", 2, 42, 41, 0x542F3A, false),
        ("eat_south", 2, 37, 49, 0x6264A0, false),
        ("eat_south", 2, 39, 45, 0xE3DACA, false),
        ("eat_south", 2, 35, 27, 0xBD8E19, false),
        ("eat_south", 2, 38, 35, 0x410808, false),
        ("eat_south", 2, 38, 36, 0x9E2626, false),
        ("eat_south", 3, 38, 42, 0xFBD3A7, true),
        ("eat_south", 3, 39, 40, 0xEFA67A, true),
        ("eat_south", 3, 38, 40, 0xC37555, true),
        ("eat_south", 3, 37, 40, 0x762E21, true),
        ("eat_south", 3, 41, 43, 0xCD8D8B, false),
        ("eat_south", 3, 42, 43, 0xAB615F, false),
        ("eat_south", 3, 42, 42, 0x542F3A, false),
        ("eat_south", 3, 37, 49, 0x6264A0, false),
        ("eat_south", 3, 37, 47, 0xE3DACA, false),
        ("eat_south", 3, 35, 29, 0xBD8E19, false),
        ("eat_south", 4, 38, 41, 0xFBD3A7, true),
        ("eat_south", 4, 39, 41, 0xEFA67A, true),
        ("eat_south", 4, 39, 40, 0xC37555, true),
        ("eat_south", 4, 34, 47, 0x762E21, true),
        ("eat_south", 4, 38, 42, 0xCD8D8B, false),
        ("eat_south", 4, 36, 42, 0xAB615F, false),
        ("eat_south", 4, 37, 41, 0x542F3A, false),
        ("eat_south", 4, 35, 45, 0x6D70AF, false),
        ("eat_south", 4, 34, 45, 0xAEB0DF, false),
        ("eat_south", 4, 37, 49, 0x6264A0, false),
        ("eat_south", 4, 39, 45, 0xE3DACA, false),
        ("eat_south", 4, 35, 28, 0xBD8E19, false),
        ("eat_east", 0, 41, 42, 0xFBD3A7, true),
        ("eat_east", 0, 44, 42, 0xEFA67A, true),
        ("eat_east", 0, 40, 40, 0xC37555, true),
        ("eat_east", 0, 44, 37, 0x762E21, true),
        ("eat_east", 0, 39, 42, 0xCD8D8B, false),
        ("eat_east", 0, 39, 41, 0xAB615F, false),
        ("eat_east", 0, 38, 41, 0x542F3A, false),
        ("eat_east", 0, 42, 43, 0x6D70AF, false),
        ("eat_east", 0, 42, 42, 0xAEB0DF, false),
        ("eat_east", 0, 44, 47, 0x6264A0, false),
        ("eat_east", 0, 40, 45, 0xE3DACA, false),
        ("eat_east", 0, 36, 28, 0xBD8E19, false),
        ("eat_east", 1, 46, 40, 0xFBD3A7, true),
        ("eat_east", 1, 45, 40, 0xEFA67A, true),
        ("eat_east", 1, 47, 41, 0xC37555, true),
        ("eat_east", 1, 46, 42, 0x762E21, true),
        ("eat_east", 1, 40, 41, 0xCD8D8B, false),
        ("eat_east", 1, 39, 41, 0xAB615F, false),
        ("eat_east", 1, 39, 43, 0x542F3A, false),
        ("eat_east", 1, 44, 42, 0x6D70AF, false),
        ("eat_east", 1, 44, 41, 0xAEB0DF, false),
        ("eat_east", 1, 44, 40, 0x6264A0, false),
        ("eat_east", 1, 40, 45, 0xE3DACA, false),
        ("eat_east", 1, 37, 28, 0xBD8E19, false),
        ("eat_east", 1, 41, 39, 0x9E2626, false),
        ("eat_east", 2, 43, 40, 0xFBD3A7, true),
        ("eat_east", 2, 44, 40, 0xEFA67A, true),
        ("eat_east", 2, 41, 49, 0xC37555, true),
        ("eat_east", 2, 44, 34, 0x762E21, true),
        ("eat_east", 2, 39, 41, 0xCD8D8B, false),
        ("eat_east", 2, 38, 42, 0xAB615F, false),
        ("eat_east", 2, 38, 41, 0x542F3A, false),
        ("eat_east", 2, 43, 41, 0x6D70AF, false),
        ("eat_east", 2, 42, 40, 0xAEB0DF, false),
        ("eat_east", 2, 44, 47, 0x6264A0, false),
        ("eat_east", 2, 40, 45, 0xE3DACA, false),
        ("eat_east", 2, 36, 27, 0xBD8E19, false),
        ("eat_east", 2, 40, 35, 0x410808, false),
        ("eat_east", 2, 40, 36, 0x9E2626, false),
        ("eat_east", 3, 41, 43, 0xFBD3A7, true),
        ("eat_east", 3, 40, 40, 0xEFA67A, true),
        ("eat_east", 3, 39, 40, 0xC37555, true),
        ("eat_east", 3, 38, 40, 0x762E21, true),
        ("eat_east", 3, 39, 43, 0xCD8D8B, false),
        ("eat_east", 3, 39, 42, 0xAB615F, false),
        ("eat_east", 3, 42, 41, 0x542F3A, false),
        ("eat_east", 3, 42, 43, 0xAEB0DF, false),
        ("eat_east", 3, 42, 44, 0x6264A0, false),
        ("eat_east", 3, 39, 46, 0xE3DACA, false),
        ("eat_east", 3, 36, 29, 0xBD8E19, false),
        ("eat_east", 4, 41, 42, 0xFBD3A7, true),
        ("eat_east", 4, 40, 41, 0xEFA67A, true),
        ("eat_east", 4, 40, 40, 0xC37555, true),
        ("eat_east", 4, 44, 37, 0x762E21, true),
        ("eat_east", 4, 39, 42, 0xCD8D8B, false),
        ("eat_east", 4, 39, 41, 0xAB615F, false),
        ("eat_east", 4, 38, 41, 0x542F3A, false),
        ("eat_east", 4, 40, 45, 0x6D70AF, false),
        ("eat_east", 4, 41, 44, 0xAEB0DF, false),
        ("eat_east", 4, 44, 47, 0x6264A0, false),
        ("eat_east", 4, 39, 46, 0xE3DACA, false),
        ("eat_east", 4, 36, 28, 0xBD8E19, false),
        ("drink_north", 0, 44, 44, 0xFBD3A7, true),
        ("drink_north", 0, 34, 45, 0xEFA67A, true),
        ("drink_north", 0, 46, 42, 0xC37555, true),
        ("drink_north", 0, 45, 42, 0x762E21, true),
        ("drink_north", 0, 38, 42, 0xCD8D8B, false),
        ("drink_north", 0, 38, 41, 0xAB615F, false),
        ("drink_north", 0, 37, 41, 0x542F3A, false),
        ("drink_north", 0, 46, 45, 0x6D70AF, false),
        ("drink_north", 0, 46, 44, 0xAEB0DF, false),
        ("drink_north", 0, 38, 46, 0xE3DACA, false),
        ("drink_north", 0, 44, 28, 0xBD8E19, false),
        ("drink_north", 1, 44, 42, 0xEFA67A, true),
        ("drink_north", 1, 45, 41, 0xC37555, true),
        ("drink_north", 1, 45, 43, 0x762E21, true),
        ("drink_north", 1, 43, 42, 0xCD8D8B, false),
        ("drink_north", 1, 38, 42, 0xAB615F, false),
        ("drink_north", 1, 37, 42, 0x542F3A, false),
        ("drink_north", 1, 45, 42, 0x6D70AF, false),
        ("drink_north", 1, 44, 41, 0xAEB0DF, false),
        ("drink_north", 1, 38, 46, 0xE3DACA, false),
        ("drink_north", 1, 44, 29, 0xBD8E19, false),
        ("drink_north", 2, 44, 44, 0xFBD3A7, true),
        ("drink_north", 2, 34, 45, 0xEFA67A, true),
        ("drink_north", 2, 46, 42, 0xC37555, true),
        ("drink_north", 2, 45, 42, 0x762E21, true),
        ("drink_north", 2, 38, 42, 0xCD8D8B, false),
        ("drink_north", 2, 38, 41, 0xAB615F, false),
        ("drink_north", 2, 37, 41, 0x542F3A, false),
        ("drink_north", 2, 46, 45, 0x6D70AF, false),
        ("drink_north", 2, 46, 44, 0xAEB0DF, false),
        ("drink_north", 2, 38, 46, 0xE3DACA, false),
        ("drink_north", 2, 44, 28, 0xBD8E19, false),
        ("drink_south", 0, 38, 41, 0xFBD3A7, true),
        ("drink_south", 0, 39, 41, 0xEFA67A, true),
        ("drink_south", 0, 39, 40, 0xC37555, true),
        ("drink_south", 0, 45, 47, 0x762E21, true),
        ("drink_south", 0, 38, 42, 0xCD8D8B, false),
        ("drink_south", 0, 37, 42, 0xAB615F, false),
        ("drink_south", 0, 37, 41, 0x542F3A, false),
        ("drink_south", 0, 37, 49, 0x6264A0, false),
        ("drink_south", 0, 39, 45, 0xE3DACA, false),
        ("drink_south", 0, 35, 28, 0xBD8E19, false),
        ("drink_south", 1, 35, 40, 0xFBD3A7, true),
        ("drink_south", 1, 39, 40, 0xEFA67A, true),
        ("drink_south", 1, 38, 40, 0xC37555, true),
        ("drink_south", 1, 42, 40, 0x762E21, true),
        ("drink_south", 1, 38, 43, 0xCD8D8B, false),
        ("drink_south", 1, 37, 43, 0xAB615F, false),
        ("drink_south", 1, 42, 42, 0x542F3A, false),
        ("drink_south", 1, 35, 43, 0x6D70AF, false),
        ("drink_south", 1, 34, 43, 0xAEB0DF, false),
        ("drink_south", 1, 37, 49, 0x6264A0, false),
        ("drink_south", 1, 39, 45, 0xE3DACA, false),
        ("drink_south", 1, 35, 29, 0xBD8E19, false),
        ("drink_south", 2, 38, 41, 0xFBD3A7, true),
        ("drink_south", 2, 39, 41, 0xEFA67A, true),
        ("drink_south", 2, 39, 40, 0xC37555, true),
        ("drink_south", 2, 45, 47, 0x762E21, true),
        ("drink_south", 2, 38, 42, 0xCD8D8B, false),
        ("drink_south", 2, 37, 42, 0xAB615F, false),
        ("drink_south", 2, 37, 41, 0x542F3A, false),
        ("drink_south", 2, 37, 49, 0x6264A0, false),
        ("drink_south", 2, 39, 45, 0xE3DACA, false),
        ("drink_south", 2, 35, 28, 0xBD8E19, false),
        ("drink_east", 0, 39, 41, 0xFBD3A7, true),
        ("drink_east", 0, 42, 42, 0xEFA67A, true),
        ("drink_east", 0, 40, 40, 0xC37555, true),
        ("drink_east", 0, 44, 37, 0x762E21, true),
        ("drink_east", 0, 37, 43, 0xCD8D8B, false),
        ("drink_east", 0, 37, 42, 0xAB615F, false),
        ("drink_east", 0, 38, 41, 0x542F3A, false),
        ("drink_east", 0, 39, 43, 0x6D70AF, false),
        ("drink_east", 0, 39, 44, 0xAEB0DF, false),
        ("drink_east", 0, 44, 47, 0x6264A0, false),
        ("drink_east", 0, 43, 45, 0xE3DACA, false),
        ("drink_east", 0, 36, 28, 0xBD8E19, false),
        ("drink_east", 1, 39, 40, 0xFBD3A7, true),
        ("drink_east", 1, 40, 40, 0xEFA67A, true),
        ("drink_east", 1, 40, 42, 0xC37555, true),
        ("drink_east", 1, 42, 35, 0x762E21, true),
        ("drink_east", 1, 36, 43, 0xCD8D8B, false),
        ("drink_east", 1, 36, 42, 0xAB615F, false),
        ("drink_east", 1, 37, 41, 0x542F3A, false),
        ("drink_east", 1, 38, 43, 0x6D70AF, false),
        ("drink_east", 1, 38, 42, 0xAEB0DF, false),
        ("drink_east", 1, 44, 47, 0x6264A0, false),
        ("drink_east", 1, 40, 45, 0xE3DACA, false),
        ("drink_east", 1, 34, 28, 0xBD8E19, false),
        ("drink_east", 2, 39, 41, 0xFBD3A7, true),
        ("drink_east", 2, 42, 42, 0xEFA67A, true),
        ("drink_east", 2, 40, 40, 0xC37555, true),
        ("drink_east", 2, 44, 37, 0x762E21, true),
        ("drink_east", 2, 37, 43, 0xCD8D8B, false),
        ("drink_east", 2, 37, 42, 0xAB615F, false),
        ("drink_east", 2, 38, 41, 0x542F3A, false),
        ("drink_east", 2, 39, 43, 0x6D70AF, false),
        ("drink_east", 2, 39, 44, 0xAEB0DF, false),
        ("drink_east", 2, 44, 47, 0x6264A0, false),
        ("drink_east", 2, 43, 45, 0xE3DACA, false),
        ("drink_east", 2, 36, 28, 0xBD8E19, false),
    ];
    let temp = tempfile::tempdir().unwrap();
    for preset in set["presets"].as_array().unwrap() {
        let id = preset["id"].as_str().unwrap();
        let recipe = json!({
            "regions": profile["regions"],
            "color_groups": profile["color_groups"],
            "rgba_map": profile["source_colors"].as_array().unwrap().iter()
                .zip(preset["colors"].as_array().unwrap())
                .map(|(from, to)| (from.as_str().unwrap().to_owned(), to.clone()))
                .collect::<serde_json::Map<_, _>>()
        });
        let recipe_path = temp.path().join(format!("{id}.json"));
        fs::write(&recipe_path, serde_json::to_vec(&recipe).unwrap()).unwrap();
        let output = temp.path().join(id);
        let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
            .args(["apply", "--input"])
            .arg(&original)
            .arg("--palette")
            .arg(&recipe_path)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let targets = [8, 9, 10, 4].map(|i| {
            rgba(u32::from_str_radix(&preset["colors"][i].as_str().unwrap()[1..7], 16).unwrap())
        });
        let mut changed = 0;

        for (name, counts) in cases {
            let prefix = "summer";
            let asset = format!(
                "assets/animations/NPCs/Valen/Sprites/Summer/spr_npc_valen_{prefix}_{name}.png"
            );
            let before = image::open(original.join(&asset)).unwrap().to_rgba8();
            let after = image::open(output.join(&asset)).unwrap().to_rgba8();
            assert_eq!(before.dimensions(), (counts.len() as u32 * 80, 80));
            assert_eq!(before.dimensions(), after.dimensions());
            let meta = asset.replace(".png", ".meta.toml");
            assert_eq!(
                fs::read(original.join(&meta)).unwrap(),
                fs::read(output.join(meta)).unwrap()
            );
            let mut actual = vec![0; counts.len()];
            for (x, y, pixel) in before.enumerate_pixels() {
                let expected = match skin.iter().position(|c| rgba(*c) == pixel.0) {
                    Some(i) => {
                        actual[x as usize / 80] += 1;
                        targets[i]
                    }
                    _ => pixel.0,
                };
                assert_eq!(
                    after.get_pixel(x, y).0,
                    expected,
                    "skin/material boundary {id} {name} [{x},{y}]"
                );
            }
            assert_eq!(actual, counts, "per-frame coverage {name}");
            changed += actual.iter().sum::<usize>();
            for &(case, frame, x, y, color, changes) in &landmarks {
                if case == name {
                    let x = frame * 80 + x;
                    assert_eq!(before.get_pixel(x, y).0, rgba(color));
                    assert_eq!(before.get_pixel(x, y) != after.get_pixel(x, y), changes);
                }
            }
        }
        assert_eq!(changed, 1495);

        for r in &profile["regions"].as_array().unwrap()[..138] {
            let asset = r["asset"].as_str().unwrap();
            for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
                assert_eq!(fs::read(output.join(&path)).unwrap(),fs::read(root.join(format!("generated/characters-balor-valen-summer-eiland-spring-finish-trial/characters/valen/variants/{id}/{path}"))).unwrap(),"prior reviewed output {id} {path}");
            }
        }
    }

    // Practical controls: omit small skin components; prove a polluted mouth map changes protected red.
    let asset = "assets/animations/NPCs/Valen/Sprites/Summer/spr_npc_valen_summer_eat_south.png";
    let source = temp.path().join("controls-source");
    fs::create_dir_all(source.join(asset).parent().unwrap()).unwrap();
    for path in [asset.to_owned(), asset.replace(".png", ".meta.toml")] {
        fs::copy(original.join(&path), source.join(path)).unwrap();
    }
    let region = profile["regions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["asset"] == asset)
        .unwrap()
        .clone();
    let map = profile["source_colors"]
        .as_array()
        .unwrap()
        .iter()
        .zip(set["presets"][0]["colors"].as_array().unwrap())
        .map(|(from, to)| (from.as_str().unwrap().to_owned(), to.clone()))
        .collect::<serde_json::Map<_, _>>();
    let apply_control =
        |id: &str, region: Value, map: serde_json::Map<String, Value>, groups: Value| {
            let recipe = temp.path().join(format!("control-{id}.json"));
            fs::write(
                &recipe,
                serde_json::to_vec(&if region.is_null() {
                    json!({"rgba_map":map,"color_groups":groups})
                } else {
                    json!({"regions":[region],"rgba_map":map,"color_groups":groups})
                })
                .unwrap(),
            )
            .unwrap();
            let output = temp.path().join(format!("control-{id}"));
            let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
                .args(["apply", "--input"])
                .arg(&source)
                .arg("--palette")
                .arg(recipe)
                .arg("--output")
                .arg(&output)
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            image::open(output.join(asset)).unwrap().to_rgba8()
        };
    let correct = apply_control(
        "correct",
        region.clone(),
        map.clone(),
        profile["color_groups"].clone(),
    );
    for (id, point, color) in [
        ("missing-forehead", [39, 33], 0xEFA67A),
        ("missing-finger", [197, 40], 0xFBD3A7),
    ] {
        let mut missing = region.clone();
        missing["seeds"]
            .as_array_mut()
            .unwrap()
            .retain(|s| *s != json!(point));
        let omitted = apply_control(id, missing, map.clone(), profile["color_groups"].clone());
        assert_eq!(omitted.get_pixel(point[0], point[1]).0, rgba(color));
        assert_ne!(
            omitted.get_pixel(point[0], point[1]),
            correct.get_pixel(point[0], point[1])
        );
    }
    let mut too_broad = map;
    too_broad.insert("#9E2626".into(), json!("#FFFFFF"));
    let mut broad_groups = profile["color_groups"].clone();
    broad_groups
        .as_array_mut()
        .unwrap()
        .push(json!(["#9E2626"]));
    let spilled = apply_control("mouth-spill", Value::Null, too_broad, broad_groups);
    for point in [[199, 37], [198, 36]] {
        assert_eq!(correct.get_pixel(point[0], point[1]).0, rgba(0x9E2626));
        assert_eq!(spilled.get_pixel(point[0], point[1]).0, rgba(0xFFFFFF));
    }
}
