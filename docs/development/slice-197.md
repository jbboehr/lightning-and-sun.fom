# Elsie Winter blinking and sitting

Following `9a8b713`, this slice adds Elsie's Winter blinking in East and
South and sitting in East, North and South. Five strips contain nine source
frames and four native West mirrors, giving thirteen review cases. User art
approval is pending. Local outputs use `generated/review/` and `generated/build/`.

Elsie has 146 sources: 36 portraits and 110 world strips. The 36-character
collection has 4,897 sources and 19,588 variants. Z retains Vanilla, Debug
Blue, Hayden, Ryis and Seridia choices with portrait/world synchronization.
This slice adds no runtime code or color roles. All 141 prior regions, groups,
target mappings and collection sets remain unchanged. Historical pixel tests
retain their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 141 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and component groups are reused. Across the
nine frames, 175 component seeds select 348 pixels. Exposed facial skin and
closed eyelids change. Her Winter gloves and collar cover her hands and neck;
North-facing sitting has no exposed skin, so its empty seed list preserves
every pixel. Gray hair, lashes, eye details, gold hood, cream fur trim, gloves,
green dress, brooch and boots retain their original colors. None of the four
world skin shades occurs in other materials in these pinned sources.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the closed eyelids, face, hair, hood, collar, gloves,
dress and boots. The material test checks every pixel independently of component
seeds, plus 254 distinct literal material landmarks, per-frame counts,
transparency, metadata, unchanged profile roles and prior output bytes against
slice 196. North-facing sitting additionally requires complete image equality.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Blink East | 3 | 40, 50, 40 |
| Blink South | 3 | 44, 54, 44 |
| Sit East | 1 | 36 |
| Sit North | 1 | 0 |
| Sit South | 1 | 40 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Blinking
retains durations `[0.075, 0.125, 0.075]`; sitting has one frame and no duration
property. Both cycles remain linear with South as default and native West
mirroring. Sitting retains `is_seated = true`; blinking is unseated. Neither
cycle has a final-frame hold or configured speaking/background pause settings.
The offline animator harness supplies a one-second default for omitted sitting
durations; live engine timing remains unverified.

## Offline review

- [Five-choice sample summary](../../../generated/slice-197-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-197-review/index.html).

The summary samples Blink East frame 2, Blink South frame 2 and Sit South
frame 1. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full
review includes all thirteen cases, Vanilla left and Debug Blue right, with
full views and enlarged details. Full crops include every nontransparent
pixel. Images, exports and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 409 regular tests and the
release build passed through the Nix shell; 479 opt-in tests were ignored.
All twenty-five Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the five registry additions. Four valid mask mutations
omitted components around the closed East eyelids, closed South eyelids,
seated East cheek or seated South chin. Each generated successfully and failed
the intended pixel assertion. All five metadata sidecars matched independent
archive reads.

All 584 standalone Elsie variants passed strict recipe validation; 1,168 final
PNG/metadata files match the author candidate. All 19,588 combined variants
validated. The 1,410 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 196 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All nine Chromium gallery pages loaded local images and links without overflow.
Thirteen cases include twenty-six full-view bindings, enlarged details and
fifteen summary bindings; 2,614,560 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
301,447 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-197-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`65dcda51d11667abf03e179dd9e8ed4e10dfae7ae3ddfd0c006975444fc2bfbc`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed 65 frame/palette
observations and thirty-five linear completions under Fabricator. Palette
binding, frame phase, West mirroring, seated status, wrapper idempotence, cycle
counters and Winter portrait synchronization were preserved. Engine services
are simulated and cycles selected directly. Natural gameplay dispatch and live
rendering were not exercised. The desktop launcher and installed game were
not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-197/`. Scratch outputs use `tmp/slice-197-` and
`tmp/elsie-winter-actions-author-`. Fresh exports are in
`extracted/elsie-winter-actions-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-196-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring, 32 Summer and 32 Autumn world strips are covered, plus eleven
of 29 Winter strips. Elsie's remaining world inventory has 18 Winter strips.
Next: Winter eating and drinking, six strips. User art approval and live-game
verification remain pending.
