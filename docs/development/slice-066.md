# Nora Spring sweeping

Following `006b7f8`, this slice adds three Spring sweeping strips to Nora's
accepted 61 sources: start, loop and end South. The 17 source frames give 17
review cases, approved by the user on 2026-10-03. This native sequence has no West
mirrors. It completes all 32 strips in Nora's Spring sprite folder. Completed
outputs use `generated/review/` and `generated/build/`.

Nora has 64 sources; the 36-character collection has 4,295 sources and 17,180
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-065-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 61 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 061](slice-061.md). Its 374 seeds select 816 skin pixels across 17 frames:
face, ears, neck, moving hands and exposed ankles. Blonde hair, eye detail, pale
blouse, blue overalls, shoes, broom and dust stay original. Every frame was
inspected in Vanilla/Blue contact sheets, with five-choice late-loop views and
source grids around the hands, broom and hair.

The first three world shades are skin throughout these exact strips. The
deeper shared shade `#762E21` belongs to hair through row 40 and to eight hand
contour pixels at rows 47–48. Those hand pixels are included. The broom's brown
and gold shades and the dust's gray are distinct from the selected skin.
Tracked masks contain component seeds and hashes, not a new runtime rule.

The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Start and end each have one frame and no duration field. The 15-frame loop
uses 0.1 seconds per frame except frames 6, 12 and 15, which use 0.2, 0.4 and
0.8 seconds respectively. The native `sweep` cycle is complex, South-only and
not seated; it switches to `idle` when paused for speaking. All stages and
cycle metadata remain unchanged.

## Offline review

- [Five-choice sample summary](../../generated/slice-066-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-066-review/index.html).

The summary samples the start and loop frames 4 and 15. The full gallery
includes every frame, with complete character views and enlarged face, hand
and ankle details. The full-view crop is taller to retain the broom's lowest
positions; every opaque pixel fits. Its eight pages and images total 353,019
bytes (about 345 KiB). Game-derived images and packages remain local and
ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 278 regular tests and the
release build passed; 348 local opt-ins remain ignored in the normal run.
134 local tests passed separately: the new material test and all 133 checks
retained from slice 065, including both Nora portrait tests and three local GML
tests. The package test failed on the unregistered sweeping-end strip before
registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, a missing dark hand contour,
a seed spilling onto hair and omitted ankle selections.

All 17,180 combined variants validate. All 256 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 610
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Nora's expected total.

The original archive's native animator and NPC object pass 85 frame/palette
observations and 20 sweeping-stage transitions with simulated engine services
and direct cycle selection. Checks cover palette binding, frame phase, wrapper
idempotence, standing status, cycle counters, completion and Spring portrait
synchronization. These strips have no linear-cycle completions or final-frame
holds. Natural scheduling, speaking interruptions, outfit/interaction dispatch
and full-engine rendering remain untested.

All eight gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, metadata binding, full character view, enlarged detail
and all 15 summary bindings: 3,558,048 rendered pixels. The short review path
also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-066-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-066-`; author records use
`tmp/nora-spring-sweeping-author-`. Fresh exports are in
`extracted/nora-spring-sweeping-study`, reached through
`extracted/test-corpus/nora`. The fixed prior-art baseline is
`generated/slice-065-build/characters/nora`. `evidence.json` summarizes results;
`scope.log` checks the nine expected paths and unchanged historical tests.

## Remaining work

Nora's Spring sprite folder is complete. Summer, Autumn and Winter each have
29 remaining strips. The inventory is `tmp/slice-066-remainder.json`. After
artwork review, start Summer with standing and walking.
