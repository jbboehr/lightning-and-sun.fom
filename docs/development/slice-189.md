# Elsie Autumn standing and walking

Following `a10b700`, this slice adds Elsie's Autumn standing and walking in
North, South and East, plus native West mirroring. Six strips contain fifteen
source frames and five mirrored views, giving twenty review cases. User art
approval is pending. Local outputs use `generated/review/` and `generated/build/`.

Elsie has 109 sources: 36 portraits and seventy-three world strips. The
36-character collection has 4,860 sources and 19,440 variants. Z retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 103 prior
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Elsie
coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 103 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across fifteen frames, 363 component seeds select 631 pixels. Forehead, face,
neckline and hands follow the palette. Gray hair, eyebrows, lashes, eye details,
gold earrings, blue collar, burgundy gown, pink trim and shoes retain their
original colors. The four world shades occur only in skin in these six pinned
sources. North views expose a small neckline area and hands. Pale pink shoe
highlights remain unchanged. The gown uses `#4C0F1E`, which is also a portrait
palette role; the world component groups preserve those clothing pixels.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the face, neckline, hands, clothing and shoes. The material
test checks every pixel independently of component seeds, plus 299 distinct
literal material landmarks, per-frame counts, transparency, metadata, unchanged
profile roles and prior output bytes against slice 188.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Idle North | 1 | 13 |
| Idle South | 1 | 59 |
| Idle East | 1 | 55 |
| Walk North | 4 | 13, 11, 13, 11 |
| Walk South | 4 | 59, 57, 59, 57 |
| Walk East | 4 | 55, 60, 55, 54 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Idle has
one frame and no duration property; walking retains four frames at 0.15 seconds
each. Both cycles remain linear, default to South and use native West mirroring.
Neither is seated or has a final-frame hold. Both retain
`on_pause_speaking_turn = true`; walking also retains speaking and background
pause transitions to idle. The offline animator harness supplies a one-second
default for omitted idle durations; live timing and pause dispatch remain
unverified.

## Offline review

- [Five-choice sample summary](../../../generated/slice-189-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-189-review/index.html).

The summary samples Idle South frame 1, Walk East frame 2 and Walk North frame 2.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 401 regular tests and the
release build passed through the Nix shell; 471 opt-in tests were ignored.
All seventeen Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the Idle South face, neckline, viewer-left hand or viewer-right hand.
Each generated successfully and failed the intended pixel assertion. All six
metadata sidecars matched independent archive reads.

All 436 standalone Elsie variants passed strict recipe validation; 872 final
PNG/metadata files match the author candidate.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty cases include forty full-view bindings, enlarged details and fifteen
summary bindings; 3,770,400 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is 420,870 bytes.

All 19,440 combined variants validated. The 1,030 prior Elsie PNG/metadata
files and 47,685 other-character files match slice 188 byte for byte. Previous
runtime rows, choices and hotkeys also match. All 36 export reports retain prior
entries and the archive hash. The five frozen configuration inputs and original
archive retain their hashes. The short review alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-189-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`b7d8837e3ca45a26d43c0ecaa037c22070082c17ede26f3008f3a33ad9e8186e`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed 100 frame/palette
observations and forty linear completions under Fabricator. Palette binding,
frame phase, standing status, West mirroring, wrapper idempotence, cycle counters
and Autumn portrait synchronization were preserved. Engine services are
simulated and cycles selected directly. Natural gameplay dispatch, interruptions
and live rendering were not exercised. The desktop launcher and installed game
were not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-189/`. Scratch outputs use `tmp/slice-189-` and
`tmp/elsie-autumn-world-author-`. Fresh exports are in
`extracted/elsie-autumn-world-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-188-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring and 32 Summer world strips are covered. Elsie's remaining world
inventory has 26 Autumn and 29 Winter strips. Next: Autumn blinking and sitting,
five strips. User art approval and live-game verification remain pending.
