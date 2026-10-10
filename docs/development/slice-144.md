# Darcy Summer standing and walking

Following `0cdbac3`, this slice adds six Summer standing and walking strips.
Fifteen source frames and five native West mirrors give twenty review cases.
The user approved the artwork on 2026-10-10. Outputs use `generated/review/` and `generated/build/`.

Darcy has 53 sources; the 36-character collection has 4,649 sources and 18,596
variants. B retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only six regions and registry entries are
added. Source colors, color groups, target mappings, sets and previous regions
remain unchanged. No runtime code changes. Historical pixel tests retain fixed
expectations; only the central Darcy coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-143-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed
game version. All 47 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world shades from [slice 141](slice-141.md):
`#FCD9B3`, `#F6996C`, `#C76539` and `#7B2806`. Its 300 component seeds select
513 skin pixels across all fifteen frames: face, ears, front neck, exposed
arms and moving hands. These shades belong exclusively to skin in this batch.
Summer exposes more arm skin than Spring, while the rear neck is covered by
the green collar. Gray hair, eye details, the green blouse, coral headwear and
skirt, and cream footwear stay original. The footwear uses separate
`#E6C0A4` and `#FEF3DE` shades above and below its coral trim. Profiles contain
explicit seeds and source hashes, without new runtime rules.

Every source frame was inspected in five-choice sheets, with source grids
around shoulder and arm boundaries, the collar, hands and footwear. The Summer
portrait was also inspected to check the outfit. The material test checks
every output pixel against an independent shade oracle, 95 fixed landmarks
and per-frame selected counts, plus transparency and metadata. The landmarks
cover both hand areas and both footwear shades in every frame, the rear
collar, face shading, exposed arms, eye details, hair and clothing. Prior output
bytes are compared against the fixed slice 143 baseline.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Standing has one frame; walking has four 0.15-second frames. Both native cycles
are linear, with West mirroring East. The speaking-turn and walking pause
settings remain unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-144-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-144-review/index.html).

The summary samples standing South and walking East/North. The full gallery
includes every source frame and native West mirror, with Vanilla left and
Debug Blue right, full character views and enlarged face, hand and clothing
details. Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 356 regular tests and the release
build passed through the Nix shell. The regular run left 426 opt-in tests
ignored; the selected local suite passed 215 tests, including Darcy's unchanged
portrait and Spring tests, the new material test, retained character checks and
three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the six new entries. Four valid
mask mutations independently omitted the face, left arm, right arm or a moving
arm in a walking frame; each generated successfully and failed the intended
material assertion. All six raw metadata sidecars matched independent archive
reads.

All 212 standalone Darcy variants passed strict recipe validation, and their
424 PNG/metadata files matched the author candidate. All 18,596 combined
variants validated. The 470 prior Darcy PNG/metadata files and 46,135 other
character files are byte-identical to slice 143; previous runtime rows, choices
and hotkeys also match. All prior regions, colors, groups and target mappings
remain unchanged, as do the collection sets. All 36 export reports retain
prior source entries and the archive hash. The five frozen configuration inputs
and both original archive copies retained their hashes.

All ten Chromium gallery pages loaded their local links and images without
overflow. The twenty cases include five West mirrors, 40 full-view bindings,
enlarged details and fifteen summary bindings; 3,862,560 rendered pixels match
their source crops. Full-view bounds include every nontransparent pixel. The
complete review is 396,022 bytes. The short review/build aliases point to this
slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-144-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `23de05bf66324e39674b735c1ef5c0952cdf33af68942b7a03cec43c3abc6536`.
The archive's native animator and NPC animate function passed one hundred
frame/palette observations and forty linear completions under Fabricator.
Checks preserve native mirroring, palette binding, frame phase, wrapper
idempotence, cycle counters and Summer portrait synchronization. Engine
services are simulated and cycles selected directly. This does not exercise
natural gameplay dispatch, speaking transitions or live rendering. The desktop
launcher and installed game were not changed.

Evidence uses `tmp/slice-144-`; author records use
`tmp/darcy-summer-movement-author-`. Fresh exports are in
`extracted/darcy-summer-movement-study`, reached through
`extracted/test-corpus/darcy`. The fixed prior-art baseline is
`generated/slice-143-build/characters/darcy`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves nine Summer strips and fifteen each for Autumn
and Winter; Spring is complete. Next: Summer blinking, sitting and eating,
followed by general actions. Inventory is recorded in
`tmp/slice-144-remainder.json`. Natural gameplay dispatch and live rendering
remain pending; this coverage describes the verified archive. The previously
deferred portrait lip-highlight contrast is unchanged.
