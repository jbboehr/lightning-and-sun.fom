# Josephine Spring singing

Following `9a965f2`, this slice adds Josephine's three Spring singing
start/loop/end strips to her accepted 61 sources. All fourteen frames face
South; there are no native West mirrors. The user approved the artwork on 2026-10-08.
Outputs use `generated/review/` and `generated/build/`.

Josephine has 64 sources; the 36-character collection has 4,525 sources and
18,100 variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices
with portrait/world synchronization. Only three regions and registry entries
are added. Source colors, color groups, target mappings, sets, previous regions
and runtime code remain unchanged. Historical pixel tests retain fixed
expectations; only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-113-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 61 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The batch reuses the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Its 530 component seeds select
983 skin pixels across all fourteen frames, including both moving hands and
exposed ankles. These shades belong exclusively to skin in this batch. Brown
hair, eye details, pink earrings, orange shawl, pink skirt, shoes, mouth interiors
and floating musical notes stay original. The red `#9E2626` and dark `#410808`
mouth pixels remain unchanged, as do the notes' blue `#599DFD`, pink `#FF6888`
and green `#95DA53` shades.

Every source frame was inspected in five-choice contact sheets, with source
grids around the mouth, hands, clothing, shoes and musical notes. The material
test independently checks every output pixel against a shade oracle, 52 fixed
landmarks and per-frame selected counts, plus alpha and metadata. Both hands
have a literal landmark in every frame. Additional landmarks cover skin and
neighboring materials in each stage, three open-mouth poses and each note
color. Profiles contain explicit component seeds and source hashes, without
new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Start and end each have two frames; loop has ten. Every frame lasts 0.1 seconds.
The native singing cycle is a South-facing complex start/loop/end sequence.
Its existing speaking-pause behavior selects idle; that dispatch is unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-114-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-114-review/index.html).

The summary samples start frame 1, loop frame 3 and end frame 1. The full gallery
includes all fourteen frames, with Vanilla left and Debug Blue right, full
views and enlarged face, hands and ankles. The wider full-view crop includes
every floating musical note. Game-derived images and packages remain local
and ignored by Git.
The seven pages and images total 294,279 bytes (about 287 KiB). Close-ups focus
on the face, hands and ankles, excluding cut-off note fragments.

## Verification

Formatting, strict release Clippy, all 326 regular tests and the release build
passed. The regular run leaves 396 opt-in tests ignored; a separate run passed
184 local artwork and GML tests. Four negative controls omit the face, left
hand, right hand or ankles. Each generates successfully, then fails the intended
material assertion. The package test failed on an unregistered singing strip
before registration and passed afterward.

All 256 standalone variants validate strictly; their 512 PNG/metadata files
match the inspected candidate. All seven gallery pages pass Chromium loading
and overflow checks. Exact comparisons cover all fourteen cases, metadata
bindings, full views, enlarged details and fifteen summary bindings:
3,064,000 rendered pixels. Full-view bounds include every nontransparent pixel.

The archive's native animator and NPC animation function pass seventy
frame/palette observations and twenty complex stage transitions using simulated
engine services and direct cycle selection. Checks retain frame phase, wrapper
idempotence, cycle counters and Spring portrait synchronization. There are no
linear completions or final-frame holds here. Speaking-pause dispatch is not
exercised by these direct-cycle checks.

All 18,100 combined variants validate. The 610 prior Josephine PNG/metadata
files and 44,785 other-character files match the previous combined build byte
for byte. Previous runtime rows and all 36 source export reports retain their
expected values. Five frozen registry/recipe inputs remain unchanged through
generation and installation.

MOMI installation, compilation and installed sprite/metadata verification
passed in isolated `tmp/slice-114-playtest`. Both original backups retain the
source hash above. The short review path passes Chromium loading and local
link resolution. No preview helper was installed or desktop launcher changed.
Natural gameplay dispatch, live rendering, an uninstall roundtrip and the
other local opt-in tests have not been exercised.

Evidence uses `tmp/slice-114-`; author records use
`tmp/josephine-spring-singing-author-`. Fresh exports are in
`extracted/josephine-spring-singing-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-113-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves four Spring strips: South-facing sweeping
start/loop/end, plus chopping North. Summer, Autumn and Winter each have 33
strips left. Next: Spring chopping, then sweeping. Inventory is recorded in
`tmp/slice-114-remainder.json`. Natural gameplay dispatch and
live rendering remain pending; this coverage describes the pinned archive.
