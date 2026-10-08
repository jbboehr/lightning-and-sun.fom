# Josephine Spring sweeping

Following `beab7ac`, this slice adds Josephine's three Spring sweeping strips:
start, loop and end. All fifteen source frames face South, with no native West
mirrors. The user approved the artwork on 2026-10-08. Outputs use `generated/review/` and
`generated/build/`.

Josephine now has 68 sources: 32 portraits and all 36 Spring world strips. The
36-character collection has 4,529 sources and 18,116 variants. P retains Vanilla,
Debug Blue, Hayden, Ryis and Seridia choices with portrait/world synchronization.
Only three regions and registry entries are added. Source colors, groups,
target mappings, sets and runtime code remain unchanged. Historical pixel tests
retain their expectations; only the central Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-115-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 65 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 416 component seeds select
852 skin pixels across fifteen frames. These shades belong exclusively to skin
here. Face, neck, moving hands and exposed ankles follow the palette. Brown
hair, eyes, earrings, orange shawl, pink skirt, shoes, broom and dust stay
original. The broom's wood shades `#653D25`, `#936244` and `#BB8151`, yellow
bristles `#E9B03A` and `#FFE36F`, and gray dust `#A0A0A0` are distinct from skin.

Every frame was inspected in five-choice contact sheets, with source grids
around the overlapping broom handle and hands, opposite sweep poses, clothing,
ankles and dust. The material test independently checks every output pixel
against a literal shade oracle, 68 fixed landmarks and per-frame selected
counts, plus alpha and metadata. It also compares the earlier 65 sources with
the fixed slice 115 baseline. The profile contains component seeds and pinned
source hashes, without new runtime rules.

Metadata retains 80×80 frames, the Default atlas, numeric offsets 40/54 and
original durations. Start and end each have three frames at 0.1 seconds; the
nine-frame loop uses `[0.05, 0.1, 0.125, 0.1, 0.1, 0.1, 0.125, 0.1, 0.05]`.
The native cycle is complex, with start/loop/end transitions, and speaking
selects idle. Its configuration is unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-116-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-116-review/index.html).

The summary samples start frame 2, loop frame 3 and end frame 3. The full gallery
includes all fifteen frames with Vanilla left and Debug Blue right, full views
including the broom and every dust particle, and enlarged face, hands and
ankles. Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy, all 328 regular tests and the release build passed through
the Nix shell. The regular run left 398 opt-in tests ignored; the selected local
suite passed 186 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted face, left-hand, right-hand and ankle regions;
each generated successfully and failed the intended material assertion. Three
raw metadata sidecars matched independent original-archive reads.

All 272 standalone Josephine variants validated, and their 544 PNG/metadata
files matched the author candidate. All 18,116 combined variants validated.
The 650 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 115; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The seven Chromium pages loaded all local links and images without overflow.
The fifteen cases have thirty full-view bindings plus enlarged details and
fifteen summary bindings; 2,950,440 rendered pixels match their source crops.
The complete review is 313,640 bytes. The short review/build aliases point to
this slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-116-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `db1a2932845b98ab751ab5f4036ec59f30f385cd66592b953d96c2f3dfc7d1ef`.
The archive's native animator and NPC animate function passed 75 frame/palette
observations and twenty complex transitions under Fabricator. Engine services
were simulated and the cycle selected directly. This does not exercise natural
gameplay dispatch, speaking-pause transitions or live rendering. The desktop
launcher and installed game were not changed.

Evidence uses `tmp/slice-116-`; author records use
`tmp/josephine-spring-sweeping-author-`. Fresh exports are in
`extracted/josephine-spring-sweeping-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-115-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Spring world coverage is complete for this pinned archive. Summer, Autumn and
Winter each have 33 strips left. Next: the six Summer idle/walk strips.
Inventory is recorded in `tmp/slice-116-remainder.json`. Natural gameplay
dispatch and live rendering remain pending.
