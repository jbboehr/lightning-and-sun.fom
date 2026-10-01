# Wedding completion and Winter studies

Following the artwork accepted in `28bec57`, this slice adds 18 strips:
54 source frames and 21 native West mirrors, giving 75 review cases.
The user approved the artwork for commit on 2026-09-30. Completed outputs use `generated/review/`
and `generated/build/`.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Valen | 9 | 34 + 15 | 257 | Wedding 15/15, complete |
| Eiland | 9 | 20 + 6 | 244 | Winter 37/41 |

The 36-character collection has 3,948 sources and 15,792 variants. O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change. Historical pixel
tests remain byte-identical; growing totals change only in
`tests/profile_coverage.rs`. Local corpus links advance to the fresh exports.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 483 previously covered PNG pins for these two characters match fresh exports.
No pin refresh or hash mismatch override was used. Earlier regions, source roles,
color groups and target ramps remain unchanged. All eighteen sidecars match
independent archive reads, retaining 80×80 frames, the Default atlas and native
Middle/54 origins.

Valen's Wedding general actions, blinking, sitting and kissing use her existing
four world skin shades for faces, hands and exposed ankles. Her suit, shoes,
eyes and hair remain original. Each action direction has seven frames;
East/South blinking have three, East kissing four and each sitting direction one.
All East frames have native West mirrors. North views expose skin and change.

Eiland's Winter magnifying-glass and seated-writing start/loop/end phases use
his existing five world skin shades for faces. Source-grid inspection found
126 coat and gold-trim pixels sharing the skin shadow `#BA6A4C`; these remain
original. Gloves, magnifying-glass rim and lens, writing tools, boots, eyes and
hair also remain original. The local material test independently records every
protected coordinate and checks all remaining skin pixels across all four
palettes. Magnifying-glass phases use 3/1/2 start/loop/end frames in East and
South, with West mirrors. Seated writing uses 2/4/2 South-only frames. All native
phase timings are retained, including writing's varied per-frame durations.

## Offline review

- [Five-choice sample summary](../../generated/slice-009-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-009-review/index.html).

The combined index links both `slice-009-<character>-review/` galleries.
Every new source frame and native West mirror appears, with enlarged details
alongside each full view. The three review directories total 1,469,671 bytes
(about 1.4 MiB). Images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 221 regular tests and the release
build passed; the normal suite leaves 291 local opt-ins ignored. Seventy-three
local tests passed separately: two new material tests, sixty-eight retained
corpus tests and three GML runtime tests. Four omission/spill controls generated
before failing their intended pixel assertions, including restoring a protected
Winter coat pixel to the skin mask. The synthetic package test rejected Valen's
unregistered Wedding action strip before the additions and passed afterward.

All 15,792 combined variants validate. All 2,004 standalone variants pass strict
recipe validation and match the combined build. All 4,830 prior original/variant
PNG and metadata files for these characters and all 34,640 files for the other
34 remain byte-identical. Previous runtime rows, choices and hotkeys remain exact.

The current archive's native animator and NPC object pass 375 frame/palette
observations, 65 linear completions, 40 last-frame hold checks and 80 complex-phase
transitions using simulated engine services and direct cycle selection. Checks
cover native mirroring, palette binding, wrapper idempotence, frame phase, cycle
counters, completion state, seated flags and portrait synchronization. Natural
scheduling, interaction/outfit dispatch, attached effects and full-engine
rendering remain untested.

All 34 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks cover every case, native mirrors, metadata and complete opaque
crops, plus all ten source/palette bindings in the combined summary. The
`review` shortcut also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-009-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; eight
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-009-`. Author records use
`tmp/valen-wedding-finish-author-` and `tmp/eiland-winter-magnify-author-`.
Fresh exports live in the corresponding `extracted/<character>-<batch>-study`
directories, reached by the stable `extracted/test-corpus/<character>` links.
`evidence.json` summarizes verification; `scope.log` checks the expected eleven
paths and unchanged historical tests.

## Remaining inventory and next coverage

The local archive inventory now finds every adult portrait and sprite under
`assets/animations/NPCs/Valen/` covered. Only her separate Children folders remain:
four portrait strips and 46 sprite strips. `tmp/slice-009-remainder.json` records
that inventory. Valen can leave the adult coverage rotation alongside Balor.

Finish Eiland's four remaining Winter tool animations next: axe, pickaxe, brush
and trowel. All are present in the current archive. His Beach and Wedding folders,
and the separate Children assets, remain future work.
