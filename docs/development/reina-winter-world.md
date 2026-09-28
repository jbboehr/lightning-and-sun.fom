# Reina Winter idle and walk

Six strips add Winter idle and walk North, South and East: 15 source frames and
five native West mirrors, giving 20 review cases. Reina's profile grows from
211 to 217 sources and produces 868 variants. Choices remain Vanilla, Debug
Blue, Hayden, Ryis and Seridia. The user approved this artwork on 2026-09-27.

Sources are under `assets/animations/NPCs/Reina/Sprites/Winter/`, named
`spr_npc_reina_winter_{idle,walk}_{north,south,east}.png`. The fresh corpus is
`extracted/reina-winter-study`; raw sidecars are recorded in
`tmp/reina-winter-author-metadata.json`. All six match the independent archive
read and retain 80×80 frames, Default atlas and `Middle`/54 origins. Idle keeps
its one-frame defaults; walking has four frames at 0.15 seconds. Native West
mirrors East. Source pins remain strict, with no earlier pin refreshed or
relaxed.

The 236 new component seeds select 484 skin pixels per target, covering faces
and bare hands. The Winter jacket, scarf, trousers, boots, hair, eyes and
outlines keep their original colors. The scarf hides the nape visible in some
other outfits; its cream and ochre collar pixels remain original.

Scarf shading reuses the portrait skin color `#9E512F`: two pixels in every
East/South idle or walk frame, 20 pixels total. These are deliberately excluded
from the new masks. The existing world ramp covers all exposed skin; no source
colors, groups or mappings changed. Every actual frame was inspected in Vanilla
and all four targets using source grids and all-target sheets. All earlier
material choices, including Autumn boot exclusions, remain intact.

| Strip | Skin pixels per frame |
| --- | --- |
| Idle East | 40 |
| Idle North | 12 |
| Idle South | 46 |
| Walk East | 40, 43, 40, 39 |
| Walk North | 12, 10, 12, 10 |
| Walk South | 46, 44, 46, 44 |

- [Five-choice summary](../../generated/reina-winter-preview/summary.png):
  idle North/South and second-frame walk East/South samples.
- [Complete Vanilla/Blue review](../../generated/reina-winter-preview/blue-review/index.html):
  all 20 cases and 40 views across three pages of at most eight cases.

The focused corpus test and targeted Clippy passed. Literal skin/material
landmarks and full-pixel checks cover omissions, scarf collisions, per-frame
counts, alpha, metadata, common masks and all previous outputs.
`FOM_REINA_WINTER_WORLD_PRESETS` supports ignored control recipes. Removing
the hand-highlight seed at idle North `[32,46]` fails the material check.
Selecting scarf shading at idle South `[38,42]` also fails it, using the existing
portrait mapping. Both controls reach the intended assertion after successful
generation and leave production definitions unchanged. Logs are
`tmp/reina-winter-author-focused.log` and
`tmp/reina-winter-author-{omission,spill}.log`.

All 868 final variants passed exact recipe validation. All 211 earlier region
objects, colors, groups and mappings remain unchanged. All 2,110 previous
original and variant PNG/metadata files match the accepted March Autumn injured
baseline byte for byte; all 48 new variant files match the inspected candidate.
The canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-winter-author-compare.json`,
`tmp/reina-winter-author-final-vs-candidate.json` and
`tmp/reina-winter-author-frozen.json`.

The complete preview is 286,577 bytes, about 280 KiB. Crop `[31,27,18,29]`
includes every opaque source pixel. Saved-image checks compared 3,233,790 exact
rendered pixels across all-target sheets, full review sheets and 20 summary
bindings, including source/palette identity and exact West reversal. Chromium
checked all five HTML pages, all 20 cases, image decodes and local links, with
no horizontal overflow. Reports are `tmp/reina-winter-author-preview.log`,
`tmp/reina-winter-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-winter.md) for native selection
and isolated installation. Static images do not establish animation timing,
scheduling or state transitions. Artwork remains ignored; this slice changes
no runtime behavior. Further Winter actions remain for later batches.
