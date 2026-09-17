# Celine's Summer reading, sweeping and watering

This slice adds ten regular Summer strips: standing and seated reading and
sweeping start/loop/end South, plus watering East. Their 41 source frames contain
33 distinct images; native West watering adds four review cases. Summer garden
outfit assets remain outside this slice.

Sources are `spr_npc_celine_specialanimation_summer_*.png` under
`assets/animations/NPCs/Celine/Sprites/Summer/`, from read-only
`tmp/momi-lab/assets.bak.zip` (SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`).
All frames are 80×80, Default atlas, Middle/54 origin. Exact sidecars are retained
in `tmp/celine-seasonal-special-metadata.json`.

Reading uses three start, four loop and three end frames, with loop durations
`[3.0,0.1,3.0,0.1]` and 0.1-second start/end frames. Sweeping uses one start,
fifteen loop and one end frame; its variable loop timing remains unchanged.
Watering has four frames with durations `[0.15,1.0,0.15,2.5]`. Native reading and
sweeping cycles are complex; watering is linear. Seated reading pauses into
`sit`, while the other three cycles pause into `idle`.

The profile grows from 258 to 268 regions. All prior regions, seven color groups
and source/target mappings remain unchanged. Existing world shades `FCD9B3`,
`F0B988`, `D37A57` and selected `672115` components retain their
light/middle/shadow/deep roles; no new colors were needed.

Masks cover face edges, bare Summer arms, moving fingers, calves and exposed
toes. Shared dark hair, belt and sandal components stay original, as do clothing,
book pages and binding, broom, dust and watering can. The Summer arm shadows
include watering frame one's `[38,44]`, sweeping frame seven's `[38,45]`, and
final sweeping frame's `[43,42]` and `[43,44]`. Watering frame two's `[45,43]`
remains the garment edge beside the can, matching its clothed Spring counterpart.
Coordinates are frame-local; frame numbers here start at one.

The 623 new seeds select 2,149 pixels per target: 1,702 core skin and 447 outline
pixels. Another 64 matching outline pixels stay protected. Every frame has
30–76 selected pixels. Profile SHA-256:
`771a4f70b0e093dda212c0cb6fa46b67630c5088dedab66903e143983cd1072f`.

All source frames and four targets were inspected. The focused opt-in
`tests/celine_seasonal_special.rs` passes 43 literal source-art landmarks, four
target ramps, every new pixel, alpha, exact metadata, complete core shade
coverage and common selection. All 1,072 variants validate. The standalone
bundle matches the inspected candidate, all 2,580 previous original/variant PNG
and metadata files remain byte-identical, and all 32 duplicate-target checks pass.

- [Five-choice summary](../../generated/celine-seasonal-special-preview/summary.png):
  standing/seated reading, sweeping and watering samples.
- [Complete Vanilla/Blue review](../../generated/celine-seasonal-special-preview/blue-review/index.html):
  all 45 direction/frame cases on eight pages, at most eight cases per page.

Fresh crop `[25,24,39,39]` includes all art, including broom bristles, dust and
the extended watering-can spout. Exact checks cover 13,689,000 full-review pixels,
273,780 summary pixels, all source/palette/sample bindings, original metadata and
West reversal. Chromium verifies all eight pages, 45 images and navigation links.
Evidence is under `tmp/celine-seasonal-special-`, including `final-audit.json`,
`test.log`, `mask-decisions.json`, `preview-check.log` and `preview-browser.json`.

The user accepted this offline artwork on 2026-09-16. Static review does not
exercise live outfit
switching, game timing or separately drawn held items. Shared native,
installation and full-check evidence belongs in
[the seasonal special batch notes](world-seasonal-special.md). Extracted artwork,
generated previews and local evidence remain ignored.
