# Olric Spring movement pilot

Following `ca64028`, this slice adds six normal Spring standing/walking strips
to Olric's accepted 36 portraits. The 15 source frames and five native West
mirrors give 20 review cases. Artwork approved for commit on 2026-10-02. Completed outputs
use `generated/review/` and `generated/build/`.

Olric has 42 sources; the 36-character collection now has 3,987 sources and
15,948 variants. His K control keeps the existing Vanilla, Debug Blue, Hayden,
Ryis and Seridia choices, with portrait/world synchronization. The collection
switches only Olric to the new `olric-world-trial.json` set. His separate
portrait-only profile, set, recipe and test remain untouched. No runtime code
changes. Historical pixel tests retain their fixed expectations; the growing
Olric total changes only in `tests/profile_coverage.rs`.

## Source and materials

The mount was absent. This slice uses the original archive backup at
`tmp/slice-015-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 36 portrait pins match fresh exports. No pin refresh or
hash override was needed.

The new world profile preserves the portrait regions, eight source colors and
three color groups. It appends four separate world groups and their target
roles, without changing existing target colors:

| Source | Role | Debug Blue |
| --- | --- | --- |
| `#F2D7A7` | Light skin | `#9DB9D4` |
| `#E4BD7C` | Light shadow | `#7F9FBD` |
| `#C4854C` | Middle shadow | `#6687AD` |
| `#684810` | Deep contour | `#445F83` |

All four shades are exposed skin or its contours in these six strips. The 422
new seeds cover 623 source pixels per target, including forehead and eye-edge
shading, ears, neck and shifting upper-arm exposure beside the gloves.
Hair, eye whites and black details, brown gloves, red shirt, green trousers and
boots remain original. Every frame was inspected in source grids and the
Vanilla/Blue contact sheet. The local material test independently checks every
pixel, literal source landmarks, identical masks across presets, transparency
and per-frame counts. Prior portrait PNGs and metadata use the fixed accepted
baseline in `generated/slice-015-build/characters/olric`.

Native metadata retains 80×80 frames, the Default atlas, numeric 40/54 offsets,
one frame per idle direction and four 0.15-second frames per walking direction.
West mirrors the East strips. North also has exposed ears, neck and arms.

## Offline review

- [Five-choice sample summary](../../generated/slice-016-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-016-review/index.html).

The summary samples standing South, walking East and walking North. The full
gallery includes all 20 cases with complete character crops and enlarged
details. The ten-page gallery totals 391,819 bytes (about 383 KiB). Images and
generated packages stay local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 228 regular tests and the
release build passed; 298 local opt-ins remain ignored in the normal run.
Eighty-one local tests passed separately: the new world material test, 76
retained corpus tests, Olric's original portrait test and three GML runtime
tests. Release artifacts avoid recreating the debug cache removed earlier.
The package test rejected the unregistered idle strip before registration and
passed afterward. Both omission and glove-color spill controls generated before
failing the intended pixel assertion.

All 15,948 combined variants validate. All 168 standalone Olric variants pass
strict recipe validation and match the combined build. The 360 accepted Olric
portrait PNG/metadata files and 39,625 files for the other 35 characters remain
byte-identical. Previous runtime rows, choices and hotkeys remain exact.
The original Olric portrait recipes and tests remain byte-identical in Git.

The source archive's native animator and NPC object pass 100 frame/palette
observations and 40 linear completions with simulated engine services and
direct cycle selection. Checks cover native mirroring, palette binding, frame
phase, wrapper idempotence, cycle counters, completion and portrait synchronization.
Natural scheduling, outfit/interaction dispatch and full-engine rendering remain
untested.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character crop,
enlarged detail and all 15 summary bindings: 3,862,560 rendered pixels. The
`review` shortcut also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-016-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-016-`; author records use `tmp/olric-world-author-`.
Fresh exports live in `extracted/olric-world-study`, reached through the stable
`extracted/test-corpus/olric` link. `evidence.json` summarizes the results and
`scope.log` checks the twelve expected paths and unchanged historical tests.

## Remaining work

Olric's Spring folder has 45 remaining strips; Summer, Autumn and Winter each
have 48. Bunny-ear overworld variants are outside this pilot, although their
portraits remain covered. `tmp/slice-016-remainder.json` records the inventory.
Continue with normal Spring blinking, sitting, eating and drinking animations.
