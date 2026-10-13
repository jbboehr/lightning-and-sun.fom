# Elsie Spring standing writing

Following `a640806`, this slice adds Elsie's Spring standing-writing start,
loop and end. Eight South-facing source frames give eight review cases.
The user approved the artwork on 2026-10-13. Local outputs use `generated/review/` and
`generated/build/`.

Elsie has 62 sources: 36 portraits and twenty-six world strips. The
36-character collection has 4,813 sources and 19,252 variants. Z retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 59 prior
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Elsie
coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-178-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 59 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across eight frames, 242 component seeds select 465 pixels. Forehead, face,
neckline and the moving writing hand follow the palette. Gray hair, eyebrows,
lashes, eye details, cosmetics, earrings, pink gown, shoes, book and quill
stay original. The four world shades occur only in skin in these three pinned
sources. The book's cream pages and tan edges remain distinct from skin, as
do the quill's pink feather and pale shaft beside the writing hand.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the face, hand, quill, book and clothing boundaries.
The material test checks every pixel independently of component seeds, plus
211 distinct literal material landmarks, per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 178.

| Strip | Frames | Selected pixels per frame | Native durations |
| --- | --- | --- | --- |
| Write start South | 2 | 68, 57 | 0.125 per frame |
| Write loop South | 4 | 46, 53, 58, 58 | 0.1, 0.125, 0.1, 0.3 |
| Write end South | 2 | 57, 68 | 0.125, 0.1 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. The
cycle remains complex, South-only and unseated. The installed Elsie NPC
configuration matches the source byte for byte.

## Offline review

- [Five-choice sample summary](../../../generated/slice-179-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-179-review/index.html).

The summary samples start frame 1, loop frame 2 and end frame 1. Columns are
Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review includes every
source frame, Vanilla left and Debug Blue right, with full views and enlarged
details. No West mirrors apply. Images, exports and packages remain local and
ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 391 regular tests and the
release build passed through the Nix shell; 461 opt-in tests were ignored.
The selected local suite passed 252 tests, including all earlier Elsie
material checks, the new writing material test, historical character checks
and three GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted the loop face, start neckline, start hand or loop hand. Each generated
successfully and failed the intended pixel assertion. All three metadata
sidecars matched independent archive reads.

All 248 standalone Elsie variants passed strict recipe validation; 496 final
PNG/metadata files match the author candidate. All 19,252 combined variants
validated. The 590 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 178 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All five Chromium gallery pages loaded local images and links without overflow.
Eight cases include sixteen full-view bindings, enlarged details and fifteen
summary bindings; 1,788,960 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is 217,980 bytes;
its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-179-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`7f16b1853af4f45b1982cfa464068045c857f6b6f7c0edec78993bf7965fade2`.
The archive's native animator and NPC animate function passed forty
frame/palette observations and twenty complex transitions under Fabricator.
For each palette, start advances to loop, loop repeats, then advances to end,
and end completes back at start. Palette binding, frame phase, unseated status,
wrapper idempotence, cycle counters and Spring portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch and live rendering were not exercised. The desktop launcher
and installed game were not changed.

Evidence uses `tmp/slice-179-`; author records use
`tmp/elsie-spring-writing-author-`. Fresh exports are in
`extracted/elsie-spring-writing-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-178-build/characters/elsie`. Scope verification checks nine
expected paths, an empty Git index, unchanged historical pixel tests and ignored
game-derived artifacts.

## Remaining work

Elsie's remaining world inventory has 9 Spring, 32 Summer, 32 Autumn and
29 Winter strips. Next: Spring seated writing, six strips. The user approved
this slice; live-game verification remains pending.
