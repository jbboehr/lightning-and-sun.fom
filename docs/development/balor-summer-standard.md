# Balor's Summer general actions, sleep and kiss

Five strips add general actions North/South/East, sleep East and kiss East.
They contain 26 source frames plus 12 native West views: 38 review cases.
Balor now has 166 sources and 664 variants, with 22/29 Summer strips covered.
The user approved the offline artwork on 2026-09-29.

Fresh sources are under `assets/animations/NPCs/Balor/Sprites/Summer/`, named
`spr_npc_balor_summer_{action,sleep,kiss}_{direction}.png`. Raw metadata matches
independent archive reads: 80×80 frames, Default atlas, Middle/54 origin.
Actions have seven frames at 0.1/0.25/0.25/0.25/0.25/0.1/0.4 seconds; kiss has
four at 0.15/0.15/0.8/0.15; sleep uses single-frame defaults. Strict PNG pins
come from archive SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 624 new component seeds select 886 skin pixels per target. All 26 actual
frames were inspected in Vanilla and all four targets. Existing world roles
cover faces, kiss jaw shading, hands, forearms and ankles. Hair, eyes, shirt,
cuffs, wrist accessories, belt, trousers and shoes stay original. All 196
material pixels using portrait shade `#612026` remain excluded. No new source
roles or material exceptions were needed.

- [Five-choice summary](../../generated/balor-summer-standard-preview/summary.png)
- [Every-frame Vanilla/Debug Blue review](../../generated/balor-summer-standard-preview/blue-review/index.html)

The complete preview is 430,567 bytes. It includes all 38 cases across five
detail pages, with source/palette bindings, complete crops and native West
mirroring checked. Saved-image checks compare 5,482,550 exact pixels; Chromium
checks all seven HTML pages, image decodes, local links and overflow.

The focused material test checks 210 literal landmarks, every pixel in every
target, per-frame counts, alpha, metadata and prior output preservation.
An omitted kiss-jaw seed at strip `[203,39]` and a selected belt seed at
`[199,45]` both fail their intended material assertions after successful
generation. All seven retained tests change only corpus paths and totals.

All 664 standalone variants pass strict recipe validation. All 1,610 prior
original/variant PNG and metadata files remain identical. The full combined
comparison verifies the standalone output against the final collection.
Evidence: `tmp/balor-summer-standard-author-{focused,final-output,omission-red,spill-red}.log`
and `tmp/bve-summer-expansion-balor-retained.log`.

See [shared integration](balor-valen-eiland-summer-expansion.md) for complete
verification and native/installation checks. Static images do not establish
live timing, interaction or scheduling. Game-derived artwork remains ignored.
