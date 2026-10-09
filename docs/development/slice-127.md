# Josephine Autumn general actions

Following `02c0cd2`, this slice adds Josephine's three Autumn general-action
strips. There are 21 source frames plus seven native West mirrors: 28 review
cases. The user approved the artwork on 2026-10-08. Outputs use `generated/review/` and
`generated/build/`.

Josephine now has 121 sources: 32 portraits, all 36 Spring world strips, all 33
Summer world strips and 20 Autumn world strips. The 36-character collection has
4,582 sources and 18,328 variants. P retains Vanilla, Debug Blue, Hayden, Ryis
and Seridia choices with portrait/world synchronization. Only three regions and
registry entries are added. Source colors, groups, target mappings, sets and
runtime code remain unchanged. Historical pixel tests retain their expectations;
only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-126-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 118 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 456 component seeds select
805 skin pixels across 21 source frames. These shades belong exclusively to skin
here. Face, neck, moving hands and exposed ankles follow the palette. Brown hair,
eye details, blue earrings, orange clothing and shoes stay original. Clothing
shades including `#D8753A`, `#F5A95B`, `#FCD288` and `#FFFAD7`, and shoe highlight
`#FF914C`, remain unchanged.

Every source frame was inspected in five-choice contact sheets, with source
grids around extended hands, face, clothing and ankles. The material test
independently checks every output pixel against a literal shade oracle,
89 fixed landmarks and per-frame selected counts, plus alpha and metadata.
The earlier 118 sources are compared with the fixed slice 126 baseline. The
profile contains component seeds and pinned source hashes, without new runtime
rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Each direction has seven frames timed at
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`. The native `action` cycle remains
linear, defaults to North, mirrors East for West and retains its final-frame
hold range `[240, 360]`. Its speaking pause transition to `idle` is unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-127-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-127-review/index.html).

The summary samples action East frame 2, North frame 6 and South frame 3. Its
columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full gallery
includes all 28 cases with Vanilla left and Debug Blue right, full views and
enlarged face, hands and ankles. West views mirror the East source exactly as
the native animator does. Game-derived images and packages remain local and
ignored by Git.

## Verification

Formatting, Clippy, all 339 regular tests and the release build passed through
the Nix shell. The regular run left 409 opt-in tests ignored; the selected local
suite passed 197 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted face, left hand, right hand and ankle regions;
each generated successfully and failed the intended material assertion. Three
raw metadata sidecars matched independent original-archive reads.

All 484 standalone Josephine variants validated, and their 968 PNG/metadata
files matched the author candidate. All 18,328 combined variants validated.
The 1,180 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 126; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The ten Chromium pages loaded all local links and images without overflow.
The 28 cases have 56 full-view bindings plus enlarged details and fifteen
summary bindings; 5,220,384 rendered pixels match their source crops. Full-view
bounds include every nontransparent pixel. The complete review is 511,215 bytes.
The short review/build aliases point to this slice, and the review loads through
its short alias.

The isolated MOMI installation passed in `tmp/slice-127-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `d0ef5535dfe13e7c1a37ec22489c816d64ede05895d44521382661cc8153c87f`.
The archive's native animator and NPC animate function passed 140 frame/palette
observations, twenty linear completions and forty final-frame hold checks under
Fabricator, including West mirroring. Checks preserve frame phase, wrapper
idempotence, cycle counters, the seated flag and Autumn portrait synchronization.
Engine services were simulated and cycles selected directly. This does not
exercise natural gameplay dispatch, speaking transitions or live rendering.
The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-127-`; author records use
`tmp/josephine-autumn-general-author-`. Fresh exports are in
`extracted/josephine-autumn-general-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-126-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Spring and Summer world folders are complete. Autumn has thirteen strips left
and Winter has 33. Next: Autumn writing (start, loop and end). Inventory is
recorded in `tmp/slice-127-remainder.json`. Natural gameplay dispatch, speaking
transitions and live rendering remain pending.
