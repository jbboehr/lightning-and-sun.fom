// Locally packaged spring portraits, with vanilla as the session default.
function __lns_palette_runtime() {
    if (global[$ "__lns_palette"] == undefined) {
        global.__lns_palette = {
            registered: false,
            initialized: false,
            ready: false,
            blue: false,
            pairs: [],
        };
    }
    return global.__lns_palette;
}

function lns_palette_apply(menu) {
    var state = __lns_palette_runtime();
    if (!state.ready || !is_struct(menu)) return;
    var node = menu.portrait;
    var current = node.get_sprite();
    for (var i = 0; i < array_length(state.pairs); i++) {
        var pair = state.pairs[i];
        if (current != pair[0] && current != pair[1]) continue;
        var target = state.blue ? pair[1] : pair[0];
        if (current == target) return;
        // set_sprite resets the index; preserve the raw phase because get_index floors it.
        var index = node.index;
        node.set_sprite(target).set_index(index);
        return;
    }
}

function lns_palette_set_speaker(speaker) {
    self.__lns_palette_original_set_speaker(speaker);
    lns_palette_apply(self);
}

function lns_palette_menu_opened(ctx) {
    if (ctx.kind != Menu.Textbox) return;
    var menu = ctx.menu;
    if (menu[$ "__lns_palette_original_set_speaker"] == undefined) {
        menu.__lns_palette_original_set_speaker = menu.set_speaker;
        menu.set_speaker = method(menu, lns_palette_set_speaker);
    }
    lns_palette_apply(menu);
}

function lns_palette_toggle() {
    var state = __lns_palette_runtime();
    if (!state.ready) return;
    state.blue = !state.blue;
    lns_palette_apply(ANCHOR.get_menu(Menu.Textbox));
    mmapi_log_info("lns_palette", state.blue ? "Palette: blue" : "Palette: vanilla");
}

function lns_palette_initialize() {
    var state = __lns_palette_runtime();
    if (state.initialized) return;
    state.initialized = true;
    var names = lns_palette_assets();
    for (var i = 0; i < array_length(names); i++) {
        var vanilla_sprite = try_string_to_asset(names[i][0]);
        var blue_sprite = try_string_to_asset(names[i][1]);
        if (vanilla_sprite == undefined || blue_sprite == undefined) {
            mmapi_log_warn("lns_palette", "Palette study disabled: a portrait asset is missing.");
            return;
        }
        array_push(state.pairs, [vanilla_sprite, blue_sprite]);
    }
    state.ready = true;
    mmapi_hotkey_register(mmapi_hotkey_vk_from_name("F6"), lns_palette_toggle);
}

mmapi_mod_declare("lns_palette", "0.1.0");
if (!__lns_palette_runtime().registered) {
    __lns_palette_runtime().registered = true;
    mmapi_on("ui.menu_opened", lns_palette_menu_opened);
    mmapi_register(lns_palette_initialize);
}
