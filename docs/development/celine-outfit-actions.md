# Celine's garden blinking and sitting

Five strips extend the accepted garden pilot: blink South/East and sit
North/South/East. They contain nine source frames and seven distinct images;
native West mirrors add four review cases. Other garden actions remain outside
this slice.

## Sources and masks

Sources are `spr_npc_celine_spring_garden_{blink,sit}_{direction}.png` under
`assets/animations/NPCs/Celine/Sprites/Spring/`. All frames are 80×80, Default
atlas, Middle/54 origin. Blink has three frames with durations
`[0.075,0.125,0.075]`; sitting uses single-frame defaults. Exact sidecars are in
`tmp/celine-outfit-actions-metadata.json`. The read-only archive remains
`tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

The profile grows from 225 to 230 regions. All prior regions, seven color groups
and target mappings remain unchanged. World shades `FCD9B3`, `F0B988`, `D37A57`
and selected `672115` outlines retain their light/middle/shadow/deep roles.
No new colors were needed.

Closed-eye skin and exposed seated hands recolor; hair/braid, scarf, green
clothing, belt, eyes and boots remain original. The brown seated lower-body
areas are boots. In East sitting, hand outline `[35,47]` recolors, while boot
outlines `[44,47]` and `[41,48]` remain original. North sitting wrist `[36,44]`
recolors while belt `[37,45]` stays unchanged. Coordinates are frame-local.

The 115 new seeds select 381 pixels per target: 283 core skin pixels and
98 outline pixels, with 54 matching outline pixels protected. Blink East/South
select 136/153 pixels; sit East/North/South select 36/18/38. Every source frame
contains selected skin, ranging from 18 to 57 pixels.

Profile SHA-256:
`9aec1425e3ec2f6e105aa7001c4da8807c4ce7dfe66eec7a725d3484024f8bc7`.

## Review and evidence

All nine source frames and all four targets were inspected. The focused opt-in
`tests/celine_outfit_actions.rs` passes four literal target ramps, 25 independent
skin/material landmarks, every new pixel, alpha, metadata, complete core shade
coverage and common selection. All 920 variants validate. The standalone
bundle matches the inspected candidate, all 2,250 prior original/variant PNG
and metadata files remain byte-identical, and eight duplicate-target checks pass.

- [Five-choice summary](../../generated/celine-outfit-actions-preview/summary.png):
  closed-eye South blink and all three sitting directions.
- [Complete Vanilla/Blue review](../../generated/celine-outfit-actions-preview/blue-review/index.html):
  all 13 direction/frame cases on three pages, at most eight cases per page.

Fresh crop `[29,25,22,32]` preserves every visible pixel. Exact checks cover
1,830,400 full-review pixels, 225,280 summary pixels, every sample/palette/source
binding, original metadata and West reversal. Chromium checks every page,
image and link. Evidence is under `tmp/celine-outfit-actions-`, especially
`final-audit.json`, `test-final.log`, `mask-decisions.json`, `preservation.log`,
`duplicates.log`, `preview-pixel-check.log` and `preview-browser-check.json`.

The user accepted the offline artwork review. Static images do not demonstrate live
timing or automatic outfit changes. Shared native probes, installation and
full-check evidence belong in [the batch notes](world-outfit-actions.md).
Extracted artwork, generated previews and local evidence remain ignored.
