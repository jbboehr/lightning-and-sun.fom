# Hayden's Winter idle and walk

Six strips add Winter idle and walk North/South/East. The 15 source frames
produce 20 review cases with five native West mirrors. The profile now covers
252 sources and 1,008 variants. All 246 accepted regions, color groups and
palette mappings remain unchanged. The user approved this slice for commit on 2026-09-25.

The read-only source is `tmp/fields-of-mistria/assets.zip`, unchanged from the
accepted Autumn special batch, under
`assets/animations/NPCs/Hayden/Sprites/Winter/` with the
`spr_npc_hayden_winter_` prefix. The full corpus is
`extracted/hayden-winter-expansion-study`; raw sidecars are recorded in
`tmp/hayden-winter-expansion-metadata.json`. All frames retain 80×80 geometry,
Default atlas and Middle/54 origin. Idle retains single-frame defaults; walk
retains four frames at 0.15 seconds each. All 2,460 accepted original/variant
PNG and metadata files still match byte for byte; no old source hash was changed.

The 165 reviewed component seeds select 320 skin pixels per target and protect
54 matching-color clothing pixels. Winter sleeves reuse skin shades above their
cream cuffs, so Autumn mask propagation alone incorrectly selects parts of the
coat. The final masks remove 13 such sleeve pixels and add 13 exposed-neckline
pixels beneath the beard. Examples include the unchanged `#815A2E` sleeve at
zero-based walk North frame 1 `[46,41]`, the unchanged `#AB7E3F` sleeve at walk
South frame 1 `[45,41]`, and the changed `#E7B172` neckline at idle South
`[39,40]` and `[40,40]`. Hands below the cuffs change; coat seams, cuffs, shirt,
trousers, boots, hair and beard remain original. No new shade or mapping was
needed.

All 15 source frames were inspected in Vanilla and all four targets on ignored
`tmp/hayden-winter-expansion-art-*.png` sheets. Enlarged sleeve comparisons with
the Autumn source helped distinguish matching-color clothing from exposed skin.
The final sheets and previews use the actual standalone bundle.

- [Five-choice summary](../../generated/hayden-winter-expansion-preview/summary.png):
  idle North/South and walk East/South samples.
- [Complete Vanilla/Blue review](../../generated/hayden-winter-expansion-preview/blue-review/index.html):
  all 20 cases and 40 views across three pages of at most eight cases.

The full crop `[31,26,18,30]` includes every opaque pixel. Saved-image checks
cover 3,345,300 exact pixels across all-target art sheets, review sheets and
the 20 summary bindings, including source/palette identity and West reversal.
Chromium decoded every image across all five HTML pages, verified every case
and local link, and found no horizontal overflow. Evidence is in
`tmp/hayden-winter-expansion-preview-check.log`,
`tmp/hayden-winter-expansion-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test checks
42 literal material boundaries, broader coat and lower-clothing preservation,
full pixels, per-frame coverage, alpha, metadata and all accepted outputs.
All 1,008 variants passed exact recipe validation. Results are recorded in
`tmp/hayden-winter-expansion-test.log` and
`tmp/hayden-winter-expansion-compare.json`.

See [combined integration](world-winter-expansion.md) for shared checks,
native animation verification and installation. Static images do not establish
live animation timing, natural NPC scheduling or state transitions. Artwork
remains ignored; this character slice changes no runtime code.
