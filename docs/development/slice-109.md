# Josephine Spring standing and walking

Following `bea8755`, this slice starts Josephine's world coverage with six Spring
standing and walking strips. Fifteen source frames and five native West mirrors
give twenty review cases. The user approved the artwork on 2026-10-07. Outputs use
`generated/review/` and `generated/build/`.

Josephine has 38 sources; the 36-character collection has 4,499 sources and
17,996 variants. P retains Vanilla, Debug Blue, Hayden, Ryis and Seridia choices
with portrait/world synchronization. Only Josephine switches collection sets,
from her portrait set to `josephine-world-trial.json`. Her portrait-only profile,
set, stylized recipe and tests remain unchanged. No runtime code changes.
Historical pixel tests retain fixed expectations; only the central Josephine
coverage total grows.

## Source and materials

The mount was empty. Source is the original archive backup at
`tmp/slice-108-playtest/.mistria-palette/previous.zip`, with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
It matches the last approved archive; this does not check a newer installed game
version. All 32 prior portrait PNG pins match fresh exports. No hash override
or pin refresh was needed.

Josephine's world sprites share their light skin shade with her portraits but
use three different darker shades. The world profile appends these roles while
preserving all nine portrait roles and their target mappings:

| World source | Role | Debug Blue |
| --- | --- | --- |
| `#9B643A` | Light skin; existing portrait role | `#9DB9D4` |
| `#7E442F` | Middle skin; new world role | `#7F9FBD` |
| `#713222` | Skin shadow; new world role | `#6687AD` |
| `#57271D` | Deep skin edge; new world role | `#445F83` |

The three new shades form a separate connected-color group. Other presets reuse
accepted middle, shadow and deep portrait targets. The 341 component seeds
select 623 skin pixels across all fifteen frames: face, ears, neck, hands and
exposed ankles. These four shades belong exclusively to skin in this batch;
there is no footwear cutoff. Brown hair, eyes, pink earrings, orange shawl,
pink skirt and shoes stay original. Profiles contain explicit seeds and source
hashes, without new runtime rules.

Every source frame was inspected in five-choice sheets, with source grids for
face shadows, hands, ankles and nearby materials. The material test checks
every output pixel with an independent shade oracle,
42 fixed landmarks and per-frame selected counts, plus transparency and metadata.
Walking landmarks distinguish exposed ankles from adjacent orange shoes.

Metadata retains 80×80 frames, the Default atlas and numeric offsets 40/54.
Standing has one frame; walking has four 0.15-second frames. Both native cycles
are linear, with West mirroring East.

## Offline review

- [Five-choice sample summary](../../generated/slice-109-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-109-review/index.html).

The summary samples standing South and walking East/North. The full gallery
includes every source frame and native West mirror, with Vanilla left and Debug
Blue right, full character views and enlarged face, hand and ankle details.
The gallery totals 407,816 bytes (about 398 KiB). Game-derived images and
packages remain local and ignored by Git.

## Verification

Formatting, release Clippy with warnings denied, all 321 regular tests and the
release build passed. The regular run left 391 opt-in tests ignored; 179 local
tests passed separately, including Josephine's unchanged portrait test, the new
material test, retained character checks and three GML tests. The package test
failed on an unregistered standing strip before registration, then passed.

Four negative controls generated successfully and failed the intended pixel
assertion: missing face, hand, ankle and forehead selections. All 152 standalone
variants passed strict recipe validation and matched the candidate and combined
build. All 17,996 combined variants validated. The 320 accepted Josephine
PNG/metadata files and 44,785 files for other characters remained byte-identical.
Previous runtime rows, choices and hotkeys stayed exact. Only Josephine changes
collection sets; historical pixel tests remain unchanged.

The archive's native animator and NPC object passed 100 frame/palette
observations and 40 linear completions, with simulated engine services and direct
cycle selection. Checks cover native mirroring, palette binding, frame phase,
wrapper idempotence, cycle counters, completion and Spring portrait synchronization.
Natural scheduling, pause/interaction dispatch and full-engine rendering remain
untested.

All ten gallery pages passed Chromium image/link and overflow checks. Exact
pixel checks covered every frame, mirror, metadata binding, full character view,
enlarged detail and all fifteen summary bindings: 3,862,560 rendered pixels.
The short review/build links resolve to this slice; Chromium also loaded the
review through its short link.

MOMI built and installed the combined package in the isolated
`tmp/slice-109-playtest` lab, including installed-pixel and metadata checks.
Five registry/recipe inputs stayed frozen, and both original archive backups
retained the source hash above. No preview helper was installed or desktop
launcher changed. Live gameplay, an uninstall roundtrip and other opt-in tests
were not rerun. The user approved the offline artwork on 2026-10-07.

Evidence uses `tmp/slice-109-`; author records use `tmp/josephine-world-author-`.
Fresh exports are in `extracted/josephine-world-study`, reached through
`extracted/test-corpus/josephine`. The fixed portrait baseline is
`generated/slice-108-build/characters/josephine`. `evidence.json` summarizes the
results; `scope.log` checks the twelve expected paths and unchanged historical
tests.

## Remaining work

The archive inventory leaves 30 Spring strips and 33 each for Summer, Autumn
and Winter. Continue Spring with blinking, sitting, eating and drinking before
general actions and special activities. Inventory is recorded in
`tmp/slice-109-remainder.json`. Natural gameplay dispatch and
live rendering remain pending; this coverage describes the pinned archive.
