# Reina Summer standing and seated writing

Six strips add the start, loop and end of standing and seated writing. They
contain 16 South-facing source frames, with no West mirrors. Reina's world
profile grows from 166 to 172 sources and produces 688 variants. The choices
remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved this
artwork for commit on 2026-09-26.

Sources are under `assets/animations/NPCs/Reina/Sprites/Summer/`, named
`spr_npc_reina_specialanimation_summer_write_{start,loop,end}_south.png` and
`spr_npc_reina_specialanimation_summer_write_sit_{start,loop,end}_south.png`.
The complete local corpus is `extracted/reina-summer-specials-study`; raw
sidecars are recorded in `tmp/reina-summer-specials-author-metadata.json`. All
six match the independent archive read and retain 80×80 frames and Default
atlas. Standing writing retains numeric horizontal origin `40.0`, while seated
writing retains `Middle`; both keep vertical `54.0`. Start strips retain two
frames at 0.125 seconds; loops have four with `[0.1,0.125,0.1,0.3]`; ends have
two with `[0.125,0.1]`. Both native cycles are complex and South-only;
`write_sit` is seated. The read-only archive is `tmp/fields-of-mistria/assets.zip`;
all source pins remain strict and no earlier pin was refreshed or relaxed.

The existing four world skin shades cover all exposed skin in these strips.
No source colors, groups, target mappings or material exclusions were added.
The 507 new component seeds select 1,014 skin pixels per target, covering faces,
necks, hands gripping the pencil and clipboard, arms, legs and toes. Orange
pencil colors, pale wood and point, clipboard backing `#B28159`, dark edge
`#57342B`, paper and metal clip retain their separate colors. Green fabric,
cream/tan blouse, gold button, hair, eyes, accessories, sandal straps and
outlines also remain original. The three-pixel hand highlight beginning at
strip coordinate `[116,46]` in standing-writing loop frame two changes;
clipboard brown at `[46,46]` in standing-writing start stays original. Every
actual frame was inspected in Vanilla and all four targets using source
material grids and ignored all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Standing-writing start | 74, 65 |
| Standing-writing loop | 68, 71, 69, 71 |
| Standing-writing end | 65, 74 |
| Seated-writing start | 63, 53 |
| Seated-writing loop | 56, 56, 56, 57 |
| Seated-writing end | 53, 63 |

- [Five-choice summary](../../generated/reina-summer-specials-preview/summary.png):
  second-frame standing and seated start/loop samples.
- [Complete Vanilla/Blue review](../../generated/reina-summer-specials-preview/blue-review/index.html):
  all 16 cases and 32 views across two pages of eight cases.

The complete preview directory is about 257 KiB. Previews bind to the final
standalone bundle. Crop `[30,27,22,28]` includes every opaque pixel, including
the pencil and clipboard. Saved-image checks cover 3,400,320 exact pixels across
all-target sheets, review sheets and 20 summary bindings, including source and
palette identity and zero mirrored cases. Chromium decoded all images across
four HTML pages, checked every case and local link, and found no horizontal
overflow. Evidence is in `tmp/reina-summer-specials-author-preview-check.log`,
`tmp/reina-summer-specials-author-preview-browser-check.json`, and the gallery's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test uses literal
skin/material landmarks and checks every source pixel against independently
reviewed skin classes, exact per-frame counts, alpha, metadata, common masks and
all previous outputs. A full-corpus omission control fails at the exposed pencil
hand component `[116,46]` in standing-writing loop frame two. A spill control
deliberately aliases clipboard brown `#B28159` to skin in an ignored copy and
selects `[46,46]` in standing-writing start; the material check rejects the
resulting clipboard recoloring. Both controls reach the intended material
assertion after successful generation and leave production recipes unchanged.
`FOM_REINA_SUMMER_SPECIALS_PRESETS` supports these ignored copies. Logs are
`tmp/reina-summer-specials-author-focused.log` and
`tmp/reina-summer-specials-author-{omission,spill}.log`.

All 688 final variants passed exact recipe validation. All 166 earlier region
objects, groups and mappings remain unchanged; all 1,660 previous original and
variant PNG/metadata files match the accepted Summer reading baseline byte for
byte. All 48 new variant PNG/metadata files exactly match the visually inspected
candidate. Final sidecars match the independent raw inventory, and the canonical
Debug Blue recipe matches the preset set. Reports are
`tmp/reina-summer-specials-author-compare.json` and
`tmp/reina-summer-specials-author-final-vs-candidate.json`; frozen hashes are in
`tmp/reina-summer-specials-author-frozen.json`.

See [shared integration](reina-juniper-march-summer-specials.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. Summer chopping and polishing remain for a later
batch.
