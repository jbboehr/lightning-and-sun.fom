# Balor's Summer everyday actions

Eleven strips add blinking East/South and sitting, eating and drinking
North/South/East: 31 source frames plus 12 native West views. Balor now has
161 sources and 644 variants. All 150 earlier region objects, source pins,
groups and palette mappings remain unchanged. The user approved the offline artwork on 2026-09-29.

Sources are `assets/animations/NPCs/Balor/Sprites/Summer/` files named
`spr_npc_balor_summer_{blink,sit,eat,drink}_{direction}.png`. Fresh raw metadata
in `tmp/balor-summer-actions-author-metadata.json` matches independent archive
reads: 80×80 frames, Default atlas and Middle/54 origin. Blink durations remain
0.075/0.125/0.075 seconds; South/East eating retains five frames with
0.125/0.15/0.175/0.125/0.6 seconds. North eating and all drinking retain three
frames at 1.0 seconds; sitting retains single-frame defaults. Strict pins
come from the read-only archive with SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 727 new component seeds cover 1,018 skin pixels per target using existing
world roles. Faces, eyelids, ears, necks, forearms, fingers and visible ankles
change. Hair, eyes, shirt/cuffs, wrist accessories, belt, trousers and shoes
stay original. All 171 material pixels sharing portrait shade `#612026` and
all 40 red mouth-interior pixels (`#410808`, `#9E2626`) remain excluded. Every
actual frame was inspected in Vanilla and all four target palettes. No new
roles or material exceptions were needed.

| Strip | Skin pixels per frame |
| --- | --- |
| Blink East | 43, 47, 43 |
| Blink South | 51, 55, 51 |
| Sit North / South / East | 9 / 38 / 34 |
| Eat North | 12, 7, 12 |
| Eat South | 40, 42, 35, 46, 38 |
| Eat East | 30, 32, 29, 32, 35 |
| Drink North | 12, 7, 12 |
| Drink South | 40, 45, 40 |
| Drink East | 31, 39, 31 |

- [Five-choice summary](../../generated/balor-summer-actions-preview/summary.png)
- [Complete Vanilla/Blue review](../../generated/balor-summer-actions-preview/blue-review/index.html)

The complete preview is about 438 KiB: 43 cases and 86 views across six review
pages. Full-art crops fit within `[31,27,18,28]`. Saved-image checks compare
4,970,625 pixels across all-target sheets, summary and review images, including
exact source/palette bindings and West reversal. Chromium checks all eight
HTML pages, image decodes, local links, case coverage and horizontal overflow.
Evidence is under `tmp/balor-summer-actions-author-*` and `coverage.json`.
These images review the character strips; they do not show scene attachments
or animation timing.

The focused material test, six retained tests and targeted Clippy pass.
Literal landmarks and complete pixel expectations cover every frame, moving
hands, wrist details, mouth interiors and clothing, alongside counts, alpha,
metadata and common
masks. Removing the raised hand seed at drink-East strip `[119,40]` fails its
skin assertion; selecting the belt component at `[119,46]` fails material
preservation. Both controls fail after successful generation, using
`FOM_BALOR_SUMMER_ACTIONS_PRESETS` for ignored definitions.

All 644 standalone variants pass exact recipe validation. All 1,500 earlier
original/variant PNG and metadata files remain identical; all 88 new variant
files match the inspected candidate. Frozen hashes and comparison reports
use the same ignored helper prefix. Canonical Blue and portrait-only
definitions remain unchanged.

See [shared integration](balor-valen-summer-eiland-spring-finish.md) for native
animation and isolated installation checks. Live gameplay was not exercised
here. Generated artwork remains ignored; this slice changes no runtime code.
