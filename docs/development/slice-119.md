# Josephine Summer general actions

Following `0b7aafe`, this slice adds Josephine's three Summer general-action
strips. There are 21 source frames and seven native West mirrors: 28 review
cases. The user approved the artwork on 2026-10-08. Outputs use `generated/review/` and
`generated/build/`.

Josephine now has 88 sources: 32 portraits, all 36 Spring world strips and 20
Summer world strips. The 36-character collection has 4,549 sources and 18,196
variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Source colors, groups, target mappings, sets and runtime code remain
unchanged. Historical pixel tests retain their expectations; only the central
Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-118-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 85 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 561 component seeds select
918 skin pixels across 21 source frames. These shades belong exclusively to
skin here. Face, neck, moving hands and exposed ankles follow the palette.
Brown hair, eye details, earrings, pink flower, cream and peach clothing and
yellow shoes stay original.

Every source frame was inspected in five-choice contact sheets, with source
grids around extended hands, face, clothing and ankles. The material test
independently checks every output pixel against a literal shade oracle,
89 fixed landmarks and per-frame selected counts, plus alpha and metadata.
The earlier 85 sources are compared with the fixed slice 118 baseline. The
profile contains component seeds and pinned source hashes, without new runtime
rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Each direction has seven frames timed at
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`. The native `action` cycle remains
linear, defaults to North, mirrors East for West and retains its final-frame
hold range `[240, 360]`. Its speaking pause transition to `idle` is unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-119-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-119-review/index.html).

The summary samples action East frame 2, North frame 6 and South frame 3.
The full gallery includes all 28 cases with Vanilla left and Debug Blue right,
full views and enlarged face, hands and ankles. West views mirror the East
source exactly as the native animator does. Game-derived images and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy, all 331 regular tests and the release build passed through
the Nix shell. The regular run left 401 opt-in tests ignored; the selected local
suite passed 189 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted face, left hand, right hand and ankle regions;
each generated successfully and failed the intended material assertion. Three
raw metadata sidecars matched independent original-archive reads.

All 352 standalone Josephine variants validated, and their 704 PNG/metadata
files matched the author candidate. All 18,196 combined variants validated.
The 850 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 118; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The ten Chromium pages loaded all local links and images without overflow.
The 28 cases have 56 full-view bindings plus enlarged details and fifteen
summary bindings; 5,220,384 rendered pixels match their source crops. The
complete review is 499,587 bytes. The short review/build aliases point to this
slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-119-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `523cbc0ae7b36febfb34a765bda13b7842d71bc312755c450e05b5ece2b61334`.
The archive's native animator and NPC animate function passed 140 frame/palette
observations, twenty linear completions and forty final-frame hold boundary
checks under Fabricator, including West mirroring. Engine services were
simulated and cycles selected directly. This does not exercise natural gameplay
dispatch, speaking transitions or live rendering. The desktop launcher and
installed game were not changed.

Evidence uses `tmp/slice-119-`; author records use
`tmp/josephine-summer-general-author-`. Fresh exports are in
`extracted/josephine-summer-general-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-118-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Summer has thirteen strips left; Autumn and Winter each have 33. Next: Summer
writing (start, loop and end). Inventory is recorded in
`tmp/slice-119-remainder.json`. Natural gameplay dispatch, speaking transitions
and live rendering remain pending.
