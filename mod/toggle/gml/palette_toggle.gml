// Locally packaged animations, with vanilla as each character's session default.
function __lns_palette_runtime() {
    if (global[$ "__lns_palette"] == undefined) {
        global.__lns_palette = { registered:false, initialized:false, ready:false, characters:[] };
    }
    return global.__lns_palette;
}

// Return a render asset without changing the animator's frame, timing or packs.
function lns_palette_world_sprite() {
    var current = self.__lns_palette_original_sprite();
    if (!__lns_palette_runtime().ready) return current;
    var character = self.__lns_palette_character;
    for (var i = 0; i < array_length(character.world_pairs); i++) {
        var pair = character.world_pairs[i];
        if (current == pair[0]) return pair[character.selected];
    }
    return current;
}

function lns_palette_world_install(refresh) {
    var state = __lns_palette_runtime();
    if (!state.ready) return;
    for (var c = 0; c < array_length(state.characters); c++) {
        var character = state.characters[c];
        if (array_length(character.world_pairs) == 0) continue;
        for (var i = 0; i < instance_number(character.object); i++) {
            var actor = instance_find(character.object, i);
            var animator = actor.animator;
            if (!is_struct(animator) || animator.current == undefined) continue;
            var attach = animator[$ "__lns_palette_original_sprite"] == undefined;
            if (attach) {
                animator.__lns_palette_original_sprite = animator.sprite;
                animator.__lns_palette_character = character;
                animator.sprite = method(animator, lns_palette_world_sprite);
            }
            // Also refresh while paused; calling animate() would advance time.
            if (attach || refresh) actor.sprite_index = animator.sprite();
        }
    }
}

function lns_palette_apply(menu) {
    var state = __lns_palette_runtime();
    if (!state.ready || !is_struct(menu)) return;
    var node = menu.portrait;
    var current = node.get_sprite();
    for (var c = 0; c < array_length(state.characters); c++) {
        var character = state.characters[c];
        for (var i = 0; i < array_length(character.pairs); i++) {
            var pair = character.pairs[i];
            var included = false;
            for (var j = 0; j < array_length(pair); j++) {
                if (current == pair[j]) included = true;
            }
            if (!included) continue;
            var target = pair[character.selected];
            if (current == target) return;
            // set_sprite resets the index; retain the raw fractional phase.
            var index = node.index;
            node.set_sprite(target).set_index(index);
            return;
        }
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
function lns_palette_toggle(character) {
    if (!__lns_palette_runtime().ready) return;
    character.selected = (character.selected + 1) % array_length(character.names);
    lns_palette_apply(ANCHOR.get_menu(Menu.Textbox));
    lns_palette_world_install(true);
    mmapi_log_info("lns_palette", character.label + " palette: " + character.names[character.selected]);
}
function lns_palette_initialize() {
    var state = __lns_palette_runtime();
    if (state.initialized) {
        // MMAPI reruns installers each begin-step, including after room changes.
        lns_palette_world_install(false);
        return;
    }
    state.initialized = true;
    var definitions = lns_palette_definitions();
    if (array_length(definitions) == 0) {
        mmapi_log_warn("lns_palette", "Palette study disabled: empty character table.");
        return;
    }
    for (var c = 0; c < array_length(definitions); c++) {
        var definition = definitions[c];
        var character = {
            id:definition[0], label:definition[1], hotkey:definition[2], object:undefined,
            names:definition[3], selected:0, pairs:[], world_pairs:[]
        };
        array_push(state.characters, character);
        var names = definition[4];
        if (array_length(character.names) < 2 || array_length(names) == 0) {
            mmapi_log_warn("lns_palette", "Palette study disabled: empty preset table.");
            return;
        }
        for (var i = 0; i < array_length(names); i++) {
            if (array_length(names[i]) != array_length(character.names)) {
                mmapi_log_warn("lns_palette", "Palette study disabled: incomplete preset table.");
                return;
            }
            var sprites = [];
            for (var j = 0; j < array_length(names[i]); j++) {
                var sprite = try_string_to_asset(names[i][j]);
                if (sprite == undefined) {
                    mmapi_log_warn("lns_palette", "Palette study disabled: an animation asset is missing.");
                    return;
                }
                array_push(sprites, sprite);
            }
            if (string_pos("spr_npc_", names[i][0]) == 1) array_push(character.world_pairs, sprites);
            else array_push(character.pairs, sprites);
        }
        if (array_length(character.world_pairs) > 0) {
            var npc_id = try_string_to_npc_id(character.id);
            if (npc_id != undefined) character.object = npc_id_to_gm_obj_id(npc_id);
            if (character.object == undefined) {
                mmapi_log_warn("lns_palette", "Palette study disabled: a world object is missing.");
                return;
            }
        }
    }
    state.ready = true;
    for (var c = 0; c < array_length(state.characters); c++) {
        var character = state.characters[c];
        mmapi_hotkey_register(mmapi_hotkey_vk_from_name(character.hotkey), method(character, function() { lns_palette_toggle(self); }));
    }
    lns_palette_world_install(true);
}

mmapi_mod_declare("lns_palette", "0.1.0");
if (!__lns_palette_runtime().registered) {
    __lns_palette_runtime().registered = true;
    mmapi_on("ui.menu_opened", lns_palette_menu_opened);
    mmapi_register(lns_palette_initialize);
}
