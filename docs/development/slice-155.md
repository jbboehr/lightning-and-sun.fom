# Dell Spring eating and drinking

Following `75dca71`, this slice adds eating and drinking in East/North/South
orientations. Twenty-two source frames and eight native West mirrors give
thirty review cases. The user approved the artwork on 2026-10-10. Local outputs use
`generated/review/` and `generated/build/`.

Dell has 49 sources: 32 portraits and seventeen world strips. The 36-character
collection has 4,705 sources and 18,820 variants. Y retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This
slice adds no runtime code or color roles. All 43 previous regions, groups,
target mappings and collection sets remain unchanged. Historical pixel tests
retain their expectations; only the central Dell coverage total grows.

## Source and materials

The game mount was empty. Source is the preserved original archive at
`tmp/slice-154-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 43 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across 22
frames, 458 component seeds select 843 pixels. Moving faces, eyelids, neck,
hands and exposed ankles follow the palette. As in the approved blink, the
closed-eye East poses connect cheek shading to dark temple/fringe pixels:
five in drinking frame 2 and three each in eating frames 2–4. Those connected
edges follow the skin palette; adjacent blonde locks and all other hair stay
original. Lashes, eye details, the two mouth-interior shades, ribbon, shirt,
overalls, brown trim and boots retain their colors. The North poses expose
hands with the rear neck covered by hair. These PNGs contain the character
poses; separately rendered food or drink objects are outside this batch.

All source frames were inspected in five-choice sheets and source grids. The
material test checks every pixel independently of profile seeds, accounting
for head translation, eyelids, raised hands, connected temple edges and the
expanded mouth in the eating pose. It fixes 236 literal landmarks and per-frame
counts, plus transparency, metadata, unchanged profile roles and prior output
bytes against slice 154. Separate color groups keep neighboring hair shades
from joining skin selections.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Drink East | 3 | 41, 52, 41 |
| Drink North | 3 | 15, 11, 15 |
| Drink South | 3 | 50, 62, 50 |
| Eat East | 5 | 37, 49, 37, 43, 38 |
| Eat North | 3 | 15, 11, 15 |
| Eat South | 5 | 51, 54, 44, 62, 50 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Drinking
and eating North use three one-second frames. Eating East/South use five
frames with durations [0.125, 0.15, 0.175, 0.125, 0.6]. Both native cycles are
linear and seated, retain the [240, 360] final-frame hold, and mirror East
for West. Their original cycle configuration remains unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-155-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-155-review/index.html).

The summary samples drinking East frame 2, eating South frame 3 and eating
North frame 2. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia.
The full review includes every source frame and native West mirror, Vanilla
left and Debug Blue right, with full views and enlarged details. The open
mouth keeps its original red interior. Images, exports and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 367 regular tests and the release
build passed through the Nix shell. The regular run left 437 opt-in tests
ignored. The selected local suite passed 227 tests, including all previous
Dell material tests, the new meal test, historical character checks and three
GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted face pixels, a raised hand or an exposed ankle, or selected blonde hair.
Each generated successfully and failed the intended pixel assertion. All six
metadata sidecars matched independent archive reads.

All 196 standalone Dell variants passed strict recipe validation; 392 final
PNG/metadata files match the author candidate. All 18,820 combined variants
validated. The 430 prior Dell PNG/metadata files and 46,735 other-character
files match slice 154 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All thirteen Chromium gallery pages loaded their local images and links
without overflow. Thirty cases include eight West mirrors, sixty full-view
bindings, enlarged details and fifteen summary bindings; 5,559,840 rendered
pixels match their source crops. Full views include every nontransparent
pixel. The complete review is 527,634 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-155-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`4b8f1047a219ed1177fece3292b2658db8508a140b67a2082ad2cec5758bca73`.
The archive's native animator and NPC animate function passed 150 frame/palette
observations, forty linear completions and eighty final-frame hold checks under
Fabricator. Native mirroring, palette binding, frame phase, seated status,
wrapper idempotence, cycle counters and Spring portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. This
does not exercise natural dispatch, speaking transitions, separately rendered
food/drink objects or live gameplay. The desktop launcher and installed game
were not changed.

Evidence uses `tmp/slice-155-`; author records use
`tmp/dell-spring-meals-author-`. Fresh exports are in
`extracted/dell-spring-meals-study`, reached through
`extracted/test-corpus/dell`. The fixed baseline is
`generated/slice-154-build/characters/dell`. The evidence report summarizes
results; the scope check verifies nine expected paths, an empty Git index,
unchanged historical pixel tests and ignored game-derived artifacts.

## Remaining work

The archive inventory leaves sixteen Spring, 24 Summer, 21 Autumn and 21 Winter
world strips. Next: Spring general actions and the shocked reaction, followed
by scratching her head, yawning, stretching and swinging a stick. Natural
gameplay dispatch, speaking transitions, food/drink prop composition and live
rendering remain untested.
