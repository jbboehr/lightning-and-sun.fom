# Elsie Summer seated writing

Following `5a55720`, this slice adds Elsie's Summer seated-writing start,
loop and end in East and South, plus native West mirroring. Sixteen source
frames give twenty-four review cases. User art approval is pending.
Local outputs use `generated/review/` and `generated/build/`.

Elsie has 100 sources: 36 portraits and sixty-four world strips. The
36-character collection has 4,851 sources and 19,404 variants. Z retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 94 prior
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Elsie
coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 94 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across sixteen frames, 383 component seeds select 757 pixels. Forehead, face,
closed eyelids, visible neckline and the moving writing hand follow the palette.
Gray hair, eyebrows, black lashes, eye details, turquoise earrings, gold scarf,
blue shawl, dress, shoes, book and quill stay original. The four world shades
occur only in skin in these six pinned sources. Pale book pages, tan edges,
pink feathers and the quill shaft remain distinct from the writing hand.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the eyelids, neckline, hand, quill, book and clothing.
The material test checks every pixel independently of component seeds, plus
498 distinct literal material landmarks, per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 186.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Write sit start East | 2 | 58, 42 |
| Write sit loop East | 4 | 35, 40, 40, 42 |
| Write sit end East | 2 | 39, 58 |
| Write sit start South | 2 | 62, 47 |
| Write sit loop South | 4 | 42, 46, 49, 48 |
| Write sit end South | 2 | 47, 62 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54.
Start duration remains scalar `0.125`, loop durations remain
`[0.1, 0.125, 0.1, 0.3]`, and end durations remain `[0.125, 0.1]`.
The cycle remains complex and seated, with South as default and native West
mirroring. There are no configured pause or final-frame-hold settings.

## Offline review

- [Five-choice sample summary](../../../generated/slice-187-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-187-review/index.html).

The summary samples East start frame 1, East loop frame 2 and South loop frame 3.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 399 regular tests and the
release build passed through the Nix shell; 469 opt-in tests were ignored.
All fifteen Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the East loop face, South start neckline, East start hand or South loop
hand. Each generated successfully and failed the intended pixel assertion.
All six metadata sidecars matched independent archive reads.

All 400 standalone Elsie variants passed strict recipe validation; 800 final
PNG/metadata files match the author candidate. All 19,404 combined variants
validated. The 940 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 186 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All eleven Chromium gallery pages loaded local images and links without overflow.
Twenty-four cases include forty-eight full-view bindings, enlarged details and
fifteen summary bindings; 4,430,880 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is 495,506
bytes.

The isolated MOMI installation passed in `tmp/slice-187-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`7517e26952ea0efcde01ed3100c8193f2e6a3d0581d062414eec0a2a2a9b419b`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed 120 frame/palette
observations and sixty complex transitions under Fabricator. For every palette
and facing, start advances to loop, loop repeats, then advances to end, and end
completes back at start. Palette binding, frame phase, seated status, West
mirroring, wrapper idempotence, cycle counters and Summer portrait synchronization
were preserved. Engine services are simulated and cycles selected directly.
Natural gameplay dispatch and live rendering were not exercised. The desktop
launcher and installed game were not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-187/`. Scratch outputs use `tmp/slice-187-` and
`tmp/elsie-summer-seated-writing-author-`. Fresh exports are in
`extracted/elsie-summer-seated-writing-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-186-build/characters/elsie`. Required inputs remain outside
the disposable output directories, as described in [local storage](local-storage.md).

## Remaining work

Elsie's remaining world inventory has 3 Summer, 32 Autumn and 29 Winter strips.
Next: Summer singing, three strips, completing her Summer world folder.
User art approval and live-game verification remain pending for this slice.
