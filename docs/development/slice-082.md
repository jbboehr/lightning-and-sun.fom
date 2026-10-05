# Nora Winter seated writing

Following `0fe3a3c`, this slice adds three Winter writing strips to Nora's
accepted 142 sources: seated start, loop and end East. Eight source frames and
eight native West mirrors give 16 review cases, approved by the user on 2026-10-05.
Completed outputs use `generated/review/` and `generated/build/`.

Nora has 145 sources; the 36-character collection has 4,376 sources and 17,504
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-081-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 142 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 061](slice-061.md). Its 97 seeds select 234 skin pixels across the eight
frames: face, ears, neck and moving hands. Blonde hair, eye detail, blue scarf,
pale coat, purple skirt, brown boots, pink earrings and writing tools stay
original. The boots cover the ankles. Every frame was inspected in Vanilla/Blue
contact sheets, with a five-choice writing-loop sheet and source grids around
the hands, board, pen and hair.

The first three world shades are skin throughout these three pinned strips.
The deepest shade, `#762E21`, belongs to hair except for two lowered-hand
contour pixels at `[35,47]`: start frame 1 and end frame 2 (one-based frames).
Those pixels are included; all hair remains unchanged. The board's brown and
the pen's pale and yellow shades are distinct from the selected skin. These
observations describe the exact sources; tracked masks contain component seeds
and hashes, not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Start has two frames at 0.125 seconds each; the four-frame loop uses
0.1/0.125/0.1/0.3 seconds; end uses 0.125/0.1 seconds. The native `write_sit`
cycle is complex and seated, with East as its default direction. West mirrors
East. All stages and cycle metadata remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-082-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-082-review/index.html).

The summary samples the start, loop and end. The full gallery includes every
frame and native West mirror, with Vanilla left and Debug Blue right, complete
character views and enlarged face, hand and boot details. Its eight pages
and images total 327,591 bytes (about 320 KiB). Game-derived images and packages
remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 294 regular tests and the
release build passed; 364 local opt-ins remain ignored in the normal run.
150 local tests passed separately: the new material test and all 149 checks
retained from slice 081, including both Nora portrait tests and three local GML
tests. The package test failed on the unregistered Winter writing-end strip
before registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, a missing lowered-hand contour,
a seed spilling onto hair and omitted hand selections beside the board.

All 17,504 combined variants validate. All 580 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 1,420
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Nora's expected total.

The original archive's native animator and NPC object pass 80 frame/palette
observations and 40 writing-stage transitions with simulated engine services
and direct cycle selection. Checks cover native mirroring, palette binding,
frame phase, wrapper idempotence, seated status, cycle counters, completion and
Winter portrait synchronization. These strips have no linear-cycle completions
or final-frame holds. Natural scheduling, speaking interruptions, seasonal
rollover, outfit/interaction dispatch and full-engine rendering remain untested.

All eight gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 3,183,648 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-082-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-082-`; author records use
`tmp/nora-winter-writing-author-`. Fresh exports are in
`extracted/nora-winter-writing-study`, reached through
`extracted/test-corpus/nora`. The fixed prior-art baseline is
`generated/slice-081-build/characters/nora`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Nora's Spring, Summer and Autumn sprite folders remain complete. Winter has six
remaining strips: tapping and sweeping, each with start/loop/end stages.
The inventory is `tmp/slice-082-remainder.json`.
Continue with Winter tapping.
