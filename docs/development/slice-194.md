# Elsie Autumn seated writing

Following `e176050`, this slice adds Elsie's Autumn seated-writing start,
loop and end in East and South, plus native West mirroring. Sixteen source
frames give twenty-four review cases. User art approval is pending.
Local outputs use `generated/review/` and `generated/build/`.

Elsie has 132 sources: 36 portraits and ninety-six world strips. The
36-character collection has 4,883 sources and 19,532 variants. Z retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 126 prior
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Elsie
coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 126 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across sixteen frames, 383 component seeds select 766 pixels. Forehead, face,
closed eyelids, visible neckline and the moving writing hand follow the palette.
Gray hair, eyebrows, black lashes, eye details, gold earrings, blue collar,
burgundy gown, pink trim, shoes, book and quill stay original. The four world
shades occur only in skin in these six pinned sources. Pale book pages, tan
edges, pink feathers and the quill shaft remain distinct from the writing hand.
Pale pink shoe highlights stay original. Clothing shade `#4C0F1E` is preserved
despite its separate portrait palette role.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the eyelids, neckline, hand, quill, book, clothing and shoes.
The material test checks every pixel independently of component seeds, plus
511 distinct literal material landmarks, per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 193.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Write sit start East | 2 | 60, 41 |
| Write sit loop East | 4 | 35, 40, 39, 42 |
| Write sit end East | 2 | 41, 60 |
| Write sit start South | 2 | 62, 47 |
| Write sit loop South | 4 | 42, 47, 51, 50 |
| Write sit end South | 2 | 47, 62 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54.
Start duration remains scalar `0.125`, loop durations remain
`[0.1, 0.125, 0.1, 0.3]`, and end durations remain `[0.125, 0.1]`.
The cycle remains complex and seated, with South as default and native West
mirroring. There are no configured pause or final-frame-hold settings.

## Offline review

- [Five-choice sample summary](../../../generated/slice-194-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-194-review/index.html).

The summary samples East start frame 1, East loop frame 2 and South loop frame 3.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 406 regular tests and the
release build passed through the Nix shell; 476 opt-in tests were ignored.
All twenty-two Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the East loop face, South start neckline, East start hand or South loop
hand. Each generated successfully and failed the intended pixel assertion.
All six metadata sidecars matched independent archive reads.

All 528 standalone Elsie variants passed strict recipe validation; 1,056 final
PNG/metadata files match the author candidate. All 19,532 combined variants
validated. The 1,260 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 193 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All eleven Chromium gallery pages loaded local images and links without overflow.
Twenty-four cases include forty-eight full-view bindings, enlarged details and
fifteen summary bindings; 4,430,880 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
491,624 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-194-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`72313f8e7a012dfe3813237fd52aec5023bc47aadafa2b730c9cb7e37cd37c5d`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed 120 frame/palette
observations and sixty complex transitions under Fabricator. For every palette
and facing, start advances to loop, loop repeats, then advances to end, and end
completes back at start. Palette binding, frame phase, seated status, West
mirroring, wrapper idempotence, cycle counters and Autumn portrait synchronization
were preserved. Engine services are simulated and cycles selected directly.
Natural gameplay dispatch and live rendering were not exercised. The desktop
launcher and installed game were not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-194/`. Scratch outputs use `tmp/slice-194-` and
`tmp/elsie-autumn-seated-writing-author-`. Fresh exports are in
`extracted/elsie-autumn-seated-writing-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-193-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring and 32 Summer world strips are covered. Elsie's remaining world
inventory has 3 Autumn and 29 Winter strips. Next: Autumn singing, three strips,
completing her Autumn world folder. User art approval and live-game verification
remain pending.
