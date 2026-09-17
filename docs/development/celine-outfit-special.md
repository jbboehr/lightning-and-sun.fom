# Celine's remaining Spring garden animations

This slice adds garden kiss East, shocked start/loop/end South, watering East
and harvesting East. These six strips finish the Spring garden folder:
21 source frames, 13 distinct images and 18 native West mirrors, giving
39 direction/frame review cases.

## Sources and masks

Sources are under `assets/animations/NPCs/Celine/Sprites/Spring/`. Kiss and
shocked use `spr_npc_celine_spring_garden_`; watering and harvesting use
`spr_npc_celine_specialanimation_spring_garden_`. All frames are 80×80, Default
atlas, Middle/54 origin. Harvest has ten frames, watering and kiss four each,
and shocked phases one each. Exact durations and sidecars are recorded in
`tmp/celine-outfit-special-metadata.json`.

The read-only source remains `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The profile grows from 230 to 236 regions; all prior regions, seven color
groups and target mappings remain unchanged. No new source colors were needed.
World shades `FCD9B3`, `F0B988`, `D37A57` and reviewed `672115` outlines retain
their light/middle/shadow/deep roles.

Moving hands, raised fingers, facial edges and the two exposed leg pixels in
the shocked loop recolor. The golden watering can, eyes and mouth interior,
hair/braid, scarf, clothing, belt and boots stay original. Same-color braid
components need separate treatment: watering frame two `[37,39]`, `[38,41]`
and `[38,44]` remain hair, while `[47,38]` is a face edge. Harvest frame three
`[50,51]` and `[46,52]` are hand shadows; `[40,47]` is hair and `[36,52]` is
a boot shadow. Coordinates are frame-local and frame numbers start at one.

The 253 new seeds select 1,010 pixels per target: 782 core skin pixels and
228 outline pixels. Another 91 matching outline pixels remain protected.
Harvest/water/kiss select 514/157/174 pixels; shocked start/loop/end select
57/51/57. Every frame contains selected skin, ranging from 35 to 57 pixels.

Profile SHA-256:
`131b6d08b459cbe8b8f73ed80960060e01a9e2bd89c189d209229153df5104ba`.

## Review and evidence

All 21 source frames and all four targets were inspected. The focused opt-in
`tests/celine_outfit_special.rs` passes four literal target ramps, 39 independent
skin/material landmarks, every new pixel, alpha, metadata, complete core shade
coverage and common selection. All 944 variants validate; the standalone
bundle matches the inspected candidate. All 2,300 prior original/variant PNG
and metadata files remain byte-identical, and 32 duplicate-target checks pass.

- [Five-choice summary](../../generated/celine-outfit-special-preview/summary.png):
  kiss, shocked, harvest and watering samples.
- [Complete Vanilla/Blue review](../../generated/celine-outfit-special-preview/blue-review/index.html):
  all 39 direction/frame cases on six pages, at most eight cases per page.

Fresh crop `[27,23,37,34]` preserves every visible pixel, including the full
watering spout and raised hands. Exact checks cover 9,812,400 full-review
pixels, 226,440 summary pixels, sample/palette/source bindings, original
metadata and West reversal. Chromium checks every page, image and link.
Evidence is under `tmp/celine-outfit-special-`, especially `final-audit.json`,
`test-final.log`, `mask-decisions.json`, `preservation.log`, `duplicates.log`,
`preview-pixel-check.log` and `preview-browser-check.json`.

The user accepted the offline artwork on 2026-09-16. Static review does not demonstrate live
timing, automatic outfit changes or separately rendered effects/held items.
Shared native probes, installation and full-check evidence belong in
[the batch notes](world-outfit-special.md). Extracted artwork, generated
previews and local evidence remain ignored.
