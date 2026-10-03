# Olric Spring actions and shocked reaction

Following `64cf8a0`, this slice adds three directional action strips and the
shocked start/loop/end sequence. The 24 source frames and seven native West
mirrors give 31 review cases. Artwork approved for commit on 2026-10-02. Completed outputs
use `generated/review/` and `generated/build/`.

Olric now has 59 sources; the 36-character collection has 4,004 sources and
16,016 variants. K retains Vanilla, Debug Blue, Hayden, Ryis and Seridia with
portrait/world synchronization. Earlier artwork, target colors and runtime code
remain unchanged. Historical pixel tests retain their fixed expectations; only
the central Olric total changes in `tests/profile_coverage.rs`.

## Source and materials

The mount remains absent. This slice uses the original archive backup at
`tmp/slice-017-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive, without checking for a newer installed
game version. All 53 prior PNG pins match fresh exports. No pin refresh or hash
override was needed.

The profile appends 679 seeds covering 1,048 source pixels per target. The same
four world shades (`#F2D7A7`, `#E4BD7C`, `#C4854C`, `#684810`) cover skin and
its contours throughout these strips, including raised arms, ears, face and neck.
No shared-color exclusions are needed. Gloves, hair, eyes, clothing, boots and
the mouth interior stay original. All frames were inspected in Vanilla/Blue
contact sheets, with source grids for material boundaries and the raised-arm pose.

The new material test checks every pixel against an independent four-color map,
literal landmarks, per-frame counts, alpha, metadata and identical masks across
presets. Earlier sources and variants use the fixed accepted baseline
`generated/slice-017-build/characters/olric`.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Each directional action has seven frames timed
0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds. The shocked start, loop and end strips
each have one frame. West mirrors action East; the shocked sequence faces South.
The native action cycle retains its last-frame hold; shocked retains its complex
start/loop/end progression.

## Offline review

- [Five-choice sample summary](../../generated/slice-018-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-018-review/index.html).

The summary samples action East, action North and the shocked loop. The full
gallery includes all 31 cases with complete character crops and enlarged details.
Generated images and game packages stay local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 230 regular tests and the
release build passed; 300 local opt-ins remain ignored in the normal run.
Another 83 local tests passed: the new material test, 78 retained corpus tests,
Olric's original portrait test and three GML runtime tests. Release artifacts
avoid recreating the previously removed debug cache.

The new package test rejected the unregistered action strip before registration,
then passed. Omission and glove-color spill controls both generated successfully
before failing the intended material assertion. All six raw metadata sidecars
match independent reads from the original archive.

All 16,016 combined variants validate. All 236 standalone Olric variants pass
strict recipe validation and match the combined build. The 530 earlier Olric
PNG/metadata files and 39,625 files for the other 35 characters remain
byte-identical. Previous runtime rows, choices and hotkeys remain exact. Source
colors, groups, target mappings, collection sets and previous export report
entries are preserved; precisely six registry entries were added.

The source archive's native animator and NPC object pass 155 frame/palette
observations, 20 linear completions, 40 last-frame hold checks and 20 complex
transitions with simulated engine services and direct cycle selection. Checks
cover native mirroring, frame phase, cycle counters, wrapper idempotence,
portrait synchronization and shocked start/loop/end transitions under all five
choices. Natural scheduling, speaking/interactions and full-engine rendering
remain untested.

All 13 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character crop,
enlarged detail and all 15 summary bindings: 5,729,568 rendered pixels. The
`review` shortcut also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-018-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-018-`; author records use
`tmp/olric-spring-reactions-author-`. Fresh exports live in
`extracted/olric-spring-reactions-study`, reached through the stable
`extracted/test-corpus/olric` link. `evidence.json` summarizes results and
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Olric's Spring folder has 28 remaining strips; Summer, Autumn and Winter each
have 48. Continue with his Spring work animations, starting with hammer,
pickaxe and tongs, then the other work sequences and bunny-ear variants.
The inventory is recorded in `tmp/slice-018-remainder.json`.
