# Dell Winter standing and walking

Following `8a7d025`, this slice adds Dell's Winter standing and walking in
South, North and East, with native West mirroring. Fifteen source frames give
twenty review cases. The user approved the artwork on 2026-10-12.
Local outputs use `generated/review/` and `generated/build/`.

Dell has 116 sources: 32 portraits and 84 world strips. All 33 Spring, 24 Summer
and 21 Autumn strips remain covered, alongside six of 21 Winter strips. The
36-character collection has 4,772 sources and 19,088 variants. Y retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 110 previous
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Dell
coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-168-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 110 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across fifteen
frames, 207 component seeds select 332 pixels. Faces and exposed hands follow
the palette. Blonde hair, eyebrow edges, eye details, blue coat, orange shirt
and cuffs, trousers and brown boots stay original. The coat covers the arms and
the trousers cover the legs. Each frame has three or four exposed hand pixels.
The orange cuffs are distinct from those hand pixels. No connected hair
exceptions are needed for this batch.

All source frames were inspected in five-choice sheets. Original pixel grids
separate matching skin and hair colors. The material test checks every pixel
independently of profile seeds, using reviewed face rows and literal hand
coordinates. It fixes 276 distinct material landmarks and per-frame counts,
plus transparency, metadata, unchanged profile roles and prior output bytes
against slice 168. Clothing uses `#A8571C`, also present in Dell's portrait
roles; these world masks preserve it by selecting only reviewed skin groups.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Idle East | 1 | 29 |
| Idle North | 1 | 4 |
| Idle South | 1 | 34 |
| Walk East | 4 | 29, 30, 29, 29 |
| Walk North | 4 | 4, 3, 4, 3 |
| Walk South | 4 | 34, 33, 34, 33 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Idle single
frames have no explicit duration; walking retains scalar duration `0.15`. Both
cycles are linear with South as default and native West mirroring of East.
Neither cycle has a final-frame hold. Native speaking-turn flags and walk
speaking/background-to-idle pauses remain unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-169-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-169-review/index.html).

The summary samples Idle South frame 1, Walk East frame 2 and Walk North frame 2.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 381 regular tests and the
release build passed through the Nix shell; 451 opt-in tests were ignored.
The selected local suite passed 241 tests, including every previous Dell
material test, the new Winter standing/walking test, historical character checks
and three GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted face pixels, the left hand or the right hand, or selected blonde hair.
Each generated successfully and failed the intended pixel assertion. All six
metadata sidecars matched independent archive reads.

All 464 standalone Dell variants passed strict recipe validation; 928
final PNG/metadata files match the author candidate. All 19,088 combined variants
validated. The 1,100 prior Dell PNG/metadata files and 46,735 other-character
files match slice 168 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty cases include forty full-view bindings, enlarged details and fifteen
summary bindings; 4,428,840 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is
389,077 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-169-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`9a9c811a95de52ad256c4923bcbbce70cef48539385263481805a4d56c783d73`.
The archive's native animator and NPC animate function passed 100 frame/palette
observations and forty linear completions under Fabricator. Palette binding,
frame phase, West mirroring, unseated status, wrapper idempotence, cycle counters
and Winter portrait synchronization were preserved. Engine services are simulated
and cycles selected directly. Natural gameplay dispatch, speaking/background
pause transitions and live rendering were not exercised. The desktop launcher
and installed game were not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-169-`; author records use
`tmp/dell-winter-world-author-`. Fresh exports are in
`extracted/dell-winter-world-study`, reached through `extracted/test-corpus/dell`.
The fixed baseline is `generated/slice-168-build/characters/dell`.

## Remaining work

Spring, Summer and Autumn are complete for this archive. The remaining Dell
world inventory is fifteen Winter strips. Next: Winter blinking and sitting,
five strips.
