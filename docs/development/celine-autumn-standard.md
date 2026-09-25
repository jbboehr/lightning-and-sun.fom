# Celine's Autumn blink, sit, eat and drink sprites

This slice adds 11 regular Autumn strips: blink East/South and sit/eat/drink
North/South/East. Their 31 source frames contain 22 distinct images. Native
West mirroring adds 12 review cases; the garden outfit is outside this slice.

Sources are under `assets/animations/NPCs/Celine/Sprites/Autumn/`, from read-only
`tmp/momi-lab/assets.bak.zip` (SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`).
All frames remain 80×80, Default atlas, Middle/54 origin. Blink retains
`[0.075,0.125,0.075]`; drink and North eat retain `1.0`; East/South eat retain
`[0.125,0.15,0.175,0.125,0.6]`. Sit retains single-frame defaults. Raw sidecars
are preserved in `tmp/celine-autumn-standard-metadata.json`.

The profile grows from 288 to 299 regions. Earlier regions, seven color groups,
source shades and target mappings remain unchanged. World skin uses `FCD9B3`,
`F0B988`, `D37A57` and selected `672115` components as light/middle/shadow/deep.
No new shades were required.

Autumn sleeves and leggings remain covered. Masks include facial contours and
bare fingers while preserving hair, cuffs, clothing, scarf, belt, eyes, boots,
mouth interiors and action details. North eat/drink frame two's tiny fingertip
`[34,47]` maps while adjacent hair `[35,46]` stays original. East drink frame two's
hand `[39,41]`/`[40,42]` maps while cuff `[38,42]` remains unchanged. South eat
frame four's hand outline `[35,43]` maps while the neighboring cuff `[39,44]`
stays original. Coordinates are frame-local; these frame numbers start at one.

The 318 new seeds select 956 pixels per target: 670 core skin and 286 outline
pixels. Another 244 matching dark material pixels remain protected. Every frame
has 2–50 selected pixels. Profile SHA-256:
`38bb0c717f71004d2e9c6c465e94ec1a84c97ddf3e7d50a89fa9b74965d94af5`.

All source frames and four target palettes were inspected. The focused opt-in
`tests/celine_autumn_standard.rs` passes 41 literal source-art boundaries,
four target ramps, every new pixel, alpha, exact metadata and common selection.
All 1,196 variants validate; all 2,880 previous original/variant PNG and metadata
files remain byte-identical. The standalone bundle matches the inspected
candidate, and all 36 duplicate-target comparisons pass.

- [Five-choice summary](../../generated/celine-autumn-standard-preview/summary.png):
  blink, sit, drink and open-mouth eating samples.
- [Complete Vanilla/Blue review](../../generated/celine-autumn-standard-preview/blue-review/index.html):
  all 43 direction/frame cases across eight pages, at most eight cases per page.

Fresh crop `[28,23,23,34]` preserves every visible pixel. Exact checks cover
6,725,200 full-review pixels, 250,240 summary pixels, all 20 sample bindings,
raw metadata and West reversal. Chromium verifies all eight pages, 43 images
and navigation links. Local evidence is under `tmp/celine-autumn-standard-`,
including `final-audit.json`, `test.log`, `mask-decisions.json`,
`preview-check.log` and `preview-browser.json`.

The user approved this slice for commit on 2026-09-25. Static review does not
exercise live outfit switching, timing or separately drawn held items. Shared native, installation
and full-check evidence belongs in [the Autumn standard batch notes](world-autumn-standard.md).
Extracted artwork, generated previews and local evidence remain ignored.
