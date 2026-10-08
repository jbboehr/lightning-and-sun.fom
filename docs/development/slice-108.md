# Holt Winter sweeping

Following `3386565`, this slice adds Holt's three Winter sweeping start/loop/end
strips to his accepted 140 sources. All twelve frames face South; there are no
native West mirrors. The user approved the artwork on 2026-10-07. Outputs use
`generated/review/` and `generated/build/`.

This completes Holt's four seasonal sprite folders in the pinned archive. His
143 sources comprise 32 portraits and 111 world strips. The 36-character
collection has 4,493 sources and 17,972 variants. H retains Vanilla, Debug Blue,
Hayden, Ryis and Seridia choices with portrait/world synchronization. Only three
regions and registry entries are added. Prior regions, source colors, color
groups, mappings, sets and runtime code remain unchanged. Historical pixel tests
retain their fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was empty. Source is the original archive backup at
`tmp/slice-107-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 140 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 249 component seeds select 601 skin pixels.
All twelve source frames were inspected in five-choice contact sheets, with
source grids around gripping hands, sleeves, broom, dust and boots. The four
shades belong exclusively to skin in these Winter strips. Removing the
provisional footwear boundary from the author helper produces the identical
profile. The material test checks every output pixel against an independent
shade oracle, 34 literal landmarks and fixed per-frame selected counts.

Each loop frame has a fixed gripping-hand landmark and a neighboring sleeve
landmark. For example, frame 1's hand at local (40,42) uses `#F0CC8E`, while the
sleeve at (42,42) uses `#622A10`. Frame 7's hand at (43,43) uses `#CD9067`, while
the sleeve at (45,43) uses `#E37D45`. Only the hands change. The shirt's mapped
`#B45732` at (34,41) and moustache's `#762E21` at (36,37) in loop frame 1 stay
original. The broom retains `#BB8151` at (42,47) and `#E9B03A` at (46,53);
frame 2's dust retains `#A0A0A0` at (55,53). Ginger hair, moustache, eyes, orange
plaid shirt, teal scarf, apron, trousers, boots, broom and dust stay original.
The profile stores seeds and source hashes, without adding runtime rules.

Metadata retains 80×80 frames, Default atlas and numeric offsets 40/54. Start
and end each have one frame with no explicit duration. The ten loop frames run
at 0.1 seconds each. The native sweeping cycle is a South-facing complex
start/loop/end sequence.

## Offline review

- [Five-choice sample summary](../../generated/slice-108-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-108-review/index.html).

The summary samples start frame 1 and loop frames 2 and 7. The full gallery
includes all twelve frames, with Vanilla left and Debug Blue right, full
character views and enlarged face, hand, broom and boot details. Game-derived
images and packages remain local and ignored by Git.

## Verification

- Formatting, release Clippy with warnings denied, all 320 regular tests and the
  release build passed. The regular run left 390 opt-in tests ignored; 177 local
  tests passed separately, including retained material checks and three GML checks.
- The new package test failed before registration with the expected unsupported
  animation error, then passed after registration. Four negative controls for
  missing face/hand pixels and clothing/moustache spill each reached and failed
  the intended pixel assertion after successful generation.
- All 572 standalone variants passed strict validation and matched the author
  candidate and combined package. All 17,972 combined variants validated;
  1,400 prior Holt PNG/metadata files and 43,675 other-character files stayed
  byte-identical, along with all previous runtime table rows.
- Preview checks compared 2,742,624 exact pixels, 24 full-frame bindings and
  15 summary bindings, plus metadata, frame and crop coverage. Chromium loaded
  all seven gallery pages with working images and links and no overflow.
  The gallery totals 278,599 bytes. The short review/build links resolve to
  this slice, and Chromium also loaded the review through its short link.
- The archive's native animator passed 60 frame/palette observations and
  20 complex-cycle transitions. Engine services were simulated and cycles
  selected directly; this does not exercise natural gameplay dispatch.
- MOMI built and installed the combined package in the isolated
  `tmp/slice-108-playtest` lab, including installed pixel and metadata checks.
  Five build inputs stayed frozen, and both original archive backups retained
  the source hash above. The desktop game and launcher were not changed.

Interactive gameplay, an uninstall roundtrip and other opt-in tests were not
rerun. The user approved the offline artwork on 2026-10-07.

Evidence uses `tmp/slice-108-`; author records use
`tmp/holt-winter-sweeping-author-`. Fresh exports are in
`extracted/holt-winter-sweeping-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-107-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Coverage and remaining work

Winter now covers all 27 strips. Alongside Spring's 30, Summer's 27 and Autumn's
27, all 111 seasonal world strips have masks. The archive inventory finds no
remaining unprofiled PNGs under Holt's animation directory, recorded in
`tmp/slice-108-remainder.json`. Coverage does not establish live-game behavior or
coverage of a newer game version. The user approved this final batch on 2026-10-07.

Further art work can move to another character's overworld pilot using the
existing one-character-at-a-time workflow.
