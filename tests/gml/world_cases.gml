// Native instance lookup is the boundary; the shipped wrapper and selector run.
globalvar obj_adeline;
obj_adeline = 1000;
global.actors = [];
function instance_number(object) { assert(object == obj_adeline); return array_length(global.actors); }
function instance_find(object, index) { assert(object == obj_adeline); return global.actors[index]; }
function TestActor() constructor {
    animator = {
        current: { sprite: 300, index: 0, frame: 0.25 },
        calls: 0,
        sprite: function() { self.calls++; return self.current.sprite; }
    };
    sprite_index = 300;
    image_index = 0;
    image_xscale = 1;
    x = 34;
    y = 72;
}

global.world = true;
global.characters = true;
global.__lns_palette = undefined;
global.menu = new TestTextbox();
global.initialize();
assert(__lns_palette_runtime().ready, "World palette must resolve the NPC object through the game mapping");
global.hotkey();
assert(global.menu.portrait.sprite == 20);
// Entering the room after choosing a palette attaches once and updates display.
var actor = new TestActor();
global.actors = [actor];
global.initialize();
assert(actor.sprite_index == 301, "Spawned world sprite did not follow the portrait palette");
var wrapper = actor.animator.sprite;
global.initialize();
assert(actor.animator.sprite == wrapper, "Repeated installation stacked wrappers");
// Rendering uses the original getter; facing, timing and world coordinates stay owned by the game.
actor.animator.current = {sprite: 310, index: 2, frame: 7.75};
actor.image_index = 2;
actor.image_xscale = -1;
assert(actor.animator.sprite() == 311);
assert(actor.animator.current.sprite == 310);
assert(actor.animator.current.index == 2);
assert(actor.animator.current.frame == 7.75);
global.hotkey_hayden();
assert(actor.animator.sprite() == 311, "Hayden selection changed Adeline's world palette");
assert(actor.animator.current.frame == 7.75);
assert(actor.x == 34 && actor.y == 72 && actor.image_xscale == -1);
global.hotkey();
assert(actor.sprite_index == 310);
assert(actor.image_index == 2 && actor.image_xscale == -1);
assert(actor.x == 34 && actor.y == 72);
assert(actor.animator.current.frame == 7.75);
assert(global.menu.portrait.sprite == 10);
global.hotkey();
assert(actor.sprite_index == 311);
assert(global.menu.portrait.sprite == 20);
// A different outfit or special action passes through unchanged.
actor.animator.current.sprite = 999;
assert(actor.animator.sprite() == 999);
global.hotkey();
assert(actor.sprite_index == 999);
// A replacement instance is discovered after room re-entry; a not-yet-ready animator waits.
var replacement = new TestActor();
replacement.animator.current = undefined;
global.actors = [replacement];
global.initialize();
replacement.animator.current = {sprite:300,index:0,frame:0};
global.hotkey();
assert(replacement.sprite_index == 301);
assert(replacement.animator.sprite() == 301);
// The game can replace the animator without replacing the actor. The next
// begin-step must attach to the new animator and retain the selected palette.
var replaced_animator = replacement.animator;
replacement.animator = {
    current: {sprite: 310, index: 3, frame: 2.5},
    calls: 0,
    sprite: function() { self.calls++; return self.current.sprite; }
};
replacement.sprite_index = 310;
global.initialize();
assert(replacement.animator != replaced_animator);
assert(replacement.animator.sprite() == 311);
assert(replacement.sprite_index == 311);
assert(replacement.animator.current.index == 3);
assert(replacement.animator.current.frame == 2.5);
// Missing a world variant disables the whole selection before anything is wrapped.
global.__lns_palette = undefined;
global.missing_asset_name = "spr_lns_npc_adeline_spring_walk_east_blue";
global.hotkey_count = 0;
var untouched = new TestActor();
global.actors = [untouched];
var original_getter = untouched.animator.sprite;
global.initialize();
assert(!__lns_palette_runtime().ready && global.hotkey_count == 0);
assert(untouched.animator.sprite == original_getter);
// Unknown NPC IDs must disable initialization before the game's strict object
// mapping is called, before controls register, and before any animator is wrapped.
global.__lns_palette = undefined;
global.missing_asset_name = undefined;
global.missing_npc_id = true;
global.warning_count = 0;
global.menu = new TestTextbox();
global.initialize();
global.initialize();
assert(!__lns_palette_runtime().ready && global.hotkey_count == 0);
assert(global.warning_count == 1);
assert(untouched.animator.sprite == original_getter);
assert(global.menu.portrait.sprite == 10);
