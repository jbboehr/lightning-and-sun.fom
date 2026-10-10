# Josephine Winter standing writing

Following `a368fe6`, this slice adds Josephine's three Winter writing
start/loop/end strips. All eight frames face South; there are no native West
mirrors. The user approved the artwork on 2026-10-09. Outputs use `generated/review/` and
`generated/build/`.

Josephine now has 157 sources: 32 portraits, all 36 Spring world strips, all 33
Summer world strips, all 33 Autumn world strips and 23 Winter world strips.
The 36-character collection has 4,618 sources and 18,472 variants. P retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. Only three regions and registry entries are added. Source
colors, groups, target mappings, sets and
runtime code remain unchanged. Historical pixel tests retain their expectations;
only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-135-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed game
version. All 154 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 165 component seeds select
405 skin pixels across eight source frames. These shades belong exclusively to
skin here, including the writing hand and the hand beneath the board. Brown
hair, eyes, gold earrings, pink clothing, dark leggings, boots, board and pen
stay original. The board uses `#B28159`; the pen's orange `#D36A0E` and white
`#F5F5F5` details stay original, as do its dark `#663409` tip and the board
edge detail `#C3D1DD`. Winter clothing covers the ankles. The portrait skin
shade `#3A1D24` is dark clothing here and stays outside the mask.

Every source frame was inspected in five-choice contact sheets, with source
grids around both hands, board, pen, clothing and boots. The material test
independently checks every output pixel against a literal shade oracle,
85 fixed landmarks and per-frame selected counts, plus alpha and metadata.
The earlier 154 sources are compared with the fixed slice 135 baseline. The
profile contains component seeds and pinned source hashes, without new runtime
rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Start has two frames at 0.125 seconds each; loop has four frames at
0.1/0.125/0.1/0.3 seconds; end has two at 0.125/0.1 seconds. The native writing
cycle remains a South-facing complex start/loop/end sequence.

## Offline review

- [Five-choice sample summary](../../../generated/slice-136-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-136-review/index.html).

The summary samples start frame 1, loop frame 3 and end frame 1. Its columns are
Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full gallery includes all eight
frames, with Vanilla left and Debug Blue right, full character views and enlarged
face, hands, writing tools and clothing. Game-derived images and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy, all 348 regular tests and the release build passed through
the Nix shell. The regular run left 418 opt-in tests ignored; the selected local
suite passed 206 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted the face, writing hand or hand beneath the board,
or leaked recoloring into dark leggings through a `#3A1D24` seed. Each generated
successfully and failed the intended material assertion. Three raw metadata
sidecars matched independent original-archive reads.

All 628 standalone Josephine variants validated, and their 1,256 PNG/metadata
files matched the author candidate. All 18,472 combined variants validated.
The 1,540 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 135; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The five Chromium pages loaded all local links and images without overflow.
The eight cases have sixteen full-view bindings plus enlarged details and
fifteen summary bindings; 1,879,072 rendered pixels match their source crops.
Full-view bounds include every nontransparent pixel. The complete review is
198,014 bytes. The short review/build aliases point to this slice, and the
review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-136-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `d199e14236ca474c15def5dc7f41db288f2fd0a9bc92d61e8dbb5cb4d33bf019`.
The archive's native animator and NPC animate function passed forty frame/palette
observations and twenty complex stage transitions under Fabricator. Checks
preserve frame phase, wrapper idempotence, cycle counters and Winter portrait
synchronization. There are no linear completions or final-frame holds here.
Engine services were simulated and cycles selected directly. This does not
exercise natural gameplay dispatch or live rendering. The desktop launcher and
installed game were not changed.

Evidence uses `tmp/slice-136-`; author records use
`tmp/josephine-winter-writing-author-`. Fresh exports are in
`extracted/josephine-winter-writing-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-135-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Spring, Summer and Autumn world folders are complete. Winter has ten strips
left: polishing, singing and sweeping start/loop/end, plus chopping North.
Next: Winter polishing. Inventory is recorded in
`tmp/slice-136-remainder.json`. Natural gameplay dispatch and live rendering
remain pending.
