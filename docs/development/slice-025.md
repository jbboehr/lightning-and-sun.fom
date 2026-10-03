# Olric Summer sitting, eating and drinking

Following `fca86c4`, this slice adds nine normal Summer seated-action strips:
25 source frames and nine native West mirrors, giving 34 review cases. Artwork
was approved on 2026-10-02. Completed outputs use `generated/review/` and
`generated/build/`.

Olric now has 104 sources; the 36-character collection has 4,049 sources and
16,196 variants. K retains Vanilla, Debug Blue, Hayden, Ryis and Seridia with
portrait/world synchronization. Earlier artwork, existing target colors and
runtime code remain unchanged. Historical pixel tests keep their fixed
expectations; only the central Olric total changes in `tests/profile_coverage.rs`.

## Source and materials

The mount remains absent. This slice uses the original archive backup at
`tmp/slice-024-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive, without checking for a newer installed
game version. All 95 prior PNG pins match fresh exports. No pin refresh or hash
override was needed.

The profile appends 793 seeds covering 1,214 source pixels per target. Four
existing world shades cover most skin and its contours, including the neck,
exposed arms, closed eyelids and lower legs. Source-grid inspection found an
additional midtone, `#E8B171`, on 11 drinking-arm pixels: ten across the three
East frames and one in the second South frame. A separate color group maps it
to the existing midtone target in each preset and the Debug Blue recipe.
Earlier groups and mappings remain exact. No shared-color exclusions are needed.
Gloves, apple, hair, eyes and clothing stay original. All frames were inspected
in five-choice contact sheets, with source grids around moving arms and the apple.

The new material test checks every pixel against an independent five-color map,
literal landmarks for all 11 additional pixels, per-frame counts, alpha,
metadata and identical masks across presets. Earlier sources and variants use
the fixed accepted baseline `generated/slice-024-build/characters/olric`.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets:

| Cycle | Source directions | Frames per strip | Timing |
| --- | --- | --- | --- |
| Sitting | North, South, East | 1 | Native static metadata, no duration field |
| Eating | South, East | 5 | 0.125, 0.15, 0.175, 0.125, 0.6 seconds |
| Eating | North | 3 | 1 second each |
| Drinking | North, South, East | 3 | 1 second each |

West mirrors East for all three cycles. The original NPC configuration includes
the `summer` outfit for these cycles; all use native linear packs and seated
state. Eating and drinking retain their native last-frame hold behavior.

## Offline review

- [Five-choice sample summary](../../generated/slice-025-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-025-review/index.html).

The summary samples sitting South, eating East and drinking East. The full
gallery includes all 34 cases with complete character views and enlarged details.
Generated images and game packages stay local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 237 regular tests and the
release build passed; 307 local opt-ins remain ignored in the normal run.
Another 90 local tests passed: the new material test, 85 retained corpus tests,
Olric's original portrait test and three GML runtime tests. Release artifacts
avoid recreating the previously removed debug cache.

The new package test rejected the unregistered Summer drink strip before
registration, then passed. The material test failed on the extra arm shade at
East frame 1, coordinate (37,43), before the recipe correction, then passed.
Omission and glove-color spill controls both generated successfully before
failing the intended material assertion. All nine raw metadata sidecars match
independent reads from the original archive.

All 16,196 combined variants validate. All 416 standalone Olric variants pass
strict recipe validation and match the combined build. The 950 earlier Olric
PNG/metadata files and 39,625 files for the other 35 characters remain
byte-identical. Previous runtime rows, choices and hotkeys remain exact. The
source colors, groups and target mappings append only the additional arm shade;
collection sets and previous export report entries remain exact. Precisely nine
registry entries were added.

The source archive's native animator and NPC object pass 170 frame/palette
observations, 60 linear completions and 80 last-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native West mirroring,
seated state, frame phase, cycle counters, wrapper idempotence and synchronization
with the matching Summer neutral portrait under all five choices. Natural outfit
selection, scheduling, speaking/interactions and full-engine rendering remain
untested.

All 17 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full crop, enlarged detail
and all 15 summary bindings: 6,238,752 rendered pixels. Full crops independently
check every nontransparent source pixel in both directions. The `review`
shortcut also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-025-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-025-`; author records use `tmp/olric-summer-actions-author-`.
Fresh exports live in `extracted/olric-summer-actions-study`, reached through the
stable `extracted/test-corpus/olric` link. `evidence.json` summarizes results and
`scope.log` checks the 11 expected paths and unchanged historical tests.

## Remaining work

Olric's Spring folder remains complete. Summer has 31 strips left; Autumn and
Winter each have 48. Continue with Summer generic actions, then work animations
and bunny-ear variants. The inventory is recorded in `tmp/slice-025-remainder.json`.
