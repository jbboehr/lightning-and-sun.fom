# Reina and Juniper portrait batch

This batch expands the existing character workflow with two authors working
concurrently and a separate art reviewer. Each author owns one character's
profile, palette recipes, review configuration, tests, and authoring record.
The integration owner maintains the shared registry, collection, package, and
playtest controls. Temporary programs and images use separate local directories.

The user receives one compact comparison rather than component selections or
separate agent reports. `generated/parallel-characters-preview/index.html`
and its `summary.png` show both new characters with their five choices.
The individual [Reina](reina-portraits.md) and [Juniper](juniper-portraits.md)
records describe source colors, masks, exclusions, and detailed art checks.

## Coverage and choices

| Character | Each season | Beach/bath | Wedding | Total strips | Palette key |
| --- | --- | --- | --- | --- | --- |
| Reina | 19 | 20 | 7 | 103 | Home |
| Juniper | 25 | 27 | 5 | 132 | Page Down |

Both characters start on Vanilla. The remaining choices are Debug Blue, Hayden,
Ryis, and Seridia colors. Each uses the same source-bound masks across all
recolors. Source artwork, head hair, clothes, accessories, and unselected facial
details retain their original pixels. This batch does not add overworld, UI,
or child sprites.

The combined collection now has 620 original strips and 2,480 variants across
Adeline, Hayden, Ryis, Reina, and Juniper. Of those originals, 603 are portrait
strips and 17 are the previously reviewed Adeline world animations. The shared
Rust and GML runtime is unchanged; the registry and preset sets supply the new
characters. Each character can also be installed alone through its preset set.

## Atlas and preview controls

Registry atlas values come from each original metadata sidecar. Juniper's beach
portraits use `PortraitsMisc`; Reina's use `PortraitsSummer`. The local preview
helper now chooses atlases from a lookup generated from the registry, including
when switching characters within the same outfit. It retains `PortraitsSpring`
for the game and Hayden's beach strip stored there.

Juniper's four `beach_accident_*` strips belong to the native `beach_accident`
wardrobe. The preview lists them within the beach group but passes that separate
wardrobe and the unprefixed expression to `NpcSpeaker`. Requesting them as
`beach` plus `accident_*` fails native portrait lookup. The local helper handles
this translation; its exact sprite/palette assertions remain unchanged.

The pinned MOMI key resolver explicitly supports `HOME` and `PAGE_DOWN`. These
bindings avoid the keys already used by the other characters and the preview.
A future palette menu can replace this growing list of prototype hotkeys.

This batch's ignored launcher used `tmp/parallel-character-playtest`
with separate state and read-only source game mounts. F7 opens Reina and advances
expressions; Page Up goes backward. F4 changes character, F5 changes outfit,
Home cycles Reina, and Page Down cycles Juniper. The existing F6/F8/F10 bindings
remain Adeline/Hayden/Ryis. The previous three-character trial is retained in
`tmp/characters-natural-playtest`.
The shared `./tmp/play-characters` launcher now follows the
[latest combined trial](characters.md#local-visual-check); the five-character
copy remains available separately.

## Rebuild

```sh
target/release/mistria-palette build-characters \
  --archive tmp/fields-of-mistria/assets.zip \
  --characters palettes/sets/characters-trial.json \
  --output generated/characters-parallel-trial
```

Use a fresh output directory. Game assets, generated variants, preview images,
packages, and test helpers remain ignored. Only recipe data, registry paths,
tests, and documentation belong in Git.

## Verification

The standalone/combined character regression was extended to Reina, Juniper,
and all five characters together. It first failed with `Unsupported character:
reina`, then passed after registry integration. It retains the Ryis standalone
and three-character cases, differing preset-count checks, generated image
checks, and exact runtime control expectations.

Both new corpus tests were exercised against incomplete or unrestricted recipes
before passing with the final masks. Independent review checked all output pixels
in all four palettes, plus 28 Reina and 37 Juniper skin/clothing landmarks. It
caught shared-color jewelry and clothing boundaries, including Juniper's spring
and wedding armband rims and a winter wrist jewel. Those corrections were
rechecked in enlarged images. No concrete art defect remains from that review;
unclassified fine details can still require later visual correction.

Fresh integration checks passed formatting, Clippy with warnings denied, all
87 active synthetic tests, three opt-in GML interpreter tests, both new local
portrait corpus tests, and the release build. The final combined package passed
exact recipe validation for all 2,480 variants. A separate registry audit matched
all 235 new paths and their atlases to the source archive, checked preset color
counts and catalog values, and matched standalone Blue to each profile.

MOMI installed the final package into the isolated playtest copy and verified
installed pixels, animation metadata, scripts, and preserved mod selection/order.
Uninstalling a separate copy restored the supplied archive byte for byte. The
source archive retains SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

The installed portrait atlases occupy 13 pages, compared with 12 in the approved
three-character natural-palette trial. Full RGBA8 capacity across those portrait
pages is 832 MiB versus 768 MiB; this is texture capacity, not measured resident
memory. The preview loads only the atlas groups needed for the current outfit,
plus Spring. Actual desktop performance still depends on the game and GPU.

The first automated preview attempt exposed a test-harness timing assumption:
the first textbox became ready after about one second, while the assertion ran
after 300 ms. Two isolated runs reproduced that delay. The harness now waits for
the expected current label with a five-second deadline; production GML needed
no change.

The real game, running with software rendering in an isolated display, exercised
all 603 portrait strips across all five choices: 3,015 distinct source/palette
pairs exactly matched the configured collection. Checks asserted the selected
sprite, fractional animation phase, independent character selections, and a
hidden debugger console. Home and Page Down each completed a full cycle back to
Vanilla. Enlarged screenshots of Reina and Juniper in Blue and a natural palette,
plus Juniper's beach-accident portrait, were inspected.

A separate world pass checked all six outfit transitions and switching all five
characters within the beach group, then passed 24 action/facing phase and
movement checks. The initial world pass revealed a fixture ordering issue:
previewing Adeline's beach portrait changes the shared NPC wardrobe, while the
world helper expects reviewed Spring animations. Pressing the existing F1 again
before the action loop resets Spring and safely replaces the test actor. The
world helper and its assertions needed no changes. Portrait and world evidence
come from separate phases; the original failed fixture log remains available.

For manual world checks, press F1 after finishing portrait/outfit switches, then
use F3/F12/F11. The user accepted this batch visually and approved committing it,
with possible mouth-color issues explicitly deferred to a later art pass. The
affected character, expression, and pixels have not yet been isolated. Keep this
follow-up alongside Hayden's previously deferred embarrassed-expression pixels;
neither is part of the next character-coverage batch.

Local evidence includes `tmp/parallel-character-final-checks.log`,
`tmp/parallel-character-build-report.json`,
`tmp/parallel-character-final-install-report.log`,
`tmp/parallel-character-uninstall-report.json`,
`tmp/parallel-character-validation-count.txt`,
`tmp/parallel-character-atlas-memory.json`,
`tmp/parallel-character-live-verification.txt`,
`tmp/parallel-character-portrait-matrix-game.log`,
`tmp/parallel-character-world-headless/state/game.log`,
`tmp/reina-integration-audit-report.json`, and
`tmp/parallel-art-review-final.md`. The individual author records contain their
corpus, mask roundtrip, and mutation-check evidence.
