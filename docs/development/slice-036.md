# Olric Autumn polishing, brow wipe and potion

Following `881b6cf`, this slice adds polishing start/loop/end, wiping his brow
and drinking a potion: five strips, 21 source frames and nine native West mirrors,
giving 30 review cases. Artwork was approved on 2026-10-03. Completed outputs use
`generated/review/` and `generated/build/`.

Olric now has 165 sources; the 36-character collection has 4,110 sources and
16,440 variants. K retains Vanilla, Debug Blue, Hayden, Ryis and Seridia with
portrait/world synchronization. Earlier artwork, target colors and runtime code
remain unchanged. Historical pixel tests keep their fixed expectations; only
the central Olric total changes in `tests/profile_coverage.rs`.

## Source and materials

The mount remains absent. This slice uses the original archive backup at
`tmp/slice-035-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive, without checking for a newer installed
game version. All 160 prior PNG pins match fresh exports. No pin refresh or hash
override was needed.

The profile appends 532 seeds covering 970 source pixels per target. The same
four world shades (`#F2D7A7`, `#E4BD7C`, `#C4854C`, `#684810`) cover skin and
its contours on the face, ears and neck, including partially covered features
and closed eyelids. Sleeves and gloves cover the arms and hands.
The extra drinking-arm shade does not occur in these strips. No new colors or
shared-color exclusions are needed. Cloth, held objects, the potion bottle,
gloves, hair, eyes, clothing and boots stay original. All frames were inspected
in five-choice contact sheets, with source grids at material boundaries.

The new material test checks every pixel against an independent four-color map,
literal landmarks, per-frame counts, alpha, metadata and identical masks across
presets. Earlier sources and variants use the fixed accepted baseline
`generated/slice-035-build/characters/olric`.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
The original timing in seconds is preserved:

| Strip | Frames | Duration |
| --- | --- | --- |
| Potion South | 6 | 0.1, 0.1, 0.2, 0.1, 0.1, 0.1 |
| Polish start East | 3 | 0.125 each |
| Polish loop East | 4 | 0.1, 0.125, 0.1, 0.225 |
| Polish end East | 2 | 0.125, 0.1 |
| Brow wipe South | 6 | 0.15, 0.15, 0.45, 0.1, 0.075, 1.0 |

West mirrors the three polishing strips. The native polishing cycle retains its
complex start/loop/end progression.

## Offline review

- [Five-choice sample summary](../../../generated/slice-036-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-036-review/index.html).

The summary samples the polishing loop, brow wipe and potion. The full gallery
includes all 30 cases with complete character views and enlarged face/arm details.
Generated images and game packages stay local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 248 regular tests and the
release build passed; 318 local opt-ins remain ignored in the normal run.
Another 101 local tests passed: the new material test, 96 retained corpus tests,
Olric's original portrait test and three GML runtime tests. Release artifacts
avoid recreating the previously removed debug cache.

The new package test rejected the unregistered potion strip before registration,
then passed. Omission and bottle-color spill controls both generated successfully
before failing the intended material assertion. All five raw metadata sidecars
match independent reads from the original archive.

All 16,440 combined variants validate. All 660 standalone Olric variants pass
strict recipe validation and match the combined build. The 1,600 earlier Olric
PNG/metadata files and 39,625 files for the other 35 characters remain
byte-identical. Previous runtime rows, choices and hotkeys remain exact. Source
colors, groups, target mappings, collection sets and previous export report
entries are preserved; precisely five registry entries were added.

The source archive's native animator and NPC object pass 150 frame/palette
observations, ten linear completions and 40 complex transitions with simulated
engine services and direct cycle selection. Checks cover native mirroring,
frame phase, cycle counters, wrapper idempotence, matching Autumn portrait
synchronization and polishing start/loop/end transitions under all five choices.
Natural outfit selection, scheduling, speaking/interactions and full-engine
rendering remain untested.

All 12 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full crop, enlarged detail
and all 15 summary bindings: 5,559,840 rendered pixels. Full crops independently
check every nontransparent source pixel in both directions. The `review`
shortcut also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-036-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-036-`; author records use `tmp/olric-autumn-polish-author-`.
Fresh exports live in `extracted/olric-autumn-polish-study`, reached through the
stable `extracted/test-corpus/olric` link. `evidence.json` summarizes results and
`scope.log` checks the nine expected paths and unchanged historical tests.
The pre-existing `flake.nix` and `flake.lock` edits remain untouched and outside
this artwork slice.

## Remaining work

Olric's Spring and Summer folders remain complete. Autumn has 18 remaining
strips and Winter has 48. Continue with Autumn lifting and squatting, then writing,
sweeping and bunny-ear variants. The inventory is recorded in
`tmp/slice-036-remainder.json`.
