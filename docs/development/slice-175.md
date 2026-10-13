# Elsie Spring blinking and sitting

Following `55ceb5f`, this slice adds Elsie's Spring blinking in East and South
and sitting in East, North and South, plus native West mirroring. Nine source
frames give thirteen review cases. The user approved the artwork on 2026-10-12. Local outputs
use `generated/review/` and `generated/build/`.

Elsie has 47 sources: 36 portraits and eleven world strips. The 36-character
collection has 4,798 sources and 19,192 variants. Z retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This slice
adds no runtime code or color roles. All 42 prior regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests retain
their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-174-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 42 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across nine frames, 268 component seeds select 503 pixels. Forehead, face,
closed eyelids, neckline and exposed hands follow the palette. Gray hair,
eyebrows, black lashes, eye details, cosmetics, earrings, pink gown and shoes
stay original. These four shades occur only in skin in the five reviewed
sources. North sitting exposes only the hands. The seated hands and neckline
use a different balance of source shades from the standing poses; their shading
is retained through the existing targets.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked at eyelid, hand and clothing boundaries. The material test checks
every pixel independently of component seeds, plus 187 distinct literal
material landmarks, per-frame counts, transparency, metadata, unchanged profile
roles and prior output bytes against slice 174.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Blink East | 3 | 57, 67, 57 |
| Blink South | 3 | 67, 77, 67 |
| Sit East | 1 | 46 |
| Sit North | 1 | 6 |
| Sit South | 1 | 59 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Blinking
retains durations `[0.075, 0.125, 0.075]`; sitting has one frame and no duration
property. Both cycles remain linear with South as default and native West
mirroring. Sitting retains `is_seated = true`; blinking is unseated. Neither
cycle has a final-frame hold or a configured speaking-pause transition.

## Offline review

- [Five-choice sample summary](../../../generated/slice-175-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-175-review/index.html).

The summary samples Blink East frame 2, Blink South frame 2 and Sit South frame 1.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 387 regular tests and the
release build passed through the Nix shell; 457 opt-in tests were ignored.
The selected local suite passed 248 tests, including the original Elsie portrait
and world tests, the new blinking/sitting material test, historical character
checks and three GML runtime tests. Historical pixel-test expectations are
unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the five registry additions. Four valid mask mutations
omitted the closed-eye face, seated neckline, seated East hand or seated North
right hand. Each generated successfully and failed the intended pixel assertion.
All five metadata sidecars matched independent archive reads.

All 188 standalone Elsie variants passed strict recipe validation; 376 final
PNG/metadata files match the author candidate. All 19,192 combined variants
validated. The 420 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 174 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All nine Chromium gallery pages loaded local images and links without overflow.
Thirteen cases include twenty-six full-view bindings, enlarged details and
fifteen summary bindings; 2,674,464 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is 310,406
bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-175-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`b5a198a8d0069824318925f5b9b3444ac0a106e24e7c9b705e8f3b6d8ba86bb7`.
The archive's native animator and NPC animate function passed 65 frame/palette
observations and 35 linear completions under Fabricator. Palette binding,
frame phase, West mirroring, seated status, wrapper idempotence, cycle counters
and Spring portrait synchronization were preserved. Engine services are
simulated and cycles selected directly. Natural gameplay dispatch and live
rendering were not exercised. The desktop launcher and installed game were
not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-175-`; author records use
`tmp/elsie-spring-actions-author-`. Fresh exports are in
`extracted/elsie-spring-actions-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-174-build/characters/elsie`.

## Remaining work

Elsie's remaining world inventory has 24 Spring, 32 Summer, 32 Autumn and
29 Winter strips. Next: Spring eating and drinking, six strips. The user
approved this slice; live-game verification remains pending.
