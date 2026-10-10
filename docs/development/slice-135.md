# Josephine Winter general actions

Following `d9f1a2e`, this slice adds Josephine's three Winter general-action
strips. There are 21 source frames plus seven native West mirrors: 28 review
cases. The user approved the artwork on 2026-10-09. Outputs use `generated/review/` and
`generated/build/`.

Josephine now has 154 sources: 32 portraits, all 36 Spring world strips, all 33
Summer world strips, all 33 Autumn world strips and 20 Winter world strips.
The 36-character collection has 4,615 sources and 18,460 variants. P retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. Only three regions and
registry entries are added. Source colors, groups, target mappings, sets and
runtime code remain unchanged. Historical pixel tests retain their expectations;
only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-134-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed game
version. All 151 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 313 component seeds select
689 skin pixels across 21 source frames. These shades belong exclusively to skin
here. Face, neck and moving hands follow the palette. Brown hair, eye details,
gold earrings, pink clothing, dark leggings and boots stay original. Clothing
shades including `#E18A8A`, `#FAC0B5`, `#A15663` and `#633438`, and boot highlight
`#EE678F`, remain unchanged. Winter clothing covers the ankles. The portrait
skin shade `#3A1D24` belongs to dark clothing here and stays outside the mask.

Every source frame was inspected in five-choice contact sheets, with source
grids around extended hands, face, clothing and boots. The material test
independently checks every output pixel against a literal shade oracle,
91 fixed landmarks and per-frame selected counts, plus alpha and metadata.
The earlier 151 sources are compared with the fixed slice 134 baseline. The
profile contains component seeds and pinned source hashes, without new runtime
rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Each direction has seven frames timed at
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`. The native `action` cycle remains
linear, defaults to North, mirrors East for West and retains its final-frame
hold range `[240, 360]`. Its speaking pause transition to `idle` is unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-135-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-135-review/index.html).

The summary samples action East frame 2, North frame 6 and South frame 3. Its
columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full gallery
includes all 28 cases with Vanilla left and Debug Blue right, full views and
enlarged face, hands and clothing. West views mirror the East source exactly as
the native animator does. Game-derived images and packages remain local and
ignored by Git.

## Verification

Formatting, Clippy, all 347 regular tests and the release build passed
through the Nix shell. The regular run left 417 opt-in tests ignored;
the selected local suite passed 205 tests, including the new material test,
retained character checks and three GML runtime tests. Historical pixel tests
are unchanged. The initial formatting check found indentation in the new package
test; project-edition formatting fixed it before the successful full run.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted the face, left hand or right hand, or leaked
recoloring into dark leggings through a `#3A1D24` seed; each generated
successfully and failed the intended material assertion. Three raw metadata
sidecars matched independent original-archive reads.

All 616 standalone Josephine variants validated, and their 1,232 PNG/metadata
files matched the author candidate. All 18,460 combined variants validated.
The 1,510 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 134; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The ten Chromium pages loaded all local links and images without overflow.
The 28 cases have 56 full-view bindings plus enlarged details and fifteen
summary bindings; 5,220,384 rendered pixels match their source crops. Full-view
bounds include every nontransparent pixel. The complete review is
512,280 bytes. The short review/build aliases point to this slice, and the
review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-135-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `0cacf62fffa7c6be0deae3f56fdae2fb6a5da013128438ad9d1d7609e99ac66a`.
The archive's native animator and NPC animate function passed 140 frame/palette
observations, twenty linear completions and forty final-frame hold checks under
Fabricator, including West mirroring. Checks preserve frame phase, wrapper
idempotence, cycle counters, the seated flag and Winter portrait synchronization.
Engine services were simulated and cycles selected directly. This does not
exercise natural gameplay dispatch, speaking transitions or live rendering.
The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-135-`; author records use
`tmp/josephine-winter-general-author-`. Fresh exports are in
`extracted/josephine-winter-general-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-134-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Spring, Summer and Autumn world folders are complete. Winter has thirteen strips
left. Next: Winter writing (start, loop and end). Inventory is recorded in
`tmp/slice-135-remainder.json`. Natural gameplay dispatch, speaking transitions
and live rendering remain pending.
