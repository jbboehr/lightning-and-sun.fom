# Darcy Spring blinking and seated actions

Following `d3b6a9c`, this slice adds six Spring strips: blinking East/South,
sitting North/South/East and eating South. Fourteen source frames and four native
West mirrors give eighteen review cases. The user approved the artwork on
2026-10-09. Outputs use `generated/review/` and `generated/build/`.

Darcy has 44 sources; the 36-character collection has 4,640 sources and 18,560
variants. B retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only six regions and registry entries are added.
Source colors, color groups, target mappings, sets and previous regions remain
unchanged. No runtime code changes. Historical pixel tests retain fixed
expectations; only the central Darcy coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-141-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed
game version. All 38 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world shades from [slice 141](slice-141.md):
`#FCD9B3`, `#F6996C`, `#C76539` and `#7B2806`. Its 313 component seeds select
523 skin pixels across all fourteen frames: face, ears, neck, eyelids and moving
hands. These shades belong exclusively to skin in this batch. Gray hair,
eyelashes and eye details, red headwear and skirt, teal clothing, dark leggings,
boots and mouth interiors stay original. No ankles are exposed. The red
`#9E2626` and dark `#410808` mouth pixels remain unchanged during eating.
Profiles contain explicit seeds and source hashes, without new runtime rules.

Every source frame was inspected in five-choice sheets, with source grids
around eyelids, mouth, hands and the exposed rear neck. The material test
checks every output pixel against an independent shade oracle, 72 fixed
landmarks and per-frame selected counts, plus transparency and metadata.
Closed-eyelid landmarks distinguish skin from black lashes; eating landmarks
protect the mouth interior and raised hand. Prior output bytes are compared
against the fixed slice 141 baseline.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Blinking has three frames with 0.075/0.125/0.075-second durations; eating has
five with 0.125/0.15/0.175/0.125/0.6-second durations; sitting has one frame.
All cycles are linear. Seated status and eating's 240–360-tick final-frame
hold are preserved. West mirrors East; eating is South-only.

## Offline review

- [Five-choice sample summary](../../../generated/slice-142-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-142-review/index.html).

The summary samples blinking South, eating South and sitting North. The full
gallery includes every source frame and native West mirror, with Vanilla left
and Debug Blue right, full character views and enlarged face, hand and clothing
details. Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 354 regular tests and the release
build passed through the Nix shell. The regular run left 424 opt-in tests
ignored; the selected local suite passed 213 tests, including Darcy's unchanged
portrait and standing/walking tests, the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the six new entries. Four valid
mask mutations independently omitted the face, raised hand, rear neck or eyelid
selection; each generated successfully and failed the intended material
assertion. All six raw metadata sidecars matched independent archive reads.

All 176 standalone Darcy variants passed strict recipe validation, and their
352 PNG/metadata files matched the author candidate. All 18,560 combined
variants validated. The 380 prior Darcy PNG/metadata files and 46,135 other
character files are byte-identical to slice 141; previous runtime rows, choices
and hotkeys also match. All prior regions, colors, groups and target mappings
remain unchanged, as do the collection sets. All 36 export reports retain
prior source entries and the archive hash. The five frozen configuration inputs
and both original archive copies retained their hashes.

All eleven Chromium gallery pages loaded their local links and images without
overflow. The eighteen cases include four West mirrors, 36 full-view bindings,
enlarged details and fifteen summary bindings; 3,523,104 rendered pixels match
their source crops. Full-view bounds include every nontransparent pixel. The
complete review is 371,929 bytes. The short review/build aliases point to this
slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-142-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `e339c96eecd1048de4d5414d7e48cbb592608ac5de3b458a22129880efc5f2a3`.
The archive's native animator and NPC animate function passed ninety
frame/palette observations, forty linear completions and ten final-frame hold
checks under Fabricator. Checks preserve native mirroring, seated status,
palette binding, frame phase, wrapper idempotence, cycle counters and Spring
portrait synchronization. Engine services are simulated and cycles selected
directly. This does not exercise natural gameplay dispatch, speaking transitions
or live rendering. The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-142-`; author records use
`tmp/darcy-spring-actions-author-`. Fresh exports are in
`extracted/darcy-spring-actions-study`, reached through
`extracted/test-corpus/darcy`. The fixed prior-art baseline is
`generated/slice-141-build/characters/darcy`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves three Spring general-action strips and fifteen
each for Summer, Autumn and Winter. Next: finish Spring with the three general
actions, then start Summer. Inventory is recorded in
`tmp/slice-142-remainder.json`. Natural gameplay dispatch and live rendering
remain pending; this coverage describes the verified archive. The previously
deferred portrait lip-highlight contrast is unchanged.
