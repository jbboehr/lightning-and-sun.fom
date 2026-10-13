# Completing Dell's Winter sprites

Following `ede2da6`, this slice adds Dell's Winter stick swing in East with
native West mirroring. Seven source frames give fourteen review cases. The user
approved the artwork on 2026-10-12. Local outputs use `generated/review/` and
`generated/build/`.

All 21 Winter world strips are now covered, alongside all 33 Spring, 24 Summer
and 21 Autumn strips. Dell has 131 sources: 32 portraits and 99 world strips.
The 36-character collection has 4,787 sources and 19,148 variants. Y retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 130 previous
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Dell
coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-172-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 130 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across seven
frames, 134 component seeds select 232 pixels. Faces, closed eyelids and moving
hands follow the palette. Blonde hair, eyebrow edges, eye details, blue coat,
orange shirt and cuffs, trousers, brown boots, stick and motion trail stay
original. The mask follows the displaced head and exposed hands throughout the
swing, including the closed-eye frame. No connected hair exceptions are needed.

All source frames were inspected in five-choice sheets. Original pixel grids
separate matching skin and hair colors. The material test checks every pixel
independently of profile seeds, using reviewed face rows and hand bounds. It
also checks 200 distinct literal material landmarks and per-frame counts,
transparency, metadata, unchanged profile roles and prior output bytes against
slice 172. The stick's `#A8571C` also appears in Dell's portrait roles; it stays
original because these world masks select only reviewed skin groups.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Swing stick East | 7 | 29, 27, 32, 31, 37, 38, 38 |

The strip retains 80×80 frames, the Default atlas and offsets 40/54. Durations
remain `[0.1, 0.1, 0.5, 0.075, 0.1, 0.075, 0.5]`. The cycle remains linear with
East as default and native West mirroring. It has no final-frame hold and retains
`on_pause_speaking = "idle"`; it is not seated.

## Offline review

- [Five-choice sample summary](../../../generated/slice-173-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-173-review/index.html).

The summary samples swing frames 2, 4 and 6. Columns are Vanilla, Debug Blue,
Hayden, Ryis and Seridia. The full review includes every source frame and native
West mirror, Vanilla left and Debug Blue right, with full views and enlarged
details. Images, exports and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 385 regular tests and the
release build passed through the Nix shell; 455 opt-in tests were ignored.
The selected local suite passed 245 tests, including every previous Dell
material test, the new Winter stick-swing test, historical character checks
and three GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the registry addition. Four valid mask mutations
omitted closed-eye face pixels, the extended left hand or the right hand, or
selected blonde hair. Each generated successfully and failed the intended pixel
assertion. The metadata sidecar matched an independent archive read.

All 524 standalone Dell variants passed strict recipe validation; 1,048
final PNG/metadata files match the author candidate. All 19,148 combined variants
validated. The 1,300 prior Dell PNG/metadata files and 46,735 other-character
files match slice 172 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All six Chromium gallery pages loaded local images and links without overflow.
Fourteen cases include twenty-eight full-view bindings, enlarged details and
fifteen summary bindings; 3,275,688 rendered pixels match their source crops.
Full views include every nontransparent pixel, including the stick and trail.
The complete review is 297,436 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-173-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`22744f2107d458a62a4fec1e9cccf494aac08c9e8b318621a77aeb3e102df5f0`.
The archive's native animator and NPC animate function passed seventy frame/palette
observations and ten linear completions under Fabricator. Palette binding, frame
phase, West mirroring, unseated status, wrapper idempotence, cycle counters and
Winter portrait synchronization were preserved. Engine services are simulated
and cycles selected directly. Natural gameplay dispatch, speaking-pause behavior
and live rendering were not exercised. The desktop launcher and installed game
were not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-173-`; author records use
`tmp/dell-winter-finish-author-`. Fresh exports are in
`extracted/dell-winter-finish-study`, reached through `extracted/test-corpus/dell`.
The fixed baseline is `generated/slice-172-build/characters/dell`.

## Remaining work

Dell's Spring, Summer, Autumn and Winter sprites are complete for this archive.
The user approved this slice; live-game verification remains pending. Further
roster coverage can continue after this visual checkpoint.
