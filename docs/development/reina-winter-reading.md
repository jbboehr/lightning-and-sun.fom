# Reina Winter seated reading

Three strips add the start, loop and end of Winter seated reading. They contain
10 South-facing source frames; there are no West mirrors. Reina's world profile
grows from 233 to 236 sources and produces 944 variants. The choices remain
Vanilla, Debug Blue, Hayden, Ryis and Seridia. This artwork awaits user review.

Sources are under `assets/animations/NPCs/Reina/Sprites/Winter/`, named
`spr_npc_reina_specialanimation_winter_read_sit_{start,loop,end}_south.png`.
The full local corpus is `extracted/reina-winter-reading-study`; raw sidecars
are in `tmp/reina-winter-reading-author-metadata.json`. All three retain 80×80
frames, Default atlas and `Middle`/54 offsets. Start/end retain three frames at
0.1 seconds; the loop retains four frames with `[3.0,0.1,3.0,0.1]` durations.
The native cycle is complex, South-only and seated. Sources were freshly
exported with strict pins; no earlier source hash was refreshed or relaxed.

The 192 new component seeds select 432 skin pixels per target, covering faces
and exposed hands beside the book. The pink cover, pale paper and page shading,
Winter jacket, trousers, boots, hair, eyes and outlines retain their original
colors. All 12 scarf pixels using the portrait skin color `#9E512F` remain
excluded. No source colors, groups or mappings changed. Every actual frame was
inspected in Vanilla and all four targets.

Winter's sleeves cover more of the hands than Autumn's outfit. The three-pixel
hand highlight beginning at strip `[114,46]` in reading start changes; the
jacket pixel immediately above at `[114,45]` stays original. Literal landmarks
also protect all four pink/brown cover colors, both pale page shades and every
scarf pixel that shares a portrait mapping.

| Strip | Skin pixels per frame |
| --- | --- |
| Reading start | 41, 46, 44 |
| Reading loop | 35, 50, 35, 50 |
| Reading end | 54, 36, 41 |

- [Five-choice summary](../../generated/reina-winter-reading-preview/summary.png):
  start frames one/three, loop frame two and end frame one.
- [Complete Vanilla/Blue review](../../generated/reina-winter-reading-preview/blue-review/index.html):
  all 10 cases and 20 views across two pages of at most eight cases.

The focused corpus test uses literal skin/material landmarks and checks every
new pixel against the inspected skin classes, per-frame counts, alpha,
metadata, common masks and all previous outputs.
`FOM_REINA_WINTER_READING_PRESETS` supports ignored control copies. The omission
control removes the hand highlight `[114,46]` from reading start. The spill
control selects its first-frame scarf pixel `[38,42]`, which already has a
portrait mapping. Both controls failed at the intended material assertion
after successful generation. They use no synthetic color alias and do not
change production recipes. The production focused test and targeted Clippy
passed. Logs are `tmp/reina-winter-reading-author-focused.log` and
`tmp/reina-winter-reading-author-{omission,spill}.log`.

All 944 final variants passed exact recipe validation. All 233 earlier region
objects, groups and mappings remain unchanged; all 2,330 previous original and
variant PNG/metadata files match the accepted Winter standard baseline byte
for byte. All 24 new variant PNG/metadata files match the visually inspected
candidate. The three raw sidecars match the independent archive inventory;
the canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-winter-reading-author-compare.json`,
`tmp/reina-winter-reading-author-final-vs-candidate.json` and
`tmp/reina-winter-reading-author-frozen.json`.

The complete preview is 190,265 bytes, about 186 KiB. Crop `[29,27,22,25]`
contains every opaque pixel, including the open book. Saved-image checks
compared 1,963,500 exact rendered pixels across all-target evidence, complete
review sheets and 20 summary bindings, including source/palette identity and
zero mirrored cases. Chromium checked all four HTML pages, every case, image
decode and local link, with no horizontal overflow. Reports are
`tmp/reina-winter-reading-author-preview.log`,
`tmp/reina-winter-reading-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-winter-reading.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. Further Winter special actions remain.
