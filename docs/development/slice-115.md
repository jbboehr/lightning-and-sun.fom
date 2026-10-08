# Josephine Spring chopping

Following `70d2161`, this slice adds Josephine's Spring chopping strip to her
accepted 64 sources. All twenty frames face North; there are no native West
mirrors. The user approved the artwork on 2026-10-08. Outputs use `generated/review/` and
`generated/build/`.

Josephine has 65 sources; the 36-character collection has 4,526 sources and
18,104 variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices
with portrait/world synchronization. Only one region and registry entry are
added. Source colors, color groups, target mappings, sets, previous regions and
runtime code remain unchanged. Historical pixel tests retain fixed expectations;
only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-114-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 64 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strip reuses the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Its 109 component seeds select
178 skin pixels across all twenty frames, including the moving knife hand,
the other hand when visible and exposed ankles. These shades belong exclusively
to skin here. Brown hair, orange shawl, pink skirt, shoes, knife and green food
particles stay original. The shawl's light `#FED5AD` remains unchanged. Knife
colors include `#8A9B9E`, `#CEE9EE` and `#423C39`; food colors include `#3B6D5B`,
`#68CCA7` and `#82E5C0`. None belongs to the skin ramp.

Every source frame was inspected in five-choice contact sheets, with source
grids around the raised knife hand, chopping poses, resting arms, clothing,
shoes and food particles. The material test independently checks every output
pixel against a shade oracle, 52 fixed landmarks and per-frame selected counts,
plus alpha and metadata. The knife hand and ankles have literal landmarks in
every frame. Additional landmarks distinguish the other hand, hair, shawl,
skirt, knife and food. The profile contains explicit component seeds and a
source hash, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas, numeric offsets 40/54 and all
twenty original durations, including the initial 0.15/0.3 seconds, the faster
0.075/0.05-second chopping frames and the final 0.8 seconds. The native cycle
is North-facing and linear. Its existing chopping sound and speaking-pause
behavior are unchanged; speaking selects idle.

## Offline review

- [Five-choice sample summary](../../generated/slice-115-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-115-review/index.html).

The summary samples frames 1, 2 and 20: initial pose, raised knife and resting
arms. The full gallery includes all twenty frames, with Vanilla left and Debug
Blue right, full views including every food particle and enlarged hands,
knife and ankles. Game-derived images and packages remain local and ignored
by Git.

## Verification

Formatting, Clippy, all 327 regular tests and the release build passed through
the Nix shell. The regular run left 397 opt-in tests ignored; the selected local
suite passed 185 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entry. Four valid mask
mutations independently omitted the knife hand, other hand, left ankle and
right ankle; each generated successfully and failed the intended material
assertion. One raw metadata sidecar matched the original archive.

All 260 standalone Josephine variants validated, and their 520 PNG/metadata
files matched the author candidate. All 18,104 combined variants validated.
The 640 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 114; previous runtime rows also match. All 36 export
reports retain the prior source entries and archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The seven Chromium pages loaded all local links and images without overflow.
The twenty cases have forty full-view bindings plus enlarged details and
fifteen summary bindings; 3,596,320 rendered pixels match their source crops.
The complete review is 369,943 bytes. `generated/review/` and `generated/build/`
now resolve to this slice, and the review also loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-115-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `2921a5af574d1f4badc27073f725d3bcf4e92a0571e31dfe1159f6ee7f53f2ac`.
The archive's native animator and NPC animate function passed 100 frame/palette
observations and five linear completions under Fabricator, with engine services
simulated and the cycle selected directly. This does not exercise natural
gameplay dispatch, speaking-pause transitions, audible playback or live
rendering. The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-115-`; author records use
`tmp/josephine-spring-chopping-author-`. Fresh exports are in
`extracted/josephine-spring-chopping-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-114-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Only the three South-facing sweeping start/loop/end strips remain in Spring.
Summer, Autumn and Winter each have 33 strips left. Next: Spring sweeping.
Inventory is recorded in `tmp/slice-115-remainder.json`. Natural gameplay
dispatch and live rendering remain pending; this coverage
describes the pinned archive.
