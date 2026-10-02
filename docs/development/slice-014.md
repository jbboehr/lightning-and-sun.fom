# Eiland Wedding movement

Following the artwork accepted in `c22879a`, this slice adds Eiland's six Wedding
idle/walk strips. Their 15 source frames and five native West mirrors give
20 review cases. The user approved the artwork for commit on 2026-10-01. Completed outputs use
`generated/review/` and `generated/build/`.

Eiland now has 268 sources, covering six of 15 Wedding strips. The 36-character
collection has 3,972 sources and 15,888 variants. J retains its five choices,
Vanilla default and portrait/world synchronization. Only registry/profile data,
tests and documentation change. Historical pixel tests remain byte-identical;
the growing total changes only in `tests/profile_coverage.rs`. Eiland's local
corpus link advances to the fresh export.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 262 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All six sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/54 origins.

Faces and exposed necks use Eiland's existing five world skin shades. Ninety-two
clothing and gold-trim pixels share the skin shadow `#9C5241`; their individually
reviewed coordinates are excluded from the masks. White clothing, gloves, boots,
hair and eyes remain original. The local material test checks every pixel across
all four palettes, with fixed per-frame changed-pixel counts and reviewed
skin/material landmarks.

The north-facing idle and walk strips show only hair and clothing. Their region
seeds are empty, and the material test checks that every generated north-facing
PNG retains the original bytes. Those views remain in the registry and review.

Each idle direction has one frame; each walk direction has four. East supplies
the native West mirror. Walk frames retain their native 0.15-second durations.
No runtime code, target ramps or existing source masks change.

## Offline review

- [Five-choice sample summary](../../generated/slice-014-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-014-review/index.html).

This slice has one gallery directly in `slice-014-review/`. The summary shows
standing South, walking East and walking North across all five choices. The
full review includes every source frame and West mirror, with complete character
views and enlarged details. The unchanged north-facing views are included.
The gallery totals 418,709 bytes (about 409 KiB). Images and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 226 regular tests and the release
build passed; the normal suite leaves 296 local opt-ins ignored. Seventy-eight
local tests passed separately: one new material test, seventy-four retained
corpus tests and three GML runtime tests. Two omission/spill controls generated
before failing their intended pixel assertions, including restoring a protected
gold-trim pixel to the skin mask. The synthetic package test rejected the
unregistered Wedding idle strip before the additions and passed afterward.

All 15,888 combined variants validate. All 1,072 standalone Eiland variants pass
strict recipe validation and match the combined build. All 2,620 prior Eiland
original/variant PNG and metadata files and all 37,215 files for the other 35
characters remain byte-identical. Previous runtime rows, choices and hotkeys
remain exact. The new north-facing strips retain their source PNG bytes in
all four variants.

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

MOMI installation passed in the isolated `tmp/slice-014-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; five
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-014-`; author records use
`tmp/eiland-wedding-world-author-`. Fresh exports live in
`extracted/eiland-wedding-world-study`, reached by the stable
`extracted/test-corpus/eiland` link. `evidence.json` summarizes verification;
`scope.log` checks the expected nine paths and unchanged historical tests.

## Remaining inventory and next coverage

Eiland's seasonal and Beach sprite folders remain complete. Nine Wedding strips
remain: three actions, two blinks, three sitting directions and one kiss. His
separate Children folders (four portraits and 46 sprites) remain future work.
`tmp/slice-014-remainder.json` records the inventory.

Continue with Eiland's remaining nine Wedding strips to finish his adult inventory.
Children assets remain separate future work.
