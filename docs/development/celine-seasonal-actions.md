# Celine's Summer blinking, sitting, eating and drinking

This slice adds eleven regular Summer strips: blink South/East and sit/eat/drink
North/South/East. Their 31 source frames contain 22 distinct images. Native West
mirroring adds twelve review cases. Other Summer actions remain outside this slice.

Sources are `spr_npc_celine_summer_{cycle}_{direction}.png` under
`assets/animations/NPCs/Celine/Sprites/Summer/`, from read-only
`tmp/momi-lab/assets.bak.zip` (SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`).
All frames are 80×80, Default atlas, Middle/54 origin. Blink has three frames,
sit one, drink three, and eat three North or five South/East. Exact original
sidecars, including frame durations, are in `tmp/celine-seasonal-actions-metadata.json`.

The profile grows from 242 to 253 regions. Earlier regions, seven color groups
and source/target mappings remain unchanged. Existing world shades `FCD9B3`,
`F0B988`, `D37A57` and selected `672115` components retain their
light/middle/shadow/deep roles; no new colors were needed.

Exposed forearms, armpits, fingertips, seated legs and toes recolor. The long
hair, pink outfit, belt, scarf, eyes, sandal straps and mouth interiors remain
original. In East eating, dark `[39,43]` in frame one and `[40,43]` in frame two
are exposed arm contours; frame two `[43,44]` belongs to the belt. In South
eating, `[36,43]` in frame one and `[38,44]` in frame two follow bare arms;
`[38,42]` in frame two borders the scarf and stays original. Coordinates are
frame-local, with frame numbers starting at one.

The 453 new seeds select 1,433 pixels per target: 1,130 core skin pixels and
303 outline pixels. Another 196 matching outline pixels stay protected. Every
frame has selected skin, ranging from two to 80 pixels; North eating/drinking
can expose only a tiny hand below the hair. Profile SHA-256:
`d444899a66765bde81aaa182fe154d70bcf68fe83dd57853ccbcf1e2e03acae4`.

All source frames and four targets were inspected. The focused opt-in
`tests/celine_seasonal_actions.rs` passes four literal target ramps, 45 source-art
landmarks, every new pixel, alpha, metadata, complete core shade coverage and
common selection. All 1,012 variants validate. The standalone bundle matches
the inspected candidate, all 2,420 previous original/variant PNG and metadata
files remain byte-identical, and all 36 duplicate-target comparisons pass.

- [Five-choice summary](../../generated/celine-seasonal-actions-preview/summary.png):
  one sample from each action in Vanilla, Debug Blue, Hayden, Ryis and Seridia.
- [Complete Vanilla/Blue review](../../generated/celine-seasonal-actions-preview/blue-review/index.html):
  all 43 direction/frame cases on eight pages, at most eight cases per page.

Fresh crop `[28,23,23,34]` preserves every visible pixel. Exact checks cover
6,725,200 full-review pixels, 250,240 summary pixels, all source/palette/sample
bindings, original metadata and West reversal. Chromium verifies all eight
pages, images and links. Local evidence is under `tmp/celine-seasonal-actions-`,
including `final-audit.json`, `test.log`, `mask-decisions.json`,
`preview-check.log` and `preview-browser-check.json`.

The user accepted the offline artwork on 2026-09-16. Static review does not exercise live outfit
switching, timing or separately drawn food/drink objects. Shared native,
installation and full-check evidence belongs in
[the seasonal actions batch notes](world-seasonal-actions.md). Extracted artwork,
generated previews and local evidence remain ignored.
