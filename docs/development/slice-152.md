# Darcy Winter general actions

Following `83c694b`, this slice adds the three North/South/East general-action
strips, completing Darcy's fifteen Winter world strips. Twenty-one source
frames and seven native West mirrors give twenty-eight review cases. The user
approved the artwork on 2026-10-10. Outputs use `generated/review/` and `generated/build/`.

Darcy has 92 sources: 32 portraits and 60 world strips. The 36-character
collection has 4,688 sources and 18,752 variants. B retains Vanilla, Debug
Blue, Hayden, Ryis and Seridia choices with portrait/world synchronization.
Only three regions and registry entries are added. Source colors, color
groups, target mappings, sets and previous regions remain unchanged. No
runtime code changes. Historical pixel tests retain fixed expectations;
only the central Darcy coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-151-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed
game version. All 89 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world shades from [slice 141](slice-141.md):
`#FCD9B3`, `#F6996C`, `#C76539` and `#7B2806`. Its 281 component seeds select
474 skin pixels across all twenty-one frames: face, ears and exposed hands.
These shades belong exclusively to skin in this batch. Winter's raspberry
scarf covers the rear neck. Gray hair, eye details, scarf, cream coat, white
cuffs, mint outfit and dark boots stay original. Scarf shades are `#681830`,
`#8D2544` and `#D03563`; the coat uses `#50453A`, `#8F7F69`, `#BCAB8A` and
`#EBD9B6`. Cuffs are white `#FFFFFF`, while the mint outfit uses `#2C6C69`,
`#65A99F`, `#86DCC7` and `#C4FFEC`. Profiles contain explicit seeds and
source hashes, without new runtime rules.

Every source frame was inspected in five-choice sheets, with source grids
around extended fingers, moving hands, scarf, cuffs, coat and boots. The
material test checks every output pixel against an independent shade oracle,
229 fixed landmarks and per-frame selected counts, plus transparency and
metadata. Landmarks cover the face where visible, moving hands, three scarf
shades, coat, cuffs, mint clothing and boots in every frame, plus hair and
headwear. Specific landmarks cover all four shades in the extended fingers,
eye details and the small coat patch beside the scarf in North frames 2–5.
North frames 2–5 have just four selected hand pixels each; the other hand
and rear neck are covered. East frame 2 has one dark finger pixel at local
(48,45), which must change with the rest of the hand. Prior output bytes are
compared against the fixed slice 151 baseline.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
All three strips have seven frames with durations
0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds. The native cycle is linear with a
240–360-tick final-frame hold; West mirrors East. Its speaking-pause fallback
to idle remains unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-152-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-152-review/index.html).

The summary samples actions South (frame 3), East (frame 2) and North (frame 6).
The full gallery includes every source frame and native West mirror, with
Vanilla left and Debug Blue right, full character views and enlarged face,
hand and clothing details. Game-derived images and packages remain local and
ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 364 regular tests and the release
build passed through the Nix shell. The regular run left 434 opt-in tests
ignored; the selected local suite passed 223 tests, including Darcy's unchanged
portrait and prior Spring/Summer/Autumn/Winter tests, the new material test, retained
character checks and three GML runtime tests. Historical pixel tests are
unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the three new entries. Four
valid mask mutations independently omitted the face, extended forward hand,
rear-view raised hand or single dark finger pixel; each
generated successfully and failed the intended material assertion. All three
raw metadata sidecars matched independent archive reads.

All 368 standalone Darcy variants passed strict recipe validation, and their
736 PNG/metadata files matched the author candidate. All 18,752 combined
variants validated. The 890 prior Darcy PNG/metadata files and 46,135 other
character files are byte-identical to slice 151; previous runtime rows, choices
and hotkeys also match. All prior regions, colors, groups and target mappings
remain unchanged, as do the collection sets. All 36 export reports retain
prior source entries and the archive hash. The five frozen configuration inputs
and both original archive copies retained their hashes.

All ten Chromium gallery pages loaded their local links and images without
overflow. The twenty-eight cases include seven West mirrors, 56 full-view
bindings, enlarged details and fifteen summary bindings; 5,220,384 rendered
pixels match their source crops. Full-view bounds include every nontransparent
pixel. The complete review is 497,431 bytes. The short review/build aliases
point to this slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-152-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `2f4155908374d74c6ac0ccff1bfb3ca8265e2dc8948ed805d891c5cfa35eb3ef`.
The archive's native animator and NPC animate function passed 140
frame/palette observations, twenty linear completions and forty final-frame
hold checks under Fabricator. Checks preserve native mirroring, palette
binding, frame phase, wrapper idempotence, cycle counters and Winter portrait
synchronization. Engine services are simulated and cycles selected directly.
This does not exercise natural gameplay dispatch, speaking transitions or
live rendering. The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-152-`; author records use
`tmp/darcy-winter-general-author-`. Fresh exports are in
`extracted/darcy-winter-general-study`, reached through
`extracted/test-corpus/darcy`. The fixed prior-art baseline is
`generated/slice-151-build/characters/darcy`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves no uncovered Darcy PNGs. All four seasons have
fifteen world strips, alongside the 32 existing portraits. The inventory is
recorded in `tmp/slice-152-remainder.json`. After art review, work can move to
the next character's world sprites. Natural gameplay dispatch, speaking
transitions and live rendering remain pending. The previously deferred
portrait lip-highlight contrast is unchanged.
