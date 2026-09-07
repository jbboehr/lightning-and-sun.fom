// Only the engine boundary is simulated. The test runs the shipped mod's GML.
enum Menu { Textbox, Other, InfoToasts }
enum NpcId { Adeline, Hayden = 10 }
globalvar ANCHOR;
global.menu = undefined;
global.missing_asset = false;
global.missing_asset_name = undefined;
global.missing_npc_id = false;
global.hotkey_count = 0;
global.warning_count = 0;
global.hook_count = 0;
global.third_preset = false;
global.max_presets = false;
global.seasons = false;
global.world = false;
global.characters = false;
global.notices = [];
global.palette_log = undefined;
ANCHOR = {
    get_menu: function(kind) {
        if (kind == Menu.InfoToasts) return { create_notification: function(label) { array_push(global.notices, label); } };
        return global.menu;
    },
    wrap_for_local: function(label) { return label; }
};

function mmapi_mod_declare(name, version) {}
function mmapi_on(name, callback) {
    assert(name == "ui.menu_opened");
    global.menu_opened = callback;
    global.hook_count += 1;
}
function mmapi_register(callback) { global.initialize = callback; }
function mmapi_hotkey_vk_from_name(name) { assert(name == "F6" || name == "F8"); return name == "F6" ? 117 : 119; }
function mmapi_hotkey_register(key, callback) {
    assert(key == 117 || key == 119);
    if (key == 117) global.hotkey = callback;
    else global.hotkey_hayden = callback;
    global.hotkey_count += 1;
}
function mmapi_log_info(name, message) { global.palette_log = message; }
function mmapi_log_warn(name, message) { global.warning_count += 1; }
function lns_palette_assets() {
    if (global.world) return [
        ["spr_portrait_adeline_spring_neutral", "spr_lns_adeline_spring_neutral_blue"],
        ["spr_npc_adeline_spring_idle_south", "spr_lns_npc_adeline_spring_idle_south_blue"],
        ["spr_npc_adeline_spring_walk_east", "spr_lns_npc_adeline_spring_walk_east_blue"]
    ];
    if (global.seasons) return [
        ["spr_portrait_adeline_spring_neutral", "spr_lns_adeline_spring_neutral_blue", "spr_lns_adeline_spring_neutral_warm"],
        ["spr_portrait_adeline_summer_neutral", "spr_lns_adeline_summer_neutral_blue", "spr_lns_adeline_summer_neutral_warm"],
        ["spr_portrait_adeline_autumn_neutral", "spr_lns_adeline_autumn_neutral_blue", "spr_lns_adeline_autumn_neutral_warm"],
        ["spr_portrait_adeline_winter_neutral", "spr_lns_adeline_winter_neutral_blue", "spr_lns_adeline_winter_neutral_warm"],
        ["spr_portrait_adeline_beach_neutral", "spr_lns_adeline_beach_neutral_blue", "spr_lns_adeline_beach_neutral_warm"],
        ["spr_portrait_adeline_beach_bath_neutral", "spr_lns_adeline_beach_bath_neutral_blue", "spr_lns_adeline_beach_bath_neutral_warm"],
        ["spr_portrait_adeline_wedding_neutral", "spr_lns_adeline_wedding_neutral_blue", "spr_lns_adeline_wedding_neutral_warm"]
    ];
    if (global.max_presets) return [[
        "spr_portrait_adeline_spring_neutral",
        "spr_lns_adeline_spring_neutral_blue",
        "spr_lns_adeline_spring_neutral_two",
        "spr_lns_adeline_spring_neutral_three",
        "spr_lns_adeline_spring_neutral_four",
        "spr_lns_adeline_spring_neutral_five",
        "spr_lns_adeline_spring_neutral_six",
        "spr_lns_adeline_spring_neutral_seven",
        "spr_lns_adeline_spring_neutral_eight"
    ]];
    if (global.third_preset) return [
        ["spr_portrait_adeline_spring_neutral", "spr_lns_adeline_spring_neutral_blue", "spr_lns_adeline_spring_neutral_warm"],
        ["spr_portrait_adeline_spring_happy", "spr_lns_adeline_spring_happy_blue", "spr_lns_adeline_spring_happy_warm"]
    ];
    return [
        ["spr_portrait_adeline_spring_neutral", "spr_lns_adeline_spring_neutral_blue"],
        ["spr_portrait_adeline_spring_happy", "spr_lns_adeline_spring_happy_blue"]
    ];
}
function lns_palette_names() {
    if (global.max_presets) return [
        "Vanilla", "Debug Blue", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight"
    ];
    return global.third_preset ? ["Vanilla", "Debug Blue", "Warm trial"] : ["Vanilla", "Debug Blue"];
}
function try_string_to_npc_id(name) {
    if (global.missing_npc_id) return undefined;
    if (name == "adeline") return NpcId.Adeline;
    if (name == "hayden") return NpcId.Hayden;
    return undefined;
}
function npc_id_to_gm_obj_id(npc_id) {
    // The game's mapping rejects an undefined ID instead of returning an object.
    assert(npc_id != undefined, "World lookup must guard an unknown NPC ID");
    if (npc_id == NpcId.Adeline) return 1000;
    if (npc_id == NpcId.Hayden) return 1001;
    assert(false, "Unexpected NPC ID");
}
function try_string_to_asset(name) {
    if (global.missing_asset || name == global.missing_asset_name) return undefined;
    if (name == "spr_portrait_hayden_spring_neutral") return 400;
    if (name == "spr_lns_hayden_spring_neutral_blue") return 401;
    if (name == "spr_npc_adeline_spring_idle_south") return 300;
    if (name == "spr_lns_npc_adeline_spring_idle_south_blue") return 301;
    if (name == "spr_npc_adeline_spring_walk_east") return 310;
    if (name == "spr_lns_npc_adeline_spring_walk_east_blue") return 311;
    if (name == "spr_portrait_adeline_spring_neutral") return 10;
    if (name == "spr_lns_adeline_spring_neutral_blue") return 20;
    if (name == "spr_portrait_adeline_spring_happy") return 40;
    if (name == "spr_lns_adeline_spring_happy_blue") return 50;
    if (name == "spr_lns_adeline_spring_neutral_warm") return 60;
    if (name == "spr_lns_adeline_spring_happy_warm") return 70;
    if (name == "spr_portrait_adeline_summer_neutral") return 80;
    if (name == "spr_lns_adeline_summer_neutral_blue") return 90;
    if (name == "spr_lns_adeline_summer_neutral_warm") return 100;
    if (name == "spr_portrait_adeline_autumn_neutral") return 110;
    if (name == "spr_lns_adeline_autumn_neutral_blue") return 120;
    if (name == "spr_lns_adeline_autumn_neutral_warm") return 130;
    if (name == "spr_portrait_adeline_winter_neutral") return 140;
    if (name == "spr_lns_adeline_winter_neutral_blue") return 150;
    if (name == "spr_lns_adeline_winter_neutral_warm") return 160;
    if (name == "spr_portrait_adeline_beach_neutral") return 170;
    if (name == "spr_lns_adeline_beach_neutral_blue") return 180;
    if (name == "spr_lns_adeline_beach_neutral_warm") return 190;
    if (name == "spr_portrait_adeline_beach_bath_neutral") return 200;
    if (name == "spr_lns_adeline_beach_bath_neutral_blue") return 210;
    if (name == "spr_lns_adeline_beach_bath_neutral_warm") return 220;
    if (name == "spr_portrait_adeline_wedding_neutral") return 230;
    if (name == "spr_lns_adeline_wedding_neutral_blue") return 240;
    if (name == "spr_lns_adeline_wedding_neutral_warm") return 250;
    if (name == "spr_lns_adeline_spring_neutral_two") return 61;
    if (name == "spr_lns_adeline_spring_neutral_three") return 62;
    if (name == "spr_lns_adeline_spring_neutral_four") return 63;
    if (name == "spr_lns_adeline_spring_neutral_five") return 64;
    if (name == "spr_lns_adeline_spring_neutral_six") return 65;
    if (name == "spr_lns_adeline_spring_neutral_seven") return 66;
    if (name == "spr_lns_adeline_spring_neutral_eight") return 67;
    return undefined;
}
function TestPortrait() constructor {
    sprite = 10;
    index = 1.25;
    changes = 0;
    function get_sprite() { return self.sprite; }
    // Match Anchor Node: callers see only the rendered integer frame even though
    // the animation advances with a fractional phase in the public index field.
    function get_index() { return floor(self.index); }
    function set_sprite(value) {
        self.sprite = value;
        self.index = 0;
        self.changes += 1;
        return self;
    }
    function set_index(value) { self.index = value; return self; }
}
function TestTextbox() constructor {
    portrait = new TestPortrait();
    original_calls = 0;
    function set_speaker(speaker) {
        self.original_calls += 1;
        self.portrait.set_sprite(speaker.sprite).set_index(speaker.index);
    }
}
function lns_palette_definitions() {
    var definitions = [["adeline", "Adeline", "F6", lns_palette_names(), lns_palette_assets()]];
    if (global.characters) array_push(definitions, ["hayden", "Hayden", "F8", ["Vanilla","Debug Blue"], [["spr_portrait_hayden_spring_neutral","spr_lns_hayden_spring_neutral_blue"]]]);
    return definitions;
}
