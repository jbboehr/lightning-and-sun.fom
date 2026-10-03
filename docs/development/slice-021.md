# Olric Spring lifting and squatting

Following `1187fc7`, this slice adds lifting start/loop/end and squatting:
four strips and 20 South-facing frames. Neither cycle has native West mirrors.
Artwork was approved on 2026-10-02. Completed outputs use `generated/review/` and
`generated/build/`.

Olric now has 73 sources; the 36-character collection has 4,018 sources and
16,072 variants. K retains Vanilla, Debug Blue, Hayden, Ryis and Seridia with
portrait/world synchronization. Earlier artwork, target colors and runtime code
remain unchanged. Historical pixel tests keep their fixed expectations; only
the central Olric total changes in `tests/profile_coverage.rs`.

## Source and materials

The mount remains absent. This slice uses the original archive backup at
`tmp/slice-020-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive, without checking for a newer installed
game version. All 69 prior PNG pins match fresh exports. No pin refresh or hash
override was needed.

The profile appends 728 seeds covering 1,134 source pixels per target. The same
four world shades (`#F2D7A7`, `#E4BD7C`, `#C4854C`, `#684810`) cover skin and
its contours, including moving upper arms and closed eyelids. No shared-color
exclusions are needed. Weights, gloves, mouth interior, hair, eyes, clothing
and boots stay original. All frames were inspected in five-choice contact
sheets, with source grids at the arm, glove, weight and mouth boundaries.

The new material test checks every pixel against an independent four-color map,
literal landmarks, per-frame counts, alpha, metadata and identical masks across
presets. Earlier sources and variants use the fixed accepted baseline
`generated/slice-020-build/characters/olric`.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
The original timing in seconds is preserved:

| Strip | Frames | Duration |
| --- | --- | --- |
| Lift start South | 3 | 0.15, 0.1, 0.6 |
| Lift loop South | 8 | 0.1, 0.4, 0.1, 0.6, 0.1, 0.4, 0.1, 0.6 |
| Lift end South | 3 | 0.1, 0.15, 1.0 |
| Squat South | 6 | 0.1, 0.15, 1.0, 0.15, 0.1, 1.0 |

The lifting cycle retains its complex start/loop/end progression; squatting
uses the native linear cycle.

## Offline review

- [Five-choice sample summary](../../generated/slice-021-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-021-review/index.html).

The summary samples both arms in the lifting loop and the crouched squat frame.
The full gallery includes all 20 cases with complete character views and
enlarged face/arm details. Generated images and game packages stay local and
ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 233 regular tests and the
release build passed; 303 local opt-ins remain ignored in the normal run.
Another 86 local tests passed: the new material test, 81 retained corpus tests,
Olric's original portrait test and three GML runtime tests. Release artifacts
avoid recreating the previously removed debug cache.

The new package test rejected the unregistered lift-end strip before
registration, then passed. Omission and weight-color spill controls both
generated successfully before failing the intended material assertion. All
four raw metadata sidecars match independent reads from the original archive.

All 16,072 combined variants validate. All 292 standalone Olric variants pass
strict recipe validation and match the combined build. The 690 earlier Olric
PNG/metadata files and 39,625 files for the other 35 characters remain
byte-identical. Previous runtime rows, choices and hotkeys remain exact. Source
colors, groups, target mappings, collection sets and previous export report
entries are preserved; precisely four registry entries were added.

The source archive's native animator and NPC object pass 100 frame/palette
observations, five linear completions and 20 complex transitions with simulated
engine services and direct cycle selection. Checks cover frame phase, cycle
counters, wrapper idempotence, portrait synchronization and lifting
start/loop/end transitions under all five choices. Natural scheduling,
speaking/interactions and full-engine rendering remain untested.

All eight gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, metadata binding, full crop, enlarged detail and
all 15 summary bindings: 3,862,560 rendered pixels. Full crops independently
check every nontransparent source pixel. The `review` shortcut also passes
Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-021-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-021-`; author records use `tmp/olric-spring-lift-author-`.
Fresh exports live in `extracted/olric-spring-lift-study`, reached through the
stable `extracted/test-corpus/olric` link. `evidence.json` summarizes results and
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Olric's Spring folder has 14 remaining strips; Summer, Autumn and Winter each
have 48. Continue with writing and sweeping, then bunny-ear variants. The
inventory is recorded in `tmp/slice-021-remainder.json`.
