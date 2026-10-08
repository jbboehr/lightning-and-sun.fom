# Josephine Summer blinking and seated actions

Following `d804182`, this slice adds Josephine's eleven Summer blink, sit, eat
and drink strips. There are 31 source frames and twelve native West mirrors:
43 review cases. The user approved the artwork on 2026-10-08. Outputs use
`generated/review/` and `generated/build/`.

Josephine now has 85 sources: 32 portraits, all 36 Spring world strips and 17
Summer world strips. The 36-character collection has 4,546 sources and 18,184
variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only eleven regions and registry entries are
added. Source colors, groups, target mappings, sets and runtime code remain
unchanged. Historical pixel tests retain their expectations; only the central
Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-117-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 74 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 824 component seeds select
1,532 skin pixels across 31 source frames. These shades belong exclusively to
skin here. Face, neck, eyelids, moving hands and exposed ankles follow the
palette. Brown hair, eye details, earrings, pink flower, cream and peach
clothing, yellow shoes and mouth interiors stay original. The red `#9E2626`
and dark `#410808` mouth colors remain unchanged in the open-mouth eating poses.

Every source frame was inspected in five-choice contact sheets, with source
grids around closed eyes, raised hands, mouth edges and clothing. The material
test independently checks every output pixel against a literal shade oracle,
74 fixed landmarks and per-frame selected counts, plus alpha and metadata.
Landmarks cover closed eyelids, standing ankles, footwear, moving hands and
both mouth colors. The earlier 74 sources are compared with the fixed slice
117 baseline. The profile contains component seeds and pinned source hashes,
without new runtime rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Blink timing is `[0.075, 0.125, 0.075]`; sit strips are single-frame without
explicit timing. Drink and eat North retain three frames at 1 second. Eat
East/South retain five frames at `[0.125, 0.15, 0.175, 0.125, 0.6]`. Native
seated state, linear cycles, West mirroring and the eat/drink final-frame hold
range `[240, 360]` remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-118-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-118-review/index.html).

The summary samples blink South frame 2, eat East frame 2 and drink North frame 2.
The full gallery includes all 43 cases with Vanilla left and Debug Blue right,
full views and enlarged face, hands and ankles. West views mirror the East
source exactly as the native animator does. Game-derived images and packages
remain local and ignored by Git.

## Verification

Formatting, Clippy, all 330 regular tests and the release build passed through
the Nix shell. The regular run left 400 opt-in tests ignored; the selected local
suite passed 188 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted face, hand, ankle and eyelid regions; each
generated successfully and failed the intended material assertion. Eleven raw
metadata sidecars matched independent original-archive reads.

All 340 standalone Josephine variants validated, and their 680 PNG/metadata
files matched the author candidate. All 18,184 combined variants validated.
The 740 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 117; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The twenty Chromium pages loaded all local links and images without overflow.
The 43 cases have 86 full-view bindings plus enlarged details and fifteen
summary bindings; 7,766,304 rendered pixels match their source crops. The
complete review is 763,384 bytes. The short review/build aliases point to this
slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-118-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `76071bdb5dfa9b57a2bd4f289ac09f1e996f5a8fb53ff91f5041f0481b276017`.
The archive's native animator and NPC animate function passed 215 frame/palette
observations, 75 linear completions and 80 final-frame hold boundary checks
under Fabricator, including West mirroring and seated state. Engine services
were simulated and cycles selected directly. This does not exercise natural
gameplay dispatch or live rendering. The desktop launcher and installed game
were not changed.

Evidence uses `tmp/slice-118-`; author records use
`tmp/josephine-summer-actions-author-`. Fresh exports are in
`extracted/josephine-summer-actions-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-117-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Summer has sixteen strips left; Autumn and Winter each have 33. Next: Summer
general actions. Inventory is recorded in `tmp/slice-118-remainder.json`.
Natural gameplay dispatch and live rendering remain pending.
