# Nora Spring tapping

Following `1d29918`, this slice adds three Spring tapping strips to Nora's
accepted 58 sources: start, loop and end South. Five source frames give five
review cases, approved by the user on 2026-10-03. This native sequence has no West
mirrors. Completed outputs use `generated/review/` and `generated/build/`.

Nora has 61 sources; the 36-character collection has 4,292 sources and 17,168
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-064-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 58 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the reviewed world skin shades from [slice 061](slice-061.md).
Its 101 seeds select 197 skin pixels across five frames: face, ears, neck,
moving hands and exposed ankles. Blonde hair, eye detail, pale blouse, blue
overalls, shoes, board and pen stay original. Every frame was inspected in
Vanilla/Blue contact sheets, with a five-choice tapping-loop sheet and source
grids around hands, tools and hair.

The three shades `#FCD9B3`, `#F0B988` and `#D37A57` are skin throughout these
exact strips. The deeper shared shade `#762E21` is entirely hair here, including
tips at row 40 in start/end. The board's brown shades and the pen's gold, pale
and pink details are distinct from the selected skin. Tracked masks contain
component seeds and hashes, not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Start and end each have one frame and no duration field. The three-frame loop
uses 0.4/0.125/0.1 seconds. The native `tap` cycle is complex, with South as its
only direction, and is not seated. All stages and cycle metadata remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-065-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-065-review/index.html).

The summary samples the start, loop and end. The full gallery includes every
frame, with complete character views and enlarged face, hand and ankle details.
Its five pages and images total 157,457 bytes (about 154 KiB). Game-derived
images and packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 277 regular tests and the
release build passed; 347 local opt-ins remain ignored in the normal run.
133 local tests passed separately: the new material test and all 132 checks
retained from slice 064, including both Nora portrait tests and three local GML
tests. The package test failed on the unregistered tapping-end strip before
registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face, hand or ankle selections, and a seed spilling
onto hair.

All 17,168 combined variants validate. All 244 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 580
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Nora's expected total.

The original archive's native animator and NPC object pass 25 frame/palette
observations and 20 tapping-stage transitions with simulated engine services
and direct cycle selection. Checks cover palette binding, frame phase, wrapper
idempotence, standing status, cycle counters, completion and Spring portrait
synchronization. These strips have no linear-cycle completions or final-frame
holds. Natural scheduling, outfit/interaction dispatch and full-engine rendering
remain untested.

All five gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, metadata binding, full character view, enlarged detail
and all 15 summary bindings: 1,316,640 rendered pixels. The short review path
also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-065-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-065-`; author records use
`tmp/nora-spring-tapping-author-`. Fresh exports are in
`extracted/nora-spring-tapping-study`, reached through
`extracted/test-corpus/nora`. The fixed prior-art baseline is
`generated/slice-064-build/characters/nora`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Nora's Spring folder has three remaining strips: sweeping start, loop and end.
Summer, Autumn and Winter each have 29. The inventory is
`tmp/slice-065-remainder.json`. After artwork review, finish Spring with sweeping.
