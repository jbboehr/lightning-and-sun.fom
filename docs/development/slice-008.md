# Wedding movement and Winter reading/writing

Following the artwork accepted in `347f2b7`, this slice adds 12 strips:
33 source frames and five native West mirrors, giving 38 review cases.
The user approved the artwork for commit on 2026-09-30. Completed outputs use `generated/review/`
and `generated/build/`.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Valen | 6 | 15 + 5 | 248 | Wedding 6/15 |
| Eiland | 6 | 18 + 0 | 235 | Winter 28/41 |

The 36-character collection has 3,930 sources and 15,720 variants. O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change. Historical pixel
tests remain byte-identical; growing totals change only in
`tests/profile_coverage.rs`. Local corpus links advance to the fresh exports.
Balor's completed adult coverage stays unchanged.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 471 previously covered PNG pins for these two characters match fresh exports.
No pin refresh or hash mismatch override was used. Earlier regions, source roles,
color groups and target ramps remain unchanged. All twelve sidecars match
independent archive reads, retaining 80×80 frames, the Default atlas and native
Middle/54 origins.

Valen's Wedding idle and walking use her existing four world skin shades for
faces, hands and exposed ankles. Her suit, shoes, eyes and hair remain original.
Each idle direction has one frame; each walking direction has four, with native
East-to-West mirroring. Unlike her covered seasonal north-facing sprites, these
Wedding North views expose hands and ankles and therefore change.

Eiland's Winter seated-reading and standing-writing start/loop/end phases use
his existing five world skin shades for faces. Source-grid inspection found
68 coat and gold-trim pixels sharing the skin shadow `#BA6A4C`; these remain
original. Gloves, books, writing tools, boots, eyes and hair also remain original.
The local material test independently records every protected coordinate and
checks all remaining skin pixels across all four palettes. Reading uses 3/4/3
start/loop/end frames; writing uses 2/4/2. These South-only complex cycles retain
native timing, including reading's three-second pauses and writing's varied
per-frame durations.

## Offline review

- [Five-choice sample summary](../../generated/slice-008-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-008-review/index.html).

The combined index links both `slice-008-<character>-review/` galleries.
Every new source frame and native West mirror appears, with enlarged details
alongside each full view. The three review directories total 827,174 bytes
(about 808 KiB). Images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 220 regular tests and the release
build passed; the normal suite leaves 289 local opt-ins ignored. Seventy-one
local tests passed separately: two new material tests, sixty-six retained corpus
tests and three GML runtime tests. Four omission/spill controls generated before
failing their intended pixel assertions, including restoring a protected Winter
coat pixel to the skin mask. The synthetic package test rejected Valen's
unregistered Wedding idle strip before the additions and passed afterward.

All 15,720 combined variants validate. All 1,932 standalone variants pass strict
recipe validation and match the combined build. All 4,710 prior original/variant
PNG and metadata files for these characters and all 34,640 files for the other
34 remain byte-identical. Previous runtime rows, choices and hotkeys remain exact.

The current archive's native animator and NPC object pass 190 frame/palette
observations, 40 linear completions and 40 complex-phase transitions using
simulated engine services and direct cycle selection. Checks cover native
mirroring, palette binding, wrapper idempotence, frame phase, cycle counters,
completion state, seated flags and portrait synchronization. Natural scheduling,
interaction/outfit dispatch, attached effects and full-engine rendering remain
untested.

All nineteen review pages pass Chromium link/image loading and overflow checks.
Exact pixel checks cover every case, native mirrors, metadata and complete opaque
crops, plus all ten source/palette bindings in the combined summary. The
`review` shortcut also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-008-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; eight
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-008-`. Author records use
`tmp/valen-wedding-world-author-` and `tmp/eiland-winter-writing-author-`.
Fresh exports live in the corresponding `extracted/<character>-<batch>-study`
directories, reached by the stable `extracted/test-corpus/<character>` links.
`evidence.json` summarizes verification; `scope.log` checks the expected eleven
paths and unchanged historical tests.

## Next coverage

Finish Valen's nine remaining Wedding strips: general actions, blinking,
sitting and kissing. Add Eiland's Winter seated-writing and magnifying-glass
cycles, nine strips, before his final four Winter tool animations. All are
present in the current archive. His Beach and Wedding folders and the separate
Children assets remain future work.
