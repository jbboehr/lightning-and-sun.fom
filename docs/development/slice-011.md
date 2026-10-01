# Eiland Beach movement

Following the artwork accepted in `d45db99`, this slice adds Eiland's six Beach
idle/walk strips. Their 15 source frames and five native West mirrors give
20 review cases. The user approved the artwork for commit on 2026-10-01. Completed outputs use
`generated/review/` and `generated/build/`.

Eiland now has 254 sources, covering six of 14 Beach strips. The 36-character
collection has 3,958 sources and 15,832 variants. J retains its five choices,
Vanilla default and portrait/world synchronization. Only registry/profile data,
tests and documentation change. Historical pixel tests remain byte-identical;
the growing total changes only in `tests/profile_coverage.rs`. Eiland's local
corpus link advances to the fresh export.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 248 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All six sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/54 origins.

Faces, exposed torsos, arms and legs use Eiland's existing five world skin
shades. Source-grid inspection confirmed that `#C9785A` belongs to the swimwear
trim in these strips; it remains original even though the portrait recipe also
uses that shade. Pink swimwear, bracelets, sandals, eyes and hair stay original.
Ten trim pixels also share the skin shadow `#9C5241`; their individual
coordinates are excluded from the skin mask. The local material test checks
every pixel across all four palettes, with fixed
per-frame changed-pixel counts and reviewed skin/material landmarks.

Each idle direction has one frame; each walk direction has four. East supplies
the native West mirror. Walk frames retain their native 0.15-second durations.
No runtime code, target ramps or existing source masks change.

## Offline review

- [Five-choice sample summary](../../generated/slice-011-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-011-review/index.html).

This slice has one gallery directly in `slice-011-review/`. The summary shows
standing South, walking East and walking North across all five choices. The
full review includes every source frame and West mirror, with complete character
views and enlarged details. The gallery totals 403,260 bytes (about 394 KiB).
Images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 223 regular tests and the release
build passed; the normal suite leaves 293 local opt-ins ignored. Seventy-five
local tests passed separately: one new material test, seventy-one retained corpus
tests and three GML runtime tests. The swimwear regression test failed on a
recolored trim pixel before the correction and passed afterward. Two omission/spill
controls generated before failing their intended pixel assertions, including
restoring a protected trim pixel to the skin mask. The synthetic package test
rejected the unregistered Beach idle strip before the additions and passed afterward.

All 15,832 combined variants validate. All 1,016 standalone Eiland variants pass
strict recipe validation and match the combined build. All 2,480 prior Eiland
original/variant PNG and metadata files and all 37,215 files for the other 35
characters remain byte-identical. Previous runtime rows, choices and hotkeys
remain exact.

The current archive's native animator and NPC object pass 100 frame/palette
observations and 40 linear completions using simulated engine services and direct
cycle selection. Checks cover native mirroring, palette binding, wrapper
idempotence, frame phase, cycle counters, completion state and portrait
synchronization. Natural scheduling, interaction/outfit dispatch and full-engine
rendering remain untested.

All ten review pages pass Chromium link/image loading and overflow checks.
Exact pixel checks cover every case, native mirrors, metadata, complete character
crops, enlarged details and all fifteen source/palette bindings in the summary:
3,862,560 rendered pixels in total. The `review` shortcut also passes Chromium
loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-011-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; five
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-011-`; author records use
`tmp/eiland-beach-world-author-`. Fresh exports live in
`extracted/eiland-beach-world-study`, reached by the stable
`extracted/test-corpus/eiland` link. `evidence.json` summarizes verification;
`scope.log` checks the expected nine paths and unchanged historical tests.

## Remaining inventory and next coverage

Eiland's seasonal sprite folders remain complete. Eight Beach strips remain:
three general actions, two blinks, one kiss and two swimming strips. His 15
Wedding strips and separate Children folders (four portraits and 46 sprites)
remain future work. `tmp/slice-011-remainder.json` records the inventory.

Continue with Eiland's six Beach action/blink/kiss strips, then swimming.
