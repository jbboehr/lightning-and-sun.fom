# Dell Spring general actions and shocked reaction

Following `af6ee03`, this slice adds general actions in East/North/South
orientations and the shocked South start/loop/end sequence. Twenty-four source
frames and seven native West mirrors give thirty-one review cases. The user
approved the artwork on 2026-10-10. Local outputs use `generated/review/` and
`generated/build/`.

Dell has 55 sources: 32 portraits and twenty-three world strips. The
36-character collection has 4,711 sources and 18,844 variants. Y retains Vanilla,
Debug Blue, Hayden, Ryis and Seridia choices with portrait/world synchronization.
This slice adds no runtime code or color roles. All 49 previous regions, groups,
target mappings and collection sets remain unchanged. Historical pixel tests
retain their expectations; only the central Dell coverage total grows.

## Source and materials

The game mount was empty. Source is the preserved original archive at
`tmp/slice-155-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 49 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across 24
frames, 529 component seeds select 812 pixels. Faces, closed eyelids, necks,
moving hands and exposed ankles follow the palette. Matching blonde hair,
eyebrow edges, lashes, eye details, the two mouth-interior shades, ribbon,
shirt, overalls, brown trim and red boots stay original. This includes the
extra boot shade `#5E0721` in the East/North action strips. North poses expose
hands with the rear neck covered by hair. The shocked jump raises both hands
alongside the face and spreads the hair; its open mouth keeps its red interior.
The compressed start/end poses close the eyes. No additional connected hair
components are selected in this batch.

All source frames were inspected in five-choice sheets. Original grids and
literal source samples establish face, hand and hair boundaries. The material
test checks every pixel independently of profile seeds, accounting for head
translation, closed eyelids and the raised hands. It fixes 284 literal landmarks
and per-frame counts, plus transparency, metadata, unchanged profile roles and
prior output bytes against slice 155. Separate color groups keep neighboring
hair shades from joining skin selections.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Action East | 7 | 32, 39, 41, 39, 41, 32, 42 |
| Action North | 7 | 11, 11, 14, 11, 14, 11, 14 |
| Action South | 7 | 40, 42, 46, 42, 46, 40, 46 |
| Shocked start South | 1 | 52 |
| Shocked loop South | 1 | 54 |
| Shocked end South | 1 | 52 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Actions use
seven frames with durations [0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4], the native
[240, 360] final-frame hold, and East mirroring for West. Shocked uses the native
complex start/loop/end sequence; each strip is a single frame with no explicit
duration. Neither cycle is seated. Native cycle configuration is unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-156-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-156-review/index.html).

The summary samples action East frame 3, action North frame 3 and shocked loop
South frame 1. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The
full review includes every source frame and native West mirror, Vanilla left
and Debug Blue right, with full views and enlarged details. Images, exports
and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 368 regular tests and the release
build passed through the Nix shell. The regular run left 438 opt-in tests
ignored. The selected local suite passed 228 tests, including all previous
Dell material tests, the new reaction test, historical character checks and
three GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted face pixels, a raised hand or an exposed ankle, or selected blonde hair.
Each generated successfully and failed the intended pixel assertion. All six
metadata sidecars matched independent archive reads.

All 220 standalone Dell variants passed strict recipe validation; 440 final
PNG/metadata files match the author candidate. All 18,844 combined variants
validated. The 490 prior Dell PNG/metadata files and 46,735 other-character
files match slice 155 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All thirteen Chromium gallery pages loaded their local images and links
without overflow. Thirty-one cases include seven West mirrors, sixty-two
full-view bindings, enlarged details and fifteen summary bindings; 5,729,568
rendered pixels match their source crops. Full views include every
nontransparent pixel. The complete review is 554,403 bytes; its short alias
loads successfully.

The isolated MOMI installation passed in `tmp/slice-156-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`180bcd43fa24c514d8319f19634eb5071ac7e2719f6436419d3c0ad7fc4dbdf3`.
The archive's native animator and NPC animate function passed 155 frame/palette
observations, twenty linear completions, forty final-frame hold checks and
twenty complex transitions under Fabricator. Native mirroring, palette binding,
frame phase, seated status, wrapper idempotence, cycle counters and Spring
portrait synchronization were preserved. Engine services are simulated and
cycles selected directly. This does not exercise natural dispatch, speaking
transitions or live gameplay. The desktop launcher and installed game were
not changed.

Evidence uses `tmp/slice-156-`; author records use
`tmp/dell-spring-reactions-author-`. Fresh exports are in
`extracted/dell-spring-reactions-study`, reached through
`extracted/test-corpus/dell`. The fixed baseline is
`generated/slice-155-build/characters/dell`. The evidence report summarizes
results; the scope check verifies nine expected paths, an empty Git index,
unchanged historical pixel tests and ignored game-derived artifacts.

## Remaining work

The archive inventory leaves ten Spring, 24 Summer, 21 Autumn and 21 Winter
world strips. Next: Spring scratching her head and yawning (six strips), then
stretching and swinging a stick (four strips). Natural gameplay dispatch,
speaking transitions and live rendering remain untested.
