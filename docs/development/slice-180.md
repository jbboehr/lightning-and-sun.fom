# Elsie Spring seated writing

Following `1feb5e8`, this slice adds Elsie's Spring seated-writing start,
loop and end in East and South, plus native West mirroring. Sixteen source
frames give twenty-four review cases. The user approved the artwork on 2026-10-13. Local
outputs use `generated/review/` and `generated/build/`.

Elsie has 68 sources: 36 portraits and thirty-two world strips. The
36-character collection has 4,819 sources and 19,276 variants. Z retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 62 prior
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Elsie
coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-179-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 62 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across sixteen frames, 385 component seeds select 767 pixels. Forehead, face,
visible neckline and the moving writing hand follow the palette. Gray hair,
eyebrows, lashes, eye details, cosmetics, earrings, pink gown, shoes, book and
quill stay original. The four world shades occur only in skin in these six
pinned sources. Cream pages, tan book edges, pink feathers and the pale quill
shaft retain their own colors. Some East frames hide the writing hand behind
the quill and book; those frames retain the smaller visible skin regions.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the face, hand, quill, book and clothing boundaries.
The material test checks every pixel independently of component seeds, plus
423 distinct literal material landmarks, per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 179.

| Strip | Frames | Selected pixels per frame | Native durations |
| --- | --- | --- | --- |
| Write sit start East | 2 | 55, 38 | 0.125 per frame |
| Write sit loop East | 4 | 30, 37, 34, 39 | 0.1, 0.125, 0.1, 0.3 |
| Write sit end East | 2 | 38, 55 | 0.125, 0.1 |
| Write sit start South | 2 | 68, 53 | 0.125 per frame |
| Write sit loop South | 4 | 42, 49, 54, 54 | 0.1, 0.125, 0.1, 0.3 |
| Write sit end South | 2 | 53, 68 | 0.125, 0.1 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. The
cycle remains complex and seated, with South as default and native West
mirroring. The installed Elsie NPC configuration matches the source byte
for byte.

## Offline review

- [Five-choice sample summary](../../../generated/slice-180-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-180-review/index.html).

The summary samples East start frame 1, East loop frame 2 and South loop frame 3.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 392 regular tests and the
release build passed through the Nix shell; 462 opt-in tests were ignored.
The selected local suite passed 253 tests, including all earlier Elsie
material checks, the new seated-writing material test, historical character
checks and three GML runtime tests. Historical pixel-test expectations are
unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the East loop face, South start neckline, East start hand or South loop
hand. Each generated successfully and failed the intended pixel assertion.
All six metadata sidecars matched independent archive reads.

All 272 standalone Elsie variants passed strict recipe validation; 544 final
PNG/metadata files match the author candidate. All 19,276 combined variants
validated. The 620 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 179 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All eleven Chromium gallery pages loaded local images and links without overflow.
Twenty-four cases include forty-eight full-view bindings, enlarged details and
fifteen summary bindings; 4,430,880 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
493,410 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-180-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`e8e6d7aec825325331acff8dca7717a05ace966e6e9eaa8c119bf25ba052fb0d`.
The archive's native animator and NPC animate function passed 120 frame/palette
observations and sixty complex transitions under Fabricator. For each palette
and facing, start advances to loop, loop repeats, then advances to end, and end
completes back at start. Palette binding, frame phase, seated status, West
mirroring, wrapper idempotence, cycle counters and Spring portrait
synchronization were preserved. Engine services are simulated and cycles
selected directly. Natural gameplay dispatch and live rendering were not
exercised. The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-180-`; author records use
`tmp/elsie-spring-seated-writing-author-`. Fresh exports are in
`extracted/elsie-spring-seated-writing-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-179-build/characters/elsie`. Scope verification checks nine
expected paths, an empty Git index, unchanged historical pixel tests and ignored
game-derived artifacts.

## Remaining work

Elsie's remaining world inventory has 3 Spring, 32 Summer, 32 Autumn and
29 Winter strips. Next: Spring singing, three strips, completing her Spring
world folder. The user approved this slice; live-game verification remains pending.
