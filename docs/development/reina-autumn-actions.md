# Reina Autumn blinking, sitting, eating and drinking

Eleven strips add Autumn blink East/South and sit, eat and drink North/South/East.
They contain 31 source frames and 12 native West mirrors, for 43 review cases.
Reina's world profile grows from 182 to 193 sources and produces 772 variants.
The choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user
approved this artwork for commit on 2026-09-27.

Sources are under `assets/animations/NPCs/Reina/Sprites/Autumn/`, named
`spr_npc_reina_autumn_{blink,sit,eat,drink}_{direction}.png`. The complete local
corpus is `extracted/reina-autumn-actions-study`; raw sidecars are recorded in
`tmp/reina-autumn-actions-author-metadata.json`. All eleven retain 80×80 frames,
Default atlas and `Middle`/54 offsets. Blink retains `[0.075,0.125,0.075]`, sit
retains its one-frame defaults, East/South eating retains
`[0.125,0.15,0.175,0.125,0.6]`, and North eating and all drinking strips retain
three frames with duration `1.0`. Native West mirrors East. Sources were
exported afresh from the read-only archive with strict pins; no previous hash
was refreshed or relaxed.

The 624 new component seeds select 1,288 skin pixels per target, covering faces,
necklines, the small exposed nape and hands. Hair, eyes, checkered shirt,
trousers, gold trim, boots, red mouth interiors and outlines keep their original
colors. All four existing world skin shades belong to exposed skin in these
new strips. No source colors, groups or mappings changed. The eight boot-sole
exclusions in the earlier Autumn walking strips remain intact.

Drink East frame two has a one-pixel hand highlight at strip coordinate
`[118,42]`; the gold collar at blink East `[40,41]` remains original. Every
actual frame was inspected in Vanilla and all four target palettes using
source material grids and ignored all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Blink East | 50, 60, 50 |
| Blink South | 58, 68, 58 |
| Sit East | 40 |
| Sit North | 13 |
| Sit South | 50 |
| Eat East | 37, 49, 37, 46, 38 |
| Eat North | 10, 7, 10 |
| Eat South | 51, 61, 56, 67, 50 |
| Drink East | 42, 49, 42 |
| Drink North | 10, 7, 10 |
| Drink South | 50, 62, 50 |

- [Five-choice summary](../../generated/reina-autumn-actions-preview/summary.png):
  blink South, sit East, eat South and drink East samples.
- [Complete Vanilla/Blue review](../../generated/reina-autumn-actions-preview/blue-review/index.html):
  all 43 cases and 86 views across six pages of at most eight cases.

The focused test uses literal skin/material landmarks and checks every new
source pixel against the inspected skin classes, per-frame counts, alpha,
metadata, common masks and all previous outputs.
`FOM_REINA_AUTUMN_ACTIONS_PRESETS` supports ignored control copies. The omission
control removes the hand highlight `[118,42]` from drink East. The spill control
adds a gold `#D2962F` alias in an ignored copy and selects blink East `[40,41]`.
Both controls failed at the intended material assertion after full generation;
neither changed production recipes. The production focused test and targeted
Clippy passed. Logs are `tmp/reina-autumn-actions-author-focused.log` and
`tmp/reina-autumn-actions-author-{omission,spill}.log`.

All 772 final variants passed exact recipe validation. All 182 earlier region
objects, groups and mappings remain unchanged, and all 1,820 previous original
and variant PNG/metadata files match the accepted Autumn pilot baseline byte
for byte. All 88 new variant PNG/metadata files match the visually inspected
candidate. The eleven raw sidecars match the independent archive inventory;
the canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-autumn-actions-author-compare.json`,
`tmp/reina-autumn-actions-author-final-vs-candidate.json` and
`tmp/reina-autumn-actions-author-frozen.json`.

The complete preview is 530,191 bytes, about 518 KiB. Crop `[29,26,20,29]`
contains every opaque pixel. Saved-image checks compared 7,421,100 exact
rendered pixels across all-target evidence, complete review sheets and 20
summary bindings, including source/palette identity and exact West reversal.
Chromium checked all eight HTML pages, every case, image decode and local link,
with no horizontal overflow. Reports are
`tmp/reina-autumn-actions-author-preview-check.log`,
`tmp/reina-autumn-actions-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-autumn-actions.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. General Autumn actions remain for later batches.
