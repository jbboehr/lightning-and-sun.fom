# Josephine Autumn standing and walking

Following `f3a16e9`, this slice adds Josephine's six Autumn standing and walking
strips. Fifteen source frames plus five native West mirrors give twenty review
cases. The user approved the artwork on 2026-10-08. Outputs use `generated/review/` and
`generated/build/`.

Josephine now has 107 sources: 32 portraits, all 36 Spring world strips, all 33
Summer world strips and six Autumn world strips. The 36-character collection
has 4,568 sources and 18,272 variants. P retains Vanilla, Debug Blue, Hayden,
Ryis and Seridia choices with portrait/world synchronization. Only six regions
and registry entries are added. Source colors, groups, target mappings, sets and
runtime code remain unchanged. Historical pixel tests retain their expectations;
only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-124-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 101 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 358 component seeds select
634 skin pixels across fifteen source frames, including the face, neck, moving
hands and exposed ankles. These shades belong exclusively to skin here. Brown
hair, eye details, blue earrings, orange clothing and shoes stay original.
The clothing's `#9E3D25`, `#D8753A`, `#F5A95B`, `#FCD288` and `#FFFAD7` shades,
the earrings' `#254277`, `#3762B2` and `#5D95CE`, and the shoes' `#AC4134`,
`#D65A41` and `#FF914C` remain unchanged.

Every source frame was inspected in five-choice contact sheets, with source
grids around the skin, clothing and footwear boundaries. The material test
independently checks every output pixel against a literal shade oracle,
100 fixed landmarks and per-frame selected counts, plus alpha and metadata.
Landmarks cover all six strips, their visible skin and ankles, and neighboring
hair, eyes, earrings, clothing and shoes. The earlier 101 sources are compared
with the fixed slice 124 baseline. The profile contains component seeds and
pinned source hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Each standing strip has one frame with no explicit duration or frame count;
each walking strip has four frames at 0.15 seconds per frame. The native idle
and walk cycles remain linear, with North, South and East assets and mirrored
West rendering. Both keep their speaking-turn behavior; walking retains its
background/speaking pause transition to idle.

## Offline review

- [Five-choice sample summary](../../generated/slice-125-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-125-review/index.html).

The summary samples South standing frame 1, East walking frame 2 and North
walking frame 2. The full gallery includes all fifteen source frames and five
West mirrors, with Vanilla left and Debug Blue right, full views and enlarged
face, hand and ankle details. Game-derived images and packages remain local and
ignored by Git.

## Verification

Formatting, Clippy, all 337 regular tests and the release build passed through
the Nix shell. The regular run left 407 opt-in tests ignored; the selected local
suite passed 195 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted the face, left hand, right hand and ankle
regions; each generated successfully and failed the intended material assertion.
All six raw metadata sidecars matched independent original-archive reads.

All 428 standalone Josephine variants validated, and their 856 PNG/metadata
files matched the author candidate. All 18,272 combined variants validated.
The 1,010 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 124; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The ten Chromium pages loaded all local links and images without overflow.
The twenty cases have 40 full-view bindings plus enlarged details and fifteen
summary bindings; 3,862,560 rendered pixels match their source crops. Full-view
bounds include every nontransparent pixel. The complete review is 407,515 bytes.
The short review/build aliases point to this slice, and the review loads through
its short alias.

The isolated MOMI installation passed in `tmp/slice-125-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `38bf82e2728d1effe1926682c323bdff8ceca6e67cf0bad206a9439de9e9ea07`.
The archive's native animator and NPC animate function passed one hundred
frame/palette observations and forty linear completions under Fabricator.
Checks preserve frame phase, wrapper idempotence, cycle counters, West
mirroring and Autumn portrait synchronization. There are no final-frame holds
or complex transitions here. Engine services were simulated and cycles selected
directly. This does not exercise natural gameplay dispatch, speaking/background
transitions or live rendering. The desktop launcher and installed game were
not changed.

Evidence uses `tmp/slice-125-`; author records use
`tmp/josephine-autumn-movement-author-`. Fresh exports are in
`extracted/josephine-autumn-movement-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-124-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Spring and Summer world folders are complete. Autumn has 27 strips left and
Winter has 33. Next: Autumn blinking and seated actions. Inventory is recorded
in `tmp/slice-125-remainder.json`. Natural gameplay dispatch, speaking/background
transitions and live rendering remain pending.
