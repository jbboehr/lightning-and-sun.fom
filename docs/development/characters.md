# Independent character palettes

The combined trial includes Adeline's 143 reviewed animations, Hayden's 133,
Ryis's 109, Reina's 103, Juniper's 132, Celine's 183, March's 181, Balor's 110,
Valen's 92, Eiland's 78, Olric's 36, Landen's 32, Nora's 32, Holt's 32,
Josephine's 32, Darcy's 32, Dell's 32, Elsie's 36, Errol's 36, Hemlock's 32,
Louis's 32, Luc's 32, and Maple's 32 portrait strips, each with four recolors:
6,768 generated variant strips. All twenty-three characters start on Vanilla
each session.
F6 cycles Adeline, F8 Hayden, F10 Ryis,
Home Reina, Page Down Juniper,
Insert Celine, U March, I Balor, O Valen, J Eiland, K Olric, L Landen, N Nora,
H Holt, P Josephine, B Darcy, Y Dell, Z Elsie, X Errol, F Hemlock, V Louis,
F11 Luc, and F12 Maple.
Each has five choices.
The [shared NPC presets](shared-npc-presets.md)
record the earlier natural colors; the [parallel portrait batch](parallel-portraits.md)
records Reina and Juniper. The [Celine and March batch](celine-march-portraits.md)
adds gardening and special-expression coverage. The next
[Balor and Valen batch](balor-valen-portraits.md) adds 202 portrait strips.
The [Eiland and Olric batch](eiland-olric-portraits.md) adds 114 more, including
Olric's seasonal bunny-ears portraits.
The [Landen and Nora batch](landen-nora-portraits.md) adds 64 seasonal portrait strips.
The [Holt and Josephine batch](holt-josephine-portraits.md) adds another 64.
The [Darcy and Dell batch](darcy-dell-portraits.md) adds 64 more seasonal strips.
The [Elsie and Errol batch](elsie-errol-portraits.md) adds another 72.
The [Hemlock and Louis batch](hemlock-louis-portraits.md) adds 64 more seasonal strips.
The [Luc and Maple batch](luc-maple-portraits.md) adds 64 more.
Their overworld sprites are not covered. The small embarrassed-expression art
follow-up remains deferred in [Hayden portraits](hayden-portraits.md).

## Add and review one character at a time

- `mod/config/characters.json` records character IDs, labels, temporary hotkeys,
  and exact reviewed animation paths with their original atlas.
- A region profile under `palettes/profiles/` owns the character's reviewed masks,
  source hashes and color groups. A preset set under `palettes/sets/` supplies
  colors. There is no requirement for characters to have the same preset count.
- A collection such as `palettes/sets/characters-trial.json` selects the characters
  and points to each preset set. Paths are relative to the containing definition.
  Select just one entry to build or install that character alone. Existing
  `build-presets` and `install --presets` also accept any single character set.

The registry contains filenames and atlas names, not image data. When expanding
coverage, add only reviewed paths and verify the atlas against local metadata;
do not infer it from the filename. Hayden's `beach_shy_special` strip lives in the
Spring folder and uses `PortraitsSpring` even though its name says beach.
Juniper's beach portraits use `PortraitsMisc`, whereas Reina's use
`PortraitsSummer`.
The export command accepts up to 256 distinct strips per character. Celine's
183 and March's 181 required raising the earlier 143-strip prototype limit.

Build a combined local bundle:

```sh
nix-shell --pure --run 'cargo run --locked -- build-characters \
  --archive tmp/fields-of-mistria/assets.zip \
  --characters palettes/sets/characters-trial.json \
  --output generated/characters-trial'
```

The bundle contains per-character originals and generated variants, one MOMI
package, and a JSON report. All generated content stays under ignored paths.
The installer uses the same collection and package builder, verifies every
character's vanilla and variant atlas frames and the complete generated runtime
table, then publishes one reversible archive transaction. Existing mod selection
and load-order checks still apply. Uninstall restores the entire previous archive.
The MOMI manifest retains the historical `Adeline Palette Toggle Study` name.

## Runtime

