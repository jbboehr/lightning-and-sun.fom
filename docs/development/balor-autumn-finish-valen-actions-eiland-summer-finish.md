# Autumn reading and actions, and Summer tools

Following the approved batch in `9d99b3d`, this slice adds 21 strips: 85 source
frames and 41 native West mirrors, giving 126 review cases. The user approved the
offline artwork on 2026-09-30.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 6 | 25 + 0 | 201 | Autumn 28/28, complete |
| Valen | 11 | 31 + 12 | 183 | Autumn 17/31 |
| Eiland | 4 | 29 + 29 | 166 | Summer 41/41, complete |

The 36-character collection now has 3,735 sources and 14,940 variants. I, O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change; runtime Rust and GML
code remain unchanged.

## Source and material decisions

The mounted archive SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 529 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All 21 new sidecars match independent archive
reads, retaining 80×80 frames, the Default atlas and native Middle/54 origins.

Balor's seated reading and gem inspection use his four existing world skin
shades. Moving hands, faces and necks change; book pages and cover, gem and
sparkles, scarf, clothing, eyes and hair stay original. Both cycles face South:
reading has 3/4/3 start/loop/end frames and inspection has 4/7/4.

Valen's Autumn blinking, sitting, eating and drinking use her four world skin
shades. Goggles, coat, clothing, cup, food, eyes and hair stay original. North
sitting exposes no skin: its seed list stays empty and its PNG remains
byte-identical in every palette. North eating and drinking each expose 1/2/1
hand pixels over their three frames. Literal landmarks and per-frame counts
guard those tiny regions as well as the larger East/South skin areas.

Eiland's Summer axe, pickaxe, brush and trowel animations have 6, 6, 7 and 10
East frames respectively, each also mirrored West. His five world skin shades
cover faces and moving fingers. Wooden shafts, tool heads, brush bristles,
swing trails, gold trim, pink clothing, eyes and hair stay original. Unlike
Spring, these Summer strips need no shared skin-color clothing exclusions.

## Offline review

- [Five-choice sample summary](../../generated/balor-autumn-finish-valen-actions-eiland-summer-finish-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/balor-autumn-finish-valen-actions-eiland-summer-finish-preview/index.html).

The combined index is the entry point for all three character galleries. They
contain every source frame and native West mirror, including Valen's unchanged
North sitting. Eiland's full views include the entire tool swing and trail, with
additional enlarged details. All game-derived images and packages remain local
and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 209 active tests and the release
build passed. The normal suite leaves 260 local opt-ins ignored. Forty-two local
tests passed separately: three new material tests, thirty-six retained corpus
tests and three GML runtime tests. Six omission/spill controls generated successfully
before failing their intended pixel assertions. The synthetic package test rejected
an unregistered Balor gem strip before the additions and passed afterward.

All 14,940 combined variants validate. All 2,200 standalone character variants
pass strict recipe validation and match the combined output. All 5,290 prior
original/variant PNG and metadata files for these characters and all 32,015 files
for the other 33 characters remain byte-identical. Previous runtime rows, choices
and hotkeys remain exact. The thirty-six retained tests change only corpus paths
and source totals.

The current archive's native animator and NPC object pass 630 frame/palette
observations, 115 linear completions, 80 last-frame hold checks and 40 complex
phase transitions using simulated engine services and direct cycle selection.
Checks cover mirroring, palette binding, wrapper idempotence, frame phase,
cycle counters, completion state and portrait synchronization. Natural scheduling,
interaction/outfit dispatch, attached effects and full-engine rendering remain untested.

All 50 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks cover every case, native mirrors, metadata and complete opaque crops,
plus all fifteen source/palette bindings in the combined summary. The four preview
directories total 2,332,789 bytes (about 2.2 MiB).

MOMI installation passed in the fresh isolated
`tmp/bve-autumn-finish-actions-summer-finish-9d99b3d-playtest` copy, including required
compilation and installed-pixel/animation-metadata verification. Installed archive:
`cc0ec726d0b9f74cff9f554505318f3b9b80630d208d9755d4bdfaf858195017`.
The mounted archive and lab's original backup retain the source hash above; all
eleven registry/recipe inputs remained unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall roundtrip,
other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/bve-autumn-finish-actions-summer-finish-`. Author source records
use `tmp/balor-autumn-finish-author-`, `tmp/valen-autumn-actions-author-` and
`tmp/eiland-summer-finish-author-`. Fresh corpora use the corresponding
`extracted/<character>-<batch>-study` directories.
The `evidence.json` report summarizes the test, comparison, gallery and installation
logs; `scope.log` verifies that only the expected 48 paths changed and the Git index
is empty. Game-derived artifacts remain ignored.

## Next coverage

Continue with Balor's Winter idle/walk, Valen's Autumn general actions/sleep/kiss,
and Eiland's Autumn idle/walk. Valen still has 14 Autumn strips outside this batch.
