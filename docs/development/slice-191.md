# Elsie Autumn eating and drinking

Following `ce00af7`, this slice adds Elsie's Autumn eating and drinking in
East, North and South, plus native West mirroring. Six strips contain twenty-two
source frames and eight mirrored views, giving thirty review cases. User art
approval is pending. Local outputs use `generated/review/` and `generated/build/`.

Elsie has 120 sources: 36 portraits and eighty-four world strips. The
36-character collection has 4,871 sources and 19,484 variants. Z retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 114 prior
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Elsie
coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 114 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across twenty-two frames, 510 component seeds select 943 pixels. Forehead,
face, closed eyelids, neckline and moving hands follow the palette. Gray hair,
eyebrows, black lashes, eye details, dark red mouth interiors, gold earrings,
blue collar, burgundy gown, pink trim and shoes retain their original colors.
The four world shades occur only in skin in these six pinned sources. North
views expose the neckline and hands. Pale pink shoe highlights remain unchanged.
Where present, clothing shade `#4C0F1E` is preserved despite its separate
portrait palette role.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the open mouths, closed eyelids, neckline, moving hands,
clothing and shoes. The material test checks every pixel independently of
component seeds, plus 381 distinct literal material landmarks, per-frame counts,
transparency, metadata, unchanged profile roles and prior output bytes against
slice 190.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Eat East | 5 | 47, 58, 48, 56, 49 |
| Eat North | 3 | 12, 8, 12 |
| Eat South | 5 | 58, 68, 54, 72, 53 |
| Drink East | 3 | 45, 49, 45 |
| Drink North | 3 | 12, 8, 12 |
| Drink South | 3 | 55, 67, 55 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54.
Eating East/South retains durations `[0.125, 0.15, 0.175, 0.125, 0.6]`;
North eating and all drinking strips retain scalar duration `1.0`. Both cycles
remain linear with South as default, native West mirroring, `is_seated = true`
and `last_frame_hold = [240, 360]`. Neither cycle configures speaking or
background pause settings.

## Offline review

- [Five-choice sample summary](../../../generated/slice-191-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-191-review/index.html).

The summary samples Eat South frame 3, Drink East frame 2 and Eat North frame 2.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 403 regular tests and the
release build passed through the Nix shell; 473 opt-in tests were ignored.
All nineteen Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the open-mouth East eating face, South eating neckline, South eating
hand or North drinking viewer-right hand. Each generated successfully and
failed the intended pixel assertion. All six metadata sidecars matched
independent archive reads.

All 480 standalone Elsie variants passed strict recipe validation; 960 final
PNG/metadata files match the author candidate. All 19,484 combined variants
validated. The 1,140 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 190 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All thirteen Chromium gallery pages loaded local images and links without
overflow. Thirty cases include sixty full-view bindings, enlarged details and
fifteen summary bindings; 5,421,600 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
561,033 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-191-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`f38ab5f4f9ae2b25cb3747a8c1e4a97307c19377359b49ad106b7332a33dd5c1`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed 150 frame/palette
observations, forty linear completions and eighty final-frame-hold checks under
Fabricator. Palette binding, frame phase, seated status, West mirroring,
wrapper idempotence, cycle counters and Autumn portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch and live rendering were not exercised. The desktop launcher
and installed game were not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-191/`. Scratch outputs use `tmp/slice-191-` and
`tmp/elsie-autumn-food-author-`. Fresh exports are in
`extracted/elsie-autumn-food-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-190-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring and 32 Summer world strips are covered. Elsie's remaining world
inventory has 15 Autumn and 29 Winter strips. Next: Autumn general actions,
three strips. User art approval and live-game verification remain pending.
