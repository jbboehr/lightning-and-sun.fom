# Elsie Spring shocked reaction

Following `2bd120d`, this slice adds Elsie's Spring shocked reaction: start,
loop and end. Each South-facing strip has one frame, giving three review cases.
The user approved the artwork on 2026-10-13. Local outputs use `generated/review/` and
`generated/build/`.

Elsie has 59 sources: 36 portraits and twenty-three world strips. The
36-character collection has 4,810 sources and 19,240 variants. Z retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 56 prior
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Elsie
coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-177-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 56 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across three frames, 125 component seeds select 239 pixels. Forehead, face,
neckline, raised hands and exposed ankles follow the palette. Gray hair,
eyebrows, lashes, eye details, mouth interiors, cosmetics, earrings, pink gown
and shoes stay original. The four world shades occur only in skin in these
three pinned sources. Pale pink gown and shoe highlights remain distinct from
skin; the small exposed ankle regions above the shoes in the loop change.
The start and end PNGs are identical, with distinct native metadata IDs.

All three source frames were inspected in five-choice sheets and original
pixel grids. The material test checks every pixel independently of component
seeds, plus 70 distinct literal material landmarks, per-frame counts,
transparency, metadata, unchanged profile roles and prior output bytes against
slice 177.

| Strip | Frames | Selected pixels |
| --- | --- | --- |
| Shocked start South | 1 | 77 |
| Shocked loop South | 1 | 85 |
| Shocked end South | 1 | 77 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Their
sidecars omit explicit frame counts and durations, retaining the native
single-frame defaults. The cycle remains complex, South-only and unseated.
The installed Elsie NPC configuration matches the source byte for byte.

## Offline review

- [Five-choice summary of all three phases](../../../generated/slice-178-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-178-review/index.html).

Rows are start, loop and end; columns are Vanilla, Debug Blue, Hayden, Ryis
and Seridia. The full review includes every source frame, Vanilla left and
Debug Blue right, with full views and enlarged details. No West mirrors apply.
Images, exports and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 390 regular tests and the
release build passed through the Nix shell; 460 opt-in tests were ignored.
The selected local suite passed 251 tests, including the original Elsie
portrait, world, blinking/sitting, eating/drinking and general-action tests,
the new reaction material test, historical character checks and three GML
runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted the loop face, neckline, left hand or right hand. Each generated
successfully and failed the intended pixel assertion. All three metadata
sidecars matched independent archive reads.

All 236 standalone Elsie variants passed strict recipe validation; 472 final
PNG/metadata files match the author candidate. All 19,240 combined variants
validated. The 560 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 177 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All five Chromium gallery pages loaded local images and links without overflow.
Three cases include six full-view bindings, enlarged details and fifteen
summary bindings; 963,360 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is 136,485 bytes;
its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-178-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`c569ea8d5a08203c662bc2ef40ad9a96d70d1c6572ed413caa96b158a8532b5e`.
The archive's native animator and NPC animate function passed fifteen
frame/palette observations and twenty complex transitions under Fabricator.
For each palette, start advances to loop, loop repeats, then advances to end,
and end completes back at start. Palette binding, frame phase, unseated status,
wrapper idempotence, cycle counters and Spring portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch and live rendering were not exercised. The desktop launcher
and installed game were not changed.

Evidence uses `tmp/slice-178-`; author records use
`tmp/elsie-spring-reactions-author-`. Fresh exports are in
`extracted/elsie-spring-reactions-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-177-build/characters/elsie`. Scope verification checks nine
expected paths, an empty Git index, unchanged historical pixel tests and ignored
game-derived artifacts.

## Remaining work

Elsie's remaining world inventory has 12 Spring, 32 Summer, 32 Autumn and
29 Winter strips. Next: Spring standing writing, three strips. The user
approved this slice; live-game verification remains pending.
