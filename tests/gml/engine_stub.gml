// Only the engine boundary is simulated. The test runs the shipped mod's GML.
enum Menu { Textbox, Other }
globalvar ANCHOR;
global.menu = undefined;
global.missing_asset = false;
global.hotkey_count = 0;
global.warning_count = 0;
global.hook_count = 0;
ANCHOR = { get_menu: function(kind) { return global.menu; } };

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
function try_string_to_asset(name) {
    if (global.missing_asset) return undefined;
    if (name == "spr_portrait_adeline_spring_neutral") return 10;
    if (name == "spr_lns_adeline_spring_neutral_blue") return 20;
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
