assert(global.hook_count == 1);
assert(global.hotkey_count == 0);
assert(!__lns_palette_runtime().ready);
global.initialize();
global.initialize();
assert(global.hotkey_count == 1);
assert(__lns_palette_runtime().ready);
assert(!__lns_palette_runtime().blue);

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
global.hotkey();
assert(global.menu.portrait.sprite == 10);
assert(global.menu.portrait.index == 1.25);

// A later speaker assignment uses the selected palette immediately.
global.hotkey();
global.menu.set_speaker({sprite: 10, index: 0.75});
assert(global.menu.original_calls == 1);
assert(global.menu.portrait.sprite == 20);
assert(global.menu.portrait.index == 0.75);

// Other expressions and characters are left alone in either state.
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
assert(!__lns_palette_runtime().blue);
