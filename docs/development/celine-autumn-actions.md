# Celine's Autumn idle and walking sprites

This slice adds six regular Autumn idle/walk strips, North/South/East. Their
15 source frames contain nine distinct images; native West mirroring adds five
review cases. Other Autumn actions and the garden outfit remain outside this slice.

Sources are `spr_npc_celine_autumn_{idle,walk}_{direction}.png` under
`assets/animations/NPCs/Celine/Sprites/Autumn/`, from read-only
`tmp/momi-lab/assets.bak.zip` (SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`).
All frames are 80×80, Default atlas, Middle/54 origin. Idle retains single-frame
defaults; walk retains four frames at duration `0.15`. Raw sidecars are preserved
in `tmp/celine-autumn-actions-metadata.json`. Both native cycles are linear;
walking retains its pause transition to idle.

The profile grows from 282 to 288 regions. Earlier regions, seven color groups,
source shades and target mappings remain unchanged. World skin uses `FCD9B3`,
`F0B988`, `D37A57` and selected `672115` components as light/middle/shadow/deep.
No new colors were required.

Autumn sleeves and leggings cover the arms and legs. Masks recolor the face and
bare hands while preserving hair, blue clothing, white cuffs, scarf, belt, eyes
and boots. North walking frame two's fingertip `[34,48]` maps while nearby hair
`[33,46]` and `[36,47]` stays original. East walking frame four's hand `[36,48]`
maps while hair `[35,44]`, belt `[43,46]` and boot `[44,52]` remain unchanged.
Coordinates are frame-local; these frame numbers start at one.

The 162 new seeds select 435 pixels per target: 304 core skin and 131 outline
pixels. Another 177 matching dark material pixels stay protected. Every frame
has 7–42 selected pixels. Profile SHA-256:
`fbf506a7466b24b090bd82c20d3d21b26a72afd75d334d8cbb3d7cc7f71aca59`.

All source frames and four target palettes were inspected. The focused opt-in
`tests/celine_autumn_actions.rs` covers 34 literal source-art boundaries, four
target ramps, every new pixel, alpha, exact metadata and common selection.
All 1,152 variants validate; all 2,820 previous original/variant PNG and metadata
files remain byte-identical. The standalone bundle matches the inspected
candidate, and all 24 duplicate-target comparisons pass.

- [Five-choice summary](../../generated/celine-autumn-actions-preview/summary.png):
  South/North idle and East/South walking samples.
- [Complete Vanilla/Blue review](../../generated/celine-autumn-actions-preview/blue-review/index.html):
  all 20 direction/frame cases on three pages, at most eight cases per page.

Fresh crop `[29,24,22,34]` preserves every visible pixel. Exact checks cover
2,992,000 full-review pixels, 239,360 summary pixels, all source/palette/sample
bindings, raw metadata and West reversal. Chromium verifies all three pages,
20 images and navigation links. Local evidence is under `tmp/celine-autumn-actions-`,
including `final-audit.json`, `test.log`, `mask-decisions.json`,
`preview-check.log` and `preview-browser.json`.

The user accepted the offline artwork on 2026-09-19. Static review does not exercise live outfit
switching or game timing. Shared native, installation and full-check evidence
belongs in [the Autumn actions batch notes](world-autumn-actions.md). Extracted
artwork, generated previews and local evidence remain ignored.
