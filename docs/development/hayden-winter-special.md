# Hayden's Winter tools and seated reading

Eight strips finish Hayden's 30 Winter sources: hammer, harvest, till and water
East; wipebrow South; and seated-reading start, loop and end South. Their 41
source frames produce 66 review cases with 25 native West mirrors. The profile
now covers 276 sources and 1,104 variants. All 268 accepted regions, color
groups and palette mappings remain unchanged. The user approved this slice for
commit on 2026-09-25.

The read-only source is `tmp/fields-of-mistria/assets.zip`, unchanged from the
accepted Winter standard batch, with SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Sources live under `assets/animations/NPCs/Hayden/Sprites/Winter/`, with the
`spr_npc_hayden_specialanimation_winter_` prefix. The full corpus is
`extracted/hayden-winter-special-study`; raw sidecars are recorded in
`tmp/hayden-winter-special-metadata.json`. All frames retain 80×80 geometry,
Default atlas and Middle/54 origin. No source hash was relaxed or refreshed.
All 2,680 accepted original/variant PNG and metadata files still match byte
for byte.

Native metadata remains unchanged: hammer has six frames, harvest ten, till
five, water four, wipebrow six, and seated reading start/loop/end three/four/
three. Their timing arrays are respectively `[0.1,0.2,0.1,0.1,0.1,0.1]`,
`[0.4,0.15,0.2,0.2,0.2,0.2,0.2,0.2,0.15,1.4]`,
`[0.125,0.125,0.125,0.125,1.2]`, `[0.15,1,0.15,2.5]`,
`[0.15,0.15,0.45,0.1,0.075,1]`, and `0.1`/`[3,0.1,3,0.1]`/`0.1`.

The 439 reviewed component seeds select 1,091 skin pixels per target and
protect 81 matching-color coat and seam pixels. Comparing against Autumn
gave an initial candidate, then material inspection added 33 exposed-neckline
pixels. The gold Winter sleeve uses the same `#AB7E3F` and `#815A2E` as skin,
so those colors cannot be selected globally. In zero-based hammer frame 2,
the sleeve at `[40,42]` and `[41,42]` stays original; in till frame 3 the
`#AB7E3F` at `[38,44]` stays on the coat side of the cream cuff. By contrast,
the small reading fingers, water-can grip and exposed neckline below the beard
change. The raised wipebrow arm remains sleeved. Gold coat, cream cuffs, blue
shirt, trousers, boots, hair, beard, mouth, book and tools remain original.
No new source shade or target mapping was needed.

Every source frame was inspected in Vanilla and all four targets on ignored
`tmp/hayden-winter-special-art-*.png` sheets. Enlarged comparisons against the
Autumn source and exact material-color grids resolved cuff and sleeve edges.
The final sheets and previews use the actual standalone bundle.

- [Five-choice summary](../../generated/hayden-winter-special-preview/summary.png):
  seated reading, raised hammer, harvest and water samples.
- [Complete Vanilla/Blue review](../../generated/hayden-winter-special-preview/blue-review/index.html):
  all 66 cases and 132 views across nine pages of at most eight cases.

The common crop `[29,22,48,42]` includes every opaque pixel, including tool
extents. Saved-image checks cover 37,588,320 exact pixels across all-target art
sheets, review sheets and 20 summary bindings, including source/palette
identity and West reversal. Chromium decoded every image across all 11 HTML
pages, verified every case and local link, and found no horizontal overflow.
Evidence is in `tmp/hayden-winter-special-preview-check.log`,
`tmp/hayden-winter-special-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test checks
80 literal skin/material landmarks, full pixels, per-frame coverage, alpha,
metadata and all accepted outputs. All 1,104 variants passed exact recipe
validation. Results are recorded in `tmp/hayden-winter-special-test.log` and
`tmp/hayden-winter-special-compare.json`.

See [combined integration](world-winter-special.md) for shared checks, native
animation verification and installation. Static images do not establish live
animation timing, natural NPC scheduling or state transitions. Artwork stays
ignored; this character slice changes no runtime code.
