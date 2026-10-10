# Darcy Summer general actions

Following `75d6a46`, this slice adds the three North/South/East general-action
strips, completing Darcy's fifteen Summer world strips. Twenty-one source
frames and seven native West mirrors give twenty-eight review cases.
The user approved the artwork on 2026-10-10. Outputs use `generated/review/` and `generated/build/`.

Darcy has 62 sources; the 36-character collection has 4,658 sources and 18,632
variants. B retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only three regions and registry entries are
added. Source colors, color groups, target mappings, sets and previous regions
remain unchanged. No runtime code changes. Historical pixel tests retain fixed
expectations; only the central Darcy coverage total grows.

## Source and materials

The game mount was empty. Source is the original archive backup at
`tmp/slice-145-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last verified archive; this does not check a newer installed
game version. All 59 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world shades from [slice 141](slice-141.md):
`#FCD9B3`, `#F6996C`, `#C76539` and `#7B2806`. Its 377 component seeds select
623 skin pixels across all twenty-one frames: face, ears, front neck,
exposed arms and moving hands. These shades belong exclusively to skin in
this batch. Gray hair, eye details, green blouse and rear collar, coral
headwear and skirt, and cream footwear stay original. The rear neck is covered
by the green collar. Footwear uses the separate `#E6C0A4` and `#FEF3DE`
shades; South frames 1 and 6 show only the lighter cream shade. Profiles contain
explicit seeds and source hashes, without new runtime rules.

Every source frame was inspected in five-choice sheets, with source grids
around extended fingers, rear hand fragments beside the hair, moving arms,
collar and footwear. The material test checks every output pixel against an
independent shade oracle, 125 fixed landmarks and per-frame selected counts,
plus transparency and metadata. Landmarks cover the face, moving hands, collar
and footwear in each applicable frame, all four extended-finger shades,
far-side hands, eye details, hair and clothing. Prior output bytes are compared
against the fixed slice 145 baseline.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
All three strips have seven frames with durations
0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds. The native cycle is linear with a
240–360-tick final-frame hold; West mirrors East. Its speaking-pause fallback
to idle remains unchanged.

## Offline review

- [Five-choice sample summary](../../../generated/slice-146-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../../generated/slice-146-review/index.html).

The summary samples actions South (frame 3), East (frame 2) and North (frame 6).
The full gallery includes every source frame and native West mirror, with
Vanilla left and Debug Blue right, full character views and enlarged face,
hand and clothing details. Game-derived images and packages remain local and
ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 358 regular tests and the release
build passed through the Nix shell. The regular run left 428 opt-in tests
ignored; the selected local suite passed 217 tests, including Darcy's unchanged
portrait and prior Spring/Summer tests, the new material test, retained character
checks and three GML runtime tests. Historical pixel tests are unchanged.

The synthetic package test failed before registration with the expected
unsupported-animation error, then passed with the three new entries. Four
valid mask mutations independently omitted the face, extended forward hand,
rear-view raised hand or the small hand fragment beside the hair; each
generated successfully and failed the intended material assertion. All three
raw metadata sidecars matched independent archive reads.

All 248 standalone Darcy variants passed strict recipe validation, and their
496 PNG/metadata files matched the author candidate. All 18,632 combined
variants validated. The 590 prior Darcy PNG/metadata files and 46,135 other
character files are byte-identical to slice 145; previous runtime rows, choices
and hotkeys also match. All prior regions, colors, groups and target mappings
remain unchanged, as do the collection sets. All 36 export reports retain
prior source entries and the archive hash. The five frozen configuration inputs
and both original archive copies retained their hashes.

All ten Chromium gallery pages loaded their local links and images without
overflow. The twenty-eight cases include seven West mirrors, 56 full-view
bindings, enlarged details and fifteen summary bindings; 5,220,384 rendered
pixels match their source crops. Full-view bounds include every nontransparent
pixel. The complete review is 500,998 bytes. The short review/build aliases
point to this slice, and the review loads through its short alias.

The isolated MOMI installation passed in `tmp/slice-146-playtest`, including
installed-pixel and animation-metadata verification. The installed archive's
SHA-256 is `d66ebed7ee53fcf342deaf1932ba8580b8152c136b8974bb00bd42b7c7785f03`.
The archive's native animator and NPC animate function passed 140
frame/palette observations, twenty linear completions and forty final-frame
hold checks under Fabricator. Checks preserve native mirroring, palette
binding, frame phase, wrapper idempotence, cycle counters and Summer portrait
synchronization. Engine services are simulated and cycles selected directly.
This does not exercise natural gameplay dispatch, speaking transitions or
live rendering. The desktop launcher and installed game were not changed.

Evidence uses `tmp/slice-146-`; author records use
`tmp/darcy-summer-general-author-`. Fresh exports are in
`extracted/darcy-summer-general-study`, reached through
`extracted/test-corpus/darcy`. The fixed prior-art baseline is
`generated/slice-145-build/characters/darcy`. `evidence.json` summarizes
results; `scope.log` checks the nine expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves fifteen strips each for Autumn and Winter;
Spring and Summer are complete. Next: Autumn standing and walking.
Inventory is recorded in `tmp/slice-146-remainder.json`. Natural gameplay
dispatch and live rendering remain pending; this coverage describes the
verified archive. The previously deferred portrait lip-highlight contrast is
unchanged.
