# Dell Summer blinking and sitting

Following `88f8c5e`, this slice adds Dell's Summer blinking in South and East,
and sitting in North, South and East, plus native West mirroring. Nine source
frames give thirteen review cases. The user approved the artwork on 2026-10-10. Local outputs
use `generated/review/` and `generated/build/`.

Dell has 76 sources: 32 portraits and 44 world strips. The 36-character collection
has 4,732 sources and 18,928 variants. Y retains Vanilla, Debug Blue, Hayden, Ryis
and Seridia choices with portrait/world synchronization. This slice adds no
runtime code or color roles. All 71 previous regions, groups, target mappings
and collection sets remain unchanged. Historical pixel tests retain their
expectations; only the central Dell coverage total grows.

## Source and materials

The game mount was empty. Source is the preserved original archive at
`tmp/slice-159-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 71 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across nine
frames, 259 component seeds select 425 pixels. Faces, eyelids, exposed arms,
hands and legs follow the palette. In the closed East blink, two `#DF8D4B`
pixels at `(42,37)` and `(43,37)` connect to the eyelid component and follow
its edge. This differs from Spring's connected dark temple outline; the
Summer mask retains the adjacent hair at `(44,36)` and `(44,37)`.
Other blonde hair, lashes, ribbon, eye details, blue shirt, brown overalls,
pale rolled cuffs and shoes stay original.

All nine source frames were inspected in original grids and five-choice
sheets. The material test checks every pixel independently of profile seeds.
It fixes 141 literal landmarks and per-frame counts, plus transparency,
metadata, unchanged profile roles and prior output bytes against slice 159.
The seated face is three pixels higher. In the North seated view, only hands
and exposed arms change; hair and clothing cover the remaining body.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Blink East | 3 | 47, 59, 47 |
| Blink South | 3 | 52, 62, 52 |
| Sit East | 1 | 36 |
| Sit North | 1 | 16 |
| Sit South | 1 | 54 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Blinking
uses durations `[0.075, 0.125, 0.075]`; the seated single frames have no explicit
duration. Both native cycles are linear with no final-frame hold. Sitting
retains its native seated status. Original cycle configuration remains unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-160-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-160-review/index.html).

The summary samples blink East frame 2, blink South frame 2 and sit East frame 1.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 372 regular tests and the release
build passed through the Nix shell; 442 opt-in tests were ignored. The selected
local suite passed 232 tests, including every previous Dell material test,
the new blinking/seated test, historical character checks and three GML runtime
tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the five registry additions. Four valid mask mutations
omitted closed-eye pixels, a seated hand or exposed leg pixels, or selected hair.
Each generated successfully and failed the intended pixel assertion. All five
metadata sidecars matched independent archive reads.

All 304 standalone Dell variants passed strict recipe validation; 608 final
PNG/metadata files match the author candidate. All 18,928 combined variants
validated. The 710 prior Dell PNG/metadata files and 46,735 other-character
files match slice 159 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All nine Chromium gallery pages loaded their local images and links without
overflow. Thirteen cases include twenty-six full-view bindings, enlarged details
and fifteen summary bindings; 3,083,496 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
282,952 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-160-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`2774846902b4cdafda9ce8e3a941790ddf93d29662a0406b3952324b8566e628`.
The archive's native animator and NPC animate function passed 65 frame/palette
observations and thirty-five linear completions under Fabricator. Palette
binding, frame phase, West mirroring, seated status, wrapper idempotence, cycle
counters and Summer portrait synchronization were preserved. Engine services
are simulated and cycles selected directly. Natural gameplay dispatch,
speaking/background pause transitions and live rendering were not exercised.
The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-160-`; author records use
`tmp/dell-summer-actions-author-`.

Fresh exports are in `extracted/dell-summer-actions-study`, reached through
`extracted/test-corpus/dell`. The fixed baseline is
`generated/slice-159-build/characters/dell`. The evidence report summarizes
results; the scope check verifies nine expected paths, an empty Git index,
unchanged historical pixel tests and ignored game-derived artifacts.

## Remaining work

The archive inventory leaves 13 Summer, 21 Autumn and 21 Winter world strips.
Spring is complete. Next: Dell Summer eating and drinking (six strips).
Natural gameplay dispatch, speaking/background pause transitions and live
rendering remain untested.
