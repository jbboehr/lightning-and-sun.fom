# Darcy Spring standing and walking

Following `5618597`, this slice starts Darcy's world coverage with six Spring
standing and walking strips. Fifteen source frames and five native West mirrors
give twenty review cases. The user approved the artwork on 2026-10-09. Outputs use
`generated/review/` and `generated/build/`.

Darcy has 38 sources: her 32 portraits and six world strips. The 36-character
collection has 4,634 sources and 18,536 variants. B retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. Only Darcy
switches collection sets, from her portrait set to `darcy-world-trial.json`.
Her portrait-only profile, set, stylized recipe and tests remain unchanged.
No runtime code changes. Historical pixel tests retain fixed expectations;
only the central Darcy coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-140-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed game
version. All 32 prior portrait PNG pins match fresh exports. The retained portrait
test corpus also matches the fresh PNGs and metadata. No hash override or pin
refresh was needed.

Darcy's world sprites reuse one deep portrait shade but have different light,
middle and shadow shades. The world profile appends three roles while preserving
all ten portrait roles and their target mappings:

| World source | Role | Debug Blue |
| --- | --- | --- |
| `#FCD9B3` | Light skin; new world role | `#9DB9D4` |
| `#F6996C` | Middle skin; new world role | `#7F9FBD` |
| `#C76539` | Skin shadow; new world role | `#6687AD` |
| `#7B2806` | Deep skin contour; existing portrait role | `#445F83` |

The three new shades form a separate connected-color group. Their targets
reuse accepted light, middle and shadow portrait targets for each preset.
The 262 component seeds select 427 skin pixels across all fifteen frames:
face, ears, neck and moving hands. These four shades belong exclusively to skin
in this batch. Gray hair, eye details, red headwear and skirt, teal clothing,
dark leggings and boots stay original. No ankles are exposed. Profiles contain
explicit seeds and source hashes, without new runtime rules.

Every source frame was inspected in five-choice sheets, with source grids for
face shadows, hands, neck and nearby materials. The material test checks every
output pixel with an independent shade oracle, 63 fixed landmarks and per-frame
selected counts, plus transparency and metadata. The landmarks include the
face or exposed neck and both hand areas in every frame, plus neighboring hair,
eye details, clothing and boots. All original portrait regions, color groups
and target mappings are checked, along with their fixed earlier output bytes.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Standing has one frame; walking has four 0.15-second frames. Both native cycles
are linear, with West mirroring East. Native speaking-turn and walking pause
settings remain unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-141-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-141-review/index.html).

The summary samples standing South and walking East/North. The full gallery
includes every source frame and native West mirror, with Vanilla left and Debug
Blue right, full character views and enlarged face, hand and clothing details.
Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 353 regular tests and the release
build passed through the Nix shell. The regular run left 423 opt-in tests
ignored; the selected local suite passed 212 tests, including Darcy's unchanged
portrait test, the new material test, retained character checks and three GML
runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the six new entries. Four valid
mask mutations independently omitted the face, left hand, right hand or rear
neck; each generated successfully and failed the intended material assertion.
All six raw metadata sidecars matched independent original-archive reads.

All 152 standalone Darcy variants passed strict recipe validation, and their
304 PNG/metadata files matched the author candidate. All 18,536 combined variants
validated. The 320 prior Darcy PNG/metadata files and 46,135 other-character
files are byte-identical to slice 140; previous runtime rows, choices and
hotkeys also match. Only Darcy changes collection sets. All 36 export reports
retain prior source entries and the archive hash. The five frozen configuration
inputs and both original archive copies retained their hashes.

All ten Chromium gallery pages loaded their local links and images without
overflow. The twenty cases include five West mirrors, 40 full-view bindings,
enlarged details and fifteen summary bindings; 3,862,560 rendered pixels match
their source crops. Full-view bounds include every nontransparent pixel. The
complete review is 397,699 bytes. The short review/build aliases point to this
slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-141-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `728d106cffb7506f149fe30538cf2fb6fde4e97c807a1d22d3d164a591d4a7fd`.
The archive's native animator and NPC animate function passed one hundred
frame/palette observations and forty linear completions under Fabricator. Checks
preserve native mirroring, palette binding, frame phase, wrapper idempotence,
cycle counters and Spring portrait synchronization. Engine services are
simulated and cycles selected directly. This does not exercise natural gameplay
dispatch, speaking transitions or live rendering. The desktop launcher and
installed game were not changed.

Evidence uses `tmp/slice-141-`; author records use `tmp/darcy-world-author-`.
Fresh exports are in `extracted/darcy-world-study`, reached through
`extracted/test-corpus/darcy`. The fixed portrait baseline is
`generated/slice-140-build/characters/darcy`. `evidence.json` summarizes results;
`scope.log` checks the twelve expected paths and unchanged historical tests.

## Remaining work

The archive inventory leaves nine Spring strips and fifteen each for Summer,
Autumn and Winter. Next: Spring blinking, sitting and eating, followed by general
actions. Inventory is recorded in `tmp/slice-141-remainder.json`.
Natural gameplay dispatch and live rendering remain pending; this coverage
describes the verified archive. The previously deferred portrait lip-highlight
contrast is unchanged.
