# Dell Spring stretching and swinging a stick

Following `1bb7baf`, this slice adds stretching start/loop/end and swinging a
stick. Eleven source frames and seven native West mirrors give eighteen review
cases. The user approved the artwork on 2026-10-10. Local outputs use
`generated/review/` and `generated/build/`.

Dell has 65 sources: 32 portraits and all 33 Spring world strips. The
36-character collection has 4,721 sources and 18,884 variants. Y retains Vanilla,
Debug Blue, Hayden, Ryis and Seridia choices with portrait/world synchronization.
This slice adds no runtime code or color roles. All 61 previous regions,
groups, target mappings and collection sets remain unchanged. Historical pixel
tests retain their expectations; only the central Dell coverage total grows.

## Source and materials

The game mount was empty. Source is the preserved original archive at
`tmp/slice-157-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 61 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across eleven
frames, 319 component seeds select 507 pixels. Faces, eyelids, necks, exposed
hands and ankles follow the palette. Matching blonde hair, lashes, eye details,
ribbon, shirt, overalls, brown trim, boots, stick and motion trail stay original.
The stick and trail use `#A8571C` and `#712922`, outside the skin mapping. During
the swing, the head moves four pixels right and the hand extends to the left;
its upper row belongs to skin even though earlier poses have hair at that height.
The arc temporarily covers the ankles, leaving no visible ankle skin in frame 4.

All source frames were inspected in original grids and five-choice sheets.
The material test checks every pixel independently of profile seeds. It fixes
153 literal landmarks and per-frame counts, plus transparency, metadata,
unchanged profile roles and prior output bytes against slice 157. It explicitly
preserves the shared-color locks bordering the cheek, stick and trail while
checking the closed eyelids and extended hand.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Stretch start South | 1 | 56 |
| Stretch loop South | 2 | 46, 44 |
| Stretch end South | 1 | 56 |
| Swing stick East | 7 | 42, 37, 43, 36, 47, 50, 50 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. The stretch
loop uses two 0.35-second frames; start and end have no explicit duration. The
swing uses durations `[0.1, 0.1, 0.5, 0.075, 0.1, 0.075, 0.5]`. Stretching uses
its native complex start/loop/end sequence facing South. Swinging uses its native
linear East sequence and West mirror. Neither is seated or has a final-frame
hold. Original pause behavior and cycle configuration remain unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-158-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-158-review/index.html).

The summary samples stretch loop frame 2 and swing frames 4 and 6. Columns
are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review includes every
source frame and native West mirror, Vanilla left and Debug Blue right, with
full views and enlarged details. Full views use a wider crop to include the
entire stick and motion trail on both sides. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 370 regular tests and the release
build passed through the Nix shell; 440 opt-in tests were ignored. The selected
local suite passed 230 tests, including every previous Dell material test,
the new stretching/stick test, historical character checks and three GML runtime
tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the four registry additions. Four valid mask mutations
omitted face pixels, the extended hand or an exposed ankle, or selected hair.
Each generated successfully and failed the intended pixel assertion. All four
metadata sidecars matched independent archive reads.

All 260 standalone Dell variants passed strict recipe validation; 520 final
PNG/metadata files match the author candidate. All 18,884 combined variants
validated. The 610 prior Dell PNG/metadata files and 46,735 other-character
files match slice 157 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All nine Chromium gallery pages loaded their local images and links without
overflow. Eighteen cases include thirty-six full-view bindings, enlarged details
and fifteen summary bindings; 4,044,456 rendered pixels match their source crops.
Full views include every nontransparent pixel, including both mirrored extremes
of the stick. The complete review is 376,663 bytes; its short alias loads
successfully.

The isolated MOMI installation passed in `tmp/slice-158-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`3dfc75b7be6f0842207ae5377cab9f51e496cf82c2b300dddd9b16ce7f512e53`.
The archive's native animator and NPC animate function passed 90 frame/palette
observations, ten linear completions and twenty complex transitions under
Fabricator. Palette binding, frame phase, West mirroring, seated status, wrapper
idempotence, cycle counters and Spring portrait synchronization were preserved.
Engine services are simulated and cycles selected directly. Natural gameplay
dispatch, speaking/background pause transitions and live rendering were not
exercised. The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-158-`; author records use
`tmp/dell-spring-finish-author-`.

Fresh exports are in `extracted/dell-spring-finish-study`, reached through
`extracted/test-corpus/dell`. The fixed baseline is
`generated/slice-157-build/characters/dell`. The evidence report summarizes
results; the scope check verifies nine expected paths, an empty Git index,
unchanged historical pixel tests and ignored game-derived artifacts.

## Remaining work

The archive inventory leaves 24 Summer, 21 Autumn and 21 Winter world strips;
Spring is complete. Next: Dell Summer standing and walking (six strips).
Natural gameplay dispatch, speaking/background pause transitions and live
rendering remain untested.
