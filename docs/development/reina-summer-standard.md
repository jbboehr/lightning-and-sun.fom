# Reina Summer general action, sleep and kiss

Five strips add Summer general action North/South/East, sleep East and kiss East.
They contain 26 source frames and 12 native West mirrors, making 38 review cases.
Reina's world profile grows from 158 to 163 sources and produces 652 variants.
The choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. This artwork
awaits user review.

Sources are under `assets/animations/NPCs/Reina/Sprites/Summer/`, named
`spr_npc_reina_summer_action_{north,south,east}.png`,
`spr_npc_reina_summer_sleep_east.png` and `spr_npc_reina_summer_kiss_east.png`.
The full local corpus is `extracted/reina-summer-standard-study`; raw sidecars
are recorded in `tmp/reina-summer-standard-author-metadata.json`. All five match
the independent archive read and retain 80×80 frames, Default atlas and numeric
origin `[40.0,54.0]`. General actions retain seven frames with durations
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss retains four frames with
`[0.15,0.15,0.8,0.15]`; sleep retains single-frame defaults. The read-only archive
is `tmp/fields-of-mistria/assets.zip`; all source pins remain strict and no
earlier pin was refreshed or relaxed.

The native action cycle has directional packs and retains its `[240,360]`
last-frame hold. Kiss and sleep list only East in the NPC definition, but use
Single packs: the native animation handler returns that pack regardless of
cardinality, and the NPC object flips the image when facing West. The complete
review therefore includes mirrored sleep and kiss artwork as well as general
action West. This establishes renderable directions, not when schedules request
each direction.

All four existing world skin shades belong to exposed skin in these strips;
no source colors, groups or target mappings were added or changed. The 900 new
component seeds select 1,739 skin pixels per target, covering faces, necklines,
shoulders, gesturing hands, exposed back, arms, legs and toes. Hair, eyes, cream
and tan blouse pixels, green fabric and straps, gold buttons, pink accessories,
sandal straps and outlines retain their original colors. In particular,
general-action-East frame two's seven-pixel hand-highlight component begins at
strip coordinate `[125,43]`; the first frame's dark green strap at `[39,42]`
stays original. Every actual source frame was inspected in Vanilla and all four
target palettes using source material grids and ignored all-target sheets.

| Strip | Skin pixels per frame |
| --- | --- |
| General action East | 66, 67, 68, 67, 68, 66, 75 |
| General action North | 50, 41, 41, 41, 41, 53, 55 |
| General action South | 77, 78, 79, 78, 79, 77, 86 |
| Kiss East | 68, 77, 86, 81 |
| Sleep East | 74 |

- [Five-choice summary](../../generated/reina-summer-standard-preview/summary.png):
  second-frame general actions South/East, sleeping and first-frame kiss samples.
- [Complete Vanilla/Blue review](../../generated/reina-summer-standard-preview/blue-review/index.html):
  all 38 cases and 76 views across five pages of at most eight cases.

The complete preview directory is about 476 KiB. Previews bind to the final
standalone bundle. Crop `[30,26,21,29]` includes every opaque source pixel.
Saved-image checks cover 6,802,530 exact pixels across all-target sheets,
complete review sheets and the 20 summary bindings, including source/palette
identity and exact West reversal. Chromium decoded all images across seven HTML
pages, checked every case and local link, and found no horizontal overflow.
Evidence is in `tmp/reina-summer-standard-author-preview-check.log`,
`tmp/reina-summer-standard-author-preview-browser-check.json`, and the gallery's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test uses literal
skin/material landmarks and checks every source pixel against independently
reviewed skin classes, exact per-frame counts, alpha, metadata, common masks and
all previous outputs. A full-corpus omission control failed at the exposed hand
component `[125,43]` in general action East. Because no clothing in these strips
reuses a skin shade, the spill control deliberately adds an ignored green-fabric
alias and seed; it fails at the first general-action-East frame's strap
`[39,42]`. Both controls reached the intended material assertion after successful
generation; neither altered the production recipes.
`FOM_REINA_SUMMER_STANDARD_PRESETS` supports these ignored control copies. Logs
are `tmp/reina-summer-standard-author-focused.log` and
`tmp/reina-summer-standard-author-{omission,spill}.log`.

All 652 final variants passed exact recipe validation. All 158 earlier region
objects, groups and mappings remain unchanged, and all 1,580 previous
original/variant PNG and metadata files match the accepted Summer actions
baseline byte for byte. All 40 new variant PNG/metadata files exactly match the
visually inspected candidate. Final sidecars also match the independent raw
inventory. The canonical Debug Blue recipe matches the preset set. Reports are
`tmp/reina-summer-standard-author-compare.json` and
`tmp/reina-summer-standard-author-final-vs-candidate.json`; data and test hashes
are in `tmp/reina-summer-standard-author-frozen.json`.

See [shared integration](reina-juniper-march-summer-standard.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. Summer reactions and special actions remain for
later batches.
