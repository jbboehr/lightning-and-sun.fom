# Dell Spring blinking and sitting

Following `b5df04f`, this slice adds five strips: blinking East/South and sitting
East/North/South. Nine source frames and four native West mirrors give thirteen
review cases. The user approved the artwork on 2026-10-10. Local outputs use
`generated/review/` and `generated/build/`.

Dell has 43 sources: 32 portraits and eleven world strips. The 36-character
collection has 4,699 sources and 18,796 variants. Y retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This
slice adds no runtime code or color roles; previous regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests keep
their fixed expectations; only the central Dell coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-153-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies the preserved archive, not a newer installed game version. All
38 previous Dell PNG pins match fresh exports; no hash override or pin refresh
was needed.

The existing four world shades and separate color groups are reused. Across
nine frames, 238 component seeds select 392 pixels. Face, eyelids, neck, hands
and exposed ankles follow the palette. The closed East blink joins its dark
cheek outline to three temple/fringe pixels at (44,36), (43,37) and (44,37).
These are included with the connected edge; this is a deliberate art choice
for review, not a claim that every selected pixel belongs exclusively to skin.
The adjacent blonde locks and remaining hair are preserved. Lashes, eye
details, ribbon, pink shirt, blue overalls, brown trim and red boots stay
original. The North seated pose exposes hands, with its neck covered by hair.

All source frames were inspected in five-choice sheets and labeled grids.
The material test checks every pixel against reviewed face rows, seated
position offsets, the closed-eye edge and body colors independently of seeds.
It includes 151 literal landmarks, per-frame counts, transparency, exact
metadata, unchanged profile roles and previous output bytes against slice 153.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Blink East | 3 | 42, 55, 42 |
| Blink South | 3 | 46, 56, 46 |
| Sit East | 1 | 39 |
| Sit North | 1 | 16 |
| Sit South | 1 | 50 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Blinking
retains durations [0.075, 0.125, 0.075]; sitting is a single frame with no
explicit duration. Both cycles are linear, West mirrors East, and sitting
retains native seated status.

## Offline review

- [Five-choice sample summary](../../../generated/slice-154-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-154-review/index.html).

The summary samples closed-eye blinking East and sitting East/North, with
Vanilla, Debug Blue, Hayden, Ryis and Seridia from left to right. The full
review includes every source frame and West mirror, Vanilla left and Debug
Blue right, with full views and enlarged details. Game-derived images and
packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 366 regular tests and the release
build passed through the Nix shell. The regular run left 436 opt-in tests
ignored. The selected local suite passed 226 tests, including the unchanged
Dell portrait and standing/walking tests, the new material test, historical
character checks and three GML runtime tests.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with five registry additions. Four
valid mask mutations omitted closed eyelids, a seated hand or exposed ankles,
or selected a blonde hair component. Each generated successfully and failed
the intended pixel assertion. All five metadata sidecars matched independent
archive reads.

All 172 standalone Dell variants passed strict recipe validation; 344 final
PNG/metadata files match the author candidate. All 18,796 combined variants
validated. The 380 prior Dell PNG/metadata files and 46,735 other-character
files match slice 153 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All nine Chromium gallery pages loaded their local images and links without
overflow. Thirteen cases include four West mirrors, 26 full-view bindings,
enlarged details and fifteen summary bindings; 2,674,464 rendered pixels match
their source crops. Full views include every nontransparent pixel. The complete
review is 285,423 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-154-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`6540e306a94467e7bc3501e4f1a57cb058e2b796dfdcf9de01b4d5d077f2a27a`.
The archive's native animator and NPC animate function passed 65 frame/palette
observations and 35 linear completions under Fabricator. Checks preserve native
mirroring, palette binding, frame phase, seated status, wrapper idempotence,
cycle counters and Spring portrait synchronization. Engine services are
simulated and cycles selected directly. Natural gameplay dispatch, speaking
transitions and live rendering remain untested. The desktop launcher and
installed game were not changed.

Evidence uses `tmp/slice-154-`; author records use
`tmp/dell-spring-actions-author-`. Fresh exports are in
`extracted/dell-spring-actions-study`, reached through
`extracted/test-corpus/dell`. The fixed baseline is
`generated/slice-153-build/characters/dell`. The evidence report summarizes
results; the scope check verifies nine expected paths, an empty Git index,
unchanged historical pixel tests and ignored game-derived artifacts.

## Remaining work

The archive inventory leaves 22 Spring, 24 Summer, 21 Autumn and 21 Winter
world strips. Next: Spring eating and drinking, followed by general actions
and special animations. Natural gameplay dispatch, speaking transitions and
live rendering have not been exercised in this slice.
