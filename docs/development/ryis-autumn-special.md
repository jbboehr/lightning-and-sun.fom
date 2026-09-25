# Ryis's Winter action, sleep and kiss

This slice adds five strips: 26 source frames and twelve native West mirrors.
The profile now has 233 sources and 932 variants. All 228 prior region objects,
other profile fields, source colors and target mappings remain unchanged.
The `autumn-special` artifact prefix follows the shared batch name; Ryis's
content is Winter.

Sources were exported from the updated, read-only `tmp/fields-of-mistria/assets.zip`
into `extracted/ryis-autumn-special-study`. Every prior Ryis PNG hash and all
456 original PNG/metadata files match the retained previous combined build.
New sources under `assets/animations/NPCs/Ryis/Sprites/Winter/` use the prefix
`spr_npc_ryis_winter_`: action North/South/East, sleep East and kiss East.
All retain 80×80 frames, Default atlas and Middle/54 origin. Exact sidecars
are in `tmp/ryis-autumn-special-metadata.json`.

| Strip | Frames | Duration | Skin pixels per frame, per target |
| --- | ---: | --- | --- |
| Action East | 7 | `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` | 44, 44, 44, 44, 44, 44, 45 |
| Action North | 7 | Same | 24, 24, 24, 24, 24, 24, 26 |
| Action South | 7 | Same | 49, 49, 47, 49, 47, 49, 50 |
| Sleep East | 1 | Engine default | 48 |
| Kiss East | 4 | `[0.15,0.15,0.8,0.15]` | 44, 47, 51, 51 |

The 67 component seeds select 1,060 skin pixels per target with the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Every frame was inspected in
Vanilla and all four targets. Exposed fingertips, including the extended
action hand and raised sleep hand, are included. Dark gloves (`353A50`,
`222436`), orange coat, scarf, pink shirt, covered legs and brown boots remain
original. The `5E423B` rear-head patch remains short hair. Black closed-eye
lines and the kissing-mouth outline are preserved. Evidence:
`tmp/ryis-autumn-special-components.json` and
`tmp/ryis-autumn-special-art-{cycle}-{direction}-{page}.png`.

The [five-choice summary](../../generated/ryis-autumn-special-preview/summary.png)
shows one sample per cycle. The [complete Vanilla/Blue review](../../generated/ryis-autumn-special-preview/blue-review/index.html)
covers all 38 cases across eight pages, with at most seven cases per page.
Crop `[25,20,32,39]` includes every visible pixel and the extended fingers.
Each case also has enlarged face details. Exact checks verified 6,504,528
preview pixels, all 76 full-frame and fifteen summary bindings, metadata and
West reversal. Chromium checked all ten pages, including both indexes: every
image and local link loaded, with no horizontal overflow. Evidence:
`tmp/ryis-autumn-special-preview-check.log` and
`tmp/ryis-autumn-special-preview-browser-check.json`.

The opt-in `tests/ryis_autumn_special.rs` passed all new pixels in all four
targets, per-frame counts, sidecars and nineteen literal skin/material landmarks
(`tmp/ryis-autumn-special-corpus-test.log`). The final
`generated/ryis-autumn-special-trial` preserves all 2,280 prior PNG/metadata
files byte-for-byte; all 932 variants pass exact-palette validation
(`tmp/ryis-autumn-special-preservation.log`). Profile SHA-256:
`d968c8e4432cc8a595aa6df8a0210b45e734edc2eb1cfced0fe9717b67448d3d`.

See [the combined integration report](world-autumn-special.md) for shared
packaging, updated-game compatibility, native probes, full checks and installation.
Static previews do not establish timing, natural outfit changes or live gameplay.
This slice awaits user art review.
