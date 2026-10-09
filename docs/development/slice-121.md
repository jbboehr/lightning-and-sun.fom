# Josephine Summer polishing

Following `3f4acf1`, this slice adds Josephine's three Summer polishing
start/loop/end strips. Nine source frames and nine native West mirrors give
eighteen review cases. The user approved the artwork on 2026-10-08. Outputs use
`generated/review/` and `generated/build/`.

Josephine now has 94 sources: 32 portraits, all 36 Spring world strips and 26
Summer world strips. The 36-character collection has 4,555 sources and 18,220
variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Source colors, groups, target mappings, sets and runtime code remain
unchanged. Historical pixel tests retain their expectations; only the central
Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-120-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 91 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 243 component seeds select
477 skin pixels across nine source frames. These shades belong exclusively to
skin here. Exposed hands in start/end and ankles change; the loop's hands are
hidden behind the cloth and glass. Brown hair, eyes, earrings, pink flower,
cream and peach clothing, yellow shoes, polishing cloth and glass stay original.
The blue glass includes `#85CAEA` and the cloth includes `#547BB0`; neither
belongs to the skin ramp. Their original blue shades can make a Debug Blue
comparison less obvious, so they have explicit preservation checks.

Every source frame was inspected in five-choice contact sheets, with source
grids around the exposed hands, cloth, glass, clothing and ankles. The material
test independently checks every output pixel against a literal shade oracle,
56 fixed landmarks and per-frame selected counts, plus alpha and metadata.
Landmarks cover face, ankles, hair and clothing in every frame, glass and cloth
when present, and light/deep hand shades in the exposed poses. The earlier 91
sources are compared with the fixed slice 120 baseline. The profile contains
component seeds and pinned source hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Start has three frames at 0.125 seconds each; loop has four frames at
0.1/0.125/0.1/0.225 seconds; end has two at 0.125/0.1 seconds. The native
polishing cycle remains an East-facing complex start/loop/end sequence; West
uses its native horizontal mirror.

## Offline review

- [Five-choice sample summary](../../generated/slice-121-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-121-review/index.html).

The summary samples start frame 1, loop frame 3 and end frame 1. The full gallery
includes all nine source frames and their West mirrors, with Vanilla left and
Debug Blue right, full character views and enlarged face, hands, cloth, glass
and ankles. Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy, all 333 regular tests and the release build passed through
the Nix shell. The regular run left 403 opt-in tests ignored; the selected local
suite passed 191 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted face, start hand, end hands and ankle regions;
each generated successfully and failed the intended material assertion. Three
raw metadata sidecars matched independent original-archive reads.

All 376 standalone Josephine variants validated, and their 752 PNG/metadata
files matched the author candidate. All 18,220 combined variants validated.
The 910 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 120; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The eight Chromium pages loaded all local links and images without overflow.
The eighteen cases have 36 full-view bindings plus enlarged details and fifteen
summary bindings; 3,642,912 rendered pixels match their source crops. The
complete review is 356,564 bytes. The short review/build aliases point to this
slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-121-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `a79d11f5a6cdcf35b2550e92685ee0a0186f8d04f40025e743bbe93793cbb837`.
The archive's native animator and NPC animate function passed ninety
frame/palette observations and forty complex stage transitions under Fabricator.
Checks preserve frame phase, West mirroring, wrapper idempotence, cycle counters
and Summer portrait synchronization. There are no linear completions or
final-frame holds here. Engine services were simulated and cycles selected
directly. This does not exercise natural gameplay dispatch or live rendering.
The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-121-`; author records use
`tmp/josephine-summer-polishing-author-`. Fresh exports are in
`extracted/josephine-summer-polishing-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-120-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Summer has seven strips left: singing and sweeping start/loop/end, plus chopping
North. Autumn and Winter each have 33. Next: Summer singing. Inventory is
recorded in `tmp/slice-121-remainder.json`. Natural gameplay dispatch and live
rendering remain pending.
