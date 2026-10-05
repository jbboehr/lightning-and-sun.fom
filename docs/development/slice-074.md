# Nora Autumn blinking, sitting, eating and drinking

Following `2588938`, this slice adds eleven Autumn strips to Nora's accepted 99
sources: blinking East/South and sitting, eating and drinking North/South/East.
The 31 source frames and twelve native West mirrors give 43 review cases,
approved by the user on 2026-10-04. Completed outputs use `generated/review/` and
`generated/build/`.

Nora has 110 sources; the 36-character collection has 4,341 sources and 17,364
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-073-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 99 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 061](slice-061.md). Its 520 seeds select 1,005 skin pixels across 31
frames: face, ears, neck, eyelids, raised hands and exposed ankles. Blonde hair,
eye detail, blue shirt, pale apron, purple trousers, shoes and mouth interiors
stay original. Every frame was inspected in Vanilla/Blue contact sheets, with
five-choice East-eating views and source grids around the raised hands and hair.

The first three world shades are skin throughout these exact strips. The
deeper shared shade `#762E21` belongs to hair through row 41 and to exposed
hand contours from row 42. This includes `[46,42]` in East-eating frame 2,
`[45,42]` in North-eating/drinking frames 1 and 3, and `[35,43]` in
South-eating frame 4 (one-based frames, frame-local coordinates). Hair tips
at row 41 remain original as the head moves. The pale apron, purple trousers,
and mouth-interior shades `#410808` and `#9E2626` remain unchanged. Tracked
masks contain component seeds and hashes, not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Blinks have three frames at 0.075/0.125/0.075 seconds. Drinks and North eating
have three frames at one second each. East/South eating has five frames at
0.125/0.15/0.175/0.125/0.6 seconds. Sitting strips have one frame and no duration
field. All four native cycles are linear; sitting, eating and drinking are
seated, and eating/drinking retain the native 240–360 final-frame hold range.
Blinking has East/South directions with South as its default; the other cycles
have North/South/East directions. West mirrors East. Metadata and native
pause/speaking settings remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-074-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-074-review/index.html).

The summary samples blinking South, eating East and drinking North. The full
gallery includes every source frame and native West mirror, with complete
character views and enlarged face, hand and ankle details. Its twenty pages
and images total 776,779 bytes (about 759 KiB). Game-derived images and packages
remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 286 regular tests and the
release build passed; 356 local opt-ins remain ignored in the normal run.
142 local tests passed separately: the new material test and all 141 checks
retained from slice 073, including both Nora portrait tests and three local GML
tests. The package test failed on the unregistered Autumn blinking strip before
registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, a missing raised-hand contour,
a seed spilling onto a moving hair tip and omitted ankle selections.

All 17,364 combined variants validate. All 440 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 990
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Nora's expected total.

The original archive's native animator and NPC object pass 215 frame/palette
observations, 75 linear-cycle completions and 80 final-frame hold checks with
simulated engine services and direct cycle selection. Checks cover native
mirroring, palette binding, frame phase, wrapper idempotence, seated status,
cycle counters, completion and Autumn portrait synchronization. These strips
have no complex transitions. Natural scheduling, speaking interruptions,
seasonal rollover, outfit/interaction dispatch and full-engine rendering remain
untested.

All twenty gallery pages pass Chromium loading and overflow checks. Exact
pixel checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 7,766,304 rendered pixels. The short
review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-074-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-074-`; author records use
`tmp/nora-autumn-actions-author-`. Fresh exports are in
`extracted/nora-autumn-actions-study`, reached through `extracted/test-corpus/nora`.
The fixed prior-art baseline is `generated/slice-073-build/characters/nora`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Nora's Spring and Summer sprite folders remain complete. Autumn has 12 remaining
strips; Winter has 29. The inventory is `tmp/slice-074-remainder.json`.
After artwork review, continue with her three Autumn general-action strips.
