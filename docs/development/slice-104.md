# Holt Winter blinking and seated actions

Following `e965e77`, this slice adds Holt's eleven Winter blinking, sitting,
eating and drinking strips to his accepted 122 sources. The 31 source frames
and twelve native West mirrors give 43 review cases. The user approved the artwork
on 2026-10-07. Completed outputs use `generated/review/` and `generated/build/`.

Holt has 133 sources; the 36-character collection has 4,483 sources and 17,932
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. Only eleven regions and registry entries are
added. Source colors, color groups, target mappings, collection sets and prior
regions stay unchanged. No runtime code changes. Historical pixel tests retain
their fixed expectations; only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-103-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 122 prior PNG pins match fresh exports. No hash override or
pin refresh was needed.

The batch reuses the four reviewed world skin shades from
[slice 085](slice-085.md). Its 538 component seeds select 1,167 skin pixels:
face, ears, neck, eyelids, nose edge and moving hands. Every source frame was
inspected in five-choice contact sheets, with source grids around lowered hands,
sleeves, boots, mouth interiors and shared shades.

Unlike Autumn's seated strips, the four skin shades belong exclusively to skin
here; Winter's footwear uses different colors. The lowered hand in South eating
frame 2 reaches row 49, beside a boot using `#D37B52` at local (38,50).
The Autumn hand contours using `#762E21` are absent in Winter: the East blinking
sleeve edge at local (45,45) instead uses `#622A10` and stays original. Winter's
`#762E21` belongs to hair and moustache. The orange shirt also uses the existing
mapped `#B45732` at local (32,44) in North sitting; that clothing stays original.
The mask follows Winter's source pixels and does not copy Autumn's exceptions.
Removing the unnecessary Autumn bounds from the local author helper produced an
identical candidate profile.

Ginger hair, moustache, eyes, orange plaid shirt, scarf, apron, trousers, boots
and red mouth interiors remain original. These observations describe the pinned
source; the tracked profile contains component seeds and hashes, not new runtime
rules. The material test independently checks reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Blinking has three frames at 0.075/0.125/0.075 seconds. Drinking and North eating
have three frames at one second each; East/South eating have five frames at
0.125/0.15/0.175/0.125/0.6 seconds. Sitting has one frame without explicit duration.
Native cycles retain East-to-West mirroring, standing blinking, seated
sitting/eating/drinking and eating/drinking final-frame holds of 240–360 ticks.

## Offline review

- [Five-choice sample summary](../../generated/slice-104-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-104-review/index.html).

The summary samples frame 2 of South blinking, East eating and North drinking.
The full gallery includes all 31 source frames and twelve native West mirrors,
with Vanilla left and Debug Blue right, full character views and enlarged face,
hand, clothing and boot details. Game-derived images and packages remain local
and ignored by Git.
The twenty pages and images total 780,751 bytes (about 762 KiB).

## Verification

Formatting, release Clippy with warnings denied, all 316 regular tests and the
release build passed; 386 local opt-ins remain ignored in the normal run.
173 local tests passed separately: the new material test and all 172 checks
retained from slice 103, including Holt's portrait and earlier world tests and
three local GML tests. The package test failed on an unregistered Winter blinking
strip before registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face selections, the omitted lowered hand beside a
boot, a seed spilling onto the orange shirt and a seed spilling onto a moustache
edge.

All twenty gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, native West mirror, metadata binding, full character
view, enlarged detail and all 15 summary bindings: 7,766,304 rendered pixels.

All 17,932 combined variants validate. All 532 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 1,220
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
All historical pixel tests remain unchanged; only `tests/profile_coverage.rs`
changes Holt's expected total.

The original archive's native animator and NPC object pass 215 frame/palette
observations, 75 linear completions and 80 final-frame hold checks with simulated
engine services and direct cycle selection. Checks cover native mirroring,
palette binding, frame phase, wrapper idempotence, standing/seated status, cycle
counters, completion and Winter portrait synchronization. These cycles have no
complex stage transitions. Natural scheduling, pause/interaction dispatch and
full-engine rendering remain untested.

MOMI installation passed in the isolated `tmp/slice-104-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. The short review path also passes Chromium
loading and local link resolution. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-104-`; author records use
`tmp/holt-winter-actions-author-`. Fresh exports are in
`extracted/holt-winter-actions-study`, reached through `extracted/test-corpus/holt`.
The fixed prior-art baseline is `generated/slice-103-build/characters/holt`.
`evidence.json` summarizes results; `scope.log` checks the nine expected paths
and unchanged historical tests.

## Remaining work

Holt's Spring, Summer and Autumn sprite folders are complete. Winter now covers
17 of 27 strips; ten remain, recorded in `tmp/slice-104-remainder.json`.
Next: Winter general actions in North, South and East.
