// Only the engine boundary is simulated. The test runs the shipped mod's GML.
enum Menu { Textbox, Other, InfoToasts }
globalvar ANCHOR;
global.menu = undefined;
global.missing_asset = false;
global.missing_asset_name = undefined;
global.hotkey_count = 0;
global.warning_count = 0;
global.hook_count = 0;
global.third_preset = false;
global.max_presets = false;
global.notice = undefined;
ANCHOR = {
    get_menu: function(kind) {
        if (kind == Menu.InfoToasts) return { create_notification: function(label) { global.notice = label; } };
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
function mmapi_hotkey_vk_from_name(name) { assert(name == "F6"); return 117; }
function mmapi_hotkey_register(key, callback) {
    assert(key == 117);
    global.hotkey = callback;
    global.hotkey_count += 1;
}
function mmapi_log_info(name, message) {}
function mmapi_log_warn(name, message) { global.warning_count += 1; }
function lns_palette_assets() {
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
function try_string_to_asset(name) {
    if (global.missing_asset || name == global.missing_asset_name) return undefined;
    if (name == "spr_portrait_adeline_spring_neutral") return 10;
    if (name == "spr_lns_adeline_spring_neutral_blue") return 20;
    if (name == "spr_portrait_adeline_spring_happy") return 40;
    if (name == "spr_lns_adeline_spring_happy_blue") return 50;
    if (name == "spr_lns_adeline_spring_neutral_warm") return 60;
    if (name == "spr_lns_adeline_spring_happy_warm") return 70;
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
