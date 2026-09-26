# Reina Summer blinking, sitting, eating and drinking

Eleven strips add Summer blink East/South and sit, eat and drink North/South/East.
They contain 31 source frames and 12 native West mirrors, for 43 review cases.
Reina's world profile grows from 147 to 158 sources and produces 632 variants.
The choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. This artwork
awaits user review.

Sources are under `assets/animations/NPCs/Reina/Sprites/Summer/`, named
`spr_npc_reina_summer_{blink,sit,eat,drink}_{direction}.png`. The complete local
corpus is `extracted/reina-summer-actions-study`; raw sidecars are recorded in
`tmp/reina-summer-actions-author-metadata.json`. All eleven match the independent
archive read and retain 80×80 frames, Default atlas and numeric origin
`[40.0,54.0]`. Blink retains `[0.075,0.125,0.075]`, sit retains its one-frame
defaults, East/South eating retains `[0.125,0.15,0.175,0.125,0.6]`, and North
eating and all drinking strips retain three frames and duration `1.0`. Native
West uses the East artwork mirrored. The read-only archive is
`tmp/fields-of-mistria/assets.zip`; all source pins remain strict and no earlier
pin was refreshed or relaxed.

All four existing world skin shades belong to exposed skin in these strips;
no source colors, groups or target mappings were added or changed. The 984 new
component seeds select 1,889 skin pixels per target, covering faces, necklines,
shoulders, arms, hands, exposed back, seated legs and toes. Hair, eyes, cream and
tan blouse pixels, green fabric and straps, gold buttons, pink accessories,
sandal straps, red mouth interiors and outlines retain their original colors.
Drink-East frame two includes an isolated exposed hand highlight at strip
coordinate `[118,42]`; blink-East's dark green strap at `[38,41]` stays original.
Every actual source frame was inspected in Vanilla and all four target palettes
using source material grids and ignored all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Blink East | 79, 89, 79 |
| Blink South | 90, 100, 90 |
| Sit East | 56 |
| Sit North | 33 |
| Sit South | 66 |
| Eat East | 53, 64, 60, 60, 54 |
| Eat North | 34, 23, 34 |
| Eat South | 66, 73, 69, 79, 66 |
| Drink East | 56, 66, 56 |
| Drink North | 34, 23, 34 |
| Drink South | 63, 77, 63 |

- [Five-choice summary](../../generated/reina-summer-actions-preview/summary.png):
  blink South, sit East, eat South and drink East samples.
- [Complete Vanilla/Blue review](../../generated/reina-summer-actions-preview/blue-review/index.html):
  all 43 cases and 86 views across six pages of at most eight cases.

The complete preview directory is about 517 KiB. Previews bind to the final
standalone bundle. Crop `[29,26,20,29]` includes every opaque source pixel.
Saved-image checks cover 7,421,100 exact pixels across all-target sheets,
complete review sheets and the 20 summary bindings, including source/palette
identity and exact West reversal. Chromium decoded all images across eight HTML
pages, checked every case and local link, and found no horizontal overflow.
Evidence is in `tmp/reina-summer-actions-author-preview-check.log`,
`tmp/reina-summer-actions-author-preview-browser-check.json`, and the gallery's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test uses literal
skin/material landmarks and checks every source pixel against independently
reviewed skin classes, exact per-frame counts, alpha, metadata, common masks and
all previous outputs. A full-corpus omission control failed at the exposed hand
component `[118,42]` in drink East. Because no clothing in these strips reuses a
skin shade, the spill control deliberately adds an ignored green-fabric alias
and seed; it fails at blink-East's strap `[38,41]`. Both controls reached the
intended material assertion after successful generation; neither altered the
production recipes. `FOM_REINA_SUMMER_ACTIONS_PRESETS` supports these ignored
control copies. Logs are `tmp/reina-summer-actions-author-focused.log` and
`tmp/reina-summer-actions-author-{omission,spill}.log`.

All 632 final variants passed exact recipe validation. All 147 earlier region
objects, groups and mappings remain unchanged, and all 1,470 previous
original/variant PNG and metadata files match the accepted Summer pilot baseline
byte for byte. All 88 new variant PNG/metadata files exactly match the visually
inspected candidate. Final sidecars also match the independent raw inventory.
The canonical Debug Blue recipe matches the preset set. Reports are
`tmp/reina-summer-actions-author-compare.json` and
`tmp/reina-summer-actions-author-final-vs-candidate.json`; data and test hashes are
in `tmp/reina-summer-actions-author-frozen.json`.

See [shared integration](reina-juniper-march-summer-actions.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. General Summer actions remain for later batches.
