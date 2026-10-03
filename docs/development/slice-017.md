# Olric Spring blinking and seated actions

Following `715b997`, this slice adds 11 normal Spring blinking, sitting, eating
and drinking strips. The 31 source frames and 12 native West mirrors give 43
review cases. Artwork approved for commit on 2026-10-02. Completed outputs use
`generated/review/` and `generated/build/`.

Olric now has 53 sources; the 36-character collection has 3,998 sources and
15,992 variants. K retains Vanilla, Debug Blue, Hayden, Ryis and Seridia, keeping
portraits and supported world sprites synchronized. All earlier artwork, target
colors and runtime code remain unchanged. Historical pixel tests keep their
fixed expectations; only the central Olric total changes in
`tests/profile_coverage.rs`.

## Source and materials

The mount remains absent. This slice uses the original archive backup at
`tmp/slice-016-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive, without establishing whether a newer game
version exists. All 42 prior PNG pins match fresh exports. No pin refresh or hash
override was needed.

The profile appends 841 seeds covering 1,377 source pixels per target. It reuses
the four world shades from the movement pilot: `#F2D7A7`, `#E4BD7C`, `#C4854C`
and `#684810`. They remain skin or its contours throughout these strips, including
closed eyelids, the neck and moving arms. No shared-color exclusions are needed.
Gloves, apple, hair, eyes, clothing and boots stay original. Every frame appears
in the inspected Vanilla/Blue contact sheets; source grids also expose individual
skin and material boundaries.

The new material test checks every pixel against an independent four-color map,
literal landmarks, per-frame counts, alpha, metadata and identical masks across
presets. All 42 prior sources and their variants are compared with the fixed
accepted baseline `generated/slice-016-build/characters/olric`.

Native metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Blink strips have three frames timed 0.075/0.125/0.075 seconds. East/South eating
has five frames timed 0.125/0.15/0.175/0.125/0.6 seconds. North eating and all
drinking strips have three one-second frames; sitting uses one frame per strip.
West mirrors East. The native eating/drinking cycles retain their seated state
and last-frame hold behavior.

## Offline review

- [Five-choice sample summary](../../generated/slice-017-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-017-review/index.html).

The summary samples blinking South, eating East and drinking North. The full
gallery includes all 43 cases, with complete character views and enlarged details.
Its 20 pages total 756,644 bytes (about 739 KiB). Generated images and game packages
stay local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 229 regular tests and the
release build passed. The normal run leaves 299 local opt-ins ignored. Another
82 local tests passed: the new material test, 77 retained corpus tests, Olric's
original portrait test and three GML runtime tests. Release artifacts avoid
recreating the previously removed debug cache.

The new package test rejected the unregistered blink strip before registration,
then passed. Omission and glove-color spill controls both generated successfully
before failing the intended material assertion. All 11 raw metadata sidecars
match independent reads from the original archive.

All 15,992 combined variants validate. All 212 standalone Olric variants pass
strict recipe validation and match the combined build. The 420 earlier Olric
PNG/metadata files and 39,625 files for the other 35 characters remain
byte-identical. Previous runtime rows, choices and hotkeys remain exact.
The source colors, groups, target mappings, collection sets and previous export
report entries remain unchanged; precisely 11 registry entries were added.

The source archive's native animator and NPC object pass 215 frame/palette
observations, 75 linear completions and 80 last-frame hold checks with simulated
engine services and direct cycle selection. Checks cover mirroring, seated state,
palette binding, frame phase, wrapper idempotence, cycle counters and portrait
synchronization. Natural scheduling, interaction dispatch and full-engine
rendering remain untested.

All 20 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character crop, enlarged
detail and all 15 summary bindings: 7,766,304 rendered pixels. The `review`
shortcut also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-017-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-017-`; author records use
`tmp/olric-spring-actions-author-`. Fresh exports live in
`extracted/olric-spring-actions-study`, reached through the stable
`extracted/test-corpus/olric` link. `evidence.json` summarizes results and
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Olric's Spring folder has 34 remaining strips; Summer, Autumn and Winter each
have 48. Continue with Spring generic actions and the shocked start/loop/end
sequence, then his work animations and bunny-ear variants. The remaining
inventory is recorded in `tmp/slice-017-remainder.json`.
