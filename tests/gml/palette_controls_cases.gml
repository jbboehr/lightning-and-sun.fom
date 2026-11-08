function test_boot() {
    global.__lns_palette = undefined;
    global.menu = new TestTextbox();
    global.actors = [new TestActor()];
    global.warning_count = 0;
    global.warnings = [];
    global.notices = [];
    global.initialize();
    assert(__lns_palette_runtime().ready, "Palette runtime should initialize");
    assert(array_length(__lns_palette_runtime().characters) == 36);
    assert(global.hotkey_count == 0, "Release must not register character hotkeys");
}
function test_settings() {
    var menu = new SettingsProbe();
    global.settings_menu = menu;
    for (var i = 0; i < 7; i++) menu.create_category("native_"+string(i),method(menu,function(){}),1);
    global.menu_opened({kind:Menu.Settings,menu:menu});
    return menu;
}
function test_all_vanilla() {
    var state = __lns_palette_runtime();
    for (var i = 0; i < array_length(state.characters); i++) assert(state.characters[i].selected == 0);
}
function test_saved(id) {
    var selections = {};
    var definitions = lns_palette_definitions();
    for (var i = 0; i < array_length(definitions); i++) selections[$ definitions[i][0]] = id;
    return {__config_version:1,remember:true,selections:selections};
}
function test_put(saved) {
    global.disk[$ mmapi_config_path("lns_palette")] = json_stringify(saved);
}
function test_visible_error(menu) {
    assert(__lns_palette_runtime().controls.error);
    assert(string_pos("Not saved.",menu.__lns_palette_status.text_label.local_key) > 0);
    assert(global.warning_count > 0);
    assert(array_length(global.notices)>0, "Offscreen status is insufficient: failure must immediately notify");
    assert(string_pos("RAW:Palettes: Not saved.",global.notices[array_length(global.notices)-1]) == 1);
}

