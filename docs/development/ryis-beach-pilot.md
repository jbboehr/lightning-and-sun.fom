# Ryis's Beach idle and walk

This slice adds six Beach strips: 15 source frames and five native West
mirrors. The profile now has 250 sources and 1,000 variants. All 244 prior
region objects, other profile fields, source colors and target mappings remain
unchanged. Beach actions and specials remain outside this pilot.

Sources were exported from the read-only `tmp/fields-of-mistria/assets.zip`
into `extracted/ryis-beach-pilot-study`. Every prior Ryis PNG hash and all 488
original PNG/metadata files match the retained Winter-finish combined build.
New sources under `assets/animations/NPCs/Ryis/Sprites/Beach/` use the prefix
`spr_npc_ryis_beach_`: idle and walk North/South/East. All retain 80×80 frames,
Default atlas and Middle/54 origin. Idle uses the single-frame defaults;
walk has four frames at `0.15`. Exact sidecars are in
`tmp/ryis-beach-pilot-author-metadata.json`.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Idle East | 91 |
| Idle North | 79 |
| Idle South | 102 |
| Walk East | 91, 94, 91, 88 |
| Walk North | 79, 70, 79, 71 |
| Walk South | 102, 94, 102, 93 |

The 138 component seeds select 1,326 skin pixels per target with the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Every actual Beach frame was
inspected in Vanilla and all four targets. Face, ears, neck, exposed upper
chest/back, arms, hands, legs and toes change. The pink tank (`FF948B`,
`FF5C64`, `D33159`), blue shorts and sandal straps (`3A4A6B`, `6482AA`,
`72B2D9`), and dark wristbands (`332727`, `354647`) remain original.
The `5E423B` rear-head patch remains short hair. Eyes, outlines and other
materials are unchanged. Evidence: `tmp/ryis-beach-pilot-author-components.json`
and `tmp/ryis-beach-pilot-author-art-{cycle}_{direction}-0.png`.

The [five-choice summary](../../generated/ryis-beach-pilot-preview/summary.png)
shows idle South, walk East and walk North. The [complete Vanilla/Blue review](../../generated/ryis-beach-pilot-preview/blue-review/index.html)
covers all twenty cases across eight pages, with at most four cases per page.
Crop `[25,20,32,39]` includes every visible pixel, including stepping feet and
West mirrors. Each case also has an 8× face and upper-chest detail. Exact checks
verified 4,231,200 preview pixels, all forty full-frame and fifteen summary
bindings, source metadata and West reversal. Chromium checked all ten pages,
including both indexes: every image and local link loaded, with no horizontal
overflow. Evidence: `tmp/ryis-beach-pilot-author-preview-check.log` and
`tmp/ryis-beach-pilot-author-preview-browser-check.json`.

The opt-in `tests/ryis_beach_pilot.rs` passed all new pixels in all four targets,
per-frame counts, sidecars and 29 literal skin/material landmarks. Two practical
controls confirm that omitting a leg component leaves the pinned shin unchanged,
and broadening the palette into blue clothing spills onto the pinned sandal
strap. Log: `tmp/ryis-beach-pilot-author-corpus-test.log`.
The final `generated/ryis-beach-pilot-trial` preserves all 2,440 prior original
and variant PNG/metadata files byte-for-byte; all 1,000 variants pass exact-palette
validation (`tmp/ryis-beach-pilot-author-preservation.log`). Profile SHA-256:
`560b692b592063be58db1620f1e87c65b21751475f07c174a0396b2b07de6621`.

See [the combined integration report](world-beach-pilot.md) for shared packaging,
native probes, full checks and installation. Static previews do not establish
timing, natural outfit changes or live gameplay. The user approved this slice for
commit on 2026-09-25.
