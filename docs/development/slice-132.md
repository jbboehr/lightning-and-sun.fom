# Josephine Autumn sweeping

Following `666d20b`, this slice adds Josephine's three Autumn sweeping
start/loop/end strips and completes her Autumn world folder. All fifteen frames
face South; there are no native West mirrors. User art review is pending.
Outputs use `generated/review/` and `generated/build/`.

Josephine now has 134 sources: 32 portraits, all 36 Spring world strips, all
33 Summer world strips and all 33 Autumn world strips. The 36-character
collection has 4,595 sources and 18,380 variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices
with portrait/world synchronization. Only three regions and registry entries
are added. Source colors, groups, target mappings, sets and runtime code remain
unchanged. Historical pixel tests retain their expectations; only the central
Josephine coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-131-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 131 prior PNG pins match fresh exports. No hash override or pin
refresh was needed.

The strips reuse the four reviewed world shades from [slice 109](slice-109.md):
`#9B643A`, `#7E442F`, `#713222` and `#57271D`. Their 368 component seeds select
805 skin pixels across fifteen source frames, including the face, moving hands
and exposed ankles. These shades belong exclusively to skin here. Brown hair,
eye details, cream and orange clothing, blue earrings, shoes, broom and dust
stay original. The broom's wood uses `#653D25`, `#936244` and `#BB8151`; its
bristles use `#E9B03A` and `#FFE36F`. Gray `#A0A0A0` dust remains unchanged.

Every source frame was inspected in five-choice contact sheets, with source
grids around the hands, clothing and broom. The material test independently
checks every output pixel against a literal shade oracle, 57 fixed landmarks
and per-frame selected counts, plus alpha and metadata. Landmarks span all three
stages and include the face, hands, visible ankles and neighboring materials.
The earlier 131 sources are compared with the fixed slice 131 baseline. The
profile contains component seeds and pinned source hashes, without new runtime
rules.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Start and end each have three frames at 0.1 seconds per frame. The nine-frame
loop retains durations 0.05, 0.1, 0.125, 0.1, 0.1, 0.1, 0.125, 0.1 and 0.05
seconds. The native sweeping cycle remains a South-facing complex start/loop/end
sequence. Its existing speaking-pause behavior selects idle.

## Offline review

- [Five-choice sample summary](../../../generated/slice-132-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-132-review/index.html).

The summary samples start frame 2, loop frame 3 and end frame 3. The full gallery
includes all fifteen frames, with Vanilla left and Debug Blue right, full views
and enlarged face, hand and ankle details. Full views include the broom and all
dust particles. Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy, all 344 regular tests and the release build passed through
the Nix shell. The regular run left 414 opt-in tests ignored; the selected local
suite passed 202 tests, including the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the new entries. Four valid mask
mutations independently omitted the face, left hand, right hand and ankle
regions; each generated successfully and failed the intended material assertion.
All three raw metadata sidecars matched independent original-archive reads.

All 536 standalone Josephine variants validated, and their 1,072 PNG/metadata
files matched the author candidate. All 18,380 combined variants validated.
The 1,310 prior Josephine PNG/metadata files and 44,785 other-character files are
byte-identical to slice 131; previous runtime rows also match. All 36 export
reports retain prior source entries and the archive hash. The five frozen
configuration inputs and both original archive copies retained their hashes.

The seven Chromium pages loaded all local links and images without overflow.
The fifteen cases have 30 full-view bindings plus enlarged details and fifteen
summary bindings; 2,950,440 rendered pixels match their source crops. Full-view
bounds include every nontransparent pixel. The complete review is 312,743 bytes.
The short review/build aliases point to this slice, and the review loads through
its short alias.

The isolated MOMI installation passed in `tmp/slice-132-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `285e0fc1108775e6111443b81015240997d84ed5f383bc68007fedf128f4ccb6`.
The archive's native animator and NPC animate function passed seventy-five
frame/palette observations and twenty complex stage transitions under Fabricator.
Checks preserve frame phase, wrapper idempotence, cycle counters and Autumn
portrait synchronization. There are no linear completions or final-frame holds
here. Engine services were simulated and cycles selected directly. This does
not exercise natural gameplay dispatch, speaking transitions or live rendering.
The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-132-`; author records use
`tmp/josephine-autumn-sweeping-author-`. Fresh exports are in
`extracted/josephine-autumn-sweeping-study`, reached through
`extracted/test-corpus/josephine`. The fixed prior-art baseline is
`generated/slice-131-build/characters/josephine`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

Spring, Summer and Autumn world folders are complete. Winter has 33 strips left.
Next: Winter standing and walking. Inventory is recorded in
`tmp/slice-132-remainder.json`. Natural gameplay dispatch, speaking transitions
and live rendering remain pending.
