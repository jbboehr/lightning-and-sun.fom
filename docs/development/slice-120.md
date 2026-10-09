# Josephine Summer standing writing

Following `832c622`, this slice adds Josephine's three Summer writing
start/loop/end strips. All eight frames face South; there are no native West
mirrors. The user approved the artwork on 2026-10-08. Outputs use `generated/review/` and
`generated/build/`.

Josephine now has 91 sources: 32 portraits, all 36 Spring world strips and 23
Summer world strips. The 36-character collection has 4,552 sources and 18,208
variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Source colors, groups, target mappings, sets and runtime code remain
unchanged. Historical pixel tests retain their expectations; only the central
Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-119-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 88 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 223 component seeds select
466 skin pixels across eight source frames. These shades belong exclusively to
skin here, including the writing hand, the hand beneath the board and exposed
ankles. Brown hair, eyes, earrings, pink flower, cream and peach clothing,
yellow shoes, board and pen stay original. The board uses `#B28159`; the pen's
orange `#D36A0E` and white `#F5F5F5` details stay original.

Every source frame was inspected in five-choice contact sheets, with source
grids around both hands, board, pen, clothing and ankles. The material test
independently checks every output pixel against a literal shade oracle,
70 fixed landmarks and per-frame selected counts, plus alpha and metadata.
The earlier 88 sources are compared with the fixed slice 119 baseline. The
profile contains component seeds and pinned source hashes, without new runtime
rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Start has two frames at 0.125 seconds each; loop has four frames at
0.1/0.125/0.1/0.3 seconds; end has two at 0.125/0.1 seconds. The native writing
cycle remains a South-facing complex start/loop/end sequence.

## Offline review

- [Five-choice sample summary](../../generated/slice-120-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-120-review/index.html).

The summary samples start frame 1, loop frame 3 and end frame 1. The full gallery
includes all eight frames, with Vanilla left and Debug Blue right, full character
views and enlarged face, hands, writing tools and ankles. Game-derived images
and packages remain local and ignored by Git.

## Verification

Formatting, Clippy, all 332 regular tests and the release build passed through
the Nix shell. The regular run left 402 opt-in tests ignored; the selected local
suite passed 190 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted face, writing hand, hand beneath the board and
ankle regions; each generated successfully and failed the intended material
assertion. Three raw metadata sidecars matched independent original-archive reads.

All 364 standalone Josephine variants validated, and their 728 PNG/metadata
files matched the author candidate. All 18,208 combined variants validated.
The 880 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 119; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The five Chromium pages loaded all local links and images without overflow.
The eight cases have sixteen full-view bindings plus enlarged details and
fifteen summary bindings; 1,879,072 rendered pixels match their source crops.
The complete review is 192,341 bytes. The short review/build aliases point to
this slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-120-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `eac2f548bd62fdf90de238e1438e951bac8e45d2d88323e9c0214ee447833149`.
The archive's native animator and NPC animate function passed forty frame/palette
observations and twenty complex stage transitions under Fabricator. Checks
preserve frame phase, wrapper idempotence, cycle counters and Summer portrait
synchronization. There are no linear completions or final-frame holds here.
Engine services were simulated and cycles selected directly. This does not
exercise natural gameplay dispatch or live rendering. The desktop launcher and
installed game were not changed.

Evidence uses `tmp/slice-120-`; author records use
`tmp/josephine-summer-writing-author-`. Fresh exports are in
`extracted/josephine-summer-writing-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-119-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Summer has ten strips left: polishing, singing and sweeping start/loop/end,
plus chopping North. Autumn and Winter each have 33. Next: Summer polishing.
Inventory is recorded in `tmp/slice-120-remainder.json`. Natural gameplay
dispatch and live rendering remain pending.
