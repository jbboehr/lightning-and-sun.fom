# Dell Winter general actions

Following `db3c165`, this slice adds Dell's Winter general actions in South,
North and East, with native West mirroring. Twenty-one source frames give
twenty-eight review cases. The user approved the artwork on 2026-10-12.
Local outputs use `generated/review/` and `generated/build/`.

Dell has 130 sources: 32 portraits and 98 world strips. All 33 Spring, 24 Summer
and 21 Autumn strips remain covered, alongside twenty of 21 Winter strips. The
36-character collection has 4,786 sources and 19,144 variants. Y retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 127 previous
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Dell
coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-171-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 127 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across twenty-one
frames, 268 component seeds select 445 pixels. Faces and moving hands follow the
palette. Blonde hair, eyebrow edges, eye details, blue coat, orange shirt and
cuffs, trousers and brown boots stay original. The Winter outfit covers arms,
legs and ankles; the coat also hides the raised hand in the North frames.
Some East and South poses expose fewer hand pixels than Autumn. The orange
cuffs are distinct from those hand pixels; no connected hair exceptions are
needed.

All source frames were inspected in five-choice sheets. Original pixel grids
separate matching skin and hair colors. The material test checks every pixel
independently of profile seeds, using reviewed face rows and hand bounds. It
also checks 398 distinct literal material landmarks and per-frame counts,
transparency, metadata, unchanged profile roles and prior output bytes against
slice 171.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Action East | 7 | 28, 27, 30, 27, 30, 28, 29 |
| Action North | 7 | 2, 2, 2, 2, 2, 2, 4 |
| Action South | 7 | 32, 32, 34, 32, 34, 32, 34 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Durations
remain `[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`. The cycle remains linear with
South as default and native West mirroring of East. It retains
`last_frame_hold = [240, 360]` and `on_pause_speaking = "idle"`; it is not seated.

## Offline review

- [Five-choice sample summary](../../../generated/slice-172-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-172-review/index.html).

The summary samples Action East frame 2, Action North frame 7 and Action South
frame 3. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 384 regular tests and the
release build passed through the Nix shell; 454 opt-in tests were ignored.
The selected local suite passed 244 tests, including every previous Dell
material test, the new Winter general-action test, historical character checks
and three GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted face pixels, the moving East hand or the final North right hand, or
selected blonde hair. Each generated successfully and failed the intended pixel
assertion. All three metadata sidecars matched independent archive reads.

All 520 standalone Dell variants passed strict recipe validation; 1,040
final PNG/metadata files match the author candidate. All 19,144 combined variants
validated. The 1,270 prior Dell PNG/metadata files and 46,735 other-character
files match slice 171 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty-eight cases include fifty-six full-view bindings, enlarged details and
fifteen summary bindings; 5,966,376 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
490,188 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-172-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`1fb5e91ae01c10bd91f1a07efebe54caa9edde5922e6a11301f67bc4eedf274b`.
The archive's native animator and NPC animate function passed 140 frame/palette
observations, twenty linear completions and forty final-frame hold checks under
Fabricator. Palette binding, frame phase, West mirroring, unseated status,
wrapper idempotence, cycle counters and Winter portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch, speaking-pause behavior and live rendering were not exercised.
The desktop launcher and installed game were not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-172-`; author records use
`tmp/dell-winter-general-author-`. Fresh exports are in
`extracted/dell-winter-general-study`, reached through `extracted/test-corpus/dell`.
The fixed baseline is `generated/slice-171-build/characters/dell`.

## Remaining work

Spring, Summer and Autumn are complete for this archive. The remaining Dell
world inventory is one Winter strip. Next: Winter swinging a stick, completing
her Winter folder and all four seasonal sprite sets.