The generated GML table holds one row per character: ID, label, hotkey,
preset labels, and animation groups. Each group starts with vanilla.
The runtime resolves the complete table before enabling any hotkeys, and keeps
selection and sprite pairs in separate character states. The shared textbox hook
finds the owner of the current portrait, including already-recolored variants,
and preserves its fractional animation phase. World animators reference their
character's state; they retain their original timing, movement and animation packs.
World objects come from the game's `try_string_to_npc_id` and
`npc_id_to_gm_obj_id` mapping. The real game does not resolve objects through
`try_string_to_asset`, and it does not expose Fabricator's `asset_get_index` API.

The per-character keys are test controls. A future character-and-palette menu
should call the same `lns_palette_toggle(character)` selection path (or a direct
selection function) instead of assigning a new key to every NPC. We can finish
masks for one character before adding the next; changing the shared game hook
is not part of each art pass.

## Verification

Run the normal checks from [verification](verification.md). The synthetic
character bundle test checks different preset counts and rejects wrong-owner
and duplicate selections. Installer tests check a corrupt second character's
atlas cannot be published and that uninstall restores the complete archive.

```sh
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" cargo test --locked --test gml_runtime -- --ignored --nocapture'
```

These interpreter tests exercise both character callbacks, Vanilla restoration,
speaker/menu changes, missing assets and NPC IDs, fractional portrait phase, and
Adeline's world animator while Hayden changes palette.

Initial Adeline/Hayden verification, 2026-09-07: formatting, Clippy, 86 active
tests, the three opt-in GML tests, and the release build passed. All 705 regenerated strips passed exact
recipe validation. A fresh MOMI installation verified every selected vanilla and
variant frame, metadata, and generated script. Uninstall restored the prior
archive byte-for-byte. A separate Hayden-only bundle contained 133 variants and
only his F8 control.

The real game ran in a new isolated state directory under Xvfb. Its 259 unique
portrait checks covered every included expression across both characters and
all six outfits, cycling both characters' selections while checking the displayed
sprite and fractional phase. Another 24 checks exercised Adeline's world actions
and directions while both palette controls cycled, preserving animator state,
movement, facing, and food/drink renderers. No script errors or disabled palette
callbacks were logged. An additional live check cycled all six portrait outfits
with the world preview active, also without script or callback errors.
The supplied source archive's SHA-256 remained
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

The independent test pass added a Hayden-only runtime table check. Live testing
found the object-lookup mismatch described above; the corrected engine stub first
failed the world regression, then passed with the native NPC mapping. A separate
review suggestion to verify unselected animations against arbitrary MOMI
corruption concerned the pre-existing verifier scope; no new defect was
demonstrated there, and that broader work was deferred.

Reliability verdict: **PASS_WITH_RESIDUAL_RISK**. The remaining concrete gaps are
interaction with other mods that wrap the same game methods and the deferred
Hayden embarrassed-expression pixels.
The interpreter and game tests cover runtime behavior; they are not a finished
art review of every recolored frame.

The first preview used F2, which also opens the game's debugger. The sprite
checks missed that overlay. The preview now uses F4, and its F9 check also asserts
that the debug console is hidden. A live regression failed with F2 and passed
with F4 in both directions; Hayden's blue portrait was visually confirmed with
the console closed. The rebuilt launcher uses the separate
`tmp/characters-f4-playtest` copy. Formatting, Clippy, all 86 active tests, the
release build, and fresh MOMI verification passed for this correction.
The user then tried the corrected preview on their desktop and accepted its
appearance. This was a visual spot check, not a review of every frame.

### Ryis integration, 2026-09-07

The registry adds Ryis's 109 reviewed paths with atlas names taken from their
source metadata. His standalone preset set and the combined collection use the
same profile and Debug Blue colors as the accepted offline art. This addition
does not change the shared Rust or GML runtime.

The new synthetic test first failed with `Unsupported character: ryis`, then
passed for Ryis alone and alongside Adeline and Hayden. It checks Ryis's F10
control, vanilla-first choices, generated pixels, and the existing characters'
different preset counts. Formatting, Clippy, all 87 active tests, the three
opt-in GML tests, the Ryis corpus test, and the release build passed.

