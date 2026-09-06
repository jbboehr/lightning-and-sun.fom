// One spring neutral portrait, with vanilla as the session default.
function __lns_palette_runtime() {
    if (global[$ "__lns_palette"] == undefined) {
        global.__lns_palette = {
            registered: false,
            initialized: false,
            ready: false,
            blue: false,
            vanilla_sprite: undefined,
            blue_sprite: undefined,
        };
    }
    return global.__lns_palette;
}

function lns_palette_apply(menu) {
    var state = __lns_palette_runtime();
    if (!state.ready || !is_struct(menu)) return;
    var node = menu.portrait;
    var current = node.get_sprite();
    if (current != state.vanilla_sprite && current != state.blue_sprite) return;
    var target = state.blue ? state.blue_sprite : state.vanilla_sprite;
    if (current == target) return;
    // set_sprite resets the index; preserve the raw phase because get_index floors it.
    var index = node.index;
    node.set_sprite(target).set_index(index);
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
    state.vanilla_sprite = try_string_to_asset("spr_portrait_adeline_spring_neutral");
    state.blue_sprite = try_string_to_asset("spr_lns_adeline_spring_neutral_blue");
    if (state.vanilla_sprite == undefined || state.blue_sprite == undefined) {
        mmapi_log_warn("lns_palette", "Palette study disabled: a portrait asset is missing.");
        return;
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
