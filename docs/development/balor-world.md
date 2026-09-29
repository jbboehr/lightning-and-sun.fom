# Balor's first overworld batch

Six Spring idle/walk strips add North, South and East: 15 source frames plus
five native West mirrors. The new world definitions preserve all 110 accepted
portrait regions and extend the corpus to 116 sources and 464 variants.
Portrait-only definitions remain unchanged. The user approved this artwork on 2026-09-29.

Sources are `assets/animations/NPCs/Balor/Sprites/Spring/` files named
`spr_npc_balor_spring_{idle,walk}_{north,south,east}.png`. Fresh raw metadata in
`tmp/balor-world-author-metadata.json` matches the independent archive read:
80×80 frames, Default atlas, Middle/54 origin, single-frame idle defaults and
four walk frames at 0.15 seconds each. Read-only `tmp/fields-of-mistria/assets.zip`
has SHA-256 `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All source pins remain strict.

World shades `#FCD9B3`, `#F0B988`, `#D37A57` and `#672115` receive separate
new groups and map to the existing highlight, middle, shadow and darkest
targets. All seven portrait roles and four original groups remain unchanged.
The 300 new component seeds cover 425 skin pixels per target: faces, ears,
necks and hands. Hair, eyes, scarf, shirt, cuffs, bag, trousers and boots stay
original. In particular, 38 trouser pixels reuse portrait skin shade `#612026`
and are excluded. Every actual frame was inspected in all four target palettes.

| Strip | Skin pixels per frame |
| --- | --- |
| Idle East / North / South | 33 / 12 / 41 |
| Walk East | 33, 36, 33, 33 |
| Walk North | 12, 11, 12, 9 |
| Walk South | 41, 39, 41, 39 |

- [Five-choice summary](../../generated/balor-world-preview/summary.png)
- [Complete Vanilla/Blue review](../../generated/balor-world-preview/blue-review/index.html)

The complete preview is about 267 KiB: 20 cases and 40 views across three
review pages. Crop `[31,27,18,29]` contains all artwork. Saved-image verification
checks 3,233,790 pixels across all-target sheets, summary and review images,
including exact source/palette bindings and West reversal. Chromium decoded
every image, checked all links/cases across five HTML pages and found no
horizontal overflow. Evidence is under `tmp/balor-world-author-*`, with complete
coverage in the gallery's `coverage.json`.

The focused local test and targeted Clippy pass. Literal landmarks, full-pixel
material expectations, per-frame counts, alpha, metadata and common masks
cover the new strips. Removing the South idle neck seed fails at `[39,40]`;
selecting a trouser component fails at `[36,46]`. Both negative controls fail
after successful generation, using `FOM_BALOR_WORLD_PRESETS` to select ignored
candidate definitions.

All 464 standalone variants pass exact recipe validation. All 1,100 accepted
portrait original/variant PNG and metadata files are byte-identical; all 48
new variant files match the visually inspected candidate. The canonical Blue
recipe matches the preset mappings. Frozen data/test hashes and comparison
reports are retained under the same ignored helper prefix.

See [shared integration](balor-valen-eiland-world.md) for native animation and
isolated installation checks. Live gameplay was not exercised here. Generated
art remains ignored; this batch changes no runtime code.
