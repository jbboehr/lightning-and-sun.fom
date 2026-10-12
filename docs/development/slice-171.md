# Dell Winter eating and drinking

Following `3eead4b`, this slice adds Dell's Winter eating and drinking in South,
North and East, with native West mirroring. Twenty-two source frames give thirty
review cases. The user approved the artwork on 2026-10-12.
Local outputs use `generated/review/` and `generated/build/`.

Dell has 127 sources: 32 portraits and 95 world strips. All 33 Spring, 24 Summer
and 21 Autumn strips remain covered, alongside seventeen of 21 Winter strips. The
36-character collection has 4,783 sources and 19,132 variants. Y retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 121 previous
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Dell
coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-170-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 121 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across twenty-two
frames, 344 component seeds select 656 pixels. Moving faces, eyelids and exposed
hands follow the palette. Connected dark East eye edges include five temple
pixels in drinking frame 2 and three in eating frames 2 and 4. Winter eating
frame 3 has a different connected dark edge: `[42,31]`, `[42,32]`, `[43,32]`,
`[44,32]` and `[44,33]`. The adjacent lighter hair at `[42,33]` and `[43,33]`
stays original. Two dark chin corners, `[36,38]` and `[43,38]`, also follow the
skin in South eating frame 3. Other blonde hair, lashes, eye details, mouth
interiors, blue coat, orange shirt and cuffs, trousers and brown boots stay
original. Legs and ankles are covered. Raised hands are followed separately
from the displaced faces.

All source frames were inspected in five-choice sheets, with original pixel
grids used to distinguish matching skin and hair colors. The material test
checks every pixel independently of profile seeds, using reviewed face and
raised-hand bounds and explicit connected-eye exceptions. It fixes 353 distinct
material landmarks and per-frame counts, plus transparency, metadata,
unchanged profile roles and prior output bytes against slice 170.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Drink East | 3 | 32, 43, 32 |
| Drink North | 3 | 7, 5, 7 |
| Drink South | 3 | 41, 49, 41 |
| Eat East | 5 | 30, 36, 29, 38, 30 |
| Eat North | 3 | 7, 5, 7 |
| Eat South | 5 | 39, 45, 39, 54, 40 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Drinking
and North eating retain scalar duration `1.0`. East/South eating retain durations
`[0.125, 0.15, 0.175, 0.125, 0.6]`. Both cycles are linear with South as default
and native West mirroring of East. Both retain `is_seated = true` and
`last_frame_hold = [240, 360]`. Neither has explicit pause transitions in its
source configuration.

## Offline review

- [Five-choice sample summary](../../../generated/slice-171-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-171-review/index.html).

The summary samples Drink East frame 2, Eat East frame 3 and Eat South frame 3.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 383 regular tests and the release
build passed through the Nix shell; 453 opt-in tests were ignored. The selected
local suite passed 243 tests, including every previous Dell material test,
the new Winter eating/drinking test, historical character checks and three
GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the open-mouth face pixels, the raised drinking hand or the resting
hand, or selected blonde hair. Each generated successfully and failed the
intended pixel assertion. All six metadata sidecars matched independent
archive reads.

All 508 standalone Dell variants passed strict recipe validation; 1,016 final
PNG/metadata files match the author candidate. All 19,132 combined variants
validated. The 1,210 prior Dell PNG/metadata files and 46,735 other-character
files match slice 170 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All thirteen Chromium gallery pages loaded their local images and links without
overflow. Thirty cases include sixty full-view bindings, enlarged details and
fifteen summary bindings; 6,350,760 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
533,287 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-171-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`7b189a756d54fafc7f9621d978f63f2b48262b627e7994c34aa5075a359f43b4`.
The archive's native animator and NPC animate function passed 150 frame/palette
observations, forty linear completions and eighty final-frame hold checks under
Fabricator. Palette binding, frame phase, West mirroring, seated status,
wrapper idempotence, cycle counters and Winter portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch and live rendering were not exercised. The desktop launcher
and installed game were not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-171-`; author records use
`tmp/dell-winter-food-author-`. Fresh exports are in
`extracted/dell-winter-food-study`, reached through `extracted/test-corpus/dell`.
The fixed baseline is `generated/slice-170-build/characters/dell`.

## Remaining work

Spring, Summer and Autumn are complete for this archive. The remaining Dell
world inventory is four Winter strips. Next: Winter general actions (three
strips), followed by swinging a stick to complete Winter.
