# Hayden's Spring riding idle and walk pilot

This pilot adds `ride_idle_1` and `ride_walk` North/South/East strips under
`spr_npc_hayden_specialanimation_spring_`: six sources, 15 source frames and
five native West mirrors. The extended profile has 174 sources and 696
variants. All 168 accepted regions, groups and mappings remain unchanged.
The user accepted the offline artwork review.

Sources remain read-only in `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Spring/`. The full local corpus is
`extracted/hayden-outfit-pilot-study`. Raw sidecars are recorded in
`tmp/hayden-outfit-pilot-metadata.json`: all frames retain 80×80 geometry,
Default atlas and Middle/54 origin. Idle retains implicit single-frame
defaults; walk retains four frames at scalar duration `0.125`.

The horse, mane/tail, saddle and gear are baked into every strip. They remain
original, including the saddle's skin-like `#E7BA83`. No new skin shade was
needed. The six masks use 250 component seeds, selecting 489 skin pixels
per target while excluding 135 matching shirt-shadow pixels. For example,
idle North `[35,26]` changes while neighboring shirt `[36,26]` stays original;
idle East `[36,29]` changes while shirt `[33,29]` stays original. The North
view exposes only small arm/hand components; no hair changes.

All 15 source frames were inspected in all four targets on
`tmp/hayden-outfit-pilot-art-*.png`. Their 48 PNG/metadata files match
`generated/hayden-outfit-pilot-trial` byte for byte. Component evidence and
frozen hashes are in `tmp/hayden-outfit-pilot-refinement.json` and
`tmp/hayden-outfit-pilot-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-outfit-pilot-preview/summary.png):
  East, North and South samples, about 61 KB.
- [Complete Vanilla/Blue review](../../generated/hayden-outfit-pilot-preview/blue-review/index.html):
  all 20 cases across three pages containing 8, 8 and 4 cases, about 652 KB.

Exact checks cover the full horse/rider crop `[22,8,37,50]`, 7,400,000
full-review pixels, 444,000 summary pixels, 15 sample bindings and all metadata.
Chromium decoded every image and checked every case and link. The focused
opt-in corpus test and targeted Clippy pass: 30 literal boundaries protect
skin, shirt folds, horse and gear, plus the entire lower horse region.
All 1,680 prior original/variant PNG and metadata files remain byte-identical.
Evidence: `tmp/hayden-outfit-pilot-{preview-check,corpus,preservation,art-binding}.log`
and `tmp/hayden-outfit-pilot-preview-browser-check.json`.

See [the combined integration](world-outfit-pilots.md) for full checks, native
probes and isolated installation. Static review does not exercise live timing,
schedules or state/outfit transitions. Game artwork remains ignored, and
this pilot changes no runtime code.
