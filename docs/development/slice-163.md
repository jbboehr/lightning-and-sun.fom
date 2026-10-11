# Completing Dell's Summer sprites

Following `28dc4f1`, this slice adds Dell's Summer head scratching (South
start/loop/end) and swinging a stick (East and native West mirroring).
Eleven source frames give eighteen review cases. The user approved the artwork on 2026-10-11.
Local outputs use `generated/review/` and `generated/build/`.

All 24 Summer world strips are now covered. Dell has 89 sources: 32 portraits
and 57 world strips, including all 33 Spring strips. The 36-character collection
has 4,745 sources and 18,980 variants. Y retains Vanilla, Debug Blue, Hayden, Ryis
and Seridia choices with portrait/world synchronization. This slice adds no
runtime code or color roles. All 85 previous regions, groups, target mappings
and collection sets remain unchanged. Historical pixel tests retain their
expectations; only the central Dell coverage total grows.

## Source and materials

The game mount was empty. Source is the preserved original archive at
`tmp/slice-162-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 85 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across eleven
frames, 342 component seeds select 584 pixels. Faces, eyelids, exposed arms,
moving hands and legs follow the palette. Blonde hair, eyebrow edges, lashes,
ribbon, eye details, blue shirt, brown overalls, pale rolled cuffs, red shoes,
stick and motion trail stay original. The masks follow the swing's displaced
head and extended wrists. No connected hair exceptions are needed for this batch.

All source frames were inspected in five-choice sheets, with original pixel
grids used to distinguish matching skin and hair colors. The material test
checks every pixel independently of profile seeds. It fixes 197 literal
landmarks and per-frame counts, plus transparency, metadata, unchanged profile
roles and prior output bytes against slice 162. The stick's `#A8571C` also
appears in Dell's portrait color roles; it remains unchanged in these world
sprites because only the reviewed skin groups are selected.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Scratch head start South | 1 | 68 |
| Scratch head loop South | 2 | 59, 62 |
| Scratch head end South | 1 | 68 |
| Swing stick East | 7 | 47, 39, 49, 37, 47, 54, 54 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. The scratch
loop has scalar duration `0.2`; start/end single frames have no explicit duration.
Swing durations are `[0.1, 0.1, 0.5, 0.075, 0.1, 0.075, 0.5]`. Scratching uses a
complex start/loop/end cycle; swinging uses a linear cycle. Neither cycle has
a final-frame hold. The source configuration retains speaking-to-idle pauses
and scratch background-to-idle pauses; these transitions were not exercised.
Original cycle configuration remains unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-163-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-163-review/index.html).

The summary samples scratch loop frame 1 and stick swing frames 4 and 6.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 375 regular tests and the release
build passed through the Nix shell; 445 opt-in tests were ignored. The selected
local suite passed 235 tests, including every previous Dell material test,
the new scratching/stick test, historical character checks and three GML runtime
tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the four registry additions. Four valid mask mutations
omitted face pixels, the scratching hand or exposed leg pixels, or selected hair.
Each generated successfully and failed the intended pixel assertion. All four
metadata sidecars matched independent archive reads.

All 356 standalone Dell variants passed strict recipe validation; 712 final
PNG/metadata files match the author candidate. All 18,980 combined variants
validated. The 850 prior Dell PNG/metadata files and 46,735 other-character
files match slice 162 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All nine Chromium gallery pages loaded their local images and links without
overflow. Eighteen cases include thirty-six full-view bindings, enlarged
details and fifteen summary bindings; 4,044,456 rendered pixels match their
source crops. Full views include every nontransparent pixel, including the
stick and trail. The complete review is 371,445 bytes; its short alias loads
successfully.

The isolated MOMI installation passed in `tmp/slice-163-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`c6ca71a8c90801052fbf5055e48fb5acb20327fd1a5e6de3541926c83e664670`.
The archive's native animator and NPC animate function passed ninety frame/palette
observations, ten linear completions and twenty complex-cycle transitions under
Fabricator. Palette binding, frame phase, West mirroring, standing status, wrapper
idempotence, cycle counters and Summer portrait synchronization were preserved.
Engine services are simulated and cycles selected directly. Natural gameplay
dispatch, speaking/background pause transitions and live rendering were not
exercised. The desktop launcher and installed game were not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-163-`; author records use
`tmp/dell-summer-finish-author-`. Fresh exports are in
`extracted/dell-summer-finish-study`, reached through `extracted/test-corpus/dell`.
The fixed baseline is `generated/slice-162-build/characters/dell`.

## Remaining work

Spring and Summer are complete for this archive. The remaining Dell world
inventory is 21 Autumn and 21 Winter strips. Next: Dell Autumn standing and
walking (six strips).
Natural gameplay dispatch, speaking/background pause transitions and live
rendering remain untested.
