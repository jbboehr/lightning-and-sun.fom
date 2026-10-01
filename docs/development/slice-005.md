# Beach completion and seasonal movement

Following the artwork accepted in `1952236`, this slice adds 15 strips:
43 source frames and 14 native West mirrors, giving 57 review cases.
The user approved the artwork on 2026-09-30. The completed review and build are available
through `generated/review/` and `generated/build/`.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 3 | 13 + 4 | 247 | Beach 18/18, complete |
| Valen | 6 | 15 + 5 | 234 | Beach 6/14 |
| Eiland | 6 | 15 + 5 | 213 | Winter 6/41 |

The 36-character collection has 3,879 sources and 15,516 variants. I, O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change. Historical pixel
tests remain byte-identical; the three growing totals change only in
`tests/profile_coverage.rs`. Local corpus links advance to the fresh exports.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 679 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All 15 sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/54 origins.

Balor's East/South swimming and South hair flip use his existing four world
skin shades. Each swimming strip has four frames; the hair flip has five.
Faces and exposed body/limb skin change. Water, splashes, open shirt, shorts,
eyes and hair remain original. East swimming has native West mirrors. Tests
and packaging retain the hair flip's distinct `specialanimation_beach` name.

Valen's Beach idle and walking use her existing four world shades for faces,
exposed shoulders, arms, hands, legs and feet. Her swimsuit, wrap, eyes and hair
stay original. Unlike her covered seasonal North sprites, these Beach North
frames expose skin and are recolored too.

Eiland's Winter idle and walking use his existing five world shades for faces
and moving hands. His Winter coat, cape, gold trim, trousers, boots, eyes and
hair stay original. These strips need no shared-color clothing exclusions.
Both movement pilots cover East, North and South, with native West mirrors of
East frames. Each idle has one frame and each walk four.

## Offline review

- [Five-choice sample summary](../../generated/slice-005-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-005-review/index.html).

The combined index links all three `slice-005-<character>-review/` galleries.
Every new source frame and native West mirror appears. The initial crop check
caught Balor's swimming splashes below the usual character crop; his full views
now extend through row 63 and pass the complete opaque-pixel check. Enlarged
details accompany each full view. The four review directories total 1,191,115
bytes (about 1.1 MiB). Images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 217 regular tests and the release
build passed; the normal suite leaves 281 local opt-ins ignored. Sixty-three
local tests passed separately: three new material tests, fifty-seven retained
corpus tests and three GML runtime tests. Six omission/spill controls generated
before failing their intended pixel assertions. The synthetic package test
rejected the unregistered Beach swimming strip before the additions and passed
afterward.

All 15,516 combined variants validate. All 2,776 standalone variants pass strict
recipe validation and match the combined build. All 6,790 prior original/variant
PNG and metadata files for these characters and all 32,015 files for the other
33 remain byte-identical. Previous runtime rows, choices and hotkeys remain exact.

The current archive's native animator and NPC object pass 285 frame/palette
observations and 100 linear completions using simulated engine services and
direct cycle selection. Checks cover mirroring, palette binding, wrapper
idempotence, frame phase, cycle counters, completion state and portrait
synchronization. These cycles have no complex phases or last-frame hold cases.
Natural scheduling, interaction/outfit dispatch, attached effects and
full-engine rendering remain untested.

All 28 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks cover every case, native mirrors, metadata and complete opaque
crops, plus all fifteen source/palette bindings in the combined summary.
The `review` shortcut also passes Chromium loading; its 25 linked HTML pages and
49 image references resolve through the short path.

MOMI installation passed in the isolated `tmp/slice-005-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; eleven
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-005-`. Author records use `tmp/balor-beach-finish-author-`,
`tmp/valen-beach-world-author-` and `tmp/eiland-winter-world-author-`.
Fresh exports live in their corresponding `extracted/<character>-<batch>-study`
directories, reached by the stable `extracted/test-corpus/<character>` links.
`evidence.json` summarizes verification; `scope.log` checks the expected 13 paths
and unchanged historical tests.

## Next coverage

Start Balor's Wedding idle/walk, add Valen's Beach blinks, general actions and
kissing, and add Eiland's Winter blinking, sitting, eating and drinking.
These 23 strips are present in the current archive. Valen's two swimming strips
can follow as a small Beach completion batch.
