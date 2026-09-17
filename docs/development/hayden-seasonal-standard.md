# Hayden's Summer action, sleep and kiss

This batch adds five regular Summer strips: action North/South/East, sleep
East and kiss East. The 26 source frames produce 38 direction/frame cases
with 12 native West mirrors. The profile now covers 208 sources and 832
variants; all 203 accepted region objects, groups and mappings remain
unchanged. The user accepted this offline artwork on 2026-09-16.

Sources remain read-only in `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Summer/`, with the
`spr_npc_hayden_summer_` prefix. The full corpus is
`extracted/hayden-seasonal-standard-study`; raw sidecars are recorded in
`tmp/hayden-seasonal-standard-metadata.json`. All frames retain 80×80 geometry,
Default atlas and Middle/54 origin. Action keeps seven frames at
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss keeps four at
`[0.15,0.15,0.8,0.15]`; sleep retains defaults.

The 341 reviewed component seeds select 852 skin pixels per target and
exclude 18 matching dark shirt-seam pixels. No new shade or mapping was
needed. The isolated `#E8B271` cheek at zero-based kiss frame 2 `[41,35]`
uses the existing portrait highlight mapping. That frame's `[39,44]` and
`[45,44]` stay shirt, as does sleep's `[37,44]`. The plaid shirt, straw hat,
cuffs, hair, beard and other non-skin colors remain original.

All 26 frames were inspected in all four targets on
`tmp/hayden-seasonal-standard-art-*.png`. Their 40 target PNG/metadata files
match `generated/hayden-seasonal-standard-trial` byte for byte. Component and
freeze evidence is in `tmp/hayden-seasonal-standard-refinement.json` and
`tmp/hayden-seasonal-standard-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-seasonal-standard-preview/summary.png):
  four samples, about 56 KB.
- [Complete Vanilla/Blue review](../../generated/hayden-seasonal-standard-preview/blue-review/index.html):
  all 38 cases across five pages of at most eight, about 784 KB.

Checks cover full crop `[28,23,25,34]`, 6,460,000 exact review pixels,
272,000 summary pixels, 20 sample bindings, metadata and West reversal.
Chromium decoded every image and verified all cases and links. The focused
corpus test and targeted Clippy pass: 26 literal landmarks including the
kissing cheek, full pixel checks, per-frame coverage and all 2,030 prior
PNG/metadata files. Evidence:
`tmp/hayden-seasonal-standard-{preview-check,corpus,preservation,art-binding}.log`
and `tmp/hayden-seasonal-standard-preview-browser-check.json`.

See [the combined integration](world-seasonal-standard.md) for full checks,
native probes and isolated installation. Static review does not exercise
live timing or state transitions. Artwork remains ignored; this character
slice changes no runtime code.
