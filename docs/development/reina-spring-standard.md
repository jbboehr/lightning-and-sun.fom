# Reina Spring general action, sleep and kiss

Five Spring strips add general action North/South/East, sleep East and kiss East.
They contain 26 actual source frames and 12 native West mirrors, making 38 review
cases. The existing world profile grows from 120 to 125 sources and produces
500 variants. The choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia.
The user approved this artwork for commit on 2026-09-25.

Sources are under `assets/animations/NPCs/Reina/Sprites/Spring/`, named
`spr_npc_reina_spring_action_{north,south,east}.png`,
`spr_npc_reina_spring_sleep_east.png` and `spr_npc_reina_spring_kiss_east.png`.
The full local corpus is `extracted/reina-spring-standard-study`; raw sidecars
are recorded in `tmp/reina-spring-standard-author-metadata.json`. All five match
the independent archive read and retain 80×80 frames, Default atlas and
Middle/54 origin. General actions retain seven frames with durations
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`; kiss retains four frames with
`[0.15, 0.15, 0.8, 0.15]`; sleep retains single-frame defaults.
The read-only archive is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
No old source pin was refreshed or relaxed.

The existing four world shades cover all new skin; no colors, groups or mappings
were added or changed. The 650 new component seeds select 1,254 skin pixels per
target, including exposed necks, gesturing hands, midriff and legs. Twenty-eight
isolated jacket-fold and hem pixels reuse skin highlight `#B36844` and remain
original, including the jacket fold in the sleeping pose and the small jacket
hem on the viewer-right of the leaning kiss. Hair, eyes, blouse trim, yellow
jacket, belt, shorts, shoes and outlines retain their materials. Every actual
frame was inspected in Vanilla and all four targets using source material grids
and ignored all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| General action East | 47, 53, 50, 53, 50, 47, 60 |
| General action North | 25, 17, 18, 17, 18, 25, 26 |
| General action South | 60, 62, 61, 62, 61, 60, 68 |
| Kiss East | 54, 62, 71, 66 |
| Sleep East | 61 |

- [Five-choice summary](../../generated/reina-spring-standard-preview/summary.png):
  second-frame general actions South/East, sleeping and first-frame kiss samples.
- [Complete Vanilla/Blue review](../../generated/reina-spring-standard-preview/blue-review/index.html):
  all 38 cases and 76 views across five pages of at most eight cases.

The preview directory is about 481 KiB. Previews bind to the final standalone
bundle. Crop `[30,26,21,29]` includes every opaque pixel. Saved-image checks
cover 6,802,530 exact pixels across all-target sheets, complete review sheets
and 20 summary bindings, including source/palette identity and exact West
reversal. Chromium decoded all images across seven HTML pages, checked every
case and local link, and found no horizontal overflow. Evidence is in
`tmp/reina-spring-standard-author-preview-check.log`,
`tmp/reina-spring-standard-author-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed before exact candidate
promotion. The test checks literal skin/material landmarks, every source pixel
against independently reviewed material classes and the 28 jacket exclusions,
exact per-frame counts, alpha, metadata, common masks and all previous outputs.
A full-corpus omission control failed at the second general-action-East frame's
hand `[127,43]`; a spill control failed at its jacket fold `[120,41]`. Both reached
the intended material assertion after successful generation.
`FOM_REINA_SPRING_STANDARD_PRESETS` supports these ignored control copies.
Logs are `tmp/reina-spring-standard-author-focused.log` and
`tmp/reina-spring-standard-author-{omission,spill}-test.log`.

All 500 final variants passed exact recipe validation. All 120 previous region
objects and mappings remain unchanged, and all 1,200 previous original/variant
PNG and metadata files match the accepted Spring actions baseline byte for byte.
All 40 new variant PNG/metadata files exactly match the inspected candidate.
Final sidecars also match the independent raw inventory. The canonical Debug
Blue recipe matches the preset set. Reports are
`tmp/reina-spring-standard-author-compare.json` and
`tmp/reina-spring-standard-author-final-vs-candidate.json`; data and test hashes
are in `tmp/reina-spring-standard-author-frozen.json`.

See [shared integration](reina-juniper-march-standard.md) for native selection
and isolated installation. Static images do not establish animation timing,
scheduling or state transitions. Artwork remains ignored; this slice changes
no runtime behavior. Further actions and outfits remain for later batches.
