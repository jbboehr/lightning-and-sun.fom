// Local test driver only: open a real textbox for the selected NPC.
function lns_palette_smoke_open() {
    if (ANCHOR.get_menu(Menu.Textbox) != undefined) return;
    var menu = ANCHOR.spawn_menu(Menu.Textbox);
    menu.requested_speaker = new NpcSpeaker(NpcId.Adeline);
    menu.requested_speaker.set_portrait("neutral");
    menu.give_callback(function() {});
    menu.say("misc_local/placeholder");
    mmapi_log_info("lns_palette", "Smoke textbox opened.");
}
function lns_palette_smoke_install() {
    if (global[$ "__lns_palette_smoke_ready"] == true) return;
    global.__lns_palette_smoke_ready = true;
    mmapi_hotkey_register(mmapi_hotkey_vk_from_name("F7"), lns_palette_smoke_open);
    mmapi_hotkey_register(mmapi_hotkey_vk_from_name("F9"), lns_palette_smoke_phase);
    mmapi_log_info("lns_palette", "Smoke hotkey ready.");
}
mmapi_register(lns_palette_smoke_install);
function lns_palette_smoke_phase() {
    var menu = ANCHOR.get_menu(Menu.Textbox);
    if (menu == undefined) return;
    menu.portrait.set_index(1.25);
    var state = __lns_palette_runtime().characters[0];
    var selected = state.selected;
    for (var i = 0; i < array_length(state.names); i++) {
        lns_palette_toggle(state);
        assert(menu.portrait.index == 1.25, "Palette swap lost fractional phase");
    }
    assert(state.selected == selected, "Palette cycle did not restore selection");
    mmapi_log_info("lns_palette", "Fractional phase preserved through every preset.");
}
function lns_palette_smoke_talk() {
    var menu = ANCHOR.get_menu(Menu.Textbox);
    if (menu != undefined) {
        menu.text.play("This is an animation check. This is an animation check. This is an animation check. This is an animation check. This is an animation check. This is an animation check. This is an animation check.");
    }
}
function lns_palette_smoke_observe() {
    if (global[$ "__lns_palette_smoke_frames"] == undefined) {
        global.__lns_palette_smoke_frames = {};
        mmapi_hotkey_register(mmapi_hotkey_vk_from_name("F10"), lns_palette_smoke_talk);
    }
    var menu = ANCHOR.get_menu(Menu.Textbox);
    if (menu == undefined) return;
    var frame = floor(menu.portrait.get_index());
    var label = __lns_palette_runtime().characters[0].names[__lns_palette_runtime().characters[0].selected] + "_" + string(frame);
    if (global.__lns_palette_smoke_frames[$ label] == undefined) {
        global.__lns_palette_smoke_frames[$ label] = true;
        mmapi_log_info("lns_palette", "Observed preset_frame: " + label);
    }
}
mmapi_register(lns_palette_smoke_observe);
