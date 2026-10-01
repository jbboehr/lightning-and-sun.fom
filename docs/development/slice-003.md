# Beach movement, Winter actions and Autumn studies

Following the artwork accepted in `da9cb60` and the stable test setup in
`658c668`, this slice adds 20 strips: 61 source frames and 23 native West mirrors,
giving 84 review cases. The user approved the artwork on 2026-09-30. The completed review and
build are available through `generated/review/` and `generated/build/`.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 6 | 15 + 5 | 235 | Beach 6/18 |
| Valen | 5 | 26 + 12 | 219 | Winter 22/31 |
| Eiland | 9 | 20 + 6 | 203 | Autumn 37/41 |

The 36-character collection has 3,842 sources and 15,368 variants. I, O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change. Existing pixel tests
remain byte-identical; the three growing totals change only in
`tests/profile_coverage.rs`. Local corpus links advance to the fresh exports.

## Source and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 637 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All 20 sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/54 origins.

Balor's Beach idle and walking use his existing four world skin shades for
faces, exposed chests, arms, hands, legs and feet. The open shirt, shorts, eyes
and hair remain original. All three source directions are covered, with native
West mirrors of East frames. Each idle strip has one frame; each walk has four.

Valen's Winter general actions, sleeping and kissing use her existing four
world shades for faces and visible hands. The coat, cuffs, goggles, trousers,
boots, eyes and hair remain original. North action exposes only one hand pixel
in frame one; frames two through seven remain pixel-identical. Tests fix this
inventory at `[1, 0, 0, 0, 0, 0, 0]` and check every pixel in all seven frames.
The other action directions have seven frames, kissing has four and sleeping one.

Eiland's Autumn seated writing and magnifying-glass cycles use his existing
five world shades. Twenty-eight clothing pixels share skin shadow `#BA6A4C`:
22 in magnifying-glass strips and six in seated-writing start/end strips.
Their disconnected components stay unselected; tests enumerate their literal
coordinates and check their original colors and unchanged outputs. Face and
moving hand shadows remain selected. The magnifying glass, writing tools, cape,
gold trim, pink clothing, boots, eyes and hair stay original, including outfit
shade `#C9785A`. Seated writing has 2/4/2 start/loop/end frames. Both East and
South magnifying-glass cycles have 3/1/2 frames, with native West mirrors of East.

## Offline review

- [Five-choice sample summary](../../generated/slice-003-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-003-review/index.html).

The combined index links all three `slice-003-<character>-review/` galleries.
Every new source frame and native West mirror appears, including Valen's six
unchanged North action frames. The four review directories total 1,675,972 bytes
(about 1.6 MiB). Images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 215 regular tests and the release
build passed; the normal suite leaves 275 local opt-ins ignored. Fifty-seven
local tests passed separately: three new material tests, fifty-one retained
corpus tests and three GML runtime tests. Six omission/spill controls generated
before failing their intended pixel assertions. Eiland's spill control restores
a shared-color clothing seed. The synthetic package test rejected the
unregistered Beach idle strip before the additions and passed afterward.

All 15,368 combined variants validate. All 2,628 standalone variants pass strict
recipe validation and match the combined build. All 6,370 prior original/variant
PNG and metadata files for these characters and all 32,015 files for the other
33 remain byte-identical. Previous runtime rows, choices and hotkeys remain exact.

The current archive's native animator and NPC object pass 420 frame/palette
observations, 80 linear completions, 40 last-frame hold checks and 80 complex
phase transitions using simulated engine services and direct cycle selection.
Checks cover mirroring, palette binding, wrapper idempotence, frame phase, cycle
counters, completion state and portrait synchronization. Natural scheduling,
interaction/outfit dispatch, attached effects and full-engine rendering remain
untested.

All 39 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks cover every case, native mirrors, metadata and complete opaque
crops, plus all fifteen source/palette bindings in the combined summary.
The `review` shortcut also passes Chromium loading; its 36 linked HTML pages and
71 image references resolve through the short path.

MOMI installation passed in the isolated `tmp/slice-003-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and lab's original backup retain the source hash above; eleven
registry/recipe inputs remain unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall
roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-003-`. Author records use `tmp/balor-beach-world-author-`,
`tmp/valen-winter-standard-author-` and `tmp/eiland-autumn-magnify-author-`.
Fresh exports live in their corresponding `extracted/<character>-<batch>-study`
directories, reached by the stable `extracted/test-corpus/<character>` links.
Eiland's `author-clothing.json` records the 28 protected clothing pixels, also
retained in the tracked material test. `evidence.json` summarizes verification;
`scope.log` checks the expected 13 paths and unchanged historical tests.

## Next coverage

Continue Balor's Beach blinking, sitting, general actions and kissing. Finish
Valen's Winter reading/writing/healing cycles and Eiland's four Autumn tool
animations. These remaining strips are present in the current archive.
