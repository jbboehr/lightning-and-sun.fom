# Celine's Autumn garden animations

Fourteen strips complete the Autumn garden outfit: idle/walk North, South and
East; blink East/South; sit North/South/East; kiss East; and watering/harvesting
East. There are 42 source frames containing 27 distinct images, plus 27 native
West mirrors. The profile grows from 314 to 328 regions, completing Celine's
regular and garden Spring, Summer and Autumn sprites. The batch suffix is
`winter-actions`; Celine's new outfit remains Autumn.

The source is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
unchanged from the accepted Winter-expansion batch. Exact paths are under
`assets/animations/NPCs/Celine/Sprites/Autumn/` with `autumn_garden` in each
filename. Watering and harvesting retain their `specialanimation_` prefix.
The full export is `extracted/celine-winter-actions-study`; raw sidecars and
frame counts are in `tmp/celine-winter-actions-metadata.json`.

Every frame retains 80×80 geometry, Default atlas and Middle/54 origin.
Walking retains four frames at 0.15 seconds each; blinking retains
`[0.075,0.125,0.075]`; kissing retains `[0.15,0.15,0.8,0.15]`. Watering retains
`[0.15,1.0,0.15,2.5]`, and harvesting retains
`[0.4,0.1,0.2,0.2,0.2,0.2,0.2,0.2,0.1,1.4]`. Idle and seated poses retain
their single-frame defaults.

The accepted Summer garden artwork guided source inspection. Autumn's long
sleeves, trousers and boots replace Summer's exposed limbs, so Summer mask
coordinates were not copied blindly. Faces, moving fingers and exposed hands
change; braids, braid ties, scarf, clothing, belts, boots, eyes and the watering
can remain original. Existing `FCD9B3`, `F0B988`, `D37A57` and selected `672115`
components retain light/middle/shadow/deep roles. No source shade or target map
was added, and all previous region objects and seven color groups are unchanged.

Three disconnected watering face contours needed explicit selection: zero-based
frame 0 `[44,37..38]`, frame 1 `[47,36]`, and frame 2 `[44,37..38]`. Conversely,
kiss frame 2 `[42,40]` belongs to the scarf edge and stays original. Kiss
`[40,40]` is now inside the braid boundary, and walk East frame 1 `[39,51]` is
part of a boot; both stay original despite matching Summer skin coordinates.
Harvest frame 0 finger `[40,53]` and frame 3 fingertips `[51,52]` and `[47,53]`
change beside protected braid, belt and sleeve pixels. Coordinates are
frame-local; the frame numbers in this paragraph start at zero.

The 459 new seeds select 1,461 pixels per target: 1,058 core skin and 403 deep
contour pixels. Another 262 matching dark material pixels remain original.
Every frame has 6–47 selected pixels.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Idle North / South / East | 1 / 1 / 1 | 12 / 38 / 36 |
| Walk North / South / East | 4 / 4 / 4 | 44 / 148 / 146 |
| Blink South / East | 3 / 3 | 123 / 118 |
| Sit North / South / East | 1 / 1 / 1 | 6 / 32 / 34 |
| Kiss East | 4 | 163 |
| Water East | 4 | 135 |
| Harvest East | 10 | 426 |

Profile SHA-256:
`81dac7b78cae4c92c4c322a4f89f2f590a0c7508aa39334728b338979b998d15`.
The common new-frame selection digest is
`91460773ccea4d6b137d389c8f0e566ad68295680c00f63ab48842c82b83da2c`.

All 42 source frames and four actual target versions were inspected, including
enlarged hand, face, scarf, braid and boot boundaries. The focused opt-in
`tests/celine_winter_actions.rs` checks 59 literal source-art landmarks, four
target ramps, every new pixel, complete core skin coverage, alpha, metadata,
per-frame coverage and common selection. It first failed on the watering
omission `[44,37]`, then on the kiss scarf spill `[42,40]`, before passing the
final masks. Local negative controls use `FOM_CELINE_WINTER_ACTIONS_PROFILE`.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_winter_actions -- --ignored'
```

All 1,312 standalone variants validate with strict source hashes and exact
recipes. The final bundle in `generated/celine-winter-actions-trial` matches
the inspected candidates byte for byte. All 3,140 earlier original/variant PNG
and metadata files match
`generated/characters-world-winter-expansion-trial/characters/celine` exactly;
no previous hash or mask was refreshed.

- [Five-choice summary](../../generated/celine-winter-actions-preview/summary.png):
  idle, seated, harvesting and watering samples.
- [Complete Vanilla/Blue review](../../generated/celine-winter-actions-preview/blue-review/index.html):
  all 69 direction/frame cases on eleven content pages, at most eight per page.

Crop `[17,26,46,31]` includes every visible source pixel and is symmetric for
native West reversal. Exact checks cover 7,084,368 full-review pixels, 256,680
summary pixels, all 20 summary bindings, raw metadata and West reversal.
All 27 West pairs were also visually inspected.
Chromium verifies all twelve HTML pages, 70 decoded images, navigation links
and absence of horizontal overflow. Evidence is under
`tmp/celine-winter-actions-`, including `final-audit.json`, `green.log`,
`omission-red.log`, `spill-red.log`, `validation.json`, `preview-pixels.json`
and `preview-browser.json`.

See [combined integration](world-winter-actions.md) for shared checks, native
metadata and installation. Static images do not establish live animation
timing, natural outfit/state transitions or separately drawn held items.
Source art, generated variants, previews and temporary helpers remain ignored.
This character slice changes no runtime code.
