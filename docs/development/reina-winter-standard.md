# Reina Winter general action, sleep and kiss

Five strips add Winter general action North/South/East, sleep East and kiss East.
They contain 26 source frames and 12 native West mirrors, making 38 review cases.
Reina's world profile grows from 228 to 233 sources and produces 932 variants.
The choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user
approved the artwork for commit on 2026-09-28.

Sources are under `assets/animations/NPCs/Reina/Sprites/Winter/`, named
`spr_npc_reina_winter_action_{north,south,east}.png`,
`spr_npc_reina_winter_sleep_east.png` and `spr_npc_reina_winter_kiss_east.png`.
The full local corpus is `extracted/reina-winter-standard-study`; raw sidecars
are recorded in `tmp/reina-winter-standard-author-metadata.json`. All five
retain 80×80 frames, Default atlas and `Middle`/54 offsets. General actions
retain seven frames with durations `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss
retains four frames with `[0.15,0.15,0.8,0.15]`; sleep retains single-frame
defaults. Sources were freshly exported with strict pins; no earlier source
hash was refreshed or relaxed.

The native general action cycle has directional packs. Kiss and sleep declare
only East in the NPC definition, but use Single packs, which permit West
cardinality with the NPC object's horizontal mirroring. The complete review
includes mirrored sleep and kiss artwork as well as general action West.
These are renderable views, not a claim about natural sleep or kiss dispatch.

The 422 new component seeds select 867 skin pixels per target, covering faces
and moving hands. Hair, eyes, jacket, scarf, trousers, boots and outlines keep
their original colors. All four existing world skin shades belong to skin in
these strips. The 26 scarf pixels using the portrait skin color `#9E512F` are
excluded. No source colors, groups or mappings changed. Every actual frame was
inspected in Vanilla and all four targets.

The second general-action-East frame has a four-pixel hand highlight beginning
at strip `[128,43]`, immediately beside the jacket sleeve at `[127,43]`.
The sleeping hand beside the face includes highlight `[43,40]` and darkest
skin `[44,39]`; sleeve `[42,40]` stays original. Literal landmarks also guard
the kiss mouth and the sleeve beside its extended hand.

| Strip | Skin pixels per frame |
| --- | --- |
| General action East | 37, 39, 38, 39, 38, 37, 40 |
| General action North | 8, 6, 6, 6, 6, 8, 12 |
| General action South | 44, 44, 44, 44, 44, 44, 46 |
| Kiss East | 38, 47, 52, 51 |
| Sleep East | 49 |

- [Five-choice summary](../../generated/reina-winter-standard-preview/summary.png):
  second-frame general actions South/East, sleep and first-frame kiss samples.
- [Complete Vanilla/Blue review](../../generated/reina-winter-standard-preview/blue-review/index.html):
  all 38 cases and 76 views across five pages of at most eight cases.

The focused corpus test uses literal material landmarks and checks every new
pixel against the inspected skin classes, per-frame counts, alpha, metadata,
common masks and all previous outputs. `FOM_REINA_WINTER_STANDARD_PRESETS`
supports ignored control copies. The omission control removes the action-East
hand component at `[128,43]`; the spill control selects the first-frame scarf
pixel at `[43,43]`, which already has a portrait mapping. Both controls failed
at their intended material assertion after successful generation. They require
no synthetic color alias and do not change production recipes. The production
focused test and targeted Clippy passed. Logs are
`tmp/reina-winter-standard-author-focused.log` and
`tmp/reina-winter-standard-author-{omission,spill}.log`.

All 932 final variants passed exact recipe validation. All 228 earlier region
objects, groups and mappings remain unchanged; all 2,280 previous original and
variant PNG/metadata files match the accepted Winter actions baseline byte for
byte. All 40 new variant PNG/metadata files match the visually inspected
candidate. The five raw sidecars match the independent archive inventory, and
the canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-winter-standard-author-compare.json`,
`tmp/reina-winter-standard-author-final-vs-candidate.json` and
`tmp/reina-winter-standard-author-frozen.json`.

The complete preview is 492,545 bytes, about 481 KiB. Crop `[30,26,21,29]`
contains every opaque pixel. Saved-image checks compared 6,802,530 exact
rendered pixels across all-target evidence, complete review sheets and 20
summary bindings, including source/palette identity and exact West reversal.
Chromium checked all seven HTML pages, every case, image decode and local link,
with no horizontal overflow. Reports are
`tmp/reina-winter-standard-author-preview.log`,
`tmp/reina-winter-standard-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-winter-standard.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. Winter special actions remain for later batches.
