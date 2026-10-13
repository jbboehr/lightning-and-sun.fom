# Elsie Spring general actions

Following `4e6058f`, this slice adds Elsie's Spring general actions in East,
North and South, plus native West mirroring. Twenty-one source frames give
twenty-eight review cases. The user approved the artwork on 2026-10-13. Local outputs use
`generated/review/` and `generated/build/`.

Elsie has 56 sources: 36 portraits and twenty world strips. The 36-character
collection has 4,807 sources and 19,228 variants. Z retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This slice
adds no runtime code or color roles. All 53 prior regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests retain
their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-176-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 53 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across twenty-one frames, 456 component seeds select 813 pixels. Forehead,
face, neckline and moving hands follow the palette. Gray hair, eyebrows, black
lashes, eye details, cosmetics, earrings, pink gown and shoes stay original.
The four world shades occur only in skin in these three pinned sources.
North views expose small hand regions; pale pink sleeve and skirt highlights
remain distinct from the skin highlights.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked at face, hand, sleeve and clothing boundaries. The material test
checks every pixel independently of component seeds, plus 357 distinct literal
material landmarks, per-frame counts, transparency, metadata, unchanged profile
roles and prior output bytes against slice 176.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Action East | 7 | 53, 46, 46, 46, 46, 53, 53 |
| Action North | 7 | 8, 6, 6, 6, 6, 7, 10 |
| Action South | 7 | 60, 59, 60, 59, 60, 60, 63 |

All strips retain 80×80 frames, the Default atlas, offsets 40/54 and durations
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`. The cycle remains linear and
unseated, with South as default, native West mirroring and
`last_frame_hold = [240, 360]`. Its `on_pause_speaking = "idle"` configuration
is preserved. The installed Elsie NPC configuration matches the source byte
for byte; natural speaking-pause dispatch was not exercised.

## Offline review

- [Five-choice sample summary](../../../generated/slice-177-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-177-review/index.html).

The summary samples Action East frame 2, Action South frame 7 and Action North
frame 7. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full
review includes every source frame and native West mirror, Vanilla left and
Debug Blue right, with full views and enlarged details. Images, exports and
packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 389 regular tests and the
release build passed through the Nix shell; 459 opt-in tests were ignored.
The selected local suite passed 250 tests, including the original Elsie portrait,
world, blinking/sitting and eating/drinking tests, the new general-action
material test, historical character checks and three GML runtime tests.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted the East face, neckline, South left hand or North right hand. Each
generated successfully and failed the intended pixel assertion. All three
metadata sidecars matched independent archive reads.

All 224 standalone Elsie variants passed strict recipe validation; 448 final
PNG/metadata files match the author candidate. All 19,228 combined variants
validated. The 530 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 176 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty-eight cases include fifty-six full-view bindings, enlarged details and
fifteen summary bindings; 5,220,384 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
552,955 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-177-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`8dcd2c19d97195c64dcb5bc59f7d4d62d5fe55ab2352b5704bd6467258c0369a`.
The archive's native animator and NPC animate function passed 140 frame/palette
observations, 20 linear completions and 40 final-frame-hold checks under
Fabricator. Palette binding, frame phase, West mirroring, unseated status,
wrapper idempotence, cycle counters and Spring portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch and live rendering were not exercised. The desktop launcher
and installed game were not changed.

Evidence uses `tmp/slice-177-`; author records use
`tmp/elsie-spring-general-author-`. Fresh exports are in
`extracted/elsie-spring-general-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-176-build/characters/elsie`. Scope verification checks nine
expected paths, an empty Git index, unchanged historical pixel tests and ignored
game-derived artifacts.

## Remaining work

Elsie's remaining world inventory has 15 Spring, 32 Summer, 32 Autumn and
29 Winter strips. Next: Spring shocked reactions, three strips. The user
approved this slice; live-game verification remains pending.
