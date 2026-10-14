# Elsie Winter standing and walking

Following `7810d5c`, this slice adds Elsie's Winter standing and walking
sprites: six strips, fifteen source frames and five native West mirrors.
User art approval is pending. Local outputs use `generated/review/` and
`generated/build/`.

Elsie has 141 sources: 36 portraits and 105 world strips. The 36-character
collection has 4,892 sources and 19,568 variants. Z retains Vanilla, Debug
Blue, Hayden, Ryis and Seridia choices with portrait/world synchronization.
This slice adds no runtime code or color roles. All 135 prior regions, groups,
target mappings and collection sets remain unchanged. Historical pixel tests
retain their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 135 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and component groups are reused. Across the
fifteen frames, 200 component seeds select 380 pixels. Only exposed facial
skin changes. Her Winter gloves and collar cover her hands and neck, and the
North-facing strips have no exposed skin: their empty seed lists explicitly
preserve every pixel. Gray hair, eye details, gold hood, cream fur trim, gloves,
green dress, brooch and boots retain their original colors. None of the four
world skin shades occurs in other materials in these pinned sources.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the face, hair, hood, collar, gloves, dress and boots.
The material test checks every pixel independently of component seeds, plus
389 distinct literal material landmarks, per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 195.
North-facing frames additionally require complete image equality.

| Strip | Frames | Selected pixels per frame | Native durations |
| --- | --- | --- | --- |
| Idle North | 1 | 0 | Omitted in source metadata |
| Idle South | 1 | 40 | Omitted in source metadata |
| Idle East | 1 | 36 | Omitted in source metadata |
| Walk North | 4 | 0, 0, 0, 0 | 0.15 per frame |
| Walk South | 4 | 40, 40, 40, 40 | 0.15 per frame |
| Walk East | 4 | 36, 36, 36, 36 | 0.15 per frame |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Idle and
walk remain linear, standing cycles with North, South and East source sprites;
West mirrors East. The default direction is South and speaking may turn the
actor. Walking pauses to idle for speaking and background interruptions.
The offline animator harness supplies a one-second default for omitted idle
durations; live engine timing and pause dispatch remain unverified.

## Offline review

- [Five-choice sample summary](../../../generated/slice-196-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-196-review/index.html).

The summary samples idle South frame 1, walk East frame 2 and walk North
frame 2. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full
review includes all twenty cases, Vanilla left and Debug Blue right, with
full views and enlarged details. Full crops include every nontransparent
pixel. Images, exports and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 408 regular tests and the
release build passed through the Nix shell; 478 opt-in tests were ignored.
All twenty-four Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted forehead, viewer-left cheek, viewer-right cheek or chin components from
idle South. Each generated successfully and failed the intended pixel assertion.
All six metadata sidecars matched independent archive reads.

All 564 standalone Elsie variants passed strict recipe validation; 1,128 final
PNG/metadata files match the author candidate. All 19,568 combined variants
validated. The 1,350 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 195 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty cases include forty full-view bindings, enlarged details and fifteen
summary bindings; 3,770,400 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is
414,428 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-196-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`64cf9b33b7a85571ecb379e4c896cbe05199214259f0d57efb12b34a9f63f0fb`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed 100 frame/palette
observations and forty linear completions under Fabricator. Palette binding,
frame phase, West mirroring, standing status, wrapper idempotence, cycle counters
and Winter portrait synchronization were preserved. Engine services are simulated
and cycles selected directly. Natural gameplay dispatch, speech interruption and
live rendering were not exercised. The desktop launcher and installed game were
not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-196/`. Scratch outputs use `tmp/slice-196-` and
`tmp/elsie-winter-world-author-`. Fresh exports are in
`extracted/elsie-winter-world-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-195-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring, 32 Summer and 32 Autumn world strips are covered, plus six of
29 Winter strips. Elsie's remaining world inventory has 23 Winter strips.
Next: Winter blinking and sitting, five strips. User art approval and live-game
verification remain pending.
