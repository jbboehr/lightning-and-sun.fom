# Elsie Winter general actions

Following `7b7877d`, this slice adds Elsie's Winter general actions in East,
North and South. Three strips contain twenty-one source frames and seven native
West mirrors, giving twenty-eight review cases. User art approval is pending.
Local outputs use `generated/review/` and `generated/build/`.

Elsie has 155 sources: 36 portraits and 119 world strips. The 36-character
collection has 4,906 sources and 19,624 variants. Z retains Vanilla, Debug
Blue, Hayden, Ryis and Seridia choices with portrait/world synchronization.
This slice adds no runtime code or color roles. All 152 prior regions, groups,
target mappings and collection sets remain unchanged. Historical pixel tests
retain their expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 152 prior
Elsie PNG pins match fresh exports. No hash override or pin refresh was needed.

The existing four world shades and component groups are reused. Across the
twenty-one frames, 280 component seeds select 532 pixels. Exposed facial skin
changes. Her moving hands stay gloved and the collar covers her neck;
North-facing frames have no exposed skin, so their empty seed lists preserve
every pixel. Gray hair, lashes, eye details, gold hood, cream fur trim, gloves,
green dress, brooch and boots retain their original colors. None of the four
world skin shades occurs in other materials in these pinned sources.

All source frames were inspected in five-choice sheets. Original pixel grids
were checked around the face, hair, hood, collar, moving gloves, dress and boots.
The material test checks every pixel independently of component seeds, plus
539 distinct literal material landmarks, per-frame counts, transparency,
metadata, unchanged profile roles and prior output bytes against slice 198.
North-facing frames additionally require complete image equality. Brooch
highlights are absent from some East-facing poses, and some South-facing poses
lack the darker boot shade; literal landmarks reflect the actual visible colors.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Action East | 7 | 36, 36, 36, 36, 36, 36, 36 |
| Action North | 7 | 0, 0, 0, 0, 0, 0, 0 |
| Action South | 7 | 40, 40, 40, 40, 40, 40, 40 |

All strips retain 80×80 frames, the Default atlas, offsets 40/54 and durations
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`. The action cycle remains linear
with South as default, native West mirroring, standing status,
`on_pause_speaking = "idle"` and `last_frame_hold = [240, 360]`. It configures
neither background pause nor speaking-turn pause behavior.

## Offline review

- [Five-choice sample summary](../../../generated/slice-199-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-199-review/index.html).

The summary samples Action East frame 2, Action South frame 3 and Action North
frame 7. Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full
review includes all twenty-eight cases, Vanilla left and Debug Blue right,
with full views and enlarged details. Full crops include every nontransparent
pixel. Images, exports and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 411 regular tests and the
release build passed through the Nix shell; 481 opt-in tests were ignored.
All twenty-seven Elsie material tests and three GML runtime tests passed locally.
Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the three registry additions. Four valid mask mutations
omitted East or South forehead or cheek components. Each generated successfully
and failed the intended pixel assertion. All three metadata sidecars matched
independent archive reads.

All 620 standalone Elsie variants passed strict recipe validation; 1,240 final
PNG/metadata files match the author candidate. All 19,624 combined variants
validated. The 1,520 prior Elsie PNG/metadata files and 47,685 other-character
files match slice 198 byte for byte. Previous runtime rows, choices and hotkeys
also match. All 36 export reports retain prior entries and the archive hash.
The five frozen configuration inputs and original archive retain their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty-eight cases include fifty-six full-view bindings, enlarged details and
fifteen summary bindings; 5,091,360 rendered pixels match their source crops.
Full views include every nontransparent pixel. The complete review is
526,070 bytes; its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-199-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`9b8d6a495ceb4d5ea50291e41226c9e77cbd3c78cafcfc85bbf9a6c38e12c860`.
The installed Elsie NPC configuration matches the original byte for byte.

The archive's native animator and NPC animate function passed 140 frame/palette
observations, twenty linear completions and forty final-frame-hold checks under
Fabricator. Palette binding, frame phase, West mirroring, standing status,
wrapper idempotence, cycle counters and Winter portrait synchronization were
preserved. Engine services are simulated and cycles selected directly. Natural
gameplay dispatch, speaking interruptions and live rendering were not exercised.
The desktop launcher and installed game were not changed.

Reusable authoring sources and verification records are retained under
`local/authoring/slice-199/`. Scratch outputs use `tmp/slice-199-` and
`tmp/elsie-winter-general-author-`. Fresh exports are in
`extracted/elsie-winter-general-study`, reached through
`extracted/test-corpus/elsie`. The fixed comparison baseline is
`local/fixtures/slice-198-build/characters/elsie`. Unchanged characters in that
snapshot link to their existing slice 186 fixtures to avoid duplicate storage.
Required inputs remain outside the disposable output directories; see
[local storage](local-storage.md).

## Remaining work

All 35 Spring, 32 Summer and 32 Autumn world strips are covered, plus twenty
of 29 Winter strips. Elsie's remaining world inventory has nine Winter strips:
three standing-writing strips and six seated-writing strips. Next: Winter
standing writing. User art approval and live-game verification remain pending.
