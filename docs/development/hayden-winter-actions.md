# Hayden's Winter blink, sit, eat and drink

Eleven strips add Winter blink East/South and sit, eat and drink North/South/East.
The 31 source frames produce 43 review cases with 12 native West mirrors. The
profile now covers 263 sources and 1,052 variants. All 252 accepted regions,
color groups and palette mappings remain unchanged. The user approved this slice for commit on 2026-09-25.

The read-only source is `tmp/fields-of-mistria/assets.zip`, unchanged from the
accepted Winter expansion batch, under
`assets/animations/NPCs/Hayden/Sprites/Winter/` with the
`spr_npc_hayden_winter_` prefix. The full corpus is
`extracted/hayden-winter-actions-study`; raw sidecars are recorded in
`tmp/hayden-winter-actions-metadata.json`. All frames retain 80×80 geometry,
Default atlas and Middle/54 origin. Blink retains three frames at
`[0.075,0.125,0.075]`; sit retains single-frame defaults. Drink and North eat
retain three frames at 1.0 seconds; East/South eat retain five frames at
`[0.125,0.15,0.175,0.125,0.6]`. All 2,520 accepted original/variant PNG and
metadata files still match byte for byte; no old source hash was changed.

The 339 reviewed component seeds select 837 skin pixels per target and protect
63 matching-color coat-seam pixels. Autumn mask reuse misses 27 small Winter
neckline pixels plus the isolated `#AB7E3F` eating finger at zero-based South
frame 2 `[36,37]`; the final masks include these. Gold sleeves, cream cuffs,
coat seams, blue shirt, trousers, boots, hair and beard stay original. Both
mouth reds, `#410808` and `#9E2626`, remain unchanged beside the raised hands.
No new shade or mapping was needed. Separate cup or food artwork is not baked
into these source strips.

All 31 source frames were inspected in Vanilla and all four targets on ignored
`tmp/hayden-winter-actions-art-*.png` sheets. Enlarged comparisons with the
Autumn source helped distinguish the raised hands from Winter sleeves and
cuffs. The final sheets and previews use the actual standalone bundle.

- [Five-choice summary](../../generated/hayden-winter-actions-preview/summary.png):
  blink South, sit North, eat East and drink East samples.
- [Complete Vanilla/Blue review](../../generated/hayden-winter-actions-preview/blue-review/index.html):
  all 43 cases and 86 views across six pages of at most eight cases.

The full crop `[30,25,20,30]` includes every opaque pixel. Saved-image checks
cover 7,677,000 exact pixels across all-target art sheets, review sheets and
the 20 summary bindings, including source/palette identity and West reversal.
Chromium decoded every image across all eight HTML pages, verified every case
and local link, and found no horizontal overflow. Evidence is in
`tmp/hayden-winter-actions-preview-check.log`,
`tmp/hayden-winter-actions-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test checks
54 literal material boundaries, lower-clothing preservation, full pixels,
per-frame coverage, alpha, metadata and all accepted outputs. All 1,052 variants
passed exact recipe validation. Results are recorded in
`tmp/hayden-winter-actions-test.log` and
`tmp/hayden-winter-actions-compare.json`.

See [combined integration](world-winter-actions.md) for shared checks,
native animation verification and installation. Static images do not establish
live animation timing, natural NPC scheduling or state transitions. Artwork
remains ignored; this character slice changes no runtime code.
