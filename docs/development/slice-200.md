# Elsie Winter standing writing

Following `e47e668`, this slice adds Elsie's Winter standing writing. Three
South-facing strips contain eight frames across the start, loop and end phases.
There is no West mirror. User art approval is pending. Local outputs use
`generated/review/` and `generated/build/`.

Elsie has 158 sources: 36 portraits and 122 world strips. The 36-character
collection has 4,909 sources and 19,636 variants. Z retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This slice
adds no runtime code or color roles. All 155 prior regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests retain
their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 155 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and component groups are reused. Across eight
frames, 174 component seeds select 369 pixels. Exposed facial skin and closed
eyelids change. Her moving hands stay gloved and the collar covers her neck.
Gray hair, lashes, eye details, gold hood, cream fur trim, gloves, green dress,
brooch, boots, book and quill retain their original colors. None of the four
world skin shades occurs in other materials in these pinned sources. Pale book
pages and quill highlights remain distinct from the cream gloves.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around closed eyelids, cheeks beside the quill, hair, collar,
moving gloves, book, quill, dress and boots. The material test checks every pixel
independently of component seeds, plus 293 distinct literal material landmarks,
per-frame counts, transparency, metadata, unchanged profile roles and prior
output bytes against slice 199. The book obscures lighter dress shades in some
poses; literal landmarks reflect the actual visible colors.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Write Start South | 2 | 54, 44 |
| Write Loop South | 4 | 41, 44, 44, 44 |
| Write End South | 2 | 44, 54 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54.
Start duration remains scalar `0.125`, loop durations remain
`[0.1, 0.125, 0.1, 0.3]`, and end durations remain `[0.125, 0.1]`.
The write cycle remains complex with South as the only/default direction.
It has no seated flag, configured pause settings or final-frame hold.

## Offline review

- [Five-choice sample summary](../../../generated/slice-200-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-200-review/index.html).

The summary samples Write Start frame 1, Write Loop frame 4 and Write End
frame 1. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full
review includes every source frame, Vanilla left and Debug Blue right, with
full views and enlarged details. Full crops include every nontransparent pixel.
Images, exports and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 412 regular tests and the
release build passed through the Nix shell; 482 opt-in tests were ignored.
All twenty-eight Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted components near the start/end closed eyelids, loop forehead or loop cheek.
Each generated successfully and failed the intended pixel assertion. All three
metadata sidecars matched independent archive reads.

All 632 standalone Elsie variants passed strict recipe validation; 1,264 final
PNG/metadata files match the author candidate. All 19,636 combined variants
validated. The 1,550 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 199 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All five Chromium gallery pages loaded local images and links without overflow.
Eight cases include sixteen full-view bindings, enlarged details and fifteen
summary bindings; 1,788,960 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is
203,285 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-200-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`4117cb2eee476dc6acd770221416a9f6fbf3c8bacb5863780b4cbc90d54cb16d`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed forty frame/palette
observations and twenty complex phase transitions under Fabricator. Checks cover
start-to-loop, repeated loop, loop-to-end and completion/reset for all five
choices. Palette binding, frame phase, standing status, wrapper idempotence,
cycle counters and Winter portrait synchronization were preserved. Engine
services are simulated and cycles selected directly. Natural gameplay dispatch
and live rendering were not exercised. The desktop launcher and installed game
were not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-200/`. Scratch outputs use `tmp/slice-200-` and
`tmp/elsie-winter-writing-author-`. Fresh exports are in
`extracted/elsie-winter-writing-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-199-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring, 32 Summer and 32 Autumn world strips are covered, plus twenty-three
of 29 Winter strips. Elsie's remaining world inventory has six Winter seated-writing
strips. Next: those seated-writing strips, completing her four seasonal world
folders. User art approval and live-game verification remain pending.
