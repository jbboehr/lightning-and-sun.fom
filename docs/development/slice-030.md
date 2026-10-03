# Olric Summer writing and sweeping

Following `5f3ab2f`, this slice adds writing and sweeping start/loop/end:
six strips and 23 South-facing frames. Neither cycle has native West mirrors.
Artwork approved by the user on 2026-10-02. Completed outputs use `generated/review/` and
`generated/build/`.

Olric now has 127 sources; the 36-character collection has 4,072 sources and
16,288 variants. K retains Vanilla, Debug Blue, Hayden, Ryis and Seridia with
portrait/world synchronization. Earlier artwork, target colors and runtime code
remain unchanged. Historical pixel tests keep their fixed expectations; only
the central Olric total changes in `tests/profile_coverage.rs`.

## Source and materials

The mount remains absent. This slice uses the original archive backup at
`tmp/slice-029-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive, without checking for a newer installed
game version. All 121 prior PNG pins match fresh exports. No pin refresh or hash
override was needed.

The profile appends 840 seeds covering 1,408 source pixels per target. The same
four world shades (`#F2D7A7`, `#E4BD7C`, `#C4854C`, `#684810`) cover skin and
its contours, including moving upper arms, closed eyelids and exposed lower legs.
The extra drinking-arm shade does not occur in these strips. No new colors or
shared-color exclusions are needed. The broom, dust, clipboard, pencil, gloves,
hair, eyes, clothing and boots stay original. All frames were inspected in five-choice
contact sheets, with source grids at the arm, glove, broom and writing-tool
boundaries.

The new material test checks every pixel against an independent four-color map,
literal landmarks, per-frame counts, alpha, metadata and identical masks across
presets. Earlier sources and variants use the fixed accepted baseline
`generated/slice-029-build/characters/olric`.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
The original timing in seconds is preserved:

| Strip | Frames | Duration |
| --- | --- | --- |
| Sweep start South | 3 | 0.1 each |
| Sweep loop South | 9 | 0.05, 0.1, 0.125, 0.1, 0.1, 0.1, 0.125, 0.1, 0.05 |
| Sweep end South | 3 | 0.1 each |
| Write start South | 2 | 0.125 each |
| Write loop South | 4 | 0.1, 0.125, 0.1, 0.3 |
| Write end South | 2 | 0.125, 0.1 |

Both cycles retain their native complex start/loop/end progression.

## Offline review

- [Five-choice sample summary](../../generated/slice-030-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-030-review/index.html).

The summary samples writing and both sweeping extremes. The full gallery
includes all 23 cases with complete character/prop views and enlarged face/arm
details. Wider full-view crops retain all broom and dust pixels. Generated
images and game packages stay local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 242 regular tests and the
release build passed; 312 local opt-ins remain ignored in the normal run.
Another 95 local tests passed: the new material test, 90 retained corpus tests,
Olric's original portrait test and three GML runtime tests. Release artifacts
avoid recreating the previously removed debug cache.

The new package test rejected the unregistered sweep-end strip before
registration, then passed. Omission and broom-color spill controls both
generated successfully before failing the intended material assertion. All
six raw metadata sidecars match independent reads from the original archive.

All 16,288 combined variants validate. All 508 standalone Olric variants pass
strict recipe validation and match the combined build. The 1,210 earlier Olric
PNG/metadata files and 39,625 files for the other 35 characters remain
byte-identical. Previous runtime rows, choices and hotkeys remain exact. Source
colors, groups, target mappings, collection sets and previous export report
entries are preserved; precisely six registry entries were added.

The source archive's native animator and NPC object pass 115 frame/palette
observations and 40 complex transitions with simulated engine services and
direct cycle selection. Checks cover frame phase, cycle counters, wrapper
idempotence, matching Summer portrait synchronization and both start/loop/end
progressions under all five choices. Natural outfit selection, scheduling,
speaking/interactions and full-engine rendering remain untested.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, metadata binding, full crop, enlarged detail and
all 15 summary bindings: 4,399,056 rendered pixels. Full crops independently
check every nontransparent source pixel. The `review` shortcut also passes
Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-030-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-030-`; author records use
`tmp/olric-summer-write-sweep-author-`. Fresh exports live in
`extracted/olric-summer-write-sweep-study`, reached through the stable
`extracted/test-corpus/olric` link. `evidence.json` summarizes results and
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Olric's Spring folder remains complete. Summer has eight bunny-ear strips left;
Autumn and Winter each have 48. Continue with the Summer bunny-ear variants.
The inventory is recorded in `tmp/slice-030-remainder.json`.
