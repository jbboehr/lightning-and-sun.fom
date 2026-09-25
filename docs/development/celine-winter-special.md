# Celine's Winter blink, sit, eat and drink

Eleven strips add Winter blink East/South and sit/eat/drink North/South/East.
Their 31 source frames contain 22 distinct images; native West mirroring adds
12 review cases. The profile grows from 334 to 345 regions. Other Winter
actions remain outside this slice.

The source is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
unchanged from the accepted Winter-standard batch. Exact paths are under
`assets/animations/NPCs/Celine/Sprites/Winter/`, named
`spr_npc_celine_winter_{blink,sit,eat,drink}_{direction}.png`. The full export is
`extracted/celine-winter-special-study`; raw sidecars and frame counts are in
`tmp/celine-winter-special-metadata.json`.

Every frame retains 80×80 geometry, Default atlas and Middle/54 origin. Blink
retains `[0.075,0.125,0.075]`; drinks and North eating retain three frames at
1.0 seconds each. East/South eating retain `[0.125,0.15,0.175,0.125,0.6]`.
Sitting retains single-frame defaults. Earlier region objects, seven color
groups and source/target mappings remain unchanged. The existing `FCD9B3`,
`F0B988`, `D37A57` and selected `672115` components retain
light/middle/shadow/deep roles. No additional shade is needed.

Masks include face and lip contours, raised hands and tiny fingertips while
preserving hair, fur cuffs, coat, scarf, belt, boots, eye marks and actual mouth
interiors. The full actual Winter corpus was inspected using accepted Autumn
counterparts as a guide. Two isolated North-facing fingertips needed explicit
selection: eating and drinking frame 1 `[34,47]`, separated from core skin by
the pose's black boundary. Adjacent hair `[35,46]` stays original. East drinking
frame 1 hand `[39,41]`/`[40,42]` changes while its white cuff `[38,42]` stays.
South eating frame 3 hand outline `[35,43]` changes while nearby orange hair
`[39,44]` remains; the latter coordinate was a cuff in Autumn. Coordinates are
frame-local and frame numbers in this paragraph start at zero.

The 318 new seeds select 960 pixels per target: 674 core skin and 286 deep
contour pixels. Another 250 matching dark material pixels remain original.
Every frame has 2–50 selected pixels.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Blink East / South | 3 / 3 | 121 / 130 |
| Sit North / South / East | 1 / 1 / 1 | 6 / 34 / 35 |
| Drink North / South / East | 3 / 3 / 3 | 8 / 119 / 118 |
| Eat North / South / East | 3 / 5 / 5 | 8 / 208 / 173 |

Profile SHA-256:
`1272701e9db0cfe3f20243480c02354209204901134efe8cd68d9fac5e12d819`.
The common new-frame selection digest is
`2275dbcff70827b5984a48cbcac997c1114b1a127e651d30bb8e085979edd436`.

All 31 source frames and four actual target versions were inspected, including
enlarged faces, open mouths, moving fingers and material boundaries. The focused
opt-in `tests/celine_winter_special.rs` checks 41 literal source-art landmarks,
four target ramps, every new pixel, complete core skin coverage, alpha,
metadata, per-frame coverage and common selection. The initial candidate failed
at North drinking's missing fingertip `[34,47]`; a deliberately selected hair
component failed at blink East `[34,32]`. The final profile passes. Local
negative controls use `FOM_CELINE_WINTER_SPECIAL_PROFILE`.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_winter_special -- --ignored'
```

All 1,380 standalone variants validate with strict source hashes and exact
recipes. The final bundle in `generated/celine-winter-special-trial` matches
the inspected candidates byte for byte. All 3,340 earlier original/variant PNG
and metadata files match
`generated/characters-world-winter-standard-trial/characters/celine` exactly;
no previous source hash or mask was refreshed.

- [Five-choice summary](../../generated/celine-winter-special-preview/summary.png):
  blink, seated, drinking and open-mouth eating samples.
- [Complete Vanilla/Blue review](../../generated/celine-winter-special-preview/blue-review/index.html):
  all 43 direction/frame cases on eight content pages, at most eight per page.

Crop `[29,24,22,32]` includes every visible source pixel and is symmetric for
native West reversal. Exact checks cover 6,054,400 full-review pixels, 352,000
summary pixels, all 20 summary bindings, raw metadata and West reversal.
All 12 West pairs were also visually inspected.
Chromium verifies all nine HTML pages, 44 decoded images, navigation links
and absence of horizontal overflow. Evidence is under
`tmp/celine-winter-special-`, including `final-audit.json`, `green.log`,
`omission-red.log`, `spill-red.log`, `validation.json`, `preview-pixels.json`
and `preview-browser.json`.

See [combined integration](world-winter-special.md) for shared checks, native
metadata and installation. Static images do not establish live animation
timing, natural outfit/state transitions or separately drawn held items.
Source art, generated variants, previews and temporary helpers remain ignored.
This character slice changes no runtime code.
