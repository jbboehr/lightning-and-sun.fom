# Winter completion and Autumn reading and writing

Following the approved batch in `bf9866b`, this slice adds 23 strips: 74 source
frames and 12 native West mirrors, giving 86 review cases. The user approved
the artwork on 2026-09-30. The latest completed review and build use the fixed shortcuts
`generated/review/` and `generated/build/`.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 6 | 25 + 0 | 229 | Winter 28/28, complete |
| Valen | 11 | 31 + 12 | 214 | Winter 17/31 |
| Eiland | 6 | 18 + 0 | 194 | Autumn 28/41 |

The 36-character collection has 3,822 sources and 15,288 variants. I, O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change.

## Source and material decisions

The mounted archive SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 614 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All 23 sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/54 origins.

Balor's Winter seated reading and gem inspection use his existing four world
skin shades for visible faces. Gloves, scarf, coat, trousers, boots, eyes and
hair stay original, as do the book, gem and sparkles. Reading has 3/4/3
start/loop/end frames; gem inspection has 4/7/4. Both are native South cycles.
These six strips complete his Winter sprite folder.

Valen's Winter blink, sit, eat and drink use her existing four world skin shades
for faces and visible hands. Coat, cuffs, goggles, boots, eyes, hair, cup and food
stay original. North sitting exposes no skin, has no seeds and remains
PNG-byte-identical in every palette. North eating and drinking each expose
1/2/1 hand pixels; literal frame counts and whole-image material checks guard
these small details. East strips retain their native West mirrors.

Eiland's Autumn seated reading and standing writing use his existing five world
skin shades. Nine clothing pixels share `#BA6A4C` with skin: two in reading start,
two in reading end, two in writing start, two in writing end and one in the third
writing-loop frame. Their disconnected components stay unselected. Tests
independently enumerate these coordinates and require their source colors and
unchanged output. Moving hand shadows stay selected. The teal book, writing
tools, cape, gold trim, pink clothing, boots, eyes and hair remain original,
including outfit color `#C9785A`. Reading has 3/4/3 start/loop/end frames;
standing writing has 2/4/2. Both are native South cycles.

## Offline review

- [Five-choice sample summary](../../generated/slice-002-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-002-review/index.html).

The combined index links the three `slice-002-<character>-review/` galleries.
Every new source frame and native West mirror appears, including unchanged
North sitting and all gem sparkles. The four review directories total 1,657,268
bytes (about 1.6 MiB). Game-derived images and packages remain local and ignored
by Git.

## Verification

Formatting, Clippy with warnings denied, all 213 active tests and the release
build passed; the normal suite leaves 272 local opt-ins ignored. Fifty-four
local tests passed separately: three new material tests, forty-eight retained
corpus tests and three GML runtime tests. Six omission/spill controls generated
before failing their intended pixel assertions. Eiland's spill control
reintroduces a shared-color clothing seed. The synthetic package test rejected
an unregistered Winter gem-inspection strip before the additions and passed afterward.

All 15,288 combined variants validate. All 2,548 standalone variants pass strict
recipe validation and match the combined build. All 6,140 prior original/variant
PNG and metadata files for these characters and all 32,015 files for the other
33 remain byte-identical. Previous runtime rows, choices and hotkeys remain
exact. The forty-eight retained tests change only corpus paths and source totals.

The current archive's native animator and NPC object pass 430 frame/palette
observations, 75 linear completions, 80 last-frame hold checks and 80 complex
phase transitions using simulated engine services and direct cycle selection.
Checks cover mirroring, palette binding, wrapper idempotence, frame phase, cycle
counters, completion state and portrait synchronization. Natural scheduling,
interaction/outfit dispatch, attached effects and full-engine rendering remain
untested.

All 38 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks cover every case, native mirrors, metadata and complete opaque
crops, plus all fifteen source/palette bindings in the combined summary.
The `review` shortcut also passes Chromium loading; all 35 linked HTML pages
and 69 image references resolve through the short path.

MOMI installation passed in the isolated `tmp/slice-002-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; eleven
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-002-`. Author records use
`tmp/balor-winter-finish-author-`, `tmp/valen-winter-actions-author-` and
`tmp/eiland-autumn-writing-author-`; fresh corpora use the corresponding
`extracted/<character>-<batch>-study` directories. Eiland's `author-clothing.json`
records the nine protected clothing pixels, retained in the tracked material
test. `evidence.json` summarizes tests, comparisons, galleries and installation,
including the installed archive hash. `shortcuts.json` records the browser and
link checks through `generated/review/`. `scope.log` checks the expected 60 paths
and empty index.

## Next coverage

Begin Balor's Beach idle/walk pilot; all six strips and native outfit support
are present in the current archive's 18-strip Beach folder. Continue Valen's
Winter general actions/sleep/kiss and Eiland's Autumn seated writing and
magnifying-glass cycles.
