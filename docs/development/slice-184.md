# Elsie Summer eating and drinking

Following `56f5d90`, this slice adds Elsie's Summer eating and drinking in
East, North and South, plus native West mirroring. Six strips contain twenty-two
source frames and eight mirrored views, giving thirty review cases. User art
approval was received on 2026-10-13. Local outputs use `generated/review/` and
`generated/build/`.

Elsie has 88 sources: 36 portraits and fifty-two world strips. The 36-character
collection has 4,839 sources and 19,356 variants. Z retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This slice
adds no runtime code or color roles. All 82 prior regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests retain
their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-183-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 82 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across twenty-two frames, 575 component seeds select 1,031 pixels. Forehead,
face, closed eyelids, neckline, exposed arms, moving hands and ankles follow
the palette. Gray hair, eyebrows, black lashes, eye details, mouth interiors,
turquoise earrings, gold scarf, blue shawl, dress and shoes retain their
original colors. The four world shades occur only in skin in these six pinned
sources. North views expose the neckline and arms/hands; the open-mouth eating
poses retain their dark red mouth interiors.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the open mouths, closed eyelids, neckline, moving hands,
clothing and ankles. The material test checks every pixel independently of
component seeds, plus 429 distinct literal material landmarks, per-frame counts,
transparency, metadata, unchanged profile roles and prior output bytes against
slice 183.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Eat East | 5 | 51, 62, 52, 60, 53 |
| Eat North | 3 | 18, 11, 18 |
| Eat South | 5 | 58, 67, 57, 74, 55 |
| Drink East | 3 | 52, 58, 52 |
| Drink North | 3 | 18, 11, 18 |
| Drink South | 3 | 57, 72, 57 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54.
Eating East/South retains durations `[0.125, 0.15, 0.175, 0.125, 0.6]`;
North eating and all drinking strips retain scalar duration `1.0`. Both cycles
remain linear with South as default, native West mirroring, `is_seated = true`
and `last_frame_hold = [240, 360]`. Neither cycle configures speaking or
background pause settings. The installed Elsie NPC configuration matches the
source byte for byte.

## Offline review

- [Five-choice sample summary](../../../generated/slice-184-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-184-review/index.html).

The summary samples Eat South frame 3, Drink East frame 2 and Eat North frame 2.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 396 regular tests and the
release build passed through the Nix shell; 466 opt-in tests were ignored.
The selected local suite passed 257 tests, including all earlier Elsie
material checks, the new eating/drinking material test, historical character
checks and three GML runtime tests. Historical pixel-test expectations are
unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the open-mouth East eating face, South eating neckline, South eating
hand or North drinking viewer-right hand. Each generated successfully and
failed the intended pixel assertion. All six metadata sidecars matched
independent archive reads.

All 352 standalone Elsie variants passed strict recipe validation; 704 final
PNG/metadata files match the author candidate. All 19,356 combined variants
validated. The 820 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 183 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All thirteen Chromium gallery pages loaded local images and links without
overflow. Thirty cases include sixty full-view bindings, enlarged details and
fifteen summary bindings; 5,421,600 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
569,109 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-184-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`36df26c58a431b7989d681d8c410470620e003db5be0c78e90df4abf9ddb8506`.
The archive's native animator and NPC animate function passed 150 frame/palette
observations, forty linear completions and eighty final-frame-hold checks under
Fabricator. Palette binding, frame phase, seated status, West mirroring,
wrapper idempotence, cycle counters and Summer portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch and live rendering were not exercised. The desktop launcher
and installed game were not changed.

Evidence uses `tmp/slice-184-`; author records use
`tmp/elsie-summer-food-author-`. Fresh exports are in
`extracted/elsie-summer-food-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-183-build/characters/elsie`. Scope verification checks nine
expected paths, an empty Git index, unchanged historical pixel tests and ignored
game-derived artifacts.

## Remaining work

Elsie's remaining world inventory has 15 Summer, 32 Autumn and 29 Winter
strips. Next: Summer general actions, three strips. Live-game verification
remains pending for this slice.
