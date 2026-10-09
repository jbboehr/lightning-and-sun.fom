# Josephine Summer singing

Following `04a2695`, this slice adds Josephine's three Summer singing
start/loop/end strips. All fourteen frames face South; there are no native West
mirrors. The user approved the artwork on 2026-10-08. Outputs use `generated/review/` and
`generated/build/`.

Josephine now has 97 sources: 32 portraits, all 36 Spring world strips and 29
Summer world strips. The 36-character collection has 4,558 sources and 18,232
variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Source colors, groups, target mappings, sets and runtime code remain
unchanged. Historical pixel tests retain their expectations; only the central
Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-121-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 94 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 592 component seeds select
1,064 skin pixels across fourteen source frames, including both moving hands
and exposed ankles. These shades belong exclusively to skin here. Brown hair,
eye details, earrings, pink flower, cream and peach clothing, yellow shoes,
mouth interiors and floating musical notes stay original. The red `#9E2626`
and dark `#410808` mouth pixels remain unchanged, as do the notes' blue
`#599DFD`, pink `#FF6888` and green `#95DA53` shades.

Every source frame was inspected in five-choice contact sheets, with source
grids around the mouth, hands, clothing, ankles and notes. The material test
independently checks every output pixel against a literal shade oracle,
52 fixed landmarks and per-frame selected counts, plus alpha and metadata.
Both hands have a landmark in every frame; additional landmarks cover neighboring
materials in each stage, three open-mouth poses and each note color. The earlier
94 sources are compared with the fixed slice 121 baseline. The profile contains
component seeds and pinned source hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Start and end each have two frames; loop has ten. Every frame lasts 0.1 seconds.
The native singing cycle remains a South-facing complex start/loop/end sequence.
Its existing speaking-pause behavior selects idle; that dispatch is unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-122-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-122-review/index.html).

The summary samples start frame 1, loop frame 3 and end frame 1. The full gallery
includes all fourteen frames, with Vanilla left and Debug Blue right, full views
and enlarged face, hands and ankles. The wider full-view crop includes every
floating note. Close-ups focus on the face, hands and ankles, excluding cut-off
note fragments. Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy, all 334 regular tests and the release build passed through
the Nix shell. The regular run left 404 opt-in tests ignored; the selected local
suite passed 192 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted face, left hand, right hand and ankle regions;
each generated successfully and failed the intended material assertion. Three
raw metadata sidecars matched independent original-archive reads.

All 388 standalone Josephine variants validated, and their 776 PNG/metadata
files matched the author candidate. All 18,232 combined variants validated.
The 940 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 121; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The seven Chromium pages loaded all local links and images without overflow.
The fourteen cases have 28 full-view bindings plus enlarged details and fifteen
summary bindings; 3,064,000 rendered pixels match their source crops. Full-view
bounds include every nontransparent pixel. The complete review is 289,279 bytes.
The short review/build aliases point to this slice, and the review loads through
its short alias.

The isolated MOMI installation passed in `tmp/slice-122-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `479eca6ba7e554da84d4f4cf48fd8201e6dd43443cfd4efeb57017ee33c95d56`.
The archive's native animator and NPC animate function passed seventy
frame/palette observations and twenty complex stage transitions under Fabricator.
Checks preserve frame phase, wrapper idempotence, cycle counters and Summer
portrait synchronization. There are no linear completions or final-frame holds
here. Engine services were simulated and cycles selected directly. This does
not exercise natural gameplay dispatch, speaking transitions or live rendering.
The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-122-`; author records use
`tmp/josephine-summer-singing-author-`. Fresh exports are in
`extracted/josephine-summer-singing-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-121-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Summer has four strips left: chopping North and sweeping start/loop/end. Autumn
and Winter each have 33. Next: Summer chopping, then sweeping. Inventory is
recorded in `tmp/slice-122-remainder.json`. Natural gameplay dispatch, speaking
transitions and live rendering remain pending.
