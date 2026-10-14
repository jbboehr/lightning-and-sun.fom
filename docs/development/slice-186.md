# Elsie Summer standing writing

Following `be409d8`, this slice adds Elsie's Summer standing writing. Three
South-facing strips contain eight frames across the start, loop and end phases.
There is no West mirror. User art approval was received on 2026-10-13.
Local outputs use `generated/review/` and `generated/build/`.

Elsie has 94 sources: 36 portraits and fifty-eight world strips. The 36-character
collection has 4,845 sources and 19,380 variants. Z retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This slice
adds no runtime code or color roles. All 91 prior regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests retain
their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-185-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 91 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across eight frames, 233 component seeds select 447 pixels. Forehead, face,
closed eyelids, neckline, moving hands and exposed ankles follow the palette.
Gray hair, eyebrows, black lashes, eye details, turquoise earrings, gold scarf,
blue shawl, dress, shoes, book and quill retain their original colors. The four
world shades occur only in skin in these three pinned sources. Pale book pages
and the pink quill's light and shaded pixels remain distinct from the writing
hand.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the closed eyelids, neckline, writing hand, book, quill,
clothing and ankles. The material test checks every pixel independently of
component seeds, plus 266 distinct literal material landmarks, per-frame counts,
transparency, metadata, unchanged profile roles and prior output bytes against
slice 185.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Write Start South | 2 | 65, 53 |
| Write Loop South | 4 | 48, 52, 56, 55 |
| Write End South | 2 | 53, 65 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54.
Start duration remains scalar `0.125`, loop durations remain
`[0.1, 0.125, 0.1, 0.3]`, and end durations remain `[0.125, 0.1]`.
The write cycle remains complex with South as the only/default direction.
It has no seated flag, configured pause settings or final-frame hold.
The installed Elsie NPC configuration matches the source byte for byte.

## Offline review

- [Five-choice sample summary](../../../generated/slice-186-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-186-review/index.html).

The summary samples Write Start frame 1, Write Loop frame 4 and Write End
frame 1. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full
review includes every source frame, Vanilla left and Debug Blue right, with
full views and enlarged details. Images, exports and packages remain local
and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 398 regular tests and the
release build passed through the Nix shell; 468 opt-in tests were ignored.
The selected local suite passed 259 tests, including all earlier Elsie
material checks, the new standing-writing material test, historical character
checks and three GML runtime tests. Historical pixel-test expectations are
unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted the start face/closed eyelids, loop neckline, loop writing hand or end
ankles. Each generated successfully and failed the intended pixel assertion.
All three metadata sidecars matched independent archive reads.

All 376 standalone Elsie variants passed strict recipe validation; 752 final
PNG/metadata files match the author candidate. All 19,380 combined variants
validated. The 910 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 185 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All five Chromium gallery pages loaded local images and links without overflow.
Eight cases include sixteen full-view bindings, enlarged details and fifteen
summary bindings; 1,788,960 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is
206,699 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-186-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`ef7b29bd167bbc8bba4d00c7884b521448583949a36fb4833e76abe6453020b4`.
The archive's native animator and NPC animate function passed forty frame/palette
observations and twenty complex phase transitions under Fabricator. Checks cover
start-to-loop, repeated loop, loop-to-end and completion/reset for all five
choices. Palette binding, frame phase, standing status, wrapper idempotence,
cycle counters and Summer portrait synchronization were preserved. Engine
services are simulated and cycles selected directly. Natural gameplay dispatch
and live rendering were not exercised. The desktop launcher and installed game
were not changed.

Evidence uses `tmp/slice-186-`; author records use
`tmp/elsie-summer-writing-author-`. Fresh exports are in
`extracted/elsie-summer-writing-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-185-build/characters/elsie`. Scope verification checks nine
expected paths, an empty Git index, unchanged historical pixel tests and ignored
game-derived artifacts.

## Remaining work

Elsie's remaining world inventory has 9 Summer, 32 Autumn and 29 Winter strips.
Next: Summer seated writing, six strips. Live-game verification remains pending
for this slice.
