# Hayden's shocked reactions and seated reading

This batch adds six Spring strips: normal `shocked_{start,loop,end}_south` and
special `read_sit_{start,loop,end}_south`. Their 13 source frames are native
South-only artwork. The extended profile has 161 sources and 644 variants;
all 155 accepted region objects, color groups and mappings remain unchanged.
The user accepted the offline artwork review.

Sources are read-only in `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Spring/`. Reading uses the
`spr_npc_hayden_specialanimation_spring_` prefix; shocked uses
`spr_npc_hayden_spring_`. The full local corpus is
`extracted/hayden-reactions-study`. Raw sidecars and phase/frame inventory are in
`tmp/hayden-reactions-metadata.json`. All frames are 80×80, Default atlas,
Middle/54 origin. Reading start/end contain three frames at `0.1`; its four-frame
loop uses `[3.0,0.1,3.0,0.1]`. Each shocked phase uses implicit single-frame
engine defaults.

The additions use 224 component seeds, selecting 510 skin pixels per target
and excluding 106 matching material pixels. Book pages and cover, mouth, beard,
hair and shirt folds stay original. The shocked loop's two `E8B271` cheek
pixels use the existing portrait highlight mapping. Its sleeve-edge pixels
`[35,35]` and `[44,35]` remain original beside selected forearm shadows. All 13
frames were inspected in every target on `tmp/hayden-reactions-art-*.png`;
all 52 target frames match `generated/hayden-reactions-trial` byte for byte.
Component evidence and frozen hashes are in `tmp/hayden-reactions-refinement.json`
and `tmp/hayden-reactions-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-reactions-preview/summary.png):
  shocked and reading samples, about 51 KB.
- [Complete Vanilla/Blue review](../../generated/hayden-reactions-preview/blue-review/index.html):
  all 13 frames across three pages containing 3, 7 and 3 cases. Preview files
  occupy about 384 KB.

Exact checks cover the full-art crop `[27,20,26,37]`, 2,501,200 full-review pixels,
519,480 summary pixels, 15 sample bindings and source/variant metadata. Chromium
decoded all images and checked every case and link. The focused opt-in corpus
test and Clippy pass; 24 literal landmarks cover skin, shirt edges, book/page
colors, cheeks, mouth and beard. Exact comparison preserves all 1,550 prior
original/variant PNG and metadata files from the previous standard bundle.
Evidence: `tmp/hayden-reactions-preview-pixel-check.log`,
`tmp/hayden-reactions-preview-browser-check.json`,
`tmp/hayden-reactions-focused-test.log` and
`tmp/hayden-reactions-comparison.log`.

See [the combined integration](spring-world-reactions.md) for full checks,
native sequence probes and isolated installation. Static review does not
exercise live timing, natural schedules or state/outfit changes. Game artwork
remains ignored, and this data slice changes no Rust or GML runtime code.