if (global.controls_case == "title") {
    assert(!__lns_palette_runtime().initialized);
    var title = test_settings();
    assert(__lns_palette_runtime().ready, "Title Settings must initialize without Game begin-step");
    assert(array_length(title.category_pilot.nodes) == 8);
    title.__lns_palette_category.tap();
    assert(array_length(title.option_pilot.nodes) == 39);
    assert(!__lns_palette_runtime().controls.remember && global.config_write_count == 0);
    title.option_pilot.nodes[0].tap();
    title.option_pilot.nodes[36].tap();
    assert(__lns_palette_runtime().controls.remember && global.config_write_count == 1);
    global.__lns_palette = undefined;
    var restarted = test_settings();
    restarted.__lns_palette_category.tap();
    assert(__lns_palette_runtime().controls.remember);
    assert(__lns_palette_runtime().characters[0].selected == 1);
    assert(global.config_write_count == 1, "Title restore must not rewrite settings");
    var state = __lns_palette_runtime();
    global.initialize();
    assert(__lns_palette_runtime() == state && state.characters[0].selected == 1);
    assert(global.config_write_count == 1);
    show_debug_message("PASS: fresh title Settings initializes, saves, restores before Game and tolerates later installer drain");
} else if (global.controls_case == "title_notice") {
    test_put(test_saved("candidate"));
    global.toast_menu = undefined;
    var menu = test_settings();
    menu.__lns_palette_category.tap();
    assert(global.toast_menu == undefined && global.toast_spawn_count == 0,
        "Opening title Settings must not create an unused notification menu");
    var scroller = menu.option_scroller;
    var pilot = menu.option_pilot;
    scroller.scroll = 0;
    pilot.focus = 0;
    global.write_mode = "noop";
    menu.option_pilot.nodes[0].tap();
    test_visible_error(menu);
    assert(global.toast_spawn_count == 1, "Missing title toast menu must be created natively");
    var toasts = global.toast_menu;
    assert(!toasts.close_requested && array_length(global.notices) == 1);
    menu.option_pilot.nodes[1].tap();
    menu.option_pilot.nodes[38].tap();
    assert(global.toast_menu == toasts && global.toast_spawn_count == 1);
    assert(array_length(global.notices) == 3, "Subsequent failures and rereading must reuse the toast menu");
    assert(menu.option_scroller == scroller && menu.option_pilot == pilot);
    assert(scroller.scroll == 0 && pilot.focus == 0 && ANCHOR.active == pilot);
    assert(__lns_palette_runtime().characters[0].selected == 2,
        "A notification must not undo the session choice");
    menu.close();
    assert(menu.original_close_calls == 1 && toasts.close_requested,
        "Title-created toast menu must close with Settings before Game creates its own");
    // After returning from Settings, Game creates its own notification menu.
    global.toast_menu = new InfoToastsProbe();
    var gameplay_toasts = global.toast_menu;
    test_boot();
    menu = test_settings();
    menu.__lns_palette_category.tap();
    menu.option_pilot.nodes[0].tap();
    test_visible_error(menu);
    assert(global.toast_menu == gameplay_toasts && global.toast_spawn_count == 1);
    menu.close();
    assert(menu.original_close_calls == 1 && !gameplay_toasts.close_requested,
        "Settings must not close the game's existing notification menu");
    show_debug_message("PASS: title save failure creates/reuses native toast, preserves focus/scroll, closes owned menu and leaves gameplay toast alone");
} else if (global.controls_case == "menu") {
    __lns_palette_runtime().initialized = true;
    var early = test_settings();
    assert(array_length(early.category_pilot.nodes) == 7, "Unready runtime must not expose palette actions");
    test_boot();
    global.menu_opened({kind:Menu.Settings,menu:early});
    assert(array_length(early.category_pilot.nodes) == 8);
    var state = __lns_palette_runtime();
    assert(!state.controls.remember && global.config_write_count == 0);
    test_all_vanilla();
    global.initialize();
    assert(global.config_write_count == 0, "Begin-step reinstall must not write");
    global.menu_opened({kind:Menu.Textbox,menu:global.menu});
    assert(global.menu[$ "__lns_palette_category"] == undefined);
    var menu = test_settings();
    global.menu_opened({kind:Menu.Settings,menu:menu});
    assert(array_length(menu.category_pilot.nodes) == 8, "Duplicate hook appended another category");
    assert(menu.category_scroller.subscribed == menu.category_pilot);
    assert(menu.__lns_palette_category.text_label.local_key == "RAW:Palettes");
    menu.__lns_palette_category.tap();
    assert(menu.active_page == "lns_palette" && menu.__lns_palette_category.is_selected());
    assert(ANCHOR.active == menu.option_pilot);
    assert(array_length(menu.option_pilot.nodes) == 39, "36 characters, remember, reset, rereadable status");
    assert(array_length(menu.option_scroller.nodes) == 40, "Only help is noninteractive");
    assert(menu.option_scroller.subscribed == menu.option_pilot);
    assert(menu.option_pilot.nodes[38] == menu.__lns_palette_status);
    var status_writes = global.config_write_count;
    menu.option_pilot.nodes[38].tap();
    assert(global.config_write_count == status_writes && array_length(global.notices) == 0);
    for (var i = 0; i < 36; i++) {
        assert(menu.option_pilot.nodes[i] == menu.__lns_palette_rows[i].row);
        assert(menu.__lns_palette_rows[i].character == state.characters[i]);
        assert(menu.option_pilot.nodes[i].text_label.local_key == "RAW:"+state.characters[i].label+": Vanilla");
    }
    var scroller = menu.option_scroller;
    var pilot = menu.option_pilot;
    scroller.scroll = 127;
    pilot.focus = 18;
    menu.option_pilot.nodes[0].tap();
    assert(state.characters[0].selected == 1 && state.characters[1].selected == 0);
    assert(menu.option_pilot.nodes[0].text_label.local_key == "RAW:"+state.characters[0].label+": Candidate");
    assert(global.menu.portrait.sprite == 20 && global.menu.portrait.index == 1.75);
    assert(global.actors[0].sprite_index == 301);
    assert(global.actors[0].animator.current.frame == 7.75 && global.actors[0].animator.current.index == 2);
    assert(global.actors[0].image_xscale == -1 && global.actors[0].x == 34 && global.actors[0].y == 72);
    for (var i = 1; i < 36; i++) menu.option_pilot.nodes[i].tap();
    for (var i = 0; i < 36; i++) assert(state.characters[i].selected == 1);
    assert(menu.option_scroller == scroller && menu.option_pilot == pilot);
    assert(scroller.scroll == 127 && pilot.focus == 18 && ANCHOR.active == pilot);
    menu.option_pilot.nodes[0].tap();
    assert(state.characters[0].selected == 2 && state.characters[1].selected == 1);
    menu.option_pilot.nodes[0].tap();
    assert(state.characters[0].selected == 0 && global.menu.portrait.sprite == 10 && global.actors[0].sprite_index == 300);
    menu.option_pilot.nodes[37].tap();
    test_all_vanilla();
    assert(global.config_write_count == 0, "Default-off selection and reset must not write");
    menu.option_pilot.nodes[36].tap();
    assert(state.controls.remember && global.config_write_count == 1 && !state.controls.error);
    menu.option_pilot.nodes[0].tap();
    var saves = global.config_write_count;
    menu.option_pilot.nodes[37].tap();
    assert(global.config_write_count == saves+1, "Reset must save once, not once per character");
    test_all_vanilla();
    menu.option_pilot.nodes[0].tap();
    var frees = global.free_count;
    menu.__lns_palette_category.tap();
    assert(global.free_count == frees+1 && menu.option_pilot.reset_count == 2);
    assert(array_length(menu.option_pilot.nodes) == 39);
    assert(menu.option_pilot.nodes[0].text_label.local_key == "RAW:"+state.characters[0].label+": Candidate");
    var reopened = test_settings();
    reopened.__lns_palette_category.tap();
    assert(array_length(reopened.category_pilot.nodes) == 8);
    assert(reopened.option_pilot.nodes[0].text_label.local_key == menu.option_pilot.nodes[0].text_label.local_key);
    assert(reopened.__lns_palette_remember.text_label.local_key == "RAW:Remember choices: On");
    assert(global.native_settings_writes == 0);
    show_debug_message("PASS: native palette page, 36 rows, callback binding, cycling/reset, wrapped labels, focus identity, reopen, synchronized portrait/world phase");
} else if (global.controls_case == "max_presets") {
    test_boot();
    test_all_vanilla();
    var state = __lns_palette_runtime();
    for (var i = 0; i < 36; i++) {
        assert(array_length(state.characters[i].names) == 14);
        assert(array_length(state.characters[i].preset_ids) == 14);
        assert(state.characters[i].preset_ids[10] == "player_35");
        assert(state.characters[i].preset_ids[11] == "player_27");
        assert(state.characters[i].preset_ids[12] == "npc_stillwell");
        assert(state.characters[i].preset_ids[13] == "npc_terithia");
    }
    var menu = test_settings();
    menu.__lns_palette_category.tap();
    assert(array_length(menu.option_pilot.nodes) == 39);
    var scroller = menu.option_scroller;
    var pilot = menu.option_pilot;
    scroller.scroll = 127;
    pilot.focus = 18;
    for (var preset = 1; preset <= 13; preset++) {
        menu.option_pilot.nodes[0].tap();
        assert(state.characters[0].selected == preset && state.characters[1].selected == 0);
        assert(menu.option_pilot.nodes[0].text_label.local_key == "RAW:"+state.characters[0].label+": "+state.characters[0].names[preset]);
        assert(global.menu.portrait.sprite == 10 * (preset + 1) && global.menu.portrait.index == 1.75);
        assert(global.actors[0].sprite_index == 300 + preset);
        assert(global.actors[0].animator.current.frame == 7.75 && global.actors[0].animator.current.index == 2);
        assert(global.actors[0].image_xscale == -1 && global.actors[0].x == 34 && global.actors[0].y == 72);
        assert(menu.option_scroller == scroller && menu.option_pilot == pilot);
        assert(scroller.scroll == 127 && pilot.focus == 18 && ANCHOR.active == pilot);
        if (preset == 11) {
            assert(state.characters[0].preset_ids[preset] == "player_27");
            assert(menu.option_pilot.nodes[0].text_label.local_key == "RAW:"+state.characters[0].label+": Deep umber (Player 27)");
        }
        if (preset == 12) assert(menu.option_pilot.nodes[0].text_label.local_key == "RAW:"+state.characters[0].label+": Muted light (Stillwell)");
    }
    assert(state.characters[0].preset_ids[state.characters[0].selected] == "npc_terithia");
    assert(menu.option_pilot.nodes[0].text_label.local_key == "RAW:"+state.characters[0].label+": Muted medium (Terithia)");
    menu.option_pilot.nodes[0].tap();
    assert(state.characters[0].selected == 0 && state.characters[1].selected == 0);
    assert(global.menu.portrait.sprite == 10 && global.menu.portrait.index == 1.75);
    assert(global.actors[0].sprite_index == 300 && global.actors[0].animator.current.frame == 7.75);
    menu.option_pilot.nodes[1].tap();
    assert(state.characters[0].selected == 0 && state.characters[1].selected == 1);
    assert(global.config_write_count == 0 && !state.controls.remember);

    for (var preset = 13; preset >= 0; preset--) {
        assert(lns_palette_select(state.characters[0], preset));
        lns_palette_controls_refresh(menu);
        assert(state.characters[0].selected == preset && state.characters[1].selected == 1);
        assert(global.menu.portrait.sprite == 10 * (preset + 1) && global.menu.portrait.index == 1.75);
        assert(global.actors[0].sprite_index == 300 + preset && global.actors[0].animator.current.frame == 7.75);
        assert(global.actors[0].animator.current.index == 2 && global.actors[0].image_xscale == -1);
        assert(menu.option_scroller == scroller && menu.option_pilot == pilot);
        assert(scroller.scroll == 127 && pilot.focus == 18 && ANCHOR.active == pilot);
    }
    assert(global.config_write_count == 0);

    var definitions = lns_palette_definitions();
    for (var preset = 0; preset <= 13; preset++) {
        test_put(test_saved(definitions[0][5][preset]));
        var original_bytes = global.disk[$ mmapi_config_path("lns_palette")];
        test_boot();
        state = __lns_palette_runtime();
        for (var i = 0; i < 36; i++) assert(state.characters[i].selected == preset);
        assert(state.controls.remember && global.config_write_count == 0);
        assert(global.disk[$ mmapi_config_path("lns_palette")] == original_bytes);
    }

    test_put(test_saved("player_35"));
    test_boot();
    state = __lns_palette_runtime();
    assert(state.controls.remember && global.config_write_count == 0);
    for (var i = 0; i < 36; i++) assert(state.characters[i].selected == 10);
    assert(global.menu.portrait.sprite == 110 && global.actors[0].sprite_index == 310);
    menu = test_settings();
    menu.__lns_palette_category.tap();
    menu.option_pilot.nodes[0].tap();
    assert(state.characters[0].selected == 11 && state.characters[1].selected == 10);
    assert(global.config_write_count == 1 && !state.controls.error);
    var saved = try_read_json_file(mmapi_config_path("lns_palette"), undefined, false);
    assert(saved.__config_version == 1 && saved.remember);
    assert(saved.selections[$ state.characters[0].id] == "player_27");
    assert(saved.selections[$ state.characters[1].id] == "player_35");
    test_boot();
    state = __lns_palette_runtime();
    assert(state.controls.remember && state.characters[0].selected == 11);
    for (var i = 1; i < 36; i++) assert(state.characters[i].selected == 10);
    assert(global.config_write_count == 1 && global.warning_count == 0);
    assert(global.menu.portrait.sprite == 120 && global.menu.portrait.index == 1.75);
    assert(global.actors[0].sprite_index == 311 && global.actors[0].animator.current.frame == 7.75);
    menu = test_settings();
    menu.__lns_palette_category.tap();
    menu.option_pilot.nodes[0].tap();
    assert(state.characters[0].selected == 12 && state.characters[1].selected == 10);
    saved = try_read_json_file(mmapi_config_path("lns_palette"), undefined, false);
    assert(saved.__config_version == 1 && saved.remember);
    assert(saved.selections[$ state.characters[0].id] == "npc_stillwell");
    assert(saved.selections[$ state.characters[1].id] == "player_35");
    test_boot();
    state = __lns_palette_runtime();
    assert(state.controls.remember && state.characters[0].selected == 12);
    for (var i = 1; i < 36; i++) assert(state.characters[i].selected == 10);
    assert(global.config_write_count == 2 && global.warning_count == 0);
    assert(global.menu.portrait.sprite == 130 && global.menu.portrait.index == 1.75);
    assert(global.actors[0].sprite_index == 312 && global.actors[0].animator.current.frame == 7.75);
    menu = test_settings();
    menu.__lns_palette_category.tap();
    menu.option_pilot.nodes[0].tap();
    assert(state.characters[0].selected == 13 && state.characters[1].selected == 10);
    saved = try_read_json_file(mmapi_config_path("lns_palette"), undefined, false);
    assert(saved.__config_version == 1 && saved.remember);
    assert(saved.selections[$ state.characters[0].id] == "npc_terithia");
    assert(saved.selections[$ state.characters[1].id] == "player_35");
    test_boot();
    state = __lns_palette_runtime();
    assert(state.controls.remember && state.characters[0].selected == 13);
    for (var i = 1; i < 36; i++) assert(state.characters[i].selected == 10);
    assert(global.config_write_count == 3 && global.warning_count == 0);
    assert(global.menu.portrait.sprite == 140 && global.menu.portrait.index == 1.75);
    assert(global.actors[0].sprite_index == 313 && global.actors[0].animator.current.frame == 7.75);
    menu = test_settings();
    menu.__lns_palette_category.tap();
    menu.option_pilot.nodes[0].tap();
    assert(state.characters[0].selected == 0 && state.characters[1].selected == 10);
    test_boot();
    assert(__lns_palette_runtime().characters[0].selected == 0 && __lns_palette_runtime().characters[1].selected == 10);
    assert(global.config_write_count == 4 && global.native_settings_writes == 0);

    test_put(test_saved("npc_terithia"));
    test_boot();
    menu = test_settings();
    menu.__lns_palette_category.tap();
    var writes = global.config_write_count;
    menu.option_pilot.nodes[37].tap();
    test_all_vanilla();
    assert(global.config_write_count == writes + 1);
    saved = try_read_json_file(mmapi_config_path("lns_palette"), undefined, false);
    assert(saved.__config_version == 1 && saved.remember);
    for (var i = 0; i < 36; i++) assert(saved.selections[$ definitions[i][0]] == "vanilla");
    test_boot();
    test_all_vanilla();

    test_put(test_saved("npc_stillwell"));
    test_boot();
    state = __lns_palette_runtime();
    menu = test_settings();
    menu.__lns_palette_category.tap();
    menu.option_pilot.nodes[1].tap();
    menu.option_pilot.nodes[36].tap();
    assert(!state.controls.remember && state.characters[0].selected == 12 && state.characters[1].selected == 13);
    original_bytes = global.disk[$ mmapi_config_path("lns_palette")];
    writes = global.config_write_count;
    test_boot();
    test_all_vanilla();
    assert(!__lns_palette_runtime().controls.remember && global.config_write_count == writes);
    menu = test_settings();
    menu.__lns_palette_category.tap();
    for (var preset = 1; preset <= 13; preset++) menu.option_pilot.nodes[0].tap();
    assert(__lns_palette_runtime().characters[0].selected == 13);
    assert(global.config_write_count == writes && global.disk[$ mmapi_config_path("lns_palette")] == original_bytes);
    test_boot();
    test_all_vanilla();

    for (var preset = 12; preset <= 13; preset++) {
        test_put(test_saved(definitions[0][5][preset - 1]));
        test_boot();
        state = __lns_palette_runtime();
        menu = test_settings();
        menu.__lns_palette_category.tap();
        scroller = menu.option_scroller;
        pilot = menu.option_pilot;
        scroller.scroll = 127;
        pilot.focus = 18;
        original_bytes = global.disk[$ mmapi_config_path("lns_palette")];
        global.write_mode = "noop";
        menu.option_pilot.nodes[0].tap();
        assert(state.characters[0].selected == preset && state.characters[1].selected == preset - 1);
        test_visible_error(menu);
        assert(global.disk[$ mmapi_config_path("lns_palette")] == original_bytes);
        assert(global.menu.portrait.sprite == 10 * (preset + 1) && global.menu.portrait.index == 1.75);
        assert(global.actors[0].sprite_index == 300 + preset && global.actors[0].animator.current.frame == 7.75);
        assert(menu.option_scroller == scroller && menu.option_pilot == pilot);
        assert(scroller.scroll == 127 && pilot.focus == 18 && ANCHOR.active == pilot);
        global.write_mode = "ok";
        test_boot();
        assert(__lns_palette_runtime().characters[0].selected == preset - 1);
    }
    assert(global.native_settings_writes == 0);
    show_debug_message("PASS: fourteen choices, stable old IDs and new donor IDs, independent rows, phase/focus, forward wrap/reverse selections, version-1 restart/reset/opt-out and donor write failure");
} else {
    var path = mmapi_config_path("lns_palette");
    test_boot();
    assert(!__lns_palette_runtime().controls.remember && global.config_write_count == 0);
    global.disk[$ path+".bak"] = json_stringify(test_saved("other"));
    test_boot();
    test_all_vanilla();
    assert(!__lns_palette_runtime().controls.remember && global.config_write_count == 0, "Missing primary must ignore stale backup");
    test_put(test_saved("other"));
    test_boot();
    assert(__lns_palette_runtime().controls.remember);
    for (var i = 0; i < 36; i++) assert(__lns_palette_runtime().characters[i].selected == 2);
    assert(global.menu.portrait.sprite == 30 && global.actors[0].sprite_index == 302);
    global.definitions_reordered = true;
    test_boot();
    for (var i = 0; i < 36; i++) assert(__lns_palette_runtime().characters[i].selected == 1, "Saved stable ID must survive preset reorder");
    assert(global.menu.portrait.sprite == 30 && global.actors[0].sprite_index == 302);
    global.definitions_reordered = false;
    var saved = test_saved("candidate");
    saved.selections.unknown_character = {invalid:"ignored"};
    test_put(saved);
    test_boot();
    assert(__lns_palette_runtime().controls.remember && global.warning_count == 0);
    saved.selections[$ lns_palette_definitions()[0][0]] = "removed_preset";
    test_put(saved);
    test_boot();
    assert(__lns_palette_runtime().characters[0].selected == 0 && __lns_palette_runtime().characters[1].selected == 1);
    assert(global.warning_count > 0 && string_pos("unavailable",__lns_palette_runtime().controls.message) > 0);
    saved = test_saved("candidate");
    saved.selections = {};
    var definitions = lns_palette_definitions();
    for(var i=1;i<array_length(definitions);i++) saved.selections[$ definitions[i][0]] = "candidate";
    test_put(saved);
    test_boot();
    assert(__lns_palette_runtime().characters[0].selected == 0 && global.warning_count > 0);
    var invalid = ["[]","123","false","\"text\"","{}",
        "{\"__config_version\":2,\"remember\":true,\"selections\":{}}",
        "{\"__config_version\":\"1\",\"remember\":true,\"selections\":{}}",
        "{\"__config_version\":1,\"remember\":1,\"selections\":{}}",
        "{\"__config_version\":1,\"remember\":\"true\",\"selections\":{}}",
        "{\"__config_version\":1,\"remember\":true,\"selections\":[]}",
        "{\"__config_version\":1,\"remember\":true}",
        "{\"__config_version\":1,\"selections\":{}}"];
    for (var i = 0; i < array_length(invalid); i++) {
        global.disk[$ path] = invalid[i];
        var writes = global.config_write_count;
        test_boot();
        test_all_vanilla();
        assert(!__lns_palette_runtime().controls.remember && global.warning_count > 0);
        assert(global.config_write_count == writes, "Invalid startup must not overwrite config");
    }
    // A parseable incompatible primary must not silently restore a stale backup.
    global.disk[$ path+".bak"] = json_stringify(test_saved("other"));
    for (var i = 0; i < array_length(invalid); i++) {
        global.disk[$ path] = invalid[i];
        test_boot();
        test_all_vanilla();
        assert(!__lns_palette_runtime().controls.remember && global.warning_count > 0,
            "Unsupported primary schema must keep remembering off even with a valid backup");
    }
    global.disk[$ path+".bak"] = "{";
    var load_menu = test_settings();
    var notices = array_length(global.notices);
    load_menu.__lns_palette_category.tap();
    assert(array_length(global.notices)==notices+1, "Load warning must notify as soon as the palette page opens");
    assert(string_pos("RAW:Palettes: Saved palette settings were invalid",global.notices[array_length(global.notices)-1])==1);
    var status_writes = global.config_write_count;
    load_menu.option_pilot.nodes[38].tap();
    assert(array_length(global.notices)==notices+2, "Status must be focusable and replay its diagnostic");
    assert(global.config_write_count == status_writes, "Rereading status must never save");
    saved = test_saved("candidate");
    saved.selections[$ lns_palette_definitions()[0][0]] = false;
    test_put(saved);
    test_boot();
    test_all_vanilla();
    assert(!__lns_palette_runtime().controls.remember && global.warning_count > 0);
    global.disk[$ path] = "{";
    global.disk[$ path+".bak"] = json_stringify(test_saved("other"));
    test_boot();
    assert(__lns_palette_runtime().controls.remember && __lns_palette_runtime().characters[0].selected == 2);
    assert(string_pos("Recovered",__lns_palette_runtime().controls.message) > 0);
    var recovered_menu = test_settings();
    recovered_menu.__lns_palette_category.tap();
    assert(string_pos("Recovered",global.notices[array_length(global.notices)-1])>0);
    global.disk[$ path+".bak"] = "{";
    test_boot();
    test_all_vanilla();
    assert(!__lns_palette_runtime().controls.remember && global.warning_count > 0);
    test_put(test_saved("other"));
    global.read_mode = "throw";
    test_boot();
    test_all_vanilla();
    assert(!__lns_palette_runtime().controls.remember && global.warning_count > 0);
    global.read_mode = "ok";
    test_boot();
    var menu = test_settings();
    menu.__lns_palette_category.tap();
    menu.option_pilot.nodes[36].tap();
    assert(!__lns_palette_runtime().controls.remember && !__lns_palette_runtime().controls.error);
    assert(__lns_palette_runtime().characters[0].selected == 2, "Opt-out should keep current session choices");
    test_boot();
    test_all_vanilla();
    assert(!__lns_palette_runtime().controls.remember);
    var modes = ["throw","noop","truncate","wrong"];
    for (var i = 0; i < array_length(modes); i++) {
        global.write_mode = "ok";
        test_put(test_saved("candidate"));
        test_boot();
        menu = test_settings();
        menu.__lns_palette_category.tap();
        global.write_mode = modes[i];
        menu.option_scroller.scroll = 0;
        menu.option_pilot.focus = 0;
        var scroller = menu.option_scroller;
        var pilot = menu.option_pilot;
        var notices = array_length(global.notices);
        menu.option_pilot.nodes[0].tap();
        assert(array_length(global.notices)==notices+1, "Failed top-row write must notify without scrolling to status");
        assert(menu.option_scroller==scroller && menu.option_pilot==pilot && scroller.scroll==0 && pilot.focus==0);
        var status_writes = global.config_write_count;
        menu.option_pilot.nodes[38].tap();
        assert(array_length(global.notices)==notices+2 && global.config_write_count==status_writes);
        assert(__lns_palette_runtime().characters[0].selected == 2, "Write failure lost session choice");
        assert(global.menu.portrait.sprite == 30 && global.actors[0].sprite_index == 302);
        test_visible_error(menu);
        menu.option_pilot.nodes[1].tap();
        assert(__lns_palette_runtime().characters[1].selected == 2, "Write failure blocked further choices");
        test_visible_error(menu);
    }
    global.write_mode = "ok";
    global.disk[$ path] = "{";
    global.disk[$ path+".bak"] = json_stringify(test_saved("other"));
    test_boot();
    lns_palette_select(__lns_palette_runtime().characters[0],1);
    menu = test_settings();
    menu.__lns_palette_category.tap();
    global.write_mode = "noop";
    menu.option_pilot.nodes[0].tap();
    test_visible_error(menu);
    assert(mmapi_config_read_valid("lns_palette",1).selections[$ lns_palette_definitions()[0][0]] == "other", "A matching recovery backup must not establish successful primary write");
    global.write_mode = "ok";
    test_put(test_saved("candidate"));
    test_boot();
    menu = test_settings();
    menu.__lns_palette_category.tap();
    global.read_mode = "throw";
    menu.option_pilot.nodes[0].tap();
    test_visible_error(menu);
    global.read_mode = "ok";
    test_boot();
    assert(__lns_palette_runtime().characters[0].selected == 2, "Completed but unconfirmed write may still survive restart");
    test_put(test_saved("candidate"));
    test_boot();
    menu = test_settings();
    menu.__lns_palette_category.tap();
    global.write_mode = "noop";
    menu.option_pilot.nodes[36].tap();
    assert(!__lns_palette_runtime().controls.remember);
    test_visible_error(menu);
    assert(string_pos("Previous saved choices may return",menu.__lns_palette_status.text_label.local_key) > 0);
    assert(string_pos("Previous saved choices may return",global.notices[array_length(global.notices)-1]) > 0);
    test_boot();
    assert(__lns_palette_runtime().controls.remember, "Failed opt-out must honestly warn that old settings can return");
    global.write_mode = "ok";
    menu = test_settings();
    menu.__lns_palette_category.tap();
    var writes = global.config_write_count;
    menu.option_pilot.nodes[37].tap();
    test_all_vanilla();
    assert(global.config_write_count == writes+1 && !__lns_palette_runtime().controls.error);
    test_boot();
    test_all_vanilla();
    assert(__lns_palette_runtime().controls.remember);
    assert(global.native_settings_writes == 0);
    show_debug_message("PASS: stable IDs, opt-in/opt-out/reset, schema recovery, backup recovery, primary-only confirmation, throwing/no-op/truncated/wrong writes and visible failure");
}
