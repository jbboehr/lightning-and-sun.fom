# Elsie Autumn general actions

Following `dcd3114`, this slice adds Elsie's Autumn general action in East,
North and South, plus native West mirroring. Three strips contain twenty-one
source frames and seven mirrored views, giving twenty-eight review cases.
User art approval is pending. Local outputs use `generated/review/` and
`generated/build/`.

Elsie has 123 sources: 36 portraits and eighty-seven world strips. The
36-character collection has 4,874 sources and 19,496 variants. Z retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 120 prior
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Elsie
coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 120 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across twenty-one frames, 494 component seeds select 875 pixels. Forehead,
face, neckline and moving hands follow the palette. Gray hair, eyebrows,
black lashes, eye details, gold earrings, blue collar, burgundy gown, pink trim
and shoes retain their original colors. The four world shades occur only in
skin in these three pinned sources. North views expose small neckline and hand
areas; the extended East hand stays distinct from the sleeve. Pale pink shoe
highlights remain unchanged. Clothing shade `#4C0F1E` is preserved despite its
separate portrait palette role.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the eyes, neckline, extended fingertips, sleeves and shoes.
The material test checks every pixel independently of component seeds, plus
393 distinct literal material landmarks, per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 191.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Action East | 7 | 54, 54, 50, 54, 50, 54, 55 |
| Action North | 7 | 13, 12, 14, 12, 14, 13, 13 |
| Action South | 7 | 56, 60, 61, 60, 61, 56, 59 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54.
Durations remain `[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`. The action cycle
remains linear with South as default, North/South/East directions and native
West mirroring. It has no seated flag, preserves `last_frame_hold = [240, 360]`
and pauses to idle when speaking. Background pause and speaking-turn settings
remain absent.

## Offline review

- [Five-choice sample summary](../../../generated/slice-192-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-192-review/index.html).

The summary samples Action East frame 2, Action South frame 3 and Action North
frame 7. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full
review includes every source frame and native West mirror, Vanilla left and
Debug Blue right, with full views and enlarged details. Images, exports and
packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 404 regular tests and the
release build passed through the Nix shell; 474 opt-in tests were ignored.
All twenty Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted the East action face, South neckline, South viewer-left hand or North
viewer-right hand. Each generated successfully and failed the intended pixel
assertion. All three metadata sidecars matched independent archive reads.

All 492 standalone Elsie variants passed strict recipe validation; 984 final
PNG/metadata files match the author candidate. All 19,496 combined variants
validated. The 1,200 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 191 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty-eight cases include fifty-six full-view bindings, enlarged details and
fifteen summary bindings; 5,091,360 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
531,633 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-192-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`ab12f2007f135da413c906cfaec63c474c269e4e0f3f3060ba5c8980bce0b902`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed 140 frame/palette
observations, twenty linear completions and forty final-frame-hold checks under
Fabricator. Palette binding, frame phase, standing status, West mirroring,
wrapper idempotence, cycle counters and Autumn portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch, speaking pauses and live rendering were not exercised. The
desktop launcher and installed game were not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-192/`. Scratch outputs use `tmp/slice-192-` and
`tmp/elsie-autumn-general-author-`. Fresh exports are in
`extracted/elsie-autumn-general-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-191-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring and 32 Summer world strips are covered. Elsie's remaining world
inventory has 12 Autumn and 29 Winter strips. Next: Autumn standing writing,
three strips. User art approval and live-game verification remain pending.
