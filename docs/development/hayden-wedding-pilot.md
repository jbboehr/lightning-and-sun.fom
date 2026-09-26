# Hayden's Wedding idle and walk

Six strips add Wedding idle/walk North, South and East. Their 15 source frames
produce 20 review cases with five native West mirrors. Hayden now covers 296
sources and 1,184 variants. All 290 accepted regions, source pins, color groups
and palette mappings remain unchanged. The user approved this artwork for commit on 2026-09-25.

The read-only source is `tmp/fields-of-mistria/assets.zip`, unchanged from the
accepted Beach swimming batch, with SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Sources live under `assets/animations/NPCs/Hayden/Sprites/Wedding/` with the
`spr_npc_hayden_wedding_` prefix. The full corpus is
`extracted/hayden-wedding-pilot-study`; exported raw sidecars are recorded in
`tmp/hayden-wedding-pilot-author-metadata.json`. The native metadata retains
80×80 frames, Default atlas and Middle/54 origin. Idle uses native defaults;
walk has four frames at 0.15 seconds. No source hash was changed or relaxed.

The 196 reviewed component seeds select 310 skin pixels per target. The three
world shades `#E7B172`, `#AB7E3F` and `#815A2E` select faces and hands. Five
isolated East cheek pixels in `#E8B271` use the existing portrait-highlight
mapping. The darkest world shade, `#523C26`, appears on 63 shoe pixels in this
outfit; every one stays original. No global shade mapping changed.

Cuffs, collar, suit, vest, flower, shoes, hair, beard, eyes, outlines and
transparency stay original. Every actual frame was inspected in Vanilla and
all four targets using material-color grids and ignored all-target sheets.
Final sheets and previews use the actual standalone bundle. Skin counts are:

| Strip | Pixels per frame |
| --- | --- |
| Idle East | 23 |
| Idle North | 12 |
| Idle South | 28 |
| Walk East | 23, 26, 23, 21 |
| Walk North | 12, 11, 12, 11 |
| Walk South | 28, 26, 28, 26 |

- [Five-choice summary](../../generated/hayden-wedding-pilot-preview/summary.png):
  idle North/South and second-frame walk East/South examples.
- [Complete Vanilla/Blue review](../../generated/hayden-wedding-pilot-preview/blue-review/index.html):
  all 20 cases and 40 views across three pages of at most eight cases.

The complete preview directory is about 360 KiB. Crop `[31,26,18,30]` includes
every opaque pixel. Saved-image checks cover 3,345,300 exact pixels across
all-target sheets, review sheets and 20 summary bindings, including source
and palette identity and exact West reversal. Chromium decoded every image
across all five HTML pages, checked every case and local link, and found no
horizontal overflow. Evidence is in
`tmp/hayden-wedding-pilot-author-preview-check.log`,
`tmp/hayden-wedding-pilot-author-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed before candidate
promotion. The test checks 67 literal skin/material landmarks, every source
pixel against the independently reviewed material classes, frame coverage,
strip totals, alpha, metadata, common masks and all accepted output bytes.
Production data was checked for exact object equality with that passing
candidate. Removing a cheek seed failed at East `[43,35]`; selecting a shoe
component failed at `[38,52]`. Both intended failures occurred in the material
assertion after successful generation. The optional
`FOM_HAYDEN_WEDDING_PRESETS` test input supports these ignored negative-control
copies. Logs are `tmp/hayden-wedding-pilot-author-test.log` and
`tmp/hayden-wedding-pilot-author-{omission,spill}-test.log`.

All 1,184 final variants passed exact recipe validation, and all 2,900 accepted
original/variant PNG and metadata files remain byte-identical. Final new
variants also match the inspected candidate files exactly. The comparison
report is `tmp/hayden-wedding-pilot-author-compare.json`; frozen data and test
hashes are in `tmp/hayden-wedding-pilot-author-frozen-inputs.sha256`.

See [combined integration](world-wedding-pilot.md) for shared checks, native
animation verification and installation. Static images do not establish live
animation timing, NPC scheduling or state transitions. Artwork stays ignored;
this character slice changes no runtime code.
