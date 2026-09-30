# Balor's Summer idle and walk animations

Six strips add Summer idle/walk North, South and East: 15 source frames plus
five native West views. Balor now has 150 sources and 600 variants. All 144
earlier region objects, source pins, groups and palette mappings remain
unchanged. The user approved this artwork on 2026-09-29.

Sources are `assets/animations/NPCs/Balor/Sprites/Summer/` files named
`spr_npc_balor_summer_{idle,walk}_{north,south,east}.png`. Fresh raw metadata in
`tmp/balor-summer-world-author-metadata.json` matches the independent archive
read: 80×80 frames, Default atlas, Middle/54 origin, single-frame idle defaults
and four walk frames at 0.15 seconds each. Strict pins come from the read-only
archive with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 398 new component seeds cover 543 skin pixels per target using existing
world roles. Faces, ears, necks, forearms, fingers and the bare ankles between
rolled trouser cuffs and shoes change. Hair, eyes, shirt, wrist accessories,
belt, trousers, cuffs and shoes stay original. All 99 belt/shoe pixels sharing
portrait skin shade `#612026` remain excluded. Every actual frame was
inspected in Vanilla and all four target palettes. No new roles or material
exceptions were needed.

| Strip | Skin pixels per frame |
| --- | --- |
| Idle North / South / East | 20 / 49 / 41 |
| Walk North | 20, 17, 20, 17 |
| Walk South | 49, 46, 49, 46 |
| Walk East | 41, 42, 41, 45 |

- [Five-choice summary](../../generated/balor-summer-world-preview/summary.png)
- [Complete Vanilla/Blue review](../../generated/balor-summer-world-preview/blue-review/index.html)

The complete preview is about 249 KiB: 20 cases and 40 views across three
review pages. Full-art crops fit within `[31,27,18,29]`. Saved-image checks
compare 3,068,520 pixels across all-target sheets, summary and review images,
including exact source/palette bindings and West reversal. Chromium checks
all five HTML pages, image decodes, local links, case coverage and horizontal
overflow. Evidence is under
`tmp/balor-summer-world-author-*` and the gallery's `coverage.json`.

Literal landmarks and complete pixel expectations guard the ankle/cuff/shoe
and forearm/accessory boundaries in every frame, alongside per-frame counts,
alpha, metadata and common masks. The focused corpus test, five retained
material tests and targeted Clippy pass. Removing the South idle ankle seed
`[37,51]` fails its skin assertion;
selecting a shoe component at `[37,52]` fails material preservation. Both
controls fail after successful generation, using `FOM_BALOR_SUMMER_WORLD_PRESETS`
for ignored definitions.

All 600 standalone variants pass exact recipe validation. All 1,440 earlier
original/variant PNG and metadata files remain identical; all 48 new variant
files match the inspected candidate. Frozen hashes and comparison reports
use the same ignored helper prefix. The canonical Blue recipe and the
portrait-only definitions remain unchanged.

See [shared integration](balor-summer-valen-heal-eiland-magnify.md) for native
animation and isolated installation checks. Live gameplay was not exercised
here. Generated artwork remains ignored; this slice changes no runtime code.
