# Celine's Summer general actions, sleeping and kissing

This slice adds five regular Summer strips: general action North/South/East,
sleep East and kiss East. Their 26 source frames contain 17 distinct images.
Native West mirroring adds twelve review cases. Remaining Summer animations
are outside this slice.

Sources are `spr_npc_celine_summer_{cycle}_{direction}.png` under
`assets/animations/NPCs/Celine/Sprites/Summer/`, from read-only
`tmp/momi-lab/assets.bak.zip` (SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`).
All frames are 80×80, Default atlas, Middle/54 origin. General action has seven
frames per direction, kiss four and sleep one. Original sidecars and durations
are in `tmp/celine-seasonal-standard-metadata.json`.

The profile grows from 253 to 258 regions. All earlier regions, seven color
groups and source/target mappings remain unchanged. Existing world shades
`FCD9B3`, `F0B988`, `D37A57` and selected `672115` components retain their
light/middle/shadow/deep roles; no new colors were needed.

The masks recolor bare arms, extended fingers, kissing face edges, the sleeping
hand, calves and exposed toes. Long hair, pink clothing, belt, scarf, eye pixels
and sandal straps stay original. In East action frame two, fingertip `[48,45]`
maps while hair `[38,44]`, belt `[44,45]` and sandal edge `[44,52]` stay original.
The sleeping hand edge `[44,39]` maps while the dark hair component beginning
at `[36,43]` stays original. Coordinates are frame-local, with frame numbers
starting at one.

The 367 new seeds select 1,291 pixels per target: 1,056 core skin and 235 outline
pixels. Another 206 matching outline pixels stay protected. Every frame has
selected skin, ranging from 15 to 70 pixels. Profile SHA-256:
`7abd61b0d8e84fba878aaa77fe93ad3d8232c124823e8e885d5cb862c5d16002`.

All source frames and four targets were inspected. The focused opt-in
`tests/celine_seasonal_standard.rs` passes four literal target ramps, 36 source-art
landmarks, every new pixel, alpha, metadata, complete core shade coverage and
common selection. All 1,032 variants validate. The standalone bundle matches
the inspected candidate, all 2,530 previous original/variant PNG and metadata
files remain byte-identical, and all 36 duplicate-target comparisons pass.

- [Five-choice summary](../../generated/celine-seasonal-standard-preview/summary.png):
  South/East general action, sleep and kiss samples.
- [Complete Vanilla/Blue review](../../generated/celine-seasonal-standard-preview/blue-review/index.html):
  all 38 direction/frame cases on six pages, at most eight cases per page.

Fresh crop `[29,23,24,34]` preserves every visible pixel. Exact checks cover
6,201,600 full-review pixels, 261,120 summary pixels, all source/palette/sample
bindings, original metadata and West reversal. Chromium verifies all six pages,
images and links. Local evidence is under `tmp/celine-seasonal-standard-`,
including `final-audit.json`, `test.log`, `mask-decisions.json`,
`preview-check.log` and `preview-browser-check.json`.

The user accepted this offline artwork on 2026-09-16. Static review does not
exercise live outfit
switching, game timing or separately drawn held items. Shared native,
installation and full-check evidence belongs in
[the seasonal standard batch notes](world-seasonal-standard.md). Extracted
artwork, generated previews and local evidence remain ignored.
