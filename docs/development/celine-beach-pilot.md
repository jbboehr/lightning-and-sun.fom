# Celine's Beach idle and walk

Six strips add Beach idle/walk North, South and East. Their 15 source frames
contain nine distinct images; native West mirroring adds five review cases.
The profile grows from 361 to 367 regions. Other Beach animations remain
outside this slice.

The source is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
unchanged from the accepted Winter-finish batch. Exact paths are under
`assets/animations/NPCs/Celine/Sprites/Beach/`, named
`spr_npc_celine_beach_{idle,walk}_{north,south,east}.png`. The complete export
is `extracted/celine-beach-pilot-study`; raw sidecars and frame counts are in
`tmp/celine-beach-pilot-author-metadata.json`.

All frames retain 80×80 geometry, Default atlas and Middle/54 origin. Idle
uses single-frame defaults; walking retains four frames at 0.15 seconds each.
All prior region objects, seven color groups and source/target mappings are
unchanged. The existing `FCD9B3`, `F0B988`, `D37A57` and selected `672115`
components retain light/middle/shadow/deep roles. No new shade is needed.

Beach exposes Celine's arms, legs and bare feet. Masks cover them and the face
while preserving the swimsuit, flower, hair, black outlines and eye details.
Actual Beach materials were inspected independently; seasonal masks were not
transferred. In walk East frame 1, heel `[37,50]` is exposed skin, although the
same coordinate was a boot in Winter. Detached `672115` thigh contour
`[42,49]` directly below the swimsuit hem also needs explicit selection;
swimsuit `[42,48]` is `3B5675` and stays original. Idle East flower `[32,29]`
and hair `[35,31]` stay original beside changing arm `[34,45]`, finger
`[35,47]` and toe `[40,53]`. Coordinates are frame-local and frame numbers in
this paragraph start at zero.

The 200 new seeds select 884 pixels per target: 764 core skin and 120 deep
contour pixels. Another 137 matching dark material pixels remain original.
Every frame has 29–77 selected pixels.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Idle North | 1 | 36 |
| Idle South | 1 | 77 |
| Idle East | 1 | 70 |
| Walk North | 4 | 130 |
| Walk South | 4 | 288 |
| Walk East | 4 | 283 |

Profile SHA-256:
`ef7da1518ae15c430d5eab9f87af4037db00d8cc4230ada374040ba837995867`.
The common new-frame selection digest is
`7762daa98686f504a0219420eccf92054afccb90d1daaba88e40224119bb7559`.

All 15 source frames and all four actual target versions were inspected,
including enlarged face, swimsuit, hair, fingers and bare-foot boundaries.
The focused opt-in `tests/celine_beach_pilot.rs` checks 44 literal source-art
landmarks, four target ramps, every new pixel, complete core skin coverage,
alpha, metadata, per-frame coverage and common selection. The initial mask
failed on the detached thigh contour `[42,49]`; the corrected mask passes.
Separate negative controls failed on an omitted heel `[37,50]` and a selected
hair component at idle East `[35,31]`. Local negative controls use
`FOM_CELINE_BEACH_PILOT_PROFILE`.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_beach_pilot -- --ignored'
```

All 1,468 standalone variants validate with strict source hashes and exact
recipes. The final bundle in `generated/celine-beach-pilot-trial` matches the
inspected final candidate PNG and metadata files byte for byte. All 3,610
earlier original/variant PNG and metadata files match
`generated/characters-world-winter-finish-trial/characters/celine` exactly;
no previous source hash or mask was refreshed.

- [Five-choice summary](../../generated/celine-beach-pilot-preview/summary.png):
  idle North/South and walking East/South samples.
- [Complete Vanilla/Blue review](../../generated/celine-beach-pilot-preview/blue-review/index.html):
  all 20 direction/frame cases on three content pages, at most eight per page.

Crop `[30,25,20,32]` contains every visible source pixel and is symmetric for
native West reversal. Exact checks cover 2,560,000 full-review pixels, 320,000
summary pixels, all 20 summary bindings, raw metadata and West reversal.
All five West pairs were also visually inspected. Chromium verifies all four
HTML pages, 21 decoded images, navigation links and absence of horizontal
overflow. Evidence is under `tmp/celine-beach-pilot-author-`, including
`final-audit.json`, `candidate-green.log`, `omission-red.log`,
`thigh-omission-red.log`, `spill-red.log`, `validation.json`,
`preview-pixels.json` and `preview-browser.json`.

See [combined integration](world-beach-pilot.md) for shared checks, native
metadata and installation. Static images do not establish live timing,
natural outfit/state transitions or separately drawn held items. Source art,
generated variants, previews and temporary helpers remain ignored. This slice
changes no runtime code.
