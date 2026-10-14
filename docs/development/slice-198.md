# Elsie Winter eating and drinking

Following `2501452`, this slice adds Elsie's Winter eating and drinking in
East, North and South. Six strips contain twenty-two source frames and eight
native West mirrors, giving thirty review cases. User art approval is pending.
Local outputs use `generated/review/` and `generated/build/`.

Elsie has 152 sources: 36 portraits and 116 world strips. The 36-character
collection has 4,903 sources and 19,612 variants. Z retains Vanilla, Debug
Blue, Hayden, Ryis and Seridia choices with portrait/world synchronization.
This slice adds no runtime code or color roles. All 146 prior regions, groups,
target mappings and collection sets remain unchanged. Historical pixel tests
retain their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 146 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and component groups are reused. Across the
twenty-two frames, 347 component seeds select 667 pixels. Exposed facial skin
and closed eyelids change. Her moving hands stay gloved and the collar covers
her neck; North-facing frames have no exposed skin, so their empty seed lists
preserve every pixel. Mouth interiors, gray hair, lashes, eye details, gold
hood, cream fur trim, gloves, green dress, brooch and boots retain their
original colors. None of the four world skin shades occurs in other materials
in these pinned sources.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the open mouths, closed eyelids, face, hair, hood, collar,
moving gloves, dress and boots. The material test checks every pixel independently
of component seeds, plus 556 distinct literal material landmarks, per-frame
counts, transparency, metadata, unchanged profile roles and prior output bytes
against slice 197. North-facing frames additionally require complete image equality.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Eat East | 5 | 36, 43, 37, 45, 36 |
| Eat North | 3 | 0, 0, 0 |
| Eat South | 5 | 40, 46, 38, 54, 40 |
| Drink East | 3 | 36, 48, 36 |
| Drink North | 3 | 0, 0, 0 |
| Drink South | 3 | 40, 52, 40 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54.
Eating East/South retains durations `[0.125, 0.15, 0.175, 0.125, 0.6]`;
North eating and all drinking strips retain scalar duration `1.0`. Both cycles
remain linear with South as default, native West mirroring, `is_seated = true`
and `last_frame_hold = [240, 360]`. Neither cycle configures speaking or
background pause settings.

## Offline review

- [Five-choice sample summary](../../../generated/slice-198-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-198-review/index.html).

The summary samples Eat South frame 3, Drink East frame 2 and Eat North frame 2.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes all thirty cases, Vanilla left and Debug Blue right, with full views
and enlarged details. Full crops include every nontransparent pixel. Images,
exports and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 410 regular tests and the
release build passed through the Nix shell; 480 opt-in tests were ignored.
All twenty-six Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted components around the East or South open-mouth faces, or the East or
South closed eyelids while drinking. Each generated successfully and failed
the intended pixel assertion. All six metadata sidecars matched independent
archive reads.

All 608 standalone Elsie variants passed strict recipe validation; 1,216 final
PNG/metadata files match the author candidate. All 19,612 combined variants
validated. The 1,460 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 197 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All thirteen Chromium gallery pages loaded local images and links without
overflow. Thirty cases include sixty full-view bindings, enlarged details and
fifteen summary bindings; 5,421,600 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
560,537 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-198-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`26445148945db9bea3f4acc5ff4b3126ec9e7600c42aa3b46f07ca632846f801`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed 150 frame/palette
observations, forty linear completions and eighty final-frame-hold checks under
Fabricator. Palette binding, frame phase, West mirroring, seated status, wrapper
idempotence, cycle counters and Winter portrait synchronization were preserved.
Engine services are simulated and cycles selected directly. Natural gameplay
dispatch and live rendering were not exercised. The desktop launcher and
installed game were not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-198/`. Scratch outputs use `tmp/slice-198-` and
`tmp/elsie-winter-food-author-`. Fresh exports are in
`extracted/elsie-winter-food-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-197-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring, 32 Summer and 32 Autumn world strips are covered, plus seventeen
of 29 Winter strips. Elsie's remaining world inventory has twelve Winter strips.
Next: Winter general actions, three strips. User art approval and live-game
verification remain pending.
