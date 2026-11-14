// Mirrors the game's Map: lookups are static methods on a shared constructor.
function __Map(_inner) constructor {
    inner = _inner;
    static get = function(key) { return self.inner[$ key]; }
    static keys = function() { return struct_get_names(self.inner); }
}
// No world actors are spawned; only the portrait maps matter here.
function instance_number(object) { return 0; }
function test_prototype(portraits) {
    return { wardrobe_outfits: new __Map({ spring: { portraits: new __Map(portraits) } }) };
}

global.characters = true;
global.third_preset = true;
global.cameos = true;
global.initialize();
assert(__lns_palette_runtime().ready && !__lns_palette_runtime().portraits_installed,
    "Without prototypes the install waits for a later step");

NPC_PROTOTYPES = array_create(NpcId.Hayden + 1, undefined);
NPC_PROTOTYPES[NpcId.Adeline] = test_prototype({ neutral: 10, happy: 40, sly: 999 });
NPC_PROTOTYPES[NpcId.Hayden] = test_prototype({ neutral: 400 });
CAMEO_PROTOTYPES = array_create(5, undefined);
var cameos = [CameoId.Linnet, CameoId.Wiscar, CameoId.Darren, CameoId.Wynne];
for (var i = 0; i < array_length(cameos); i++) CAMEO_PROTOTYPES[cameos[i]] = test_prototype({ neutral: 700 + i });
var unrelated = new __Map({ neutral: 10 });
global.initialize();
global.initialize();

var adeline = NPC_PROTOTYPES[NpcId.Adeline].wardrobe_outfits.get("spring").portraits;
var hayden = NPC_PROTOTYPES[NpcId.Hayden].wardrobe_outfits.get("spring").portraits;
var wiscar = CAMEO_PROTOTYPES[CameoId.Wiscar].wardrobe_outfits.get("spring").portraits;
assert(adeline.get("neutral") == 10 && adeline.get("happy") == 40 && hayden.get("neutral") == 400,
    "Vanilla must show the original portraits");
assert(wiscar[$ "__lns_palette_character"] != undefined && wiscar.get("neutral") == 701,
    "Cameo portrait maps are wrapped and keep sprites without a recipe");

global.hotkey();
assert(adeline.get("neutral") == 20 && adeline.get("happy") == 50, "Every screen must read the selected variant");
assert(adeline.get("sly") == 999 && adeline.get("missing") == undefined);
assert(hayden.get("neutral") == 400, "Adeline's choice changed Hayden");
assert(unrelated.get("neutral") == 10, "Unrelated maps keep the original lookup");
assert(adeline.inner.neutral == 10 && adeline.inner.happy == 40, "Stored portrait sprites must stay unchanged");
global.hotkey();
assert(adeline.get("neutral") == 60);
// The cameo test definitions reuse Hayden's hotkey, so select him directly.
assert(lns_palette_select(__lns_palette_runtime().characters[1], 1));
assert(hayden.get("neutral") == 401);
global.hotkey();
assert(adeline.get("neutral") == 10, "Wrapping back to Vanilla restores the original");
show_debug_message("PASS: portrait maps follow each character's selection on every screen");
