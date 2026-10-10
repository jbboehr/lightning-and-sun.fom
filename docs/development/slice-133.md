# Josephine Winter standing and walking

Following [slice 132](slice-132.md), this slice adds Josephine's six Winter
standing and walking strips. Fifteen source frames plus five native West
mirrors give twenty review cases. This work started from the uncommitted slice
132 state. The user approved the Winter movement artwork on 2026-10-09.
The earlier Autumn sweeping slice has no separately recorded art approval.
Outputs use `generated/review/` and `generated/build/`.

Josephine now has 140 sources: 32 portraits, all 36 Spring world strips, all 33
Summer world strips, all 33 Autumn world strips and six Winter world strips.
The 36-character collection has 4,601 sources and 18,404 variants. P retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. Only six regions and registry entries are added. Source colors,
groups, target mappings, sets and runtime code remain unchanged. Historical pixel
tests retain their expectations; only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-132-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed game
version. All 134 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 243 component seeds select
540 skin pixels across fifteen source frames, including the face, neck and
moving hands. These shades belong exclusively to skin here. Brown hair, eye
details, gold earrings, pink clothing, dark leggings and boots stay original.
Winter clothing covers the ankles. In particular, the portrait skin shade
`#3A1D24` appears on dark clothing here and must remain outside the skin mask.
The clothing's `#A15663`, `#E18A8A` and `#FAC0B5`, the earrings' `#DD934C`,
`#EDC57E` and `#FFEAB3`, and the boots' `#7F213B` and `#EE678F` stay original.

Every source frame was inspected in five-choice contact sheets, with source
grids around the skin, clothing and footwear boundaries. The material test
independently checks every output pixel against a literal shade oracle,
102 fixed landmarks and per-frame selected counts, plus alpha and metadata.
Landmarks cover all six strips, their visible skin and protected leggings,
and neighboring hair, eyes, earrings, clothing and boots. The earlier 134
sources are compared with the fixed slice 132 baseline. The profile contains
component seeds and pinned source hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Each standing strip has one frame with no explicit duration or frame count;
each walking strip has four frames at 0.15 seconds per frame. The native idle
and walk cycles remain linear, with North, South and East assets and mirrored
West rendering. Both keep their speaking-turn behavior; walking retains its
background/speaking pause transition to idle.

## Offline review

- [Five-choice sample summary](../../../generated/slice-133-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-133-review/index.html).

The summary samples South standing frame 1, East walking frame 2 and North
walking frame 2. The full gallery includes all fifteen source frames and five
West mirrors, with Vanilla left and Debug Blue right, full views and enlarged
face, hand and clothing details. Game-derived images and packages remain local
and ignored by Git. The preceding Autumn sweeping review remains available at
`generated/slice-132-review/`.

## Verification

Formatting, Clippy, all 345 regular tests and the release build passed through
the Nix shell. The regular run left 415 opt-in tests ignored; the selected local
suite passed 203 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations omitted the face, left hand or right hand, or leaked the recoloring into
the leggings through a #3A1D24 seed. Each generated successfully and failed the
intended material assertion.
All six raw metadata sidecars matched independent original-archive reads.

All 560 standalone Josephine variants validated, and their 1,120 PNG/metadata
files matched the author candidate. All 18,404 combined variants validated.
The 1,340 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 132; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The ten Chromium pages loaded all local links and images without overflow.
The twenty cases have 40 full-view bindings plus enlarged details and fifteen
summary bindings; 3,862,560 rendered pixels match their source crops. Full-view
bounds include every nontransparent pixel. The complete review is 409,112 bytes.
The short review/build aliases point to this slice, and the review loads through
its short alias.

The isolated MOMI installation passed in `tmp/slice-133-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `d30ae52d70cba64b4d692be985e8585bb599b712e51c594909d9d54e21dcb68b`.
The archive's native animator and NPC animate function passed one hundred
frame/palette observations and forty linear completions under Fabricator.
Checks preserve frame phase, wrapper idempotence, cycle counters, West
mirroring and Winter portrait synchronization. There are no final-frame holds
or complex transitions here. Engine services were simulated and cycles selected
directly. This does not exercise natural gameplay dispatch, speaking/background
transitions or live rendering. The desktop launcher and installed game were
not changed.

Evidence uses `tmp/slice-133-`; author records use
`tmp/josephine-winter-movement-author-`. Fresh exports are in
`extracted/josephine-winter-movement-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-132-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the twelve expected paths across slices 132 and 133,
unchanged historical tests and intact slice 132 records. Starting working-tree
snapshots in `tmp/slice-133-start/` establish the uncommitted prior state.

## Remaining work

Spring, Summer and Autumn world folders are complete. Winter has 27 strips left.
Next: Winter blinking and seated actions. Inventory is recorded in
`tmp/slice-133-remainder.json`. Natural gameplay dispatch, speaking/background
transitions and live rendering remain pending.
