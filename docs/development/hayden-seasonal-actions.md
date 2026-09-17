# Hayden's Summer blink, sit, eat and drink

This batch adds 11 Summer strips: blink South/East, plus sit, eat and drink
North/South/East. The 31 source frames produce 43 direction/frame cases with
12 native West mirrors. The profile now covers 203 sources and 812 variants;
all 192 accepted region objects, groups and mappings remain unchanged.
The user accepted the offline artwork on 2026-09-16.

Sources remain read-only in `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Summer/`, with the
`spr_npc_hayden_summer_` prefix. The full corpus is
`extracted/hayden-seasonal-actions-study`; raw sidecars are recorded in
`tmp/hayden-seasonal-actions-metadata.json`. All frames retain 80×80 geometry,
Default atlas and Middle/54 origin. Blink keeps `[0.075,0.125,0.075]`;
East/South eat keeps `[0.125,0.15,0.175,0.125,0.6]`; North eat and all drink
keep three frames at `1.0`; sit keeps defaults.

The 486 reviewed component seeds select 1,242 skin pixels per target and
exclude 27 matching dark shirt-seam pixels. No new shade was needed.
The plaid shirt, straw hat, cuffs, hair, beard and mouth reds remain original.
For example, drink South `[36,43]` and `[43,43]` remain shirt. Zero-based eat
South frame 1 reaches exposed skin at `[37,48]` while retaining the shirt at
`[43,43]`. These boundaries and the open mouth are tested literally.

All 31 frames were inspected in all four targets on
`tmp/hayden-seasonal-actions-art-*.png`. Their 88 target PNG/metadata files
match `generated/hayden-seasonal-actions-trial` byte for byte. Component and
freeze evidence is in `tmp/hayden-seasonal-actions-refinement.json` and
`tmp/hayden-seasonal-actions-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-seasonal-actions-preview/summary.png):
  four samples, about 52 KB.
- [Complete Vanilla/Blue review](../../generated/hayden-seasonal-actions-preview/blue-review/index.html):
  all 43 cases across six pages of at most eight, about 876 KB.

Checks cover full crop `[28,23,24,34]`, 7,017,600 exact review pixels,
261,120 summary pixels, 20 sample bindings, metadata and West reversal.
Chromium decoded every image and verified all cases and links. The focused
corpus test and targeted Clippy pass: 34 literal landmarks, full pixel checks,
per-frame coverage and all 1,920 prior PNG/metadata files. Evidence:
`tmp/hayden-seasonal-actions-{preview-check,corpus,preservation,art-binding}.log`
and `tmp/hayden-seasonal-actions-preview-browser-check.json`.

See [the combined integration](world-seasonal-actions.md) for full checks,
native probes and isolated installation. Static review does not exercise live
timing, state transitions or separately rendered food/cup overlays. Artwork
remains ignored; this character slice changes no runtime code.
