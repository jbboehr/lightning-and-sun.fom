# Reina Spring chopping and polishing

Four strips add chopping North and the start, loop and end of polishing East.
They contain 29 source frames: 20 chopping and nine polishing. Native polishing
also uses nine West mirrors, making 38 review cases. The existing world profile
grows from 137 to 141 sources and produces 564 variants. The choices remain
Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved this artwork for
commit on 2026-09-26.

Sources are under `assets/animations/NPCs/Reina/Sprites/Spring/`, named
`spr_npc_reina_specialanimation_spring_chop_north.png` and
`spr_npc_reina_specialanimation_spring_polish_{start,loop,end}_east.png`.
The full local corpus is `extracted/reina-spring-finish-study`; raw sidecars
are recorded in `tmp/reina-spring-finish-author-metadata.json`. All four match
the independent archive read and retain 80×80 frames, Default atlas and numeric
origin `[40.0,54.0]`. The chopping strip retains its full 20-frame duration array,
including the final 0.1/0.8-second frames. Polishing start has three frames at
0.125 seconds; loop has four with `[0.1,0.125,0.1,0.225]`; end has two with
`[0.125,0.1]`.
The read-only archive is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
No old source pin was refreshed or relaxed.

The existing four world shades cover all new skin; no colors, groups or mappings
were added or changed. The 365 new component seeds select 660 skin pixels per
target, including exposed hands, neckline, midriff and legs. Eight isolated
jacket-fold pixels reuse skin highlight `#B36844` and stay original. The single
highlight at strip coordinate `[118,45]` in polishing-start frame two is exposed
midriff and changes; the polishing-loop shoulder fold `[38,42]` stays original.
The knife, green chopping effects, blue glass and polishing cloth retain their
own colors. Hair, eyes, blouse trim, yellow jacket, belt, shorts, shoes and
outlines also retain their materials. Every actual frame was inspected in
Vanilla and all four targets using source material grids and ignored all-target
sheets, including all 20 chopping frames.

| Strip | Skin pixels per frame |
| --- | --- |
| Chop North | 20, 13, 9, 8, 7, 12, 9, 8, 7, 12, 9, 8, 7, 12, 9, 8, 7, 12, 20, 26 |
| Polishing start | 52, 48, 47 |
| Polishing loop | 41, 41, 41, 42 |
| Polishing end | 55, 70 |

- [Five-choice summary](../../generated/reina-spring-finish-preview/summary.png):
  first-frame chopping and polishing start/loop/end samples.
- [Complete Vanilla/Blue review](../../generated/reina-spring-finish-preview/blue-review/index.html):
  all 38 cases and 76 views across five pages of at most eight cases.

The preview directory is about 516 KiB. Previews bind to the final standalone
bundle. Crop `[26,27,28,28]` includes every opaque pixel, including
chopping effects. Saved-image checks cover 9,051,280 exact pixels across all-target
sheets, complete review sheets and 20 summary bindings, including source/palette
identity and exact West reversal. Chromium decoded all images across seven
HTML pages, checked every case and local link, and found no horizontal overflow.
Evidence is in `tmp/reina-spring-finish-author-preview-check.log`,
`tmp/reina-spring-finish-author-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed before exact candidate
promotion. The test checks literal skin/material landmarks, every source pixel
against independently reviewed material classes and the eight jacket exclusions,
exact per-frame counts, alpha, metadata, common masks and all previous outputs.
A full-corpus omission control failed at the exposed midriff component
`[118,45]` in polishing start; a spill control failed at the polishing-loop
jacket fold `[38,42]`. Both reached the intended material assertion after
successful generation. `FOM_REINA_SPRING_FINISH_PRESETS`
supports these ignored control copies. Logs are
`tmp/reina-spring-finish-author-focused.log` and
`tmp/reina-spring-finish-author-{omission,spill}-test.log`.

All 564 final variants passed exact recipe validation. All 137 previous region
objects and mappings remain unchanged, and all 1,370 previous original/variant
PNG and metadata files match the accepted specials baseline byte for byte.
All 32 new variant PNG/metadata files exactly match the inspected candidate.
Final sidecars also match the independent raw inventory. The canonical Debug
Blue recipe matches the preset set. Reports are
`tmp/reina-spring-finish-author-compare.json` and
`tmp/reina-spring-finish-author-final-vs-candidate.json`; data and test hashes
are in `tmp/reina-spring-finish-author-frozen.json`.

See [shared integration](reina-juniper-march-finish.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. Further outfits remain for later batches.
