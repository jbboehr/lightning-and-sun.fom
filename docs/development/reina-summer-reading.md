# Reina Summer seated reading

Three strips add the start, loop and end of Summer seated reading. They contain
10 South-facing source frames; there are no West mirrors. Reina's world profile
grows from 163 to 166 sources and produces 664 variants. The choices remain
Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved this artwork
for commit on 2026-09-26.

Sources are under `assets/animations/NPCs/Reina/Sprites/Summer/`, named
`spr_npc_reina_specialanimation_summer_read_sit_{start,loop,end}_south.png`.
The full local corpus is `extracted/reina-summer-reading-study`; raw sidecars
are in `tmp/reina-summer-reading-author-metadata.json`. All three match the
independent archive read and retain 80×80 frames, Default atlas and numeric
origin `[40.0,54.0]`. Start/end retain three frames at 0.1 seconds; the loop
retains four frames with `[3.0,0.1,3.0,0.1]` durations. The native cycle is complex,
South-only and seated. The read-only archive is `tmp/fields-of-mistria/assets.zip`;
all source pins remain strict and no earlier pin was refreshed or relaxed.

The existing four world skin shades cover all exposed skin in these strips.
No source colors, groups, target mappings or material exclusions were added.
The 250 new component seeds select 516 skin pixels per target, covering faces,
necks, exposed hands beside the book, legs and toes. Pink book-cover colors,
beige paper and shading, green fabric, cream/tan blouse, gold button, hair, eyes,
accessories, sandal straps and outlines retain their original colors. The
four-pixel hand highlight beginning at strip coordinate `[115,45]` in reading
start changes; the book-cover pixel `[35,46]` in the reading loop stays original.
Every actual frame was inspected in Vanilla and all four targets using source
material grids and ignored all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| Reading start | 62, 52, 48 |
| Reading loop | 42, 54, 42, 54 |
| Reading end | 58, 42, 62 |

- [Five-choice summary](../../generated/reina-summer-reading-preview/summary.png):
  start frames one/three, loop frame two and end frame one.
- [Complete Vanilla/Blue review](../../generated/reina-summer-reading-preview/blue-review/index.html):
  all 10 cases and 20 views across two pages of at most eight cases.

The complete preview directory is about 185 KiB. Previews bind to the final
standalone bundle. Crop `[29,27,22,25]` includes every opaque pixel, including
the open book. Saved-image checks cover 1,963,500 exact pixels across all-target
sheets, review sheets and 20 summary bindings, including source/palette identity
and zero mirrored cases. Chromium decoded all images across four HTML pages,
checked every case and local link, and found no horizontal overflow. Evidence
is in `tmp/reina-summer-reading-author-preview-check.log`,
`tmp/reina-summer-reading-author-preview-browser-check.json`, and the gallery's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test uses literal
skin/material landmarks and checks every source pixel against independently
reviewed skin classes, exact per-frame counts, alpha, metadata, common masks and
all previous outputs. A full-corpus omission control fails at the exposed hand
component `[115,45]` in reading start. A spill control deliberately aliases pink
book-cover shade `#DF7175` to skin in an ignored copy and selects `[35,46]` in the
reading loop; the material check rejects the resulting book recoloring. Both
controls reach the intended material assertion after successful generation and
leave production recipes unchanged. `FOM_REINA_SUMMER_READING_PRESETS` supports
these ignored copies. Logs are `tmp/reina-summer-reading-author-focused.log`
and `tmp/reina-summer-reading-author-{omission,spill}.log`.

All 664 final variants passed exact recipe validation. All 163 earlier region
objects, groups and mappings remain unchanged; all 1,630 previous original and
variant PNG/metadata files match the accepted Summer standard baseline byte for
byte. All 24 new variant PNG/metadata files exactly match the visually inspected
candidate. Final sidecars match the independent raw inventory, and the canonical
Debug Blue recipe matches the preset set. Reports are
`tmp/reina-summer-reading-author-compare.json` and
`tmp/reina-summer-reading-author-final-vs-candidate.json`; frozen hashes are in
`tmp/reina-summer-reading-author-frozen.json`.

See [shared integration](reina-juniper-march-summer-reading.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. Further Summer special actions remain.
