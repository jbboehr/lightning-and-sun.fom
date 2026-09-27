# Reina Autumn seated reading

Three strips add the start, loop and end of Autumn seated reading. They contain
10 South-facing source frames; there are no West mirrors. Reina's world profile
grows from 198 to 201 sources and produces 804 variants. The choices remain
Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved this artwork on 2026-09-27.

Sources are under `assets/animations/NPCs/Reina/Sprites/Autumn/`, named
`spr_npc_reina_specialanimation_autumn_read_sit_{start,loop,end}_south.png`.
The full local corpus is `extracted/reina-autumn-reading-study`; raw sidecars
are in `tmp/reina-autumn-reading-author-metadata.json`. All three retain 80×80
frames, Default atlas and `Middle`/54 offsets. Start/end retain three frames at
0.1 seconds; the loop retains four frames with `[3.0,0.1,3.0,0.1]` durations.
The native cycle is complex, South-only and seated. Sources were freshly
exported with strict pins; no earlier source hash was refreshed or relaxed.

The 206 new component seeds select 460 skin pixels per target, covering faces,
necklines and exposed hands beside the book. The pink cover, pale paper and
page shading, checkered shirt, trousers, gold trim, boots, hair, eyes and
outlines retain their original colors. No source colors, groups, mappings or
material exclusions were added. Earlier Autumn walking boot exclusions remain
intact. Every actual frame was inspected in Vanilla and all four targets.

The four-pixel hand highlight beginning at strip `[115,45]` in reading start
changes; book-cover pixel `[35,46]` in the reading loop stays original. Tests
also preserve all four pink/brown cover colors and both pale page shades.

| Strip | Skin pixels per frame |
| --- | --- |
| Reading start | 46, 50, 44 |
| Reading loop | 38, 52, 38, 52 |
| Reading end | 54, 40, 46 |

- [Five-choice summary](../../generated/reina-autumn-reading-preview/summary.png):
  start frames one/three, loop frame two and end frame one.
- [Complete Vanilla/Blue review](../../generated/reina-autumn-reading-preview/blue-review/index.html):
  all 10 cases and 20 views across two pages of at most eight cases.

The focused corpus test uses literal skin/material landmarks and checks every
new pixel against the inspected skin classes, per-frame counts, alpha,
metadata, common masks and all previous outputs.
`FOM_REINA_AUTUMN_READING_PRESETS` supports ignored control copies. The omission
control removes the hand highlight `[115,45]` from reading start. The spill
control adds a pink `#DF7175` alias in an ignored copy and selects reading-loop
book cover `[35,46]`. Both controls failed at the intended material assertion
after successful full generation; neither changed production recipes. The
production focused test and targeted Clippy passed. Logs are
`tmp/reina-autumn-reading-author-focused.log` and
`tmp/reina-autumn-reading-author-{omission,spill}.log`.

All 804 final variants passed exact recipe validation. All 198 earlier region
objects, groups and mappings remain unchanged; all 1,980 previous original and
variant PNG/metadata files match the accepted Autumn standard baseline byte
for byte. All 24 new variant PNG/metadata files match the visually inspected
candidate. The three raw sidecars match the independent archive inventory;
the canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-autumn-reading-author-compare.json`,
`tmp/reina-autumn-reading-author-final-vs-candidate.json` and
`tmp/reina-autumn-reading-author-frozen.json`.

The complete preview is 189,795 bytes, about 185 KiB. Crop `[29,27,22,25]`
contains every opaque pixel, including the open book. Saved-image checks
compared 1,963,500 exact rendered pixels across all-target evidence, complete
review sheets and 20 summary bindings, including source/palette identity and
zero mirrored cases. Chromium checked all four HTML pages, every case, image
decode and local link, with no horizontal overflow. Reports are
`tmp/reina-autumn-reading-author-preview-check.log`,
`tmp/reina-autumn-reading-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-autumn-reading.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. Further Autumn special actions remain.
