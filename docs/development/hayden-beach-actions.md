# Hayden's Beach blink, action and kiss

Six strips add Beach blink East/South, general action North/South/East and
kiss East. Their 31 source frames produce 45 review cases with 14 native West
mirrors. The profile now covers 288 sources and 1,152 variants. All 282
accepted regions, color groups and palette mappings remain unchanged. The user approved this slice for commit on 2026-09-25.

The read-only source is `tmp/fields-of-mistria/assets.zip`, unchanged from the
accepted Beach pilot batch, with SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Sources live under
`assets/animations/NPCs/Hayden/Sprites/Beach/` with the
`spr_npc_hayden_beach_` prefix. The full corpus is
`extracted/hayden-beach-actions-study`; raw sidecars are recorded in
`tmp/hayden-beach-actions-author-metadata.json`. All frames retain 80×80
geometry, Default atlas and Middle/54 origin. General action retains seven
frames at `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; blink retains three at
`[0.075,0.125,0.075]`; kiss retains four at `[0.15,0.15,0.8,0.15]`.
No old source hash was changed or relaxed.

The 917 reviewed component seeds select 2,623 skin pixels per target. These
six strips use the existing four world shades exclusively for skin: 1,407
highlight, 772 midtone, 351 shadow and 93 darkest-detail pixels. Exposed
torso, moving arms and fingers, kissing cheek, legs and feet change. Dark
forearm and body shading keep the accepted Beach pilot treatment. In
zero-based action North frame 0, the tiny `#815A2E` hand pixel at `[32,45]`
changes independently of the large straw hat; the hat fill at `[39,38]`
stays `#DFC6A1`. The Beach kissing cheek uses the world ramp, with
`#AB7E3F` at frame 2 `[41,35]`, so no extra shade or mapping is needed.

The carried straw hat, its rim and edging, orange swimwear, pale drawstrings,
head hair, beard, eyes and outlines remain original. Every actual frame was
inspected in Vanilla and all four targets on ignored
`tmp/hayden-beach-actions-author-art-*.png` sheets and an exact material-color
grid. Earlier seasonal actions were references for pose boundaries; their
clothing masks were not transferred onto bare Beach skin.

- [Five-choice summary](../../generated/hayden-beach-actions-preview/summary.png):
  action North/South/East and kiss samples.
- [Complete Vanilla/Blue review](../../generated/hayden-beach-actions-preview/blue-review/index.html):
  all 45 cases and 90 views across six pages of at most eight cases.

The final sheets and previews use the actual standalone bundle. The complete
preview directory is about 728 KiB. Crop `[30,25,21,30]` includes every opaque
pixel. Saved-image checks cover 8,312,850 exact pixels across all-target art
sheets, review sheets and 20 summary bindings, including source/palette identity
and West reversal. Chromium decoded every image across all eight HTML pages,
verified every case and local link, and found no horizontal overflow. Evidence
is in `tmp/hayden-beach-actions-author-preview-check.log`,
`tmp/hayden-beach-actions-author-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed before the candidate
profile was published to the production path. The test checks 66 literal
skin/material landmarks, every source pixel against the independently reviewed
material ramp, every frame, alpha, metadata and all accepted outputs. The
production profile was then checked for exact object equality with that
passing candidate; all 282 prior region objects and mappings remain unchanged.

The test accepts `FOM_HAYDEN_BEACH_ACTION_PRESETS` for ignored negative-control
copies. Removing the isolated hand component failed at action North `[32,45]`;
selecting the straw-hat fill failed at `[38,37]`. Both failures occurred in
the material assertion after successful generation. Production data was
untouched while these controls ran. Evidence is in
`tmp/hayden-beach-actions-author-candidate-test.log` and
`tmp/hayden-beach-actions-author-{omit-hand,spill-hat}-red.log`.

All 1,152 variants passed exact recipe validation, and all 2,820 accepted
original/variant PNG and metadata files remain byte-identical. The comparison
report is `tmp/hayden-beach-actions-author-compare.json`. The focused test also
passed against the final production recipe, recorded in
`tmp/hayden-beach-actions-author-test.log`.

Frozen inputs are recorded in
`tmp/hayden-beach-actions-author-frozen-inputs.sha256`. See
[combined integration](world-beach-actions.md) for shared checks, native
animation verification and installation. Static images do not establish live
animation timing, natural NPC scheduling or state transitions. Artwork stays
ignored; this character slice changes no runtime code.
