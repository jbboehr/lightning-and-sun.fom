# Josephine Summer chopping

Following `ba8a857`, this slice adds Josephine's Summer chopping strip. All
twenty frames face North; there are no native West mirrors. The user approved
the artwork on 2026-10-08. Outputs use `generated/review/` and `generated/build/`.

Josephine now has 98 sources: 32 portraits, all 36 Spring world strips and 30
Summer world strips. The 36-character collection has 4,559 sources and 18,236
variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only one region and registry entry are added.
Source colors, groups, target mappings, sets and runtime code remain unchanged.
Historical pixel tests retain their expectations; only the central Josephine
coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-122-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 97 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strip reuses the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Its 103 component seeds select
113 skin pixels across twenty source frames, including the moving hands and
exposed ankles. These shades belong exclusively to skin here. Brown hair,
cream and peach clothing, yellow shoes, knife and food particles stay original.
The knife's `#8A9B9E`, `#CEE9EE` and `#423C39` pixels and the food's `#3B6D5B`,
`#68CCA7` and `#82E5C0` pixels remain unchanged.

Every source frame was inspected in five-choice contact sheets, with source
grids around the hands, clothing, ankles, knife and food. The material test
independently checks every output pixel against a literal shade oracle,
49 fixed landmarks and per-frame selected counts, plus alpha and metadata.
Landmarks cover the knife hand in its sixteen visible frames, both ankles in
their ten visible frames, the other hand in three frames, and neighboring hair,
clothing, knife and food materials. The earlier 97 sources are compared with the
fixed slice 122 baseline. The profile contains component seeds and a pinned
source hash, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas, numeric offsets 40/54 and the
twenty original durations: 0.15, 0.3, 0.075, 0.05, 0.075, 0.075, 0.075, 0.05,
0.075, 0.075, 0.075, 0.05, 0.075, 0.075, 0.075, 0.05, 0.075, 0.075, 0.1, 0.8
seconds. The native chopping cycle is a North-facing linear sequence with
`SoundEffects/SpecialEvents/ReinaChop`; its speaking-pause behavior selects idle.

## Offline review

- [Five-choice sample summary](../../generated/slice-123-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-123-review/index.html).

The summary samples frames 1, 2 and 20. The full gallery includes all twenty
frames, with Vanilla left and Debug Blue right, full views and enlarged hand and
ankle details. Full views include the knife and every food particle. Game-derived
images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy, all 335 regular tests and the release build passed through
the Nix shell. The regular run left 405 opt-in tests ignored; the selected local
suite passed 193 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entry. Four valid mask
mutations independently omitted the knife hand, other hand, left ankle and right
ankle; each generated successfully and failed the intended material assertion.
The raw metadata sidecar matched an independent original-archive read.

All 392 standalone Josephine variants validated, and their 784 PNG/metadata
files matched the author candidate. All 18,236 combined variants validated.
The 970 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 122; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The seven Chromium pages loaded all local links and images without overflow.
The twenty cases have 40 full-view bindings plus enlarged details and fifteen
summary bindings; 3,596,320 rendered pixels match their source crops. Full-view
bounds include every nontransparent pixel. The complete review is 361,003 bytes.
The short review/build aliases point to this slice, and the review loads through
its short alias.

The isolated MOMI installation passed in `tmp/slice-123-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `e023fb52079f4f0145fa4b49e9215232df51dcca0559838b8478aa125bc835f6`.
The archive's native animator and NPC animate function passed one hundred
frame/palette observations and five linear completions under Fabricator. Checks preserve
frame phase, wrapper idempotence, cycle counters and Summer portrait
synchronization. There are no final-frame holds or complex transitions here.
Engine services are simulated and the cycle selected directly. This does not
exercise natural gameplay dispatch, speaking transitions, audible playback or
live rendering. The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-123-`; author records use
`tmp/josephine-summer-chopping-author-`. Fresh exports are in
`extracted/josephine-summer-chopping-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-122-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Summer has three strips left: sweeping start/loop/end. Autumn and Winter each
have 33. Next: Summer sweeping, completing her Summer world folder. Inventory is
recorded in `tmp/slice-123-remainder.json`. Natural gameplay dispatch, speaking
transitions and live rendering remain pending.
