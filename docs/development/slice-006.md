# Wedding movement, Beach and Winter actions

Following the artwork accepted in `eca26e2`, this slice adds 23 strips:
77 source frames and 31 native West mirrors, giving 108 review cases.
The user approved the artwork on 2026-09-30. The completed review and build are available
through `generated/review/` and `generated/build/`.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 6 | 15 + 5 | 253 | Wedding 6/15 |
| Valen | 6 | 31 + 14 | 240 | Beach 12/14 |
| Eiland | 11 | 31 + 12 | 224 | Winter 17/41 |

The 36-character collection has 3,902 sources and 15,608 variants. I, O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change. Historical pixel
tests remain byte-identical; the three growing totals change only in
`tests/profile_coverage.rs`. Local corpus links advance to the fresh exports.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 694 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All 23 sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/54 origins.

Balor's Wedding idle and walking use his existing four world skin shades for
faces and hands. His suit, tie, trousers, shoes, eyes and hair stay original.
All three source directions are covered, with native West mirrors of East.
Each idle strip has one frame and each walk four.

Valen's Beach general actions, blinking and kissing use her existing four world
shades for faces, exposed shoulders, arms, hands, legs and feet. Her swimsuit,
wrap, eyes and hair stay original. General actions cover all three source
directions with seven frames each; East/South blinking has three frames each,
and East kissing has four. East frames have native West mirrors.

Eiland's Winter blinking, sitting, eating and drinking use his existing five
world shades for faces and visible hands. His coat, cape, gold trim, trousers,
boots, food, cup, eyes and hair stay original. These strips need no shared-color
clothing exclusions. East/South blinking and all three drinking directions
have three frames each; sitting has one per direction. East/South eating has
five frames, while North eating has three. All East frames have West mirrors.

## Offline review

- [Five-choice sample summary](../../generated/slice-006-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-006-review/index.html).

The combined index links all three `slice-006-<character>-review/` galleries.
Every new source frame and native West mirror appears, with enlarged details
alongside each full view. The four review directories total 2,026,750 bytes
(about 1.9 MiB). Images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 218 regular tests and the release
build passed; the normal suite leaves 284 local opt-ins ignored. Sixty-six local
tests passed separately: three new material tests, sixty retained corpus tests
and three GML runtime tests. Six omission/spill controls generated before failing
their intended pixel assertions. The synthetic package test rejected the
unregistered Wedding idle strip before the additions and passed afterward.

All 15,608 combined variants validate. All 2,868 standalone variants pass strict
recipe validation and match the combined build. All 6,940 prior original/variant
PNG and metadata files for these characters and all 32,015 files for the other
33 remain byte-identical. Previous runtime rows, choices and hotkeys remain exact.

The current archive's native animator and NPC object pass 540 frame/palette
observations, 160 linear completions and 120 last-frame hold checks using
simulated engine services and direct cycle selection. Checks cover mirroring,
palette binding, wrapper idempotence, frame phase, cycle counters, completion
state and portrait synchronization. These cycles have no complex phases.
Natural scheduling, interaction/outfit dispatch, attached effects and
full-engine rendering remain untested.

All 46 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks cover every case, native mirrors, metadata and complete opaque
crops, plus all fifteen source/palette bindings in the combined summary.
The `review` shortcut also passes Chromium loading; its 43 linked HTML pages and
85 image references resolve through the short path.

MOMI installation passed in the isolated `tmp/slice-006-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; eleven
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-006-`. Author records use `tmp/balor-wedding-world-author-`,
`tmp/valen-beach-actions-author-` and `tmp/eiland-winter-actions-author-`.
Fresh exports live in their corresponding `extracted/<character>-<batch>-study`
directories, reached by the stable `extracted/test-corpus/<character>` links.
`evidence.json` summarizes verification; `scope.log` checks the expected 13 paths
and unchanged historical tests.

## Next coverage

Finish Balor's Wedding folder with blinking, sitting, general actions and kissing;
finish Valen's Beach folder with East/South swimming; add Eiland's Winter general
actions, sleeping and kissing. These sixteen strips are present in the current
archive.
