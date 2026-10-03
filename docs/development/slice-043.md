# Olric Winter hammer, pickaxe and tongs

Following `b836315`, this slice adds hammer, pickaxe and the tongs start/loop/end
sequence: five strips, 17 source frames and 17 native West mirrors, giving 34
review cases. Artwork was approved on 2026-10-03. Completed outputs use
`generated/review/` and `generated/build/`.

Olric now has 208 sources; the 36-character collection has 4,153 sources and
16,612 variants. K retains Vanilla, Debug Blue, Hayden, Ryis and Seridia with
portrait/world synchronization. Earlier artwork, target colors and runtime code
remain unchanged. Historical pixel tests keep their fixed expectations; only
the central Olric total changes in `tests/profile_coverage.rs`.

## Source and materials

The game mount remains empty. This slice uses the original archive backup at
`tmp/slice-042-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive, without checking for a newer installed
game version. All 203 prior PNG pins match fresh exports. No pin refresh or hash
override was needed.

The profile appends 377 seeds covering 704 source pixels per target. Four existing
world shades (`#F2D7A7`, `#E4BD7C`, `#C4854C`, `#684810`) cover skin and its
contours on the face, ears and neck. Sleeves and gloves cover the arms and
hands. The extra drinking-arm shade does not occur in these strips. Winter's
gray gloves and boots use separate colors, so the shared skin-shadow exclusions
from the Autumn tool strips are unnecessary here. No new palette colors or
exclusions are needed. Gloves, tool handles and heads, swing effects, hair,
eyes, clothing and boots remain original. All
frames were inspected in five-choice contact sheets, with source grids for
material boundaries and swinging arms.

The new material test checks every pixel against an independent four-color map,
literal landmarks, per-frame counts, alpha, metadata and identical masks across
presets. Earlier sources and variants use the fixed accepted baseline
`generated/slice-042-build/characters/olric`.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Hammer and pickaxe each have seven 0.1-second frames; the three tongs strips
each have one frame. All face East with native West mirroring. Tongs retains
its complex start/loop/end progression. The original configuration includes
the Winter outfit for all three cycles and a hammer sound effect.

## Offline review

- [Five-choice sample summary](../../../generated/slice-043-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-043-review/index.html).

The summary samples hammer impact, raised pickaxe and the tongs loop. The full
gallery includes all 34 cases with enlarged face/arm details. Its full views
use an 80×50 crop spanning the canvas width to include every tool and swing-effect
pixel in either direction; summary samples use a 56×50 East-facing crop.
Generated images and game packages stay local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 255 regular tests and the
release build passed; 325 local opt-ins remain ignored in the normal run.
Another 108 local tests passed: the new material test, 103 retained corpus tests,
Olric's original portrait test and three GML runtime tests. Release artifacts
avoid recreating the previously removed debug cache.

The new package test rejected the unregistered Winter hammer strip before
registration, then passed. Omission and clothing-color spill controls both generated
successfully before failing the intended material assertion. All five raw
metadata sidecars match independent reads from the original archive.

All 16,612 combined variants validate. All 832 standalone Olric variants pass
strict recipe validation and match the combined build. The 2,030 earlier Olric
PNG/metadata files and 39,625 files for the other 35 characters remain
byte-identical. Previous runtime rows, choices and hotkeys remain exact. Source
colors, groups, target mappings, collection sets and previous export report
entries are preserved; precisely five registry entries were added.

The source archive's native animator and NPC object pass 170 frame/palette
observations, 20 linear completions and 40 complex transitions with simulated
engine services and direct cycle selection. Checks cover native mirroring,
frame phase, cycle counters, wrapper idempotence, matching Winter portrait
synchronization and tongs start/loop/end transitions under all five choices.
Natural outfit selection, scheduling, speaking/interactions, audible sound and
full-engine rendering remain untested.

All 16 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full crop, enlarged detail
and all 15 summary bindings: 7,445,648 rendered pixels. Full crops independently
check every nontransparent source pixel in both directions. The `review`
shortcut also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-043-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

The first concurrent local-test and installation attempts ran out of disk space.
The failed installation left the lab's original archive unchanged. Both checks
passed when rerun separately after space became available; the initial logs are
retained as `tmp/slice-043-*-space-failure.log`.

Evidence uses `tmp/slice-043-`; author records use `tmp/olric-winter-tools-author-`.
Fresh exports live in `extracted/olric-winter-tools-study`, reached through the
stable `extracted/test-corpus/olric` link. `evidence.json` summarizes results and
`scope.log` checks the nine expected paths and unchanged historical tests.
The pre-existing `flake.nix` and `flake.lock` edits remain untouched and outside
this artwork slice.

## Remaining work

Olric's Spring, Summer and Autumn folders remain complete. Winter has 23 strips
left. Continue with Winter polishing, wiping his brow and drinking a potion,
then the remaining work sequences and bunny-ear variants. The inventory is
recorded in `tmp/slice-043-remainder.json`.
