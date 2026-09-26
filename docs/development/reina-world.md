# Reina's first overworld batch

Six Spring idle/walk strips add North, South and East, with 15 source frames
and five native West mirrors. The new world profile, set and Debug Blue recipe
copy all 103 accepted portrait regions and extend them to 109 sources and 436
variants. The portrait-only definitions and tests remain unchanged. The five
choices are Vanilla, Debug Blue, Hayden, Ryis and Seridia. This slice awaits
user review.

Sources are under `assets/animations/NPCs/Reina/Sprites/Spring/`, named
`spr_npc_reina_spring_{idle,walk}_{north,south,east}.png`. The full local corpus
is `extracted/reina-world-study`; exported raw sidecars are recorded in
`tmp/reina-world-author-metadata.json`. All six match the independent archive
read and retain 80×80 frames, Default atlas and Middle/54 origin. Idle uses
native single-frame defaults; walk has four frames at 0.15 seconds each.
The read-only archive is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
No source hash was changed or relaxed.

World highlight `#B36844` already exists in the accepted portrait palette and
keeps its highlight role. Three additional shades, `#8E4538`, `#712922` and
`#571D1F`, map to the existing middle, shadow and darkest target shades. Each
gets a separate new color group. The eight prior mapping roles and original
color group remain unchanged, and every portrait output remains identical.

The 382 new component seeds select 743 skin pixels per target. Faces, ears,
necks, hands, midriff and legs change; hair, eyes, blouse, yellow jacket, belt,
shorts and shoes retain their materials. Fourteen isolated jacket-fold pixels
reuse `#B36844` and are excluded explicitly. The four-pixel strip above the
belt is exposed midriff: its interpretation was checked against the accepted
portrait's source and blue output. The blouse's pink neckline stays original.
Every actual frame was inspected in Vanilla and all four targets using source
material grids and ignored all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Idle East / North / South | 60 / 26 / 68 |
| Walk East | 60, 64, 60, 57 |
| Walk North | 26, 19, 26, 19 |
| Walk South | 68, 61, 68, 61 |

- [Five-choice summary](../../generated/reina-world-preview/summary.png): idle
  North/South and second-frame walking East/South samples.
- [Complete Vanilla/Blue review](../../generated/reina-world-preview/blue-review/index.html):
  all 20 cases and 40 views across three pages of at most eight cases.

The full preview directory is about 384 KiB. Final sheets and previews use the
actual standalone bundle. Crop `[31,27,18,29]` contains every opaque pixel.
Saved-image checks cover 3,233,790 exact pixels across all-target sheets,
review sheets and 20 summary bindings, including source/palette identity and
exact West reversal. Chromium decoded every image across all five HTML
pages, checked every case and local link, and found no horizontal overflow.
Evidence is in `tmp/reina-world-author-preview-check.log`,
`tmp/reina-world-author-preview-browser-check.json`, and the review's
`coverage.json`.

The final focused local corpus test and targeted Clippy passed before
candidate promotion. The test checks all prior region definitions and color
roles, 72 literal skin/material landmarks, every source pixel against the
reviewed material classes and 14 explicit jacket exclusions, exact per-frame
counts, alpha, metadata, common masks and all prior output bytes. Removing a
midriff component failed at East `[39,45]`; selecting a jacket fold failed at
`[37,43]`. Both intended failures occurred in the material assertion after
successful generation. `FOM_REINA_WORLD_PRESETS` supports these ignored
negative-control copies. Logs are `tmp/reina-world-author-final-test.log` and
`tmp/reina-world-author-{omission,spill}-test.log`.

All 436 final variants passed exact recipe validation. All 1,030 accepted
portrait original/variant PNG and metadata files remain byte-identical, and
all 48 new variant PNG/metadata files match the inspected candidate exactly.
The canonical Debug Blue recipe also matches every source role in the preset
set. Reports are `tmp/reina-world-author-compare.json` and
`tmp/reina-world-author-final-vs-candidate.json`; frozen data and test hashes
are in `tmp/reina-world-author-frozen-inputs.sha256`.

See [shared integration](reina-juniper-march-world.md) for native animation
selection and isolated installation.
Static images do not establish live animation timing, NPC scheduling or state
transitions. Artwork stays ignored; this character slice changes no runtime
code. Further Reina actions and outfits remain for later batches.

The user approved this artwork for commit on 2026-09-25.
