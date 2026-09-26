# Completing Celine's Wedding animations

Nine strips add blink East/South, sit and general action North/South/East,
and kiss East. Their 34 source frames contain 23 distinct images; native
West mirroring adds 15 review cases. Together with the accepted idle/walk
pilot, this covers all 15 PNG strips in Celine's Wedding folder. The profile
grows from 381 to 390 regions.

The source is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Exact paths are under `assets/animations/NPCs/Celine/Sprites/Wedding/`, named
`spr_npc_celine_wedding_{blink,sit,action,kiss}_{direction}.png`. The full
export is `extracted/celine-wedding-finish-study`; raw sidecars are in
`tmp/celine-wedding-finish-author-metadata.json`.

All frames retain 80×80 geometry, Default atlas and Middle/54 origin. Blink
durations are `[0.075,0.125,0.075]`, sitting uses single-frame defaults,
general actions retain `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`, and kissing
retains `[0.15,0.15,0.8,0.15]`. Native seated flags and general-action holds
are unchanged.

Every actual frame was inspected in all four target palettes. Masks cover
face, neck, exposed wrists, ankles and toes while preserving the pink
dress/veil, white gloves, hair, eye details and pink shoe straps/heels.
The existing `FCD9B3`, `F0B988`, `D37A57` and selected `672115` keep their
light/middle/shadow/deep roles; no shade or mapping was added.

Sitting North has **no exposed skin**. Its veil, dress and gloves cover the
entire figure, so its explicit source-bound region has no seeds. All four
outputs must remain identical to that original frame, including metadata.
Other frames have at least four selected pixels.

Literal source-art examples, with frame-local coordinates and zero-based
frames:

- General action East frame 2 protects `672115` at glove tip `[45,46]`;
  general action South frame 2 protects the same glove shade at `[38,47]`.
  These coordinates were exposed fingers in the Beach outfit.
- General action North frame 5 maps the isolated wrist `[33,45]`, while
  neighboring white glove `[33,46]` and hair `[35,45]` remain original.
- Sitting East maps neck `[39,41]`, wrist `[36,44]` and toe `[44,48]`,
  preserving adjacent hair `[33,44]`, glove `[35,46]` and pink shoe `[41,49]`.
- Kiss frame 2 maps wrist `[38,44]` and toe `[37,53]`, preserving the pink
  glove shading `[35,46]` and shoe strap `[37,52]`.

The 407 new seeds select 1,242 pixels per target: 932 core skin and 310 deep
contour pixels. Another 433 matching dark hair/glove pixels remain original.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Blink East | 3 | 142 |
| Blink South | 3 | 166 |
| Sit East | 1 | 38 |
| Sit North | 1 | 0 |
| Sit South | 1 | 46 |
| General action East | 7 | 286 |
| General action North | 7 | 29 |
| General action South | 7 | 350 |
| Kiss East | 4 | 185 |

The focused opt-in test checks 52 literal source-art landmarks, every new
pixel, four target ramps, alpha, metadata, complete core skin coverage,
per-frame coverage and equal selections. It explicitly checks the unchanged
North sitting frame. Negative controls fail on omitted blink East face
`[44,36]` and a recolored action East frame 2 glove tip `[45,46]`; the
unchanged candidate then passes. Local controls use
`FOM_CELINE_WEDDING_FINISH_PROFILE`.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_wedding_finish -- --ignored'
```

All 381 prior region objects, seven color groups and source/target mappings
are unchanged. All 3,810 prior original/variant PNG and metadata files match
`generated/characters-world-wedding-pilot-trial/characters/celine` exactly.
The final `generated/celine-wedding-finish-trial` validates all 1,560 variants
with strict source hashes and matches the inspected candidate byte for byte.
No existing pin or mask was refreshed.

Profile SHA-256:
`fbbc6a790cf2420e9530e3d1f8e90d60107c6e7510d5b08c0a86dc4091f84a60`.
The common new-frame selection digest is
`15df43ac085846ecfcf0c5fc634083320604a8e5aa55e9851328d13ed82ed936`.

- [Five-choice summary](../../generated/celine-wedding-finish-preview/summary.png):
  blinking, sitting, general actions and kissing.
- [Complete Vanilla/Blue review](../../generated/celine-wedding-finish-preview/blue-review/index.html):
  all 49 direction/frame cases on eight content pages, at most eight per page.

Crop `[28,24,24,32]` includes every visible source pixel and is symmetric for
native West reversal. Exact checks cover 7,526,400 full-review pixels,
307,200 summary pixels, all 25 summary bindings, raw metadata and mirrors.
All 15 West pairs were visually inspected. Chromium verifies nine HTML
pages, 50 decoded images, all links and no horizontal overflow. Evidence is
under `tmp/celine-wedding-finish-author-`, including `initial-audit.json`,
`green.log`, `omission-red.log`, `spill-red.log`, `validation.json`,
`preview-pixels.json` and `preview-browser.json`.

See [combined integration](world-wedding-finish.md) for shared checks,
native metadata and installation. Static previews do not establish live
timing or natural outfit/state transitions. Source art, generated variants
and helpers remain ignored. This slice changes no runtime code.

The user approved this artwork for commit on 2026-09-25.
