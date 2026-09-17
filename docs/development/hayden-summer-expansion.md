# Hayden's Summer idle and walk pilot

This batch adds six Summer strips: idle and walk North, South and East.
The 15 source frames produce 20 direction/frame cases with five native West
mirrors. The profile now covers 192 sources and 768 variants. All 186 accepted
region objects, color groups and target mappings remain unchanged. User
artwork acceptance was received on 2026-09-16.

Sources remain read-only in `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Summer/`, with the
`spr_npc_hayden_summer_` prefix. The full local corpus is
`extracted/hayden-summer-expansion-study`. Raw sidecars are recorded in
`tmp/hayden-summer-expansion-metadata.json`: 80×80 frames, Default atlas,
Middle/54 origin. Idle retains defaults; walk retains four frames at `0.15`.

Reviewed masks use 207 component seeds, selecting 519 skin pixels per target
while excluding 14 matching shirt-shadow pixels. No new shade was needed.
The plaid shirt, straw hat, hair, beard, cuffs, trousers and boots remain
original. Summer idle East's `[43,44]` is shirt even though the accepted
Spring pixel at that location is skin. Zero-based walk East frame 1
`[35,43]` and frame 3 `[44,42]` are skin; `[43,45]` remains shirt in both
frames. These boundaries are tested literally.

All 15 frames were inspected in all four targets on
`tmp/hayden-summer-expansion-art-*.png`. Their 48 target PNG/metadata files
match `generated/hayden-summer-expansion-trial` byte for byte. Component and
freeze evidence is in `tmp/hayden-summer-expansion-refinement.json` and
`tmp/hayden-summer-expansion-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-summer-expansion-preview/summary.png):
  three samples, about 44 KB.
- [Complete Vanilla/Blue review](../../generated/hayden-summer-expansion-preview/blue-review/index.html):
  all 20 cases across three pages of at most eight, about 440 KB.

Checks cover full crop `[29,24,22,34]`, 2,992,000 exact review pixels,
179,520 summary pixels, all 15 sample bindings, metadata and West reversal.
Chromium decoded all images and verified every case and link. The focused
corpus test and targeted Clippy pass: 33 literal landmarks, full pixel checks,
per-frame coverage and 1,860 unchanged prior PNG/metadata files. Evidence:
`tmp/hayden-summer-expansion-{preview-check,corpus,preservation,art-binding}.log`
and `tmp/hayden-summer-expansion-preview-browser-check.json`.

See [the combined integration](world-summer-expansion.md) for full checks,
native probes and isolated installation. Static review does not exercise
live timing or state transitions. Artwork remains ignored; no runtime code
changes in this character slice.
