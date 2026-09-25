# Celine's remaining Winter specials

Eleven strips finish Celine's regular Winter world animations: seated and
standing reading start/loop/end South, sweeping start/loop/end South, watering
East and harvesting East. There are 51 source frames containing 37 distinct
images, plus 14 native West watering/harvesting mirrors. The profile grows
from 350 to 361 regions; Winter coverage is now 33/33 strips.

The source is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
unchanged from the accepted Winter-general batch. Exact paths are under
`assets/animations/NPCs/Celine/Sprites/Winter/`, with prefix
`spr_npc_celine_specialanimation_winter_`. The full export is
`extracted/celine-winter-finish-study`; raw sidecars and frame counts are in
`tmp/celine-winter-finish-author-metadata.json`.

All new frames retain 80×80 geometry, Default atlas and Middle/54 origin.
Reading starts/ends retain 0.1-second frames; their loops retain
`[3.0,0.1,3.0,0.1]`. Sweep start/end use single-frame defaults, and its loop
retains `[0.1,0.1,0.1,0.1,0.1,0.2,0.1,0.1,0.1,0.1,0.1,0.4,0.1,0.1,0.8]`.
Watering retains `[0.15,1.0,0.15,2.5]`; harvesting retains
`[0.4,0.1,0.2,0.2,0.2,0.2,0.2,0.2,0.1,1.4]`.

All prior region objects, seven color groups and source/target mappings remain
unchanged. The existing `FCD9B3`, `F0B988`, `D37A57` and selected `672115`
components retain light/middle/shadow/deep roles. No new shade is needed.
Accepted Autumn counterparts informed the inspection, including Autumn garden
harvesting; Winter's actual sleeves, moving fingers and tools were checked in
every frame rather than transferring seeds automatically.

Masks include face and lip contours, moving fingers and exposed hands while
preserving hair, covered arms, cuffs, scarf, belt, boots, book pages/covers,
broom, dust and watering-can materials. Three disconnected watering face
contours need explicit selection: zero-based frame 0 `[44,37..38]`, frame 1
`[47,36]`, and frame 2 `[44,37..38]`. Nearby frame 1 garment `[38,41]` stays
original. Standing-reading start frame 1 fingertip `[35,47]` changes beside
unchanged book pages `[39,41]`. Winter seated-reading end frame 2 sleeve
`[42,45]` is `71A5B8`, distinct from Autumn's `535B8B`, and remains original.
Harvest frame 3 fingers `[51,52]` and `[47,53]` change while adjacent `B65932`
materials `[40,47]` and `[41,50]` remain. Coordinates are frame-local; frame
numbers in this paragraph start at zero.

The 636 new seeds select 2,044 pixels per target: 1,489 core skin and 555 deep
contour pixels. Another 235 matching dark material pixels remain original.
Every frame has 24–51 selected pixels.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Seated reading start / loop / end | 3 / 4 / 3 | 95 / 136 / 95 |
| Standing reading start / loop / end | 3 / 4 / 3 | 98 / 136 / 98 |
| Sweep start / loop / end | 1 / 15 / 1 | 40 / 729 / 50 |
| Water East | 4 | 139 |
| Harvest East | 10 | 428 |

Profile SHA-256:
`768e97138c2348c8e6d0ecc914baf9c06a197e3fb99aaab5fe727294ee5f9230`.
The common new-frame selection digest is
`918b7f48b852abe00232ea905ec1aee267a176c9fc91d72e0ca2c870c47b57a2`.

All 51 source frames and all four actual target versions were inspected,
including enlarged faces, moving fingers and material boundaries. The focused
opt-in `tests/celine_winter_finish.rs` checks 49 literal source-art landmarks,
four target ramps, every new pixel, complete core skin coverage, alpha,
metadata, per-frame coverage and common selection. The initial candidate
failed at the missing watering contour `[44,37]`; a temporary spill candidate
failed at frame 1 garment `[38,41]`. The final profile passes. Local negative
controls use `FOM_CELINE_WINTER_FINISH_PROFILE`.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_winter_finish -- --ignored'
```

All 1,444 standalone variants validate with strict source hashes and exact
recipes. The final bundle in `generated/celine-winter-finish-trial` matches
the inspected candidate PNG and metadata files byte for byte. All 3,500
earlier original/variant PNG and metadata files match
`generated/characters-world-winter-general-trial/characters/celine` exactly;
no previous source hash or mask was refreshed.

- [Five-choice summary](../../generated/celine-winter-finish-preview/summary.png):
  seated reading, standing reading, sweeping, watering and harvesting.
- [Complete Vanilla/Blue review](../../generated/celine-winter-finish-preview/blue-review/index.html):
  all 65 direction/frame cases on eleven content pages, at most eight per page.

Crop `[17,25,46,37]` contains every visible source pixel, including broom,
dust and extended watering-can spout. Its symmetric bounds support native West
reversal. Exact checks cover 7,965,360 full-review pixels, 382,950 summary
pixels, all 25 summary bindings, raw metadata and West reversal. All 14 West
pairs were also visually inspected. Chromium verifies all twelve HTML pages,
66 decoded images, navigation links and absence of horizontal overflow.
Evidence is under `tmp/celine-winter-finish-author-`, including
`final-audit.json`, `green.log`, `omission-red.log`, `spill-red.log`,
`validation.json`, `preview-pixels.json` and `preview-browser.json`.

See [combined integration](world-winter-finish.md) for shared checks, native
metadata and installation. Static review does not establish live timing,
automatic state transitions or separately drawn held items. Source art,
generated variants, previews and temporary helpers remain ignored. This slice
changes no runtime code.
