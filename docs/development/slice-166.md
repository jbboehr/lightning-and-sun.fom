# Dell Autumn eating and drinking

Following `9d7096b`, this slice adds Dell's Autumn eating and drinking in South,
North and East, with native West mirroring. Twenty-two source frames give thirty
review cases. The user approved the artwork on 2026-10-11.
Local outputs use `generated/review/` and `generated/build/`.

Dell has 106 sources: 32 portraits and 74 world strips. All 33 Spring and 24
Summer strips remain covered, alongside seventeen of 21 Autumn strips. The
36-character collection has 4,762 sources and 19,048 variants. Y retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 100 previous
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Dell
coverage total grows.

## Source and materials

The game mount was empty. Source is the preserved original archive at
`tmp/slice-165-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 100 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across twenty-two
frames, 341 component seeds select 641 pixels. Moving faces, eyelids and exposed
hands follow the palette. The connected dark East eye edges use the same
geometry as Spring: five temple pixels in drinking frame 2 and three in eating
frames 2–4. Other blonde hair, lashes, ribbon, eye details, mouth interiors,
blue coat, red outfit and brown boots stay original. The Autumn outfit covers
legs and ankles. Raised hands are followed separately from the displaced faces.

All source frames were inspected in five-choice sheets, with original pixel
grids used to distinguish matching skin and hair colors. The material test
checks every pixel independently of profile seeds, using reviewed face and
raised-hand bounds and explicit connected-eye exceptions. It fixes 337 distinct
material landmarks and per-frame counts, plus transparency, metadata,
unchanged profile roles and prior output bytes against slice 165.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Drink East | 3 | 32, 42, 32 |
| Drink North | 3 | 7, 5, 7 |
| Drink South | 3 | 40, 48, 40 |
| Eat East | 5 | 30, 35, 27, 38, 30 |
| Eat North | 3 | 7, 5, 7 |
| Eat South | 5 | 38, 44, 36, 53, 38 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Drinking
and North eating retain scalar duration `1.0`. East/South eating retain durations
`[0.125, 0.15, 0.175, 0.125, 0.6]`. Both cycles are linear with South as default
and native West mirroring of East. Both retain `is_seated = true` and
`last_frame_hold = [240, 360]`. Neither has explicit pause transitions in its
source configuration.

## Offline review

- [Five-choice sample summary](../../../generated/slice-166-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-166-review/index.html).

The summary samples Drink East frame 2, Eat East frame 3 and Eat South frame 3.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 378 regular tests and the release
build passed through the Nix shell; 448 opt-in tests were ignored. The selected
local suite passed 238 tests, including every previous Dell material test,
the new Autumn eating/drinking test, historical character checks and three
GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the open-mouth face pixels, the raised drinking hand or the resting
hand, or selected blonde hair. Each generated successfully and failed the
intended pixel assertion. All six metadata sidecars matched independent
archive reads.

All 424 standalone Dell variants passed strict recipe validation; 848 final
PNG/metadata files match the author candidate. All 19,048 combined variants
validated. The 1,000 prior Dell PNG/metadata files and 46,735 other-character
files match slice 165 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All thirteen Chromium gallery pages loaded their local images and links without
overflow. Thirty cases include sixty full-view bindings, enlarged details and
fifteen summary bindings; 6,350,760 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
530,850 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-166-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`0f335c95a8afffc823000809ad9a64f53fced31fbc5be88643ee624cdc16f7f5`.
The archive's native animator and NPC animate function passed 150 frame/palette
observations, forty linear completions and eighty final-frame hold checks under
Fabricator. Palette binding, frame phase, West mirroring, seated status,
wrapper idempotence, cycle counters and Autumn portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch and live rendering were not exercised. The desktop launcher
and installed game were not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-166-`; author records use
`tmp/dell-autumn-food-author-`. Fresh exports are in
`extracted/dell-autumn-food-study`, reached through `extracted/test-corpus/dell`.
The fixed baseline is `generated/slice-165-build/characters/dell`.

## Remaining work

Spring and Summer are complete for this archive. The remaining Dell world
inventory is four Autumn and 21 Winter strips. Next: Dell Autumn general
actions (three strips), followed by swinging a stick to complete Autumn.
