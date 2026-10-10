# Josephine Winter singing

Following `a458c05`, this slice adds Josephine's three Winter singing
start/loop/end strips. All fourteen frames face South; there are no native West
mirrors. The user approved the artwork on 2026-10-09. Outputs use `generated/review/` and
`generated/build/`.

Josephine now has 163 sources: 32 portraits, all 36 Spring world strips, all 33
Summer world strips, all 33 Autumn world strips and 29 Winter world strips.
The 36-character collection has 4,624 sources and 18,496 variants. P retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. Only three regions and registry entries are added. Source
colors, groups, target mappings, sets and runtime code remain unchanged.
Historical pixel tests retain their expectations; only the central
Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-137-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed game
version. All 160 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 368 component seeds select
830 skin pixels across fourteen source frames, including both moving hands.
These shades belong exclusively to skin here. Brown hair, eye details, gold
earrings, pink clothing, dark leggings, boots, mouth interiors and floating
musical notes stay original. The red `#9E2626`
and dark `#410808` mouth pixels remain unchanged, as do the notes' blue
`#599DFD`, pink `#FF6888` and green `#95DA53` shades. Winter clothing covers
the ankles. The portrait skin shade `#3A1D24` is dark clothing here and stays
outside the mask.

Every source frame was inspected in five-choice contact sheets, with source
grids around the mouth, hands, clothing, boots and notes. The material test
independently checks every output pixel against a literal shade oracle,
52 fixed landmarks and per-frame selected counts, plus alpha and metadata.
Both hands have a landmark in every frame; additional landmarks cover
neighboring materials in each stage,
three open-mouth poses and each note color. The earlier 160 sources are compared
with the fixed slice 137 baseline. The profile contains
component seeds and pinned source hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Start and end each have two frames; loop has ten. Every frame lasts 0.1 seconds.
The native singing cycle remains a South-facing complex start/loop/end sequence.
Its existing speaking-pause behavior selects idle; that dispatch is unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-138-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-138-review/index.html).

The summary samples start frame 1, loop frame 3 and end frame 1. The full gallery
includes all fourteen frames, with Vanilla left and Debug Blue right, full views
and enlarged face, hands and clothing. The wider full-view crop includes every
floating note. Close-ups focus on the face, hands and clothing, excluding cut-off
note fragments. Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy, all 350 regular tests and the release build passed through
the Nix shell. The regular run left 420 opt-in tests ignored; the selected local
suite passed 208 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted the face, left hand or right hand, or leaked
recoloring into dark leggings through a `#3A1D24` seed. Each generated
successfully and failed the intended material assertion. Three raw metadata
sidecars matched independent original-archive reads.

All 652 standalone Josephine variants validated, and their 1,304 PNG/metadata
files matched the author candidate. All 18,496 combined variants validated.
The 1,600 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 137; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The seven Chromium pages loaded all local links and images without overflow.
The fourteen cases have 28 full-view bindings plus enlarged details and fifteen
summary bindings; 3,029,952 rendered pixels match their source crops. Full-view
bounds include every nontransparent pixel. Close-ups exclude floating notes and
keep their edge fragments out of the face and hand details. The complete review
is 298,301 bytes. The short review/build aliases point to this slice, and the
review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-138-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `d00a09bc927406ed54228309e2e4bc19c7f177d88ab425149df79420acaa8fb5`.
The archive's native animator and NPC animate function passed seventy
frame/palette observations and twenty complex stage transitions under Fabricator.
Checks preserve frame phase, wrapper idempotence, cycle counters and Winter
portrait synchronization. There are no linear completions or final-frame holds
here. Engine services were simulated and cycles selected directly. This does
not exercise natural gameplay dispatch, speaking transitions or live rendering.
The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-138-`; author records use
`tmp/josephine-winter-singing-author-`. Fresh exports are in
`extracted/josephine-winter-singing-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-137-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Spring, Summer and Autumn world folders are complete. Winter has four strips
left: chopping North and sweeping start/loop/end. Next: Winter chopping,
then sweeping. Inventory is recorded in `tmp/slice-138-remainder.json`.
Natural gameplay dispatch, speaking transitions and live rendering remain pending.
