# Josephine Spring polishing

Following `5860e1a`, this slice adds Josephine's three Spring polishing
start/loop/end strips to her accepted 58 sources. Nine source frames and nine
native West mirrors give eighteen review cases. The user approved the artwork on 2026-10-08.
Outputs use `generated/review/` and `generated/build/`.

Josephine has 61 sources; the 36-character collection has 4,522 sources and
18,088 variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices
with portrait/world synchronization. Only three regions and registry entries
are added. Source colors, color groups, target mappings, sets, previous regions
and runtime code remain unchanged. Historical pixel tests retain fixed
expectations; only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-112-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 58 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The batch reuses the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Its 209 component seeds select
421 skin pixels across all nine frames. These shades belong exclusively to
skin in this batch. The exposed hands in start/end and ankles change; the
loop's hands are hidden behind the cloth and glass. Brown hair, eyes, pink
earrings, orange shawl, pink skirt, shoes, polishing cloth and glass stay
original. The blue glass includes `#85CAEA` and the cloth includes `#547BB0`;
neither belongs to the skin ramp. Their original blue shades can make a
Debug Blue comparison less obvious, so they have explicit preservation checks.

Every source frame was inspected in five-choice contact sheets, with source
grids around the exposed hands, cloth, glass, clothing and shoes. The material
test independently checks every output pixel against a shade oracle, 56 fixed
landmarks and per-frame selected counts, plus alpha and metadata. Landmarks
cover the face, ankles, hair and skirt in every frame, glass and cloth when
present, and light/deep hand shades in the exposed poses. Profiles contain
explicit component seeds and source hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Start has three frames at 0.125 seconds each; loop has four frames at
0.1/0.125/0.1/0.225 seconds; end has two at 0.125/0.1 seconds. The native
polishing cycle is an East-facing complex start/loop/end sequence; West uses
its native horizontal mirror.

## Offline review

- [Five-choice sample summary](../../generated/slice-113-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-113-review/index.html).

The summary samples start frame 1, loop frame 3 and end frame 1. The full gallery
includes all nine source frames and their West mirrors, with Vanilla left and
Debug Blue right, full character views and enlarged face, hands, cloth, glass
and ankles. Game-derived images and packages remain local and ignored by Git.
The eight pages and images total 360,850 bytes (about 352 KiB).

## Verification

Formatting, strict release Clippy, all 325 regular tests and the release build
passed. The regular run leaves 395 opt-in tests ignored; a separate run passed
183 local artwork and GML tests. Four negative controls omit the face,
start hand, end hands or ankles. Each generates successfully, then fails the
intended material assertion. The package test failed on an unregistered
polishing strip before registration and passed afterward.

All 244 standalone variants validate strictly; their 488 PNG/metadata files
match the inspected candidate. All eight gallery pages pass Chromium loading
and overflow checks. Exact comparisons cover all eighteen cases, metadata
bindings, native mirrors, full views, enlarged details and fifteen summary
bindings: 3,642,912 rendered pixels.

The archive's native animator and NPC animation function pass ninety
frame/palette observations and forty complex stage transitions using simulated
engine services and direct cycle selection. Checks retain frame phase, West
mirroring, wrapper idempotence, cycle counters and Spring portrait
synchronization. There are no linear completions or final-frame holds here.

All 18,088 combined variants validate. The 580 prior Josephine PNG/metadata
files and 44,785 other-character files match the previous combined build byte
for byte. Previous runtime rows and all 36 source export reports retain their
expected values. Five frozen registry/recipe inputs remain unchanged through
generation and installation.

MOMI installation, compilation and installed sprite/metadata verification
passed in isolated `tmp/slice-113-playtest`. Both original backups retain the
source hash above. The short review path passes Chromium loading and local
link resolution. No preview helper was installed or desktop launcher changed.

Natural gameplay dispatch, live rendering, an uninstall roundtrip and the
other local opt-in tests have not been exercised.

Evidence uses `tmp/slice-113-`; author records use
`tmp/josephine-spring-polishing-author-`. Fresh exports are in
`extracted/josephine-spring-polishing-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-112-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves seven Spring strips: South-facing sweeping and
singing start/loop/end, plus chopping North. Summer, Autumn and Winter each
have 33 strips left. Next: Spring singing. Inventory is recorded in
`tmp/slice-113-remainder.json`. Natural gameplay dispatch and
live rendering remain pending; this coverage describes the pinned archive.
