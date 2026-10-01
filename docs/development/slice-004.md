# Beach actions, Winter studies and Autumn tools

Following the artwork accepted in `b49afb8`, this slice adds 22 strips:
87 source frames and 50 native West mirrors, giving 137 review cases.
The user approved the artwork on 2026-09-30. The completed review and build are available
through `generated/review/` and `generated/build/`.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 9 | 34 + 15 | 244 | Beach 15/18 |
| Valen | 9 | 24 + 6 | 228 | Winter 31/31, complete |
| Eiland | 4 | 29 + 29 | 207 | Autumn 41/41, complete |

The 36-character collection has 3,864 sources and 15,456 variants. I, O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change. Historical pixel
tests remain byte-identical; the three growing totals change only in
`tests/profile_coverage.rs`. Local corpus links advance to the fresh exports.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 657 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All 22 sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/40 horizontal and
54 vertical origins.

Balor's Beach general actions, blinking, sitting and kissing use his existing
four world skin shades for faces, exposed chests, arms, hands, legs and feet.
The open shirt, shorts, eyes and hair remain original. General actions and
sitting cover all three source directions; blinking covers East and South,
and kissing East. Each action has seven frames, each blink three, each sitting
strip one and kissing four. East frames have native West mirrors.

Valen's Winter healing, seated reading and standing writing use her existing
four world shades for faces and moving hands. The coat, cuffs, goggles, book,
clipboard, trousers, boots, eyes and hair stay original. Healing has 1/4/1
start/loop/end frames, seated reading 3/4/3 and writing 2/4/2. Healing faces East
with native West mirrors; reading and writing face South.

Eiland's Autumn axe, pickaxe, brush and trowel use his existing five world
shades for faces and moving hands. Thirty-three clothing pixels share skin
shadow `#BA6A4C`: five each in axe and pickaxe, seventeen in brush and six in
trowel. Their disconnected components stay unselected; the material test
enumerates their literal coordinates and checks source colors and unchanged
outputs. The tools, swing effects, cape, gold trim, pink clothing, boots, eyes
and hair stay original, including outfit shade `#C9785A`. Axe and pickaxe each
have six frames, brush seven and trowel ten, all facing East with West mirrors.

## Offline review

- [Five-choice sample summary](../../generated/slice-004-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-004-review/index.html).

The combined index links all three `slice-004-<character>-review/` galleries.
Every new source frame and native West mirror appears. Eiland's full views
include the entire 80×80 canvas so tool swings and effects are visible, alongside
enlarged details. The four review directories total 2,524,441 bytes (about
2.4 MiB). Images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 216 regular tests and the release
build passed; the normal suite leaves 278 local opt-ins ignored. Sixty local
tests passed separately: three new material tests, fifty-four retained corpus
tests and three GML runtime tests. Six omission/spill controls generated before
failing their intended pixel assertions. Eiland's spill control restores a
shared-color clothing seed. The synthetic package test rejected the
unregistered Beach action strip before the additions and passed afterward.

All 15,456 combined variants validate. All 2,716 standalone variants pass strict
recipe validation and match the combined build. All 6,570 prior original/variant
PNG and metadata files for these characters and all 32,015 files for the other
33 remain byte-identical. Previous runtime rows, choices and hotkeys remain exact.

The current archive's native animator and NPC object pass 685 frame/palette
observations, 105 linear completions, 40 last-frame hold checks and 80 complex
phase transitions using simulated engine services and direct cycle selection.
Checks cover mirroring, palette binding, wrapper idempotence, frame phase, cycle
counters, completion state and portrait synchronization. Natural scheduling,
interaction/outfit dispatch, attached effects and full-engine rendering remain
untested.

All 54 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks cover every case, native mirrors, metadata and complete opaque
crops, plus all fifteen source/palette bindings in the combined summary.
The `review` shortcut also passes Chromium loading; its 51 linked HTML pages and
101 image references resolve through the short path.

MOMI installation passed in the isolated `tmp/slice-004-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; eleven
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-004-`. Author records use `tmp/balor-beach-actions-author-`,
`tmp/valen-winter-finish-author-` and `tmp/eiland-autumn-finish-author-`.
Fresh exports live in their corresponding `extracted/<character>-<batch>-study`
directories, reached by the stable `extracted/test-corpus/<character>` links.
Eiland's `author-clothing.json` records the 33 protected clothing pixels, also
retained in the tracked material test. `evidence.json` summarizes verification;
`scope.log` checks the expected 13 paths and unchanged historical tests.

## Next coverage

Finish Balor's Beach folder with East/South swimming and the South hair flip.
Start Valen's Beach and Eiland's Winter idle/walk strips. All fifteen proposed
strips are present in the current archive.
