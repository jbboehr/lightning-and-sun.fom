# Darcy Winter blinking, sitting and eating

Following `7799a7d`, this slice adds six Winter blinking, sitting and eating
strips. Fourteen source frames and four native West mirrors give eighteen
review cases. The user approved the artwork on 2026-10-10. Outputs use `generated/review/`
and `generated/build/`.

Darcy has 89 sources; the 36-character collection has 4,685 sources and 18,740
variants. B retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only six regions and registry entries are
added. Source colors, color groups, target mappings, sets and previous regions
remain unchanged. No runtime code changes. Historical pixel tests retain fixed
expectations; only the central Darcy coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-150-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed
game version. All 83 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world shades from [slice 141](slice-141.md):
`#FCD9B3`, `#F6996C`, `#C76539` and `#7B2806`. Its 305 component seeds select
473 skin pixels across all fourteen frames: face, ears, eyelids and exposed
hands. These shades belong exclusively to skin in this batch. Winter's
raspberry scarf covers the rear neck. Gray hair, lashes, eye details, scarf,
cream coat, white cuffs, mint outfit, dark boots and mouth interiors stay
original. Scarf shades are `#681830`, `#8D2544` and `#D03563`; the coat uses
`#50453A`, `#8F7F69`, `#BCAB8A` and `#EBD9B6`. Cuffs are white `#FFFFFF`,
while the mint outfit uses `#2C6C69`, `#65A99F`, `#86DCC7` and `#C4FFEC`.
Profiles contain explicit seeds and source hashes, without new runtime rules.

Every source frame was inspected in five-choice sheets, with source grids
around eyelids, mouth, moving hands, scarf, cuffs and seated poses. The
material test checks every output pixel against an independent shade oracle,
170 fixed landmarks and per-frame selected counts, plus transparency and
metadata. Landmarks cover the face, exposed hands, three scarf shades, coat,
cuffs, mint clothing and boots where visible, plus hair and headwear.
Specific blink and eating landmarks protect lashes and mouth interiors while
checking the closed eyelid and raised hand. The rear seated view has two small
hands and no exposed neck. Prior output bytes are compared against the fixed
slice 150 baseline.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Blinking has three frames at 0.075/0.125/0.075 seconds; eating has five at
0.125/0.15/0.175/0.125/0.6 seconds; sitting has one frame without a duration.
All three native cycles are linear. Blinking uses South/East with a West
mirror; sitting adds North, while eating is South-only. Sitting and eating
retain seated status, and eating retains its 240–360-tick final-frame hold.

## Offline review

- [Five-choice sample summary](../../../generated/slice-151-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-151-review/index.html).

The summary samples closed-eye blinking South, eating South and sitting North.
The full gallery includes every source frame and native West mirror, with
Vanilla left and Debug Blue right, full character views and enlarged face,
hand and clothing details. Game-derived images and packages remain local and
ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 363 regular tests and the release
build passed through the Nix shell. The regular run left 433 opt-in tests
ignored; the selected local suite passed 222 tests, including Darcy's unchanged
portrait, Spring/Summer/Autumn and Winter standing/walking tests, the new material
test, retained character checks and three GML runtime tests. Historical pixel
tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the six new entries. Four valid
mask mutations independently omitted the face, raised hand, seated hand or eyelid
selection; each generated successfully and failed the intended material
assertion. All six raw metadata sidecars matched independent archive reads.

All 356 standalone Darcy variants passed strict recipe validation, and their
712 PNG/metadata files matched the author candidate. All 18,740 combined
variants validated. The 830 prior Darcy PNG/metadata files and 46,135 other
character files are byte-identical to slice 150; previous runtime rows, choices
and hotkeys also match. All prior regions, colors, groups and target mappings
remain unchanged, as do the collection sets. All 36 export reports retain
prior source entries and the archive hash. The five frozen configuration inputs
and both original archive copies retained their hashes.

All eleven Chromium gallery pages loaded their local links and images without
overflow. The eighteen cases include four West mirrors, 36 full-view bindings,
enlarged details and fifteen summary bindings; 3,523,104 rendered pixels match
their source crops. Full-view bounds include every nontransparent pixel. The
complete review is 368,935 bytes. The short review/build aliases point to this
slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-151-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `0ef28dcfb44d0b190f8125a526cace1e8c70ee719870d2a3b0c74e4bbc71dcdf`.
The archive's native animator and NPC animate function passed ninety
frame/palette observations, forty linear completions and ten final-frame hold
checks under Fabricator. Checks preserve native mirroring, seated status,
palette binding, frame phase, wrapper idempotence, cycle counters and Winter
portrait synchronization. Engine services are simulated and cycles selected
directly. This does not exercise natural gameplay dispatch, speaking transitions
or live rendering. The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-151-`; author records use
`tmp/darcy-winter-actions-author-`. Fresh exports are in
`extracted/darcy-winter-actions-study`, reached through
`extracted/test-corpus/darcy`. The fixed prior-art baseline is
`generated/slice-150-build/characters/darcy`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves three Winter general-action strips. Spring,
Summer and Autumn are complete. Next: finish Winter with the general actions,
completing Darcy's world set for this archive. Inventory is recorded in
`tmp/slice-151-remainder.json`. Natural gameplay dispatch and live rendering
remain pending. The previously deferred portrait lip-highlight contrast is
unchanged.
