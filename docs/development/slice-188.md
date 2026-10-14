# Elsie Summer singing

Following `1190e57`, this slice adds Elsie's Summer singing start, loop
and end, completing her Summer world folder. The three South-facing strips
contain twenty-five frames. User art approval is pending. Local outputs use
`generated/review/` and `generated/build/`.

Elsie has 103 sources: 36 portraits and sixty-seven world strips. The
36-character collection has 4,854 sources and 19,416 variants. Z retains
Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with portrait/world
synchronization. This slice adds no runtime code or color roles. All 100 prior
regions, groups, target mappings and collection sets remain unchanged.
Historical pixel tests retain their expectations; only the central Elsie
coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 100 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and separate component groups are reused.
Across twenty-five frames, 890 component seeds select 1,734 pixels. Forehead,
face, neckline, both moving hands and exposed ankles follow the palette.
Gray hair, eyebrows, lashes, eye details, mouth interiors, turquoise earrings,
gold scarf, blue shawl, dress, shoes and the blue, pink and green music notes
retain their original colors. The four world shades occur only in skin in
these three pinned sources.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the face, mouth, hands, neckline, ankles, clothing and
floating music notes. The material test checks every pixel independently of
component seeds, plus 639 distinct literal material landmarks, per-frame counts,
transparency, metadata, unchanged profile roles and prior output bytes against
slice 187.

| Strip | Frames | Selected pixels per frame | Native durations |
| --- | --- | --- | --- |
| Sing start South | 1 | 72 | Omitted in source metadata |
| Sing loop South | 23 | 64, 64, 64, 64, 64, 68, 64, 64, 64, 66, 64, 65, 68, 85, 85, 87, 70, 70, 70, 70, 70, 70, 70 | 0.1 per frame |
| Sing end South | 1 | 72 | Omitted in source metadata |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. The
cycle remains complex, South-only and standing, with `on_pause_speaking`
set to `idle`. The offline animator harness supplies a one-second default for
the omitted start/end duration; live engine timing and speech-pause dispatch
remain unverified.

## Offline review

- [Five-choice sample summary](../../../generated/slice-188-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-188-review/index.html).

The summary samples start frame 1 and loop frames 2 and 14. Columns are
Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review includes all
twenty-five frames, Vanilla left and Debug Blue right, with full views and
enlarged details. Full crops include every floating music note. The enlarged
face/hand crops can cut off distant notes; the adjacent full views retain them.
Images, exports and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 400 regular tests and the
release build passed through the Nix shell; 470 opt-in tests were ignored.
All sixteen Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted the start face, neckline, viewer-left hand or viewer-right hand. Each
generated successfully and failed the intended pixel assertion. All three
metadata sidecars matched independent archive reads.

All 412 standalone Elsie variants passed strict recipe validation; 824 final
PNG/metadata files match the author candidate. All 19,416 combined variants
validated. The 1,000 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 187 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty-five cases include fifty full-view bindings, enlarged details and fifteen
summary bindings; 5,274,600 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is 495,750 bytes.

The isolated MOMI installation passed in `tmp/slice-188-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`35616928957aa4f615e146a489d45013ed5ab643ca700ac96730eea83167029a`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed 125 frame/palette
observations and twenty complex transitions under Fabricator. For each palette,
start advances to loop, loop repeats, then advances to end, and end completes
back at start. Palette binding, frame phase, standing status, wrapper
idempotence, cycle counters and Summer portrait synchronization were preserved.
Engine services are simulated and cycles selected directly. Natural gameplay
dispatch, speech interruption and live rendering were not exercised. The desktop
launcher and installed game were not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-188/`. Scratch outputs use `tmp/slice-188-` and
`tmp/elsie-summer-singing-author-`. Fresh exports are in
`extracted/elsie-summer-singing-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-187-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring and 32 Summer world strips are covered. Elsie's remaining world
inventory has 32 Autumn and 29 Winter strips. Next: Autumn standing and walking,
six strips. User art approval and live-game verification remain pending.
