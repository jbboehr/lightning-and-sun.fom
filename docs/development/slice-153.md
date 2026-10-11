# Dell Spring standing and walking

Following `744bfe4`, this slice starts Dell's world coverage with six Spring
standing and walking strips. Fifteen source frames and five native West
mirrors give twenty review cases. The user approved the artwork on 2026-10-10. Outputs use
`generated/review/` and `generated/build/`.

Dell has 38 sources: her 32 portraits and six world strips. The 36-character
collection has 4,694 sources and 18,776 variants. Y retains Vanilla, Debug
Blue, Hayden, Ryis and Seridia choices with portrait/world synchronization.
Only Dell switches collection sets, from her portrait set to
`dell-world-trial.json`. Her portrait recipes stay available. No runtime
code changes. Historical pixel tests retain fixed expectations; only the
central Dell coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-152-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed
game version. All 32 portrait PNG pins match fresh exports, as do their
retained PNG and metadata bytes. No hash override or pin refresh was needed.

Dell's world sprites use four shades distinct from her portrait palette:

| World shade | Role | Debug Blue target |
| --- | --- | --- |
| `#ECC889` | Light skin | `#9DB9D4` |
| `#DF8D4B` | Middle skin, also hair | `#7F9FBD` |
| `#B65932` | Skin shadow, also hair | `#6687AD` |
| `#762E21` | Deep skin outline, also hair | `#445F83` |

The world profile appends these four shades in separate groups, preserving
all twelve portrait roles, six original color groups, 32 regions and target
mappings. Natural presets append their existing light/middle/shadow/deep
colors. Separate groups prevent neighboring hair shades from joining a skin
selection. The 342 explicit component seeds select 487 skin pixels across
all fifteen frames: face, ears, neck, hands and exposed ankles. Blonde hair,
including matching warm shades, eye details, red hair ribbon, pink shirt,
blue overalls, brown leg trim and red boots remain original. The rear
neck is hidden by hair; walking North frames 2 and 4 have no exposed ankle
pixels. Profiles contain source hashes and seeds, with no new runtime rules.

Every source frame was inspected in five-choice sheets and source grids.
The material test checks every pixel against reviewed face-row bounds and
body material colors, independently of component seeds, with 229 fixed
landmarks and per-frame selected counts. Landmarks cover face, neck, both
hands, ankles where visible, matching hair shades, hair highlights, eyes,
shirt, all three overall shades, brown leg trim and boots. Frame-specific
vertical offsets preserve the one-pixel walking bob. The test also checks
transparency, metadata, profile/set preservation and all prior output bytes
against the fixed slice 152 baseline.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Standing has one frame without a duration; walking has four 0.15-second
frames. Both native cycles are linear, with West mirroring East. Speaking-turn
and walking-pause settings remain unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-153-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-153-review/index.html).

The summary samples standing South and walking East/North. The full gallery
includes every source frame and native West mirror, with Vanilla left and
Debug Blue right, full character views and enlarged face, hands, clothing
and ankle details. Game-derived images and packages remain local and ignored
by Git.

## Verification

Formatting, Clippy with warnings denied, all 365 regular tests and the release
build passed through the Nix shell. The regular run left 435 opt-in tests
ignored; the selected local suite passed 225 tests, including Dell's unchanged
portrait test, the new material test, retained character checks and three GML
runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the six new entries. Four valid
mask mutations independently omitted the face, hand or exposed ankles, or
selected a hair component; each generated successfully and failed the intended
material assertion.
All six raw metadata sidecars matched independent original-archive reads.

All 152 standalone Dell variants passed strict recipe validation, and their
304 PNG/metadata files matched the author candidate. All 18,776 combined variants
validated. The 320 prior Dell PNG/metadata files and 46,735 other-character
files are byte-identical to slice 152; previous runtime rows, choices and
hotkeys also match. Only Dell changes collection sets. All 36 export reports
retain prior source entries and the archive hash. The five frozen configuration
inputs and both original archive copies retained their hashes.

All ten Chromium gallery pages loaded their local links and images without
overflow. The twenty cases include five West mirrors, 40 full-view bindings,
enlarged details and fifteen summary bindings; 3,862,560 rendered pixels match
their source crops. Full-view bounds include every nontransparent pixel. The
complete review is 385,296 bytes. The short review/build aliases point to this
slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-153-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `ccb6ebea7483b3255864d3e2cdfec028633c19f4d0d2877e41ba9e81ce7f7040`.
The archive's native animator and NPC animate function passed one hundred
frame/palette observations and forty linear completions under Fabricator. Checks
preserve native mirroring, palette binding, frame phase, wrapper idempotence,
cycle counters and Spring portrait synchronization. Engine services are
simulated and cycles selected directly. This does not exercise natural gameplay
dispatch, speaking transitions or live rendering. The desktop launcher and
installed game were not changed.

Evidence uses `tmp/slice-153-`; author records use `tmp/dell-world-author-`.
Fresh exports are in `extracted/dell-world-study`, reached through
`extracted/test-corpus/dell`. The fixed portrait baseline is
`generated/slice-152-build/characters/dell`. `evidence.json` summarizes results;
`scope.log` checks the twelve expected paths and unchanged historical tests.

## Remaining work

The archive inventory leaves 27 Spring, 24 Summer, 21 Autumn and 21 Winter
world strips. Next: Spring blinking and sitting, then eating and drinking,
general actions and special animations. Inventory is recorded in
`tmp/slice-153-remainder.json`. Natural gameplay dispatch, speaking transitions
and live rendering remain pending; this coverage describes the verified
archive.
