# Nora Spring standing and walking

Following `4b2700b`, this slice starts Nora's world coverage with six Spring
standing and walking strips. The 15 source frames and five native West mirrors
give 20 review cases, approved by the user on 2026-10-03. Completed outputs use
`generated/review/` and `generated/build/`.

Nora has 38 sources; the 36-character collection has 4,269 sources and 17,076
variants. N retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices with
portrait/world synchronization. The collection now uses `nora-world-trial.json`;
her portrait-only profile, set, stylized recipe and tests remain unchanged.
No runtime code changes. Historical pixel tests retain their fixed expectations;
only the central Nora coverage total grows.

## Source and materials

The mount was absent. The source is the original archive backup at
`tmp/slice-060-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check for a newer installed
game version. All 32 portrait PNG pins match fresh exports. No hash override or
pin refresh was needed.

Nora's world skin differs from her portrait skin. Three source roles are appended
to the world profile, alongside its unchanged ten portrait roles:

| World source | Role | Debug Blue |
| --- | --- | --- |
| `#FCD9B3` | Light skin | `#9DB9D4` |
| `#F0B988` | Middle skin | `#7F9FBD` |
| `#D37A57` | Skin shadow | `#6687AD` |
| `#762E21` | Deep hand contours; existing portrait role | `#445F83` |

The new shades form a separate connected-color group. The deepest shade retains
its own existing group because it also outlines her hair. The other three
presets reuse their accepted light/middle/shadow colors for the new roles;
portrait skin and lip mappings remain exact.

The 288 component seeds select 534 skin pixels across all 15 frames: face,
ears, neck, hands and exposed ankles. Blonde hair, eye detail, the pale blouse,
blue overalls and shoes stay original. The first three world shades are skin
throughout these six pinned strips. The deep shade belongs to hair above row 44
and hands below it. Walking lowers some hair tips to row 40; those remain
excluded. This boundary is an authoring observation for these exact sources,
not a new runtime rule. The tracked profile contains explicit seeds and hashes.

Every frame was inspected in Vanilla/Blue contact sheets, with five-choice
walking sheets and labeled source grids around hair, face, cuffs, hands and
ankles. A portrait-color-only candidate incorrectly recolored hair and missed
most skin; a subsequent candidate caught the moving hair-tip boundary. Both
were corrected before promotion. The material test checks every output pixel,
literal source landmarks, per-frame selected counts, transparency and metadata.

Metadata retains 80×80 frames, the Default atlas and numeric 40/54 offsets.
Standing has one frame; walking has four 0.15-second frames. Both native cycles
are linear, with West mirroring East.

## Offline review

- [Five-choice sample summary](../../generated/slice-061-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-061-review/index.html).

The summary samples standing South and walking East/North. The full gallery
includes every frame and native West mirror, with complete character views and
enlarged face, hand and ankle details. Its ten pages and images total
400,926 bytes (about 392 KiB). Game-derived images and packages remain local and ignored
by Git.

## Verification

Formatting, release Clippy with warnings denied, all 273 regular tests and the
release build passed; 343 local opt-ins remain ignored in the normal run.
129 local tests passed separately: the new material test, both unchanged Nora
portrait tests and all 126 checks retained from slice 060, including three local
GML tests. The package test failed on the unregistered standing strip before
registration and passed afterward.

Three negative controls generated successfully and then failed the intended
material assertion: omitted face selections, omitted ankle selections and a
seed spilling onto a lowered hair tip. Removing a single redundant face seed
initially left the image unchanged; the final omission controls remove all
seeds for their selected areas, ensuring they exercise actual pixel omissions.

All 17,076 combined variants validate. All 152 standalone Nora variants pass
strict recipe validation and match the candidate and combined build. The 320
accepted Nora PNG/metadata files and 42,485 files for the other 35 characters
remain byte-identical. Existing runtime rows, choices and hotkeys remain exact.
Only Nora switches collection sets. All historical pixel tests remain unchanged;
only `tests/profile_coverage.rs` changes her expected total.

The original archive's native animator and NPC object pass 100 frame/palette
observations and 40 linear completions with simulated engine services and
direct cycle selection. Checks cover native mirroring, palette binding, frame
phase, wrapper idempotence, seated status, cycle counters, completion and Spring
portrait synchronization. Natural scheduling, pause/interaction dispatch and
full-engine rendering remain untested.

All ten gallery pages pass Chromium loading and overflow checks. Exact pixel
checks cover every frame, mirror, metadata binding, full character view,
enlarged detail and all 15 summary bindings: 3,862,560 rendered pixels.
The short review path also passes Chromium loading and local link resolution.

MOMI installation passed in the isolated `tmp/slice-061-playtest` lab, including
required compilation and installed-pixel/metadata checks. Both original backups
retain the source hash above; five registry/recipe inputs remained unchanged
through generation and installation. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other local opt-ins
were not rerun.

Evidence uses `tmp/slice-061-`; author records use `tmp/nora-world-author-`.
Fresh exports are in `extracted/nora-world-study`, reached through
`extracted/test-corpus/nora`. The fixed portrait baseline is
`generated/slice-060-build/characters/nora`. `evidence.json` summarizes results;
`scope.log` checks the twelve expected paths and unchanged historical tests.

## Remaining work

Nora's Spring folder has 26 remaining strips; Summer, Autumn and Winter each
have 29. The inventory is `tmp/slice-061-remainder.json`.
After artwork review, continue Spring with blinking, sitting, eating and
drinking before general actions, shocked reactions and special activities.
