# Balor's Spring everyday actions

Eleven strips add blink East/South and sit/eat/drink North/South/East. They
contain 31 source frames and 12 native West mirrors. Balor now has 127 sources
and 508 variants; all 116 previous region objects, source pins, color groups
and palette mappings are unchanged. The user approved this artwork on 2026-09-29.

Fresh source files are under `assets/animations/NPCs/Balor/Sprites/Spring/`;
the local corpus is `extracted/balor-spring-actions-study`. Raw metadata in
`tmp/balor-spring-actions-author-metadata.json` matches the independent archive
read: every frame is 80×80, Default atlas, Middle/54 origin. Blink uses three
frames at 0.075/0.125/0.075 seconds. Eat East/South uses five frames at
0.125/0.15/0.175/0.125/0.6; the other eat/drink strips use three frames with
duration 1.0. Sit retains single-frame defaults. Strict source hashes come
from the read-only archive with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 613 new component seeds cover 890 skin pixels per target using the four
existing world shades. Face, ears, neck and moving hands change. Hair, eyes,
scarf, cuffs, shirt, bag, trousers, boots and mouth interiors stay original.
All 72 trouser pixels sharing portrait skin shade `#612026` remain excluded.
Every actual frame was inspected in Vanilla and all four target palettes.

| Strip | Skin pixels per frame |
| --- | --- |
| Blink East / South | 35, 39, 35 / 43, 47, 43 |
| Drink East / North / South | 28, 36, 28 / 7, 5, 7 / 37, 42, 37 |
| Eat East | 27, 29, 26, 29, 32 |
| Eat North | 7, 5, 7 |
| Eat South | 37, 40, 36, 46, 33 |
| Sit East / North / South | 28 / 6 / 33 |

- [Five-choice summary](../../generated/balor-spring-actions-preview/summary.png)
- [Complete Vanilla/Blue review](../../generated/balor-spring-actions-preview/blue-review/index.html)

The full preview is about 475 KiB: 43 cases and 86 views across six review
pages. Crop `[31,27,18,28]` contains all artwork. Saved-image checks compare
6,448,680 pixels across all-target sheets, summary and review images, including
source/palette bindings and exact West reversal. Chromium checks all eight
HTML pages, image decodes, case coverage and local links, with no horizontal
overflow. Evidence is under `tmp/balor-spring-actions-author-*` and in the
gallery's `coverage.json`.

The focused corpus test, retained pilot test and targeted Clippy pass. Literal
skin/material landmarks, complete pixel expectations, per-frame counts, alpha,
metadata and common masks guard the new artwork. An omitted drink-hand seed
fails at East `[40,42]`; selecting a trouser component fails at sit East
`[38,45]`. Both controls fail after successful generation, using
`FOM_BALOR_SPRING_ACTIONS_PRESETS` to select ignored candidate definitions.

All 508 final variants pass exact recipe validation. All 1,160 previous
original/variant PNG and metadata files remain identical, and all 88 new
variant files match the inspected candidate. Frozen hashes and comparison
reports are retained under the ignored helper prefix.

See [shared integration](balor-valen-eiland-spring-actions.md) for native
animation and isolated installation checks. Live gameplay was not exercised
here. Generated artwork remains ignored; this slice changes no runtime code.
