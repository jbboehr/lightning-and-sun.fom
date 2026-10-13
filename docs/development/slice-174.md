# Elsie Spring standing and walking

Following `082b9b9`, this slice begins Elsie's world coverage with Spring
standing and walking in East, North and South, plus native West mirroring.
Fifteen source frames give twenty review cases. The user approved the artwork
on 2026-10-12.
Local outputs use `generated/review/` and `generated/build/`.

Elsie has 42 sources: her 36 portraits and six world strips. The 36-character
collection has 4,793 sources and 19,172 variants. Z retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. No runtime
code changes are needed. Her portrait-only recipes remain available; the
combined collection now selects her world set. All previous portrait regions,
groups and target mappings are retained. Historical pixel tests keep their
expectations; only the central Elsie coverage total grows.

## Source and materials

Source is the preserved original archive at
`tmp/slice-173-playtest/.mistria-palette/previous.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This verifies that archive, not a newer installed game version. All 36 prior
Elsie PNG pins match fresh exports; portrait PNGs and metadata also match the
existing local corpus. No hash override or pin refresh was needed.

Four world shades extend her eight portrait roles: `#FBDCB9`, `#F3C193`,
`#E08C6A` and `#A45838`. Each has a separate component group, mapped to the
existing four target skin shades. Across fifteen frames, 355 seeds select 628
pixels. Forehead, face, neckline and exposed hands follow the palette. Gray
hair, eyebrows, eye details, cosmetics, earrings, pink gown and shoes stay
original. These four world shades occur only in skin in the six reviewed
sources. The final East walking frame hides the far hand behind the outfit;
North views expose only the hands. No ankles are exposed.

Every source frame was inspected in five-choice sheets and original pixel grids.
The material test checks every pixel independently of component seeds, plus
272 distinct literal material landmarks, per-frame counts, transparency,
metadata and retained portrait output bytes against slice 173. It explicitly
checks the unchanged portrait roles and mappings and the appended world roles.

| Strip | Frames | Selected pixels per frame |
| --- | --- | --- |
| Idle East | 1 | 53 |
| Idle North | 1 | 10 |
| Idle South | 1 | 63 |
| Walk East | 4 | 53, 58, 53, 52 |
| Walk North | 4 | 10, 9, 10, 9 |
| Walk South | 4 | 63, 61, 63, 61 |

All strips retain 80×80 frames, the Default atlas and offsets 40/54. Idle has
one frame and no duration property; walking retains four frames at 0.15 seconds
each. Both cycles remain linear, default to South and use native West mirroring.
Neither is seated or has a final-frame hold. Both retain
`on_pause_speaking_turn = true`; walking also retains speaking and background
pause transitions to idle.

## Offline review

- [Five-choice sample summary](../../../generated/slice-174-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-174-review/index.html).

The summary samples Idle South frame 1, Walk East frame 2 and Walk North frame 2.
Columns are Vanilla, Debug Blue, Hayden, Ryis and Seridia. The full review
includes every source frame and native West mirror, Vanilla left and Debug Blue
right, with full views and enlarged details. Images, exports and packages remain
local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 386 regular tests and the
release build passed through the Nix shell; 456 opt-in tests were ignored.
The selected local suite passed 247 tests, including the original Elsie portrait
test, the new world material test, historical character checks and three GML
runtime tests. Historical pixel-test expectations are unchanged.

The synthetic package test first failed with the expected unsupported-animation
error, then passed after the six registry additions. Four valid mask mutations
omitted the face, neckline, left hand or right hand. Each generated successfully
and failed the intended pixel assertion. All six metadata sidecars matched
independent archive reads.

All 168 standalone Elsie variants passed strict recipe validation; 336 final
PNG/metadata files match the author candidate. All 19,172 combined variants
validated. The 360 prior Elsie portrait PNG/metadata files and 47,685
other-character files match slice 173 byte for byte. Previous runtime rows,
choices and hotkeys also match. All 36 export reports retain prior entries and
the archive hash. Five frozen configuration inputs and both original archive
copies retained their hashes.

All ten Chromium gallery pages loaded local images and links without overflow.
Twenty cases include forty full-view bindings, enlarged details and fifteen
summary bindings; 3,862,560 rendered pixels match their source crops. Full views
include every nontransparent pixel. The complete review is 434,814 bytes;
its short alias loads successfully.

The isolated MOMI installation passed in `tmp/slice-174-playtest`, including
installed-pixel and animation-metadata verification. Installed archive SHA-256:
`a6cb37f758c0addfb82ea8330190b68176ebd840561031be8f84a736d31b2228`.
The archive's native animator and NPC animate function passed 100 frame/palette
observations and forty linear completions under Fabricator. Palette binding,
frame phase, West mirroring, unseated status, wrapper idempotence, cycle counters
and Spring portrait synchronization were preserved. Engine services are
simulated and cycles selected directly. Natural gameplay dispatch, pause
transitions and live rendering were not exercised. The desktop launcher and
installed game were not changed.

The scope check verifies twelve expected paths, an empty Git index, unchanged
historical pixel tests and ignored game-derived artifacts.

Evidence uses `tmp/slice-174-`; author records use `tmp/elsie-world-author-`.
Fresh exports are in `extracted/elsie-world-study`, reached through
`extracted/test-corpus/elsie`. The fixed portrait baseline is
`generated/slice-173-build/characters/elsie`.

## Remaining work

Elsie's remaining world inventory has 29 Spring, 32 Summer, 32 Autumn and
29 Winter strips. Next: Spring blinking and sitting, five strips. The user
approved this slice; live-game verification remains pending.
