# Elsie Spring eating and drinking

Following `7d566ba`, this slice adds Elsie's Spring eating and drinking in
East, North and South, plus native West mirroring. Twenty-two source frames
give thirty review cases. The user approved the artwork on 2026-10-13. Local outputs use
`generated/review/` and `generated/build/`.

Elsie has 53 sources: 36 portraits and seventeen world strips. The 36-character
collection has 4,804 sources and 19,216 variants. Z retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This slice
adds no runtime code or color roles. All 47 prior regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests retain
their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-175-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. The mounted
game directory was empty. All 47 prior Elsie PNG pins match fresh exports.
No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across twenty-two frames, 531 component seeds select 939 pixels. Forehead,
face, closed eyelids, neckline and moving hands follow the palette. Gray hair,
eyebrows, black lashes, eye details, mouth interiors, cosmetics, earrings, pink
gown and shoes stay original. The four world shades occur only in skin in
these six pinned sources. North views expose small hand regions; the open-mouth
eating poses retain their dark red mouth interiors.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked at mouth, eyelid, hand and clothing boundaries. The material test
checks every pixel independently of component seeds, plus 376 distinct literal
material landmarks, per-frame counts, transparency, metadata, unchanged profile
roles and prior output bytes against slice 175.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Eat East | 5 | 44, 56, 47, 53, 46 |
| Eat North | 3 | 8, 4, 8 |
| Eat South | 5 | 60, 66, 55, 77, 59 |
| Drink East | 3 | 45, 51, 45 |
| Drink North | 3 | 8, 4, 8 |
| Drink South | 3 | 61, 73, 61 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54.
Eating East/South retains durations `[0.125, 0.15, 0.175, 0.125, 0.6]`;
North eating and all drinking strips retain scalar duration `1.0`. Both cycles
remain linear with South as default, native West mirroring, `is_seated = true`
and `last_frame_hold = [240, 360]`. Neither cycle configures a speaking-pause
transition.

## Offline review

- [Five-choice sample summary](../../../generated/slice-176-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-176-review/index.html).

The summary samples Eat South frame 3, Drink East frame 2 and Eat North frame 2.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 388 regular tests and the
release build passed through the Nix shell; 458 opt-in tests were ignored.
The selected local suite passed 249 tests, including the original Elsie portrait,
world and blinking/sitting tests, the new eating/drinking material test,
historical character checks and three GML runtime tests. Historical pixel-test
expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the eating face, neckline, South hand or North right hand. Each generated
successfully and failed the intended pixel assertion. All six metadata sidecars
matched independent archive reads.

All 212 standalone Elsie variants passed strict recipe validation; 424 final
PNG/metadata files match the author candidate. All 19,216 combined variants
validated. The 470 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 175 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All thirteen Chromium gallery pages loaded local images and links without
overflow. Thirty cases include sixty full-view bindings, enlarged details and
fifteen summary bindings; 5,559,840 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
560,195 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-176-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`ccc000813a70113dcd5b2a17d6d8a41d3233a09f26b78278ac494d5bf8e7a764`.
The archive's native animator and NPC animate function passed 150 frame/palette
observations, 40 linear completions and 80 final-frame-hold checks under
Fabricator. Palette binding, frame phase, West mirroring, seated status, wrapper
idempotence, cycle counters and Spring portrait synchronization were preserved.
Engine services are simulated and cycles selected directly. Natural gameplay
dispatch and live rendering were not exercised. The desktop launcher and
installed game were not changed.

Evidence uses `tmp/slice-176-`; author records use
`tmp/elsie-spring-food-author-`. Fresh exports are in
`extracted/elsie-spring-food-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-175-build/characters/elsie`. Scope verification checks nine
expected paths, an empty Git index, unchanged historical pixel tests and ignored
game-derived artifacts.

## Remaining work

Elsie's remaining world inventory has 18 Spring, 32 Summer, 32 Autumn and
29 Winter strips. Next: Spring general actions, three strips. The user approved
this slice; live-game verification remains pending.
