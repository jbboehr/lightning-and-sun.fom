# Ryis's Beach blink, general actions and kiss

This slice adds six Beach strips: 31 source frames and fourteen native West
mirrors. The profile now has 256 sources and 1,024 variants. All 250 prior
region objects, other profile fields, source colors and target mappings remain
unchanged.

Sources were exported from the read-only `tmp/fields-of-mistria/assets.zip`
into `extracted/ryis-beach-actions-study`. Every prior Ryis PNG hash and all 500
original PNG/metadata files match the retained Beach-pilot combined build.
New sources under `assets/animations/NPCs/Ryis/Sprites/Beach/` use the prefix
`spr_npc_ryis_beach_`: blink East/South, action North/South/East and kiss East.
All retain 80×80 frames, Default atlas and Middle/54 origin. Exact sidecars
are in `tmp/ryis-beach-actions-author-metadata.json`.

| Strip | Duration | Skin pixels per frame, per target |
| --- | --- | --- |
| Blink East | `[0.075,0.125,0.075]` | 95, 99, 95 |
| Blink South | Same | 106, 110, 106 |
| Action East | `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` | 81, 74, 74, 74, 74, 81, 89 |
| Action North | Same | 74, 73, 73, 73, 73, 77, 79 |
| Action South | Same | 93, 94, 94, 94, 94, 93, 102 |
| Kiss East | `[0.15,0.15,0.8,0.15]` | 83, 86, 93, 90 |

The 259 component seeds select 2,696 skin pixels per target with the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Every actual Beach frame was
inspected in Vanilla and all four targets. Closed eyelids, face, ears, neck,
exposed upper chest/back, gesturing arms and hands, legs and toes change.
Pink tank fabric and straps, blue shorts and sandal straps, and dark wristbands
remain original. The `5E423B` rear-head patch remains short hair. Black closed-eye
lines and the kissing-mouth outline are preserved. Evidence:
`tmp/ryis-beach-actions-author-components.json` and
`tmp/ryis-beach-actions-author-art-{cycle}_{direction}-{page}.png`.

The [five-choice summary](../../generated/ryis-beach-actions-preview/summary.png)
shows blink South, action East and kiss East. The [complete Vanilla/Blue review](../../generated/ryis-beach-actions-preview/blue-review/index.html)
covers all 45 cases across nine pages, with at most seven cases per page.
Crop `[25,20,32,39]` includes every visible pixel, including extended fingers,
stepping feet and West mirrors. Each case also has an 8× face and upper-chest
detail. Exact checks verified 8,935,200 preview pixels, all ninety full-frame
and fifteen summary bindings, source metadata and West reversal. Chromium
checked all eleven pages, including both indexes: every image and local link
loaded, with no horizontal overflow. Evidence:
`tmp/ryis-beach-actions-author-preview-check.log` and
`tmp/ryis-beach-actions-author-preview-browser-check.json`.

The focused candidate test passed before publishing the profile. The final
opt-in `tests/ryis_beach_actions.rs` passed all new pixels in all four targets,
per-frame counts, sidecars and 31 literal skin/material landmarks. Practical
controls cover an omitted shin component and palette spill onto sandal straps.
Log: `tmp/ryis-beach-actions-author-corpus-test.log`. The final
`generated/ryis-beach-actions-trial` preserves all 2,500 prior original and
variant PNG/metadata files byte-for-byte; all 1,024 variants pass exact-palette
validation (`tmp/ryis-beach-actions-author-preservation.log`). Profile SHA-256:
`3b34c34064a88e03ca6dbcf3939a52788cc19bc3fe1dd8d6f802a04688091149`.

See [the combined integration report](world-beach-actions.md) for shared packaging,
native probes, full checks and installation. Static previews do not establish
timing, natural outfit changes or live gameplay. The user approved this slice for commit on 2026-09-25.
