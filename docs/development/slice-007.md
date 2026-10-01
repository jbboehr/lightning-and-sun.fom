# Wedding and Beach completion, Winter actions

Following the artwork accepted in `d2e5613`, this slice adds 16 strips:
68 source frames and 31 native West mirrors, giving 99 review cases.
The user approved the artwork for commit on 2026-09-30. The completed review and build are available
through `generated/review/` and `generated/build/`.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 9 | 34 + 15 | 262 | Wedding 15/15, complete |
| Valen | 2 | 8 + 4 | 242 | Beach 14/14, complete |
| Eiland | 5 | 26 + 12 | 229 | Winter 22/41 |

The 36-character collection has 3,918 sources and 15,672 variants. I, O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change. Historical pixel
tests remain byte-identical; the three growing totals change only in
`tests/profile_coverage.rs`. Local corpus links advance to the fresh exports.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 717 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All 16 sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/54 origins.

Balor's Wedding general actions, blinking, sitting and kissing use his existing
four world skin shades for faces and hands. His suit, tie, trousers, shoes,
eyes and hair stay original. General actions have seven frames per direction,
East/South blinking three, East kissing four and sitting one per direction.
East frames have native West mirrors.

Valen's East/South Beach swimming use her existing four world shades for faces.
Water, splashes, eyes and hair remain original. Each strip has four frames;
East swimming has native West mirrors. Full review crops include the splashes
through row 63.

Eiland's Winter general actions, sleeping and kissing use his existing five
world shades for faces and visible hands. His coat, cape, gold trim, trousers,
boots, eyes and hair stay original. These strips need no shared-color clothing
exclusions. Each action direction has seven frames; East kissing has four and
East sleeping one. All East frames have native West mirrors.

## Offline review

- [Five-choice sample summary](../../generated/slice-007-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-007-review/index.html).

The combined index links all three `slice-007-<character>-review/` galleries.
Every new source frame and native West mirror appears, with enlarged details
alongside each full view. The four review directories total 1,799,893 bytes
(about 1.7 MiB). Images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 219 regular tests and the release
build passed; the normal suite leaves 287 local opt-ins ignored. Sixty-nine local
tests passed separately: three new material tests, sixty-three retained corpus
tests and three GML runtime tests. Six omission/spill controls generated before
failing their intended pixel assertions. The synthetic package test rejected
the unregistered Wedding action strip before the additions and passed afterward.

All 15,672 combined variants validate. All 2,932 standalone variants pass strict
recipe validation and match the combined build. All 7,170 prior original/variant
PNG and metadata files for these characters and all 32,015 files for the other
33 remain byte-identical. Previous runtime rows, choices and hotkeys remain exact.

The current archive's native animator and NPC object pass 495 frame/palette
observations, 120 linear completions and 80 last-frame hold checks using
simulated engine services and direct cycle selection. Checks cover mirroring,
palette binding, wrapper idempotence, frame phase, cycle counters, completion
state and portrait synchronization. These cycles have no complex phases.
Natural scheduling, interaction/outfit dispatch, attached effects and
full-engine rendering remain untested.

All 39 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks cover every case, native mirrors, metadata and complete opaque
crops, plus all fifteen source/palette bindings in the combined summary.
The `review` shortcut also passes Chromium loading; its 36 linked HTML pages and
71 image references resolve through the short path.

MOMI installation passed in the isolated `tmp/slice-007-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; eleven
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-007-`. Author records use `tmp/balor-wedding-finish-author-`,
`tmp/valen-beach-finish-author-` and `tmp/eiland-winter-standard-author-`.
Fresh exports live in their corresponding `extracted/<character>-<batch>-study`
directories, reached by the stable `extracted/test-corpus/<character>` links.
`evidence.json` summarizes verification; `scope.log` checks the expected 13 paths
and unchanged historical tests.

## Remaining inventory and next coverage

The local inventory of `assets/animations/NPCs/Balor/` now finds every adult
portrait and body sprite covered except
`Sprites/spr_npc_balor_specialanimation_shadow_jump_east.png`. Inspection confirms
that this 22-frame strip contains only 1,496 opaque `#252B43` pixels and 139,304
transparent pixels: a ground shadow, with no skin to recolor. Its PNG SHA-256 is
`7887306b7a40e71944af9914112abd98a06d4a30032f5a9750047266148c85e2`.
It stays untouched and is not added to the palette profile or variant count.
The local `remainder.json` and `shadow-audit.json` record this check.

Continue with Valen's six Wedding idle/walk strips and Eiland's six Winter
seated-reading/standing-writing strips, all present in the current archive.
Balor can leave the adult coverage rotation. The separate Children folders,
including 46 sprites and four portraits under each of these three characters,
remain future work.