All 814 variants in `generated/characters-ryis-trial` passed exact recipe
validation. A separate Ryis-only bundle's 109 recolored strips matched the
approved offline outputs pixel-for-pixel. A fresh MOMI installation into
`tmp/ryis-playtest` verified every selected vanilla and variant atlas frame,
metadata, and generated scripts. Uninstalling a separate copy restored the
original archive byte-for-byte. The supplied game archive retained the SHA-256
recorded above.

The real game ran under Xvfb with separate state. Its 368 distinct portrait
checks matched every included source strip across all three characters and all
six outfits. Each check cycled every character's palettes while asserting
independent selections, the displayed sprite, fractional animation phase, and
a closed debugger console. F10 and Page Up were also exercised as actual key
bindings, and Ryis's blue portrait was visually inspected. Six outfit
transitions with Adeline's world preview active and 24 world action/direction
checks preserved animation state and movement while all palettes cycled.
No script errors or disabled palette callbacks were logged. The isolated run
logged a Steam initialization error and a starting-room placement warning;
it continued through the full test matrix. The user then tried the integrated
trial on their desktop and accepted its appearance as a visual spot check.

An earlier full check hit an intermittent assertion failure in the existing
incomplete-installer-output test. It did not reproduce in a focused rerun,
three installer-suite reruns, or the subsequent full checks. Its assertion now
prints the unexpected stderr; the cause remains undiagnosed, and no behavior
or assertion was weakened.

Local evidence includes `tmp/ryis-integration-final-checks.log`,
`tmp/ryis-combined-build-report.json`, `tmp/ryis-standalone-comparison.json`,
`tmp/ryis-install-report.json`, `tmp/ryis-uninstall-report.json`,
`tmp/ryis-live-matrix-run.log`, and `tmp/ryis-live-matrix-game.log`.

## Local visual check

The ignored `tmp/play-characters` launcher opens the combined package with its
own saves and state in `tmp/luc-maple-detail-playtest`, mounting the supplied
game files read-only. Previous trial copies are retained separately.
Run it from the normal desktop terminal:

```sh
./tmp/play-characters
```

- F7 opens the portrait trial on Luc and advances expressions; Page Up goes backward.
- F4 switches the displayed character; F5 switches outfits. F2 belongs to the
  game's debugger and is not a preview control.
- F6 cycles Adeline's palette; F8 switches Hayden's palette; F10 switches Ryis's palette.
- Home cycles Reina's palette; Page Down cycles Juniper's palette.
- Insert cycles Celine's palette; U cycles March's palette.
- I cycles Balor's palette; O cycles Valen's palette.
- J cycles Eiland's palette; K cycles Olric's palette.
- L cycles Landen's palette; N cycles Nora's palette.
- H cycles Holt's palette; P cycles Josephine's palette.
- B cycles Darcy's palette; Y cycles Dell's palette.
- Z cycles Elsie's palette; X cycles Errol's palette.
- F cycles Hemlock's palette; V cycles Louis's palette.
- F11 cycles Luc's palette; F12 cycles Maple's palette.
- F9 checks independent palette cycles and portrait phase.
- Optional world controls: press F1 after finishing portrait/outfit switches to
  reset Adeline's Spring test actor. F3 changes action, Shift+F12 changes facing,
  and Shift+F11 checks world phase and movement.

The world helper disables Adeline's town scheduling before manually placing her,
as established by the earlier pathfinding-crash fix. These preview controls and
scheduling changes are excluded from the player package. The launcher is retained
by a Nix output link; rebuild it if necessary with
`nix build --impure --file tmp/luc-maple-detail-playtest-launch.nix --out-link tmp/play-characters`.

F5 skips outfit groups with no included portraits. Olric, Landen, Nora, Holt,
Josephine, Darcy, Dell, Elsie, Errol, Hemlock, Louis, Luc, and Maple cycle spring,
summer, autumn, and winter.
F4 retains the current outfit when the next character has portraits for it,
otherwise it falls back to spring.
Bunny ears appear among Olric's expressions; the helper selects the game's
corresponding native outfit.
