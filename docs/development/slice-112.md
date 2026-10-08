# Josephine Spring standing writing

Following `429a714`, this slice adds Josephine's three Spring writing
start/loop/end strips to her accepted 55 sources. All eight frames face South;
there are no native West mirrors. The user approved the artwork on 2026-10-07. Outputs use
`generated/review/` and `generated/build/`.

Josephine has 58 sources; the 36-character collection has 4,519 sources and
18,076 variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices
with portrait/world synchronization. Only three regions and registry entries
are added. Source colors, color groups, target mappings, sets, previous regions
and runtime code remain unchanged. Historical pixel tests retain fixed
expectations; only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-111-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 55 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The batch reuses the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Its 206 component seeds select
427 skin pixels across all eight frames. These shades belong exclusively to
skin in this batch, including the writing hand, the hand beneath the board
and exposed ankles. Brown hair, eyes, pink earrings, orange shawl, pink skirt,
shoes, writing board and pen stay original. The board uses `#B28159`; the white
pen detail uses `#F5F5F5`. Neither belongs to the skin ramp.

Every frame was inspected in five-choice contact sheets, with source grids
around the hands, board, pen, clothing and shoes. The material test independently
checks every output pixel against a shade oracle, 58 fixed landmarks and
per-frame selected counts, plus alpha and metadata. It distinguishes both
moving hands from the adjacent board and pen, and checks face, ankles, hair,
clothing and writing tools in every frame. Profiles contain explicit component
seeds and source hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Start has two frames at 0.125 seconds each; loop has four frames at
0.1/0.125/0.1/0.3 seconds; end has two at 0.125/0.1 seconds. The native writing
cycle is a South-facing complex start/loop/end sequence.

## Offline review

- [Five-choice sample summary](../../generated/slice-112-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-112-review/index.html).

The summary samples start frame 1, loop frame 3 and end frame 1. The full gallery
includes all eight frames, with Vanilla left and Debug Blue right, full character
views and enlarged face, hands, writing tools and ankles. Game-derived images
and packages remain local and ignored by Git.
The five pages and images total 195,703 bytes (about 191 KiB).

## Verification

Formatting, strict release Clippy, all 324 regular tests and the release build
passed. The regular run leaves 394 opt-in tests ignored; a separate run passed
182 local artwork and GML tests. Four negative controls omit the face,
writing hand, hand beneath the board or ankles. Each generates successfully,
then fails the intended material assertion. The package test failed on an
unregistered writing strip before registration and passed afterward.

All 232 standalone variants validate strictly; their 464 PNG/metadata files
match the inspected candidate. All five gallery pages pass Chromium loading
and overflow checks. Exact comparisons cover all eight frames, metadata
bindings, full views, enlarged details and fifteen summary bindings:
1,879,072 rendered pixels.

The archive's native animator and NPC animation function pass forty
frame/palette observations and twenty complex stage transitions using
simulated engine services and direct cycle selection. Checks retain frame
phase, wrapper idempotence, cycle counters and Spring portrait synchronization.
There are no linear completions or final-frame holds in this batch.

All 18,076 combined variants validate. The 550 prior Josephine PNG/metadata
files and 44,785 other-character files match the previous combined build byte
for byte. Previous runtime rows and all 36 source export reports retain their
expected values. Five frozen registry/recipe inputs remain unchanged through
generation and installation.

MOMI installation, compilation and installed sprite/metadata verification
passed in isolated `tmp/slice-112-playtest`. Both original backups retain the
source hash above. The short review path passes Chromium loading and local
link resolution. No preview helper was installed or desktop launcher changed.
Natural gameplay dispatch, live rendering, an uninstall roundtrip and the
other local opt-in tests have not been exercised.

Evidence uses `tmp/slice-112-`; author records use
`tmp/josephine-spring-writing-author-`. Fresh exports are in
`extracted/josephine-spring-writing-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-111-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves ten Spring strips: South-facing sweeping and
singing start/loop/end; East-facing polishing start/loop/end; and chopping
North. Summer, Autumn and Winter each have 33 strips left. Next: Spring
polishing. Inventory is recorded in `tmp/slice-112-remainder.json`. Natural
gameplay dispatch and live rendering remain pending;
this coverage describes the pinned archive.
