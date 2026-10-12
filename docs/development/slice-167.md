# Dell Autumn general actions

Following `657c75c`, this slice adds Dell's Autumn general actions in South,
North and East, with native West mirroring. Twenty-one source frames give
twenty-eight review cases. The user approved the artwork on 2026-10-12.
Local outputs use `generated/review/` and `generated/build/`.

Dell has 109 sources: 32 portraits and 77 world strips. All 33 Spring and 24
Summer strips remain covered, alongside twenty of 21 Autumn strips. The
36-character collection has 4,765 sources and 19,060 variants. Y retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 106 previous
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Dell
coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-166-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 106 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across twenty-one
frames, 275 component seeds select 456 pixels. Faces and moving hands follow the
palette. Blonde hair, eyebrow edges, ribbon, eye details, blue coat, red outfit
and brown boots stay original. The Autumn outfit covers arms, legs and ankles;
the coat also hides the raised hand in the North frames.

All source frames were inspected in five-choice sheets. Original pixel grids
separate matching skin and hair colors. The material test checks every pixel
independently of profile seeds, using reviewed face rows and hand bounds. It
also checks literal material landmarks and per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 166.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Action East | 7 | 28, 29, 30, 29, 30, 28, 30 |
| Action North | 7 | 2, 2, 2, 2, 2, 2, 4 |
| Action South | 7 | 34, 33, 34, 33, 34, 34, 34 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Durations
remain `[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`. The cycle remains linear with
South as default and native West mirroring of East. It retains
`last_frame_hold = [240, 360]` and `on_pause_speaking = "idle"`; it is not seated.

## Offline review

- [Five-choice sample summary](../../../generated/slice-167-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-167-review/index.html).

The summary samples Action East frame 2, Action North frame 7 and Action South
frame 3. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 379 regular tests and the
release build passed through the Nix shell; 449 opt-in tests were ignored.
The selected local suite passed 239 tests, including every previous Dell
material test, the new Autumn general-action test, historical character checks
and three GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted face pixels, the moving East hand or the final North right hand, or
selected blonde hair. Each generated successfully and failed the intended pixel
assertion. All three metadata sidecars matched independent archive reads.

All 436 standalone Dell variants passed strict recipe validation; 872
final PNG/metadata files match the author candidate. All 19,060 combined variants
validated. The 1,060 prior Dell PNG/metadata files and 46,735 other-character
files match slice 166 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty-eight cases include fifty-six full-view bindings, enlarged details and
fifteen summary bindings; 5,966,376 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
488,617 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-167-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`915964866aabd9042f5175f70a2f1c06f6f7480f07a6e1ee53ca02810d5a42d2`.
The archive's native animator and NPC animate function passed 140 frame/palette
observations, twenty linear completions and forty final-frame hold checks under
Fabricator. Palette binding, frame phase, West mirroring, unseated status,
wrapper idempotence, cycle counters and Autumn portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch, speaking-pause behavior and live rendering were not exercised.
The desktop launcher and installed game were not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-167-`; author records use
`tmp/dell-autumn-general-author-`. Fresh exports are in
`extracted/dell-autumn-general-study`, reached through `extracted/test-corpus/dell`.
The fixed baseline is `generated/slice-166-build/characters/dell`.

## Remaining work

Spring and Summer are complete for this archive. The remaining Dell world
inventory is one Autumn and 21 Winter strips. Next: Dell Autumn swinging a
stick, completing Autumn, then Winter standing and walking.
