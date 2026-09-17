# Celine's Summer garden outfit

This slice adds all fourteen Summer garden strips: idle/walk North, South and
East; blink East/South; sit North/South/East; kiss East; and garden watering and
harvesting East. There are 42 source frames, 27 distinct images and 27 native
West mirror views. Both her Spring and Summer sprite folders are now fully covered.

Sources come from read-only `tmp/momi-lab/assets.bak.zip` (SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`), under
`assets/animations/NPCs/Celine/Sprites/Summer/`, with `summer_garden` in each
filename. All frames are 80×80, Default atlas, Middle/54 origin. Raw sidecars are
preserved in `tmp/celine-seasonal-expansion-metadata.json`. All seven native
cycles are linear. Harvesting retains its ten variable-duration frames; watering
retains `[0.15,1.0,0.15,2.5]`. Walking, blinking and kissing also retain their
original frame counts and timing; seated poses remain seated.

The profile grows from 268 to 282 regions. Existing regions, seven color groups,
source shades and target mappings remain unchanged. World skin uses `FCD9B3`,
`F0B988`, `D37A57` and selected `672115` components as light/middle/shadow/deep.
No new shade is required. These sprites have bare arms and legs.

Masks cover faces, exposed wrists, moving fingers, leg shadows and toes while
preserving braids, braid ties, pink clothing, belts, sandal straps, eye pixels and
the watering can. Summer-specific additions include North walk frame two's
hand `[34,48]`, East walk frame two's arm `[44,46]` and leg edges `[41,50]` and
`[39,51]`, and watering frame one's arm `[38,44]`. Harvest frame one's finger
`[40,53]` maps while braid `[38,45]` and belt `[43,50]` stay original. Coordinates
are frame-local and these frame numbers start at one.

The 633 new seeds select 2,259 pixels per target: 1,832 core skin and 427 outline
pixels. Another 132 matching dark material pixels stay protected. Every frame
has 22–72 selected pixels. Profile SHA-256:
`407d10cbbee0d66d2a69e755585284784e27ccd43ce912663fa6a299184fecc1`.

All 42 source frames and four target palettes were visually inspected. The
focused opt-in `tests/celine_seasonal_expansion.rs` passes 55 literal source-art
boundaries, all four ramps, every new pixel, alpha, raw metadata and common
selection. All 1,128 variants validate; all 2,680 previous original/variant PNG
and metadata files remain byte-identical. The standalone bundle matches the
reviewed candidate, and all 60 duplicate-target comparisons pass.

- [Five-choice summary](../../generated/celine-seasonal-expansion-preview/summary.png):
  idle, seated, harvesting and watering samples.
- [Complete Vanilla/Blue review](../../generated/celine-seasonal-expansion-preview/blue-review/index.html):
  all 69 direction/frame cases on eleven pages, at most eight cases per page.

Fresh crop `[29,25,35,33]` includes every visible pixel, including extended hands
and the watering-can spout. Exact preview checks cover 15,939,000 full-review
pixels, 207,900 summary pixels, all source/palette bindings, metadata and West
reversal. Chromium verifies all eleven pages, 69 images and navigation links.
Local evidence is under `tmp/celine-seasonal-expansion-`, including
`final-audit.json`, `test.log`, `mask-decisions.json`, `preview-check.log` and
`preview-browser.json`.

The user accepted this offline artwork on 2026-09-16. Static review does not
exercise live outfit
changes, gameplay timing or separately drawn held items. Shared native,
installation and full-check evidence belongs in
[the seasonal expansion batch notes](world-seasonal-expansion.md). Game artwork,
generated previews and local evidence remain ignored.
