# Reina Autumn general action, sleep and kiss

Five strips add Autumn general action North/South/East, sleep East and kiss East.
They contain 26 source frames and 12 native West mirrors, making 38 review cases.
Reina's world profile grows from 193 to 198 sources and produces 792 variants.
The choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. This artwork
was approved by the user on 2026-09-27.

Sources are under `assets/animations/NPCs/Reina/Sprites/Autumn/`, named
`spr_npc_reina_autumn_action_{north,south,east}.png`,
`spr_npc_reina_autumn_sleep_east.png` and `spr_npc_reina_autumn_kiss_east.png`.
The full local corpus is `extracted/reina-autumn-standard-study`; raw sidecars
are recorded in `tmp/reina-autumn-standard-author-metadata.json`. All five
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

The 492 new component seeds select 992 skin pixels per target, covering faces,
necklines, napes and moving hands. Hair, eyes, checkered sleeves, trousers,
gold trim, boots and outlines keep their original colors. All four existing
world skin shades belong to skin in these new strips. No source colors,
groups or mappings changed; earlier Autumn walking boot exclusions remain
intact. Every actual frame was inspected in Vanilla and all four targets.

The second general-action-East frame has a five-pixel hand highlight beginning
at strip `[127,43]`, immediately beside the checkered sleeve at `[126,43]`.
The sleeping hand beside the face includes highlight `[43,40]` and darkest
skin `[44,39]`; sleeve `[42,40]` stays original. The kissing pose similarly
keeps the sleeve separate from the extended hand.

| Strip | Skin pixels per frame |
| --- | --- |
| General action East | 42, 41, 41, 41, 41, 42, 46 |
| General action North | 11, 10, 10, 10, 10, 12, 17 |
| General action South | 51, 51, 50, 51, 50, 51, 54 |
| Kiss East | 43, 52, 59, 56 |
| Sleep East | 50 |

- [Five-choice summary](../../generated/reina-autumn-standard-preview/summary.png):
  second-frame general actions South/East, sleep and first-frame kiss samples.
- [Complete Vanilla/Blue review](../../generated/reina-autumn-standard-preview/blue-review/index.html):
  all 38 cases and 76 views across five pages of at most eight cases.

The focused corpus test uses literal material landmarks and checks every
new pixel against the inspected skin classes, per-frame counts, alpha,
metadata, common masks and all previous outputs.
`FOM_REINA_AUTUMN_STANDARD_PRESETS` supports ignored control copies. The
omission control removes the action-East hand component at `[127,43]`; the
spill control adds a gold `#D2962F` alias in an ignored copy and selects the
first-frame collar at `[41,42]`. Both controls failed at their intended
material assertion after successful full generation; neither changed the
production recipes. The production focused test and targeted Clippy passed.
Logs are `tmp/reina-autumn-standard-author-focused.log` and
`tmp/reina-autumn-standard-author-{omission,spill}.log`.

All 792 final variants passed exact recipe validation. All 193 earlier region
objects, groups and mappings remain unchanged; all 1,930 previous original and
variant PNG/metadata files match the accepted Autumn actions baseline byte for
byte. All 40 new variant PNG/metadata files match the visually inspected
candidate. The five raw sidecars match the independent archive inventory, and
the canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-autumn-standard-author-compare.json`,
`tmp/reina-autumn-standard-author-final-vs-candidate.json` and
`tmp/reina-autumn-standard-author-frozen.json`.

The complete preview is 490,563 bytes, about 479 KiB. Crop `[30,26,21,29]`
contains every opaque pixel. Saved-image checks compared 6,802,530 exact
rendered pixels across all-target evidence, complete review sheets and 20
summary bindings, including source/palette identity and exact West reversal.
Chromium checked all seven HTML pages, every case, image decode and local link,
with no horizontal overflow. Reports are
`tmp/reina-autumn-standard-author-preview-check.log`,
`tmp/reina-autumn-standard-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-autumn-standard.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. Autumn special actions remain for later batches.
