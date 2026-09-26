# Celine's Beach blinking, general actions and kissing

Six strips add Beach blink East/South, general action North/South/East and
kiss East. Their 31 source frames contain 20 distinct images; native West
mirroring adds 14 review cases. The profile grows from 367 to 373 regions.
The user approved this slice for commit on 2026-09-25.

The source is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
unchanged from the accepted Beach pilot. Exact paths are under
`assets/animations/NPCs/Celine/Sprites/Beach/`, named
`spr_npc_celine_beach_{blink,action,kiss}_{direction}.png`. The complete export
is `extracted/celine-beach-actions-study`; raw sidecars and frame counts are
in `tmp/celine-beach-actions-author-metadata.json`.

All frames retain 80×80 geometry, Default atlas and Middle/54 origin. Blink
durations are `[0.075,0.125,0.075]`, general actions retain
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`, and kissing retains
`[0.15,0.15,0.8,0.15]`. Native general actions also have a final hold; the
palette does not modify that behavior.

All prior region objects, seven color groups and source/target mappings are
unchanged. The existing `FCD9B3`, `F0B988`, `D37A57` and selected `672115`
components retain light/middle/shadow/deep roles. No new shade is needed.
Actual Beach frames were inspected independently of seasonal masks.

Masks cover the face, moving hands, legs and bare feet while preserving the
swimsuit, flower, hair, black outlines and eye details. In general action
East frame 1, deep finger `[48,45]` changes while the identical dark braid
component at `[38,44]` stays original. North frame 0 distinguishes hand
`[33,45]` from hair `[35,45]` and `[38,46]`. Blink East flower `[32,29]`,
hair `[35,31]` and swimsuit `[40,48]` stay original beside changing finger
`[35,47]` and toe `[40,53]`. Kiss frame 2 maps facial contour `[47,35]`
and preserves the white braid highlight `[38,40]`. Coordinates are
frame-local; frame numbers here start at zero.

The 395 new seeds select 1,897 pixels per target: 1,643 core skin and 254 deep
contour pixels. Another 270 matching dark material pixels remain original.
Every frame has 25–87 selected pixels.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Blink East | 3 | 220 |
| Blink South | 3 | 241 |
| General action North | 7 | 209 |
| General action South | 7 | 493 |
| General action East | 7 | 456 |
| Kiss East | 4 | 278 |

Profile SHA-256:
`88c3f526c8fd83fe725ab33aacad203c62e943a0e0eaf9ea5df0b795de6b51e1`.
The common new-frame selection digest is
`3d87a1edaa2d3f3197146fda0af0115d20a69d97310fde7b3d5842a77a099e5a`.

All 31 source frames and all four actual target versions were inspected,
including enlarged face, swimsuit, hair, fingers and bare-foot boundaries.
The focused opt-in `tests/celine_beach_actions.rs` checks 37 literal
source-art landmarks, four target ramps, every new pixel, complete core skin
coverage, alpha, metadata, per-frame coverage and common selection. Negative
controls fail at an omitted facial contour `[44,36]` and a selected hair
component `[35,31]`, both in blink East frame 0. The candidate then passes
unchanged. Local controls use `FOM_CELINE_BEACH_ACTIONS_PROFILE`.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_beach_actions -- --ignored'
```

All 1,492 standalone variants validate with strict source hashes and exact
recipes. The final bundle in `generated/celine-beach-actions-trial` matches
the inspected candidate PNG and metadata files byte for byte. All 3,670
earlier original/variant PNG and metadata files match
`generated/characters-world-beach-pilot-trial/characters/celine` exactly;
no previous source hash or mask was refreshed.

- [Five-choice summary](../../generated/celine-beach-actions-preview/summary.png):
  closed eyes, actions in three directions, and kissing.
- [Complete Vanilla/Blue review](../../generated/celine-beach-actions-preview/blue-review/index.html):
  all 45 direction/frame cases on seven content pages, at most eight per page.

Crop `[28,24,24,32]` contains every visible source pixel and is symmetric for
native West reversal. Exact checks cover 6,912,000 full-review pixels,
307,200 summary pixels, all 25 summary bindings, raw metadata and West
reversal. All 14 West pairs were also visually inspected. Chromium checks
all eight HTML pages, 46 decoded images, navigation links and absence of
horizontal overflow. Evidence is under `tmp/celine-beach-actions-author-`,
including `initial-audit.json`, `green.log`, `omission-red.log`,
`spill-red.log`, `validation.json`, `preview-pixels.json` and
`preview-browser.json`.

See [combined integration](world-beach-actions.md) for shared checks, native
metadata and installation. Static images do not establish live timing,
natural outfit/state transitions or separately drawn held items. Source art,
generated variants, previews and temporary helpers remain ignored. This
slice changes no runtime code.
