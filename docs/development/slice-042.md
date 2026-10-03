# Olric Winter general actions

Following `1bc7d93`, this slice adds three normal Winter general-action strips:
21 source frames and seven native West mirrors, giving 28 review cases. Artwork
was approved on 2026-10-03. Completed outputs use `generated/review/` and
`generated/build/`.

Olric now has 203 sources; the 36-character collection has 4,148 sources and
16,592 variants. K retains Vanilla, Debug Blue, Hayden, Ryis and Seridia with
portrait/world synchronization. Earlier artwork, target colors and runtime code
remain unchanged. Historical pixel tests keep their fixed expectations; only
the central Olric total changes in `tests/profile_coverage.rs`.

## Source and materials

The game mount remains empty. This slice uses the original archive backup at
`tmp/slice-041-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive, without checking for a newer installed
game version. All 200 prior PNG pins match fresh exports. No pin refresh or hash
override was needed.

The profile appends 467 seeds covering 745 source pixels per target. Four
existing world shades (`#F2D7A7`, `#E4BD7C`, `#C4854C`, `#684810`) cover skin
and its contours, including the rear neck. Sleeves and gloves cover the arms
and hands. The extra drinking-arm shade does not occur in these strips. No new
colors or shared-color exclusions are needed. Gloves, clothing, hair and eyes
stay original. All frames were inspected in five-choice contact sheets, with
source grids around the face, rear neck, collar and gloves.

Winter rear views expose two extra neck pixels per frame compared with Autumn.
Those 14 pixels across seven North-facing frames are included; the collar
remains original.

The new material test checks every pixel against an independent four-color map,
literal landmarks, per-frame counts, alpha, metadata and identical masks across
presets. Earlier sources and variants use the fixed accepted baseline
`generated/slice-041-build/characters/olric`.

Each North, South and East strip has seven 80×80 frames, retaining the Default
atlas and numeric 40/54 offsets. Durations are 0.1, 0.25, 0.25, 0.25, 0.25,
0.1 and 0.4 seconds. West mirrors East. The original NPC configuration includes
the `winter` outfit for the linear `action` cycle and retains its native
last-frame hold. It specifies `idle` when speaking pauses the action; that
interaction dispatch is outside this verification.

## Offline review

- [Five-choice sample summary](../../../generated/slice-042-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-042-review/index.html).

The summary samples South frame 3, East frame 2 and North frame 2. The full
gallery includes all 28 cases with complete character views and enlarged details.
Generated images and game packages stay local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 254 regular tests and the
release build passed; 324 local opt-ins remain ignored in the normal run.
Another 107 local tests passed: the new material test, 102 retained corpus tests,
Olric's original portrait test and three GML runtime tests. Release artifacts
avoid recreating the previously removed debug cache.

The new package test rejected the unregistered Winter action strip before
registration, then passed. Omission and clothing-color spill controls both
generated successfully before failing the intended material assertion. All
three raw metadata sidecars match independent reads from the original archive.

All 16,592 combined variants validate. All 812 standalone Olric variants pass
strict recipe validation and match the combined build. The 2,000 earlier Olric
PNG/metadata files and 39,625 files for the other 35 characters remain
byte-identical. Previous runtime rows, choices and hotkeys remain exact. Source
colors, groups, target mappings, collection sets and previous export report
entries are preserved; precisely three registry entries were added.

The source archive's native animator and NPC object pass 140 frame/palette
observations, 20 linear completions and 40 last-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native West mirroring,
frame phase, cycle counters, wrapper idempotence and synchronization with the
matching Winter neutral portrait under all five choices. Natural outfit
selection, scheduling, speaking/interactions and full-engine rendering remain
untested.

All 10 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full crop, enlarged detail
and all 15 summary bindings: 5,220,384 rendered pixels. Full crops independently
check every nontransparent source pixel in both directions. The `review`
shortcut also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-042-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-042-`; author records use `tmp/olric-winter-standard-author-`.
Fresh exports live in `extracted/olric-winter-standard-study`, reached through the
stable `extracted/test-corpus/olric` link. `evidence.json` summarizes results and
`scope.log` checks the nine expected paths and unchanged historical tests.
Concurrent edits to `flake.nix` and `flake.lock` are outside this artwork slice
and were preserved unchanged.

## Remaining work

Olric's Spring, Summer and Autumn folders remain complete. Winter has 28 strips
left. Continue with Winter hammer, pickaxe and tongs, then the remaining work
animations and bunny-ear variants. The inventory is recorded in
`tmp/slice-042-remainder.json`.
