# Elsie Spring singing

Following `4a4b9e7`, this slice adds Elsie's Spring singing start, loop
and end, completing her Spring world folder. The three South-facing strips
contain twenty-five frames. The user approved the artwork on 2026-10-13. Local outputs use
`generated/review/` and `generated/build/`.

Elsie has 71 sources: 36 portraits and thirty-five world strips. The
36-character collection has 4,822 sources and 19,288 variants. Z retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 68 prior
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Elsie
coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-180-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 68 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across twenty-five frames, 825 component seeds select 1,605 pixels. Forehead,
face, neckline and both moving hands follow the palette. Gray hair, eyebrows,
lashes, eye details, mouth interiors, cosmetics, earrings, pink gown, shoes
and the blue, pink and green music notes retain their original colors.
The four world shades occur only in skin in these three pinned sources.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the face, mouth, hands, neckline, clothing and floating
music notes. The material test checks every pixel independently of component
seeds, plus 601 distinct literal material landmarks, per-frame counts,
transparency, metadata, unchanged profile roles and prior output bytes against
slice 180.

| Strip | Frames | Selected pixels per frame | Native durations |
| --- | --- | --- | --- |
| Sing start South | 1 | 77 | Omitted in source metadata |
| Sing loop South | 23 | 61, 60, 60, 60, 60, 64, 60, 60, 60, 62, 60, 61, 66, 76, 76, 78, 61, 61, 61, 61, 61, 61, 61 | 0.1 per frame |
| Sing end South | 1 | 77 | Omitted in source metadata |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. The
cycle remains complex, South-only and standing, with `on_pause_speaking`
set to `idle`. The installed Elsie NPC configuration matches the source
byte for byte. The offline animator harness supplies a one-second default for
the omitted start/end duration; live engine timing and speech-pause dispatch
remain unverified.

## Offline review

- [Five-choice sample summary](../../../generated/slice-181-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-181-review/index.html).

The summary samples start frame 1 and loop frames 2 and 14. Columns are
Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review includes all
twenty-five frames, Vanilla left and Debug Blue right, with full views and
enlarged details. Full crops are wider to include every floating music note.
The enlarged face/hand crops can cut off distant notes; the adjacent full
views retain them. Images, exports and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 393 regular tests and the
release build passed through the Nix shell; 463 opt-in tests were ignored.
The selected local suite passed 254 tests, including all earlier Elsie
material checks, the new singing material test, historical character checks
and three GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask
mutations omitted the start face, neckline, viewer-left hand or viewer-right
hand. Each generated successfully and failed the intended pixel assertion.
All three metadata sidecars matched independent archive reads.

All 284 standalone Elsie variants passed strict recipe validation; 568 final
PNG/metadata files match the author candidate. All 19,288 combined variants
validated. The 680 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 180 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty-five cases include fifty full-view bindings, enlarged details and fifteen
summary bindings; 5,274,600 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is
513,809 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-181-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`473527d93d4b38db3f1051c0b751c05473f8f3cca49eb5a54bdc925528da1c03`.
The archive's native animator and NPC animate function passed 125 frame/palette
observations and twenty complex transitions under Fabricator. For each palette,
start advances to loop, loop repeats, then advances to end, and end completes
back at start. Palette binding, frame phase, standing status, wrapper
idempotence, cycle counters and Spring portrait synchronization were preserved.
Engine services are simulated and cycles selected directly. Natural gameplay
dispatch, speech interruption and live rendering were not exercised. The desktop
launcher and installed game were not changed.

Evidence uses `tmp/slice-181-`; author records use
`tmp/elsie-spring-singing-author-`. Fresh exports are in
`extracted/elsie-spring-singing-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-180-build/characters/elsie`. Scope verification checks nine
expected paths, an empty Git index, unchanged historical pixel tests and ignored
game-derived artifacts.

## Remaining work

All 35 strips in Elsie's Spring world folder are covered. Her remaining world
inventory has 32 Summer, 32 Autumn and 29 Winter strips. Next: Summer standing
and walking, six strips. Live-game verification remains pending for this slice.
