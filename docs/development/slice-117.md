# Josephine Summer movement

Following `2ab6f5f`, this slice adds Josephine's six Summer idle/walk strips.
Fifteen source frames and five native West mirrors give twenty review cases.
The user approved the artwork on 2026-10-08. Outputs use `generated/review/` and
`generated/build/`.

Josephine now has 74 sources: 32 portraits, all 36 Spring world strips and six
Summer world strips. The 36-character collection has 4,535 sources and 18,140
variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only six regions and registry entries are
added. Source colors, groups, target mappings, sets and runtime code remain
unchanged. Historical pixel tests retain their expectations; only the central
Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-116-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 68 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 457 component seeds select
736 skin pixels across fifteen source frames. These shades belong exclusively
to skin here. Face, neck, exposed arms, hands and ankles follow the palette.
Brown hair, eye details, earrings, pink flower, cream and peach clothing and
yellow shoes stay original. Clothing shades include `#E7866A`, `#F6B486`,
`#FAD8B8` and `#FFF9F0`; flower shades include `#D53E6D` and `#EF667C`. Shoes
use `#D68D47` and `#EABF67`, distinct from the skin ramp.

Every source frame was inspected in five-choice contact sheets, with source
grids around swinging arms, ankles, clothing, footwear and eye details. The
material test independently checks every output pixel against a literal shade
oracle, 97 fixed landmarks and per-frame selected counts, plus alpha and
metadata. It also compares the earlier 68 sources with the fixed slice 116
baseline. The profile contains explicit component seeds and pinned source
hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Idle strips are single-frame without explicit duration or frame count. Walk
strips retain four frames at 0.15 seconds. Both cycles are linear, using North,
South and East sprites with native West mirroring. Their speaking and
background-pause configuration is unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-117-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-117-review/index.html).

The summary samples idle South frame 1, walk East frame 2 and walk North frame 2.
The full gallery includes all twenty cases with Vanilla left and Debug Blue
right, full views and enlarged face, hands and ankles. West views mirror the
East source exactly as the native animator does. Game-derived images and
packages remain local and ignored by Git.

## Verification

Formatting, Clippy, all 329 regular tests and the release build passed through
the Nix shell. The regular run left 399 opt-in tests ignored; the selected local
suite passed 187 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted face, left-hand, right-hand and ankle regions;
each generated successfully and failed the intended material assertion. Six
raw metadata sidecars matched independent original-archive reads.

All 296 standalone Josephine variants validated, and their 592 PNG/metadata
files matched the author candidate. All 18,140 combined variants validated.
The 680 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 116; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The ten Chromium pages loaded all local links and images without overflow.
The twenty cases have forty full-view bindings plus enlarged details and
fifteen summary bindings; 3,862,560 rendered pixels match their source crops.
The complete review is 398,188 bytes. The short review/build aliases point to
this slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-117-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `f3879e6270eb8a07f8f54e0f0590e15a429e4e59fbacd407a8b5df0ece6f3922`.
The archive's native animator and NPC animate function passed 100 frame/palette
observations and forty linear completions under Fabricator, including West
mirroring. Engine services were simulated and cycles selected directly. This
does not exercise natural gameplay dispatch, pause/turn transitions or live
rendering. The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-117-`; author records use
`tmp/josephine-summer-movement-author-`. Fresh exports are in
`extracted/josephine-summer-movement-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-116-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Summer has 27 strips left; Autumn and Winter each have 33. Next: Summer blinking
and seated actions. Inventory is recorded in `tmp/slice-117-remainder.json`.
Natural gameplay dispatch and live rendering remain pending.
