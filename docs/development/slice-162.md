# Dell Summer general actions

Following `83418b2`, this slice adds Dell's Summer general actions in North,
South and East, plus native West mirroring. Twenty-one source frames give
twenty-eight review cases. The user approved the artwork on 2026-10-11. Local outputs use
`generated/review/` and `generated/build/`.

Dell has 85 sources: 32 portraits and 53 world strips. The 36-character collection
has 4,741 sources and 18,964 variants. Y retains Vanilla, Debug Blue, Hayden, Ryis
and Seridia choices with portrait/world synchronization. This slice adds no
runtime code or color roles. All 82 previous regions, groups, target mappings
and collection sets remain unchanged. Historical pixel tests retain their
expectations; only the central Dell coverage total grows.

## Source and materials

The game mount was empty. Source is the preserved original archive at
`tmp/slice-161-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 82 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across twenty-one
frames, 522 component seeds select 771 pixels. Faces, necks, exposed arms, moving
hands and legs follow the palette. Blonde hair, eyebrow edges, ribbon, eye
details, blue shirt, brown overalls, pale rolled cuffs and red shoes stay original.
The masks follow the head's one-pixel shifts and the arm positions. No connected
hair exceptions are needed for this batch.

All source frames were inspected in five-choice sheets, with original pixel
grids used to distinguish matching skin and hair colors. The material test
checks every pixel independently of profile seeds. It fixes 308 literal
landmarks and per-frame counts, plus transparency, metadata, unchanged profile
roles and prior output bytes against slice 161. In North views, hair covers the
head and neck; exposed arms, hands and legs still change.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Action East | 7 | 37, 41, 43, 41, 43, 37, 47 |
| Action North | 7 | 20, 20, 23, 20, 23, 20, 22 |
| Action South | 7 | 45, 47, 49, 47, 49, 45, 52 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Their
durations are `[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`. The native cycle is
linear, with final-frame holds `[240, 360]`. The source configuration retains
`on_pause_speaking = "idle"`; speaking transitions were not exercised.
Original cycle configuration remains unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-162-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-162-review/index.html).

The summary samples action East frame 2, action North frame 3 and action South
frame 3. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full
review includes every source frame and native West mirror, Vanilla left and
Debug Blue right, with full views and enlarged details. Images, exports and
packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 374 regular tests and the release
build passed through the Nix shell; 444 opt-in tests were ignored. The selected
local suite passed 234 tests, including every previous Dell material test,
the new general-action test, historical character checks and three GML runtime
tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted face pixels, a raised rear-view hand or exposed leg pixels, or selected hair.
Each generated successfully and failed the intended pixel assertion. All three
metadata sidecars matched independent archive reads.

All 340 standalone Dell variants passed strict recipe validation; 680 final
PNG/metadata files match the author candidate. All 18,964 combined variants
validated. The 820 prior Dell PNG/metadata files and 46,735 other-character
files match slice 161 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All ten Chromium gallery pages loaded their local images and links without
overflow. Twenty-eight cases include fifty-six full-view bindings, enlarged
details and fifteen summary bindings; 5,966,376 rendered pixels match their
source crops. Full views include every nontransparent pixel. The complete
review is 477,488 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-162-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`c5a3198a29b66874352cc3a74729550af17e0cfd61fd2fb9e09ab9f23edf731e`.
The archive's native animator and NPC animate function passed 140 frame/palette
observations, twenty linear completions and forty final-frame hold checks under
Fabricator. Palette binding, frame phase, West mirroring, standing status, wrapper
idempotence, cycle counters and Summer portrait synchronization were preserved.
Engine services are simulated and cycles selected directly. Natural gameplay
dispatch, speaking/background pause transitions and live rendering were not
exercised. The desktop launcher and installed game were not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-162-`; author records use
`tmp/dell-summer-general-author-`. Fresh exports are in
`extracted/dell-summer-general-study`, reached through `extracted/test-corpus/dell`.
The fixed baseline is `generated/slice-161-build/characters/dell`.

## Remaining work

The archive inventory leaves 4 Summer, 21 Autumn and 21 Winter world strips.
Spring is complete. Next: Dell Summer head scratching and swinging a stick
(four strips), completing Summer.
Natural gameplay dispatch, speaking/background pause transitions and live
rendering remain untested.
