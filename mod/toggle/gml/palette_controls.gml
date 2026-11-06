// Called by the installer, where MMAPI file IO is available.
function lns_palette_controls_initialize() {
    var state = __lns_palette_runtime();
    if (state[$ "controls"] != undefined) return;
    state.controls = { remember:false, message:"", error:false };
    try {
        var path = mmapi_config_path("lns_palette");
        if (!file_exists(path)) return;
        var primary = try_read_json_file(path, undefined, false);
        var saved = mmapi_config_read_valid("lns_palette", 1);
        if (!lns_palette_controls_valid(saved)) {
            lns_palette_controls_notice("Saved palette settings were invalid; using Vanilla.", false);
            return;
        }
        state.controls.remember = mmapi_config_bool(saved, "remember", false);
        if (primary == undefined) {
            lns_palette_controls_notice("Recovered the previous saved palette settings.", false);
        }
        if (!state.controls.remember) return;
        for (var i = 0; i < array_length(state.characters); i++) {
            var character = state.characters[i];
            var id = saved.selections[$ character.id];
            var selected = -1;
            for (var j = 0; j < array_length(character.preset_ids); j++) {
                if (id == character.preset_ids[j]) selected = j;
            }
            if (selected == -1) {
                lns_palette_controls_notice("Some saved palettes are unavailable; using Vanilla for those characters.", false);
                mmapi_log_warn("lns_palette", "Unavailable saved palette for " + character.id + ": " + string(id));
                selected = 0;
            }
            lns_palette_select(character, selected);
        }
    } catch (err) {
        state.controls.remember = false;
        for (var i = 0; i < array_length(state.characters); i++) lns_palette_select(state.characters[i], 0);
        lns_palette_controls_notice("Could not read saved palette settings; using Vanilla.", false);
    }
}

function lns_palette_controls_valid(saved) {
    if (!is_struct(saved)) return false;
    var version = saved[$ "__config_version"];
    if ((typeof(version) != "number" && typeof(version) != "int64") || version != 1) return false;
    if (typeof(saved[$ "remember"]) != "bool" || !is_struct(saved[$ "selections"])) return false;
    var characters = __lns_palette_runtime().characters;
    for (var i = 0; i < array_length(characters); i++) {
        var id = saved.selections[$ characters[i].id];
        if (id != undefined && !is_string(id)) return false;
    }
    return true;
}

function lns_palette_controls_notice(message, error) {
    var controls = __lns_palette_runtime().controls;
    controls.message = message;
    controls.error = error;
    mmapi_log_warn("lns_palette", message);
}

function lns_palette_controls_show_notice() {
    var message = __lns_palette_runtime().controls.message;
    if (message != "") ANCHOR.get_menu(Menu.InfoToasts).create_notification(ANCHOR.wrap_for_local("Palettes: " + message));
}

function lns_palette_controls_save() {
    var state = __lns_palette_runtime();
    var selections = {};
    for (var i = 0; i < array_length(state.characters); i++) {
        var character = state.characters[i];
        selections[$ character.id] = character.preset_ids[character.selected];
    }
    var saved = { remember:state.controls.remember, selections:selections };
    var confirmed = false;
    try {
        mmapi_config_write("lns_palette", 1, saved);
        // Recovery loading could hide a failed primary write behind a good backup.
        var path = mmapi_config_path("lns_palette");
        var actual = file_exists(path) ? try_read_json_file(path, undefined, false) : undefined;
        confirmed = lns_palette_controls_valid(actual);
        if (confirmed) {
            confirmed = actual.remember == state.controls.remember;
            for (var i = 0; i < array_length(state.characters); i++) {
                var id = state.characters[i].id;
                if (actual.selections[$ id] != selections[$ id]) confirmed = false;
            }
        }
    } catch (err) {
        confirmed = false;
    }
    if (confirmed) {
        state.controls.message = "";
        state.controls.error = false;
    } else {
        lns_palette_controls_notice(state.controls.remember
            ? "Not saved. Your choices still work this session."
            : "Not saved. Previous saved choices may return next launch.", true);
        lns_palette_controls_show_notice();
    }
    return confirmed;
}

function lns_palette_controls_label(character) {
    return character.label + ": " + character.names[character.selected];
}

function lns_palette_controls_refresh(menu) {
    var controls = __lns_palette_runtime().controls;
    for (var i = 0; i < array_length(menu.__lns_palette_rows); i++) {
        var entry = menu.__lns_palette_rows[i];
        entry.row.text_label.set_key(ANCHOR.wrap_for_local(lns_palette_controls_label(entry.character)));
    }
    menu.__lns_palette_remember.text_label.set_key(ANCHOR.wrap_for_local("Remember choices: " + (controls.remember ? "On" : "Off")));
    var status = controls.message;
    if (status == "") status = controls.remember ? "Choices will be remembered." : "Choices reset to Vanilla each launch.";
    menu.__lns_palette_status.text_label.set_key(ANCHOR.wrap_for_local(status));
}

function lns_palette_controls_cycle(menu, character) {
    if (lns_palette_select(character, (character.selected + 1) % array_length(character.names))) {
        if (__lns_palette_runtime().controls.remember) lns_palette_controls_save();
        lns_palette_controls_refresh(menu);
    }
}

function lns_palette_controls_remember(menu) {
    var controls = __lns_palette_runtime().controls;
    controls.remember = !controls.remember;
    lns_palette_controls_save();
    lns_palette_controls_refresh(menu);
}

function lns_palette_controls_reset(menu) {
    var state = __lns_palette_runtime();
    for (var i = 0; i < array_length(state.characters); i++) lns_palette_select(state.characters[i], 0);
    if (state.controls.remember) lns_palette_controls_save();
    lns_palette_controls_refresh(menu);
}

function lns_palette_controls_open_page() {
    self.__lns_palette_rows = [];
    self.element(ANCHOR.wrap_for_local("Select a character to cycle palettes."), 33, false);
    var characters = __lns_palette_runtime().characters;
    for (var i = 0; i < array_length(characters); i++) {
        var character = characters[i];
        var row = self.element(ANCHOR.wrap_for_local(lns_palette_controls_label(character)));
        row.set_tap_callback(lns_palette_controls_cycle, [self, character]);
        array_push(self.__lns_palette_rows, { row:row, character:character });
    }
    self.__lns_palette_remember = self.element(ANCHOR.wrap_for_local("Remember choices: Off"))
        .set_tap_callback(lns_palette_controls_remember, [self]);
    self.element(ANCHOR.wrap_for_local("Reset all to Vanilla"))
        .set_tap_callback(lns_palette_controls_reset, [self]);
    self.__lns_palette_status = self.element(ANCHOR.wrap_for_local(""), 99)
        .set_tap_callback(lns_palette_controls_show_notice, []);
    lns_palette_controls_refresh(self);
    lns_palette_controls_show_notice();
}

function lns_palette_controls_menu_opened(ctx) {
    if (ctx.kind != Menu.Settings || !__lns_palette_runtime().ready) return;
    var menu = ctx.menu;
    if (!is_struct(menu) || menu[$ "__lns_palette_category"] != undefined) return;
    menu.__lns_palette_category = menu.create_category("lns_palette", method(menu, lns_palette_controls_open_page), spr_ui_journal_settings_icon_gameplay);
    menu.__lns_palette_category.text_label.set_key(ANCHOR.wrap_for_local("Palettes"));
    menu.category_scroller.subscribe_to_pilot(menu.category_pilot);
}
