# Hayden's Autumn action, sleep and kiss

Five regular Autumn strips add general action North/South/East, sleep East
and kiss East. The 26 source frames produce 38 review cases with 12 native
West mirrors. The profile now covers 238 sources and 952 variants; all 233
accepted region objects, groups and mappings remain unchanged. The user
approved this slice for commit on 2026-09-25.

Sources remain read-only in `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Autumn/`, with the
`spr_npc_hayden_autumn_` prefix. The full corpus is
`extracted/hayden-autumn-standard-study`; raw sidecars are recorded in
`tmp/hayden-autumn-standard-metadata.json` and match the independent archive
inventory. Every frame retains 80×80 geometry, Default atlas and Middle/54
origin. Native `action` keeps seven frames at
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; `kiss` keeps four at
`[0.15,0.15,0.8,0.15]`; `sleep` retains single-frame defaults.

The 335 reviewed component seeds select 801 skin pixels per target. No new
shade or mapping was needed. The isolated `#E8B271` cheek at zero-based kiss
frame 2 `[41,35]` uses its existing portrait-highlight mapping. That frame's
purple sleeve pixels `[39,38]` and `[38,39]` remain original, as do sleep's
`[37,38]` and `[38,39]`. Hair, beard, mouth, shirt, trousers and boots remain
original throughout.

All 26 frames were inspected in all four targets on ignored
`tmp/hayden-autumn-standard-art-*.png` sheets. Their 40 target PNG/metadata
files match `generated/hayden-autumn-standard-trial` byte for byte.
Component decisions and frozen hashes are recorded in
`tmp/hayden-autumn-standard-refinement.json` and
`tmp/hayden-autumn-standard-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-autumn-standard-preview/summary.png):
  action East/North, kiss East and sleep East samples.
- [Complete Vanilla/Blue review](../../generated/hayden-autumn-standard-preview/blue-review/index.html):
  all 38 cases and 76 views across five pages of at most eight cases.

Verification covers 2,330 prior PNG/metadata files, full crop
`[28,23,25,34]`, 6,460,000 exact review pixels, 272,000 summary pixels,
20 sample bindings, unchanged metadata and native West reversal. The
focused corpus test and targeted Clippy passed. The test checks 30 literal
material landmarks, full pixels, per-frame coverage, alpha, metadata and all
prior outputs. Chromium decoded every image and verified all cases and
links. Evidence is in
`tmp/hayden-autumn-standard-{compare,art-binding,preview-check,test}.log`
and `tmp/hayden-autumn-standard-preview-browser-check.json`.

See [combined integration](world-autumn-standard.md) for full checks,
native probes and isolated installation. Static review does not exercise
live timing or state transitions. Artwork remains ignored; this character
slice changes no runtime code.
