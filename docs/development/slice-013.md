# Eiland Beach completion

Following the artwork accepted in `d2a7452`, this slice adds Eiland's two Beach
swimming strips. Their eight source frames and four native West mirrors give
12 review cases. The user approved the artwork for commit on 2026-10-01. Completed outputs use
`generated/review/` and `generated/build/`.

Eiland now has 262 sources, covering all 14 Beach strips. The 36-character
collection has 3,966 sources and 15,864 variants. J retains its five choices,
Vanilla default and portrait/world synchronization. Only registry/profile data,
tests and documentation change. Historical pixel tests remain byte-identical;
the growing total changes only in `tests/profile_coverage.rs`. Eiland's local
corpus link advances to the fresh export.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 260 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. Both sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/54 origins.

The exposed face uses Eiland's existing five world skin shades. Hair, eyes,
water, foam and detached droplets remain original. These strips need no
shared-color clothing exclusions or new color roles. The local material test
checks every pixel across all four palettes, with fixed per-frame changed-pixel
counts and reviewed skin/material landmarks. Both directions expose less skin
in their third and fourth frames as the waterline rises.

Each strip has four frames with 0.15-second durations. East supplies the native
West mirror; the native cycle defines South and East. No runtime code, target
ramps or existing masks change.

## Offline review

- [Five-choice sample summary](../../generated/slice-013-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-013-review/index.html).

This slice has one gallery directly in `slice-013-review/`. The summary shows
the first and third frames of each direction across all five choices. The full
review includes every source frame and West mirror, with complete water effects
and enlarged face/waterline details. The gallery totals 233,893 bytes (about
228 KiB). Images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 225 regular tests and the release
build passed; the normal suite leaves 295 local opt-ins ignored. Seventy-seven
local tests passed separately: one new material test, seventy-three retained
corpus tests and three GML runtime tests. Two omission/spill controls generated
before failing their intended pixel assertions, including recoloring a protected
foam pixel. The synthetic package test rejected the unregistered swimming strip
before the additions and passed afterward.

All 15,864 combined variants validate. All 1,048 standalone Eiland variants pass
strict recipe validation and match the combined build. All 2,600 prior Eiland
original/variant PNG and metadata files and all 37,215 files for the other 35
characters remain byte-identical. Previous runtime rows, choices and hotkeys
remain exact.

The current archive's native animator and NPC object pass 60 frame/palette
observations and 15 linear completions using simulated engine services and direct
cycle selection. Checks cover native mirroring, palette binding, wrapper
idempotence, frame phase, cycle counters, completion state and portrait
synchronization. Natural scheduling, interaction/outfit dispatch and full-engine
rendering remain untested.

All five review pages pass Chromium link/image loading and overflow checks.
Exact pixel checks cover every case, native mirrors, metadata, complete water
effects, enlarged details and all twenty source/palette bindings in the summary:
1,972,992 rendered pixels in total. The `review` shortcut also passes Chromium
loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-013-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; five
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-013-`; author records use
`tmp/eiland-beach-swim-author-`. Fresh exports live in
`extracted/eiland-beach-swim-study`, reached by the stable
`extracted/test-corpus/eiland` link. `evidence.json` summarizes verification;
`scope.log` checks the expected nine paths and unchanged historical tests.

## Remaining inventory and next coverage

Eiland's seasonal and Beach sprite folders are complete. His 15 Wedding strips
and separate Children folders (four portraits and 46 sprites) remain future
work. `tmp/slice-013-remainder.json` records the inventory.

Continue with Eiland's six Wedding idle/walk strips, then the remaining Wedding
actions. Children assets remain separate future work.
