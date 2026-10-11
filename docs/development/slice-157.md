# Dell Spring head scratching and yawning

Following `86d1202`, this slice adds head-scratching and yawning start/loop/end
sequences. Eleven source frames give eleven review cases. Both cycles face
South and have no West mirrors. The user approved the artwork on 2026-10-10.
Local outputs use `generated/review/` and `generated/build/`.

Dell has 61 sources: 32 portraits and twenty-nine world strips. The 36-character
collection has 4,717 sources and 18,868 variants. Y retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This slice
adds no runtime code or color roles. All 55 previous regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests retain
their expectations; only the central Dell coverage total grows.

## Source and materials

The game mount was empty. Source is the preserved original archive at
`tmp/slice-156-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 55 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across eleven
frames, 356 component seeds select 568 pixels. Faces, eyelids, necks, exposed
wrists, hands and ankles follow the palette. Matching blonde hair, lashes,
eye details, red mouth interiors, ribbon, shirt, overalls, brown trim and boots
stay original. The scratching loop moves her hair while exposing a small wrist
below it. The yawning hand moves in front of the mouth; its red interior uses
`#A1121D`, also used in clothing. The yawn raises the nose and connects a two-pixel
shadow at the upper-right cheek, both selected with the face.

All source frames were inspected in original grids and five-choice sheets.
Comparison with the translated standing pose distinguishes moving hair from
the exposed scratching wrist. The material test checks every pixel independently
of profile seeds. It fixes 178 literal landmarks and per-frame counts, plus
transparency, metadata, unchanged profile roles and prior output bytes against
slice 156. It explicitly preserves moving hair edges, the red mouth and eye
details while checking the wrist, raised hand and yawn cheek shadow.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Scratch head start South | 1 | 63 |
| Scratch head loop South | 2 | 55, 55 |
| Scratch head end South | 1 | 63 |
| Yawn start South | 3 | 58, 47, 43 |
| Yawn loop South | 1 | 43 |
| Yawn end South | 3 | 41, 50, 50 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. The
scratching loop uses two 0.2-second frames; yawning start and end use three
0.1-second frames each. Single-frame strips have no explicit duration. Both
cycles use native complex start/loop/end sequences and are not seated. Their
original pause behavior and cycle configuration remain unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-157-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-157-review/index.html).

The summary samples scratch head loop frame 1, yawn start frame 2 and yawn loop
frame 1. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full
review includes every source frame, Vanilla left and Debug Blue right, with
full views and enlarged details. Images, exports and packages remain local
and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 369 regular tests and the release
build passed through the Nix shell; 439 opt-in tests were ignored. The selected
local suite passed 229 tests, including every previous Dell material test,
the new special-animation test, historical character checks and three GML
runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted face pixels, the scratching wrist or an exposed ankle, or selected hair.
Each generated successfully and failed the intended pixel assertion. All six
metadata sidecars matched independent archive reads.

All 244 standalone Dell variants passed strict recipe validation; 488 final
PNG/metadata files match the author candidate. All 18,868 combined variants
validated. The 550 prior Dell PNG/metadata files and 46,735 other-character
files match slice 156 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All eight Chromium gallery pages loaded their local images and links without
overflow. Eleven cases include twenty-two full-view bindings, enlarged details
and fifteen summary bindings; 2,335,008 rendered pixels match their source
crops. Full views include every nontransparent pixel. The complete review is
262,193 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-157-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`58b30d5136bd598cd13f399b18804c626d3da4f76576ab550ce244c8800a9375`.
The archive's native animator and NPC animate function passed 55 frame/palette
observations and forty complex transitions under Fabricator. Palette binding,
frame phase, seated status, wrapper idempotence, cycle counters and Spring
portrait synchronization were preserved. Engine services are simulated and
cycles selected directly. Natural gameplay dispatch, speaking/background pause
transitions and live rendering were not exercised. The desktop launcher and
installed game were not changed.

Evidence uses `tmp/slice-157-`; author records use
`tmp/dell-spring-specials-author-`. Fresh exports are in
`extracted/dell-spring-specials-study`, reached through
`extracted/test-corpus/dell`. The fixed baseline is
`generated/slice-156-build/characters/dell`. The evidence report summarizes
results; the scope check verifies nine expected paths, an empty Git index,
unchanged historical pixel tests and ignored game-derived artifacts.

## Remaining work

The archive inventory leaves four Spring, 24 Summer, 21 Autumn and 21 Winter
world strips. Next: Spring stretching and swinging a stick (four strips),
completing that seasonal folder. Natural gameplay dispatch, speaking/background
pause transitions and live rendering remain untested.
