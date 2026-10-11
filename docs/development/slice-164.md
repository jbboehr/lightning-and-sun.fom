# Dell Autumn standing and walking

Following `42704ab`, this slice adds Dell's Autumn standing and walking in
South, North and East, with native West mirroring. Fifteen source frames give
twenty review cases. The user approved the artwork on 2026-10-11.
Local outputs use `generated/review/` and `generated/build/`.

Dell has 95 sources: 32 portraits and 63 world strips. All 33 Spring and 24
Summer strips remain covered, alongside six of 21 Autumn strips. The
36-character collection has 4,751 sources and 19,004 variants. Y retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 89 previous
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Dell
coverage total grows.

## Source and materials

The game mount was empty. Source is the preserved original archive at
`tmp/slice-163-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 89 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across fifteen
frames, 210 component seeds select 335 pixels. Faces and exposed hands follow
the palette. Blonde hair, eyebrow edges, ribbon, eye details, blue coat, red
outfit and brown boots stay original. Unlike the Summer outfit, the coat covers
the arms and the trousers cover the legs. Each frame has three or four exposed
hand pixels. No connected hair exceptions are needed for this batch.

All source frames were inspected in five-choice sheets, with original pixel
grids used to distinguish matching skin and hair colors. The material test
checks every pixel independently of profile seeds, using reviewed face rows and
literal hand coordinates. It fixes 258 material landmarks and per-frame counts,
plus transparency, metadata, unchanged profile roles and prior output bytes
against slice 163.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Idle East | 1 | 30 |
| Idle North | 1 | 4 |
| Idle South | 1 | 34 |
| Walk East | 4 | 30, 30, 30, 29 |
| Walk North | 4 | 4, 3, 4, 3 |
| Walk South | 4 | 34, 33, 34, 33 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Idle
single frames have no explicit duration; walking retains scalar duration
`0.15`. Both cycles are linear, with South as default and native West mirroring
of East. Neither cycle has a final-frame hold. Native speaking-turn flags and
walk speaking/background-to-idle pauses remain unchanged; these pause
transitions were not exercised.

## Offline review

- [Five-choice sample summary](../../../generated/slice-164-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-164-review/index.html).

The summary samples Idle South frame 1, Walk East frame 2 and Walk North frame 2.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 376 regular tests and the release
build passed through the Nix shell; 446 opt-in tests were ignored. The selected
local suite passed 236 tests, including every previous Dell material test,
the new Autumn standing/walking test, historical character checks and three
GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted face pixels, the left hand or the right hand, or selected blonde hair.
Each generated successfully and failed the intended pixel assertion. All six
metadata sidecars matched independent archive reads.

All 380 standalone Dell variants passed strict recipe validation; 760 final
PNG/metadata files match the author candidate. All 19,004 combined variants
validated. The 890 prior Dell PNG/metadata files and 46,735 other-character
files match slice 163 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All ten Chromium gallery pages loaded their local images and links without
overflow. Twenty cases include forty full-view bindings, enlarged details and
fifteen summary bindings; 4,428,840 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
386,797 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-164-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`3d524edeb9ab60b19b6f8b0777281287b5636787114162930e3202e395bc570c`.
The archive's native animator and NPC animate function passed 100 frame/palette
observations and forty linear completions under Fabricator. Palette binding,
frame phase, West mirroring, standing status, wrapper idempotence, cycle counters
and Autumn portrait synchronization were preserved. Engine services are
simulated and cycles selected directly. Natural gameplay dispatch,
speaking/background pause transitions and live rendering were not exercised.
The desktop launcher and installed game were not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-164-`; author records use
`tmp/dell-autumn-world-author-`. Fresh exports are in
`extracted/dell-autumn-world-study`, reached through `extracted/test-corpus/dell`.
The fixed baseline is `generated/slice-163-build/characters/dell`.

## Remaining work

Spring and Summer are complete for this archive. The remaining Dell world
inventory is 15 Autumn and 21 Winter strips. Next: Dell Autumn blinking and
sitting (five strips).
