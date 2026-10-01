# Eiland Winter completion

Following the artwork accepted in `83e9b41`, this slice adds Eiland's four
remaining Winter strips: axe, pickaxe, brush and trowel. Their 29 source frames
and 29 native West mirrors give 58 review cases. The user approved the artwork
for commit on 2026-10-01.
Completed outputs use `generated/review/` and `generated/build/`.

Eiland now has 248 sources, covering all 41 Winter strips. The 36-character
collection has 3,952 sources and 15,808 variants. J retains its five choices,
Vanilla default and portrait/world synchronization. Only registry/profile data,
tests and documentation change. Historical pixel tests remain byte-identical;
the growing total changes only in `tests/profile_coverage.rs`. Eiland's local
corpus link advances to the fresh export.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 244 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All four sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/54 origins.

These tools use Eiland's existing five world skin shades for faces. Source-grid
inspection found 153 coat and gold-trim pixels sharing the skin shadow `#BA6A4C`;
these remain original. Gloves, tools, swing effects, boots, eyes and hair also
remain original. The local material test independently records every protected
coordinate and checks all remaining skin pixels across all four palettes.

Axe and pickaxe have six frames each, brush seven and trowel ten. All four are
East-facing linear cycles with native West mirrors. The native per-frame timings
are retained, including the trowel's 1.4-second final frame. Full-frame views
preserve effects reaching the sprite's right edge, which would be clipped by
an ordinary character crop.

## Offline review

- [Five-choice sample summary](../../generated/slice-010-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-010-review/index.html).

This slice has one gallery directly in `slice-010-review/`. The summary shows
one sample of each tool across all five choices. The full review includes every
source frame and West mirror, with complete 80×80 views and enlarged character
details. The review directory totals 1,069,747 bytes (about 1.0 MiB). Images and
packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 222 regular tests and the release
build passed; the normal suite leaves 292 local opt-ins ignored. Seventy-four
local tests passed separately: one new material test, seventy retained corpus
tests and three GML runtime tests. Two omission/spill controls generated before
failing their intended pixel assertions, including restoring a protected Winter
coat pixel to the skin mask. The synthetic package test rejected the unregistered
Winter axe strip before the additions and passed afterward.

All 15,808 combined variants validate. All 992 standalone Eiland variants pass
strict recipe validation and match the combined build. All 2,440 prior Eiland
original/variant PNG and metadata files and all 37,215 files for the other 35
characters remain byte-identical. Previous runtime rows, choices and hotkeys
remain exact.

The current archive's native animator and NPC object pass 290 frame/palette
observations and 40 linear completions using simulated engine services and direct
cycle selection. Checks cover native mirroring, palette binding, wrapper
idempotence, frame phase, cycle counters, completion state and portrait
synchronization. Natural scheduling, interaction/outfit dispatch, attached effects
and full-engine rendering remain untested.

All twenty review pages pass Chromium link/image loading and overflow checks.
Exact pixel checks cover every case, native mirrors, metadata, complete frames,
enlarged details and all twenty source/palette bindings in the summary:
12,182,400 rendered pixels in total. The `review` shortcut also passes Chromium
loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-010-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; five
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-010-`; author records use
`tmp/eiland-winter-finish-author-`. Fresh exports live in
`extracted/eiland-winter-finish-study`, reached by the stable
`extracted/test-corpus/eiland` link. `evidence.json` summarizes verification;
`scope.log` checks the expected nine paths and unchanged historical tests.

## Remaining inventory and next coverage

The local archive inventory confirms that Eiland's four seasonal sprite folders
are complete. His remaining adult assets are 14 Beach strips and 15 Wedding
strips. The separate Children folders contain four portraits and 46 sprites.
`tmp/slice-010-remainder.json` records this inventory.

Continue with Eiland's six Beach idle/walk strips, then the remaining Beach and
Wedding actions. Children assets remain separate future work.
