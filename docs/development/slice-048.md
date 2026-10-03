# Landen Spring movement pilot

Following `88ea4d6`, this slice adds six Spring standing/walking strips to
Landen's accepted 32 portraits. The 15 source frames and five native West
mirrors give 20 review cases. Artwork was approved by the user on 2026-10-03. Completed outputs
use `generated/review/` and `generated/build/`.

Landen has 38 sources; the 36-character collection now has 4,182 sources and
16,728 variants. L keeps the existing Vanilla, Debug Blue, Hayden, Ryis and
Seridia choices, with portrait/world synchronization. The collection switches
only Landen to the new `landen-world-trial.json` set. His portrait-only profile,
set, recipe and test remain untouched. No runtime code changes. Historical pixel
tests retain their fixed expectations; the growing Landen total changes only
in `tests/profile_coverage.rs`.

## Source and materials

The mount was absent. This slice uses the original archive backup at
`tmp/slice-047-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 32 portrait pins match fresh exports. No pin refresh or hash
override was needed.

The new world profile retains all portrait regions, seven source colors, seven
color groups and four target palettes. The world sprites use the existing first
four shades:

| Source | Role | Debug Blue |
| --- | --- | --- |
| `#9F6250` | Light skin | `#9DB9D4` |
| `#854D3C` | Light shadow | `#7F9FBD` |
| `#63342A` | Middle shadow | `#6687AD` |
| `#491F1B` | Deep contour | `#445F83` |

The 297 new seeds select 661 skin pixels across all source frames. Shoes reuse
these colors: the initial candidate recolored 123 shoe pixels, which were
excluded before integration. In these six pinned strips, exposed skin lies above
row 49 and shoes below it. The tracked masks use separate connected-component
seeds; the test independently checks that spatial/material boundary, literal
source landmarks, every output pixel, per-frame counts and transparency.

Face, ears, neck and hands follow the selected palette. Hair, eye detail, the
pale scarf, purple shirt, trousers and shoes remain original. All 15 source
frames were inspected in source grids and five-choice contact sheets. Ryis
changes 481 of the 661 selected pixels because two target shades already equal
the source; the test counts selected skin separately from changed pixels.
The fixed accepted portrait baseline is
`generated/slice-047-build/characters/landen`.

Native metadata retains 80×80 frames, the Default atlas, numeric 40/54 offsets,
one frame per standing direction and four 0.15-second frames per walking
direction. West mirrors the East strips.

## Offline review

- [Five-choice sample summary](../../generated/slice-048-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-048-review/index.html).

The summary samples standing South, walking East and walking North. The full
review includes all 20 cases with complete character crops and enlarged details.
The ten-page gallery totals 394,487 bytes (about 385 KiB). Images, generated
packages and game copies remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 260 regular tests and the
release build passed; 330 local opt-ins remain ignored in the normal run.
114 local tests passed separately: the new world material test, the 112 retained
checks from slice 047 and Landen's original portrait test. This includes the
three local GML tests. The package test rejected the unregistered idle strip
before registration and passed afterward. Both a missing skin seed and an extra
shoe seed generated successfully, then failed the intended material assertion.

All 16,728 combined variants validate. All 152 standalone Landen variants pass
strict recipe validation and match both the candidate and combined build.
The 320 accepted Landen portrait PNG/metadata files and 41,615 files for the
other 35 characters remain byte-identical. Existing runtime rows, choices and
hotkeys remain exact. The portrait-only inputs and all historical pixel tests
remain unchanged in Git.

The source archive's native animator and NPC object pass 100 frame/palette
observations and 40 linear completions with simulated engine services and
direct cycle selection. Checks cover native mirroring, palette binding, frame
phase, wrapper idempotence, cycle counters, completion and portrait synchronization.
Natural scheduling, outfit/interaction dispatch and full-engine rendering remain
untested.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character crop,
enlarged detail and all 15 summary bindings: 3,862,560 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-048-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-048-`; author records use `tmp/landen-world-author-`.
Fresh exports live in `extracted/landen-world-study`, reached through the stable
`extracted/test-corpus/landen` link. `evidence.json` summarizes the results and
`scope.log` checks the twelve expected paths and unchanged historical tests.

## Remaining work

Landen's Spring folder has 18 remaining strips; Summer, Autumn and Winter each
have 21. `tmp/slice-048-remainder.json` records the inventory. Continue with
Spring blinking, sitting, eating and drinking animations after artwork review.
