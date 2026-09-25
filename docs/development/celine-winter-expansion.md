# Celine's remaining regular Autumn specials

Ten strips complete Celine's regular Autumn world animations: seated and
standing reading start/loop/end, sweeping start/loop/end, and watering East.
The batch suffix is `winter-expansion` because Hayden begins and Ryis finishes
Winter alongside this work; Celine's new art remains Autumn. Garden assets are outside
this slice. There are 41 source frames containing 33 distinct images, plus four
native West watering mirrors. The profile grows from 304 to 314 regions.

The source is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
unchanged from the accepted Autumn-special batch. Exact paths use
`assets/animations/NPCs/Celine/Sprites/Autumn/` and prefix
`spr_npc_celine_specialanimation_autumn_`. The full export is
`extracted/celine-winter-expansion-study`; raw sidecars and frame counts are in
`tmp/celine-winter-expansion-metadata.json`. No previous source hash was refreshed.

All new frames retain 80×80 geometry, Default atlas and Middle/54 origin.
Reading starts/ends retain 0.1-second frames; their loops retain
`[3.0,0.1,3.0,0.1]`. Sweep start/end use single-frame defaults, and its loop
retains `[0.1,0.1,0.1,0.1,0.1,0.2,0.1,0.1,0.1,0.1,0.1,0.4,0.1,0.1,0.8]`.
Watering retains `[0.15,1.0,0.15,2.5]`.

The existing `FCD9B3`, `F0B988`, `D37A57` and selected `672115` components retain
light/middle/shadow/deep roles. All prior region objects, seven color groups and
source/target mappings remain unchanged. Accepted Summer equivalents informed
the source inspection; Autumn's sleeves and changing hand positions were checked
in every actual frame. Face and lip contours, moving fingers and exposed hands
change. Hair, covered arms, cuffs, scarf, belt, boots, books, broom, dust and
watering-can materials stay original.

Three disconnected watering face contours required explicit selection beyond
the initial adjacent-skin mask: zero-based frame 0 `[44,37..38]`, frame 1
`[47,36]`, and frame 2 `[44,37..38]`. The same anatomical contours change in the
accepted Summer artwork. Nearby frame 1 garment `[38,41]` stays original.
Standing-reading start frame 1 fingertip `[35,47]` changes beside unchanged
book pages `[39,41]`; seated-reading end frame 2 sleeve `[42,45]` stays blue-gray.
These coordinates are frame-local and frame numbers in this paragraph start
at zero.

The 520 new seeds select 1,623 pixels per target: 1,198 core skin and 425 deep
contour pixels. Another 141 matching dark material pixels remain original.
Every frame has 24–52 selected pixels.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Seated reading start | 3 | 95 |
| Seated reading loop | 4 | 136 |
| Seated reading end | 3 | 95 |
| Standing reading start | 3 | 98 |
| Standing reading loop | 4 | 136 |
| Standing reading end | 3 | 98 |
| Sweep start | 1 | 40 |
| Sweep loop | 15 | 738 |
| Sweep end | 1 | 50 |
| Water East | 4 | 137 |

Profile SHA-256:
`a3d723a7eb4b95ed6f72c67e834eaeb2d1f96e7d19530a89dc7d0d484b9476ae`.
The common new-frame selection digest is
`f4e8632b8e3e3abec70c2dfce6a5d2d505320333f0842c473e2a04308fb8216c`.

All 41 source frames and all four actual target versions were inspected,
including enlarged faces, fingers and material boundaries. The focused opt-in
`tests/celine_winter_expansion.rs` checks 39 literal source-art landmarks, four
target ramps, every new pixel, complete core skin coverage, alpha, metadata,
per-frame coverage and common selection. The initial candidate failed on the
missing watering contour `[44,37]`; a temporary spill candidate failed on
frame 1 garment `[38,41]`. The final profile passes. Local negative controls
use `FOM_CELINE_WINTER_EXPANSION_PROFILE`.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_winter_expansion -- --ignored'
```

All 1,256 standalone variants validate with strict source hashes and exact
recipes. The final bundle in `generated/celine-winter-expansion-trial` is
byte-identical to the inspected candidates. All 3,040 earlier original/variant
PNG and metadata files match
`generated/characters-world-autumn-special-trial/characters/celine` exactly.

- [Five-choice summary](../../generated/celine-winter-expansion-preview/summary.png):
  seated reading, standing reading, sweeping and watering samples.
- [Complete Vanilla/Blue review](../../generated/celine-winter-expansion-preview/blue-review/index.html):
  all 45 direction/frame cases on eight content pages, at most eight per page.

Crop `[17,25,46,37]` includes every visible source pixel, including the broom,
dust and extended watering-can spout. Its symmetric bounds support native West
reversal. Exact checks cover 5,514,480 full-review pixels, 306,360 summary pixels,
all 20 summary bindings, metadata and West reversal. All four West pairs were
visually inspected. Chromium verifies all nine HTML pages, 46 decoded images,
navigation links and absence of horizontal overflow. Evidence is under
`tmp/celine-winter-expansion-`, including `final-audit.json`, `green.log`,
`omission-red.log`, `spill-red.log`, `validation.json`, `preview-pixels.json`
and `preview-browser.json`.

See [combined integration](world-winter-expansion.md) for shared checks, native
metadata and installation. Static review does not establish live timing,
automatic state transitions or separately drawn held items. Source art,
generated variants, previews and temporary helpers remain ignored. No runtime
code changes are needed for this character slice.
