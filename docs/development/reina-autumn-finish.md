# Reina Autumn chopping and polishing

Four strips finish all 35 PNG paths in Reina's Autumn folder: chopping North
and polishing start, loop and end East. They contain 29 source frames: 20
chopping and nine polishing. Native polishing also renders nine West mirrors,
giving 38 review cases. Reina's profile grows from 207 to 211 sources, producing
844 variants. Choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved this
artwork for commit on 2026-09-27.

Sources are under `assets/animations/NPCs/Reina/Sprites/Autumn/`, named
`spr_npc_reina_specialanimation_autumn_chop_north.png` and
`spr_npc_reina_specialanimation_autumn_polish_{start,loop,end}_east.png`.
The fresh corpus is `extracted/reina-autumn-finish-study`; raw sidecars and the
full-folder inventory are `tmp/reina-autumn-finish-author-metadata.json` and
`tmp/reina-autumn-finish-author-folder-inventory.json`. All frames are 80×80,
with Default atlas and numeric origin `[40.0,54.0]`. Chopping retains its
20-frame duration array, including the final 0.1/0.8-second frames. Polishing
start has three frames at 0.125 seconds; loop has four with
`[0.1,0.125,0.1,0.225]`; end has two with `[0.125,0.1]`. All four raw sidecars
match the independent archive read. Chopping is a North-only linear cycle with
its sound hook; polishing uses a complex East pack with native West flipping.
Source pins remain strict; no earlier pin was refreshed or relaxed.

The 241 new component seeds select 468 skin pixels per target. Autumn sleeves
and trousers conceal much of the skin visible in the Summer equivalents.
Chopping frames 3, 7, 11 and 15 each expose only one nape pixel; none has zero
visible skin. Face, neckline, fingers and nape recolor while the knife, green
chopping effects, blue glass, white cloth, checkered clothing, trousers, gold
trim, boots, hair, eyes and outlines remain original. No source colors, groups,
mappings or exclusions were added. All 29 actual frames were visually inspected
in Vanilla and all four targets. Earlier Autumn walking boot exclusions remain
intact.

| Strip | Skin pixels per frame |
| --- | --- |
| Chop North | 13, 2, 1, 5, 3, 3, 1, 5, 3, 3, 1, 5, 3, 3, 1, 5, 3, 3, 13, 17 |
| Polishing start | 42, 38, 38 |
| Polishing loop | 38, 38, 38, 38 |
| Polishing end | 45, 60 |

- [Five-choice summary](../../generated/reina-autumn-finish-preview/summary.png):
  first-frame chopping and polishing start/loop/end samples.
- [Complete Vanilla/Blue review](../../generated/reina-autumn-finish-preview/blue-review/index.html):
  all 38 cases and 76 views across five pages of at most eight cases.

The focused corpus test uses literal material landmarks and full-pixel checks
for omissions, protected materials, per-frame counts, alpha, metadata, common
masks and previous outputs. In particular, it preserves the one-pixel nape at
`[199,40]` in chopping frame three, and distinguishes a five-pixel exposed hand
highlight at `[35,46]` in polishing start from glass blue `#85CAEA` at `[44,45]`
in polishing loop. `FOM_REINA_AUTUMN_FINISH_PRESETS` supports ignored omission
and glass-spill controls. Both failed at the intended material assertion after
successful generation: the omitted hand at `[35,46]`, and glass spill at
`[44,44]`. The production test and targeted Clippy passed. Logs are
`tmp/reina-autumn-finish-author-focused.log` and
`tmp/reina-autumn-finish-author-{omission,spill}.log`.

All 844 final variants passed exact recipe validation. All 207 earlier region
objects, colors, groups and mappings remain unchanged. All 2,070 previous
original and variant PNG/metadata files match the accepted Autumn specials
baseline byte for byte; all 32 new variant files match the visually inspected
candidate. The canonical Debug Blue recipe matches the preset set. Evidence is
in `tmp/reina-autumn-finish-author-compare.json`,
`tmp/reina-autumn-finish-author-final-vs-candidate.json` and
`tmp/reina-autumn-finish-author-frozen.json`.

The complete preview is 531,289 bytes, about 519 KiB. Crop `[26,27,28,28]`
includes every opaque source pixel, including the chopping effects. Saved-image
checks compared 9,051,280 exact rendered pixels across all-target evidence,
complete review sheets and 20 summary bindings, including source/palette
identity and exact West reversal. Chromium checked all seven HTML pages, all
38 cases, image decodes and local links, with no horizontal overflow. Reports
are `tmp/reina-autumn-finish-author-preview.log`,
`tmp/reina-autumn-finish-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-autumn-finish.md) for native selection
and isolated installation. Artwork remains ignored; this slice changes no
runtime behavior. Static images do not establish animation timing, scheduling
or state transitions. Other outfits remain for later batches.
