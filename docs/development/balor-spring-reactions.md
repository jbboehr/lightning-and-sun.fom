# Balor's Spring shocked and seated-reading cycles

Six South-facing strips add shocked start/loop/end and seated-reading
start/loop/end: 13 source frames, with no mirrored views. Balor now has 138
sources and 552 variants. All 132 earlier region objects, source pins, groups
and palette mappings remain unchanged. The user approved this artwork on 2026-09-29.

Fresh files are under `assets/animations/NPCs/Balor/Sprites/Spring/`, named
`spr_npc_balor_spring_shocked_{start,loop,end}_south.png` and
`spr_npc_balor_specialanimation_spring_read_sit_{start,loop,end}_south.png`.
The local corpus is `extracted/balor-spring-reactions-study`; raw sidecars in
`tmp/balor-spring-reactions-author-metadata.json` match the independent archive
read. All frames are 80×80, Default atlas, Middle/54 origin. Shocked phases
retain single-frame defaults. Reading start/end each have three frames at
0.1 seconds; the four-frame loop uses 3.0/0.1/3.0/0.1 seconds. Strict pins come
from the read-only archive with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 305 new component seeds select 429 skin pixels per target using the
existing world shades. Faces, ears, necks and exposed fingers change. The
book's four cover/page colors (`#606C76`, `#98A2A6`, `#C9AF9C`, `#F6E4D7`)
remain original, including the pale edges beside hands and faces. Hair, eyes,
mouth interiors, scarf and clothing also stay original. All 26 trouser pixels
sharing portrait skin shade `#612026` remain excluded. Every actual frame was
inspected in Vanilla and all four target palettes.

| Strip | Skin pixels per frame |
| --- | --- |
| Shocked start / loop / end | 47 / 49 / 47 |
| Reading start | 31, 27, 29 |
| Reading loop | 23, 33, 23, 33 |
| Reading end | 29, 27, 31 |

- [Five-choice summary](../../generated/balor-spring-reactions-preview/summary.png)
- [Complete Vanilla/Blue review](../../generated/balor-spring-reactions-preview/blue-review/index.html)

The complete preview is about 216 KiB: 13 cases and 26 views across two review
pages. Crop `[29,26,22,29]` contains all artwork. Saved-image checks compare
2,899,710 pixels across all-target sheets, summary and review images, including
exact source/palette bindings. Chromium checks all four HTML pages, image
decodes, case coverage, local links and horizontal overflow. Evidence is under
`tmp/balor-spring-reactions-author-*` and the gallery's `coverage.json`.

The focused material test, three retained tests and targeted Clippy pass.
Literal landmarks, complete pixel expectations, per-frame counts, alpha,
metadata and common masks guard skin and materials. Omitting a finger beside
the raised book fails at reading-start strip `[115,46]`; selecting a trouser
component fails at shocked-loop `[36,45]`. Both controls fail after successful
generation, using `FOM_BALOR_SPRING_REACTIONS_PRESETS` for ignored definitions.

All 552 standalone variants pass exact recipe validation. All 1,320 earlier
original/variant PNG and metadata files remain identical; all 48 new variant
files match the inspected candidate. Frozen hashes and comparison reports
use the same ignored helper prefix.

See [shared integration](balor-valen-eiland-spring-reactions.md) for native
animation and isolated installation checks. Live gameplay was not exercised
here. Generated artwork remains ignored; this slice changes no runtime code.
