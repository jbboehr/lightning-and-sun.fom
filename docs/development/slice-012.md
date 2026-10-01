# Eiland Beach actions

Following the artwork accepted in `d2f0df9`, this slice adds Eiland's six Beach
action/blink/kiss strips. Their 31 source frames and 14 native West mirrors give
45 review cases. The user approved the artwork for commit on 2026-10-01. Completed outputs use
`generated/review/` and `generated/build/`.

Eiland now has 260 sources, covering 12 of 14 Beach strips. The 36-character
collection has 3,964 sources and 15,856 variants. J retains its five choices,
Vanilla default and portrait/world synchronization. Only registry/profile data,
tests and documentation change. Historical pixel tests remain byte-identical;
the growing total changes only in `tests/profile_coverage.rs`. Eiland's local
corpus link advances to the fresh export.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 254 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All six sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/54 origins.

Faces, exposed torsos, arms and legs use Eiland's existing five world skin
shades. Source-grid inspection confirmed that `#C9785A` belongs to the swimwear
trim in these strips. Sixteen trim pixels also share the skin shadow `#9C5241`;
their individual coordinates are excluded from the skin mask. Swimwear,
bracelets, sandals, eyes and hair stay original. The local material test checks
every pixel across all four palettes, with fixed per-frame changed-pixel counts
and reviewed skin/material landmarks.

Each action direction has seven frames, each blink direction three and the
East-facing kiss four. East supplies native West mirrors. The original timing
arrays remain intact, including the kiss's 0.8-second third frame and the native
action cycle's last-frame hold. No runtime code, target ramps or existing masks
change.

## Offline review

- [Five-choice sample summary](../../generated/slice-012-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-012-review/index.html).

This slice has one gallery directly in `slice-012-review/`. The summary shows
an East action, a South blink and an East kiss across all five choices. The full
review includes every source frame and West mirror, with complete character views
and enlarged details. The gallery totals 784,545 bytes (about 766 KiB).
Images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 224 regular tests and the release
build passed; the normal suite leaves 294 local opt-ins ignored. Seventy-six
local tests passed separately: one new material test, seventy-two retained corpus
tests and three GML runtime tests. Two omission/spill controls generated before
failing their intended pixel assertions, including restoring a protected trim
pixel to the skin mask. The synthetic package test rejected the unregistered
Beach action strip before the additions and passed afterward.

All 15,856 combined variants validate. All 1,040 standalone Eiland variants pass
strict recipe validation and match the combined build. All 2,540 prior Eiland
original/variant PNG and metadata files and all 37,215 files for the other 35
characters remain byte-identical. Previous runtime rows, choices and hotkeys
remain exact.

The current archive's native animator and NPC object pass 225 frame/palette
observations, 45 linear completions and 40 last-frame-hold checks using simulated
engine services and direct cycle selection. Checks cover native mirroring,
palette binding, wrapper idempotence, frame phase, cycle counters, completion
state and portrait synchronization. Natural scheduling, interaction/outfit
dispatch and full-engine rendering remain untested.

All fifteen review pages pass Chromium link/image loading and overflow checks.
Exact pixel checks cover every case, native mirrors, metadata, complete character
crops, enlarged details and all fifteen source/palette bindings in the summary:
8,105,760 rendered pixels in total. The `review` shortcut also passes Chromium
loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-012-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; five
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-012-`; author records use
`tmp/eiland-beach-actions-author-`. Fresh exports live in
`extracted/eiland-beach-actions-study`, reached by the stable
`extracted/test-corpus/eiland` link. `evidence.json` summarizes verification;
`scope.log` checks the expected nine paths and unchanged historical tests.

## Remaining inventory and next coverage

Eiland's seasonal sprite folders remain complete. His two Beach swimming strips,
15 Wedding strips and separate Children folders (four portraits and 46 sprites)
remain future work. `tmp/slice-012-remainder.json` records the inventory.

Continue with Eiland's two Beach swimming strips, then Wedding coverage.
