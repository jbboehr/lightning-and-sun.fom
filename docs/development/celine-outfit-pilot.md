# Celine's first garden outfit sprites

This pilot adds six Spring garden idle/walk strips, North/South/East. Their
15 source frames contain nine distinct images; native West mirroring adds five
review cases. Other garden actions remain outside this slice.

## Source and masks

Sources are `spr_npc_celine_spring_garden_{idle,walk}_{direction}.png` under
`assets/animations/NPCs/Celine/Sprites/Spring/`. All frames are 80×80, Default
atlas, Middle/54 origin. Idle uses single-frame defaults; walk has four frames
at duration `0.15`. Exact sidecars are in `tmp/celine-outfit-pilot-metadata.json`.
The read-only archive remains `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

The profile grows from 219 to 225 regions. All earlier regions, seven color
groups and source/target mappings remain unchanged. Existing world skin shades
`FCD9B3`, `F0B988` and `D37A57` retain light/middle/shadow roles; selected
`672115` components retain the deep role. No new colors were needed.

The garden outfit exposes bare forearms and hands. Its hair/braid, red scarf,
green clothing, belt and boots remain original. Separately selected wrist
outlines distinguish skin from identical belt and hair colors. For example,
North idle `[36,44]` recolors while adjacent belt `[37,45]` remains original.
East walk frame four `[38,48]` and `[37,49]` follow the hand; boot outline
`[37,51]` remains original. Coordinates are frame-local; frame numbers start at
one. The small exposed leg pixels in East walking also recolor.

The 167 new seeds select 542 pixels per target: 411 core skin pixels and
131 outline pixels. Another 120 matching outline pixels remain protected.
Idle East/North/South select 42/22/48 pixels; walking selects 172/76/182.
Every frame contains selected skin, ranging from 16 to 48 pixels.

Profile SHA-256:
`d7b44ef179b1d1258c00ee521c326a3f04676bcedc34c3a70194e8624b00e4ea`.

## Review and verification

All 15 original and four-target frames were inspected. The focused opt-in
`tests/celine_outfit_pilot.rs` passes four literal target ramps, 26 independently
chosen skin/material landmarks, every new pixel, alpha, metadata, complete core
shade coverage and common selection. All 900 variants validate; the standalone
bundle matches the inspected candidate. All 2,190 prior original/variant PNG
and metadata files remain byte-identical, and all 24 duplicate-target checks pass.

- [Five-choice summary](../../generated/celine-outfit-pilot-preview/summary.png):
  South/North idle and East/South walking samples.
- [Complete Vanilla/Blue review](../../generated/celine-outfit-pilot-preview/blue-review/index.html):
  all 20 direction/frame cases on three pages, at most eight cases per page.

Fresh crop `[29,25,22,33]` preserves every visible pixel. Exact checks cover
2,904,000 full-review pixels, 232,320 summary pixels, all sample/palette/source
bindings, original metadata and West reversal. Chromium verifies all three
pages, images and links. Evidence is under `tmp/celine-outfit-pilot-`, especially
`final-audit.json`, `test-final.log`, `mask-decisions.json`, `duplicates.log`,
`preview-pixel-check.log` and `preview-browser-check.json`.

The user accepted the offline artwork review. Static review does not exercise live
outfit switching or game timing. Shared native, installation and full-check
evidence belongs in [the outfit pilot batch notes](world-outfit-pilots.md).
Extracted artwork, generated previews and local evidence remain ignored.
