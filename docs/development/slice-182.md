# Elsie Summer standing and walking

Following `36194d6`, this slice adds Elsie's Summer standing and walking in
North, South and East, plus native West mirroring. Six strips contain fifteen
source frames and five mirrored views, giving twenty review cases. The user
approved the artwork on 2026-10-13. Local outputs use `generated/review/` and
`generated/build/`.

Elsie has 77 sources: 36 portraits and forty-one world strips. The 36-character
collection has 4,828 sources and 19,312 variants. Z retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This slice
adds no runtime code or color roles. All 71 prior regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests retain
their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-181-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 71 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across fifteen frames, 399 component seeds select 687 pixels. Forehead, face,
neckline, exposed arms, hands and ankles follow the palette. Gray hair,
eyebrows, lashes, eye details, turquoise earrings, gold scarf, blue shawl,
dress and shoes retain their original colors. The four world shades occur only
in skin in these six pinned sources. North views expose a small neckline area,
arms and ankles; the last East walking frame hides the far hand.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the face, neckline, arms, hands, clothing and ankles.
The material test checks every pixel independently of component seeds, plus
324 distinct literal material landmarks, per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 181.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Idle North | 1 | 27 |
| Idle South | 1 | 58 |
| Idle East | 1 | 54 |
| Walk North | 4 | 27, 23, 27, 23 |
| Walk South | 4 | 58, 55, 58, 55 |
| Walk East | 4 | 54, 64, 54, 50 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Idle has
one frame and no duration property; walking retains four frames at 0.15 seconds
each. Both cycles remain linear, default to South and use native West mirroring.
Neither is seated or has a final-frame hold. Both retain
`on_pause_speaking_turn = true`; walking also retains speaking and background
pause transitions to idle. The installed Elsie NPC configuration matches the
source byte for byte. The offline animator harness supplies a one-second
default for omitted idle durations; live timing and pause dispatch remain
unverified.

## Offline review

- [Five-choice sample summary](../../../generated/slice-182-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-182-review/index.html).

The summary samples Idle South frame 1, Walk East frame 2 and Walk North frame 2.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 394 regular tests and the
release build passed through the Nix shell; 464 opt-in tests were ignored.
The selected local suite passed 255 tests, including all earlier Elsie
material checks, the new Summer material test, historical character checks
and three GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the Idle South face, neckline, viewer-left hand or viewer-right hand.
Each generated successfully and failed the intended pixel assertion. All six
metadata sidecars matched independent archive reads.

All 308 standalone Elsie variants passed strict recipe validation; 616 final
PNG/metadata files match the author candidate. All 19,312 combined variants
validated. The 710 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 181 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty cases include forty full-view bindings, enlarged details and fifteen
summary bindings; 3,770,400 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is
425,039 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-182-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`d03ab7fab6e9da76d1a91ebf7e3cd371945c61c76fec809505376fd69cfb97e9`.
The archive's native animator and NPC animate function passed 100 frame/palette
observations and forty linear completions under Fabricator. Palette binding,
frame phase, standing status, West mirroring, wrapper idempotence, cycle counters
and Summer portrait synchronization were preserved. Engine services are
simulated and cycles selected directly. Natural gameplay dispatch, interruptions
and live rendering were not exercised. The desktop launcher and installed game
were not changed.

Evidence uses `tmp/slice-182-`; author records use
`tmp/elsie-summer-world-author-`. Fresh exports are in
`extracted/elsie-summer-world-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-181-build/characters/elsie`. Scope verification checks nine
expected paths, an empty Git index, unchanged historical pixel tests and ignored
game-derived artifacts.

## Remaining work

Elsie's remaining world inventory has 26 Summer, 32 Autumn and 29 Winter
strips. Next: Summer blinking and sitting, five strips. Live-game verification
remains pending for this slice.
