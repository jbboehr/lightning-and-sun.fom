# Josephine Spring general actions and shocked reaction

Following `5c6bc86`, this slice adds six Spring strips to Josephine's accepted
49 sources: general actions North/South/East and shocked start/loop/end South.
The 24 source frames and seven native West mirrors give 31 review cases.
The user approved the artwork on 2026-10-07. Outputs use `generated/review/` and
`generated/build/`.

Josephine has 55 sources; the 36-character collection has 4,516 sources and
18,064 variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices
with portrait/world synchronization. Only six regions and registry entries are
added. Source colors, color groups, target mappings, sets and previous regions
remain unchanged. No runtime code changes. Historical pixel tests retain fixed
expectations; only the central Josephine coverage total grows.

## Source and materials

The mount was empty. Source is the original archive backup at
`tmp/slice-110-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 49 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The batch reuses the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Its 552 component seeds select
1,029 skin pixels across all 24 frames: face, ears, neck, moving hands and
exposed ankles, including both raised hands in the shocked loop. These shades
belong exclusively to skin in this batch. No footwear cutoff is needed; skin
extends through row 50 in the shocked pose. Brown hair, eye detail, pink
earrings, orange shawl, pink skirt, shoes and mouth interiors stay original.
Red `#9E2626` and dark `#410808` mouth pixels remain unchanged. Profiles contain
explicit seeds and source hashes, without new runtime rules.

Every frame was inspected in Vanilla/Blue contact sheets, with five-choice
samples and source grids around the raised fingers, moving hands, face and
feet. The material test independently checks every output pixel against a
shade oracle, 58 fixed landmarks and per-frame selected counts, plus alpha and
metadata. Landmarks distinguish both raised hands and ankles from neighboring
hair, clothing and shoes; every general-action frame has a skin landmark.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
General actions have seven frames with 0.1/0.25/0.25/0.25/0.25/0.1/0.4-second
durations and native 240–360-tick final-frame holds. West mirrors East.
The shocked reaction is a South-only complex start/loop/end sequence with one
frame in each strip. Its stages and cycle metadata remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-111-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-111-review/index.html).

The summary samples general actions East/North and the shocked loop. The full
gallery includes every source frame, native West mirror and shocked stage,
with Vanilla left and Debug Blue right, full character views and enlarged
face, hand and ankle details. Game-derived images and packages remain local
and ignored by Git.

## Verification

Formatting, strict release Clippy, all 323 regular tests and the release build
passed. The regular run leaves 393 opt-in tests ignored; a separate run passed
181 local artwork and GML tests. Four negative controls remove face, left-hand,
right-hand or ankle coverage; each generates successfully, then fails the
independent material assertion as intended. The package test failed on the
unsupported action before registration and passes with the six additions.

All 220 standalone variants and 18,064 combined variants validate strictly.
The 440 standalone PNG/metadata files match the reviewed candidate. All 490
prior Josephine PNG/metadata files and 44,785 other-character files match the
previous combined build byte for byte. Previous runtime rows, all 36 source
export reports and the five frozen build inputs retain their expected values.
The original archive backup remains unchanged.

The full gallery covers all 31 cases across thirteen browser pages and totals
585,663 bytes. Checks compare 5,729,568 preview pixels against the generated sprites and confirm
metadata, frame bounds, crops and native mirrors. Chromium loads all local
images and links without overflow, including the short review alias.

The current archive's native animator and NPC animation function passed 155
palette/frame observations, twenty linear completions, forty hold checks and
twenty complex transitions under Fabricator. Engine services and cycle
selection are simulated. MOMI installation and installed sprite/metadata
verification passed in isolated `tmp/slice-111-playtest`; no preview helper or
desktop change was made. Natural gameplay, live rendering, an uninstall
roundtrip and the other opt-in tests were not exercised.

Evidence uses `tmp/slice-111-`; author records use
`tmp/josephine-spring-reactions-author-`. Fresh exports are in
`extracted/josephine-spring-reactions-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-110-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves thirteen Spring strips: South-facing writing,
sweeping and singing, each with start/loop/end stages; East-facing polishing
start/loop/end; and chopping North. Summer, Autumn and Winter each have 33
strips left. Start the Spring special activities with South-facing writing.
Inventory is recorded in `tmp/slice-111-remainder.json`. Natural gameplay
dispatch and live rendering remain pending; this coverage
describes the pinned archive.
