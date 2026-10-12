# Dell Winter blinking and sitting

Following `a996ac1`, this slice adds Dell's Winter blinking (South and East)
and sitting (South, North and East), with native West mirroring. Nine source
frames give thirteen review cases. The user approved the artwork on 2026-10-12.
Local outputs use `generated/review/` and `generated/build/`.

Dell has 121 sources: 32 portraits and 89 world strips. All 33 Spring, 24 Summer
and 21 Autumn strips remain covered, alongside eleven of 21 Winter strips. The
36-character collection has 4,777 sources and 19,108 variants. Y retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 116 previous
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Dell
coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-169-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 116 prior
Dell PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate groups are reused. Across nine
frames, 158 component seeds select 293 pixels. Faces, eyelids and exposed hands
follow the palette. As in earlier outfits, three dark temple pixels connected
to the closed East eye follow its edge: local coordinates `[44,36]`, `[43,37]`
and `[44,37]` in frame 2. Other blonde hair, lashes, eye details, blue coat,
orange shirt and cuffs, trousers and brown boots stay original. Sitting shifts
the face up three pixels and exposes larger hands. The East seated pose exposes
one hand; South and North expose both. No legs or ankles are exposed.

All source frames were inspected in five-choice sheets, with original pixel
grids used to distinguish matching skin and hair colors. The material test
checks every pixel independently of profile seeds, using reviewed face and
hand bounds and explicit connected-eye exceptions. It fixes 179 distinct
material landmarks and per-frame counts, plus transparency, metadata,
unchanged profile roles and prior output bytes against slice 169. Clothing
uses `#A8571C`, also present in Dell's portrait roles; these world masks preserve
it by selecting only reviewed skin groups.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Blink East | 3 | 29, 42, 29 |
| Blink South | 3 | 34, 44, 34 |
| Sit East | 1 | 31 |
| Sit North | 1 | 10 |
| Sit South | 1 | 40 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Blinking
retains durations `[0.075, 0.125, 0.075]`; sitting single frames have no explicit
duration. Both cycles are linear with South as default and native West
mirroring of East. Sitting retains `is_seated = true`. Neither cycle has a
final-frame hold or explicit pause transitions in its source configuration.

## Offline review

- [Five-choice sample summary](../../../generated/slice-170-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-170-review/index.html).

The summary samples Blink East frame 2, Blink South frame 2 and Sit South frame 1.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 382 regular tests and the release
build passed through the Nix shell; 452 opt-in tests were ignored. The selected
local suite passed 242 tests, including every previous Dell material test,
the new Winter blinking/sitting test, historical character checks and three
GML runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the five registry additions. Four valid mask mutations
omitted the closed-eye face pixels, the seated left hand or the seated right
hand, or selected blonde hair. Each generated successfully and failed the
intended pixel assertion. All five metadata sidecars matched independent
archive reads.

All 484 standalone Dell variants passed strict recipe validation; 968 final
PNG/metadata files match the author candidate. All 19,108 combined variants
validated. The 1,160 prior Dell PNG/metadata files and 46,735 other-character
files match slice 169 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All nine Chromium gallery pages loaded their local images and links without
overflow. Thirteen cases include twenty-six full-view bindings, enlarged
details and fifteen summary bindings; 3,083,496 rendered pixels match their
source crops. Full views include every nontransparent pixel. The complete
review is 291,733 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-170-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`6ffe92ef336400c5945180c30c25106dd36600a36fda9cc0e9899561ee9a2a79`.
The archive's native animator and NPC animate function passed 65 frame/palette
observations and 35 linear completions under Fabricator. Palette binding,
frame phase, West mirroring, seated status, wrapper idempotence, cycle counters
and Winter portrait synchronization were preserved. Engine services are
simulated and cycles selected directly. Natural gameplay dispatch and live
rendering were not exercised. The desktop launcher and installed game were
not changed.

The scope check verifies nine expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-170-`; author records use
`tmp/dell-winter-actions-author-`. Fresh exports are in
`extracted/dell-winter-actions-study`, reached through `extracted/test-corpus/dell`.
The fixed baseline is `generated/slice-169-build/characters/dell`.

## Remaining work

Spring, Summer and Autumn are complete for this archive. The remaining Dell
world inventory is ten Winter strips. Next: Winter eating and drinking,
six strips.
