# Reina Winter blinking, sitting, eating and drinking

Eleven strips add Winter blink East/South and sit, eat and drink North/South/East.
They contain 31 source frames and 12 native West mirrors, giving 43 review
cases. Reina's profile grows from 217 to 228 sources and produces 912 variants.
Choices remain Vanilla, Debug Blue, Hayden, Ryis and Seridia. The user approved
the artwork for commit on 2026-09-27.

Sources are under `assets/animations/NPCs/Reina/Sprites/Winter/`, named
`spr_npc_reina_winter_{blink,sit,eat,drink}_{direction}.png`. The fresh corpus is
`extracted/reina-winter-actions-study`; raw sidecars are in
`tmp/reina-winter-actions-author-metadata.json`. All eleven match the independent
archive inventory and retain 80×80 frames, Default atlas and `Middle`/54
offsets. Blink retains `[0.075,0.125,0.075]`; sit keeps its one-frame defaults.
East/South eating retains `[0.125,0.15,0.175,0.125,0.6]`; North eating and all
drinking strips retain three frames with duration `1.0`. Native West mirrors
East. Source pins remain strict, with no earlier pin refreshed or relaxed.

The 548 new component seeds select 1,154 skin pixels per target, covering faces
and moving bare hands. The brown jacket, scarf, trousers, boots, hair, eyes,
red/dark mouth interiors and outlines retain their original colors. Every
actual frame was inspected in Vanilla and all four targets. Scarf shading
reuses portrait skin `#9E512F` in 32 pixels across these strips; all remain
original. The existing four world skin shades cover the exposed skin. No source
colors, groups or mappings changed, and earlier material exclusions remain
intact.

| Strip | Skin pixels per frame |
| --- | --- |
| Blink East | 44, 54, 44 |
| Blink South | 50, 60, 50 |
| Sit East | 36 |
| Sit North | 10 |
| Sit South | 44 |
| Eat East | 35, 44, 35, 44, 35 |
| Eat North | 7, 5, 7 |
| Eat South | 45, 55, 53, 62, 44 |
| Drink East | 38, 48, 38 |
| Drink North | 7, 5, 7 |
| Drink South | 45, 58, 45 |

- [Five-choice summary](../../generated/reina-winter-actions-preview/summary.png):
  blink South, sit East, eat South and drink East samples.
- [Complete Vanilla/Blue review](../../generated/reina-winter-actions-preview/blue-review/index.html):
  all 43 cases and 86 views across six pages of at most eight cases.

The focused corpus test and targeted Clippy passed. Literal material landmarks
and full-pixel checks cover omitted skin, protected scarf and jacket pixels,
mouth interiors, per-frame counts, alpha, metadata, common masks and previous
outputs. `FOM_REINA_WINTER_ACTIONS_PRESETS` supports ignored control copies.
Removing the two-pixel raised-hand highlight beginning at drink East `[119,40]`
fails the material check. Selecting already-mapped scarf brown at blink East
`[39,42]` also fails it. Both controls reach the intended assertion after
successful generation, with no production recipe changes or synthetic aliases.
Logs are `tmp/reina-winter-actions-author-focused.log` and
`tmp/reina-winter-actions-author-{omission,spill}.log`.

All 912 final variants passed exact recipe validation. All 217 earlier region
objects, source colors, groups and mappings remain unchanged. All 2,170 previous
original and variant PNG/metadata files match the accepted Winter pilot
baseline byte for byte; all 88 new variant files match the inspected candidate.
The canonical Debug Blue recipe matches the preset set. Evidence is in
`tmp/reina-winter-actions-author-compare.json`,
`tmp/reina-winter-actions-author-final-vs-candidate.json` and
`tmp/reina-winter-actions-author-frozen.json`.

The complete preview is 532,056 bytes, about 520 KiB. Crop `[29,26,20,29]`
contains every opaque source pixel. Saved-image checks compared 7,421,100 exact
rendered pixels across all-target evidence, complete review sheets and 20
summary bindings, including source/palette identity and exact West reversal.
Chromium checked all eight HTML pages, all cases, image decodes and local links,
with no horizontal overflow. Reports are
`tmp/reina-winter-actions-author-preview.log`,
`tmp/reina-winter-actions-author-preview-browser-check.json` and the gallery's
`coverage.json`.

See [shared integration](reina-juniper-march-winter-actions.md) for native
selection and isolated installation. Static images do not establish animation
timing, scheduling or state transitions. Artwork remains ignored; this slice
changes no runtime behavior. General Winter actions remain for later batches.
