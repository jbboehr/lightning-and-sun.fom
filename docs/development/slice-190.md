# Elsie Autumn blinking and sitting

Following `4294816`, this slice adds Elsie's Autumn blinking in East and
South and sitting in East, North and South, plus native West mirroring. Five
strips contain nine source frames and four mirrored views, giving thirteen
review cases. User art approval is pending. Local outputs use
`generated/review/` and `generated/build/`.

Elsie has 114 sources: 36 portraits and seventy-eight world strips. The
36-character collection has 4,865 sources and 19,460 variants. Z retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 109 prior
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Elsie
coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 109 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across nine frames, 262 component seeds select 496 pixels. Forehead, face,
closed eyelids, neckline and hands follow the palette. Gray hair, eyebrows,
black lashes, eye details, gold earrings, blue collar, burgundy gown, pink trim
and shoes retain their original colors. The four world shades occur only in
skin in these five pinned sources. North sitting exposes just a small neckline
patch and the hands. Pale pink shoe highlights remain unchanged. Where present,
clothing shade `#4C0F1E` is preserved despite its separate portrait palette role.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the closed eyelids, neckline, hands, clothing and shoes.
The material test checks every pixel independently of component seeds, plus
191 distinct literal material landmarks, per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 189.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Blink East | 3 | 59, 69, 59 |
| Blink South | 3 | 63, 73, 63 |
| Sit East | 1 | 48 |
| Sit North | 1 | 9 |
| Sit South | 1 | 53 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Blinking
retains durations `[0.075, 0.125, 0.075]`; sitting has one frame and no duration
property. Both cycles remain linear with South as default and native West
mirroring. Sitting retains `is_seated = true`; blinking is unseated. Neither
cycle has a final-frame hold or configured speaking/background pause settings.
The offline animator harness supplies a one-second default for omitted sitting
durations; live engine timing remains unverified.

## Offline review

- [Five-choice sample summary](../../../generated/slice-190-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-190-review/index.html).

The summary samples Blink East frame 2, Blink South frame 2 and Sit South frame 1.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 402 regular tests and the
release build passed through the Nix shell; 472 opt-in tests were ignored.
All eighteen Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the five registry additions. Four valid mask mutations
omitted the closed-eye East face, seated East neckline, seated East near hand
or seated North viewer-right hand. Each generated successfully and failed the
intended pixel assertion. All five metadata sidecars matched independent
archive reads.

All 456 standalone Elsie variants passed strict recipe validation; 912 final
PNG/metadata files match the author candidate. All 19,460 combined variants
validated. The 1,090 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 189 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All nine Chromium gallery pages loaded local images and links without overflow.
Thirteen cases include twenty-six full-view bindings, enlarged details and
fifteen summary bindings; 2,614,560 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
304,094 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-190-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`cd66121e8a916a5fbd46d739903596142b7af58dfe79cebed48d32495d662c5f`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed 65 frame/palette
observations and thirty-five linear completions under Fabricator. Palette
binding, frame phase, seated status, West mirroring, wrapper idempotence, cycle
counters and Autumn portrait synchronization were preserved. Engine services
are simulated and cycles selected directly. Natural gameplay dispatch and live
rendering were not exercised. The desktop launcher and installed game were
not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-190/`. Scratch outputs use `tmp/slice-190-` and
`tmp/elsie-autumn-actions-author-`. Fresh exports are in
`extracted/elsie-autumn-actions-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-189-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring and 32 Summer world strips are covered. Elsie's remaining world
inventory has 21 Autumn and 29 Winter strips. Next: Autumn eating and drinking,
six strips. User art approval and live-game verification remain pending.
