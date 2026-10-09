# Josephine Autumn blinking and seated actions

Following `da3b638`, this slice adds Josephine's eleven Autumn blinking, sitting,
eating and drinking strips. There are 31 source frames plus twelve native West
mirrors: 43 review cases. The user approved the artwork on 2026-10-08. Outputs use
`generated/review/` and `generated/build/`.

Josephine now has 118 sources: 32 portraits, all 36 Spring world strips, all 33
Summer world strips and 17 Autumn world strips. The 36-character collection has
4,579 sources and 18,316 variants. P retains Vanilla, Debug Blue, Hayden, Ryis
and Seridia choices with portrait/world synchronization. Only eleven regions
and registry entries are added. Source colors, groups, target mappings, sets and
runtime code remain unchanged. Historical pixel tests retain their expectations;
only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-125-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 107 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 699 component seeds select
1,370 skin pixels across 31 source frames, including eyelids, moving hands and
exposed standing ankles. These shades belong exclusively to skin here. Brown
hair, lashes, eye details, blue earrings, orange clothing and shoes stay original.
The open mouths retain red `#9E2626` and dark `#410808` interiors. Clothing shades
including `#D8753A` and `#F5A95B`, and shoe highlight `#FF914C`, remain unchanged.

Every source frame was inspected in five-choice contact sheets, with source
grids around the closed eyelids, mouth interiors, hands and clothing. The material
test independently checks every output pixel against a literal shade oracle,
74 fixed landmarks and per-frame selected counts, plus alpha and metadata.
Landmarks span all eleven strips and include closed eyelids, lashes, moving
hands, standing ankles, mouth interiors and neighboring materials. The earlier
107 sources are compared with the fixed slice 125 baseline. The profile contains
component seeds and pinned source hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Each blink has three frames at 0.075, 0.125 and 0.075 seconds. Each drink has
three frames at one second per frame; North eating does too. East and South
eating retain five frames at 0.125, 0.15, 0.175, 0.125 and 0.6 seconds. Sitting
has one frame per direction with no explicit duration or frame count. Native
cycles remain linear; sitting, eating and drinking retain their seated flag.
Eating and drinking retain the native `[240, 360]` final-frame hold range.

## Offline review

- [Five-choice sample summary](../../generated/slice-126-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-126-review/index.html).

The summary samples South blinking frame 2, East eating frame 2 and North
drinking frame 2. The full gallery includes all 31 source frames and twelve West
mirrors, with Vanilla left and Debug Blue right, full views and enlarged face,
hand and ankle details. Game-derived images and packages remain local and
ignored by Git.

## Verification

Formatting, Clippy, all 338 regular tests and the release build passed through
the Nix shell. The regular run left 408 opt-in tests ignored; the selected local
suite passed 196 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted the face, moving hand, ankle and closed-eyelid
regions; each generated successfully and failed the intended material assertion.
All eleven raw metadata sidecars matched independent original-archive reads.

All 472 standalone Josephine variants validated, and their 944 PNG/metadata
files matched the author candidate. All 18,316 combined variants validated.
The 1,070 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 125; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The twenty Chromium pages loaded all local links and images without overflow.
The 43 cases have 86 full-view bindings plus enlarged details and fifteen
summary bindings; 7,766,304 rendered pixels match their source crops. Full-view
bounds include every nontransparent pixel. The complete review is 775,830 bytes.
The short review/build aliases point to this slice, and the review loads through
its short alias.

The isolated MOMI installation passed in `tmp/slice-126-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `a209b61a31339fb22333c63ea8f993fa9a3e97775bc86d42c43f3446540a3248`.
The archive's native animator and NPC animate function passed 215 frame/palette
observations, 75 linear completions and 80 final-frame hold checks under
Fabricator. Checks preserve frame phase, wrapper idempotence, cycle counters,
the seated flag, West mirroring and Autumn portrait synchronization. There are
no complex transitions here. Engine services were simulated and cycles selected
directly. This does not exercise natural gameplay dispatch, eating/drinking
context, actual seated behavior or live rendering. The desktop launcher and
installed game were not changed.

Evidence uses `tmp/slice-126-`; author records use
`tmp/josephine-autumn-actions-author-`. Fresh exports are in
`extracted/josephine-autumn-actions-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-125-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Spring and Summer world folders are complete. Autumn has sixteen strips left
and Winter has 33. Next: Autumn general actions. Inventory is recorded in
`tmp/slice-126-remainder.json`. Natural gameplay dispatch, eating/drinking
context, seated behavior and live rendering remain pending.
