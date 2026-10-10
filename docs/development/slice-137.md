# Josephine Winter polishing

Following `49d55b7`, this slice adds Josephine's three Winter polishing
start/loop/end strips. Its nine source frames and nine native West mirrors give
18 review cases. The user approved the artwork on 2026-10-09. Outputs use `generated/review/` and
`generated/build/`.

Josephine now has 160 sources: 32 portraits, all 36 Spring world strips, all 33
Summer world strips, all 33 Autumn world strips and 26 Winter world strips.
The 36-character collection has 4,621 sources and 18,484 variants. P retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. Only three regions and registry entries are added. Source
colors, groups, target mappings, sets and
runtime code remain unchanged. Historical pixel tests retain their expectations;
only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-136-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed game
version. All 157 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 145 component seeds select
393 skin pixels across nine source frames. These shades belong exclusively to
skin here. Exposed hands in start/end follow the palette; the loop's hands are
hidden behind the cloth and glass. Brown hair, eyes, gold earrings, pink clothing,
dark leggings, boots, cloth and glass stay original. The glass's
`#85CAEA` and cloth's `#547BB0` are original blues, so their preservation is
checked explicitly as well as visually. Winter clothing covers the ankles. The
portrait skin shade `#3A1D24` is dark clothing here and stays outside the mask.

Every source frame was inspected in five-choice contact sheets, with source
grids around exposed hands, cloth, glass, clothing and boots. The material test
independently checks every output pixel against a literal shade oracle,
56 fixed landmarks and per-frame selected counts, plus alpha and metadata.
The earlier 157 sources are compared with the fixed slice 136 baseline.
The profile contains component
seeds and pinned source hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Start has three frames at 0.125 seconds each; loop has four frames at
0.1/0.125/0.1/0.225 seconds; end has two at 0.125/0.1 seconds. The native polishing
cycle remains an East-facing complex start/loop/end sequence with West mirrors.

## Offline review

- [Five-choice sample summary](../../../generated/slice-137-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-137-review/index.html).

The summary samples start frame 1, loop frame 3 and end frame 1. Its columns are
Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full gallery includes all 18
cases, with Vanilla left and Debug Blue right, full character views and enlarged
face, hands, cloth, glass and clothing. Game-derived images and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy, all 349 regular tests and the release build passed through
the Nix shell. The regular run left 419 opt-in tests ignored; the selected local
suite passed 207 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted the face, start hand or end hand, or leaked
recoloring into dark leggings through a `#3A1D24` seed. Each generated
successfully and failed the intended material assertion. Three raw metadata
sidecars matched independent original-archive reads.

All 640 standalone Josephine variants validated, and their 1,280 PNG/metadata
files matched the author candidate. All 18,484 combined variants validated.
The 1,570 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 136; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The eight Chromium pages loaded all local links and images without overflow.
The 18 cases have 36 full-view bindings plus enlarged details and fifteen
summary bindings; 3,642,912 rendered pixels match their source crops. Full-view
bounds include every nontransparent pixel, including the native West mirrors.
The complete review is 360,868 bytes. The short review/build aliases point to this
slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-137-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `cc745974f2e1cec572ce432e922955e6f73565c12bdc682f9c4849ff806fb030`.
The archive's native animator and NPC animate function passed ninety
frame/palette observations and forty complex stage transitions under Fabricator.
Checks preserve frame phase, West mirroring, wrapper idempotence, cycle counters
and Winter portrait synchronization. There are no linear completions or
final-frame holds here. Engine services were simulated and cycles selected
directly. This does not exercise natural gameplay dispatch or live rendering.
The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-137-`; author records use
`tmp/josephine-winter-polishing-author-`. Fresh exports are in
`extracted/josephine-winter-polishing-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-136-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Spring, Summer and Autumn world folders are complete. Winter has seven strips
left: singing and sweeping start/loop/end, plus chopping North.
Next: Winter singing. Inventory is recorded in `tmp/slice-137-remainder.json`.
Natural gameplay dispatch and live rendering remain pending.
