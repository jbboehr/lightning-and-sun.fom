# Ryis's Wedding idle and walk

This slice adds six Wedding strips: fifteen source frames and five native West
mirrors. The profile now has 264 sources and 1,056 variants. All 258 prior
region objects, other profile fields, source colors and target mappings remain
unchanged. Wedding actions and specials remain outside this pilot.

Sources were exported from the read-only `tmp/fields-of-mistria/assets.zip`
into `extracted/ryis-wedding-pilot-study`. Every prior Ryis PNG hash and all 516
original PNG/metadata files match the retained Beach-swim combined build.
New sources under `assets/animations/NPCs/Ryis/Sprites/Wedding/` use the prefix
`spr_npc_ryis_wedding_`: idle and walk North/South/East. All retain 80×80
frames, Default atlas and Middle/54 origin. Idle uses the single-frame defaults;
walk has four frames at `0.15`. Raw exported sidecars are in
`tmp/ryis-wedding-pilot-author-metadata.json` and match the independent root inventory.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Idle East | 49 |
| Idle North | 34 |
| Idle South | 58 |
| Walk East | 49, 55, 49, 49 |
| Walk North | 34, 32, 34, 32 |
| Walk South | 58, 56, 58, 56 |

The fifty component seeds select 703 skin pixels per target with the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Every actual Wedding frame was
inspected in Vanilla and all four targets. Face, ears, neck edge and both hands
change. Cream suit shades (`FFEFDC`, `E8C8B0`, `C39B7A`, `986B57`), dark
collar/cuffs/belt (`21211E`, `30302C`, `3F3F3A`), and shoes remain original.
The warm `F49F7D`/`D06B53` neckwear is clothing, as confirmed against the
original Wedding neutral portrait; it stays unchanged. Hair, including the
`5E423B` rear-head patch, eyes and outlines are preserved. Evidence:
`tmp/ryis-wedding-pilot-author-components.json` and
`tmp/ryis-wedding-pilot-author-art-{cycle}_{direction}-0.png`.

The [five-choice summary](../../generated/ryis-wedding-pilot-preview/summary.png)
shows idle South, walk East and walk North. The [complete Vanilla/Blue review](../../generated/ryis-wedding-pilot-preview/blue-review/index.html)
covers all twenty cases across eight pages, with at most four cases per page.
Crop `[25,20,32,39]` includes every visible pixel, including moving hands,
stepping shoes and West mirrors. Each case also has an 8× face and collar detail.
Exact checks verified 4,231,200 preview pixels, all forty full-frame and fifteen
summary bindings, metadata and West reversal. Chromium checked all ten pages,
including both indexes: every image and local link loaded, with no horizontal
overflow. Evidence: `tmp/ryis-wedding-pilot-author-preview-check.log` and
`tmp/ryis-wedding-pilot-author-preview-browser-check.json`.

The candidate test passed before publishing the profile. The final opt-in
`tests/ryis_wedding_pilot.rs` passed every new pixel in all four targets,
per-frame counts, sidecars and 28 literal skin/material landmarks. Practical
controls cover an omitted hand component and palette spill into warm neckwear.
Log: `tmp/ryis-wedding-pilot-author-corpus-test.log`. The final
`generated/ryis-wedding-pilot-trial` preserves all 2,580 prior original and
variant PNG/metadata files byte-for-byte; all 1,056 variants pass exact-palette
validation (`tmp/ryis-wedding-pilot-author-preservation.log`). Profile SHA-256:
`4c0632c7099eb894a7b5994e3c49115c1e1cf16164dab02181109d77c15b1317`.

See [the combined integration report](world-wedding-pilot.md) for shared packaging,
native probes, full checks and installation. Static previews do not establish
timing, natural outfit changes or live gameplay. The user approved this artwork for commit on 2026-09-25.
