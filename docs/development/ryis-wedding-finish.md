# Ryis's remaining Wedding actions

This slice adds nine Wedding strips: blink East/South, sitting and general
actions North/South/East, and kiss East. Their 34 source frames and 15 native
West mirrors make 49 review cases. All fifteen strips in Ryis's Wedding sprite
folder are now covered. The profile has 273 sources and 1,092 variants; all
264 prior region objects, other profile fields, source colors and target
mappings remain unchanged.

Sources were exported from the read-only `tmp/fields-of-mistria/assets.zip`
into `extracted/ryis-wedding-finish-study`. Every prior Ryis PNG hash and all
528 original PNG/metadata files match the retained Wedding-pilot combined
build. New sources under `assets/animations/NPCs/Ryis/Sprites/Wedding/` use
the prefix `spr_npc_ryis_wedding_`. All retain 80×80 frames, Default atlas and
Middle/54 origin. Blink durations are `[0.075, 0.125, 0.075]`; sitting uses
single-frame defaults; general actions use
`[0.1, 0.25, 0.25, 0.25, 0.25, 0.1, 0.4]`; kiss uses
`[0.15, 0.15, 0.8, 0.15]`. Raw exported sidecars are in
`tmp/ryis-wedding-finish-author-metadata.json` and match the independent root
inventory.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Blink East | 53, 57, 53 |
| Blink South | 62, 66, 62 |
| Sit East | 47 |
| Sit North | 32 |
| Sit South | 56 |
| General action East | 48, 48, 51, 48, 51, 48, 49 |
| General action North | 30, 28, 28, 28, 28, 30, 34 |
| General action South | 56, 56, 56, 56, 56, 56, 58 |
| Kiss East | 49, 51, 56, 55 |

The 99 component seeds select 1,642 skin pixels per target with the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Every actual frame was inspected
in Vanilla and all four targets. Face, ears, eyelids, neck edge and hands
change, including the extended fingertips and detached hand components.
Cream suit shades (`FFEFDC`, `E8C8B0`, `C39B7A`, `986B57`), dark
collar/cuffs/belt (`21211E`, `30302C`, `3F3F3A`) and shoes remain original.
The warm `F49F7D`/`D06B53` neckwear is clothing, as confirmed against the
original Wedding neutral portrait during the pilot; it stays unchanged.
Hair, including the `5E423B` rear-head patch, eyes and outlines are preserved.
Evidence: `tmp/ryis-wedding-finish-author-components.json` and
`tmp/ryis-wedding-finish-author-art-{cycle}_{direction}-{page}.png`.

The [five-choice summary](../../generated/ryis-wedding-finish-preview/summary.png)
shows sitting South, general action East and kiss East. The
[complete Vanilla/Blue review](../../generated/ryis-wedding-finish-preview/blue-review/index.html)
covers all 49 cases across thirteen detail pages, with at most seven cases
per page. Crop `[25,20,32,39]` includes every visible pixel, including extended
hands, shoes and West mirrors. Each case also has an 8× face and collar detail.
Exact checks verified 9,687,840 preview pixels, all 98 full-frame and fifteen
summary bindings, metadata and West reversal. Chromium checked all fifteen
pages, including both indexes: every image and local link loaded, with no
horizontal overflow. Evidence:
`tmp/ryis-wedding-finish-author-preview-check.log` and
`tmp/ryis-wedding-finish-author-preview-browser-check.json`.

The candidate test passed before publishing the profile. The final opt-in
`tests/ryis_wedding_finish.rs` passed every new pixel in all four targets,
per-frame counts, sidecars and 38 literal skin/material landmarks. Practical
controls demonstrate an omitted hand component and palette spill into warm
neckwear. Log: `tmp/ryis-wedding-finish-author-corpus-test.log`. The final
`generated/ryis-wedding-finish-trial` preserves all 2,640 prior original and
variant PNG/metadata files byte-for-byte; all 1,092 variants pass exact-palette
validation (`tmp/ryis-wedding-finish-author-preservation.log`). Profile SHA-256:
`3eaa28197e8b3b13e0f0b2464c175d071a0cabdeae5b0a1151f1273bec7cc0ba`.

See [the combined integration report](world-wedding-finish.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish timing, natural outfit changes or live gameplay. The user approved this artwork for commit on 2026-09-25.
