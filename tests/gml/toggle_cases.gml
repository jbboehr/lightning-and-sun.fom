assert(global.hook_count == 1);
assert(global.hotkey_count == 0);
assert(!__lns_palette_runtime().ready);
global.initialize();
global.initialize();
assert(global.hotkey_count == 1);
assert(__lns_palette_runtime().ready);
assert(__lns_palette_runtime().selected == 0);

// Opening another kind of menu must not require a portrait or speaker method.
global.menu_opened({kind: Menu.Other, menu: {}});
global.menu = new TestTextbox();
global.menu_opened({kind: Menu.Textbox, menu: global.menu});
var wrapped = global.menu.set_speaker;
global.menu_opened({kind: Menu.Textbox, menu: global.menu});
assert(global.menu.set_speaker == wrapped);
assert(global.menu.portrait.changes == 0);

// Both directions preserve fractional animation phase.
global.hotkey();
assert(global.menu.portrait.sprite == 20);
assert(global.menu.portrait.index == 1.25);
assert(array_length(global.notices) == 0, "Palette popup obscures the portrait");
global.hotkey();
assert(global.menu.portrait.sprite == 10);
assert(global.menu.portrait.index == 1.25);

// A later speaker assignment uses the selected palette immediately.
global.hotkey();
global.menu.set_speaker({sprite: 10, index: 0.75});
assert(global.menu.original_calls == 1);
assert(global.menu.portrait.sprite == 20);
assert(global.menu.portrait.index == 0.75);

// Each supported expression keeps its own sprite pair and animation phase.
global.menu.set_speaker({sprite: 40, index: 1.75});
assert(global.menu.portrait.sprite == 50);
assert(global.menu.portrait.index == 1.75);
global.hotkey();
assert(global.menu.portrait.sprite == 40);
assert(global.menu.portrait.index == 1.75);
global.hotkey();
assert(global.menu.portrait.sprite == 50);

// Unsupported portraits are left alone in either state.
global.menu.set_speaker({sprite: 30, index: 0.5});
assert(global.menu.portrait.sprite == 30);
assert(global.menu.portrait.index == 0.5);
global.hotkey();
assert(global.menu.portrait.sprite == 30);

// Closing/reopening a menu and toggling with no menu retain the session choice.
global.menu = undefined;
global.hotkey();
global.menu = new TestTextbox();
global.menu_opened({kind: Menu.Textbox, menu: global.menu});
assert(global.menu.portrait.sprite == 20);
assert(global.menu.portrait.index == 1.25);

// A missing package fails closed and does not register a working hotkey.
global.__lns_palette = undefined;
global.missing_asset = true;
global.hotkey_count = 0;
lns_palette_initialize();
lns_palette_initialize();
assert(global.hotkey_count == 0);
assert(global.warning_count == 1);
assert(!__lns_palette_runtime().ready);
assert(__lns_palette_runtime().selected == 0);

// Losing either side of a later pair also disables the entire toggle.
global.__lns_palette = undefined;
global.missing_asset = false;
global.missing_asset_name = "spr_portrait_adeline_spring_happy";
global.hotkey_count = 0;
global.warning_count = 0;
lns_palette_initialize();
assert(global.hotkey_count == 0);
assert(global.warning_count == 1);
assert(!__lns_palette_runtime().ready);
assert(array_length(__lns_palette_runtime().pairs) == 1);

