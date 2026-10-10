# Darcy Autumn general actions

Following `de159eb`, this slice adds the three North/South/East general-action
strips, completing Darcy's fifteen Autumn world strips. Twenty-one source
frames and seven native West mirrors give twenty-eight review cases.
The user approved the artwork on 2026-10-10. Outputs use `generated/review/` and `generated/build/`.

Darcy has 77 sources; the 36-character collection has 4,673 sources and 18,692
variants. B retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Source colors, color groups, target mappings, sets and previous regions
remain unchanged. No runtime code changes. Historical pixel tests retain fixed
expectations; only the central Darcy coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-148-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed
game version. All 74 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world shades from [slice 141](slice-141.md):
`#FCD9B3`, `#F6996C`, `#C76539` and `#7B2806`. Its 317 component seeds select
535 skin pixels across all twenty-one frames: face, ears, exposed neck and
moving hands. These shades belong exclusively to skin in this batch. Gray
hair, eye details, tan coat, white cuffs, teal clothing and dark boots stay
original. Coat shades are `#7C4E3A`, `#9F7159`, `#C69D7C` and
`#DEC1A8`; white details use `#F5F5F5`, `#C9C9CC` and `#AAAEB7`.
Dark boots use `#2C1E1E` and `#4F4040`. The rear-neck patch is exposed
in every North frame. The small patch beside the hair in North frames 2–5
is coat fabric and stays original. Profiles contain explicit seeds and source
hashes, without new runtime rules.

Every source frame was inspected in five-choice sheets, with source grids
around extended fingers, rear-neck pixels, moving hands, coat and footwear.
The material test checks every output pixel against an independent shade
oracle, 152 fixed landmarks and per-frame selected counts, plus transparency
and metadata. Landmarks cover the face or rear neck, moving hands, coat,
white details and boots in each frame, all four extended-finger shades,
far-side hands, eye details, hair and the coat patch beside the hair.
Prior output bytes are compared against the fixed slice 148 baseline.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
All three strips have seven frames with durations
0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds. The native cycle is linear with a
240–360-tick final-frame hold; West mirrors East. Its speaking-pause fallback
to idle remains unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-149-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-149-review/index.html).

The summary samples actions South (frame 3), East (frame 2) and North (frame 6).
The full gallery includes every source frame and native West mirror, with
Vanilla left and Debug Blue right, full character views and enlarged face,
hand and clothing details. Game-derived images and packages remain local and
ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 361 regular tests and the release
build passed through the Nix shell. The regular run left 431 opt-in tests
ignored; the selected local suite passed 220 tests, including Darcy's unchanged
portrait and prior Spring/Summer/Autumn tests, the new material test, retained
character checks and three GML runtime tests. Historical pixel tests are
unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the three new entries. Four
valid mask mutations independently omitted the face, extended forward hand,
rear-view raised hand or rear neck; each
generated successfully and failed the intended material assertion. All three
raw metadata sidecars matched independent archive reads.

All 308 standalone Darcy variants passed strict recipe validation, and their
616 PNG/metadata files matched the author candidate. All 18,692 combined
variants validated. The 740 prior Darcy PNG/metadata files and 46,135 other
character files are byte-identical to slice 148; previous runtime rows, choices
and hotkeys also match. All prior regions, colors, groups and target mappings
remain unchanged, as do the collection sets. All 36 export reports retain
prior source entries and the archive hash. The five frozen configuration inputs
and both original archive copies retained their hashes.

All ten Chromium gallery pages loaded their local links and images without
overflow. The twenty-eight cases include seven West mirrors, 56 full-view
bindings, enlarged details and fifteen summary bindings; 5,220,384 rendered
pixels match their source crops. Full-view bounds include every nontransparent
pixel. The complete review is 501,375 bytes. The short review/build aliases
point to this slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-149-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `8818926790be41ea7d5b80a651d35245bb0d8d0b7fb54bef83db6fe2e585cce1`.
The archive's native animator and NPC animate function passed 140
frame/palette observations, twenty linear completions and forty final-frame
hold checks under Fabricator. Checks preserve native mirroring, palette
binding, frame phase, wrapper idempotence, cycle counters and Autumn portrait
synchronization. Engine services are simulated and cycles selected directly.
This does not exercise natural gameplay dispatch, speaking transitions or
live rendering. The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-149-`; author records use
`tmp/darcy-autumn-general-author-`. Fresh exports are in
`extracted/darcy-autumn-general-study`, reached through
`extracted/test-corpus/darcy`. The fixed prior-art baseline is
`generated/slice-148-build/characters/darcy`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves fifteen Winter strips; Spring, Summer and
Autumn are complete. Next: Winter standing and walking.
Inventory is recorded in `tmp/slice-149-remainder.json`. Natural gameplay
dispatch and live rendering remain pending; this coverage describes the
verified archive. The previously deferred portrait lip-highlight contrast is
unchanged.
