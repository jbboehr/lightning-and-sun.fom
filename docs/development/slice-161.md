# Dell Summer eating and drinking

Following `f83b620`, this slice adds Dell's Summer eating and drinking in
North, South and East, plus native West mirroring. Twenty-two source frames
give thirty review cases. The user approved the artwork on 2026-10-11. Local outputs use
`generated/review/` and `generated/build/`.

Dell has 82 sources: 32 portraits and 50 world strips. The 36-character collection
has 4,738 sources and 18,952 variants. Y retains Vanilla, Debug Blue, Hayden, Ryis
and Seridia choices with portrait/world synchronization. This slice adds no
runtime code or color roles. All 76 previous regions, groups, target mappings
and collection sets remain unchanged. Historical pixel tests retain their
expectations; only the central Dell coverage total grows.

## Source and materials

The game mount was empty. Source is the preserved original archive at
`tmp/slice-160-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 76 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across twenty-two
frames, 485 component seeds select 896 pixels. Faces, eyelids, exposed arms,
moving hands and legs follow the palette. The third eating frame raises the
upper face two rows, with a taller jaw, requiring different bounds from Spring.
Four fringe pixels connected to closed East eyelids follow their edges:
frame 2 `(44,34)`; frame 3 `(43,32)` and `(44,32)`; frame 4 `(43,35)`.
Coordinates are frame-relative and frame numbers here start at one.
Other blonde hair, lashes, ribbon, eye details, mouth interiors, blue shirt,
brown overalls, pale rolled cuffs and red shoes stay original.

All source frames were inspected in five-choice sheets, with original pixel
grids used to distinguish matching skin and hair colors. The material test
checks every pixel independently of profile seeds. It fixes 304 literal
landmarks and per-frame counts, plus transparency, metadata, unchanged profile
roles and prior output bytes against slice 160. In North views, only exposed
hands and arms change; hair and clothing cover the rest.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Drink East | 3 | 41, 47, 41 |
| Drink North | 3 | 18, 14, 18 |
| Drink South | 3 | 52, 63, 52 |
| Eat East | 5 | 40, 47, 44, 44, 39 |
| Eat North | 3 | 18, 14, 18 |
| Eat South | 5 | 53, 58, 56, 65, 54 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Drinking
and North eating use scalar duration `1.0`; East/South eating use durations
`[0.125, 0.15, 0.175, 0.125, 0.6]`. Both native cycles are linear and seated,
with final-frame holds `[240, 360]`. Original cycle configuration remains unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-161-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-161-review/index.html).

The summary samples drink East frame 2, eat East frame 3 and eat South frame 3.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 373 regular tests and the release
build passed through the Nix shell; 443 opt-in tests were ignored. The selected
local suite passed 233 tests, including every previous Dell material test,
the new eating/drinking test, historical character checks and three GML runtime
tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the open-mouth face, a raised hand or exposed leg pixels, or selected hair.
Each generated successfully and failed the intended pixel assertion. All six
metadata sidecars matched independent archive reads.

All 328 standalone Dell variants passed strict recipe validation; 656 final
PNG/metadata files match the author candidate. All 18,952 combined variants
validated. The 760 prior Dell PNG/metadata files and 46,735 other-character
files match slice 160 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All thirteen Chromium gallery pages loaded their local images and links without
overflow. Thirty cases include sixty full-view bindings, enlarged details and
fifteen summary bindings; 6,350,760 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
523,406 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-161-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`c8fe292de6be4e44465516dfe0e26f96ecacbf692d9313a3b96b5adeb6f35082`.
The archive's native animator and NPC animate function passed 150 frame/palette
observations, forty linear completions and eighty final-frame hold checks under
Fabricator. Palette binding, frame phase, West mirroring, seated status, wrapper
idempotence, cycle counters and Summer portrait synchronization were preserved.
Engine services are simulated and cycles selected directly. Natural gameplay
dispatch, speaking/background pause transitions and live rendering were not
exercised. The desktop launcher and installed game were not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-161-`; author records use
`tmp/dell-summer-food-author-`. Fresh exports are in
`extracted/dell-summer-food-study`, reached through `extracted/test-corpus/dell`.
The fixed baseline is `generated/slice-160-build/characters/dell`.

## Remaining work

The archive inventory leaves 7 Summer, 21 Autumn and 21 Winter world strips.
Spring is complete. Next: Dell Summer general actions (three strips), followed
by head scratching and swinging a stick (four strips).
Natural gameplay dispatch, speaking/background pause transitions and live
rendering remain untested.
