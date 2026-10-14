# Elsie Autumn standing writing

Following `ba494c1`, this slice adds Elsie's Autumn standing writing. Three
South-facing strips contain eight frames across the start, loop and end phases.
There is no West mirror. User art approval is pending. Local outputs use
`generated/review/` and `generated/build/`.

Elsie has 126 sources: 36 portraits and ninety world strips. The 36-character
collection has 4,877 sources and 19,508 variants. Z retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This slice
adds no runtime code or color roles. All 123 prior regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests retain
their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 123 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across eight frames, 234 component seeds select 454 pixels. Forehead, face,
closed eyelids, neckline and moving hands follow the palette. Gray hair,
eyebrows, black lashes, eye details, gold earrings, blue collar, burgundy gown,
pink trim, shoes, book and quill retain their original colors. The four world
shades occur only in skin in these three pinned sources. Pale book pages and
the quill's light and shaded pixels remain distinct from the writing hand.
Pale pink shoe highlights stay original. Clothing shade `#4C0F1E` is preserved
despite its separate portrait palette role.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around closed eyelids, neckline, writing hand, book, quill, clothing
and shoes. The material test checks every pixel independently of component
seeds, plus 288 distinct literal material landmarks, per-frame counts,
transparency, metadata, unchanged profile roles and prior output bytes against
slice 192.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Write Start South | 2 | 66, 55 |
| Write Loop South | 4 | 47, 52, 57, 56 |
| Write End South | 2 | 55, 66 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54.
Start duration remains scalar `0.125`, loop durations remain
`[0.1, 0.125, 0.1, 0.3]`, and end durations remain `[0.125, 0.1]`.
The write cycle remains complex with South as the only/default direction.
It has no seated flag, configured pause settings or final-frame hold.

## Offline review

- [Five-choice sample summary](../../../generated/slice-193-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-193-review/index.html).

The summary samples Write Start frame 1, Write Loop frame 4 and Write End
frame 1. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full
review includes every source frame, Vanilla left and Debug Blue right, with
full views and enlarged details. Images, exports and packages remain local
and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 405 regular tests and the
release build passed through the Nix shell; 475 opt-in tests were ignored.
All twenty-one Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted the start face/closed eyelids, loop neckline, loop writing hand or end
writing hand. Each generated successfully and failed the intended pixel
assertion. All three metadata sidecars matched independent archive reads.

All 504 standalone Elsie variants passed strict recipe validation; 1,008 final
PNG/metadata files match the author candidate. All 19,508 combined variants
validated. The 1,230 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 192 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All five Chromium gallery pages loaded local images and links without overflow.
Eight cases include sixteen full-view bindings, enlarged details and fifteen
summary bindings; 1,788,960 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is
207,157 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-193-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`f370c6a1d917d95c28f6a942e725b4b576baf45661591c5699bf03b9ef5d0b92`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed forty frame/palette
observations and twenty complex phase transitions under Fabricator. Checks cover
start-to-loop, repeated loop, loop-to-end and completion/reset for all five
choices. Palette binding, frame phase, standing status, wrapper idempotence,
cycle counters and Autumn portrait synchronization were preserved. Engine
services are simulated and cycles selected directly. Natural gameplay dispatch
and live rendering were not exercised. The desktop launcher and installed game
were not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-193/`. Scratch outputs use `tmp/slice-193-` and
`tmp/elsie-autumn-writing-author-`. Fresh exports are in
`extracted/elsie-autumn-writing-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-192-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring and 32 Summer world strips are covered. Elsie's remaining world
inventory has 9 Autumn and 29 Winter strips. Next: Autumn seated writing,
six strips. User art approval and live-game verification remain pending.
