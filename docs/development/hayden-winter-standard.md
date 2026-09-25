# Hayden's Winter action, sleep and kiss

Five strips add Winter general action North/South/East, sleep East and kiss
East. The 26 source frames produce 38 review cases with 12 native West mirrors.
The profile now covers 268 sources and 1,072 variants. All 263 accepted regions,
color groups and palette mappings remain unchanged. The user approved this slice for commit on 2026-09-25.

The read-only source is `tmp/fields-of-mistria/assets.zip`, unchanged from the
accepted Winter actions batch, under
`assets/animations/NPCs/Hayden/Sprites/Winter/` with the
`spr_npc_hayden_winter_` prefix. The full corpus is
`extracted/hayden-winter-standard-study`; raw sidecars are recorded in
`tmp/hayden-winter-standard-metadata.json`. All frames retain 80×80 geometry,
Default atlas and Middle/54 origin. Action retains seven frames at
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss retains four at
`[0.15,0.15,0.8,0.15]`; sleep retains single-frame defaults. All 2,630 accepted
original/variant PNG and metadata files still match byte for byte; no old
source hash was changed.

The 243 reviewed component seeds select 533 skin pixels per target and protect
87 matching-color coat pixels. Autumn mask propagation initially selected 19
Winter coat pixels and missed 23 neckline/finger pixels. The final masks fix
both boundaries. In particular, the raised North-facing arm remains covered
by the Winter sleeve, while only the lowered hands change. In zero-based
action South frame 2, the `#AB7E3F` pixels at `[38,43]` and `[36,45]` lie on the
coat side of its cream cuff and remain original; the nearby hand at `[39,44]`
changes. The `#AB7E3F` sleeve shadow at kiss frame 2 `[38,42]` also stays original.
The isolated `#E8B271` kissing-cheek highlight at `[41,35]` retains its existing
portrait-highlight mapping. Gold coat, cream cuffs, blue shirt, trousers,
boots, hair, beard and mouth remain original. No new shade or mapping was needed.

All 26 source frames were inspected in Vanilla and all four targets on ignored
`tmp/hayden-winter-standard-art-*.png` sheets. Enlarged comparisons with the
Autumn source and exact material-color grids resolved the coat/cuff collisions.
The final sheets and previews use the actual standalone bundle.

- [Five-choice summary](../../generated/hayden-winter-standard-preview/summary.png):
  action East/North, kiss East and sleep East samples.
- [Complete Vanilla/Blue review](../../generated/hayden-winter-standard-preview/blue-review/index.html):
  all 38 cases and 76 views across five pages of at most eight cases.

The full crop `[30,25,21,30]` includes every opaque pixel. Saved-image checks
cover 7,037,100 exact pixels across all-target art sheets, review sheets and
the 20 summary bindings, including source/palette identity and West reversal.
Chromium decoded every image across all seven HTML pages, verified every case
and local link, and found no horizontal overflow. Evidence is in
`tmp/hayden-winter-standard-preview-check.log`,
`tmp/hayden-winter-standard-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test checks
43 literal material boundaries, broader North-facing sleeve and lower-clothing
preservation, full pixels, per-frame coverage, alpha, metadata and all accepted
outputs. All 1,072 variants passed exact recipe validation. Results are recorded
in `tmp/hayden-winter-standard-test.log` and
`tmp/hayden-winter-standard-compare.json`.

See [combined integration](world-winter-standard.md) for shared checks,
native animation verification and installation. Static images do not establish
live animation timing, natural NPC scheduling or state transitions. Artwork
remains ignored; this character slice changes no runtime code.
