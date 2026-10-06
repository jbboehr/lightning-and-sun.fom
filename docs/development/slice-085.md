# Holt Spring standing and walking

Following `964430d`, this slice starts Holt's world coverage with six Spring
standing and walking strips. The 15 source frames and five native West mirrors
give 20 review cases, approved by the user on 2026-10-05. Completed outputs use
`generated/review/` and `generated/build/`.

Holt has 38 sources; the 36-character collection has 4,388 sources and 17,552
variants. H retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. The collection now uses `holt-world-trial.json`;
his portrait-only profile, set, stylized recipe and tests remain unchanged.
No runtime code changes. Historical pixel tests retain their fixed expectations;
only the central Holt coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-084-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 32 portrait PNG pins match fresh exports. No hash override or
pin refresh was needed.

Holt's world skin differs from his portrait skin. Three source roles are appended
to the world profile, alongside its unchanged nine portrait roles:

| World source | Role | Debug Blue |
| --- | --- | --- |
| `#F0CC8E` | Light skin; new role | `#9DB9D4` |
| `#CD9067` | Middle skin; existing portrait role | `#7F9FBD` |
| `#96523A` | Skin shadow; new role | `#6687AD` |
| `#AD6945` | Side-view nose edge; new role | `#6687AD` |

The new shades form a separate connected-color group. Other presets reuse their
accepted light and shadow targets for the new roles; all portrait mappings stay
exact. The original deep shade `#762E21` remains unchanged in these world strips:
it belongs to hair, moustache, scarf detail and boots.

The 286 component seeds select 536 skin pixels across all 15 frames: face,
ears, neck and moving hands. Ginger hair and moustache, eye detail, orange scarf,
green shirt, blue apron and boots stay original. The first three world shades
also occur in the boots. Skin is above row 49 in these six pinned sources;
the boots are separated from it. This boundary describes these exact sources,
not a new runtime rule. The tracked profile contains explicit seeds and hashes.
The extra nose shade appears once in each of the five East-facing source frames.

Every frame was inspected in Vanilla/Blue and five-choice contact sheets,
with labeled source grids around the face, nose, moustache, hands and boots.
The material test independently checks the reviewed source shades, literal
landmarks, every output pixel, per-frame selected counts, transparency and
metadata. Selected pixels are counted separately from changed pixels.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Standing has one frame; walking has four 0.15-second frames. Both native cycles
are linear, with West mirroring East.

## Offline review

- [Five-choice sample summary](../../generated/slice-085-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-085-review/index.html).

The summary samples standing South and walking East/North. The full gallery
includes every frame and native West mirror, with Vanilla left and Debug Blue
right, complete character views and enlarged face, hand and boot details.
Its ten pages and images total 400,409 bytes (about 391 KiB). Game-derived images and packages
remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 297 regular tests and the
release build passed; 367 local opt-ins remain ignored in the normal run.
154 local tests passed separately: the new material test, the unchanged Holt
portrait test and all 152 checks retained from slice 084, including three local
GML tests. The package test failed on the unregistered standing strip before
registration and passed afterward.

Four negative controls generated successfully and then failed the intended
material assertion: omitted face, hand or nose-edge selections and a seed
spilling onto a boot.

All 17,552 combined variants validate. All 152 standalone Holt variants pass
strict recipe validation and match the candidate and combined build. The 320
accepted Holt PNG/metadata files and 43,675 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
Only Holt switches collection sets. All historical pixel tests remain unchanged;
only `tests/profile_coverage.rs` changes his expected total.

The original archive's native animator and NPC object pass 100 frame/palette
observations and 40 linear completions with simulated engine services and
direct cycle selection. Checks cover native mirroring, palette binding, frame
phase, wrapper idempotence, standing status, cycle counters, completion and Spring
portrait synchronization. Natural scheduling, pause/interaction dispatch and
full-engine rendering remain untested.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 3,862,560 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-085-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-085-`; author records use `tmp/holt-world-author-`.
Fresh exports are in `extracted/holt-world-study`, reached through
`extracted/test-corpus/holt`. The fixed portrait baseline is
`generated/slice-084-build/characters/holt`. `evidence.json` summarizes results;
`scope.log` checks the twelve expected paths and unchanged historical tests.

## Remaining work

Holt's Spring folder has 24 remaining strips; Summer, Autumn and Winter each
have 27. The inventory is `tmp/slice-085-remainder.json`.
Continue Spring with blinking, sitting, eating and
drinking before general actions, reactions and special activities.
