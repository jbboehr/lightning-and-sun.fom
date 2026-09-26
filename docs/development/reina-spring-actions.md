# Reina Spring blink, seated, eat and drink

Eleven Spring strips add blink East/South and sit, eat and drink North/South/East.
They contain 31 actual source frames and 12 native West mirrors, making 43 review
cases. The existing world profile grows from 109 to 120 sources and produces
480 variants. The choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia.
This artwork awaits user review.

Sources are under `assets/animations/NPCs/Reina/Sprites/Spring/`, named
`spr_npc_reina_spring_{blink,sit,eat,drink}_{direction}.png`. The full local
corpus is `extracted/reina-spring-actions-study`; raw sidecars are recorded in
`tmp/reina-spring-actions-author-metadata.json`. All eleven match the independent
archive read and retain 80×80 frames, Default atlas and Middle/54 origin.
Blink retains `[0.075, 0.125, 0.075]`; eating East/South retains five frames with
`[0.125, 0.15, 0.175, 0.125, 0.6]`. Eating North and drinking retain their three
frames and `1.0` duration; sitting retains single-frame defaults. The read-only
archive is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
No old source pin was refreshed or relaxed.

The existing four world shades cover all new skin; no colors, groups or mappings
were added or changed. The 706 new component seeds select 1,419 skin pixels per
target, including necks, raised hands, exposed midriff and legs. Forty isolated
jacket-fold pixels also use skin highlight `#B36844` and stay original. The
second drinking-East frame puts a highlighted thumb directly beside one such
fold: strip coordinates `[118,42]` change while `[117,43]` stay original.
Eating-mouth red `#C83E37` and dark red `#410808` remain unchanged, as do hair,
eyes, blouse trim, yellow jacket, belt, shorts, shoes and outlines. Every actual
frame was inspected in Vanilla and all four targets using source material grids
and ignored all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Blink East | 64, 74, 64 |
| Blink South | 72, 82, 72 |
| Sit East / North / South | 44 / 14 / 50 |
| Drink East | 43, 52, 43 |
| Drink North | 14, 9, 14 |
| Drink South | 52, 64, 52 |
| Eat East | 38, 49, 40, 46, 40 |
| Eat North | 14, 9, 14 |
| Eat South | 52, 61, 58, 69, 50 |

- [Five-choice summary](../../generated/reina-spring-actions-preview/summary.png):
  blinking South, seated East, eating South and drinking East samples.
- [Complete Vanilla/Blue review](../../generated/reina-spring-actions-preview/blue-review/index.html):
  all 43 cases and 86 views across six pages of at most eight cases.

The preview directory is about 519 KiB. Previews bind to the final standalone
bundle. Crop `[29,26,20,29]` includes every opaque pixel. Saved-image checks
cover 7,421,100 exact pixels across all-target sheets, complete review sheets
and 20 summary bindings, including source/palette identity and exact West
reversal. Chromium decoded all images across eight HTML pages, checked every
case and local link, and found no horizontal overflow. Evidence is in
`tmp/reina-spring-actions-author-preview-check.log`,
`tmp/reina-spring-actions-author-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed before exact candidate
promotion. The test checks literal skin/material landmarks, every source pixel
against independently reviewed material classes and the 40 jacket exclusions,
exact per-frame counts, alpha, metadata, common masks and all previous outputs.
A full-corpus omission control failed at the second drinking-East frame's thumb
`[118,42]`; a spill control failed at its jacket fold `[117,43]`. Both reached
the intended material assertion after successful generation.
`FOM_REINA_SPRING_ACTIONS_PRESETS` supports these ignored control copies.
Logs are `tmp/reina-spring-actions-author-focused.log` and
`tmp/reina-spring-actions-author-{omission,spill}-test.log`.

All 480 final variants passed exact recipe validation. All 109 previous region
objects and mappings remain unchanged, and all 1,090 previous original/variant
PNG and metadata files match the accepted first-world baseline byte for byte.
All 88 new variant PNG/metadata files exactly match the inspected candidate.
Final sidecars also match the independent raw inventory. The canonical Debug
Blue recipe matches the preset set. Reports are
`tmp/reina-spring-actions-author-compare.json` and
`tmp/reina-spring-actions-author-final-vs-candidate.json`; data and test hashes
are in `tmp/reina-spring-actions-author-frozen.json`.

See [shared integration](reina-juniper-march-actions.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. Further actions and outfits remain for later batches.
