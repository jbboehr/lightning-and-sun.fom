# Landen Spring blinking and seated actions

Following `1a93ed9`, this slice adds 11 Spring blinking, sitting, eating and
drinking strips to Landen's accepted 38 sources. The 31 source frames and 12
native West mirrors give 43 review cases. Artwork was approved by the user on 2026-10-03. Completed
outputs use `generated/review/` and `generated/build/`.

Landen has 49 sources; the 36-character collection has 4,193 sources and 16,772
variants. L retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only the new regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. The Debug Blue recipe description now describes its
reviewed overworld coverage. No runtime code changes. Historical pixel tests
retain their fixed expectations; only the central Landen coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-048-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 38 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 048](slice-048.md). The 576 new seeds select 1,432 skin pixels, including
the face, ears, neck, closed eyelids and hands. The same-color shoe components
remain unselected: 136 shoe pixels across these frames share the skin shades.
Gray hair, eye details, scarf, purple shirt, trousers and red apple stay original.

Seated East shoes sit higher than standing shoes, while the seated North view
has no visible shoes. South eating frame 2 lowers the hand over the left shoe;
the hand's skin remains included. The material test uses literal shoe coordinates
and a separate hand landmark, independent of profile seeds, plus exact checks
of every output pixel, per-frame selected counts, transparency and metadata.
Ryis changes 1,151 of the 1,432 selected pixels because two target shades already
match the source; selected pixels are counted separately from changed pixels.
All 31 source frames were inspected in grids and five-choice contact sheets.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Blinking has three frames with 0.075/0.125/0.075-second durations. Drinking and
North eating have three one-second frames. South/East eating has five frames
with 0.125/0.15/0.175/0.125/0.6-second durations. Sitting is static. West mirrors
East. Native eating/drinking cycles retain seated state and their final-frame
hold of 240–360 ticks.

## Offline review

- [Five-choice sample summary](../../generated/slice-049-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-049-review/index.html).

The summary samples blinking South, eating East and drinking North. The full
gallery includes every frame and native West mirror, with complete character
crops and enlarged details. Its 20 pages total 761,737 bytes (about 744 KiB).
Game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 261 regular tests and the
release build passed; 331 local opt-ins remain ignored in the normal run.
115 local tests passed separately, including the new material test, all 114
checks retained from slice 048 and the three local GML checks within that set.
The package test failed on the unregistered blinking strip before registration
and passed afterward. A missing skin seed and an extra shoe seed each generated
successfully, then failed the intended material assertion.

All 16,772 combined variants validate. All 196 standalone Landen variants pass
strict recipe validation and match the candidate and combined build. The 380
accepted Landen PNG/metadata files and 41,615 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Landen's expected total.

The original archive's native animator and NPC object pass 215 frame/palette
observations, 75 linear completions and 80 final-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native mirroring,
seated state, palette binding, frame phase, wrapper idempotence, cycle counters,
completion and portrait synchronization. Natural scheduling, outfit/interaction
dispatch and full-engine rendering remain untested.

All 20 gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character crop,
enlarged detail and all 15 summary bindings: 7,766,304 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-049-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-049-`; author records use
`tmp/landen-spring-actions-author-`. Fresh exports are in
`extracted/landen-spring-actions-study`, reached through
`extracted/test-corpus/landen`. The fixed prior-art baseline is
`generated/slice-048-build/characters/landen`. `evidence.json` summarizes results;
`scope.log` checks the ten expected paths and unchanged historical tests.

## Remaining work

Seven Landen Spring strips remain: three general actions, three shocked
start/loop/end strips and one hammer animation. Summer, Autumn and Winter each
have 21 remaining strips. The inventory is `tmp/slice-049-remainder.json`.
After artwork review, continue with the Spring general actions and shocked
reaction, then the hammer animation and seasonal folders.
