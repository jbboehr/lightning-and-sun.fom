# Reina Spring shocked and seated reading

Six strips add the start, loop and end of the shocked and seated-reading cycles.
They contain 13 actual South-facing source frames. Both cycles are South-only;
there are no West mirrors. The existing world profile grows from 125 to 131
sources and produces 524 variants. The choices remain Vanilla, Debug Blue,
Hayden, Ryis and Seridia. The user approved this artwork for commit on 2026-09-25.

Sources are under `assets/animations/NPCs/Reina/Sprites/Spring/`, named
`spr_npc_reina_spring_shocked_{start,loop,end}_south.png` and
`spr_npc_reina_specialanimation_spring_read_sit_{start,loop,end}_south.png`.
The full local corpus is `extracted/reina-spring-reactions-study`; raw sidecars
are recorded in `tmp/reina-spring-reactions-author-metadata.json`. All six match
the independent archive read and retain 80×80 frames, Default atlas and
Middle/54 origin. Each shocked strip retains single-frame defaults. Reading
start/end retain three frames at 0.1 seconds each; its loop retains four frames
with `[3.0, 0.1, 3.0, 0.1]` durations.
The read-only archive is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
No old source pin was refreshed or relaxed.

The existing four world shades cover all new skin; no colors, groups or mappings
were added or changed. The 328 new component seeds select 696 skin pixels per
target, including raised hands, exposed necks, midriff and legs. Twenty-two
isolated jacket-fold pixels reuse skin highlight `#B36844` and remain original.
These include the raised sleeves in the shocked loop and the jacket beside the
book in the reading loop. The exposed hands alongside the closing book change,
while the pink cover and beige paper retain their separate colors. Shocked mouth
interior `#410808` and red `#9E2626` also remain original. Hair, eyes, blouse trim,
yellow jacket, belt, shorts, shoes and outlines retain their materials. Every
actual frame was inspected in Vanilla and all four targets using source material
grids and ignored all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Shocked start / loop / end | 78 / 86 / 78 |
| Seated-reading start | 46, 50, 44 |
| Seated-reading loop | 37, 50, 37, 50 |
| Seated-reading end | 54, 40, 46 |

- [Five-choice summary](../../generated/reina-spring-reactions-preview/summary.png):
  shocked start/loop and seated-reading start frame two/loop frame one.
- [Complete Vanilla/Blue review](../../generated/reina-spring-reactions-preview/blue-review/index.html):
  all 13 cases and 26 views across two pages of at most eight cases.

The preview directory is about 232 KiB. Previews bind to the final standalone
bundle. Crop `[29,25,22,30]` includes every opaque pixel. Saved-image checks
cover 2,999,700 exact pixels across all-target sheets, complete review sheets
and 20 summary bindings, including source/palette identity and zero mirrored
cases. Chromium decoded all images across four HTML pages, checked every case
and local link, and found no horizontal overflow. Evidence is in
`tmp/reina-spring-reactions-author-preview-check.log`,
`tmp/reina-spring-reactions-author-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed before exact candidate
promotion. The test checks literal skin/material landmarks, every source pixel
against independently reviewed material classes and the 22 jacket exclusions,
exact per-frame counts, alpha, metadata, common masks and all previous outputs.
A full-corpus omission control failed at the hand in reading-start frame two
`[115,45]`; a spill control failed at the jacket fold beside the book in the
reading loop `[37,42]`. Both reached the intended material assertion after
successful generation. `FOM_REINA_SPRING_REACTIONS_PRESETS` supports these ignored control copies.
Logs are `tmp/reina-spring-reactions-author-focused.log` and
`tmp/reina-spring-reactions-author-{omission,spill}-test.log`.

All 524 final variants passed exact recipe validation. All 125 previous region
objects and mappings remain unchanged, and all 1,250 previous original/variant
PNG and metadata files match the accepted Spring standard baseline byte for byte.
All 48 new variant PNG/metadata files exactly match the inspected candidate.
Final sidecars also match the independent raw inventory. The canonical Debug
Blue recipe matches the preset set. Reports are
`tmp/reina-spring-reactions-author-compare.json` and
`tmp/reina-spring-reactions-author-final-vs-candidate.json`; data and test hashes
are in `tmp/reina-spring-reactions-author-frozen.json`.

See [shared integration](reina-juniper-march-reactions.md) for native selection
and isolated installation. Static images do not establish animation timing,
scheduling or state transitions. Artwork remains ignored; this slice changes
no runtime behavior. Further actions and outfits remain for later batches.
