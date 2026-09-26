# Hayden's Beach idle and walk

Six strips add Beach idle and walk North/South/East. Their 15 source frames
produce 20 review cases with five native West mirrors. The profile now covers
282 sources and 1,128 variants. All 276 accepted regions, color groups and
palette mappings remain unchanged. The user approved this slice for commit on
2026-09-25.

The read-only source is `tmp/fields-of-mistria/assets.zip`, unchanged from the
accepted Winter finish batch, with SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Sources live under `assets/animations/NPCs/Hayden/Sprites/Beach/` with the
`spr_npc_hayden_beach_` prefix. The full corpus is
`extracted/hayden-beach-pilot-study`; raw sidecars are recorded in
`tmp/hayden-beach-pilot-author-metadata.json`. All frames retain 80×80 geometry,
Default atlas and Middle/54 origin. Idle retains the native single-frame
defaults; walking retains four frames at 0.15 seconds each. All 2,760 accepted
original/variant PNG and metadata files still match byte for byte. No old
source hash was changed or relaxed.

The 421 reviewed component seeds select 1,239 skin pixels per target. These
six Beach strips use the existing four world shades exclusively for skin:
647 highlight, 392 midtone, 164 shadow and 36 darkest-detail pixels. The exposed
torso, shoulders, hands, legs and feet all change, including the darker torso
shading and forearm details. The `#523C26` detail across idle South's forearm at
`[32,44]` follows the accepted Spring sprite treatment. This keeps body shading
consistent with the earlier portrait/body-hair choice without changing any
mapping or older selection.

The large tan area in the North view is the straw hat carried on his back;
it stays original. Its `#DFC6A1` fill, `#B68D54` rim and `#84533E` edging are
distinct material colors, as are the orange swimwear and pale drawstrings.
Head hair, beard, eyes and outlines also stay original. No seasonal mask was
transferred onto the Beach artwork. Every actual frame was inspected in
Vanilla and all four targets using ignored
`tmp/hayden-beach-pilot-author-art-*.png` sheets and an exact material-color
grid. The final sheets and previews use the actual standalone bundle.

- [Five-choice summary](../../generated/hayden-beach-pilot-preview/summary.png):
  idle North/South and walk East/South samples.
- [Complete Vanilla/Blue review](../../generated/hayden-beach-pilot-preview/blue-review/index.html):
  all 20 cases and 40 views across three pages of at most eight cases.

The complete preview directory is about 356 KiB. Crop `[31,26,18,30]` contains
every opaque pixel, including moving feet and hat edges. Saved-image checks
cover 3,345,300 exact pixels across all-target art sheets, review sheets and
20 summary bindings, including source/palette identity and West reversal.
Chromium decoded every image across all five HTML pages, verified every case
and local link, and found no horizontal overflow. Evidence is in
`tmp/hayden-beach-pilot-author-preview-check.log`,
`tmp/hayden-beach-pilot-author-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed. The test checks
65 literal skin/material landmarks, every source pixel against the independently
reviewed material ramp, per-frame coverage, alpha, metadata and all accepted
outputs. It accepts `FOM_HAYDEN_BEACH_PRESETS` for ignored negative-control
copies. Removing the bare-leg component failed at idle South `[38,49]`;
selecting the straw-hat fill failed at `[33,37]`. Both failures occurred in
the material assertion after successful generation. Production data remained
unchanged throughout. Evidence is in
`tmp/hayden-beach-pilot-author-{omit-leg,spill-hat}-red.log`.

All 1,128 variants passed exact recipe validation. The production test and
comparison reports are `tmp/hayden-beach-pilot-author-test.log` and
`tmp/hayden-beach-pilot-author-compare.json`. Frozen inputs are recorded in
`tmp/hayden-beach-pilot-author-frozen-inputs.sha256`.

See [combined integration](world-beach-pilot.md) for shared checks, native
animation verification and installation. Static images do not establish live
animation timing, natural NPC scheduling or state transitions. Artwork stays
ignored; this character slice changes no runtime code.
