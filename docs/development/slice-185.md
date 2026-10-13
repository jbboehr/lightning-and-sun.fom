# Elsie Summer general actions

Following `1959573`, this slice adds Elsie's Summer general action in East,
North and South, plus native West mirroring. Three strips contain twenty-one
source frames and seven mirrored views, giving twenty-eight review cases.
User art approval was received on 2026-10-13. Local outputs use `generated/review/` and
`generated/build/`.

Elsie has 91 sources: 36 portraits and fifty-five world strips. The 36-character
collection has 4,842 sources and 19,368 variants. Z retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. This slice
adds no runtime code or color roles. All 88 prior regions, groups, target
mappings and collection sets remain unchanged. Historical pixel tests retain
their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-184-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 88 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across twenty-one frames, 516 component seeds select 906 pixels. Forehead,
face, neckline, exposed arms, moving hands and ankles follow the palette.
Gray hair, eyebrows, black lashes, eye details, turquoise earrings, gold scarf,
blue shawl, dress and shoes retain their original colors. The four world shades
occur only in skin in these three pinned sources. North views expose the
neckline and arms/hands; the extended arm in East views stays distinct from
her blue shawl.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the eyes, neckline, moving hands, clothing and ankles.
The material test checks every pixel independently of component seeds, plus
429 distinct literal material landmarks, per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 184.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Action East | 7 | 50, 55, 54, 55, 54, 50, 53 |
| Action North | 7 | 22, 15, 16, 15, 16, 24, 27 |
| Action South | 7 | 53, 58, 60, 58, 60, 53, 58 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54.
Durations remain `[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`. The action cycle
remains linear with South as default, North/South/East directions and native
West mirroring. It has no seated flag, preserves `last_frame_hold = [240, 360]`
and pauses to idle when speaking. Background pause and speaking-turn settings
remain absent. The installed Elsie NPC configuration matches the source byte
for byte.

## Offline review

- [Five-choice sample summary](../../../generated/slice-185-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-185-review/index.html).

The summary samples Action East frame 2, Action South frame 3 and Action North
frame 7. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full
review includes every source frame and native West mirror, Vanilla left and
Debug Blue right, with full views and enlarged details. Images, exports and
packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 397 regular tests and the
release build passed through the Nix shell; 467 opt-in tests were ignored.
The selected local suite passed 258 tests, including all earlier Elsie
material checks, the new general-action material test, historical character
checks and three GML runtime tests. Historical pixel-test expectations are
unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted the East action face, South action neckline, South action viewer-left
hand or North action viewer-right hand. Each generated successfully and failed
the intended pixel assertion. All three metadata sidecars matched independent
archive reads.

All 364 standalone Elsie variants passed strict recipe validation; 728 final
PNG/metadata files match the author candidate. All 19,368 combined variants
validated. The 880 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 184 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
Five frozen configuration inputs and both original archive copies retained
their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty-eight cases include fifty-six full-view bindings, enlarged details and
fifteen summary bindings; 5,091,360 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
538,606 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-185-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`82f786d68da72db9c016524f5d2cc37dfe88df22e1f7b969e012e0de2d6b00ba`.
The archive's native animator and NPC animate function passed 140 frame/palette
observations, twenty linear completions and forty final-frame-hold checks under
Fabricator. Palette binding, frame phase, standing status, West mirroring,
wrapper idempotence, cycle counters and Summer portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch, speaking-pause transitions and live rendering were not
exercised. The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-185-`; author records use
`tmp/elsie-summer-general-author-`. Fresh exports are in
`extracted/elsie-summer-general-study`, reached through
`extracted/test-corpus/elsie`. The fixed baseline is
`generated/slice-184-build/characters/elsie`. Scope verification checks nine
expected paths, an empty Git index, unchanged historical pixel tests and ignored
game-derived artifacts.

## Remaining work

Elsie's remaining world inventory has 12 Summer, 32 Autumn and 29 Winter
strips. Next: Summer standing writing, three strips. Live-game verification
remains pending for this slice.
