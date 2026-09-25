# Celine's Winter idle and walk

Six strips add Winter idle/walk North, South and East. Their 15 source frames
contain nine distinct images; native West mirroring adds five review cases.
The profile grows from 328 to 334 regions. Other Winter animations remain
outside this slice.

The source is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
unchanged from the accepted Winter-actions batch. Exact paths are under
`assets/animations/NPCs/Celine/Sprites/Winter/`, named
`spr_npc_celine_winter_{idle,walk}_{north,south,east}.png`. The full export is
`extracted/celine-winter-standard-study`; raw sidecars and frame counts are in
`tmp/celine-winter-standard-metadata.json`.

All frames retain 80×80 geometry, Default atlas and Middle/54 origin. Idle
uses single-frame defaults; walking retains four frames at 0.15 seconds each.
All prior region objects, seven color groups and source/target mappings are
unchanged. The existing `FCD9B3`, `F0B988`, `D37A57` and selected `672115`
components retain light/middle/shadow/deep roles. No additional shade is needed.

Winter leaves Celine's hands bare. Masks cover the exposed face, lower face
contours, fingertips and hand creases while preserving hair, scarf, belt, coat,
fur trim, boots and eye details. Source-art landmarks include idle East face
`[44,36]` and hand `[35,47]`, beside protected hair `[35,31]` and belt `[38,45]`.
Walk East frame 1 fingertip `[46,47]` changes while belt `[42,46]` and blue coat
edge `[39,51]` stay original. Walk North frame 1 fingertip `[34,48]` changes
beside the unchanged hair `[39,41]`. Coordinates are frame-local and frame
numbers in this paragraph start at zero. Autumn references guided inspection;
the actual Winter clothing and hand boundaries were inspected independently.

The 167 new seeds select 441 pixels per target: 305 core skin and 136 deep
contour pixels. Another 156 matching dark material pixels remain original.
Every frame has 8–44 selected pixels.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Idle North | 1 | 12 |
| Idle South | 1 | 40 |
| Idle East | 1 | 37 |
| Walk North | 4 | 40 |
| Walk South | 4 | 158 |
| Walk East | 4 | 154 |

Profile SHA-256:
`74463abadc1f35af4a09671c2a4ebaecadd664c886ff7372a63a2fe70e134f85`.
The common new-frame selection digest is
`c3333c65a86c36b23bc97823eac1f80fe758590fd1b0279b574d55d9630abfa4`.

All 15 source frames and four actual target versions were inspected, including
enlarged face, hand, coat and boot boundaries. The focused opt-in
`tests/celine_winter_standard.rs` checks 30 literal source-art landmarks, four
target ramps, every new pixel, complete core skin coverage, alpha, metadata,
per-frame coverage and common selection. A deliberately omitted face component
failed at idle East `[44,36]`; a deliberately selected hair component failed
at `[35,31]`. The final mask passes. These local negative controls use
`FOM_CELINE_WINTER_STANDARD_PROFILE`.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_winter_standard -- --ignored'
```

All 1,336 standalone variants validate with strict source hashes and exact
recipes. The final bundle in `generated/celine-winter-standard-trial` matches
the inspected candidates byte for byte. All 3,280 earlier original/variant PNG
and metadata files match
`generated/characters-world-winter-actions-trial/characters/celine` exactly;
no previous source hash or mask was refreshed.

- [Five-choice summary](../../generated/celine-winter-standard-preview/summary.png):
  idle North/South and walking East/South samples.
- [Complete Vanilla/Blue review](../../generated/celine-winter-standard-preview/blue-review/index.html):
  all 20 direction/frame cases on three content pages, at most eight per page.

Crop `[30,25,20,32]` includes every visible source pixel and is symmetric for
native West reversal. Exact checks cover 2,560,000 full-review pixels, 320,000
summary pixels, all 20 summary bindings, raw metadata and West reversal.
All five West pairs were also visually inspected.
Chromium verifies all four HTML pages, 21 decoded images, navigation links
and absence of horizontal overflow. Evidence is under
`tmp/celine-winter-standard-`, including `initial-audit.json`, `green.log`,
`omission-red.log`, `spill-red.log`, `validation.json`, `preview-pixels.json`
and `preview-browser.json`.

See [combined integration](world-winter-standard.md) for shared checks, native
metadata and installation. Static images do not establish live animation
timing, natural outfit/state transitions or separately drawn held items.
Source art, generated variants, previews and temporary helpers remain ignored.
This character slice changes no runtime code.
