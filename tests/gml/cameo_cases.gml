// The four cutscene cameos are not NpcIds, but share NpcAnimationHandler.
global.actor_groups = {};
function instance_number(object) {
    var actors = global.actor_groups[$ string(object)];
    return actors == undefined ? 0 : array_length(actors);
}
function instance_find(object, index) { return global.actor_groups[$ string(object)][index]; }
function TestCameoActor(sprite) constructor {
    animator = {
        current: {sprite:sprite, index:2, frame:7.75},
        sprite: function() { return self.current.sprite; }
    };
    sprite_index = sprite;
    image_index = 2;
    image_xscale = -1;
    x = 34;
    y = 72;
}

global.world = true;
global.cameos = true;
global.__lns_palette_debug_hotkeys = false;
global.menu = new TestTextbox();
global.__lns_palette = undefined;
global.initialize();
var state = __lns_palette_runtime();
assert(state.ready, "Four supported cameos must not disable the whole collection");
assert(array_length(state.characters) == 5);
var objects = [1000, 1100, 1101, 1103, 1104];
for (var c = 0; c < array_length(state.characters); c++) {
    var character = state.characters[c];
    assert(character.object == objects[c], "NPC and cameo mappings must remain distinct");
    var source = character.world_pairs[0][0];
    var actor = new TestCameoActor(source);
    global.actor_groups[$ string(character.object)] = [actor];
}
global.initialize();
for (var c = 0; c < array_length(state.characters); c++) {
    var character = state.characters[c];
    var actor = instance_find(character.object, 0);
    var wrapper = actor.animator.sprite;
    assert(lns_palette_select(character, 1));
    assert(actor.sprite_index == character.world_pairs[0][1]);
    global.initialize();
    assert(actor.animator.sprite == wrapper, "Begin-step must not stack wrappers");
    assert(actor.animator.current.sprite == character.world_pairs[0][0]);
    assert(actor.animator.current.index == 2 && actor.animator.current.frame == 7.75);
    assert(actor.image_index == 2 && actor.image_xscale == -1 && actor.x == 34 && actor.y == 72);
    for (var k = c + 1; k < array_length(state.characters); k++) {
        var waiting = state.characters[k];
        assert(instance_find(waiting.object, 0).sprite_index == waiting.world_pairs[0][0],
            "Cameo selections must be independent");
    }
    actor.animator.current.sprite = 999;
    assert(actor.animator.sprite() == 999, "Unselected cameo animations pass through");
    var replacement = new TestCameoActor(character.world_pairs[0][0]);
    global.actor_groups[$ string(character.object)] = [replacement];
    global.initialize();
    assert(replacement.sprite_index == character.world_pairs[0][1],
        "A new cutscene instance inherits the selected cameo palette");
    assert(lns_palette_select(character, 0));
    assert(replacement.sprite_index == character.world_pairs[0][0]);
}
assert(global.warning_count == 0);
