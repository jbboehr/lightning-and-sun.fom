# Reina Spring standing and seated writing

Six strips add the start, loop and end of standing and seated writing. They
contain 16 actual South-facing frames, with no West mirrors. The existing world
profile grows from 131 to 137 sources and produces 548 variants. The choices
remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved this
artwork for commit on 2026-09-26.

Sources are under `assets/animations/NPCs/Reina/Sprites/Spring/`, named
`spr_npc_reina_specialanimation_spring_write_{start,loop,end}_south.png` and
`spr_npc_reina_specialanimation_spring_write_sit_{start,loop,end}_south.png`.
The full local corpus is `extracted/reina-spring-specials-study`; raw sidecars
are recorded in `tmp/reina-spring-specials-author-metadata.json`. All six match
the independent archive read and retain 80×80 frames and Default atlas. Standing
writing retains numeric horizontal origin `40.0`, while seated writing retains
`Middle`; both keep vertical `54.0`. Start strips have two frames at 0.125 seconds;
loops have four with `[0.1, 0.125, 0.1, 0.3]`; ends have two with `[0.125, 0.1]`.
The read-only archive is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
No old source pin was refreshed or relaxed.

The existing four world shades cover all new skin; no colors, groups or mappings
were added or changed. The 406 new component seeds select 832 skin pixels per
target, including the hands gripping the pencil and clipboard, exposed necks,
midriff and legs. Twelve isolated jacket pixels reuse skin highlight `#B36844`
and stay original. The orange pencil, pale wood and point, clipboard backing,
dark edge, paper and metal clip all retain their own colors. In particular,
clipboard brown `#B28159` and edge `#57342B` are separate from the skin ramp.
The third standing loop frame has an exposed midriff highlight at `[200,45]`;
the seated counterpart has jacket ochre there and stays original. Hair, eyes,
blouse trim, yellow jacket, belt, shorts, shoes and outlines also retain their
materials. Every actual frame was inspected in Vanilla and all four targets
using source material grids and ignored all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Standing-writing start | 58, 52 |
| Standing-writing loop | 55, 55, 53, 56 |
| Standing-writing end | 52, 58 |
| Seated-writing start | 54, 45 |
| Seated-writing loop | 48, 48, 50, 49 |
| Seated-writing end | 45, 54 |

- [Five-choice summary](../../generated/reina-spring-specials-preview/summary.png):
  first-frame standing/seated start and loop samples.
- [Complete Vanilla/Blue review](../../generated/reina-spring-specials-preview/blue-review/index.html):
  all 16 cases and 32 views across two pages of eight cases.

The preview directory is about 258 KiB. Previews bind to the final standalone
bundle. Crop `[30,27,22,28]` includes every opaque pixel. Saved-image checks
cover 3,400,320 exact pixels across all-target sheets, complete review sheets
and 20 summary bindings, including source/palette identity and zero mirrored
cases. Chromium decoded all images across four HTML pages, checked every case
and local link, and found no horizontal overflow. Evidence is in
`tmp/reina-spring-specials-author-preview-check.log`,
`tmp/reina-spring-specials-author-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed before exact candidate
promotion. The test checks literal skin/material landmarks, every source pixel
against independently reviewed material classes and the 12 jacket exclusions,
exact per-frame counts, alpha, metadata, common masks and all previous outputs.
A full-corpus omission control failed at the seated-writing hand `[195,45]`;
a spill control failed at the standing-start jacket fold `[36,44]`. Both reached
the intended material assertion after successful generation.
`FOM_REINA_SPRING_SPECIALS_PRESETS` supports these ignored control copies.
Logs are `tmp/reina-spring-specials-author-focused.log` and
`tmp/reina-spring-specials-author-{omission,spill}-test.log`.

All 548 final variants passed exact recipe validation. All 131 previous region
objects and mappings remain unchanged, and all 1,310 previous original/variant
PNG and metadata files match the accepted reactions baseline byte for byte.
All 48 new variant PNG/metadata files exactly match the inspected candidate.
Final sidecars also match the independent raw inventory. The canonical Debug
Blue recipe matches the preset set. Reports are
`tmp/reina-spring-specials-author-compare.json` and
`tmp/reina-spring-specials-author-final-vs-candidate.json`; data and test hashes
are in `tmp/reina-spring-specials-author-frozen.json`.

See [shared integration](reina-juniper-march-specials.md) for native selection
and isolated installation. Static images do not establish animation timing,
scheduling or state transitions. Artwork remains ignored; this slice changes
no runtime behavior. Further actions and outfits remain for later batches.