// Changing outfit keeps the selected palette and the new portrait's phase.
global.__lns_palette = undefined;
global.missing_asset_name = undefined;
global.seasons = true;
global.third_preset = true;
global.menu = new TestTextbox();
lns_palette_initialize();
lns_palette_menu_opened({kind: Menu.Textbox, menu: global.menu});
global.hotkey();
global.hotkey();
assert(global.menu.portrait.sprite == 60);
global.menu.set_speaker({sprite: 80, index: 0.75});
assert(global.menu.portrait.sprite == 100);
assert(global.menu.portrait.index == 0.75);
global.menu.set_speaker({sprite: 110, index: 1.25});
assert(global.menu.portrait.sprite == 130);
assert(global.menu.portrait.index == 1.25);
global.menu.set_speaker({sprite: 140, index: 0.5});
assert(global.menu.portrait.sprite == 160);
assert(global.menu.portrait.index == 0.5);
global.menu.set_speaker({sprite: 170, index: 1.25});
assert(global.menu.portrait.sprite == 190);
assert(global.menu.portrait.index == 1.25);
global.menu.set_speaker({sprite: 200, index: 0.75});
assert(global.menu.portrait.sprite == 220);
assert(global.menu.portrait.index == 0.75);
global.menu.set_speaker({sprite: 80, index: 0.5});
assert(global.menu.portrait.sprite == 100);
assert(global.menu.portrait.index == 0.5);
global.menu.set_speaker({sprite: 10, index: 1.5});
assert(global.menu.portrait.sprite == 60);
assert(global.menu.portrait.index == 1.5);
global.hotkey();
global.menu.set_speaker({sprite: 80, index: 0.5});
assert(global.menu.portrait.sprite == 80);
assert(global.menu.portrait.index == 0.5);
global.menu.set_speaker({sprite: 110, index: 0.75});
assert(global.menu.portrait.sprite == 110);
assert(global.menu.portrait.index == 0.75);
global.menu.set_speaker({sprite: 140, index: 1.75});
assert(global.menu.portrait.sprite == 140);
assert(global.menu.portrait.index == 1.75);
global.menu.set_speaker({sprite: 170, index: 0.25});
assert(global.menu.portrait.sprite == 170);
assert(global.menu.portrait.index == 0.25);
global.menu.set_speaker({sprite: 200, index: 1.5});
assert(global.menu.portrait.sprite == 200);
assert(global.menu.portrait.index == 1.5);
global.seasons = false;
global.third_preset = false;

// A third preset must cycle on the same expression and preserve raw phase.
global.__lns_palette = undefined;
global.missing_asset_name = undefined;
global.third_preset = true;
global.menu = new TestTextbox();
lns_palette_initialize();
lns_palette_menu_opened({kind: Menu.Textbox, menu: global.menu});
global.hotkey();
assert(global.menu.portrait.sprite == 20);
global.hotkey();
assert(global.menu.portrait.sprite == 60);
assert(global.palette_log == "Adeline palette: Warm trial");
assert(global.menu.portrait.index == 1.25);
global.menu.set_speaker({sprite: 40, index: 0.75});
assert(global.menu.portrait.sprite == 70);
assert(global.menu.portrait.index == 0.75);
global.hotkey();
assert(global.menu.portrait.sprite == 40);
assert(global.menu.portrait.index == 0.75);
global.third_preset = false;

// Vanilla plus the maximum eight presets must all be reachable before wrapping.
global.__lns_palette = undefined;
global.max_presets = true;
global.menu = new TestTextbox();
lns_palette_initialize();
lns_palette_menu_opened({kind: Menu.Textbox, menu: global.menu});
var expected_max_sprites = [20, 61, 62, 63, 64, 65, 66, 67];
for (var preset = 1; preset <= 8; preset++) {
    global.hotkey();
    assert(__lns_palette_runtime().selected == preset);
    assert(global.menu.portrait.sprite == expected_max_sprites[preset - 1]);
    assert(global.menu.portrait.index == 1.25);
}
global.hotkey();
assert(__lns_palette_runtime().selected == 0);
assert(global.menu.portrait.sprite == 10);
assert(global.menu.portrait.index == 1.25);
assert(global.palette_log == "Adeline palette: Vanilla");
assert(array_length(global.notices) == 0, "Cycling palettes queued HUD popups");

// The final asset is as mandatory as the first; partial maximum tables stay disabled.
global.__lns_palette = undefined;
global.missing_asset_name = "spr_lns_adeline_spring_neutral_eight";
global.hotkey_count = 0;
global.warning_count = 0;
lns_palette_initialize();
assert(global.hotkey_count == 0);
assert(global.warning_count == 1);
assert(!__lns_palette_runtime().ready);
global.max_presets = false;

global.__lns_palette = undefined;
global.missing_asset_name = "spr_lns_adeline_spring_happy_blue";
global.hotkey_count = 0;
global.warning_count = 0;
lns_palette_initialize();
assert(global.hotkey_count == 0);
assert(global.warning_count == 1);
assert(!__lns_palette_runtime().ready);
assert(array_length(__lns_palette_runtime().pairs) == 1);
