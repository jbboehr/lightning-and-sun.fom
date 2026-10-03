# Olric Summer standing, walking and blinking

Following `1f9822c`, this slice adds eight normal Summer strips: 21 source
frames and eight native West mirrors, giving 29 review cases. Artwork was
approved on 2026-10-02. Completed outputs use `generated/review/` and `generated/build/`.

Olric now has 95 sources; the 36-character collection has 4,040 sources and
16,160 variants. K retains Vanilla, Debug Blue, Hayden, Ryis and Seridia with
portrait/world synchronization. Earlier artwork, target colors and runtime code
remain unchanged. Historical pixel tests keep their fixed expectations; only
the central Olric total changes in `tests/profile_coverage.rs`.

## Source and materials

The mount remains absent. This slice uses the original archive backup at
`tmp/slice-023-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive, without checking for a newer installed
game version. All 87 prior PNG pins match fresh exports. No pin refresh or hash
override was needed.

The profile appends 811 seeds covering 1,167 source pixels per target. The same
four world shades (`#F2D7A7`, `#E4BD7C`, `#C4854C`, `#684810`) cover skin and
its contours, including the rear neck, exposed arms, closed eyelids and lower
legs above the socks. No shared-color exclusions are needed. Gloves, shirt,
shorts, socks, shoes, hair and eyes stay original. All frames were inspected
in five-choice contact sheets, with source grids around the neck, moving arms
and lower legs.

The new material test checks every pixel against an independent four-color map,
literal landmarks, per-frame counts, alpha, metadata and identical masks across
presets. Earlier sources and variants use the fixed accepted baseline
`generated/slice-023-build/characters/olric`.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets:

| Cycle | Source directions | Frames per strip | Timing |
| --- | --- | --- | --- |
| Standing | North, South, East | 1 | Native static metadata, no duration field |
| Blinking | South, East | 3 | 0.075, 0.125, 0.075 seconds |
| Walking | North, South, East | 4 | 0.15 seconds each |

West mirrors East for all three cycles. The original NPC configuration includes
the `summer` outfit for these cycles; all use native linear packs.

## Offline review

- [Five-choice sample summary](../../generated/slice-024-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-024-review/index.html).

The summary samples standing South and walking North and East. The full gallery
includes all 29 cases with complete character views and enlarged details.
Generated images and game packages stay local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 236 regular tests and the
release build passed; 306 local opt-ins remain ignored in the normal run.
Another 89 local tests passed: the new material test, 84 retained corpus tests,
Olric's original portrait test and three GML runtime tests. Release artifacts
avoid recreating the previously removed debug cache.

The new package test rejected the unregistered Summer blink strip before
registration, then passed. Omission and glove-color spill controls both
generated successfully before failing the intended material assertion. All
eight raw metadata sidecars match independent reads from the original archive.

All 16,160 combined variants validate. All 380 standalone Olric variants pass
strict recipe validation and match the combined build. The 870 earlier Olric
PNG/metadata files and 39,625 files for the other 35 characters remain
byte-identical. Previous runtime rows, choices and hotkeys remain exact. Source
colors, groups, target mappings, collection sets and previous export report
entries are preserved; precisely eight registry entries were added.

The source archive's native animator and NPC object pass 145 frame/palette
observations and 55 linear completions with simulated engine services and
direct cycle selection. Checks cover native West mirroring, frame phase,
cycle counters, wrapper idempotence and synchronization with the matching Summer
neutral portrait under all five choices. Natural outfit selection, scheduling,
speaking/interactions and full-engine rendering remain untested.

All 13 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full crop, enlarged detail
and all 15 summary bindings: 5,390,112 rendered pixels. Full crops independently
check every nontransparent source pixel in both directions. The `review`
shortcut also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-024-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-024-`; author records use `tmp/olric-summer-world-author-`.
Fresh exports live in `extracted/olric-summer-world-study`, reached through the
stable `extracted/test-corpus/olric` link. `evidence.json` summarizes results and
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Olric's Spring folder remains complete. Summer has 40 strips left; Autumn and
Winter each have 48. Continue with Summer sitting, eating and drinking. The
inventory is recorded in `tmp/slice-024-remainder.json`.
