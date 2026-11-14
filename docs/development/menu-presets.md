# Palette presets

Implemented 2026-11-09 after the owner approved all three preset kinds for the
first release ("1) all 2) before"). Interpreter tests pass. Shown the tone order
and the character table below, with the five weak fits named, the owner answered
**"close enough for now"**. That is provisional acceptance of those two tables,
not of how any preset looks in the game. A [live run](#live-run-2026-11-09) on
the title screen passes; the owner has not seen the rows or their results.

A preset here sets every character's palette choice at once, like the presets in
the Oxygen Not Included sibling mod. The code elsewhere uses "preset" for a
single palette choice (`preset_ids`, `src/presets.rs`); the functions for this
feature are named `lns_palette_preset_*` and live in
`mod/toggle/gml/palette_controls.gml`.

## User behavior

Six rows follow **Reset all to Vanilla** on the Palettes page, ahead of the
status row. They sit there rather than above the character list so that the
actions for the whole roster stay together and the accepted positions of the 36
character rows, Remember and Reset do not move.

| Row | Effect |
| --- | --- |
| **Everyone: …** | Shows the choice all characters share, or `Mixed`. Selecting it gives every character the next of nine tones: the seven in the order below, then Muted light and Muted medium. From `Mixed`, Vanilla or any other shared choice it starts at Light golden. |
| **Tanning Bed: one step darker** | Each character gets the tone one step darker than their original look. |
| **Mirror: swap light and dark** | Each character's place in the order is flipped end to end. |
| **Milk & Honey: two light tones** | Everyone ends on Light peach or Warm peach. |
| **Grayscale Agenda: lightest or darkest** | Everyone ends on Light golden or Very deep. |
| **Agartha: everyone lightest** | Everyone ends on Light golden. |
| **Wakanda: everyone darkest** | Everyone ends on Very deep. |
| **Randomize all** | Each character gets a random one of the nine tones. |

The last seven do not depend on the current choices, so selecting one twice
changes nothing further. A preset uses the same selection path as a character
row, saves once when **Remember choices** is on and writes nothing when it is
off. Storage is unchanged: version 1, one stable choice ID per character.

Whenever a preset would leave a character at their own place in the order, that
character returns to Vanilla, so the original art is shown instead of its
approximation. **Everyone** is the exception and applies the tone literally.

A character without the requested choice goes to Vanilla. That happens only in
an installation built from selected recipes that omit the shared tones.
Debug Blue and the three borrowed tones are never applied by a preset.

## Tone order

Lightest to darkest, by mean CIELAB lightness of each tone's replacement colors
averaged over the 36 characters:

| Position | Choice | ID | Mean lightness |
| --- | --- | --- | --- |
| 0 | Light golden | `npc_march` | 73.8 |
| 1 | Light peach | `player_04` | 65.7 |
| 2 | Warm peach | `npc_wiscar` | 58.5 |
| 3 | Warm brown | `player_16` | 40.5 |
| 4 | Deep brown | `player_22` | 24.5 |
| 5 | Deep umber | `player_27` | 21.9 |
| 6 | Very deep | `player_35` | 11.7 |

Deep brown and Deep umber are close; their order is the weakest call. Muted light
(63.1) and Muted medium (54.2) are left out because they differ from their
neighbors in saturation, not depth.

## Character table

Each character's position is the ordered tone whose replacement colors are
closest to the colors they replace in that character's recipe, by mean CIELAB
distance. The inputs are `palettes/sets/<id>-native-tones.json` and the profiles
they name. As a check on the method, March, Wiscar, Stillwell and Terithia each
come out nearest the tone adapted from them when all nine tones compete.

| Position | Characters |
| --- | --- |
| 0 Light golden | Juniper, March, Olric |
| 1 Light peach | Balor, Caldarus, Celine, Darcy, Dell, Elsie, Errol, Holt, Louis, Nora, Seridia, Stillwell, Taliferro, Valen, Wheedle |
| 2 Warm peach | Adeline, Eiland, Hayden, Merri, Terithia, Wiscar |
| 3 Warm brown | Hemlock, Josephine, Landen, Linnet, Luc, Maple, Reina, Ryis, Vera, Zorel |
| 4 Deep brown | Darren, Wynne |

Weak fits, where the nearest tone is still far (distance above 17; a close fit
such as Wiscar's is 3.5): Caldarus 23.1, Seridia 21.2, Hayden 19.9, Zorel 18.2,
Dell 17.4. These five are the ones most worth an owner's eye. Errol, Louis,
Stillwell and Wheedle are nearer Muted light than any ordered tone and are
placed at Light peach, the nearest ordered one.

## Preset maps

Each preset is one array from original position to displayed position.

| Preset | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Tanning Bed | 1 | 2 | 3 | 4 | 5 | 6 | 6 |
| Mirror | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| Milk & Honey | 1 | 1 | 2 | 2 | 2 | 2 | 2 |
| Grayscale Agenda | 0 | 0 | 0 | 6 | 6 | 6 | 6 |
| Agartha | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Wakanda | 6 | 6 | 6 | 6 | 6 | 6 | 6 |

The sibling mod's two mixes assign each duplicant by hand. These two are rules
instead: Milk & Honey moves each character to the nearer of Light peach and Warm
peach, and Grayscale Agenda sends the three lightest positions to Light golden
and the rest to Very deep. Mirror leaves the ten Warm brown characters on
Vanilla because the middle of seven maps to itself. No character is at position
5 or 6 today.

## Verification

Run:

```sh
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/local/tools/fabricator/bin/interpreter" cargo test --locked --test palette_controls -- --ignored'
```

Two new cases first failed because the preset functions and rows did not exist.
`presets` uses all 36 identities with the fourteen release choices and covers
the full Everyone cycle and wrap, the `Mixed` label, each map for one character
at every occupied position, twenty randomize passes, no writes with remembering
off, one write per preset with it on, restart, a failed write, and unchanged
focus and scroll. `presets_unavailable` uses choices that lack the shared tones.
The five earlier cases changed only where they index the status row (38 to 44)
and count rows (39 to 45, 40 to 46).

## Live run, 2026-11-09

The working tree was built with `nix build`, installed with the bare `install`
command into `tmp/finish-batch-install-lab`, and run twice in the private
hardware-rendered session used for earlier live checks, with isolated settings
and no save. `gpu-status` showed only desktop workloads; at the owner's
instruction ("skip the GPU reservation") the runs did not hold `gpu-run`.
Afterwards `uninstall` restored the archive and `Maybe.toml` byte for byte.

Mouse clicks opened Settings and Palettes from the title screen; arrow keys and
**E** did the rest. Results were read from the game's own log, which names the
choice applied to each character, and from the saved settings file.

| Step | Observed |
| --- | --- |
| Page | The six rows render after Reset; the three longest labels wrap to two lines. Up from the first row wraps to the status row, so Randomize is two key presses from the top. |
| Everyone, twice | 36 characters to Light golden, then 36 to Light peach; the row label follows. |
| Tanning Bed | 3 Light peach, 15 Warm peach, 6 Warm brown, 10 Deep brown, 2 Deep umber. |
| Mirror | 3 Very deep, 15 Deep umber, 6 Deep brown, 2 Warm peach, 10 Vanilla. |
| Milk & Honey | 3 Light peach, 12 Warm peach, 21 Vanilla. |
| Grayscale Agenda | 21 Light golden, 12 Very deep, 3 Vanilla. |
| Randomize, twice | Two different spreads, each within the nine tones. |
| Remembering off | No settings file after all of the above. |
| Remember on, then Mirror | The file holds version 1, `remember: true` and the Mirror counts. |
| Restart | The Mirror counts are restored. The first Randomize differs from the first launch's first Randomize for 32 of 36 characters. |

Every count equals what the character table and maps predict. Captures, logs and
recorded inputs are in `tmp/live-release/presets1/` and `presets2/`; build,
install and uninstall logs are in `tmp/menu-presets/`.

Not exercised: the rows from the in-game journal, a preset applied while a
portrait or world character is on screen, mouse or controller activation of a
preset row, and a failed save from a preset in the game. Presets change choices
through the same call as a character row, whose live behavior is recorded in the
[live release record](live-release-verification-2026-11-06.md). How the results
look is the owner's judgment and is not established here.
