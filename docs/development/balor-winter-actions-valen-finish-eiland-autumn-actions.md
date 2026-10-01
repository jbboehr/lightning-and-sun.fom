# Winter actions and Autumn completion

Following the approved batch in `deab671`, this slice adds 31 strips: 86 source
frames and 30 native West mirrors, giving 116 review cases. The user approved
the offline artwork on 2026-09-30.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 11 | 31 + 12 | 218 | Winter 17/28 |
| Valen | 9 | 24 + 6 | 197 | Autumn 31/31, complete |
| Eiland | 11 | 31 + 12 | 183 | Autumn 17/41 |

The 36-character collection has 3,783 sources and 15,132 variants. I, O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
This slice changes registry/profile data, tests and documentation.

## Source and material decisions

The mounted archive SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 567 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All 31 sidecars match independent archive reads:
80×80 frames, Default atlas, native Middle or numeric 40 horizontal origins,
and vertical origin 54.

Balor's Winter blink, sit, eat and drink recolor visible faces with the existing
four world skin shades. Gloves, scarf, coat, trousers, boots, eyes and hair stay
original. North drink, eat and sit expose no skin: their seven frames have empty
seed lists and remain PNG-byte-identical in every palette.

Valen's Autumn seated reading, standing writing and healing use her existing
four world skin shades. Faces and moving hands change; books, writing implements,
healing props, goggles, coat, cuffs, boots, eyes and hair stay original. Reading
has 3/4/3 start/loop/end frames, writing 2/4/2, and healing 1/4/1. Healing mirrors
East to West. These nine strips complete her Autumn folder.

Eiland's Autumn blink, sit, eat and drink use his existing five world skin shades.
Seventy-three individually reviewed clothing pixels share `#BA6A4C` with skin
shadows. Their disconnected components stay unselected. Tests enumerate those
coordinates and verify source colors and unchanged output. The moving finger
shadow in the final East eating frame remains selected; North eating/drinking
each recolor 2/3/2 raised-hand pixels. North sitting exposes no skin and remains
PNG-byte-identical. Cape, gold trim, pink clothing, boots, eyes and hair stay
original, including the outfit's portrait-role colors `#C9785A` and `#6A3126`.

## Offline review

- [Five-choice sample summary](../../generated/balor-winter-actions-valen-finish-eiland-autumn-actions-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/balor-winter-actions-valen-finish-eiland-autumn-actions-preview/index.html).

The combined index links all three galleries. They cover every new source frame
and native West mirror, including unchanged covered views. The four review
directories total 2,171,651 bytes (about 2.1 MiB). Game-derived images and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 211 active tests and the release
build passed; the normal suite leaves 266 local opt-ins ignored. Forty-eight
local tests passed separately: three new material tests, forty-two retained
corpus tests and three GML runtime tests. Six omission/spill controls generated
successfully before failing their intended pixel assertions. Eiland's spill
control reintroduces a shared-color clothing seed. The synthetic package test
rejected an unregistered Winter blink strip before the additions and passed afterward.

All 15,132 combined variants validate. The 2,392 standalone variants pass strict
recipe validation and match combined output. All 5,670 prior original/variant
PNG and metadata files for these characters and all 32,015 files for the other
33 characters remain byte-identical. Previous runtime rows, choices and hotkeys
remain exact. The forty-two retained tests change only corpus paths and totals.

The current archive's native animator and NPC object pass 580 frame/palette
observations, 150 linear completions, 160 last-frame hold checks and 80 complex
phase transitions using simulated engine services and direct cycle selection.
Checks cover mirroring, palette binding, wrapper idempotence, frame phase, cycle
counters, completion state and portrait synchronization. Natural scheduling,
interaction/outfit dispatch, attached effects and full-engine rendering remain
untested.

All 55 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks cover all cases, mirrors, metadata and complete opaque crops, plus
the fifteen source/palette bindings in the combined summary.

MOMI installation passed in the fresh isolated
`tmp/bve-winter-actions-autumn-finish-actions-deab671-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification.
The mounted archive and lab's original backup retain the source hash above;
eleven registry/recipe inputs remained unchanged through generation and
installation. No preview helper was installed or desktop launcher changed.
An uninstall roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/bve-winter-actions-autumn-finish-actions-`. Author records use
`tmp/balor-winter-actions-author-`, `tmp/valen-autumn-finish-author-` and
`tmp/eiland-autumn-actions-author-`. Fresh corpora use the corresponding
`extracted/<character>-<batch>-study` directories. Eiland's `author-clothing.json`
records the 73 protected clothing pixels, retained independently in the tracked
material test. The `evidence.json` report summarizes test, comparison, gallery
and installation logs, including the installed archive hash. The final scope
includes 55 paths after documenting the shorter output naming convention.
Local `generated/review` and `generated/build` shortcuts were checked, including
52 linked review pages, 103 image references and Chromium summary loading.

## Next coverage

Continue Balor's Winter and Eiland's Autumn general actions, sleep and kiss.
Start Valen's Winter idle/walk pilot.
