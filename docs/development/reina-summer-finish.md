# Reina Summer chopping and polishing

Four strips add chopping North and polishing start, loop and end East, completing
Reina's Summer folder. They contain 29 source frames: 20 chopping and nine
polishing. Native polishing also uses nine West mirrors, making 38 review cases.
Reina's world profile grows from 172 to 176 sources and produces 704 variants.
The choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user
approved this artwork for commit on 2026-09-27.

Sources are under `assets/animations/NPCs/Reina/Sprites/Summer/`, named
`spr_npc_reina_specialanimation_summer_chop_north.png` and
`spr_npc_reina_specialanimation_summer_polish_{start,loop,end}_east.png`.
The complete local corpus is `extracted/reina-summer-finish-study`; raw sidecars
are recorded in `tmp/reina-summer-finish-author-metadata.json`. All four match
the independent archive read and retain 80×80 frames, Default atlas and numeric
origin `[40.0,54.0]`. Chopping retains its full 20-frame duration array, including
the final 0.1/0.8-second frames. Polishing start has three frames at 0.125 seconds;
loop has four with `[0.1,0.125,0.1,0.225]`; end has two with `[0.125,0.1]`.
The native chopping cycle is linear and North-only, with its sound hook;
polishing is a complex East cycle whose artwork is mirrored for West. The
read-only archive is `tmp/fields-of-mistria/assets.zip`; all source pins remain
strict and no earlier pin was refreshed or relaxed.

The existing four world skin shades cover all exposed skin in these strips.
No source colors, groups, target mappings or material exclusions were added.
The 786 new component seeds select 1,352 skin pixels per target, covering faces,
necklines, exposed back, arms, hands, legs and toes. Knife colors, green chopping
effects, blue glass, polishing cloth, green fabric, cream/tan blouse, gold
button, hair, eyes, accessories, sandal straps and outlines retain their original
colors. In particular, the seven-pixel arm/hand highlight beginning at `[36,45]`
in polishing start changes; glass blue `#85CAEA` at `[44,45]` in the polishing
loop stays original. Every actual frame was inspected in Vanilla and all four
targets using source material grids and ignored all-target sheets, including all
20 chopping frames.

| Strip | Skin pixels per frame |
| --- | --- |
| Chop North | 49, 41, 34, 36, 34, 39, 34, 36, 34, 39, 34, 36, 34, 39, 34, 36, 34, 39, 49, 55 |
| Polishing start | 66, 63, 63 |
| Polishing loop | 58, 60, 57, 58 |
| Polishing end | 76, 85 |

- [Five-choice summary](../../generated/reina-summer-finish-preview/summary.png):
  first-frame chopping and polishing start/loop/end samples.
- [Complete Vanilla/Blue review](../../generated/reina-summer-finish-preview/blue-review/index.html):
  all 38 cases and 76 views across five pages of at most eight cases.

The complete preview directory is about 516 KiB. Previews bind to the final
standalone bundle. Crop `[26,27,28,28]` includes every opaque pixel, including
the chopping effects. Saved-image checks cover 9,051,280 exact pixels across
all-target sheets, review sheets and 20 summary bindings, including source and
palette identity and exact West reversal. Chromium decoded all images across
seven HTML pages, checked every case and local link, and found no horizontal
overflow. Evidence is in `tmp/reina-summer-finish-author-preview-check.log`,
`tmp/reina-summer-finish-author-preview-browser-check.json`, and the gallery's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test uses literal
skin/material landmarks and checks every source pixel against independently
reviewed skin classes, exact per-frame counts, alpha, metadata, common masks and
all previous outputs. A full-corpus omission control fails at the exposed
arm/hand component `[36,45]` in polishing start. A spill control deliberately
aliases glass blue `#85CAEA` to skin in an ignored copy and selects `[44,45]` in
the polishing loop; the material check rejects the resulting glass recoloring
at `[44,44]`. Both controls reach the intended material assertion after
successful generation and leave production recipes unchanged.
`FOM_REINA_SUMMER_FINISH_PRESETS` supports these ignored copies. Logs are
`tmp/reina-summer-finish-author-focused.log` and
`tmp/reina-summer-finish-author-{omission,spill}.log`.

All 704 final variants passed exact recipe validation. All 172 earlier region
objects, groups and mappings remain unchanged; all 1,720 previous original and
variant PNG/metadata files match the accepted Summer specials baseline byte for
byte. All 32 new variant PNG/metadata files exactly match the visually inspected
candidate. Final sidecars match the independent raw inventory, and the canonical
Debug Blue recipe matches the preset set. Reports are
`tmp/reina-summer-finish-author-compare.json` and
`tmp/reina-summer-finish-author-final-vs-candidate.json`; frozen hashes are in
`tmp/reina-summer-finish-author-frozen.json`.

See [shared integration](reina-juniper-summer-finish.md) for native selection and
isolated installation. Static images do not establish animation timing,
scheduling or state transitions. Artwork remains ignored; this slice changes
no runtime behavior. Further outfits remain for later batches.
