# Reina Winter chopping and polishing

Four strips finish all 35 PNG paths in Reina's Winter folder: chopping North
and polishing start, loop and end East. They contain 29 source frames: 20
chopping and nine polishing. Native polishing also renders nine West mirrors,
giving 38 review cases. Reina's profile grows from 242 to 246 sources, producing
984 variants. Choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia.
The user approved this artwork for commit on 2026-09-28.

Sources are under `assets/animations/NPCs/Reina/Sprites/Winter/`, named
`spr_npc_reina_specialanimation_winter_chop_north.png` and
`spr_npc_reina_specialanimation_winter_polish_{start,loop,end}_east.png`.
The fresh corpus is `extracted/reina-winter-finish-study`; raw sidecars and the
full-folder inventory are `tmp/reina-winter-finish-author-metadata.json` and
`tmp/reina-winter-finish-author-folder-inventory.json`. All frames are 80×80,
with Default atlas and numeric origin `[40.0,54.0]`. Chopping retains its
20-frame duration array, including the final 0.1/0.8-second frames. Polishing
start has three frames at 0.125 seconds; loop has four with
`[0.1,0.125,0.1,0.225]`; end has two with `[0.125,0.1]`. Chopping is a North-only
linear cycle with its sound hook; polishing uses a complex East pack with
native West flipping. Source pins remain strict; no earlier pin was refreshed
or relaxed.

The 192 new component seeds select 397 skin pixels per target. Winter sleeves
and scarf conceal more skin than the Autumn outfit. Chopping frames 3, 7, 11
and 15 have no visible skin and remain pixel-identical in every target. Their
former nape gap, local `[39,40]`, is scarf shade `#A27F4E`. The face and exposed
fingers recolor while the knife, green chopping effects, blue glass, white
cloth, jacket, trousers, boots, hair, eyes and outlines remain original. All
nine scarf pixels using portrait shade `#9E512F` stay excluded. No source
colors, groups or mappings changed. All 29 actual frames were visually
inspected in Vanilla and all four targets.

| Strip | Skin pixels per frame |
| --- | --- |
| Chop North | 9, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 9, 12 |
| Polishing start | 37, 35, 35 |
| Polishing loop | 35, 35, 35, 35 |
| Polishing end | 41, 54 |

- [Five-choice summary](../../generated/reina-winter-finish-preview/summary.png):
  first-frame chopping and polishing start/loop/end samples.
- [Complete Vanilla/Blue review](../../generated/reina-winter-finish-preview/blue-review/index.html):
  all 38 cases and 76 views across five pages of at most eight cases.

The focused corpus test uses literal material landmarks and full-pixel checks
for omissions, protected materials, per-frame counts, alpha, metadata, common
masks and previous outputs. It explicitly verifies the four frames without
visible skin and distinguishes the four-pixel hand highlight at `[34,47]` in
polishing start from sleeve `[34,46]`, glass and cloth.

`FOM_REINA_WINTER_FINISH_PRESETS` supports ignored omission and scarf-spill
controls. Both failed at the intended material assertion after successful
generation: the omitted hand at `[34,47]`, and scarf spill at `[39,43]`.
The spill uses an existing portrait mapping; neither control adds synthetic
aliases or changes production recipes. The production test and targeted
Clippy passed. Logs are `tmp/reina-winter-finish-author-focused.log` and
`tmp/reina-winter-finish-author-{omission,spill}.log`.

All 984 final variants passed exact recipe validation. All 242 earlier region
objects, colors, groups and mappings remain unchanged. All 2,420 previous
original and variant PNG/metadata files match the accepted Winter specials
baseline byte for byte; all 32 new variant files match the visually inspected
candidate. The four raw sidecars match the independent archive inventory, and
the canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-winter-finish-author-compare.json`,
`tmp/reina-winter-finish-author-final-vs-candidate.json` and
`tmp/reina-winter-finish-author-frozen.json`.

The complete preview is 527,399 bytes, about 515 KiB. Crop `[26,27,28,28]`
includes every opaque source pixel, including the chopping effects. Saved-image
checks compared 9,051,280 exact rendered pixels across all-target evidence,
complete review sheets and 20 summary bindings, including source/palette
identity and exact West reversal. Chromium checked all seven HTML pages, all
38 cases, image decodes and local links, with no horizontal overflow. Reports
are `tmp/reina-winter-finish-author-preview.log`,
`tmp/reina-winter-finish-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-winter-finish.md) for native selection
and isolated installation. Artwork remains ignored; this slice changes no
runtime behavior. Static images do not establish animation timing, scheduling
or state transitions. Other outfits remain for later batches.
