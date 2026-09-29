# Balor's Spring actions, sleep and kiss

Five strips add action North/South/East, sleep East and kiss East: 26 source
frames plus 12 native West views. Balor now has 132 sources and 528 variants.
All 127 previous region objects, source pins, groups and mappings remain
unchanged. The user approved this artwork on 2026-09-29.

Fresh source files are under `assets/animations/NPCs/Balor/Sprites/Spring/`;
the full corpus is `extracted/balor-spring-standard-study`. Raw sidecars in
`tmp/balor-spring-standard-author-metadata.json` match the independent archive
read. Every frame is 80×80, Default atlas, Middle/54 origin. The three action
strips have seven frames at 0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds; kiss
has four at 0.15/0.15/0.8/0.15; sleep retains single-frame defaults. Strict pins
come from the read-only archive with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 471 new component seeds select 693 skin pixels per target using the
existing four world shades. Faces, ears, necks and hands change; hair, eyes,
scarf, cuffs, shirt, bag, trousers and boots stay original. All 74 trouser
pixels sharing portrait skin shade `#612026` remain excluded. The small North
hands and the kiss/sleep face and sleeve boundaries were inspected in every
actual frame across all four targets.

| Strip | Skin pixels per frame |
| --- | --- |
| Action East | 31, 30, 31, 30, 31, 31, 33 |
| Action North | 10, 2, 4, 2, 4, 9, 12 |
| Action South | 38, 36, 37, 36, 37, 38, 41 |
| Kiss East | 32, 34, 37, 38 |
| Sleep East | 29 |

- [Five-choice summary](../../generated/balor-spring-standard-preview/summary.png)
- [Complete Vanilla/Blue review](../../generated/balor-spring-standard-preview/blue-review/index.html)

The preview is about 459 KiB: 38 cases and 76 views across five review pages.
Crop `[31,26,20,29]` includes every opaque pixel. Saved-image checks compare
6,478,600 pixels across all-target sheets, summary and review images, including
source/palette bindings and exact West reversal. Chromium checks all seven
HTML pages, image decodes, case coverage, links and horizontal overflow.
Evidence is under `tmp/balor-spring-standard-author-*` and the gallery's
`coverage.json`.

The focused material test, both retained tests and targeted Clippy pass.
Literal landmarks, complete pixel expectations, per-frame counts, alpha,
metadata and common masks guard the new strips. Omitting the second action
East frame's hand component fails at strip `[128,42]`; selecting a trouser
component fails at `[119,46]`. Both controls fail after successful generation,
using `FOM_BALOR_SPRING_STANDARD_PRESETS` for ignored candidate definitions.

All 528 standalone variants pass exact recipe validation. All 1,270 previous
original/variant PNG and metadata files remain identical; all 40 new variant
files match the inspected candidate. Frozen input hashes and comparison
reports use the same ignored helper prefix.

See [shared integration](balor-valen-eiland-spring-standard.md) for native
animation and isolated installation checks. Static West views do not establish
natural sleep/kiss dispatch. Live gameplay was not exercised here. Artwork
remains ignored; this slice changes no runtime code.
