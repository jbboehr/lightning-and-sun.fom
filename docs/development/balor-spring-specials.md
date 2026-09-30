# Balor's remaining Spring special animations

Six strips add coin flip, hair flip, jump, and gem-inspection start/loop/end.
They contain 59 source frames and 22 native West views of the East-facing
jump, completing all 34 sprites in Balor's Spring folder. Balor now has 144
sources and 576 variants. All 138 earlier region objects, source pins, groups
and palette mappings remain unchanged. The user approved this artwork on 2026-09-29.

Fresh files are under `assets/animations/NPCs/Balor/Sprites/Spring/`, named
`spr_npc_balor_specialanimation_spring_{coin_flip_south,hair_flip_south,jump_east,inspect_gem_start_south,inspect_gem_loop_south,inspect_gem_end_south}.png`.
The corpus is `extracted/balor-spring-specials-study`; raw sidecars in
`tmp/balor-spring-specials-author-metadata.json` match independent archive
reads. All frames are 80×80 on the Default atlas. Jump keeps its numeric
24/54 origin; the other strips use Middle/54. Original frame durations remain
exact. Strict pins come from the read-only archive with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 1,734 new component seeds select 2,419 skin pixels per target using the
four existing world shades. Faces, ears, necks and fingers change, including
the hand beside the gem and the translating jump frames. Coin colors
`#FFD942` and `#B77F3A`, gem blues/cyan, white glints, hair highlights, scarf,
clothing and boots remain original. All 261 trouser pixels sharing portrait
skin shade `#612026` stay excluded. Every actual frame was inspected in
Vanilla and all four target palettes; no new mapping or material exception
was needed.

| Strip | Frames | Skin pixels across strip |
| --- | ---: | ---: |
| Coin flip | 17 | 789 |
| Hair flip | 5 | 223 |
| Jump | 22 | 771 |
| Gem inspection start | 4 | 185 |
| Gem inspection loop | 7 | 266 |
| Gem inspection end | 4 | 185 |

- [Five-choice summary](../../generated/balor-spring-specials-preview/summary.png)
- [Complete Vanilla/Blue review](../../generated/balor-spring-specials-preview/blue-review/index.html)

The complete preview is about 1.35 MiB: 81 cases and 162 views across 11
review pages. Each strip has a full-art crop; the jump crop contains its
entire trajectory. Saved-image checks compare 37,717,300 pixels across
all-target sheets, summary and review images, including exact source/palette
bindings and West reversal. Chromium checks all 13 HTML pages, image decodes,
case coverage, local links and horizontal overflow. Evidence is under
`tmp/balor-spring-specials-author-*` and the gallery's `coverage.json`.

The focused material test, four retained tests and targeted Clippy pass.
Literal landmarks, complete pixel expectations, per-frame counts, alpha,
metadata and common masks guard skin and materials. Omitting the finger at
gem-loop strip `[49,43]` fails its skin assertion; selecting the trouser
component at coin-flip `[36,46]` fails material preservation. Both controls
fail after successful generation, using `FOM_BALOR_SPRING_SPECIALS_PRESETS`
for ignored definitions.

All 576 standalone variants pass exact recipe validation. All 1,380 earlier
original/variant PNG and metadata files remain identical; all 48 new variant
files match the inspected candidate. Frozen hashes and comparison reports
use the same ignored helper prefix.

See [shared integration](balor-valen-eiland-spring-specials.md) for native
animation and isolated installation checks. Live gameplay was not exercised
here. Generated artwork remains ignored; this slice changes no runtime code.
