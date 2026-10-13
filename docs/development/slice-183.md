# Elsie Summer blinking and sitting

Following `26dd29a`, this slice adds Elsie's Summer blinking in East and
South and sitting in East, North and South, plus native West mirroring. Five
strips contain nine source frames and four mirrored views, giving thirteen
review cases. The user approved the artwork on 2026-10-13. Local outputs use
`generated/review/` and `generated/build/`.

Elsie has 82 sources: 36 portraits and forty-six world strips. The 36-character
collection has 4,833 sources and 19,332 variants. Z retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This slice
adds no runtime code or color roles. All 77 prior regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests retain
their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-182-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 77 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across nine frames, 272 component seeds select 501 pixels. Forehead, face,
closed eyelids, neckline, exposed arms, hands and ankles follow the palette.
Gray hair, eyebrows, black lashes, eye details, turquoise earrings, gold scarf,
blue shawl, dress and shoes retain their original colors. The four world shades
occur only in skin in these five pinned sources. North sitting exposes a small
neckline area and the arms/hands; the seated East and South views also expose
ankles above the gold shoes. Their different source shading is retained through
the existing targets.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the closed eyelids, neckline, arms, hands, clothing and
ankles. The material test checks every pixel independently of component seeds,
plus 204 distinct literal material landmarks, per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 182.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Blink East | 3 | 58, 68, 58 |
| Blink South | 3 | 62, 72, 62 |
| Sit East | 1 | 49 |
| Sit North | 1 | 17 |
| Sit South | 1 | 55 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Blinking
retains durations `[0.075, 0.125, 0.075]`; sitting has one frame and no duration
property. Both cycles remain linear with South as default and native West
mirroring. Sitting retains `is_seated = true`; blinking is unseated. Neither
cycle has a final-frame hold or configured speaking/background pause settings.
The installed Elsie NPC configuration matches the source byte for byte. The
offline animator harness supplies a one-second default for omitted sitting
durations; live engine timing remains unverified.

## Offline review

- [Five-choice sample summary](../../../generated/slice-183-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-183-review/index.html).

The summary samples Blink East frame 2, Blink South frame 2 and Sit South frame 1.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 395 regular tests and the
release build passed through the Nix shell; 465 opt-in tests were ignored.
The selected local suite passed 256 tests, including all earlier Elsie
material checks, the new blinking/sitting material test, historical character
checks and three GML runtime tests. Historical pixel-test expectations are
unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the five registry additions. Four valid mask mutations
omitted the closed-eye East face, seated East neckline, seated East near hand
or seated North viewer-right hand. Each generated successfully and failed the
intended pixel assertion. All five metadata sidecars matched independent
archive reads.

All 328 standalone Elsie variants passed strict recipe validation; 656 final
PNG/metadata files match the author candidate. All 19,332 combined variants
validated. The 770 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 182 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All nine Chromium gallery pages loaded local images and links without overflow.
Thirteen cases include twenty-six full-view bindings, enlarged details and
fifteen summary bindings; 2,614,560 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
304,203 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-183-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`cb050fdc82f5b8681185911e147bd1264254ef0277dbe7ae9bb008d6430f9812`.
The archive's native animator and NPC animate function passed 65 frame/palette
observations and thirty-five linear completions under Fabricator. Palette
binding, frame phase, seated status, West mirroring, wrapper idempotence, cycle
counters and Summer portrait synchronization were preserved. Engine services
are simulated and cycles selected directly. Natural gameplay dispatch and live
rendering were not exercised. The desktop launcher and installed game were
not changed.

Evidence uses `tmp/slice-183-`; author records use
`tmp/elsie-summer-actions-author-`. Fresh exports are in
`extracted/elsie-summer-actions-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-182-build/characters/elsie`. Scope verification checks nine
expected paths, an empty Git index, unchanged historical pixel tests and ignored
game-derived artifacts.

## Remaining work

Elsie's remaining world inventory has 21 Summer, 32 Autumn and 29 Winter
strips. Next: Summer eating and drinking, six strips. Live-game verification
remains pending for this slice.
